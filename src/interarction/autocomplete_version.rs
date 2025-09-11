use crate::client::error::ClientError;
use serenity::all::{CommandInteraction, Context, CreateAutocompleteResponse, ResolvedValue};
use tokio::fs;

pub async fn autocomplete_version(
    ctx: Context,
    command: CommandInteraction,
) -> Result<(), ClientError> {
    let json = fs::read_to_string("versions.json").await?;
    let versions: Vec<String> = serde_json::from_str(&json)?;
    let options = command.data.options();
    let opt = &options
        .iter()
        .find(|opt| opt.name == "version")
        .unwrap()
        .value;
    match opt {
        ResolvedValue::Autocomplete {
            kind: _,
            value: str,
        } => {
            let a: Vec<String> = versions
                .into_iter()
                .filter(|ver| ver.contains(str))
                .take(25)
                .collect();

            let mut auto_complete = CreateAutocompleteResponse::new();

            for b in a {
                auto_complete = auto_complete.add_string_choice(&b, &b);
            }

            command
                .create_response(
                    &ctx.http,
                    serenity::all::CreateInteractionResponse::Autocomplete(auto_complete),
                )
                .await?;
            Ok(())
        }
        _ => Err(ClientError::Other("invalid value".to_string())),
    }
}
