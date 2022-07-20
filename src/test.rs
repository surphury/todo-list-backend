use super::rocket;

use rocket::fairing::AdHoc;
use rocket::http::ContentType;
use rocket::tokio::sync::Barrier;
use rocket::{Build, Rocket};

use rocket::serde::json::Json;

use super::routes::{get_tasks, login, post_task, register_user};

use super::model::{Db, Login, Task, User};

#[rocket::async_test]
async fn add_user_and_login() {
    use rocket::local::asynchronous::Client;

    let client = Client::tracked(rocket()).await.unwrap();

    let user = User {
        username: String::from("jose261004"),
        email: String::from("jose261004@gmail.com"),
        password: String::from("jose261004"),
    };

    let req = client
        .post("/api/")
        .header(ContentType::JSON)
        .set_body(&user)
        .dispatch();
}

pub fn rocket() -> Rocket<Build> {
    rocket::build()
        .mount("/api", routes![get_tasks, login, post_task, register_user])
        .attach(AdHoc::on_ignite("", |rocket| async {
            rocket.manage(Barrier::new(2))
        }))
}
