mod confirm;
mod database;
mod model;
#[cfg(test)]
mod test;

use dotenv::dotenv;

use database::{
	connect, create_table, get_user_by_username, get_users, insert_new_user, update_password,
};

use confirm::send_confirmation_email;

use uuid::Uuid;

use std::io;

use actix_web::{
	delete, get, post,
	web::{Data, Json, Path},
	App, HttpResponse, HttpServer,
};

use sqlx::{mysql::MySqlPool, pool::PoolConnection, MySql};

/* use mongodb::{bson::doc, Client, Collection}; */

/* use std::env; */
use std::result::Result;

use model::{NewUser, UnconfirmedUser};

/// Adds a new user to the "users" collection in the database.
#[post("/user")]
async fn add_user(form: Json<NewUser>, pool: Data<MySqlPool>) -> HttpResponse {
	let key = Uuid::new_v4();
	let user = UnconfirmedUser {
		email: form.email.clone(),
		username: form.username.clone(),
		key: key.to_string(),
	};
	loop {
		match send_confirmation_email(&user).await {
			Ok(_) => break,
			Err(err) => {
				println!("{:?}", err)
			}
		}
	}
	let id = insert_new_user(user, pool).await;
	HttpResponse::Ok().body("user added")
	/* match result {
		Ok(_) => ,
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	} */
}

#[delete("/user")]
async fn delete_user(form: Json<NewUser>, pool: Data<MySqlPool>) -> HttpResponse {
	/* let collection = client.database(DB_NAME).collection::<NewUser>(COLL_NAME);
	let result = collection
		.find_one_and_delete(doc! { "username" : &form.username }, None)
		.await;
	match result {
		Ok(_) => HttpResponse::Ok().body("user deleted"),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	} */
	HttpResponse::Ok().body("user deleted")
}

/// Gets the user with the supplied username.
#[get("/user/{username}")]
async fn get_user(username: Path<String>, pool: Data<MySqlPool>) -> HttpResponse {
	let user = get_user_by_username(username.to_string(), pool).await;

	HttpResponse::Ok().json(user)
}

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
	dotenv().ok();
	let port: u16 = match std::env::var("PORT") {
		Ok(port) => port.parse::<u16>().unwrap(),
		Err(_error) => 8080,
	};
	let database_url: String = std::env::var("DATABASE_URL").unwrap();
	let pool: MySqlPool = connect(database_url)
		.await
		.expect("Could not connect to database");
	HttpServer::new(move || {
		App::new()
			.app_data(Data::new(pool.clone()))
			.service(add_user)
			.service(get_user)
			.service(delete_user)
	})
	.bind(("127.0.0.1", port))?
	.run()
	.await
}
