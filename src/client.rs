use crate::{
    config::Config,
    db::{postgres::PostgresManager, redis::RedisManager},
    events::Handler,
};

use envconfig::Envconfig;
use serenity::{prelude::GatewayIntents, Client};
use std::env;
pub struct NullClient {
    pub redis: RedisManager,
    pub postgres: PostgresManager,
}

impl NullClient {
    // Start the Client
    pub async fn start() -> Self {
        // Init Redis
        let redis = RedisManager::new().await;
        // Init Postgres
        let postgres = PostgresManager::new().await;
        let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
        let _config = Config::init_from_env().unwrap();
        let mut client = Client::builder(token, GatewayIntents::empty())
            .event_handler(Handler)
            .await
            .expect("Error creating client");

        if let Err(why) = client.start().await {
            println!("Client error: {:?}", why);
        }

        Self { redis, postgres }
    }
}
