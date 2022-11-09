use actix_web::web::Data;
use deadpool_postgres::{Client, Transaction};

use reservations_library::config::api_errors::ApiError;
use reservations_library::config::AppState;

async fn seed_database(app_state: Data<AppState>) -> Result<(), ApiError> {
    // We need to seed the database based upon the given schema
    // which means seeding the carriers, a flight, and a schedule
    // This will be 3 insert statements in 1 transaction, for a date 1 year in the future and
    // an arrival 4 hours later
    let mut client: Client = app_state.database_pool.get().await.unwrap();
    let transaction: Transaction = client.transaction().await.unwrap();

    let carrier_outcome = reservations_library::database::carriers::create(
        &transaction,
        &"DELTA".to_string(),
        &"DA".to_string(),
    ).await;

    match carrier_outcome {
        Ok(value) => {
            log::debug!("{:#?}", value);
        }
        Err(error) => {
            log::error!("{:#?}", error);
        }
    }

    let flight_outcome = reservations_library::database::flights::create(
        &transaction,
        &"DA".to_string(),
        &3,
        &6,
        &4,
    ).await;

    match flight_outcome {
        Ok(value) => {
            log::debug!("{:#?}", value);
        }
        Err(error) => {
            log::error!("{:#?}", error);
        }
    }

    let commit_result = transaction.commit().await;

    match commit_result {
        Ok(_) => {
            log::debug!("successful commit");
            Ok(())
        }
        Err(error) => {
            log::error!("{:#?}", error);
            return Err(ApiError::DatabaseError);
        }
    }
}

#[cfg(test)]
mod tests {
    use reservations_library::config::{AppState, create_app_state};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn database_seeded() {
        let app_state: Data<AppState> = Data::new(create_app_state().await);

        let seed_outcome = seed_database(app_state).await;
    }
}