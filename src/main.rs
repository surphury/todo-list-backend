#[macro_use]
extern crate rocket;

mod database;
mod hashing;
mod model;
mod routes;

/* #[cfg(test)]
mod test; */

use database::connect;

use routes::{get_tasks, login, post_task, register_user};

use model::Db;

use sqlx::MySqlPool;

use std::env::var;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    dotenv::dotenv().ok();
    let pool: MySqlPool = connect(&var("DATABASE_URL").expect("DATABASE_URL needs to be set"))
        .await
        .expect("Could not connect with database");

    let _rocket = rocket::build()
        .manage(Db { pool })
        .mount("/api", routes![register_user, login, get_tasks, post_task])
        .launch()
        .await?;

    Ok(())
}
