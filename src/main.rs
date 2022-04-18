mod model;
mod mongo;

#[cfg(test)]
mod test;

use std::io::Result;

use actix_web::{get, post, web, App, HttpResponse, HttpServer};
use mongo::connect;
use mongodb::{bson::doc, Client, Collection};

use model::User;

const DB_NAME: &str = "market";
const COLL_NAME: &str = "users";

/// Adds a new user to the "users" collection in the database.
#[post("/add_user")]
async fn add_user(client: web::Data<Client>, form: web::Json<User>) -> HttpResponse {
	let collection = client.database(DB_NAME).collection(COLL_NAME);
	let result = collection.insert_one(form.into_inner(), None).await;
	match result {
		Ok(_) => HttpResponse::Ok().body("user added"),
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	}
}

/// Gets the user with the supplied username.
#[get("/user/{username}")]
async fn get_user(client: web::Data<Client>, username: web::Path<String>) -> HttpResponse {
	let username = username.into_inner();
	let collection: Collection<User> = client.database(DB_NAME).collection(COLL_NAME);
	match collection
		.find_one(doc! { "username": &username }, None)
		.await
	{
		Ok(Some(user)) => HttpResponse::Ok().json(user),
		Ok(None) => {
			HttpResponse::NotFound().body(format!("No user found with username {}", username))
		}
		Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
	}
}

#[actix_web::main]
async fn main() -> Result<()> {
	let client = connect().await;
	HttpServer::new(move || {
		App::new()
			.app_data(web::Data::new(client.clone()))
			.service(add_user)
			.service(get_user)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
