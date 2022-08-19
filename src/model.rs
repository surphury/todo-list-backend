use actix_web::cookie::time::OffsetDateTime;

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
pub struct NewTask {
    pub name: String,
    pub description: String,
    pub status: i16,
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: i16,
    pub start_time: Option<i64>,
    pub finish_time: Option<i64>,
}

pub struct DBTask {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub status: i16,
    pub start_time: Option<OffsetDateTime>,
    pub finish_time: Option<OffsetDateTime>,
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

#[derive(Deserialize /* Serialize */)]
pub struct TaskId {
    pub id: i32,
}
