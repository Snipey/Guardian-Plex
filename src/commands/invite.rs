use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOption;

use crate::handler::plex::PlexManager;

pub fn run(_options: &[CommandDataOption]) -> String {
    "Please provide a valid attachment".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("invite")
        .description("Invite a user to your Plex Media Server")
        .create_option(|option| {
            option
                .name("user")
                .description("User to invite")
                .kind(CommandOptionType::User)
                .required(true)
        })
}

pub async fn _invite(email: &str, server_id: &str) {
    PlexManager::invite_friend(server_id, email).await;
}
