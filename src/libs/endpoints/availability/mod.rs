use actix_web::{get, HttpResponse, web};
use actix_web::HttpRequest;

use crate::config::api_errors::ApiError;

#[get("/availability")]
pub async fn handle_get(request: HttpRequest) -> Result<HttpResponse, ApiError> {
    log::debug!("Health check ping");
    log::debug!("{:#?}", request);

    Ok(HttpResponse::Ok().json("ok"))
}