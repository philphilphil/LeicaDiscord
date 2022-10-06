use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;

// TODO: Refactor to use macro and move roles into config file.

pub fn register_location(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("role-location")
        .description("Assign (or unassign) yourself location roles.")
        .create_option(|option| {
            option
                .name("role")
                .description("The role to assign or unassign.")
                .kind(CommandOptionType::String)
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
        })
}

pub fn register_camera(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("role-camera")
        .description("Assign (or unassign) yourself camera system roles.")
        .create_option(|option| {
            option
                .name("role")
                .description("The role to assign or unassign.")
                .kind(CommandOptionType::String)
                .required(true)
                .add_string_choice("Pacific Northwest", "1012496208704569354")
                .add_string_choice("", "746651395058565120")
                .add_string_choice("Q", "746651418139820123")
                .add_string_choice("SL", "746651438721007628")
                .add_string_choice("R", "746651493108678682")
                .add_string_choice("S", "746651458979495987")
                .add_string_choice("*-Lux/X/TL", "746651530123411496")
                .add_string_choice("Sofort", "746651922278383646")
                .add_string_choice("Barnack/LTM", "887087098539163658")
        })
}
