use matrix_sdk::{Client, Room, RoomState};
use matrix_sdk::event_handler::Ctx;
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent};
use tracing::debug;
use crate::commands::{Command, CommandContext, CommandData};
use crate::error::ApplicationResult;
use crate::settings::Settings;

pub(crate) async fn on_room_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    client: Client,
    Ctx(command_data): Ctx<CommandData>,
    Ctx(settings): Ctx<Settings>,
) -> ApplicationResult<()> {
    if room.state() != RoomState::Joined {
        return Ok(());
    }

    let MessageType::Text(message_content) = event.content.msgtype else {
        return Ok(());
    };

    debug!(sender = %event.sender, body = %message_content.body, "Received text message");

    let command_context = CommandContext {
        client,
        room,
        sender: event.sender,
        data: command_data,
    };

    let body = message_content.body.trim();

    dispatch(&settings.command_prefix, body, command_context).await?;

    Ok(())
}

async fn dispatch(prefix: &str, body: &str, ctx: CommandContext) -> ApplicationResult<bool> {
    let body = body.trim();

    if !body.starts_with(prefix) {
        return Ok(false);
    }

    let mut parts = body[prefix.len()..].split_whitespace();

    let first = parts.next().unwrap_or("");
    let rest: Vec<String> = parts.map(str::to_owned).collect();

    for cmd in inventory::iter::<Command> {
        let matched = cmd.name == first;

        if matched {
            (cmd.handler)(ctx, rest).await?;
            return Ok(true);
        }
    }

    Ok(false)
}