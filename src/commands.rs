use std::sync::Arc;
use serenity::model::interactions::application_command::{ApplicationCommandInteractionDataOptionValue, ApplicationCommandInteraction};
use serenity::model::interactions::InteractionResponseType;
use serenity::{
    prelude::*
};
use serenity::model::guild::Guild;
use serenity::model::id::RoleId;
use serenity::utils::Colour;
use crate::gen;

pub async fn bulk_delete(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    None
}

pub async fn gencode(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    let guild = Guild::get(&ctx.http, command.guild_id.unwrap()).await.unwrap();
    let role = guild.roles.get(&RoleId(948938714464280587)).unwrap();
    if command.user.has_role(&ctx.http, command.guild_id.unwrap(), role).await.unwrap() {
        gen(ctx.http.clone()).await;
        Some(String::from("Successfully generated a new code!"))
    } else {
        None
    }
}