use serenity::framework::standard::{macros::command, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[aliases("50mm", "best50mmlensintheworld")]
async fn fifty_mm(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .say(
            &ctx.http,
            "LEICA 50mm f/2 SUMMICRON-M - There is no better 50mm lens on Earth, or anywhere.",
        )
        .await?;
    Ok(())
}

#[command]
#[aliases("35mm", "best35mmlensintheworld")]
async fn thirty_five_mm(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "This LEICA SUMMILUX-M 35/1.4 ASPH FLE is a landmark lens, optically and mechanically. It is the only 35mm lens one will ever need, offering practical optical perfection, and ergonomic brilliance.").await?;
    Ok(())
}
