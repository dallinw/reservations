use dotenv::dotenv;
use simple_logger::SimpleLogger;
use std::env;
use std::env::VarError;
use serde::{Deserialize, Serialize};


#[derive(Deserialize, Debug)]
pub struct AppConfig {
    pub environment: String,
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
