use matrix_sdk::ClientBuildError;
use thiserror::Error;
use tokio::io;
use osu_lib::OsuError;
use crate::commands::ArgError;

pub type ApplicationResult<T, E = ApplicationError> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum ApplicationError {
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

    #[error("error retrieving environment variable")]
    EnvironmentVariableError(#[from] std::env::VarError),

    #[error("error parsing integer")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error("osu! error")]
    OsuError(#[from] OsuError),

    #[error("command arg error")]
    CommandArgError(#[from] ArgError),
}