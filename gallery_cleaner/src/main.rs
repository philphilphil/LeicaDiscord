use chrono::Utc;
use serenity::{
    futures::StreamExt,
    http::Http,
    model::{channel::Message, id::ChannelId},
    prelude::*,
};
use std::env;
use tracing::{error};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let client = Client::builder(&token).await.expect("Err creating client");

    // for (_, channelid) in purge_chan_ids {
    let purge_count = purge_channel("810536807761707121", &client.cache_and_http.http).await;

    //let purge_count = purge_channel(&channelid, ctx).await;
    let delete_count_msg = format!("Deleted {} messages in channel {}", purge_count, "");
    // msg.channel_id.say(&ctx.http, delete_count_msg).await?;
    // }

    ChannelId(818753943642964011)
        .say(&client.cache_and_http.http, delete_count_msg)
        .await
        .unwrap();

}

async fn purge_channel(channel_id: &str, ctx: &Http) -> u64 {
    let channel_id: u64 = channel_id.parse().expect("Error parsing purge channel id.");
    let channel_id = ChannelId(channel_id);
    let mut count_deleted = 0;

    let mut messages = channel_id.messages_iter(&ctx).boxed();

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message.attachments.len() == 0 && message_older_then_one_day(&message) {
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

fn message_older_then_one_day(msg: &Message) -> bool {
    let now = Utc::now();
    now.signed_duration_since(msg.timestamp).num_seconds() > 1
}
