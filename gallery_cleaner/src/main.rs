use chrono::Utc;
use serenity::{
    futures::StreamExt,
    http::Http,
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::env;
use tracing::{error, info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() {
    // Log/Output settings
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    // load config
    dotenv::from_filename("./.env").expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected token in env.");
    let admin_channel = str_to_channel_id(
        &env::var("ADMIN_CHANNEL_ID").expect("Expected admin channel id in env."),
    );
    let channels_to_clean = env::vars().filter(|c| c.0.starts_with("PURGE_CHANNEL_ID"));

    // connect to api and clean
    info!("Starting clean job.");
    let client = Client::builder(&token).await.expect("Err creating client");
    let ctx = &client.cache_and_http.http;

    for (_, channelid) in channels_to_clean {
        let channel = str_to_channel_id(&channelid);
        let channel_name = channel
            .to_channel(&client.cache_and_http)
            .await
            .unwrap()
            .guild()
            .unwrap()
            .name; // for some reason .name() on ChannelId does not work.

        let (purge_count, kept_count) = purge_channel(&channel, ctx).await;

        let delete_count_msg = format!(
            "Deleted {} messages in channel {}. Kept {}.",
            purge_count, channel_name, kept_count
        );

        admin_channel.say(ctx, &delete_count_msg).await.unwrap();
        info!("{}", &delete_count_msg);
    }
}

async fn purge_channel(channel: &ChannelId, ctx: &Http) -> (u64, u64) {
    let mut count_deleted = 0;
    let mut count_kept = 0;

    let mut messages = channel.messages_iter(&ctx).boxed();

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
                } else {
                    count_kept += 1;
                }
            }
            Err(error) => error!("Error fetching messages: {}", error),
        }
    }
    (count_deleted, count_kept)
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
