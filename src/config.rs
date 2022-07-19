use dotenv::dotenv;
use serde::Deserialize;
use std::env::{self};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub PORT: u16,
    pub MONGODB_URI: String,
    pub MAILGUN_API_KEY: String,
    pub MAILGUN_DOMAIN: String,
    pub SECRET_HASHING_KEY: String,
}

pub fn get_env_variables() -> Config {
    dotenv().ok();
    Config {
        PORT: match env::var("PORT") {
            Ok(port) => port.parse::<u16>().unwrap(),
            Err(_error) => 8080,
        },
        MONGODB_URI: env::var("MONGODB_URI").expect("MONGODB_URI needs to be set"),
        MAILGUN_API_KEY: env::var("MAILGUN_API_KEY").expect("MAILGUN_API_KEY needs to be set"),
        MAILGUN_DOMAIN: env::var("MAILGUN_DOMAIN").expect("MAILGUN_DOMAIN needs to be set"),
        SECRET_HASHING_KEY: env::var("SECRET_HASHING_KEY").expect("MAILGUN_DOMAIN needs to be set"),
    }
}
