use std::sync::Arc;
use rosu_v2::{Osu, OsuResult};
use rosu_v2::prelude::{UserExtended, UserId};

#[derive(Clone)]
pub struct OsuClient {
    rosu: Arc<Osu>
}

impl OsuClient {
    pub async fn new(client_id: u64, client_secret: &str) -> OsuResult<Self> {
        let rosu = Osu::new(client_id, client_secret).await?;

        Ok(Self {
            rosu: Arc::new(rosu),
        })
    }

    pub async fn user(&self, user_id: impl Into<UserId>) -> OsuResult<UserExtended> {
        self.rosu.user(user_id).await
    }
}