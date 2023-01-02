use envconfig::Envconfig;

#[derive(Envconfig)]
pub struct Config {
    #[envconfig(
        from = "DB_URL",
        default = "postgresql://postgres:db@localhost:5432/api"
    )]
    pub db_url: String,

    #[envconfig(from = "REDIS_URL", default = "redis://127.0.0.1:6379")]
    pub redis: String,

    #[envconfig(from = "DISCORD_TOKEN")]
    pub token: String,

    #[envconfig(from = "PLEX_TOKEN")]
    pub plex_token: String,

    #[envconfig(from = "GUILD_ID")]
    pub guild_id: String,
}
