use serenity::builder::CreateApplicationCommand;
use serenity::model::application::command::CommandOptionType;

// TODO: Refactor to use macro and move roles into config file or get from api.

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
                .add_string_choice("Pacific Northwest", "1012496208704569354")
                .add_string_choice("Puget Sound", "1012496511722070156")
                .add_string_choice("San Francisco Bay Area", "1012496749283246101")
                .add_string_choice("Los Angeles", "1012496993043628042")
                .add_string_choice("California", "1012497318374822019")
                .add_string_choice("Rockies", "1012497517491015772")
                .add_string_choice("Southwestern US", "1012497670000091146")
                .add_string_choice("Midwestern US", "1012497883553091614")
                .add_string_choice("Southeastern US", "1012498132883484672")
                .add_string_choice("Northeastern US", "1012498687617945610")
                .add_string_choice("Texas", "1012498469908381776")
                .add_string_choice("Boston", "1012499068859199559")
                .add_string_choice("New York City", "1012499469134217328")
                .add_string_choice("Western Canada", "1012499955174355005")
                .add_string_choice("Eastern Canada", "1012500135214841857")
                .add_string_choice("Motherland", "1012500529609461760")
                .add_string_choice("Berlin", "1012500693564801146")
                .add_string_choice("United Kingdom", "1012500827858014349")
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
