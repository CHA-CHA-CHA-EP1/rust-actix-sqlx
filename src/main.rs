use std::sync::Arc;

use actix_web::{web, App, HttpServer};

use rust_actix_sqlx::routes;
use rust_actix_sqlx::services;
use rust_actix_sqlx::repositories;
use rust_actix_sqlx::domain;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Listening on: 0.0.0.0:8080");

    let user_repository = repositories::user_repository::UserRepositoryImpl::new();
    let user_service = services::user_service::UserServiceImpl::new(
        Arc::new(user_repository)
    );

    HttpServer::new(move || {
        App::new()
            .route("/health-check", actix_web::web::get().to(routes::health_check::health_check))
            .configure(routes::user_handler::user_handler::config)
            .configure(routes::auth_handler::auth_handler::config)
    })

    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
