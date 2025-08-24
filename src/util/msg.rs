use serenity::all::{CreateButton, CreateEmbed};

pub struct Msg {
    pub embed: CreateEmbed,
    pub buttons: Vec<CreateButton>,
}
