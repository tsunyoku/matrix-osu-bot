mod james;
mod sas;
mod user;

use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::num::ParseIntError;
use std::pin::Pin;
use std::sync::Arc;
use matrix_sdk::{Client, Room};
use matrix_sdk::ruma::OwnedUserId;
use crate::error::ApplicationResult;

pub struct Ctx<T>(pub T);

#[derive(Clone, Default)]
pub struct CommandData(Arc<HashMap<TypeId, Box<dyn Any + Send + Sync>>>);

impl CommandData {
    pub fn get<T: Send + Sync + 'static>(&self) -> Option<&T> {
        self.0.get(&TypeId::of::<T>())?.downcast_ref()
    }
}

#[derive(Default)]
pub struct CommandDataBuilder(HashMap<TypeId, Box<dyn Any + Send + Sync>>);

impl CommandDataBuilder {
    pub fn add<T: Send + Sync + 'static>(&mut self, val: T) {
        self.0.insert(TypeId::of::<T>(), Box::new(val));
    }

    pub fn build(self) -> CommandData {
        CommandData(Arc::new(self.0))
    }
}

pub struct CommandContext {
    pub client: Client,
    pub room: Room,
    pub sender: OwnedUserId,
    pub data: CommandData,
}

type CommandFuture = Pin<Box<dyn Future<Output = ApplicationResult<()>> + Send>>;

pub struct Command {
    pub name: &'static str,
    pub handler: fn(CommandContext, Vec<String>) -> CommandFuture,
}

inventory::collect!(Command);

#[derive(Debug, thiserror::Error)]
pub enum ArgError {
    #[error("missing required argument `{0}`")]
    Missing(&'static str),

    #[error("invalid value for `{0}`: {1}")]
    Parse(&'static str, String),
}

pub trait FromArg: Sized {
    fn from_arg(name: &'static str, value: &str) -> Result<Self, ArgError>;
}

impl FromArg for String {
    fn from_arg(_: &'static str, value: &str) -> Result<Self, ArgError> {
        Ok(value.to_owned())
    }
}

impl FromArg for u32 {
    fn from_arg(name: &'static str, value: &str) -> Result<Self, ArgError> {
        value.parse().map_err(|e: ParseIntError| ArgError::Parse(name, e.to_string()))
    }
}

impl<T: FromArg> FromArg for Option<T> {
    fn from_arg(name: &'static str, value: &str) -> Result<Self, ArgError> {
        T::from_arg(name, value).map(Some)
    }
}