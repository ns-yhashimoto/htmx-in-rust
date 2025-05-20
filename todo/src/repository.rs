use crate::model::{Todo, TodoRepository, TodoResult};

pub struct PostgresTodoRepository {
    pool: sqlx::PgPool,
}

impl PostgresTodoRepository {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl TodoRepository for PostgresTodoRepository {
    async fn get_list(&self) -> TodoResult<Vec<Todo>> {
        sqlx::query_as("SELECT id, content, completed_on FROM todos ORDER BY id DESC")
            .fetch_all(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
    async fn get(&self, id: &i32) -> TodoResult<Todo> {
        sqlx::query_as("SELECT id, content, completed_on FROM todos WHERE id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
    async fn create(&self, content: &String) -> TodoResult<Todo> {
        sqlx::query_as("INSERT INTO todos(content) VALUES ($1) RETURNING id, content, completed_on")
            .bind(content)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
    async fn update(&self, todo: &Todo) -> TodoResult<Todo> {
        sqlx::query_as::<_, Todo>("UPDATE todos SET content = $2, completed_on = $3 WHERE id = $1 RETURNING id, content, completed_on")
        .bind(todo.id)
        .bind(&todo.content)
        .bind(todo.completed_on)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())
    }
    async fn delete(&self, id: &i32) -> TodoResult<i32> {
        sqlx::query_scalar::<_, i32>("DELETE FROM todos WHERE id = $1 RETURNING id")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| e.to_string())
    }
}
