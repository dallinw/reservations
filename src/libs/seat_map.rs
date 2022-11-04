//! Query from DB
//! We are going to do a few different things here.
//! With the Flight number we need to do a couple things.
//!
//! Split the flight number into abbreviation and digits
//!
//! Lookup carrier name in carrier table based on: abbreviation
//! Lookup airplane metadata in airplane table based on: airplane number + carrier abbreviation composite key
//! Lookup schedule metadata in schedule table based on: departure timestamp + flight id + carrier abbreviation
//! Lookup all reservations in reservation table base on: schedule id
//!
//! Build in memory the reservation seat map
//! Return to the seat map
//! "1": {
//!     "a": Reserved
//!     "b": Available
//!     "c": Available
//!     "d": Available
//! },
//! ...
//! and so on based upon the flight

use chrono::{DateTime, Utc};

use crate::config::api_errors::ApiError;

pub async fn fetch(
    flight: String,
    departure: DateTime<Utc>,
) -> Result<(), ApiError> {
    Ok(())
}