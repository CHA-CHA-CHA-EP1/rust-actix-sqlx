use argon2::{self, Config};

pub fn hash_data(data: &str) -> String {
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(data.as_bytes(), salt, &config).unwrap();
    hash
}
