pub mod client;
pub mod commands;
pub mod config;
pub mod db;
pub mod events;
pub mod handler;
use client::NullClient;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    // Get env from .env if available
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    NullClient::start().await;
}
