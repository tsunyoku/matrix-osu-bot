use matrix_sdk::ruma::OwnedUserId;
use tracing::{info, warn};
use matrix_macros::command;
use crate::commands::{CommandContext, Ctx};
use crate::error::ApplicationResult;
use crate::matrix::verification::PendingVerification;

#[command("sas")]
async fn sas(
    ctx: CommandContext,
    decision: String,
    Ctx(admin_user_id): Ctx<OwnedUserId>,
    Ctx(pending_verification): Ctx<PendingVerification>,
) -> ApplicationResult<()> {
    if ctx.sender != admin_user_id {
        return Ok(());
    }

    if decision != "yes" && decision != "no" {
        return Ok(());
    }

    if let Some(tx) = pending_verification.lock().unwrap().take() {
        let confirmed = decision == "yes";

        info!(confirmed, "Received verification response from admin");

        if let Err(_) = tx.send(confirmed) {
            warn!("Sending SAS confirmation dropped");
        }
    }

    Ok(())
}