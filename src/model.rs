use serde::{Deserialize, Serialize};

use sqlx::mysql::MySqlPool;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DBUser {
    pub username: String,
    pub id: i32,
}

#[derive(Deserialize, Serialize)]
pub struct Login {
    pub username: String,
    pub password: String,
}

pub struct Db {
    pub pool: MySqlPool,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub description: String,
    pub done: bool,
}

pub struct DBTask {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub done: i8,
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
}

#[derive(Deserialize, Debug, Serialize)]
pub struct Claims {
    pub id_user: i32,
    pub exp: i32,
}
