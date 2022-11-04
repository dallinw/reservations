use actix_web::{get, HttpResponse};
use actix_web::HttpRequest;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use postgres_types::{FromSql, ToSql};

use crate::config::api_errors::ApiError;

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct Route {
    pub source: String,
    pub destination: String,
}

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct RequestBody {
    pub route: Route,
    pub carrier: String,
    pub departure_timestamp: DateTime<Utc>,
}

/// Anonymous requests allows to view and fetch seats for a given request body
#[get("/seats")]
pub async fn handle_get(request: HttpRequest) -> Result<HttpResponse, ApiError> {
    log::debug!("Getting available seats with request");
    log::debug!("{:#?}", request);

    Ok(HttpResponse::Ok().json("ok"))
}