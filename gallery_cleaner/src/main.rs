use chrono::Utc;
use serenity::{
    futures::StreamExt,
    http::Http,
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::env;
use tracing::error;

#[tokio::main]
async fn main() {
    dotenv::from_filename("./.env").expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected token in env.");
    let admin_channel = str_to_channel_id(
        &env::var("ADMIN_CHANNEL_ID").expect("Expected admin channel id in env."),
    );
    let channels_to_clean = env::vars().filter(|c| c.0.starts_with("PURGE_CHANNEL_ID"));

    let client = Client::builder(&token).await.expect("Err creating client");
    let ctx = &client.cache_and_http.http;

    for (_, channelid) in channels_to_clean {
        let purge_count = purge_channel(&channelid, ctx).await;
        let delete_count_msg = format!("Deleted {} messages in channel {}", purge_count, channelid);
        admin_channel.say(ctx, delete_count_msg).await.unwrap();
    }
}

async fn purge_channel(channel_id: &str, ctx: &Http) -> u64 {
    let channel_id = str_to_channel_id(channel_id);
    let mut count_deleted = 0;

    let mut messages = channel_id.messages_iter(&ctx).boxed();

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message.attachments.len() == 0
                    && message_older_then_one_day(&message)
                    && message_not_cdn(&message)
                {
                    match message.delete(&ctx).await {
                        Ok(_) => count_deleted += 1,
                        Err(error) => {
                            error!("Error deleting msg: {}. Error: {}", message.id, error)
                        }
                    }
                }
            }
            Err(error) => error!("Error fetching messages: {}", error),
        }
    }
    count_deleted
}

fn message_not_cdn(msg: &Message) -> bool {
    // TODO: move to config
    let allowed_urls = vec![
        "instagram.com",
        "imgur.com",
        "cdn.discordapp.com",
        "media.jipvankuijk.nl",
    ];

    for url in allowed_urls {
        if msg.content.contains(url) {
            return false;
        }
    }

    true
}

fn message_older_then_one_day(msg: &Message) -> bool {
    let now = Utc::now();
    now.signed_duration_since(msg.timestamp).num_seconds() > 86400 // one day in seconds
}

fn str_to_channel_id(id_as_str: &str) -> ChannelId {
    let channel_id: u64 = id_as_str.parse().expect("Error parsing purge channel id.");
    ChannelId(channel_id)
}
