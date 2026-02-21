use config::ConfigError;
use matrix_sdk::ClientBuildError;
use thiserror::Error;
use tokio::io;

pub type Result<T, E = ApplicationError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("failed to parse config")]
    ConfigError(#[from] ConfigError),

    #[error("failed on file operation")]
    FileError(#[from] io::Error),

    #[error("failed on JSON operation")]
    SerdeJsonError(#[from] serde_json::error::Error),

    #[error("failed to build matrix client")]
    MatrixClientBuildError(#[from] ClientBuildError),

    #[error("failed on matrix operation")]
    MatrixError(#[from] matrix_sdk::Error),

    #[error("invalid Matrix user ID")]
    UserIdError(#[from] matrix_sdk::ruma::IdParseError),
}