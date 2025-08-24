pub mod data;
pub mod error;
mod serenity_handler;

use crate::client::data::PgPoolData;
use crate::client::serenity_handler::SerenityHandler;
use crate::database::postgresql::get_pool;
use crate::util::parse_key;
use serenity::prelude::GatewayIntents;
use std::error::Error;

pub struct Client {
    client: serenity::Client,
}

impl Client {
    pub async fn new() -> Result<Self, Box<dyn Error>> {
        let intents: GatewayIntents = GatewayIntents::GUILD_MEMBERS
            | GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;
        let client: serenity::Client =
            serenity::Client::builder(parse_key::<String>("DISCORD_TOKEN")?, intents)
                .event_handler(SerenityHandler)
                .application_id(parse_key::<u64>("DISCORD_APP_ID")?.into())
                .await?;

        client
            .data
            .write()
            .await
            .insert::<PgPoolData>(get_pool().await);

        Ok(Self { client })
    }

    #[inline]
    pub async fn start(&mut self) -> Result<(), serenity::Error> {
        self.client.start().await
    }
}
