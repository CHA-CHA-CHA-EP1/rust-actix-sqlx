use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String, 
    pub email: String,
    pub username: String,
    pub password: String,
}
