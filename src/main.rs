use std::error;
use actix_cors::Cors;
use actix_web::{App, get, HttpRequest, HttpResponse, HttpServer, middleware, web};
use reservations_library::config::{load_config, AppConfig};
use reservations_library::endpoints::health;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn error::Error>> {
    let app_config: AppConfig = load_config().await;
    let app_state: web::Data<AppConfig> = web::Data::new(app_config);

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
