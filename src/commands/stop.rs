use crate::client::error::ClientError;
use crate::commands::extract_filter;
use crate::database::postgresql::{PgPool, PgPooled};
use crate::database::schemas::servers::dsl as servers_dsl;
use crate::util::msg::Msg;
use crate::util::{EMBED_COLOR, get_pool_from_ctx};
use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl};
use diesel_async::RunQueryDsl;
use serenity::all::{
    CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed, ResolvedOption,
};
use tokio::process::Command;

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

    if serv_stoped {
        return Err(ClientError::OtherStatic("Ce serveur est déjà arrêté."));
    }

    Command::new("docker")
        .args(["compose", "down"])
        .current_dir(format!("worlds/{name}"))
        .status()
        .await?;

    diesel::update(servers_dsl::servers.filter(servers_dsl::name.eq(&name)))
        .set(servers_dsl::adresse.eq(None::<String>))
        .execute(&mut conn)
        .await?;

    log::info!("server stoped : {name}!");

    let msg = Msg {
        embed: CreateEmbed::new()
            .description("Serveur arrêté !")
            .color(EMBED_COLOR),
        buttons: vec![],
    };

    Ok(msg)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("stop")
        .description("Stop a server.")
        .description_localized("en-US", "Stop a server.")
        .description_localized("en-GB", "Stop a server.")
        .description_localized("fr", "Arrête un serveur.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "name",
                "Le nom du serveur a arrêté.",
            )
            .description_localized("en-US", "The name of the server to stop.")
            .description_localized("en-GB", "The name of the server to stop.")
            .required(true),
        )
}
