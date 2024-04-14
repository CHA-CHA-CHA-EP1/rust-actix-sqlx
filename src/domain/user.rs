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

#[derive(Clone)]
pub struct AppState {
    pub user_service: UserServiceImpl,
}
