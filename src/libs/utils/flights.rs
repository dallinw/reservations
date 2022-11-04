//! Small implementation to insert flights into the DB
//!
//! In the real world, carrier management would have to be 100% external and independent to the
//! actual reservations. The utility is included here to help build a functional example. It
//! is leveraged in the integration tests which are used to seed a fully end to end
//! test environment.

use deadpool_postgres::Transaction;
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use tokio_postgres::Statement;

use crate::config::api_errors::ApiError;

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct Flight {
    pub id: i64,
    pub carrier: i64,
    pub first_class_seat_rows: i64,
    pub economy_seat_rows: i64,
    pub width: i64,
}

pub async fn create(
    transaction: &Transaction<'_>,
    carrier: &i64,
    first_class_seat_rows: &i64,
    economy_seat_rows: &i64,
    width: &i64,
) -> Result<Vec<Flight>, ApiError> {
    // Insert into the Carrier table
    let query = r#"
        INSERT INTO
            flights
            (
                carrier,
                first_class_seat_rows,
                economy_seat_rows,
                width
            )
        VALUES
            ($1, $2, $3, $4)
        ON CONFLICT DO NOTHING
        RETURNING
            id, carrier, first_class_seat_rows, economy_seat_rows, width;
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_transaction_result = transaction.query(&statement, &[
        carrier,
        first_class_seat_rows,
        economy_seat_rows,
        width
    ]).await;

    let rows = match raw_transaction_result {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#?}", error);

            return Err(ApiError::DatabaseError);
        }
    };

    let mut flights: Vec<Flight> = vec![];

    for row in rows {
        // Postgres forces you to make assumptions
        let id: i64 = row.get(0);
        let carrier: i64 = row.get(1);
        let first_class_seat_rows: i64 = row.get(2);
        let economy_seat_rows: i64 = row.get(3);
        let width: i64 = row.get(4);

        let flight: Flight = Flight {
            id,
            carrier,
            first_class_seat_rows,
            economy_seat_rows,
            width,
        };

        flights.push(flight);
    }

    Ok(flights)
}