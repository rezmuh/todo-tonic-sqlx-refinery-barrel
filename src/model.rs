use sqlx::postgres::PgPool;
use crate::pb::TodoResponse;

#[derive(sqlx::FromRow, Debug)]
pub struct Todo {
    pub id: i32, // By default, using barrel's types::primary() results in i32
    pub title: String,
    pub is_completed: bool,
}

type DBResult<T> = Result<T, Box<dyn std::error::Error>>;

// TODO: cleanup and re-format query
impl Todo {

    pub async fn add(pool: &PgPool, title: String) -> DBResult<TodoResponse> {
        let todo = sqlx::query_as!(Todo, "INSERT INTO todos (title) VALUES ( $1 ) RETURNING id, title, is_completed", title)
            .fetch_one(pool)
            .await?;
        Ok(todo.into_response())
    }

    // TODO: Is it a heavy operation to convert Todo into TodoResponse?
    pub async fn all(pool: &PgPool) -> DBResult<Vec<TodoResponse>> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT id, title, is_completed FROM todos ORDER by id")
            .fetch_all(pool)
            .await?;
        let todo_responses = todos.iter().map(|t| t.into_response()).collect();
        Ok(todo_responses)
    }

    // TODO: Is it a heavy operation to convert Todo into TodoResponse?
    pub async fn incomplete(pool: &PgPool) -> DBResult<Vec<TodoResponse>> {
        let todos: Vec<Todo> = sqlx::query_as!(Todo, "SELECT id, title, is_completed FROM todos WHERE is_completed = false ORDER by id")
            .fetch_all(pool)
            .await?;
        let todo_responses = todos.iter().map(|t| t.into_response()).collect();
        Ok(todo_responses)

    }

    pub async fn get(pool: &PgPool, id: i32) -> DBResult<TodoResponse> {
        let todo = sqlx::query_as!(Todo, "SELECT id, title, is_completed from todos WHERE id = $1", id)
            .fetch_one(pool)
            .await?;
        Ok(todo.into_response())
    }

    pub async fn mark_complete(pool: &PgPool, id: i32) -> DBResult<TodoResponse> {
        let todo = sqlx::query_as!(Todo, "UPDATE todos set is_completed = true where id = $1 RETURNING id, title, is_completed", id)
            .fetch_one(pool)
            .await?;
        Ok(todo.into_response())

    }

    // Mapping Todo struct with TodoResponse (which is used to display data in GRPC)
    // TODO: Is there a better way to do it?
    fn into_response(&self) -> TodoResponse {
        TodoResponse {
            id: self.id,
            title: self.title.clone(),
            is_completed: self.is_completed
        }

    }
}
