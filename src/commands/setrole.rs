use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Set the role to".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("setrole")
        .description("Test command for number input")
        .create_option(|option| {
            option
                .name("int")
                .description("An integer from 5 to 10")
                .kind(CommandOptionType::Role)
                .required(true)
        })
}
