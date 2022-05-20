use actix_web::{
	test::{call_and_read_body, call_and_read_body_json, init_service, TestRequest},
	web::Bytes,
};

use super::*;

#[actix_web::test]
async fn can_add_new_users() {
	let database_url: String =
		std::env::var("DATABASE_URL").expect("DATABASE_URL must be set as a environment variable");
	let pool = connect(database_url).await;

	let app = init_service(
		App::new()
			.app_data(Data::new(pool))
			.service(add_user)
			.service(get_user)
			.service(delete_user),
	)
	.await;

	let user = NewUser {
		username: "janedoe".into(),
		email: "example@example.com".into(),
	};

	let req = TestRequest::post()
		.uri("/user")
		.set_form(&user)
		.to_request();

	let response = call_and_read_body(&app, req).await;
	assert_eq!(response, Bytes::from_static(b"user added"));

	let req = TestRequest::get()
		.uri(&format!("/user/{}", &user.username))
		.to_request();

	let response: NewUser = call_and_read_body_json(&app, req).await;
	assert_eq!(response, user);

	let req = TestRequest::delete()
		.uri("/user")
		.set_form(&user)
		.to_request();

	let response: NewUser = call_and_read_body_json(&app, req).await;
	assert_eq!(response, user);
}

/* #[actix_web::test]
async fn can_delete_users() {
	let client = connect().await;
	client
		.database(DB_NAME)
		.collection::<NewUser>(COLL_NAME)
		.drop(None)
		.await
		.expect("drop collection should succeed");

	let app = init_service(
		App::new()
			.app_data(Data::new(client))
			.service(add_user)
			.service(get_user)
			.service(delete_user),
	)
	.await;

	let user = NewUser {
		username: "janedoe".into(),
		email: "example@example.com".into(),
	};

	let req = TestRequest::post()
		.uri("/user")
		.set_form(&user)
		.to_request();

	let response = call_and_read_body(&app, req).await;
	assert_eq!(response, Bytes::from_static(b"user added"));

	let req = TestRequest::delete()
		.uri("/user")
		.set_form(&user)
		.to_request();

	let response = call_and_read_body(&app, req).await;
	assert_eq!(response, Bytes::from_static(b"user deleted"));
}
 */
