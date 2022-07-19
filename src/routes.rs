use super::database::{add_task, get_tasks_by_user, insert_new_user, verify_password};

use super::model::{Db, Login, Task, User};

use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;

#[post("/register_user", data = "<new_user>")]
pub async fn register_user(new_user: Json<User>, db: &State<Db>) -> Status {
    let new_user = User {
        password: new_user.password.clone(),
        username: new_user.username.clone(),
        email: new_user.email.clone(),
    };

    match insert_new_user(new_user, db).await {
        Ok(_) => Status::Created,
        Err(_) => Status::ExpectationFailed,
    }
}

#[post("/login", data = "<user>")]
pub async fn login(user: Json<Login>, db: &State<Db>) -> Status {
    let user = Login {
        password: user.password.clone(),
        username: user.username.clone(),
    };

    if verify_password(user, db).await {
        Status::Ok
    } else {
        Status::Unauthorized
    }
}

#[get("/tasks/<user_id>")]
pub async fn get_tasks(user_id: i32, db: &State<Db>) -> Json<Vec<Task>> {
    let tasks = get_tasks_by_user(user_id, db).await;

    match tasks {
        Ok(tasks) => Json(tasks),
        Err(_) => Json(vec![]),
    }
}

#[post("/tasks/<user_id>", data = "<task>")]
pub async fn post_task(user_id: i32, task: Json<Task>, db: &State<Db>) -> Status {
    let task = Task {
        name: task.name.clone(),
        description: task.description.clone(),
        done: task.done.clone(),
    };
    match add_task(user_id, task, db).await {
        Ok(_) => Status::Created,
        Err(_) => Status::ExpectationFailed,
    }
}
