use std::{env, ops::Add};

use serenity::{
    async_trait,
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::{
        gateway::Ready,
        id::{CommandId, GuildId},
        interactions::{
            application_command::{
                ApplicationCommand, ApplicationCommandInteractionDataOptionValue,
                ApplicationCommandOptionType,
            },
            Interaction, InteractionResponseType,
        },
    },
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "role" => "Hey, I'm alive!".to_string(),
                _ => "not implemented :(".to_string(),
            };

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

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands.create_application_command(|command| {
                command
                    .name("role")
                    .description("Assign (or unassign) yourself camera system roles. If you had the role it will be unassigned.")
                    .create_option(|option| {
                        option
                            .name("role")
                            .description("The role to assign or unassign.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(true)
                            .add_string_choice("M Digital", "M-Digi")
                            .add_string_choice("M Film", "M-Film")
                            .add_string_choice("Q", "Q")
                            .add_string_choice("SL", "SL")
                            .add_string_choice("R", "R")
                            .add_string_choice("S", "S")
                            .add_string_choice("*-Lux/X/TL", "*-Lux/X/TL")
                            .add_string_choice("Sofort", "Sofort")
                            .add_string_choice("Barnack/LTM", "Barnack")
                    })
                    .create_option(|option| {
                        option
                            .name("role2")
                            .description("The role to assign or unassign.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(false)
                            .add_string_choice("M Digital", "M-Digi")
                            .add_string_choice("M Film", "M-Film")
                            .add_string_choice("Q", "Q")
                            .add_string_choice("SL", "SL")
                            .add_string_choice("R", "R")
                            .add_string_choice("S", "S")
                            .add_string_choice("*-Lux/X/TL", "*-Lux/X/TL")
                            .add_string_choice("Sofort", "Sofort")
                            .add_string_choice("Barnack/LTM", "Barnack")
                    })
                    .create_option(|option| {
                        option
                            .name("role3")
                            .description("The role to assign or unassign.")
                            .kind(ApplicationCommandOptionType::String)
                            .required(false)
                            .add_string_choice("M Digital", "M-Digi")
                            .add_string_choice("M Film", "M-Film")
                            .add_string_choice("Q", "Q")
                            .add_string_choice("SL", "SL")
                            .add_string_choice("R", "R")
                            .add_string_choice("S", "S")
                            .add_string_choice("*-Lux/X/TL", "*-Lux/X/TL")
                            .add_string_choice("Sofort", "Sofort")
                            .add_string_choice("Barnack/LTM", "Barnack")
                    })
            })
        })
        .await;
    }
}

#[tokio::main]
async fn main() {
    dotenv::from_filename("./.env").expect("Failed to load .env file");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let application_id: u64 = env::var("APPLICATION_ID")
        .expect("Expected an application id in the environment")
        .parse()
        .expect("application id is not a valid id");

    let mut client = Client::builder(token)
        .event_handler(Handler)
        .application_id(application_id)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
