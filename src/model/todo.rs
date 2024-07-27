use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Serialize, Deserialize, FromRow)]
pub struct Todo {
    id: i32,
    content: String,
    completed_on: chrono::DateTime<chrono::Utc>,
}

pub async fn get_todo_list(pool: &PgPool) -> Vec<Todo> {
    sqlx::query_as("SELECT id, content, completed_on FROM todos")
        .fetch_all(pool)
        .await
        .unwrap()
}
