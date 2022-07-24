use crate::jwt::{generate_token, verify_token};

use super::database::{add_task, get_tasks_by_user, insert_new_user, verify_password};

use super::model::{Db, Login, Task, User};

use actix_web::web::{Data, Json, Path};
use actix_web::{get, post, HttpRequest, HttpResponse, Responder};

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

#[get("/tasks/{user_id}")]
pub async fn get_tasks(req: HttpRequest, user_id: Path<i32>, db: Data<Db>) -> impl Responder {
    let token = req.headers().get("Authorization");

    match token {
        Some(token) => {
            let token = verify_token(token.to_str().unwrap());

            let user_id = user_id.into_inner();

            match token {
                Ok(_) => {
                    let tasks = get_tasks_by_user(user_id, &db).await;
                    match tasks {
                        Ok(tasks) => HttpResponse::Ok().json(tasks),
                        Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
            }
        }
        _ => HttpResponse::Unauthorized().body("Verification failed"),
    }
}

#[post("/tasks")]
pub async fn post_task(req: HttpRequest, task: Json<Task>, db: Data<Db>) -> impl Responder {
    let token = req.headers().get("Authorization");

    let task = Task {
        name: task.name.clone(),
        description: task.description.clone(),
        done: task.done.clone(),
    };

    match token {
        Some(token) => {
            let decoded_token = verify_token(token.to_str().unwrap());

            match decoded_token {
                Ok(decoded_token) => {
                    let user_id = decoded_token.claims.id_user;
                    match add_task(user_id, task, &db).await {
                        Ok(_) => HttpResponse::Ok().body("Task added"),
                        Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
                    }
                }
                Err(_) => HttpResponse::Unauthorized().body("Invalid token"),
            }
        }
        _ => HttpResponse::Unauthorized().body("Verification failed"),
    }
}
