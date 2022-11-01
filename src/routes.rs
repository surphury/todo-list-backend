use crate::database::{
    add_task, delete_task, finish_task_and_save_time, get_tasks_by_user, insert_new_user,
    start_task_and_save_time, verify_password,
};

use crate::jwt::generate_token;

use crate::utils::validate_token;

use crate::model::{Db, Login, NewTask, TaskId, User};

use actix_web::web::{Data, Json, Path};
use actix_web::{HttpRequest, HttpResponse, Responder};

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
        Err(error) => error.message(),
    }
}

pub async fn delete_tasks(req: HttpRequest, task: Json<TaskId>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match delete_task(task.id, user_id, &db).await {
            Ok(_) => HttpResponse::Ok().body("Task deleted"),
            Err(_) => HttpResponse::InternalServerError().body("Error deleting task"),
        },
        Err(error) => error.message(),
    }
}

pub async fn post_task(req: HttpRequest, task: Json<NewTask>, db: Data<Db>) -> impl Responder {
    let authorization = req.headers().get("Authorization");

    let task = NewTask {
        name: task.name.clone(),
        description: task.description.clone(),
    };

    match validate_token(authorization) {
        Ok(user_id) => match add_task(user_id, task, &db).await {
            Ok(_) => match get_tasks_by_user(user_id, &db).await {
                Ok(tasks) => HttpResponse::Ok().json(tasks),
                Err(_) => HttpResponse::InternalServerError().body("Error getting tasks"),
            },
            Err(_) => HttpResponse::InternalServerError().body("Error posting tasks"),
        },
        Err(error) => error.message(),
    }
}

pub async fn start_task(task_id: Path<i32>, req: HttpRequest, db: Data<Db>) -> impl Responder {
    let task_id = task_id.into_inner();
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match start_task_and_save_time(task_id, user_id, &db).await {
            Ok(task_history) => HttpResponse::Accepted().json(task_history),
            Err(error) => error.message(),
        },
        Err(error) => error.message(),
    }
}

pub async fn finish_task(task_id: Path<i32>, req: HttpRequest, db: Data<Db>) -> impl Responder {
    let task_id = task_id.into_inner();
    let authorization = req.headers().get("Authorization");

    match validate_token(authorization) {
        Ok(user_id) => match finish_task_and_save_time(task_id, user_id, &db).await {
            Ok(task_history) => HttpResponse::Accepted().json(task_history),

            Err(err) => err.message(),
        },
        Err(error) => error.message(),
    }
}
