/* use sqlx::mysql::MySqlConnection; */
use crate::model::{NewUser, UnconfirmedUser, User};
/* use dotenv::dotenv;
use std::env; */
use chrono::NaiveDate;

use sqlx::Error;
use sqlx::MySql;
use sqlx::Pool;
use sqlx::{types::chrono::NaiveDateTime, MySqlPool, Row};
use std::result::Result;

use actix_web::web::Data;

/*  */

use sea_query::{ColumnDef, Expr, Func, Iden, MysqlQueryBuilder, OnConflict, Order, Query, Table};
use sea_query_driver_mysql::{bind_query, bind_query_as};
/* use serde_json::{json, Value as Json}; */

use sqlx::pool::PoolConnection;
use uuid::Uuid;

#[derive(Iden)]
enum NewUsers {
	Table,
	Id,
	Username,
	Key,
	Email,
	Created,
}

sea_query::sea_query_driver_mysql!();

pub async fn connect(url: String) -> Result<MySqlPool, Error> {
	let pool = MySqlPool::connect(&url).await;
	pool
}

pub async fn delete_by_username(username: String, statement: String, pool: Data<MySqlPool>) {
	// Delete
	let mut pool = pool.try_acquire().unwrap();
	let (sql, values) = Query::delete()
		.from_table(NewUsers::Table)
		.and_where(Expr::col(NewUsers::Username).eq(username))
		.build(MysqlQueryBuilder);

	let result = bind_query(sqlx::query(&sql), &values)
		.execute(&mut pool)
		.await;
	println!("Delete character: {:?}", result);
}

pub async fn create_table(pool: Data<MySqlPool>) {
	// Schema
	let mut pool = pool.try_acquire().unwrap();
	let sql = Table::create()
		.table(NewUsers::Table)
		.if_not_exists()
		.col(
			ColumnDef::new(NewUsers::Id)
				.integer()
				.not_null()
				.auto_increment()
				.primary_key(),
		)
		.col(ColumnDef::new(NewUsers::Username).string().unique_key())
		.col(ColumnDef::new(NewUsers::Email).string().unique_key())
		.col(ColumnDef::new(NewUsers::Key).string().unique_key())
		.col(ColumnDef::new(NewUsers::Created).date_time())
		.build(MysqlQueryBuilder);
	let result = sqlx::query(&sql).execute(&mut pool).await;
	println!("Create table character: {:?}\n", result);
}

pub async fn insert_new_user(user: UnconfirmedUser, pool: Data<MySqlPool>) -> u64 {
	let mut pool = pool.try_acquire().unwrap();
	// Create
	let (sql, user) = Query::insert()
		.into_table(NewUsers::Table)
		.columns([
			NewUsers::Username,
			NewUsers::Email,
			NewUsers::Key,
			NewUsers::Created,
		])
		.values_panic(vec![
			user.username.into(),
			user.email.into(),
			user.key.into(),
		])
		.build(MysqlQueryBuilder);

	let result = bind_query(sqlx::query(&sql), &user)
		.execute(&mut pool)
		.await;
	println!("Insert into character: {:?}\n", result);
	let id = result.unwrap().last_insert_id();
	id
}

pub async fn update_password(new_username: String, username: String, pool: Data<MySqlPool>) {
	let mut pool = pool.try_acquire().unwrap();
	// Update
	let (sql, values) = Query::update()
		.table(NewUsers::Table)
		.values(vec![(NewUsers::Username, new_username.into())])
		.and_where(Expr::col(NewUsers::Username).eq(username))
		.build(MysqlQueryBuilder);

	let result = bind_query(sqlx::query(&sql), &values)
		.execute(&mut pool)
		.await;
	println!("Update character: {:?}\n", result);
}

pub async fn get_users(pool: Data<MySqlPool>) {
	// Read
	let mut pool = pool.try_acquire().unwrap();

	let (sql, values) = Query::select()
		.columns([
			NewUsers::Username,
			NewUsers::Email,
			NewUsers::Key,
			NewUsers::Created,
		])
		.from(NewUsers::Table)
		.order_by(NewUsers::Id, Order::Desc)
		.build(MysqlQueryBuilder);

	let rows = bind_query_as(sqlx::query_as::<_, UnconfirmedUser>(&sql), &values)
		.fetch_all(&mut pool)
		.await
		.unwrap();
	println!("Select all users:");
	for row in rows.iter() {
		println!("{:?}", row);
	}
	println!();
}

pub async fn get_user_by_username(username: String, pool: Data<MySqlPool>) -> Vec<UnconfirmedUser> {
	// Read
	let mut pool = pool.try_acquire().unwrap();
	let (sql, values) = Query::select()
		.columns([
			NewUsers::Username,
			NewUsers::Email,
			NewUsers::Key,
			NewUsers::Created,
		])
		.from(NewUsers::Table)
		.order_by(NewUsers::Id, Order::Desc)
		.limit(1)
		.build(MysqlQueryBuilder);

	let rows = bind_query_as(sqlx::query_as::<_, UnconfirmedUser>(&sql), &values)
		.fetch_all(&mut pool)
		.await
		.unwrap();
	rows
}
