use serde_derive::Deserialize;
use serde_derive::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlexServer {
    #[serde(rename = "server_id")]
    pub server_id: String,
    #[serde(rename = "shared_server")]
    pub shared_server: SharedServer,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedServer {
    #[serde(rename = "library_section_ids")]
    pub library_section_ids: String,
    #[serde(rename = "invited_email")]
    pub invited_email: String,
}
