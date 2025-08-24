use crate::client::error::ClientError;
use serenity::all::{ResolvedOption, ResolvedValue};

pub mod create;
pub mod delete;
pub mod list_severs;
pub mod start;
pub mod stop;

pub fn extract_filter<'a>(
    index: usize,
    options: &'a [ResolvedOption<'_>],
) -> Result<&'a str, ClientError> {
    options
        .get(index)
        .and_then(|option| match &option.value {
            ResolvedValue::String(value) => Some(*value),
            _ => None,
        })
        .ok_or_else(|| ClientError::Other(format!("Invalid value at index {index}.")))
}
