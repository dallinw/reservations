use actix_web::{get, HttpResponse, web};
use actix_web::HttpRequest;
use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};

use crate::config::api_errors::ApiError;
use crate::config::AppState;

#[derive(Deserialize, Serialize, Debug, Clone, ToSql, FromSql)]
pub struct RequestBody {
    pub flight: String,
    pub departure: DateTime<Utc>,
}

/// Anonymous requests allows to view and fetch seats for a given request body
#[get("/seats")]
pub async fn handle_get(
    http_request: HttpRequest,
    app_state: web::Data<AppState>,
    body: web::Json<RequestBody>,
) -> Result<HttpResponse, ApiError> {
    log::debug!("Getting available seats with request");
    log::debug!("{:#?}", http_request);
    log::debug!("{:#?}", body);

    Ok(HttpResponse::Ok().json("ok"))
}