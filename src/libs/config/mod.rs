use dotenv::dotenv;
use simple_logger::SimpleLogger;
use std::env;

#[derive(Clone)]
pub struct AppConfig {
    // pub log_level: String,
    // pub environment: String,
}

pub async fn load_config() -> AppConfig {
    // Log configuration and bootstrap
    dotenv().ok();
    SimpleLogger::new().with_utc_timestamps().env().init().unwrap_or_default();

    let log_level = env::var("RUST_LOG");

    AppConfig {
        // log_level: "DEBUG",
        // environment: "development",
    }
}
