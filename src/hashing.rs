use argon2::Config;

use std::env::var;

pub fn hash(password: &str) -> String {
    let salt = var("SALT").expect("SALT must be set").as_bytes().to_owned();
    let config = Config::default();
    let hash = argon2::hash_encoded(password.as_bytes(), &salt, &config).unwrap();
    return hash;
}
