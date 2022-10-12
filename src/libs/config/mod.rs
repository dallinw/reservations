use std::env;
use std::env::VarError;

use deadpool_postgres::{Config, Manager, ManagerConfig, Pool, RecyclingMethod, Runtime};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;
use tokio_postgres::NoTls;

pub mod api_errors;

#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub environment: String,
    pub database_host: String,
    pub database_port: i64,
    pub database_name: String,
    pub database_username: String,
    pub database_password: String,
}

pub struct AppState {
    database_pool: Pool,
    app_config: AppConfig,
}

pub async fn load_config() -> AppConfig {
    // Log configuration and bootstrap
    dotenv().ok();
    SimpleLogger::new()
        .with_utc_timestamps()
        .env()
        .init()
        .unwrap_or_default();

    match envy::from_env::<AppConfig>() {
        Ok(config) => {
            log::info!("Successfully loaded in app configuration");

            config
        }
        Err(error) => {
            log::error!("{:#?}", error);

            panic!("{:#?}", error)
        }
    }
}

pub async fn create_app_state(app_config: AppConfig) -> AppState {
    let mut cfg = Config::new();
    cfg.dbname = Some(app_config.database_name.clone());
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
    let database_pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    AppState {
        database_pool,
        app_config,
    }
}