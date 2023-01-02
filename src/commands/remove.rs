use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Removed user".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("remove")
        .description("Remove a user from plex")
        .create_option(|option| {
            option
                .name("user")
                .description("User to remove")
                .kind(CommandOptionType::User)
                .required(true)
        })
}
