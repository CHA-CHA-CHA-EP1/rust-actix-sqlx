use std::collections::HashMap;

use actix_web::{body::BoxBody, cookie::Cookie, web, HttpRequest, HttpResponse, Responder };
use crate::{domain::user::{self, SigninResponse, SigninResponseWithToken}, services::user_service::UserService};

impl Responder for user::UserSignup {
    type Body = BoxBody;

    fn respond_to(self, _req: &HttpRequest) -> HttpResponse {
        let mut response: HashMap<&str, &str> = HashMap::new();
        response.insert("status", "ok");
        response.insert("message", "User created successfully");
        HttpResponse::Ok().json(response)
    }
}

pub async fn signin(
    signin: web::Json<user::Signin>,
    service: web::Data<crate::domain::user::AppState>
) -> impl Responder {
    let signin = signin.into_inner();
    let result = service.user_service.signin(signin).await;
    match result {
        Ok(res) => {
            let cookie = Cookie::build("refresh_token", &res.refresh_token)
                .http_only(true)
                .secure(true)
                // .same_site(actix_web::cookie::SameSite::Strict)
                .finish();

            HttpResponse::Ok().cookie(cookie).json(
                SigninResponseWithToken {
                    access_token: res.access_token,
                    refresh_token: "".to_string(),
                }
        )},
        Err(_) => HttpResponse::Ok().json("Failed to sign in")
    }
}

pub async fn signup(signup: web::Json<user::UserSignup>, service: web::Data<crate::domain::user::AppState>) -> impl Responder {
    let user_signup = signup.into_inner();
    let result = service.user_service.create_user(user_signup).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("User created successfully"),
        Err(_) => HttpResponse::Ok().json("Failed to create user")
    }
}

pub async fn refresh_token_with_httponly_cookie(
    req: HttpRequest,
    service: web::Data<crate::domain::user::AppState>
) -> impl Responder {
    let refresh_token = req.cookie("refresh_token");
    println!("[refresh_token] - {:?}", refresh_token);
    let refresh_token = match refresh_token {
        Some(token) => token.value().to_string(),
        None => {
            return HttpResponse::Ok().json("Refresh token not found");
        }
    };

    let res = service.user_service.refresh_token(refresh_token).await;
    match res {
        Ok(res) => {
            let cookie = Cookie::build("refresh_token", &res.refresh_token)
                .http_only(true)
                .secure(true)
                // .same_site(actix_web::cookie::SameSite::Strict)
                .finish();
            HttpResponse::Ok().cookie(cookie).json(
                SigninResponseWithToken {
                    access_token: res.access_token,
                    refresh_token: "".to_string(),
                }
            )
        },
        Err(_) => {
            HttpResponse::Ok().json("Failed to refresh token")
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("/auth");
    cfg.service(
        scope
            .route("/signin", web::post().to(signin))
            .route("/signup", web::post().to(signup))
            .route("/refresh-token", web::post().to(refresh_token_with_httponly_cookie))
    );
}
