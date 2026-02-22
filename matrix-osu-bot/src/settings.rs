use std::env;
use crate::error::ApplicationResult;

#[derive(Debug, Clone)]
pub(crate) struct Settings {
    pub osu_client_id: u64,
    pub osu_client_secret: String,

    pub command_prefix: String,
}

impl Settings {
    pub fn new() -> ApplicationResult<Self> {
        let osu_client_id_env = env::var("OSU_CLIENT_ID")?;
        let osu_client_id = osu_client_id_env.parse::<u64>()?;

        let osu_client_secret = env::var("OSU_CLIENT_SECRET")?;

        let command_prefix = env::var("COMMAND_PREFIX")?;

        Ok(Self {
            osu_client_id,
            osu_client_secret,
            command_prefix,
        })
    }
}