extern crate core;

mod commands;
mod utils;

use rand::Rng;
use std::env;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use chrono::*;
// Scheduler, and trait for .seconds(), .minutes(), etc.
use clokwerk::{AsyncScheduler, TimeUnits};
// Import week days and WeekDay
use clokwerk::Interval::*;
use std::thread;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use chrono::format::Numeric::Timestamp;
use chrono::Weekday::Fri;

use serenity::{
    async_trait,
    model::{
        gateway::{Ready},
        id::GuildId,
        interactions::{
            application_command::{
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};
use serenity::model::id::ChannelId;
use serenity::utils::Color;
use crate::utils::pinsec::{gen, score};

struct Handler {
    is_loop_running: AtomicBool,
}

#[async_trait]
impl EventHandler for Handler {

    async fn cache_ready(&self, ctx: Context, _guilds: Vec<GuildId>) {
        let ctx = Arc::new(ctx);
        if !self.is_loop_running.load(Ordering::Relaxed) {
            // We have to clone the Arc, as it gets moved into the new thread.
            let ctx1 = Arc::clone(&ctx);
            // tokio::spawn creates a new green thread that can run in parallel with the rest of
            // the application.
            tokio::spawn(async move {
                loop {
                    gen(ctx1.http.clone()).await;
                    tokio::time::sleep(chrono::Duration::days(7).to_std().unwrap()).await;
                }
            });
            // Now that the loop is running, we set the bool to true
            self.is_loop_running.swap(true, Ordering::Relaxed);
        }
    }

    async fn ready(&self, ctx: Context, _: Ready) {
        let guild = GuildId(948931516031959062);
        guild.set_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command.")
                })
                .create_application_command(|command| {
                    command.name("query").description("Returns information on a given server.")
                })
                .create_application_command(|command| {
                    command.name("gencode").description("Generates a random code for this week.")
                })
        }).await.expect("Failed to make slash commands! (fuck me if this happens)");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => Some("pong!".to_string()),
                "gencode" => commands::gencode(&ctx, &command).await,
                "bulkdelete" => commands::bulk_delete(&ctx, &command).await,


                _ => Some("unimplemented command".to_string())
            };
            match content {
                None => {}
                Some(content) => {
                    if let Err(why) = command
                        .create_interaction_response(&ctx.http, |response| {
                            response
                                .kind(InteractionResponseType::ChannelMessageWithSource)
                                .interaction_response_data(|message| message.content(content))
                        })
                        .await
                    {
                        println!("Cannot respond to slash command: {}", why);
                    }
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("TOKEN").expect("Expected a token in the environment");;

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = 949139214363131924;

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler {
            is_loop_running: AtomicBool::new(false),
        })
        .application_id(application_id)
        .await
        .expect("Error creating client");
    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
