use std::env;
use crate::error::ApplicationResult;

#[derive(Debug)]
pub(crate) struct Settings {
    pub osu_client_id: u64,
    pub osu_client_secret: String,
}

impl Settings {
    pub fn new() -> ApplicationResult<Self> {
        let osu_client_id_env = env::var("OSU_CLIENT_ID")?;
        let osu_client_id = osu_client_id_env.parse::<u64>()?;

        let osu_client_secret = env::var("OSU_CLIENT_SECRET")?;

        Ok(Self {
            osu_client_id,
            osu_client_secret,
        })
    }
}