use envconfig::Envconfig;
use redis::{aio::ConnectionManager, Client};
extern crate serde_json;
use crate::config::Config;

#[derive(Clone)]
pub struct RedisManager {
    pub cm: ConnectionManager,
}

impl RedisManager {
    pub async fn new() -> Self {
        let config = Config::init_from_env().unwrap();
        let client = Client::open(config.redis).unwrap();
        let cm = ConnectionManager::new(client).await.unwrap();
        println!("Connected to Redis");
        Self { cm }
    }
}
