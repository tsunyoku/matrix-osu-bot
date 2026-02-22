use matrix_sdk::ruma::events::room::message::RoomMessageEventContent;
use num_format::{Locale, ToFormattedString};
use osu_lib::user::UserExtended;
use crate::embeds::builder::EmbedBuilder;

pub(crate) fn create_user_embed(user: &UserExtended) -> RoomMessageEventContent {
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
    let play_count = statistics.map(|stats| stats.playcount).unwrap_or(0).to_formatted_string(locale);

    let play_time_secs = statistics.map(|stats| stats.playtime as u64).unwrap_or(0);
    let play_time = (play_time_secs / 3600).to_formatted_string(locale);

    let peak_rank = user.highest_rank
        .as_ref()
        .map(|rank| rank.rank)
        .unwrap_or(0)
        .to_formatted_string(locale);

    EmbedBuilder::with_title(title)
        .field("Accuracy", format!("{:.2}%", accuracy))
        .field("Playcount", format!("{} ({} hours)", play_count, play_time))
        .field("Peak rank", format!("#{}", peak_rank))
        .url(format!("https://osu.ppy.sh/users/{}", user.user_id))
        .url_text("View Profile")
        .build()
}