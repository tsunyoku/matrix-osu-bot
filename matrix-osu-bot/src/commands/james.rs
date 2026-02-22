use tracing::info;
use matrix_macros::command;
use osu_lib::osu_client::OsuClient;
use crate::commands::{CommandContext, Ctx};
use crate::embeds;
use crate::error::ApplicationResult;

#[command("james")]
async fn james(ctx: CommandContext, Ctx(osu_client): Ctx<OsuClient>) -> ApplicationResult<()> {
    info!(room_id = %ctx.room.room_id(), sender = %ctx.sender, "Responding to !james command");

    let user = osu_client.user("tsunyoku").await?;
    let embed = embeds::user::create_user_embed(&user);

    ctx.room.send(embed)
        .await?;

    Ok(())
}