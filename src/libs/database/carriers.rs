//! Small implementation to insert carriers into the DB
//!
//! In the real world, carrier management would have to be 100% external and independent to the
//! actual reservations. The utility is included here to help build a functional example. It
//! is leveraged in the integration tests which are used to seed a fully end to end
//! test environment.

use chrono::{DateTime, Utc};
use deadpool_postgres::Transaction;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Row, Statement};

use crate::config::api_errors::ApiError;

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct Carrier {
    pub name: String,
    pub abbreviation: String,
    pub created_at: DateTime<Utc>,
}

fn parse_results(rows: Vec<Row>) -> Vec<Carrier> {
    let mut carriers: Vec<Carrier> = vec![];

    for row in rows {
        // Postgres forces you to make assumptions
        let name: String = row.get(0);
        let abbreviation: String = row.get(1);
        let created_at: DateTime<Utc> = row.get(2);

        let carrier: Carrier = Carrier {
            name,
            abbreviation,
            created_at,
        };

        carriers.push(carrier);
    }

    carriers
}

pub async fn create(
    transaction: &Transaction<'_>,
    name: &String,
    abbreviation: &String,
) -> Result<Vec<Carrier>, tokio_postgres::Error> {
    // Insert into the Carrier table
    let query = r#"
        INSERT INTO
            carriers
            (
                name,
                abbreviation
            )
        VALUES
            ($1, $2)
        RETURNING
            name, abbreviation, created_at;
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_carrier_query = transaction.query(&statement, &[
        name,
        abbreviation,
    ]).await;

    let rows = match raw_carrier_query {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#?}", error);

            return Err(error);
        }
    };

    let carriers: Vec<Carrier> = parse_results(rows);

    Ok(carriers)
}

pub async fn find(
    transaction: &Transaction<'_>,
    abbreviation: &String,
) -> Result<Vec<Carrier>, tokio_postgres::Error> {
    // Insert into the Carrier table
    let query = r#"
        SELECT
            *
        FROM
            carriers
        WHERE
            carriers.abbreviation = ($1)
        RETURNING
            name,
            abbreviation,
            created_at;
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_carrier_query = transaction.query(&statement, &[
        abbreviation,
    ]).await;

    let rows = match raw_carrier_query {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#?}", error);

            return Err(error);
        }
    };

    let carriers: Vec<Carrier> = parse_results(rows);

    Ok(carriers)
}

pub async fn delete(
    transaction: &Transaction<'_>,
    abbreviation: &String,
) -> Result<Vec<Carrier>, tokio_postgres::Error> {
    // Insert into the Carrier table
    let query = r#"
        DELETE
        FROM
            carriers
        WHERE
            carriers.abbreviation = ($1);
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_carrier_query = transaction.query(&statement, &[
        abbreviation,
    ]).await;

    let rows = match raw_carrier_query {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#?}", error);

            return Err(error);
        }
    };

    let carriers: Vec<Carrier> = parse_results(rows);

    Ok(carriers)
}

#[cfg(test)]
mod tests {
    use actix_web::web::Data;
    use deadpool_postgres::Client;
    use rand::distributions::Alphanumeric;
    use rand::Rng;

    use crate::config::{AppState, create_app_state};

    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[tokio::test]
    async fn test_create() {
        let app_state: Data<AppState> = Data::new(create_app_state().await);
        let mut client: Client = app_state.database_pool.get().await.unwrap();
        let transaction: Transaction = client.transaction().await.unwrap();

        let random_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();

        let outcome = create(
            &transaction,
            &format!("test_{}", random_name),
            &format!("test_{}", random_name),
        ).await;

        // Commits only needed for integration tests when we must persist data
        // let _commit = transaction.commit().await;

        match outcome {
            Ok(_) => {
                log::debug!("Passed create, but should have failed!");
            }
            Err(error) => {
                log::info!("{:#?}", error);
                panic!("Test successful");
            }
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_create_fail() {
        let app_state: Data<AppState> = Data::new(create_app_state().await);
        let mut client: Client = app_state.database_pool.get().await.unwrap();
        let transaction: Transaction = client.transaction().await.unwrap();

        let outcome = create(
            &transaction,
            &"DELTA".to_string(),
            &"DA".to_string(),
        ).await;

        // Commits only needed for integration tests when we must persist data
        // let _commit = transaction.commit().await;

        match outcome {
            Ok(_) => {
                log::debug!("Passed create, but should have failed!");
            }
            Err(error) => {
                log::info!("{:#?}", error);
                panic!("Test successful");
            }
        }
    }

    #[tokio::test]
    async fn test_delete() {
        let app_state: Data<AppState> = Data::new(create_app_state().await);
        let mut client: Client = app_state.database_pool.get().await.unwrap();
        let transaction: Transaction = client.transaction().await.unwrap();

        let random_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(48)
            .map(char::from)
            .collect();

        let fullname = format!("test_{}", random_name);

        let outcome = create(
            &transaction,
            &fullname,
            &fullname,
        ).await;

        let outcome = delete(
            &transaction,
            &fullname,
        ).await;

        // Commits only needed for integration tests when we must persist data
        // let _commit = transaction.commit().await;

        match outcome {
            Ok(_) => {
                log::debug!("Passed create, but should have failed!");
            }
            Err(error) => {
                log::info!("{:#?}", error);
            }
        }
    }

    #[tokio::test]
    #[should_panic]
    async fn test_delete_should_fail() {
        let app_state: Data<AppState> = Data::new(create_app_state().await);
        let mut client: Client = app_state.database_pool.get().await.unwrap();
        let transaction: Transaction = client.transaction().await.unwrap();

        let outcome = delete(
            &transaction,
            &"DA".to_string(),
        ).await;


        // Commits only needed for integration tests when we must persist data
        // let _commit = transaction.commit().await;

        match outcome {
            Ok(_) => {
                log::debug!("Passed create, but should have failed!");
            }
            Err(error) => {
                log::info!("{:#?}", error);
                panic!("Passed test that should fail");
            }
        }
    }
}