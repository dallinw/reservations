use deadpool_postgres::{Client, Transaction};
use reservations_library::config::AppState;
use tokio_postgres::{Error, Statement};

async fn seed_carrier(
    transaction: &deadpool_postgres::Transaction<'_>,
    abbreviation: String,
) {}

async fn seed_database(app_state: AppState) {
    // We need to seed the database based upon the given schema
    // which means seeding the carriers, a flight, and a schedule
    // This will be 3 insert statements in 1 transaction, for a date 1 year in the future and
    // an arrival 4 hours later
    let mut client: Client = app_state.database_pool.get().await.unwrap();
    let transaction: Transaction = client.transaction().await.unwrap();

    // Insert into the Carrier table
    let query = r#"
        INSERT INTO
            carriers
            (
                id,
                name,
                abbreviation,
            )
        VALUES
            ($1, $2, $3)
        ON CONFLICT DO NOTHING
        RETURNING
            id, name, abbreviation, created_at
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_result = transaction.query(&statement, &[
        id,
        strategy,
        user_address,
        transaction_type
    ]).await;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 6);
    }
}