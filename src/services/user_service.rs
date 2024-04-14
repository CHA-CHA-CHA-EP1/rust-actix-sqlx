use std::sync::Arc;

use async_trait::async_trait;
use crate::repositories::user_repository::{self, UserRepository};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_user_by_id(&self, id: i32) -> Option<String>;
}

#[derive(Clone)]
pub struct UserServiceImpl {
    user_repository: Arc<dyn UserRepository>
}

impl UserServiceImpl {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        UserServiceImpl {
            user_repository
        }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn get_user_by_id(&self, id: i32) -> Option<String> {
        let user_name = self.user_repository.get_user_by_id(id).await;
        match user_name {
            Some(name) => Some(name),
            None => None
        }
    }
}
