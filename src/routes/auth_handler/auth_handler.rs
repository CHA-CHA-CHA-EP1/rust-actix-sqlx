use std::collections::HashMap;

use actix_web::{body::BoxBody, web, HttpRequest, HttpResponse, Responder };
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Signin {
    username: String,
    password: String
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignUp {
    full_name: String,
    age: u8,
    email: String,
    username: String,
    password: String,
}

impl Responder for SignUp {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let mut response: HashMap<&str, &str> = HashMap::new();
        response.insert("status", "ok");
        response.insert("message", "User created successfully");
        HttpResponse::Ok().json(response)
    }
}

pub async fn signin(signin: web::Json<Signin>) -> impl Responder {
    format!("Signin: username: {}, password: {}", signin.username, signin.password)
}

pub async fn signup(signup: web::Json<SignUp>) -> impl Responder {
    println!("SignUp: {:?}", signup);
    SignUp {
        full_name: signup.full_name.clone(),
        age: signup.age.clone(),
        email: signup.email.clone(),
        username: signup.username.clone(),
        password: signup.password.clone(),
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
