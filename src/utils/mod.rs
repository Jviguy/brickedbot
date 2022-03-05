use std::sync::Arc;
use std::sync::mpsc::channel;
use serenity::http::Http;
use serenity::model::channel::GuildChannel;
use serenity::model::id::{ChannelId, GuildId};

pub mod pinsec;

pub async fn find_channel(http: Arc<Http>, id: GuildId, name: String) -> Option<(ChannelId,GuildChannel)> {
    for t in id.channels(http.clone()).await.unwrap() {
        if t.1.name == name {
            return Some(t);
        }
    };
    None
}