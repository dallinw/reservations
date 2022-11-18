use chrono::{DateTime, Utc};
use deadpool_postgres::Transaction;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::{Row, Statement};

use crate::config::api_errors::ApiError;

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct Schedule {
    pub id: i64,
    pub departure: DateTime<Utc>,
    pub arrival: DateTime<Utc>,
    pub source: String,
    pub destination: String,
    pub flight: i64,
    pub carrier: String,
}

pub async fn create(
    transaction: &Transaction<'_>,
    name: &String,
    abbreviation: &String,
) -> Result<Vec<Schedule>, tokio_postgres::Error> {
    // Insert into the Carrier table
    let query = r#"
        INSERT INTO
            schedules
            (
                departure,
                arrival,
                source,
                destination,
                flight,
                carrier
            )
        VALUES
            ($1, $2, $3, $4, $5, $6)
        RETURNING
            id,
            departure,
            arrival,
            source,
            destination,
            flight,
            carrier
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_carrier_query = transaction.query(&statement, &[
        departure,
        arrival,
        source,
        destination,
        flight,
        carrier
    ]).await;

    let rows = match raw_carrier_query {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#?}", error);

            return Err(error);
        }
    };

    let carriers: Vec<Schedule> = parse_results(rows);

    Ok(carriers)
}