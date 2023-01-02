use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Plex Token Set!".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("setup")
        .description("Setup Plex Details")
        .create_option(|option| {
            option
                .name("token")
                .description("Your Plex Auth Token")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
