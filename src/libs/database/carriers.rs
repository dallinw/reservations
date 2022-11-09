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