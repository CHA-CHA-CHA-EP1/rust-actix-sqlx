use std::sync::Arc;

use async_trait::async_trait;
use chrono::Utc;
use sqlx::{decode, encode};
use crate::{domain::user::{Claims, Signin, SigninResponse, UserSignup}, repositories::user_repository::{self, UserRepository}};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

#[async_trait]
pub trait UserService: Sync + Send {
    async fn get_user_by_id(&self, id: i32) -> Option<String>;
    async fn create_user(&self, user: UserSignup) -> Result<(), String>;
    async fn signin(&self, signin: Signin) -> Result<SigninResponse, String>;
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

    async fn signin(&self, signin: Signin) -> Result<SigninResponse, String> {
        let user = self.user_repository.get_user_by_username(&signin.username).await;

        let key = b"secret";
        let current_time = Utc::now();
        let exp_time = current_time.timestamp() + 60;

        let my_claims = Claims {
            sub: String::from("user1234"),
            exp: exp_time,
        };

        let token = match encode(&Header::default(), &my_claims, &EncodingKey::from_secret(key)) {
            Ok(t) => t,
            Err(e) => return Err(e.to_string())
        };

        let new_token = encode(
            &Header::default(),
            &my_claims,
            &EncodingKey::from_secret(key),
        );

        // println!("HHHHH: {:?}", new_token.clone().unwrap());

        // let decoded_token = decode::<Claims>(
        //     &new_token.unwrap(),
        //     &DecodingKey::from_secret(key),
        //     &Validation::new(Algorithm::default())
        // );

        // println!("Decoded Token: {:?}", decoded_token.unwrap().claims);

        // `token` is a struct with 2 fields: `header` and `claims` where `claims` is your own struct.
        // let decode_token = decode::<Claims>(&token, &DecodingKey::from_secret("secret".as_ref()), &Validation::default());

        match user {
            Ok(user) => {
                let password_hashed = crate::utils::hash::hash_data(&signin.password);
                if user.password == password_hashed {
                    Ok(SigninResponse {
                        token: new_token.unwrap(),
                        user
                    })
                } else {
                    Err("Invalid password".to_string())
                }
            },
            Err(e) => Err(e)
        }
    }
}
