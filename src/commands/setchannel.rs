use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::{
    CommandDataOption, CommandDataOptionValue,
};
use serenity::model::prelude::ChannelId;

pub fn run(options: &[CommandDataOption]) -> String {
    // TODO Store channel id
    // TODO Send waitlist msg

    let option = options
        .get(0)
        .expect("Expected attachment option")
        .resolved
        .as_ref()
        .expect("Expected attachment object");

    if let CommandDataOptionValue::Channel(channel) = option {
        if let ChannelId(name) = &channel.id {
            format!("Channel id: {:?}", name)
        } else {
            "Please provide a valid channel".to_string()
        }
    } else {
        "Please provide a valid channel".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("setchannel")
        .description("Test command for number input")
        .create_option(|option| {
            option
                .name("channel")
                .description("Channel to send alerts")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
}
