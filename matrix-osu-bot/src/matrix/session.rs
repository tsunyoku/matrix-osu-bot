use matrix_sdk::authentication::matrix::MatrixSession;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct FullSession {
    pub user_session: MatrixSession,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sync_token: Option<String>,
}