use matrix_macros::command;
use osu_lib::osu_client::OsuClient;
use crate::commands::{CommandContext, Ctx};
use crate::embeds;
use crate::error::ApplicationResult;

#[command("user")]
async fn user(ctx: CommandContext, username: String, Ctx(osu_client): Ctx<OsuClient>) -> ApplicationResult<()> {
    let user = osu_client.user(username).await?;
    let embed = embeds::user::create_user_embed(&user);

    ctx.room.send(embed)
        .await?;

    Ok(())
}