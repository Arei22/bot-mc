use crate::client::error::ClientError;
use crate::commands::extract_str;
use crate::commands::extract_str_optional;
use crate::database::postgresql::PgPool;
use crate::database::postgresql::PgPooled;
use crate::database::schemas::servers::dsl as servers_dsl;
use crate::util::parse_key;
use crate::util::{EMBED_COLOR, get_pool_from_ctx};
use diesel::dsl::exists;
use diesel::{ExpressionMethods, QueryDsl, insert_into};
use diesel_async::RunQueryDsl;
use serde_yml::Mapping;
use serde_yml::Value;
use serenity::all::CommandInteraction;
use serenity::all::CreateInteractionResponseMessage;
use serenity::all::{CommandOptionType, Context, CreateCommand, CreateCommandOption, CreateEmbed};
use tokio::fs;

pub async fn run(ctx: &Context, command: &CommandInteraction) -> Result<(), ClientError> {
    let name = extract_str("name", command.data.options())?.to_lowercase();
    let ver = extract_str_optional("version", command.data.options())?;

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

    let mut services = Mapping::new();

    let mut mc = Mapping::new();
    mc.insert(
        Value::String("image".into()),
        Value::String("itzg/minecraft-server".into()),
    );
    mc.insert(Value::String("tty".into()), Value::Bool(true));
    mc.insert(Value::String("stdin_open".into()), Value::Bool(true));
    mc.insert(
        Value::String("ports".into()),
        Value::Sequence(vec![Value::String("25565:25565".into())]),
    );

    let mut env = Mapping::new();
    env.insert(Value::String("EULA".into()), Value::String("TRUE".into()));
    env.insert(
        Value::String("OPS".to_string()),
        Value::String(parse_key::<String>("ADMIN_PLAYER")?),
    );
    if let Some(version) = ver {
        env.insert(
            Value::String("VERSION".to_string()),
            Value::String(version.to_string()),
        );
    }

    mc.insert(Value::String("environment".into()), Value::Mapping(env));

    mc.insert(
        Value::String("volumes".into()),
        Value::Sequence(vec![Value::String("./data:/data".into())]),
    );

    services.insert(Value::String("mc".into()), Value::Mapping(mc));

    let mut root = Mapping::new();
    root.insert(Value::String("services".into()), Value::Mapping(services));

    let yml_str = serde_yml::to_string(&root)?;

    let dir = format!("worlds/{name}");

    fs::create_dir_all(&dir).await?;
    fs::write(format!("{dir}/docker-compose.yml"), yml_str).await?;

    insert_into(servers_dsl::servers)
        .values((
            servers_dsl::name.eq(&name),
            servers_dsl::version.eq(ver.map_or_else(|| "latest", |version| version).to_string()),
        ))
        .execute(&mut conn)
        .await?;

    log::info!("Created \"{name}\" server!");

    let embed = CreateEmbed::new()
        .description(format!("**Le serveur ``{name}`` a bien été créé !**"))
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
            .required(true)
            .max_length(25),
        )
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::String,
                "version",
                "La version du serveur.",
            )
            .description_localized("en-US", "The version of the server to be created.")
            .description_localized("en-GB", "The version of the server to be created."),
        )
}
