use std::path::PathBuf;
use config::{Config, ConfigError, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct MatrixSettings {
    pub data_directory: PathBuf,

    pub homeserver: String,
    pub database_passphrase: String,

    pub username: String,
    pub password: String,
}

impl MatrixSettings {
    pub fn new() -> Result<Self, ConfigError> {
        let config = Config::builder()
            .add_source(Environment::default())
            .build()?;

        config.try_deserialize()
    }

    pub fn session_file(&self) -> PathBuf {
        self.data_directory.join("session")
    }

    pub fn database_directory(&self) -> PathBuf {
        self.data_directory.join("database")
    }
}