use matrix_sdk::{Room, RoomState};
use matrix_sdk::event_handler::Ctx;
use matrix_sdk::ruma::OwnedUserId;
use matrix_sdk::ruma::events::room::message::{MessageType, OriginalSyncRoomMessageEvent};
use num_format::{Locale, ToFormattedString};
use tracing::{debug, info, warn};
use osu_lib::osu_client::OsuClient;
use crate::embed::EmbedBuilder;
use crate::error::ApplicationResult;
use crate::matrix::verification::PendingVerification;

pub(crate) async fn on_room_message(
    event: OriginalSyncRoomMessageEvent,
    room: Room,
    Ctx(osu_client): Ctx<OsuClient>,
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

        let user = osu_client.user("tsunyoku").await?;

        // TODO: when can the entire stats item be null rather than just pp/rank?
        let statistics = user.statistics.as_ref();

        let locale = &Locale::en;

        let title = format!(
            "{}: {:}pp (#{} {}{})",
            user.username,
            statistics.map(|stats| stats.pp as u32).unwrap_or(0).to_formatted_string(locale),
            statistics.map(|stats| stats.global_rank).flatten().unwrap_or(0).to_formatted_string(locale),
            user.country_code,
            statistics.map(|stats| stats.country_rank).flatten().unwrap_or(0).to_formatted_string(locale));

        let accuracy = statistics.map(|stats| stats.accuracy).unwrap_or(0.0);
        let play_count = statistics.map(|stats| stats.playcount).unwrap_or(0);

        let play_time_secs = statistics.map(|stats| stats.playtime as u64).unwrap_or(0);
        let play_time = (play_time_secs / 3600).to_formatted_string(locale);

        let peak_rank = user.highest_rank.map(|rank| rank.rank).unwrap_or(0).to_formatted_string(locale);

        let embed = EmbedBuilder::with_title(title)
            .field("Accuracy", format!("{:.2}", accuracy))
            .field("Playcount", format!("{} ({} hours)", play_count, play_time))
            .field("Peak rank", format!("#{}", peak_rank))
            .url(format!("https://osu.ppy.sh/users/{}", user.user_id))
            .url_text("View Profile")
            .build();

        room.send(embed)
            .await?;
    }

    Ok(())
}
