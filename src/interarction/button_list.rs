use crate::commands::list::get_page;
use serenity::all::{ComponentInteraction, Context, CreateEmbed, EditInteractionResponse};

pub async fn button_list(ctx: Context, component: ComponentInteraction) {
    let page = component
        .data
        .custom_id
        .split('-')
        .next_back()
        .and_then(|page| page.parse::<u64>().ok())
        .unwrap_or_default();

    if let Err(e) = component.defer(&ctx.http).await {
        log::error!("Failed to defer interaction: {e}");
        return;
    }
    if let Err(e) = component
        .edit_response(
            &ctx,
            get_page(&ctx, page).await.unwrap_or_else(|error| {
                EditInteractionResponse::new()
                    .embed(CreateEmbed::new().description(error.to_string()))
            }),
        )
        .await
    {
        log::error!("Failed to edit interaction response: {e}");
    }
}
