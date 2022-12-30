use chat::chat_server::{Chat, ChatServer};
use chat::{ChatRequest, ChatResponse};
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};
mod chat {
    tonic::include_proto!("chat");
}

#[derive(Default)]
pub struct MyChat {}

#[tonic::async_trait]
impl Chat for MyChat {
    type BidirectionalStream = mpsc::Receiver<Result<ChatResponse, Status>>;
    async fn bidirectional(
        &self,
        request: Request<tonic::Streaming<ChatRequest>>,
    ) -> Result<Response<Self::BidirectionalStream>, Status> {
        let mut streamer = request.into_inner();
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            while let Some(req) = streamer.message().await.unwrap() {
                tx.send(Ok(ChatResponse {
                    username: String::from("Armstrong"),
                    content: format!("hello {}", req.username),
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
    }

    async fn receive_stream(
        &self,
        request: Request<tonic::Streaming<ChatRequest>>,
    ) -> Result<Response<ChatResponse>, Status> {
        let mut stream = request.into_inner();
        let mut content = String::from("");
        while let Some(req) = stream.message().await? {
            content.push_str(&format!("Hello {}\n", req.username))
        }
        Ok(Response::new(ChatResponse {
            content,
            username: String::from("Armstrong"),
        }))
    }

    type SendStreamStream = mpsc::Receiver<Result<ChatResponse, Status>>;

    // Spawn an asynchronous task and then return the receiver
    async fn send_stream(
        &self,
        request: Request<ChatRequest>,
    ) -> Result<Response<Self::SendStreamStream>, Status> {
        let (mut tx, rx) = mpsc::channel(4);
        tokio::spawn(async move {
            for i in 0..4 {
                tx.send(Ok(ChatResponse {
                    username: String::from("Armstrong"),
                    content: format!("Content number: {}", i),
                }))
                .await;
            }
        });
        Ok(Response::new(rx))
    }

    async fn send(&self, request: Request<ChatRequest>) -> Result<Response<ChatResponse>, Status> {
        Ok(Response::new(ChatResponse {
            username: format!("hello {}", request.get_ref().username),
            content: String::from("Insert Content Here"),
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
