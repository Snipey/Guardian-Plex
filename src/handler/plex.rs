use envconfig::Envconfig;
use serde_json::json;

use crate::config::Config;

pub struct PlexManager;

impl PlexManager {
    pub async fn invite_friend(server_id: &str, email: &str) {
        let config = Config::init_from_env().unwrap();
        let _res = reqwest::Client::new()
            .post(format!(
                "https://plex.tv/api/servers/{}/shared_servers",
                server_id
            ))
            .header("X-Plex-Token", config.plex_token)
            .json(&json!({
                "server_id": &server_id,
                "shared_server": {
                    "library_section_ids": "all",
                    "invited_email": email
                }
            }))
            .send()
            .await
            .unwrap();
    }
    pub fn _remove_friend() {
        // Email
        // Libraries
        // ServerID
    }
}
