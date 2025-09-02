use crate::client::error::ClientError;
use crate::commands::extract_str;
use crate::database::postgresql::{PgPool, PgPooled};
use crate::database::schemas::servers::dsl as servers_dsl;
use crate::util::{EMBED_COLOR, get_pool_from_ctx};
use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl, delete};
use diesel_async::RunQueryDsl;
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    CreateEmbed, CreateInteractionResponseMessage,
};
use tokio::fs;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), ClientError> {
    let name = extract_str("name", command.data.options())?.to_lowercase();

    let pool: PgPool = get_pool_from_ctx(ctx).await?;
    let mut conn: PgPooled = pool.get().await?;

    let serv_exist: bool = diesel::select(exists(
        servers_dsl::servers.filter(servers_dsl::name.eq(&name)),
    ))
    .get_result(&mut conn)
    .await?;

    if !serv_exist {
        return Err(ClientError::OtherStatic("Ce serveur n'existe pas."));
    }

    let serv_stoped: bool = diesel::select(exists(
        servers_dsl::servers
            .filter(servers_dsl::name.eq(&name))
            .filter(servers_dsl::adresse.is_null()),
    ))
    .get_result(&mut conn)
    .await?;

    if !serv_stoped {
        return Err(ClientError::OtherStatic("Ce serveur est lancé."));
    }

    fs::remove_dir_all(format!("worlds/{name}")).await?;

    delete(servers_dsl::servers)
        .filter(servers_dsl::name.eq(&name))
        .execute(&mut pool.get().await?)
        .await?;

    log::info!("Deleted server : {name}!");

    let embed = CreateEmbed::new()
        .description(format!("**Serveur ``{name}`` supprimé !**"))
        .color(EMBED_COLOR);

    command
        .create_response(
            &ctx.http,
            serenity::builder::CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new().add_embed(embed),
            ),
        )
        .await?;

    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("delete")
        .description("Delete a server.")
        .description_localized("en-US", "Delete a server.")
        .description_localized("en-GB", "Delete a server.")
        .description_localized("fr", "Suppression d'un serveur.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "name",
                "Le nom du serveur a supprimer.",
            )
            .description_localized("en-US", "The name of the server to delete.")
            .description_localized("en-GB", "The name of the server to delete.")
            .required(true)
            .max_length(100),
        )
}
