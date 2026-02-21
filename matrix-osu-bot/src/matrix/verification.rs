use std::sync::{Arc, Mutex};
use matrix_sdk::Client;
use matrix_sdk::encryption::verification::{SasState, SasVerification, Verification, VerificationRequest, VerificationRequestState};
use matrix_sdk::ruma::OwnedUserId;
use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use matrix_sdk::stream::StreamExt;
use tokio::sync::oneshot;
use tracing::{error, info, warn};
use crate::error::Result;

/// Shared between the verification handler and the room message handler.
/// Holds the sender half of a oneshot channel that the message handler fires
/// when the admin replies "yes" or "no".
pub(crate) type PendingVerification = Arc<Mutex<Option<oneshot::Sender<bool>>>>;

pub(crate) async fn handle_verification_request(
    request: VerificationRequest,
    client: Client,
    admin_user_id: OwnedUserId,
    pending: PendingVerification,
) -> Result<()> {
    request.accept().await?;

    let mut stream = request.changes();

    while let Some(state) = stream.next().await {
        match state {
            VerificationRequestState::Created { .. }
            | VerificationRequestState::Requested { .. }
            | VerificationRequestState::Ready { .. } => (),
            VerificationRequestState::Transitioned { verification } => {
                if let Verification::SasV1(sas) = verification {
                    tokio::spawn(sas_verification_handler(sas, client, admin_user_id, pending));
                    break;
                }
            }
            VerificationRequestState::Done | VerificationRequestState::Cancelled(_) => break,
        }
    }

    Ok(())
}

async fn sas_verification_handler(
    sas: SasVerification,
    client: Client,
    admin_user_id: OwnedUserId,
    pending: PendingVerification,
) -> Result<()> {
    sas.accept().await?;

    let mut stream = sas.changes();

    while let Some(state) = stream.next().await {
        match state {
            SasState::KeysExchanged { emojis, decimals: _ } => {
                let Some(emojis) = emojis else {
                    error!("Non-emoji SAS verification was attempted");
                    sas.cancel().await?;
                    return Ok(());
                };

                let emoji_str = emojis
                    .emojis
                    .iter()
                    .map(|e| format!("{} {}", e.symbol, e.description))
                    .collect::<Vec<_>>()
                    .join("  ");

                let msg = format!("SAS verification — do these emojis match on both devices?\n\n{emoji_str}\n\nReply `yes` to confirm or `no` to cancel.");

                if let Some(room) = client.get_dm_room(&admin_user_id) {
                    room.send(RoomMessageEventContent::text_plain(msg)).await?;
                }

                let (tx, rx) = oneshot::channel();
                *pending.lock().unwrap() = Some(tx);

                match rx.await {
                    Ok(true) => {
                        info!("Admin confirmed SAS verification, confirming");
                        sas.confirm().await?;
                    }
                    _ => {
                        warn!("Admin rejected or dropped SAS verification, cancelling");
                        sas.cancel().await?;
                        break;
                    }
                }
            }
            SasState::Done { .. } => {
                info!("SAS verification completed successfully");

                if let Some(room) = client.get_dm_room(&admin_user_id) {
                    room.send(RoomMessageEventContent::text_plain("Verification complete.")).await?;
                }

                break;
            }
            SasState::Cancelled(cancel_info) => {
                warn!(reason = %cancel_info.reason(), "SAS verification was cancelled");
                break;
            }
            SasState::Created { .. }
            | SasState::Started { .. }
            | SasState::Accepted { .. }
            | SasState::Confirmed => (),
        }
    }

    Ok(())
}
