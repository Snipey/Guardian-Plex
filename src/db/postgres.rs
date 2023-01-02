// use migration::{Migrator, MigratorTrait};

use envconfig::Envconfig;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::time::Duration;

use crate::config::Config;
pub struct PostgresManager {
    pub pm: DatabaseConnection,
}

impl PostgresManager {
    pub async fn new() -> Self {
        let config = Config::init_from_env().unwrap();

        let mut opt = ConnectOptions::new(config.db_url);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let db = Database::connect(opt).await;
        println!("Database Connected");
        let mig = &db.unwrap().clone();
        Migrator::up(&mig, None).await.unwrap();
        println!("Database Migration Complete!");
        Self { pm: mig.clone() }
    }
}
