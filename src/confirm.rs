use crate::model::UnconfirmedUser;
use reqwest::{Client, Error, Response};

pub async fn send_confirmation_email(user: &UnconfirmedUser) -> Result<Response, Error> {
	let sender = "jose261004@gmail.com";
	let subject = "Confirmation key";
	let api_key = "5bc75a5ed380b252178c4095dde50bbc-100b5c8d-be842efc";
	let domain = "sandboxcabb39833861434dbf5e54333c8ba74a.mailgun.org";
	const BASE_URL: &str = "https://api.mailgun.net/v3";
	const MESSAGES_ENDPOINT: &str = "messages";
	let params: [(&str, &str); 5] = [
		("from", sender),
		("to", &user.email),
		("to", &user.email),
		("subject", subject),
		("text", "123445345435 hola prueba 1"),
	];
	let client = Client::new();
	let url = format!("{}/{}/{}", BASE_URL, domain, MESSAGES_ENDPOINT);
	let result = client
		.post(url)
		.basic_auth("api", Some(api_key))
		.form(&params)
		.send()
		.await;
	result
}
