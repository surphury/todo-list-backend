mod database;
mod hashing;
mod jwt;
mod model;
mod routes;
mod utils;

/* #[cfg(test)]
mod test; */

use actix_web::web;
use actix_web::web::Data;
use actix_web::{http, App, HttpServer};

use sqlx::mysql::MySqlPool;

use std::env::var;
use std::io;
use std::result::Result;

use actix_cors::Cors;

use dotenv::dotenv;

use database::connect;

use model::Db;

use routes::{delete_tasks, finish_task, get_tasks, login, post_task, register_user, start_task};

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    dotenv().ok();

    let port: u16 = match var("PORT") {
        Ok(port) => port.parse::<u16>().unwrap(),
        Err(_error) => 8080,
    };

    let database_url: String = var("DATABASE_URL").unwrap();

    let pool: MySqlPool = connect(&database_url)
        .await
        .expect("Could not connect to database");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT", "PATCH"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(Data::new(Db { pool: pool.clone() }))
            .wrap(cors)
            .route("/register_user", web::post().to(register_user))
            .route("/login", web::post().to(login))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(post_task))
            .route("/tasks", web::delete().to(delete_tasks))
            .route("/start_task/{task_id}", web::patch().to(start_task))
            .route("/finish_task/{task_id}", web::patch().to(finish_task))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
