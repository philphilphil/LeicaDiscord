use serenity::{
    model::{
        id::GuildId,
        interactions::application_command::{
            ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
        },
    },
    prelude::*,
};
use tokio::time::error::Error;

type ACI = ApplicationCommandInteraction;

pub async fn handle(ctx: &Context, command: &mut ACI) -> Result<(), Error> {
    let user_roles = &command.member.as_ref().unwrap().roles.to_owned();

    for option in &command.data.options {
        let option_str = option.value.as_ref().unwrap().as_str();
        let role_id: u64 = option_str.unwrap().parse().expect("Can't parse role id.");
        let member = command.member.as_mut().unwrap();

        if user_roles.iter().any(|r| r.0 == role_id) {
            member.remove_role(&ctx.http, role_id).await.unwrap();
        } else {
            member.add_role(&ctx.http, role_id).await.unwrap();
        }
    }
    Ok(())
}

// Creates the slash command to self assign some roles
pub async fn create_command(
    guild_id: &GuildId,
    ctx: &Context,
) -> Result<Vec<ApplicationCommand>, Error> {
    let cmds = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        // TODO: Move role setup to a config and remove repeated code of add_string_choices for all 3 options
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
                        .add_string_choice("M Digital", "835591825917870081")
                        .add_string_choice("M Film", "835591825925603368")
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
                        .add_string_choice("M Digital", "835591825917870081")
                        .add_string_choice("M Film", "835591825925603368")
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
                        .add_string_choice("M Digital", "835591825917870081")
                        .add_string_choice("M Film", "835591825925603368")
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
    Ok(cmds.unwrap())
}
