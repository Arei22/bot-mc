use crate::client::error::ClientError;
use serenity::all::{ResolvedOption, ResolvedValue};

pub mod create;
pub mod delete;
pub mod list;
pub mod start;
pub mod stop;

pub fn extract_str<'a>(
    name: &str,
    options: Vec<ResolvedOption<'a>>,
) -> Result<&'a str, ClientError> {
    options.iter().find(|option| option.name == name).map_or(
        Err(ClientError::Other(format!("Missing arg {name}."))),
        |opt| match &opt.value {
            ResolvedValue::String(value) => Ok(*value),
            _ => Err(ClientError::Other(format!("Invalid value for arg {name}."))),
        },
    )
}

pub fn extract_str_optional<'a>(
    name: &str,
    options: Vec<ResolvedOption<'a>>,
) -> Result<Option<&'a str>, ClientError> {
    options
        .iter()
        .find(|option| option.name == name)
        .map_or(Ok(None), |option| match &option.value {
            ResolvedValue::String(value) => Ok(Some(*value)),
            _ => Err(ClientError::Other(format!("Invalid value for arg {name}."))),
        })
}
