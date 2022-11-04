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
use tokio_postgres::Statement;

use crate::config::api_errors::ApiError;

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct Carrier {
    pub id: i64,
    pub name: String,
    pub abbreviation: String,
    pub created_at: DateTime<Utc>,
}

pub async fn create(
    transaction: &Transaction<'_>,
    name: &String,
    abbreviation: &String,
) -> Result<Vec<Carrier>, ApiError> {
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
        ON CONFLICT DO NOTHING
        RETURNING
            id, name, abbreviation, created_at;
    "#;

    let statement: Statement = transaction.prepare_cached(query).await.unwrap();

    let raw_carrier = transaction.query(&statement, &[
        name,
        abbreviation,
    ]).await;

    let rows = match raw_carrier {
        Ok(value) => value,
        Err(error) => {
            log::error!("{:#?}", error);

            return Err(ApiError::DatabaseError);
        }
    };

    let mut carriers: Vec<Carrier> = vec![];

    for row in rows {
        // Postgres forces you to make assumptions
        let id: i64 = row.get(0);
        let name: String = row.get(1);
        let abbreviation: String = row.get(2);
        let created_at: DateTime<Utc> = row.get(3);

        let carrier: Carrier = Carrier {
            id,
            name,
            abbreviation,
            created_at,
        };

        carriers.push(carrier);
    }

    Ok(carriers)
}