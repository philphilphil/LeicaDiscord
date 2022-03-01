use chrono::Utc;
use serenity::{
    framework::standard::{
        macros::{command},
        CommandResult,
    },
    model::{channel::Message, id::ChannelId},
    prelude::*, futures::StreamExt,
};
use tracing::{error};

#[command]
async fn purge(ctx: &Context, msg: &Message) -> CommandResult {
    let purge_count = purge_channel(ChannelId(810536807761707121), ctx).await;
    let delete_count_msg = format!("Deleted {} messages in channel {}", purge_count, "");
    msg.channel_id.say(&ctx.http, delete_count_msg).await?;
    Ok(())
}

async fn purge_channel(channel_id: ChannelId, ctx: &Context) -> u64 {
    let mut count_deleted = 0;
    let mut messages = channel_id.messages_iter(&ctx).boxed();

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                if message.attachments.len() == 0 && message_older_then_one_day(&message) {
                    match message.delete(&ctx.http).await {
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
    now.signed_duration_since(msg.timestamp).num_seconds() > 86400
}
