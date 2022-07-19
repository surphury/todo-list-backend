use serde::{Deserialize, Serialize};

use sqlx::mysql::MySqlPool;

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
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
