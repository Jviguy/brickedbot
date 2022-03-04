use std::sync::Arc;
use serenity::model::interactions::application_command::{ApplicationCommandInteractionDataOptionValue, ApplicationCommandInteraction};
use serenity::model::interactions::InteractionResponseType;
use serenity::{
    prelude::*
};
use serenity::model::guild::Guild;
use serenity::model::id::{MessageId, RoleId};
use serenity::model::Permissions;
use serenity::utils::Colour;
use crate::gen;

pub async fn bulk_delete(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    let guild = Guild::get(&ctx.http, command.guild_id.unwrap()).await.unwrap();
    let option = command.data.options.get(0).unwrap().resolved.as_ref().unwrap();
    if let ApplicationCommandInteractionDataOptionValue::Integer(amount) = option {
        let channel = guild.channels(&ctx.http).await.unwrap().get(&command.channel_id).unwrap().clone();
        let l = channel.last_message_id.unwrap();
        channel.delete_messages(&ctx.http, (u64::from(l)-(*amount as u64)..u64::from(l))
            .map(|w|MessageId(w)).collect::<Vec<MessageId>>()).await.unwrap();
        Some(String::from(format!("Successfully deleted {} messages!", amount)))
    } else {
        None
    }
}

pub async fn gencode(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    // let guild = Guild::get(&ctx.http, command.guild_id.unwrap()).await.unwrap();
    // let role = guild.roles.get(&RoleId(948938714464280587)).unwrap();
    // if command.user.has_role(&ctx.http, command.guild_id.unwrap(), role).await.unwrap() {
    //     gen(ctx.http.clone()).await;
    //     Some(String::from("Successfully generated a new code!"))
    // } else {
    //     None
    // }
    gen(ctx.http.clone()).await;
    Some(String::from("Successfully generated a new code!"))
}