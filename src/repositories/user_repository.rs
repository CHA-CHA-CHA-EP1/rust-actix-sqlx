use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Pool, Postgres};

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn get_user_by_id(&self, id: i32) -> Option<String>;
    async fn test_user(&self);
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

    async fn test_user(&self) {
        let query_result = sqlx::query!("SELECT * FROM users WHERE id = $1", 1)
            .fetch_one(&*self.db)
            .await;
        println!("{:?}", query_result);
    }
}

