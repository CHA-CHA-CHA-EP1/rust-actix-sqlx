use serde::{Serialize, Deserialize};

use crate::services::user_service::UserServiceImpl;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String, 
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserSignup {
    pub name: String,
    pub email: String,
    pub username: String,
    pub password: String,
}


#[derive(Deserialize)]
pub struct Signin {
    pub username: String,
    pub password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserServiceImpl,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninResponse {
    pub token: String,
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SigninResponseWithToken {
    pub access_token: String,
    pub refresh_token: String,
}

