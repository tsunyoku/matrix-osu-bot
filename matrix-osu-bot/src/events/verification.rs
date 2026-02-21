use matrix_sdk::Client;
use matrix_sdk::event_handler::Ctx;
use matrix_sdk::ruma::events::key::verification::request::ToDeviceKeyVerificationRequestEvent;
use matrix_sdk::ruma::OwnedUserId;
use crate::error::ApplicationResult;
use crate::matrix::verification::{handle_verification_request, PendingVerification};

pub(crate) async fn on_device_key_verification_request(
    event: ToDeviceKeyVerificationRequestEvent,
    client: Client,
    Ctx(admin_user_id): Ctx<OwnedUserId>,
    Ctx(pending_verification): Ctx<PendingVerification>,
) -> ApplicationResult<()> {
    let request = client
        .encryption()
        .get_verification_request(&event.sender, &event.content.transaction_id)
        .await
        .expect("request is in a bad state");

    tokio::spawn(handle_verification_request(request, client, admin_user_id, pending_verification));

    Ok(())
}