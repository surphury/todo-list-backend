mod database;
mod hashing;
mod jwt;
mod model;
mod routes;

/* #[cfg(test)]
mod test; */

use dotenv::dotenv;

use database::connect;

use actix_cors::Cors;

use actix_web::web::Data;
use actix_web::{http, App, HttpServer};

use sqlx::mysql::MySqlPool;

use routes::{get_tasks, login, post_task, register_user};

use std::env::var;
use std::io;
use std::result::Result;

use model::Db;

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
            .allowed_methods(vec!["GET", "POST", "DELETE", "PUT"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
            .app_data(Data::new(Db { pool: pool.clone() }))
            .wrap(cors)
            .service(login)
            .service(get_tasks)
            .service(post_task)
            .service(register_user)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
