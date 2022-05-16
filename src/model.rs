use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct UnconfirmedUser {
	pub username: String,
	pub email: String,
	pub key: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct NewUser {
	pub username: String,
	pub email: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct User {
	pub username: String,
	pub email: String,
	pub password: String,
}
