use crate::commands::{command_register, command_service};
use serenity::{
    async_trait,
    model::{application::interaction::*, gateway::Ready, id::GuildId},
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

        let test = GuildId::roles(guild_id, &ctx.http);

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| command_register::register_camera(command))
                .create_application_command(|command| command_register::register_location(command))
        })
        .await;

        println!(
            "I now have the following guild slash commands: {:#?}",
            commands
        );
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(mut command) = interaction {
            let response_message = match command.data.name.as_str() {
                "role-camera" => match command_service::run(&ctx, &mut command).await {
                    Ok(response) => response,
                    Err(why) => {
                        error!("Error handling role command: {}", why);
                        "Something went wrong.".to_string()
                    }
                },
                "role-location" => match command_service::run(&ctx, &mut command).await {
                    Ok(response) => response,
                    Err(why) => {
                        error!("Error handling role command: {}", why);
                        "Something went wrong.".to_string()
                    }
                },
                _ => panic!("Unsupported role command used, this is not possible."),
            };

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
