//! # Airplane Seat Reservation Microservice Demo
//!
//!This crate and service provides a real-world, or as close to as possible, example as possible of
//! what a Rust microservice could look like. Within the app framework is a demonstration of
//! using a database for a persistent state store, cache for speed and low-latency actions, along
//! with a suite of quality of life features, tests, pipeline demos etc.
//!
//! For deployment it is built to showcase bleeding edge deployment strategies, on Kubernetes
//! and leveraging Kubernetes features to further enhance the deployment.
//!
//! For example into the Kubernetes deployment environment feel free to reference the [Platform Deployment Helm Chart](https://github.com/dallinw/platform-deployment)

use std::error;

use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware, web};

use reservations_library::config::{AppConfig, AppState, create_app_state, load_config};
use reservations_library::endpoints::health;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app_config: AppConfig = load_config().await;
    let created_app_state: AppState = create_app_state(app_config).await;
    let app_state: web::Data<AppState> = web::Data::new(created_app_state);

    HttpServer::new(move || {
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .wrap(middleware::Compress::default())
            .app_data(app_state.clone())

            .service(health::handle_get)
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await?;

    Ok(())
}
