use crate::commands::slash_role;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{Interaction, InteractionResponseType},
    },
    prelude::*,
};
use std::env;
use tracing::error;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        slash_role::create_command(&guild_id, &ctx)
            .await
            .expect("Can't create role slash command.");
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let response_message;

            match command.data.name.as_str() {
                "role" => match slash_role::handle(&ctx, &mut command).await {
                    Ok(response) => {
                        response_message = response;
                    }
                    Err(why) => {
                        error!("Error handling role command: {}", why);
                        response_message = "Something went wrong.".to_string();
                    }
                },
                _ => panic!("Command other then \"role\" used, this is not possible."),
            }

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(response_message))
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }
}
