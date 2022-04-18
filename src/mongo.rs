use mongodb::{bson::doc, options::IndexOptions, Client, IndexModel};

use crate::model::User;

const DB_NAME: &str = "market";
const COLL_NAME: &str = "users";

/// Creates an index on the "username" field to force the values to be unique.
pub async fn create_username_index(client: &Client) {
	let options = IndexOptions::builder().unique(true).build();
	let model = IndexModel::builder()
		.keys(doc! { "username": 1 })
		.options(options)
		.build();
	client
		.database(DB_NAME)
		.collection::<User>(COLL_NAME)
		.create_index(model, None)
		.await
		.expect("creating an index should succeed");
}

pub async fn connect() -> Client {
	dotenv::dotenv().ok();
	let uri = std::env::var("MONGODB_URI").expect("MONGODB_URI must be set");
	let client = Client::with_uri_str(uri).await.expect("failed to connect");
	create_username_index(&client).await;
	return client;
}
