use crate::client::error::ClientError;
use crate::commands::extract_filter;
use crate::database::postgresql::{PgPool, PgPooled};
use crate::database::schemas::servers::dsl as servers_dsl;
use crate::util::msg::Msg;
use crate::util::{EMBED_COLOR, get_pool_from_ctx};
use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl, delete};
use diesel_async::RunQueryDsl;
use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
};

pub async fn run(ctx: &Context, options: &[ResolvedOption<'_>]) -> Result<Msg, ClientError> {
    let name = extract_filter(0, options)?.to_lowercase();

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

    delete(servers_dsl::servers)
        .filter(servers_dsl::name.eq(&name))
        .execute(&mut pool.get().await?)
        .await?;

    log::info!("Deleted server : {name}!");

    let msg = Msg {
        embed: CreateEmbed::new()
            .description("Serveur supprimé !")
            .color(EMBED_COLOR),
        buttons: vec![],
    };

    Ok(msg)
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
            .required(true),
        )
}
