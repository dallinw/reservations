use std::error;

use reservations_library::config::{load_config, AppConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app_config: AppConfig = load_config().await;

    Ok(())
}
