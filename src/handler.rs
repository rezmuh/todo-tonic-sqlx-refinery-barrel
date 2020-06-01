use sqlx::postgres::PgPool;
use tonic::{Request, Response, Status};

use crate::model::Todo;
use crate::pb::{
    action_server::Action,
    EmptyRequest,
    AddTodoRequest,
    TodoRequest,
    TodoResponse,
    TodosResponse
};

pub struct TodoService {
    pool: PgPool
}

impl TodoService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool
        }
    }
}

type ServiceResponse<T> = Result<Response<T>, Status>;

 #[tonic::async_trait]
impl Action for TodoService {

    async fn add_todo(&self, _req: Request<AddTodoRequest>) -> ServiceResponse<TodoResponse> {
        let todo = Todo::add(&self.pool, _req.into_inner().title).await.unwrap();
        Ok(Response::new(todo))
    }

    async fn all_todos(&self, _req: Request<EmptyRequest>) -> ServiceResponse<TodosResponse> {
        let todos = Todo::all(&self.pool).await.unwrap();
        Ok(Response::new(TodosResponse { todos }))
    }

    async fn incomplete(&self, _req: Request<EmptyRequest>) -> ServiceResponse<TodosResponse> {
        let todos = Todo::incomplete(&self.pool).await.unwrap();
        Ok(Response::new(TodosResponse { todos }))
    }

    async fn get_todo(&self, req: Request<TodoRequest>) -> ServiceResponse<TodoResponse> {
        let res = Todo::get(&self.pool, req.into_inner().id).await;

        match res {
            Ok(todo) => Ok(Response::new(todo)),
            Err(_err) => {
                Err(Status::new(tonic::Code::NotFound, "Object not found".to_string()))
            }
        }
    }

    async fn mark_complete(&self, req: Request<TodoRequest>) -> ServiceResponse<TodoResponse> {
        let todo = Todo::mark_complete(&self.pool, req.into_inner().id).await.unwrap();
        Ok(Response::new(todo))

    }
}
