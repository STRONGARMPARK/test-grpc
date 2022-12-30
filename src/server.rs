use tokio::sync::mpsc;
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
    type SendStreamStream=mpsc::Receiver<Result<ChatResponse,Status>>;
    // Spawn an asynchronous task and then return the receiver
    async fn send_stream(&self, request: Request<ChatRequest>) -> Result<Response<Self::SendStreamStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for i in 0..4{
                tx.send(Ok(ChatResponse {
                    username:String::from("armstrong"),
                    content:format!("Content number: {}", i)
                })).await;
            }
        });
        Ok(Response::new(rx))
    }

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

