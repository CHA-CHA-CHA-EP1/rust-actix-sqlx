use actix_web::{web, App, HttpServer};

use rust_actix_sqlx::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    println!("Listening on: 0.0.0.0:8080");

    HttpServer::new(move || {
        App::new()
            .route("/health-check", actix_web::web::get().to(routes::health_check::health_check))
            .service(
                web::scope("/user")
                    .configure(routes::user_handler::user_handler::config)
            )
            .service(
                web::scope("/auth")
                    .configure(routes::auth_handler::auth_handler::config)
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
