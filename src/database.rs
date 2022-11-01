use crate::hashing::hash;

use crate::model::{
    DBUser, Db, History, Login, NewTask, ResponseTask, Task, TaskError, TaskHistory, User,
};

use actix_web::web::Data;

use sqlx::mysql::{MySqlPool, MySqlQueryResult};

use std::result::Result;

/// `connect` takes a string, `url`, and returns a `Result<MySqlPool, sqlx::Error>`
///
/// Arguments:
///
/// * `url`: The URL to connect to the database.
///
/// Returns:
///
/// A connection pool to the database.
pub async fn connect(url: &str) -> Result<MySqlPool, sqlx::Error> {
    let pool = MySqlPool::connect(url).await;
    pool
}

/// It fetches a task from the database and returns it as a `ResponseTask` struct
///
/// Arguments:
///
/// * `task_id`: The ID of the task to fetch.
/// * `user_id`: The user ID of the user who is requesting the task.
/// * `db`: &Data<Db> - this is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A ResponseTask
pub async fn fetch_task(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<ResponseTask, sqlx::Error> {
    let task = sqlx::query_as!(
        Task,
        r#"
		SELECT id, name, description FROM tasks WHERE user_id = ?"#,
        user_id,
    )
    .fetch_one(&db.pool)
    .await?;

    let tasks_history = sqlx::query_as!(
        History,
        r#"
		SELECT task_id, start_time, finish_time FROM task_history WHERE user_id = ? AND task_id = ?"#,
        user_id,
        task_id
    )
    .fetch_all(&db.pool)
    .await?;

    let task = ResponseTask {
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
    };

    Ok(task)
}

/// It deletes a task from the database
///
/// Arguments:
///
/// * `user`: User - This is the user struct that we created in the models.rs file.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A Result<MySqlQueryResult, sqlx::Error>
pub async fn insert_new_user(user: User, db: &Data<Db>) -> Result<MySqlQueryResult, sqlx::Error> {
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
/// * `id`: The id of the task to delete
/// * `user_id`: The user_id of the user who created the task.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A Result<MySqlQueryResult, sqlx::Error>
pub async fn delete_task(
    id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<MySqlQueryResult, sqlx::Error> {
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
/// * `user_id`: The user id of the user who created the task.
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
///
/// Returns:
///
/// A boolean value.
pub async fn is_an_invalid_task_id(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<bool, TaskError> {
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
/// * `task_id`: The id of the task to start
/// * `user_id`: The user id of the user who is starting the task.
/// * `db`: &Data<Db>
///
/// Returns:
///
/// A Result<ResponseTask, TaskError>
pub async fn start_task_and_save_time(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<ResponseTask, TaskError> {
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

    let is_startable =
        task_history.len() == 0 || task_history[task_history.len() - 1].finish_time.is_some();

    if is_startable {
        sqlx::query!(
            r#"
			INSERT INTO task_history ( user_id, task_id, start_time )
		VALUES ( ?, ?, NOW() )
			"#,
            user_id,
            task_id,
        )
        .execute(&db.pool)
        .await?;

        let task = fetch_task(task_id, user_id, db).await?;
        Ok(task)
    } else {
        Err(TaskError::NotFinished)
    }
}

/// "If the task is valid, then finish the task and return the task."
///
/// The first thing we do is check if the task is valid. If it's not, then we return an error
///
/// Arguments:
///
/// * `task_id`: The id of the task to finish.
/// * `user_id`: The user id of the user who is trying to finish the task.
/// * `db`: &Data<Db> - this is the database connection pool.
///
/// Returns:
///
/// A Result<ResponseTask, TaskError>
pub async fn finish_task_and_save_time(
    task_id: i32,
    user_id: i32,
    db: &Data<Db>,
) -> Result<ResponseTask, TaskError> {
    if is_an_invalid_task_id(task_id, user_id, db).await? {
        return Err(TaskError::InvalidId);
    }

    let affected_rows = sqlx::query!(
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
    .rows_affected();

    if affected_rows == 1 {
        let task = fetch_task(task_id, user_id, db).await?;
        Ok(task)
    } else {
        Err(TaskError::NotStarted)
    }
}

/// It takes a user_id, a NewTask struct, and a database connection, and returns a Result containing
/// either a MySqlQueryResult or a sqlx::Error
///
/// Arguments:
///
/// * `user_id`: i32 - The user id of the user who created the task.
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

/// It's getting all the tasks for a user and returning them as a `Vec<ResponseTask>`
///
/// Arguments:
///
/// * `user_id`: i32 - The user ID that we're getting tasks for.
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
/// * `db`: &Data<Db> - This is the database connection pool that we created in the main.rs file.
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
