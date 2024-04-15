use std::sync::Arc;

use async_trait::async_trait;
use crate::{domain::user::{UserSignup, Signin}, repositories::user_repository::{self, UserRepository}};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_user_by_id(&self, id: i32) -> Option<String>;
    async fn create_user(&self, user: UserSignup) -> Result<(), String>;
    async fn signin(&self, signin: Signin) -> Result<(), String>;
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

    async fn create_user(&self, mut user: UserSignup) -> Result<(), String> {
        let email_exists = self.user_repository.get_user_by_email(&user.email).await;

        if email_exists.is_some() {
            return Err("Email already exists".to_string());
        }

        let username_exists = self.user_repository.get_user_by_username(&user.username).await;
        
        let password_hashed = crate::utils::hash::hash_data(&user.password);
        user.password = password_hashed;

        let result = self.user_repository.create_user(user.clone()).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    async fn signin(&self, signin: Signin) -> Result<(), String> {
        let user = self.user_repository.get_user_by_username(&signin.username).await;
        match user {
            Ok(user) => {
                let password_hashed = crate::utils::hash::hash_data(&signin.password);
                if user.password == password_hashed {
                    Ok(())
                } else {
                    Err("Invalid password".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }
}
