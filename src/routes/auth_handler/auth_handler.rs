use std::collections::HashMap;

use actix_web::{body::BoxBody, web, HttpRequest, HttpResponse, Responder };
use crate::{domain::user, services::user_service::UserService};

impl Responder for user::UserSignup {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let mut response: HashMap<&str, &str> = HashMap::new();
        response.insert("status", "ok");
        response.insert("message", "User created successfully");
        HttpResponse::Ok().json(response)
    }
}

pub async fn signin(signin: web::Json<user::Signin>) -> impl Responder {
    format!("Signin: username: {}, password: {}", signin.username, signin.password)
}

pub async fn signup(signup: web::Json<user::UserSignup>, service: web::Data<crate::domain::user::AppState>) -> impl Responder {
    let user_signup = signup.into_inner();
    let result = service.user_service.create_user(user_signup).await;
    match result {
        Ok(_) => HttpResponse::Ok().json("User created successfully"),
        Err(_) => HttpResponse::Ok().json("Failed to create user")
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/auth");
    cfg.service(
        scope
            .route("/signin", web::post().to(signin))
            .route("/signup", web::post().to(signup))
    );
}
