use matrix_sdk::{Room, RoomState};
use matrix_sdk::event_handler::Ctx;
use matrix_sdk::ruma::OwnedUserId;
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};
use tracing::{debug, info, warn};
use crate::embed::EmbedBuilder;
use crate::error::ApplicationResult;
use crate::matrix::verification::PendingVerification;

pub(crate) async fn on_room_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    Ctx(pending_verification): Ctx<PendingVerification>,
    Ctx(admin_user_id): Ctx<OwnedUserId>,
) -> ApplicationResult<()> {
    if room.state() != RoomState::Joined {
        return Ok(());
    }

    let MessageType::Text(message_content) = event.content.msgtype else {
        return Ok(());
    };

    debug!(sender = %event.sender, body = %message_content.body, "Received text message");

    if event.sender == admin_user_id {
        let body = message_content.body.trim();

        if body == "!sas yes" || body == "!sas no" {
            if let Some(tx) = pending_verification.lock().unwrap().take() {
                let confirmed = body == "!sas yes";

                info!(confirmed, "Received verification response from admin");

                if let Err(_) = tx.send(confirmed) {
                    warn!("Sending SAS confirmation dropped");
                }

                return Ok(());
            }
        }
    }

    if message_content.body.starts_with("!james") {
        info!(room_id = %room.room_id(), sender = %event.sender, "Responding to !james command");

        let embed = EmbedBuilder::with_title("TSUNYOKU")
            .field("my field", "my value")
            .url("https://osu.ppy.sh/users/tsunyoku")
            .url_text("JAMES TSUNYOKU OSU LINK")
            .build();

        room.send(embed)
            .await?;
    }

    Ok(())
}
