use crate::client::error::ClientError;
use crate::commands::extract_filter;
use crate::database::postgresql::PgPool;
use crate::database::postgresql::PgPooled;
use crate::database::schemas::servers::dsl as servers_dsl;
use crate::util::msg::Msg;
use crate::util::{EMBED_COLOR, get_pool_from_ctx};
use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl, insert_into};
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
    if serv_exist {
        return Err(ClientError::OtherStatic("Ce nom de serveur existe déjà."));
    }

    insert_into(servers_dsl::servers)
        .values((servers_dsl::name.eq(&name),))
        .execute(&mut conn)
        .await?;

    log::info!("Created \"{name}\" server!");

    let msg = Msg {
        embed: CreateEmbed::new()
            .description("Le serveur a bien été créé !")
            .color(EMBED_COLOR),
        buttons: vec![],
    };

    Ok(msg)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("create")
        .description("Create a server.")
        .description_localized("en-US", "Create a server.")
        .description_localized("en-GB", "Create a server.")
        .description_localized("fr", "Création d'un serveur.")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "name",
                "Le nom du serveur à créer.",
            )
            .description_localized("en-US", "The name of the server to be created.")
            .description_localized("en-GB", "The name of the server to be created.")
            .required(true),
        )
}
