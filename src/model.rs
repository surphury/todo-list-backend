use actix_web::{cookie::time::OffsetDateTime, HttpResponse};

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
}

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
}

#[derive(Serialize)]
pub struct TaskHistory {
    pub start_time: i64,          /* Date expressed in seconds */
    pub finish_time: Option<i64>, /* Date expressed in seconds */
}

#[derive(Serialize)]
pub struct ResponseTask {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub history: Vec<TaskHistory>,
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

/* #[derive(Clone)] */
pub struct History {
    pub task_id: i32,
    pub start_time: OffsetDateTime,
    pub finish_time: Option<OffsetDateTime>,
}

pub enum TaskError {
    InvalidId,
    IsPending,
    DbError(sqlx::Error),
}

impl From<sqlx::Error> for TaskError {
    fn from(error: sqlx::Error) -> Self {
        TaskError::DbError(error)
    }
}

impl TaskError {
    pub fn message(self) -> HttpResponse {
        match self {
            TaskError::InvalidId => {
                HttpResponse::NotFound().body("There is no task with the provided id")
            }
            TaskError::IsPending => {
                HttpResponse::Conflict().body("The tasks hasn't been completed yet")
            }
            TaskError::DbError(err) => {
                println!("{:#?}", err);
                HttpResponse::InternalServerError().body("Couldn't complete operation")
            }
        }
    }
}

pub enum VerificationError {
    InvalidToken,
    EmptyToken,
    /* ServerFailedVerifyingToken */
}

/* impl From<VerificationError> for HttpResponse {
    fn from(error: VerificationError) -> Self {
        error.message()
    }
} */

impl VerificationError {
    pub fn message(self) -> HttpResponse {
        match self {
            VerificationError::EmptyToken => {
                HttpResponse::Unauthorized().body("Empty validation token")
            }
            VerificationError::InvalidToken => HttpResponse::Unauthorized().body("Invalid Token"),
            /* VerificationError::ServerFailedVerifyingToken => {
                HttpResponse::ServiceUnavailable().body("Not able to verify at the moment")
            } */
        }
    }
}
