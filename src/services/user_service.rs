use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{decode, encode};
use crate::{domain::user::{Claims, Signin, SigninResponse, SigninResponseWithToken, UserSignup}, repositories::user_repository::{self, UserRepository}};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_user_by_id(&self, id: i32) -> Option<String>;
    async fn create_user(&self, user: UserSignup) -> Result<(), String>;
    async fn signin(&self, signin: Signin) -> Result<SigninResponseWithToken, String>;
    async fn refresh_token(&self, refresh_token: String) -> Result<SigninResponseWithToken, String>;
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
        
        if username_exists.is_ok() {
            return Err("Username already exists".to_string());
        }

        let password_hashed = crate::utils::hash::hash_data(&user.password);
        user.password = password_hashed;

        let result = self.user_repository.create_user(user.clone()).await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(e)
        }
    }

    async fn refresh_token(&self, refresh_token: String) -> Result<SigninResponseWithToken, String> {
        let key = b"secret";
        let validation = Validation::new(Algorithm::HS256);
        let token_data = decode::<Claims>(&refresh_token, &DecodingKey::from_secret(key), &validation);

        match token_data {
            Ok(token) => {
                let access_token = encode(
                    &Header::default(),
                    &Claims {
                        sub: token.claims.sub.clone(),
                        exp: token.claims.exp
                    },
                    &EncodingKey::from_secret(key)
                ).unwrap();

                let new_refresh_token = encode(
                    &Header::default(),
                    &Claims {
                        sub: token.claims.sub.clone(),
                        exp: token.claims.exp + 60
                    },
                    &EncodingKey::from_secret(key)
                ).unwrap();

                return Ok(SigninResponseWithToken {
                    access_token,
                    refresh_token: new_refresh_token
                })
            },
            Err(_) => {
                return Err("Invalid token".to_string());
            }
        }
    }

    async fn signin(&self, signin: Signin) -> Result<SigninResponseWithToken, String> {
        let user = self.user_repository.get_user_by_username(&signin.username).await;

        let key = b"secret";
        let current_time = Utc::now();
        let exp_time = current_time.timestamp() + 0;

        let my_claims = Claims {
            sub: String::from("user001"),
            exp: exp_time,
        };

        let payload_refresh_token = Claims {
            sub: String::from("user001"),
            exp: exp_time + 60,
        };

        let refresh_token = match encode(&Header::default(), &payload_refresh_token, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string())
        };

        let access_token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key),
        ).unwrap();

        match user {
            Ok(user) => {
                let password_hashed = crate::utils::hash::hash_data(&signin.password);
                if user.password == password_hashed {
                    Ok(SigninResponseWithToken {
                        access_token,
                        refresh_token
                    })
                } else {
                    Err("Invalid password".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }
}
