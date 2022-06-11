use anyhow::anyhow;
use serenity::{
    model::{
        id::GuildId,
        interactions::application_command::{
            ApplicationCommand, ApplicationCommandInteraction, ApplicationCommandOptionType,
        },
    },
    prelude::*,
};

type Aci = ApplicationCommandInteraction;

pub async fn handle(ctx: &Context, command: &mut Aci) -> anyhow::Result<String> {
    let user_roles = &command.member.as_ref().unwrap().roles.to_owned();
    let mut response = String::new();

    for option in &command.data.options {
        let role_id: u64 = get_role_int(option.value.as_ref());

        let member = command
            .member
            .as_mut()
            .ok_or_else(|| anyhow!("Can't load member."))?;

        if user_roles.iter().any(|r| r.0 == role_id) {
            member.remove_role(&ctx.http, role_id).await?;
            response = "Removed role.".to_string();
        } else {
            member.add_role(&ctx.http, role_id).await?;
            response = "Assigned role.".to_string();
        }
    }
    Ok(response)
}

fn get_role_int(value: Option<&serenity::json::Value>) -> u64 {
    let option_str = value
        .ok_or_else(|| anyhow!("Can't get option."))
        .expect("cant parse option")
        .as_str()
        .ok_or_else(|| anyhow!("Can't parse option."))
        .expect("Can't parse option.");

    let role_id: u64 = option_str.parse().expect("Can't parse option to int.");
    role_id
}

// Creates the slash command to self assign some roles
pub async fn create_command(
    guild_id: &GuildId,
    ctx: &Context,
) -> anyhow::Result<Vec<ApplicationCommand>> {
    let cmds = GuildId::set_application_commands(guild_id, &ctx.http, |commands| {
        commands.create_application_command(|command| {
            command
                .name("role")
                .description("Assign (or unassign) yourself camera system roles.")
                .create_option(|option| {
                    option
                        .name("role")
                        .description("The role to assign or unassign.")
                        .kind(ApplicationCommandOptionType::String)
                        .required(true)
                        .add_string_choice("M Digital", "746651139449159720")
                        .add_string_choice("M Film", "746651395058565120")
                        .add_string_choice("Q", "746651418139820123")
                        .add_string_choice("SL", "746651438721007628")
                        .add_string_choice("R", "746651493108678682")
                        .add_string_choice("S", "746651458979495987")
                        .add_string_choice("*-Lux/X/TL", "746651530123411496")
                        .add_string_choice("Sofort", "746651922278383646")
                        .add_string_choice("Barnack/LTM", "887087098539163658")
                    //TODO: find way to not hardcode the role and ids
                })
        })
    })
    .await
    .expect("Issue creating the role cmd");
    Ok(cmds)
}
