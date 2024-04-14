use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Sync + Send {
    async fn get_user(&self, id: i32) -> Option<String>;
}

pub struct UserRepositoryImpl {}

impl UserRepositoryImpl {
    pub fn new() -> Self {
        UserRepositoryImpl {}
    }
}

#[async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get_user(&self, id: i32) -> Option<String> {
        if id == 1 {
            Some("Alice".to_string())
        } else {
            None
        }
    }
}

