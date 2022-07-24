use crate::hashing::hash;

use crate::model::{DBTask, DBUser, Db, Login, Task, User};

use actix_web::web::Data;

use sqlx::mysql::{MySqlPool, MySqlQueryResult};
use sqlx::Error;

use std::result::Result;

pub async fn connect(url: &str) -> Result<MySqlPool, Error> {
    let pool = MySqlPool::connect(&url).await;
    pool
}

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

pub async fn add_task(
    user_id: i32,
    task: Task,
    db: &Data<Db>,
) -> Result<MySqlQueryResult, sqlx::Error> {
    sqlx::query!(
        r#"
		INSERT INTO tasks (user_id, name, description, done)
			VALUES ( ?, ?, ?, ? )
			"#,
        user_id,
        task.name,
        task.description,
        task.done
    )
    .execute(&db.pool)
    .await
}

pub async fn get_tasks_by_user(user_id: i32, db: &Data<Db>) -> Result<Vec<Task>, sqlx::Error> {
    let tasks = sqlx::query_as!(
        DBTask,
        r#"
		SELECT id, name, description, done FROM tasks WHERE user_id = ?"#,
        user_id,
    )
    .fetch_all(&db.pool)
    .await?;

    let tasks = tasks
        .into_iter()
        .map(|task| Task {
            name: task.name,
            description: task.description,
            done: task.done != 0,
        })
        .collect();

    Ok(tasks)
}

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
