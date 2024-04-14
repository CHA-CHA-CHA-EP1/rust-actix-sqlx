use actix_web::{web, Responder, HttpResponse };
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Signin {
    username: String,
    password: String
}

#[derive(Deserialize, Serialize)]
pub struct SignUp {
    full_name: String,
    age: u8,
    email: String,
    username: String,
    password: String,
}

pub async fn signin(signin: web::Json<Signin>) -> impl Responder {
    format!("Signin: username: {}, password: {}", signin.username, signin.password)
}

pub async fn signup(signup: web::Json<SignUp>) -> impl Responder {
    format!("Signup: full_name: {}, age: {}, email: {}, username: {}, password: {}", signup.full_name, signup.age, signup.email, signup.username, signup.password)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/auth");
    cfg.service(
        scope
            .route("/signin", web::post().to(signin))
            .route("/signup", web::post().to(signup))
    );
}
