use crate::client::error::ClientError;
use crate::database::postgresql::{PgPool, PgPooled};
use crate::database::schemas::servers::dsl as servers_dsl;
use crate::util::msg::Msg;
use crate::util::{EMBED_COLOR, get_pool_from_ctx};
use diesel::{QueryDsl, Queryable};
use diesel_async::RunQueryDsl;
use serenity::all::{
    Context, CreateActionRow, CreateButton, CreateCommand, CreateEmbedFooter,
    EditInteractionResponse,
};
use serenity::builder::CreateEmbed;

const ELEMENT_PER_PAGE: u64 = 8;

#[derive(Debug, Clone, Queryable)]
struct ServersList {
    pub name: String,
    pub adresse: Option<String>,
}

async fn get_servers(
    ctx: &Context,
    page: &mut u64,
) -> Result<(Vec<ServersList>, u64), ClientError> {
    let pool: PgPool = get_pool_from_ctx(ctx).await?;
    let mut conn: PgPooled = pool.get().await?;

    let servers_count = servers_dsl::servers
        .count()
        .get_result::<i64>(&mut conn)
        .await?;

    if *page != 0 {
        *page = (*page).min((servers_count as u64).div_ceil(ELEMENT_PER_PAGE));
    }

    #[allow(clippy::cast_possible_wrap)]
    let servers: Vec<ServersList> = servers_dsl::servers
        .select((servers_dsl::name, servers_dsl::adresse))
        .limit(ELEMENT_PER_PAGE as i64)
        .offset(i64::try_from(page.saturating_sub(1) * ELEMENT_PER_PAGE)?)
        .load::<ServersList>(&mut conn)
        .await?;

    Ok((servers, (servers_count as u64).div_ceil(ELEMENT_PER_PAGE)))
}

pub async fn run(ctx: &Context) -> Result<Msg, ClientError> {
    let (servers, pages_count) = get_servers(ctx, &mut 0).await?;
    if servers.is_empty() {
        let msg = Msg {
            embed: CreateEmbed::new()
                .description("Aucun serveur n'a été créé.")
                .color(EMBED_COLOR),
            buttons: vec![],
        };

        return Ok(msg);
    }

    let mut prefixes_strings: Vec<String> = Vec::new();
    for server in servers {
        prefixes_strings.push(format!(
            "* **{}**\n  * **Adresse** : [{}]",
            server.name,
            server.adresse.as_deref().unwrap_or(""),
        ));
    }

    let msg = Msg {
        embed: CreateEmbed::new()
            .title("Liste des serveurs")
            .description(prefixes_strings.join("\n"))
            .footer(CreateEmbedFooter::new(format!("Page 1/{pages_count}")))
            .color(EMBED_COLOR),
        buttons: vec![
            CreateButton::new("page-0")
                .label("Précédent")
                .disabled(true),
            CreateButton::new("page-2")
                .label("Suivant")
                .disabled(pages_count == 1),
        ],
    };

    Ok(msg)
}

pub fn register() -> CreateCommand {
    CreateCommand::new("list-servers")
        .description("Lists all created servers.")
        .description_localized("en-US", "Lists all created servers.")
        .description_localized("en-GB", "Lists all created servers.")
        .description_localized("fr", "Liste tous les serveurs créés.")
}

pub async fn get_page(
    ctx: &Context,
    mut page: u64,
) -> Result<EditInteractionResponse, ClientError> {
    page = page.max(1);
    let (servers, pages_count) = get_servers(ctx, &mut page).await?;
    if servers.is_empty() {
        let msg: EditInteractionResponse = EditInteractionResponse::new().embed(
            CreateEmbed::new()
                .description("Aucun serveur n'a été créé.")
                .color(EMBED_COLOR),
        );

        return Ok(msg);
    }

    let mut prefixes_strings: Vec<String> = Vec::new();
    for server in servers {
        prefixes_strings.push(format!(
            "* **{}**\n  * **Adresse** : [{}]",
            server.name,
            server.adresse.as_deref().unwrap_or(""),
        ));
    }

    let msg: EditInteractionResponse = EditInteractionResponse::new()
        .embed(
            CreateEmbed::new()
                .title("Liste des serveurs")
                .description(prefixes_strings.join("\n"))
                .footer(CreateEmbedFooter::new(format!("Page {page}/{pages_count}")))
                .color(EMBED_COLOR),
        )
        .components(vec![CreateActionRow::Buttons(vec![
            CreateButton::new(format!("page-{}", page - 1))
                .label("Précédent")
                .disabled(page == 1),
            CreateButton::new(format!("page-{}", page + 1))
                .label("Suivant")
                .disabled(pages_count == page),
        ])]);

    Ok(msg)
}
