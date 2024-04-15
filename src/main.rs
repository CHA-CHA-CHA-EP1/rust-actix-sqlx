use std::sync::Arc;

use actix_web::{web, App, HttpServer};
use rust_actix_sqlx::domain::user::AppState;
use rust_actix_sqlx::repositories::user_repository::UserRepository;
use sqlx::{Postgres, Pool, postgres::PgPoolOptions};

use rust_actix_sqlx::routes;
use rust_actix_sqlx::services;
use rust_actix_sqlx::repositories;
use rust_actix_sqlx::middlewares;

use dotenv::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    dotenv().ok();
    env_logger::init();

    // inital database connection
    let database_url: String = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // write log for database connecting.
    println!("Database connecting...");

    let pool: Pool<Postgres> = match PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url).await {
        Ok(pool) => pool,
        Err(e) => {
            println!("Failed to connect to database: {}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "Failed to connect to database"));
        }
    };

    // write log for database connected.
    println!("Database connected.");

    let user_repository = repositories::user_repository::UserRepositoryImpl::new(
        Arc::new(pool)
    );

    user_repository.test_user().await;

    let user_service = services::user_service::UserServiceImpl::new(
        Arc::new(user_repository)
    );

    println!("Listening on: 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(
                web::Data::new(
                    AppState {
                        user_service: user_service.clone()
                    }
                )
            )
            .wrap(middlewares::auth_middleware::Authentication)
            .route("/health-check", actix_web::web::get().to(routes::health_check::health_check))
            .configure(routes::auth_handler::auth_handler::config)
            .service(
                web::scope("/user")
                    .configure(routes::user_handler::user_handler::config)
            )
    })

    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
