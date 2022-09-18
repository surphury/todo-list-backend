use crate::hashing::hash;

use crate::model::{
    DBUser, Db, History, Login, NewTask, ResponseTask, Task, TaskError, TaskHistory, User,
};

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

/// It checks if a task id is invalid
///
/// Arguments:
///
/// * `task_id`: The id of the task to be checked
/// * `user_id`: The user id of the user who is trying to access the task.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A boolean value.
pub async fn is_an_invalid_task_id(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<bool, sqlx::Error> {
    let task = sqlx::query!(
        r#"
		SELECT * 
		FROM tasks 
		WHERE id = ? AND user_id = ?
			"#,
        task_id,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    Ok(task.len() == 0)
}

/// It starts a task and saves the time
///
/// Arguments:
///
/// * `user_id`: The user id of the user who is starting the task.
/// * `task_id`: The id of the task to start
/// * `db`: &Data<Db> - this is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// The number of rows affected by the query.
pub async fn start_task_and_save_time(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<bool, TaskError> {
    if is_an_invalid_task_id(task_id, user_id, db).await? {
        return Err(TaskError::InvalidId);
    }

    let task_history = sqlx::query!(
        r#"
			SELECT start_time, finish_time, task_id
			FROM task_history
			WHERE user_id = ? AND task_id = ?
			"#,
        user_id,
        task_id,
    )
    .fetch_all(&db.pool)
    .await?;

    if task_history.len() == 0 || task_history[task_history.len() - 1].finish_time.is_some() {
        let has_started_task = sqlx::query!(
            r#"
			INSERT INTO task_history ( user_id, task_id, start_time )
		VALUES ( ?, ?, NOW() )
			"#,
            user_id,
            task_id,
        )
        .execute(&db.pool)
        .await?
        .rows_affected()
            == 1;

        Ok(has_started_task)
    } else {
        Err(TaskError::IsPending)
    }
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
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<bool, TaskError> {
    if is_an_invalid_task_id(task_id, user_id, db).await? {
        return Err(TaskError::InvalidId);
    }
    let is_task_finished = sqlx::query!(
        r#"
		UPDATE task_history
		SET finish_time = NOW()
		WHERE task_id = ? AND user_id = ? AND start_time IS NOT NULL AND finish_time IS NULL
			"#,
        task_id,
        user_id,
    )
    .execute(&db.pool)
    .await?
    .rows_affected()
        == 1;

    Ok(is_task_finished)
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
		INSERT INTO tasks ( user_id, name, description )
			VALUES ( ?, ?, ? )
			"#,
        user_id,
        task.name,
        task.description
    )
    .execute(&db.pool)
    .await
}

/// It's getting all the tasks for a user, and then getting all the history for those tasks, and then
/// returning a list of tasks with their history
///
/// Arguments:
///
/// * `user_id`: i32 - The user id that we're going to use to get the tasks.
/// * `db`: &Data<Db> - This is the database connection pool.
///
/// Returns:
///
/// A vector of ResponseTask structs.
pub async fn get_tasks_by_user(
    user_id: i32,
    db: &Data<Db>,
) -> Result<Vec<ResponseTask>, sqlx::Error> {
    let tasks = sqlx::query_as!(
        Task,
        r#"
		SELECT id, name, description FROM tasks WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let tasks_history = sqlx::query_as!(
        History,
        r#"
		SELECT task_id, start_time, finish_time FROM task_history WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let tasks: Vec<ResponseTask> = tasks
        .iter()
        .map(|task| {
            // It's creating a new ResponseTask struct and returning it.
            ResponseTask {
                id: task.id,
                name: task.name.clone(),
                description: task.description.clone(),
                history: tasks_history
                    .iter()
                    .filter(|history| history.task_id == task.id)
                    .map(|history| TaskHistory {
                        start_time: history.start_time.unix_timestamp(),
                        finish_time: match history.finish_time {
                            Some(finish_time) => Some(finish_time.unix_timestamp()),
                            None => None,
                        },
                    })
                    .collect(),
            }
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
