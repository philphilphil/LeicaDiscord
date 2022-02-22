use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::id::{ChannelId, MessageId};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn purge(ctx: &Context, msg: &Message) -> CommandResult {
    //msg.channel_id.say(&ctx.http, "Pong!").await?;

    // you can then pass it into a function which retrieves messages:
    let channel_id = ChannelId(810536807761707121);
    let _messages = channel_id
        .messages(&ctx.http, |retriever| {
            retriever.before(MessageId(945612625356685353)).limit(5)
        })
        .await?;

    for m in _messages {
        if m.attachments.len() == 0 {
            m.delete(&ctx.http).await.unwrap();
        }

        println!("{}", m.content);
    }
    Ok(())
}
