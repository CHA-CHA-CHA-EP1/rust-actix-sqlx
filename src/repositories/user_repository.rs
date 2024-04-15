use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use crate::domain::user::UserSignup;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn get_user_by_id(&self, id: i32) -> Option<String>;
    async fn create_user(&self, user: UserSignup) -> Result<(), String>;
    async fn test_user(&self);
    async fn get_user_by_email(&self, email: &str) -> Option<String>;
    async fn get_user_by_username(&self, username: &str) -> Option<String>;
}

pub struct UserRepositoryImpl {
    db: Arc<Pool<Postgres>>
}

impl UserRepositoryImpl {
    pub fn new(
            db: Arc<Pool<Postgres>>
        ) -> Self {
        UserRepositoryImpl {
            db
        }
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user_by_id(&self, id: i32) -> Option<String> {
        let query_result = sqlx::query!("SELECT * FROM users WHERE id = $1", id)
            .fetch_one(&*self.db)
            .await;

        match query_result {
            Ok(user) => Some(user.name),
            Err(_) => None
        }
    }

    async fn create_user(&self, user: UserSignup) -> Result<(), String> {
        let query_result = sqlx::query!(
            "INSERT INTO users (name, email, username, password) VALUES ($1, $2, $3, $4)",
            user.name,
            user.email,
            user.username,
            user.password
        )
        .execute(&*self.db)
        .await;

        match query_result {
            Ok(_) => Ok(()),
            Err(_) => Err("Failed to create user".to_string())
        }
    }

    async fn test_user(&self) {
        let query_result = sqlx::query!("SELECT * FROM users WHERE id = $1", 1)
            .fetch_one(&*self.db)
            .await;
        println!("{:?}", query_result);
    }

    async fn get_user_by_email(&self, email: &str) -> Option<String> {
        let query_result = sqlx::query!("SELECT * FROM users WHERE email = $1", email)
            .fetch_one(&*self.db)
            .await;

        match query_result {
            Ok(user) => Some(user.name),
            Err(_) => None
        }
    }

    async fn get_user_by_username(&self, username: &str) -> Option<String> {
        let query_result = sqlx::query!("SELECT * FROM users WHERE username = $1", username)
            .fetch_one(&*self.db)
            .await;

        match query_result {
            Ok(user) => Some(user.name),
            Err(_) => None
        }
    }
}
