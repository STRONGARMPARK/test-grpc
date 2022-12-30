use chat::chat_server::{Chat, ChatServer};
use chat::{ChatRequest, ChatResponse};
use tonic::{transport::Server, Request, Response, Status};
mod chat {
    tonic::include_proto!("chat");
}


#[derive(Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {
    async fn send(&self, request: Request<ChatRequest>) -> Result<Response<ChatResponse>, Status> {
        Ok(Response::new(ChatResponse {
            username: format!("hello {}", request.get_ref().username),
            content: String::from("Insert Content Here")
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let chat = MyChat::default();
    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(ChatServer::new(chat))
        .serve(addr)
        .await?;
    Ok(())
}

