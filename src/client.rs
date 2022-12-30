use chat::chat_client::ChatClient;
use chat::ChatRequest;
mod chat {
    tonic::include_proto!("chat");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:50051")
        .connect()
        .await?;

    let mut client = ChatClient::new(channel);
    let request = tonic::Request::new(
        ChatRequest {
            username:String::from("armstrong"),
            content:String::from("Armstrong's Message Content")
        }
    );
    
    let mut response = client.send_stream(request).await?.into_inner();
    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    Ok(())
}

