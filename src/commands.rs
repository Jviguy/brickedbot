
use serenity::model::interactions::application_command::{ApplicationCommandInteractionDataOptionValue, ApplicationCommandInteraction};
use serenity::{
    prelude::*
};
use serenity::model::guild::Guild;
use serenity::model::id::{RoleId};
use serenity::model::prelude::application_command::ApplicationCommandOptionType;
use serenity::utils::{Color};
use source_query::info::ServerType;
use crate::gen;

pub async fn bulk_delete(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    let guild = Guild::get(&ctx.http, command.guild_id.unwrap()).await.unwrap();
    let option = command.data.options.get(0).unwrap().resolved.as_ref().unwrap();
    if let ApplicationCommandInteractionDataOptionValue::Integer(amount) = option {
        let channel = guild.channels(&ctx.http).await.unwrap().get(&command.channel_id).unwrap().clone();
        let l = channel.last_message_id.unwrap();
        let messages = channel.messages(&ctx.http, |b| {
            b.limit(*amount as u64).around(l)
        }).await.unwrap();
        channel.delete_messages(&ctx.http, messages).await.unwrap();
        command.create_interaction_response(&ctx.http, |response| {
            response.interaction_response_data(|data| {
                data.embed(|e| {
                    e
                        .title("Success!")
                        .description(format!("Deleted {} Messages!", amount))
                        .color(Color::ORANGE)
                })
            })
        }).await.unwrap();
    }
    None
}

pub async fn gencode(ctx: &Context, _: &ApplicationCommandInteraction) -> Option<String> {
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

pub async fn query(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    let ipoption = command.data.options.get(0).unwrap().resolved.as_ref().unwrap();
    let mut port: u16 = 28015;
    let portoption = command.data.options.get(1);
    if let ApplicationCommandInteractionDataOptionValue::String(ip) = ipoption {
        if let Some(portoptio) = portoption {
            let portoption = portoptio.resolved.as_ref().unwrap();
            if let ApplicationCommandInteractionDataOptionValue::Integer(p) = portoption {
                port = *p as u16;
            }
        }
        let res = source_query::info::query(format!("{}:{}", ip, port), None);
        match res {
            Ok(res) => {
                command.create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|data| {
                        data.embed(|e| {
                            e.title(res.name)
                                .field("Players: ", format!("**{}/{}**",res.players, res.max_players), true)
                                .field("Key Words: ", res.keywords.unwrap(), true)
                                .field("Game: ", res.game , true)
                                .color(Color::ORANGE)
                        })
                    })
                }).await.unwrap();
            }
            Err(e) => {
                command.create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|data| {
                        data.embed(|e| {
                            e.title("Error!")
                                .description("The server is either down or you inputted a invalid response!")
                                .color(Color::ORANGE)
                        })
                    })
                }).await.unwrap();
            }
        };
    };
    None
}

pub async fn mlrs(ctx: &Context, command: &ApplicationCommandInteraction) -> Option<String> {
    let channel = command.channel_id.to_channel(ctx).await.unwrap();
    let gc = channel.clone().guild().unwrap().clone();
    let permissions = gc.permission_overwrites;
    let channel = command.channel_id.delete(&ctx.http).await.unwrap();
    let channel = command.guild_id.unwrap().create_channel(&ctx.http, |builder| {
        if let Some(category) = gc.category_id {
            builder.category(category);
        };
        if let Some(topic) = gc.topic {
            builder.topic(topic);
        }
        if let Some(rate) = gc.slow_mode_rate {
            builder.rate_limit_per_user(rate);
        }
        if let Some(bitrate) = gc.bitrate {
            builder.bitrate(bitrate as u32);
        }
        if let Some(user_limit) = gc.user_limit {
            builder.user_limit(user_limit as u32);
        }
        builder
            .kind(gc.kind)
            .name(gc.name)
            .position(gc.position as u32)
            .permissions(permissions)
            .nsfw(gc.nsfw)
    }).await.unwrap();
    channel.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e
                .title("MLRS'ED")
                .image("https://thumbs.gfycat.com/NewBouncyBovine-max-1mb.gif")
                .color(Color::ORANGE)
        })
    }).await.unwrap();
    None
}
