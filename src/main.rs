use pb::action_server::ActionServer;
use tonic::transport::Server;

mod db;
mod model;
mod handler;
mod pb {
    tonic::include_proto!("todo");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let pool = db::establish_connection().await?;

    let todo_service = handler::TodoService::new(pool.clone());

    let addr = "127.0.0.1:50051".parse().unwrap();
    Server::builder()
        .add_service(ActionServer::new(todo_service))
        .serve(addr)
        .await?;

    Ok(())
}
