use crate::database::{finish_task_and_save_time, start_task_and_save_time};
use crate::jwt::generate_token;
use crate::utils::validate_token;

use super::database::{add_task, delete_task, get_tasks_by_user, insert_new_user, verify_password};

use super::model::{Db, Login, NewTask, Task, TaskId, User};

use actix_web::web::{Data, Json};
use actix_web::{delete, get, patch, post, HttpRequest, HttpResponse, Responder};

#[post("/register_user")]
pub async fn register_user(new_user: Json<User>, db: Data<Db>) -> impl Responder {
    let new_user = User {
        password: new_user.password.clone(),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
    };

    match insert_new_user(new_user, &db).await {
        Ok(_) => HttpResponse::Ok().body("User added"),
        Err(_) => HttpResponse::InternalServerError().body("Error adding user"),
    }
}

#[post("/login")]
pub async fn login(user: Json<Login>, db: Data<Db>) -> impl Responder {
    let user = Login {
        password: user.password.clone(),
        username: user.username.clone(),
    };

    let users = verify_password(&user, &db).await;

    match users {
        Ok(users) => {
            if users.len() == 1 {
                let token = generate_token(&users[0]);

                match token {
                    Ok(token) => HttpResponse::Ok().json(token),
                    Err(_) => HttpResponse::InternalServerError().body("Error generating token"),
                }
            } else {
                HttpResponse::Unauthorized().body("Verification failed")
            }
        }
        Err(_) => HttpResponse::InternalServerError().body("Error verifying user"),
    }
}

#[get("/tasks")]
pub async fn get_tasks(req: HttpRequest, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => {
            let tasks = get_tasks_by_user(user_id, &db).await;

            match tasks {
                Ok(tasks) => HttpResponse::Ok().json(tasks),
                Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
            }
        }
        Err(error_message) => HttpResponse::Unauthorized().body(error_message),
    }
}

#[delete("/tasks")]
pub async fn delete_tasks(req: HttpRequest, task: Json<TaskId>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match delete_task(task.id, user_id, &db).await {
            Ok(_) => HttpResponse::Ok().body("Task deleted"),
            Err(_) => HttpResponse::InternalServerError().body("Error deleting task"),
        },
        Err(error_message) => HttpResponse::Unauthorized().body(error_message),
    }
}

#[post("/tasks")]
pub async fn post_task(req: HttpRequest, task: Json<NewTask>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    let task = NewTask {
        name: task.name.clone(),
        description: task.description.clone(),
        status: task.status.clone(),
    };

    match validate_token(authorization) {
        Ok(user_id) => match add_task(user_id, task, &db).await {
            Ok(_) => match get_tasks_by_user(user_id, &db).await {
                Ok(tasks) => HttpResponse::Ok().json(tasks),
                Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Error posting tasks"),
        },
        Err(error_message) => HttpResponse::Unauthorized().body(error_message),
    }
}

#[patch("/tasks")]
pub async fn start_task(req: HttpRequest, task: Json<Task>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match start_task_and_save_time(user_id, task.id, &db).await {
            Ok(_) => HttpResponse::Ok().body("Task started"),
            Err(error) => {
                println!("{:?}", error);
                HttpResponse::InternalServerError().body("Error starting tasks")
            }
        },
        Err(error_message) => HttpResponse::Unauthorized().body(error_message),
    }
}

#[patch("/finish_task")]
pub async fn end_task(req: HttpRequest, task: Json<Task>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match finish_task_and_save_time(user_id, task.id, &db).await {
            Ok(_) => HttpResponse::Ok().body("Task started"),
            Err(_) => HttpResponse::InternalServerError().body("Error posting tasks"),
        },
        Err(error_message) => HttpResponse::Unauthorized().body(error_message),
    }
}
