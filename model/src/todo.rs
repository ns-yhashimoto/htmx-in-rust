use chrono::Utc;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

pub type TodoError = String;
pub type TodoResult<T> = Result<T, TodoError>;

#[async_trait::async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    async fn get_list(&self) -> TodoResult<Vec<Todo>>;
    async fn get(&self, id: &i32) -> TodoResult<Todo>;
    async fn create(&self, content: &String) -> TodoResult<Todo>;
    async fn update(&self, todo: &Todo) -> TodoResult<Todo>;
    async fn delete(&self, id: &i32) -> TodoResult<i32>;
}

#[derive(Serialize, Deserialize, FromRow, Clone)]
pub struct Todo {
    pub id: i32,
    pub content: String,
    pub completed_on: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn get(repos: &impl TodoRepository, id: &i32) -> TodoResult<Todo> {
    repos.get(id).await
}

pub async fn get_list(repos: &impl TodoRepository) -> TodoResult<Vec<Todo>> {
    repos.get_list().await
}

pub async fn create(repos: &impl TodoRepository, content: &String) -> TodoResult<Todo> {
    repos.create(content).await
}

pub async fn update_as_done(repos: &impl TodoRepository, id: &i32) -> TodoResult<Todo> {
    let mut todo = repos.get(id).await?;
    todo.completed_on = Some(Utc::now());

    repos.update(&todo).await
}

pub async fn update(repos: &impl TodoRepository, todo: &Todo) -> TodoResult<Todo> {
    repos.update(&todo).await
}

pub async fn delete(repos: &impl TodoRepository, id: &i32) -> TodoResult<i32> {
    repos.delete(id).await
}

#[cfg(test)]
mod tests {

    use crate::todo;

    use super::{Todo, TodoRepository, TodoResult};

    struct MockTodoRepository {}

    #[async_trait::async_trait]
    impl TodoRepository for MockTodoRepository {
        async fn get_list(&self) -> TodoResult<Vec<Todo>> {
            Ok(vec![Todo {
                id: 1,
                content: String::from("mock"),
                completed_on: None,
            }])
        }
        async fn get(&self, id: &i32) -> TodoResult<Todo> {
            Ok(Todo {
                id: *id,
                content: String::from("mock"),
                completed_on: None,
            })
        }
        async fn create(&self, content: &String) -> TodoResult<Todo> {
            Ok(Todo {
                id: 1,
                content: content.clone(),
                completed_on: None,
            })
        }
        async fn update(&self, todo: &Todo) -> TodoResult<Todo> {
            Ok(todo.clone())
        }
        async fn delete(&self, id: &i32) -> TodoResult<i32> {
            Ok(id.clone())
        }
    }

    #[tokio::test]
    pub async fn get_list_works() {
        let repos = MockTodoRepository {};
        let result = todo::get_list(&repos).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    pub async fn get_works() {
        let repos = MockTodoRepository {};
        let result = todo::get(&repos, &1).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    pub async fn create_works() {
        let repos = MockTodoRepository {};
        let content = &String::from("hoge");
        let result = todo::create(&repos, content).await.unwrap();
        assert_eq!(&result.content, content);
    }

    #[tokio::test]
    pub async fn update_as_done_works() {
        let repos = MockTodoRepository {};
        let result = todo::update_as_done(&repos, &1).await;
        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(Option::is_some(&result.completed_on));
    }

    #[tokio::test]
    pub async fn delete_works() {
        let repos = MockTodoRepository {};
        let result = todo::delete(&repos, &1).await;
        assert!(Result::is_ok(&result));
    }
}
