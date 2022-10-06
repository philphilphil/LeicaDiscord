use anyhow::anyhow;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::prelude::Context;
type ACI = ApplicationCommandInteraction;

pub async fn run(ctx: &Context, command: &mut ACI) -> anyhow::Result<String> {
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
