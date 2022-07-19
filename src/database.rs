use crate::hashing::hash;

use crate::model::{DBTask, Db, Login, Task, User};

use rocket::State;

use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlQueryResult;
use sqlx::Error;

use std::result::Result;

pub async fn connect(url: &str) -> Result<MySqlPool, Error> {
    let pool = MySqlPool::connect(&url).await;
    pool
}

pub async fn insert_new_user(user: User, db: &State<Db>) -> Result<MySqlQueryResult, sqlx::Error> {
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
    db: &State<Db>,
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

pub async fn get_tasks_by_user(user_id: i32, db: &State<Db>) -> Result<Vec<Task>, sqlx::Error> {
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

pub async fn verify_password(user: Login, db: &State<Db>) -> bool {
    let users = sqlx::query!(
        r#"
		SELECT * FROM users WHERE username = ? AND password = ?"#,
        user.username,
        hash(&user.password)
    )
    .fetch_all(&db.pool)
    .await;
    return users.unwrap().len() > 0;
}
