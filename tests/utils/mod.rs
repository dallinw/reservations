use deadpool_postgres::{Client, Transaction};

use crate::config::AppState;

async fn seed_database(app_state: AppState) {
    let mut client: Client = app_state.database_pool.get().await.unwrap();
    let transaction: Transaction = client.transaction().await.unwrap();
}