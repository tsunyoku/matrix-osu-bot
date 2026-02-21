use std::env;
use std::path::PathBuf;
use crate::error::ApplicationResult;

#[derive(Debug)]
pub(crate) struct MatrixSettings {
    pub data_directory: PathBuf,

    pub homeserver: String,
    pub database_passphrase: String,

    pub username: String,
    pub password: String,

    /// The Matrix user ID of the bot's admin (e.g. `@you:matrix.org`).
    /// Verification requests and emoji confirmations are restricted to this user.
    pub admin_user_id: String,
}

impl MatrixSettings {
    pub fn new() -> ApplicationResult<Self> {
        let data_directory_env = env::var("DATA_DIRECTORY")?;
        let data_directory = PathBuf::from(data_directory_env);

        let homeserver = env::var("HOMESERVER")?;
        let database_passphrase = env::var("DATABASE_PASSPHRASE")?;

        let username = env::var("USERNAME")?;
        let password = env::var("PASSWORD")?;

        let admin_user_id = env::var("ADMIN_USER_ID")?;

        Ok(Self {
            data_directory,
            homeserver,
            database_passphrase,
            username,
            password,
            admin_user_id,
        })
    }

    pub fn session_file(&self) -> PathBuf {
        self.data_directory.join("session")
    }

    pub fn database_directory(&self) -> PathBuf {
        self.data_directory.join("database")
    }
}