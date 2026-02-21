use std::time::Duration;
use matrix_sdk::{Client, Room};
use matrix_sdk::ruma::events::room::member::StrippedRoomMemberEvent;
use tracing::{error, info, warn};
use crate::error::ApplicationResult;

const DELAY: u64 = 2;

pub(crate) async fn on_stripped_state_member(event: StrippedRoomMemberEvent, room: Room, client: Client) -> ApplicationResult<()> {
    if event.state_key != client.user_id().expect("missing user ID on client") {
        return Ok(());
    }

    info!(room_id = %room.room_id(), "Auto joining room");

    let mut total_delay = DELAY;

    while let Err(err) = room.join().await {
        warn!(room_id = %room.room_id(), "Failed to join room, retrying in {total_delay}");

        tokio::time::sleep(Duration::from_secs(total_delay)).await;
        total_delay *= 2;

        if total_delay > 3600 {
            error!(room_id = %room.room_id(), "Failed to join room: {err}");
            break;
        }
    }

    info!(room_id = %room.room_id(), "Joined room");

    Ok(())
}