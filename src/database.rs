use crate::hashing::hash;

use crate::model::{DBUser, Db, Login, NewTask, Task, User};

use actix_web::web::Data;

use sqlx::mysql::{MySqlPool, MySqlQueryResult};
use sqlx::Error;

use std::result::Result;

/// `connect` takes a URL as a string and returns a `MySqlPool` or an `Error`
///
/// Arguments:
///
/// * `url`: The URL to the database.
///
/// Returns:
///
/// A Result<MySqlPool, Error>
pub async fn connect(url: &str) -> Result<MySqlPool, Error> {
    let pool = MySqlPool::connect(url).await;
    pool
}

/// It takes a `User` struct and a `Data<Db>` struct, and returns a `Result<MySqlQueryResult, Error>`
///
/// Arguments:
///
/// * `user`: User - This is the user object that we're going to insert into the database.
/// * `db`: &Data<Db>
///
/// Returns:
///
/// A Result<MySqlQueryResult, Error>
pub async fn insert_new_user(user: User, db: &Data<Db>) -> Result<MySqlQueryResult, Error> {
    sqlx::query!(
        r#"
		INSERT INTO users ( username, email, password )
			VALUES ( ?, ?, ? )
			"#,
        user.username,
        user.email,
        hash(&user.password),
    )
    .execute(&db.pool)
    .await
}

/// It deletes a task from the database
///
/// Arguments:
///
/// * `id`: i32 - The id of the task to delete
/// * `user_id`: The user id of the user who created the task.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A Result<MySqlQueryResult, Error>
pub async fn delete_task(id: i32, user_id: i32, db: &Data<Db>) -> Result<MySqlQueryResult, Error> {
    sqlx::query!(
        r#"
		DELETE FROM tasks
			WHERE id = ? AND user_id = ?
			"#,
        id,
        user_id,
    )
    .execute(&db.pool)
    .await
}

/// It updates the status of a task to 2 (in progress) and saves the current time as the start time
///
/// Arguments:
///
/// * `user_id`: The user id of the user who is starting the task.
/// * `task_id`: The id of the task to be started.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A boolean value.
pub async fn start_task_and_save_time(
    user_id: i32,
    task_id: i32,
    db: &Data<Db>,
) -> Result<bool, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
	UPDATE tasks
	SET status = 2,
	start_time = NOW()
	WHERE id = ? AND user_id = ? AND status = 1
		"#,
        task_id,
        user_id,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

/// It updates the status of a task to 3 (finished) and sets the finish time to the current time
///
/// Arguments:
///
/// * `user_id`: The user id of the user who owns the task.
/// * `task_id`: The id of the task to be finished.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A boolean value.
pub async fn finish_task_and_save_time(
    user_id: i32,
    task_id: i32,
    db: &Data<Db>,
) -> Result<bool, sqlx::Error> {
    let rows_affected = sqlx::query!(
        r#"
	UPDATE todos
	SET status = 3,
	finish_time = NOW()
	WHERE id = ? AND user_id = ? AND status = 2
		"#,
        task_id,
        user_id,
    )
    .execute(&db.pool)
    .await?
    .rows_affected();

    Ok(rows_affected > 0)
}

/// It takes a user_id, a NewTask struct, and a database connection, and returns a Result containing
/// either a MySqlQueryResult or a sqlx::Error
///
/// Arguments:
///
/// * `user_id`: The user id of the user who created the task.
/// * `task`: NewTask - This is the struct that we defined earlier.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A Result<MySqlQueryResult, sqlx::Error>
pub async fn add_task(
    user_id: i32,
    task: NewTask,
    db: &Data<Db>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO tasks ( user_id, name, description, status )
			VALUES ( ?, ?, ?, 1 )
			"#,
        user_id,
        task.name,
        task.description
    )
    .execute(&db.pool)
    .await
}

/// It takes a user id and a database connection, and returns a vector of tasks
///
/// Arguments:
///
/// * `user_id`: The user id of the user whose tasks we want to get.
/// * `db`: &Data<Db> - this is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A vector of tasks
pub async fn get_tasks_by_user(user_id: i32, db: &Data<Db>) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(
        Task,
        r#"
		SELECT id, name, description, status FROM tasks WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let tasks = tasks
        .into_iter()
        .map(|task| Task {
            id: task.id,
            name: task.name,
            description: task.description,
            status: task.status,
            /*  start_time: match task.start_time {
                Some(start_time) => Some(start_time.unix_timestamp()),
                None => None,
            },
            finish_time: match task.finish_time {
                Some(finish_time) => Some(finish_time.unix_timestamp()),
                None => None,
            }, */
        })
        .collect();

    Ok(tasks)
}

/// It takes a `Login` struct and a `Data<Db>` struct, and returns a `Result<Vec<DBUser>, sqlx::Error>`
///
/// Arguments:
///
/// * `user`: &Login - This is the struct that we created earlier.
/// * `db`: &Data<Db>
///
/// Returns:
///
/// A vector of DBUser structs
pub async fn verify_password(user: &Login, db: &Data<Db>) -> Result<Vec<DBUser>, sqlx::Error> {
    let users = sqlx::query_as!(
        DBUser,
        r#"
		SELECT id, username FROM users WHERE username = ? AND password = ?"#,
        user.username,
        hash(&user.password)
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(users)
}
