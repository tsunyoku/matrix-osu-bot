use matrix_sdk::{Room, RoomState};
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent, RoomMessageEventContent};
use tracing::{debug, info};
use crate::error::Result;

pub(crate) async fn on_room_message(event: OriginalSyncRoomMessageEvent, room: Room) -> Result<()> {
    if room.state() != RoomState::Joined {
        return Ok(());
    }

    let MessageType::Text(message_content) = event.content.msgtype else {
        return Ok(());
    };

    debug!(sender = %event.sender, body = %message_content.body, "Received text message");

    if message_content.body.starts_with("!james") {
        info!(room_id = %room.room_id(), sender = %event.sender, "Responding to !james command");

        let response_content = RoomMessageEventContent::text_plain("TSUNYOKU");

        room.send(response_content)
            .await?;
    }

    Ok(())
}