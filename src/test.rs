use actix_web::{
	test::{call_and_read_body, call_and_read_body_json, init_service, TestRequest},
	web::Bytes,
};

use mongo::connect;

use super::*;

#[actix_web::test]
#[ignore = "requires MongoDB instance running"]
async fn test() {
	let client = connect().await;
	// Clear any data currently in the users collection.
	client
		.database(DB_NAME)
		.collection::<User>(COLL_NAME)
		.drop(None)
		.await
		.expect("drop collection should succeed");

	let app = init_service(
		App::new()
			.app_data(web::Data::new(client))
			.service(add_user)
			.service(get_user),
	)
	.await;

	let user = User {
		username: "janedoe".into(),
		email: "example@example.com".into(),
		password: "password".into(),
	};

	let req = TestRequest::post()
		.uri("/add_user")
		.set_form(&user)
		.to_request();

	let response = call_and_read_body(&app, req).await;
	assert_eq!(response, Bytes::from_static(b"user added"));

	let req = TestRequest::get()
		.uri(&format!("/get_user/{}", &user.username))
		.to_request();

	let response: User = call_and_read_body_json(&app, req).await;
	assert_eq!(response, user);
}
