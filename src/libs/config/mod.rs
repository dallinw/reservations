use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime, Transaction};
use dotenv::dotenv;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use tokio_postgres::{NoTls, Statement};

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
    pub database_pool: Pool,
    pub app_config: AppConfig,
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
    let mut cfg: Config = Config::new();
    cfg.dbname = Some(app_config.database_name.clone());
    cfg.host = Some(app_config.database_host.clone());
    cfg.user = Some(app_config.database_username.clone());
    cfg.password = Some(app_config.database_password.clone());
    cfg.port = Some(app_config.database_port.clone() as u16);
    cfg.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });

    let database_pool: Pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();

    // Test connection
    let mut client = database_pool.get().await.unwrap();
    let transaction: Transaction = client.transaction().await.unwrap();
    let query = "SELECT 1";

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_result = transaction.query(&statement, &[]).await;
    match raw_result {
        Ok(value) => {
            log::debug!("{:#?}", value);
            log::info!("Successfully verified connection to database");
        }
        Err(error) => {
            log::error!("{:#?}", error);

            panic!("Cannot connect to DB");
        }
    };

    AppState {
        database_pool,
        app_config,
    }
}