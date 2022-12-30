use futures::stream::iter;
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
    let request = tonic::Request::new(iter(vec![
        ChatRequest {
            username: String::from("Rohan"),
            content: String::from("Rohan Content")
        },
        ChatRequest {
            username: String::from("Marcos"),
            content: String::from("Marcos Content")
        },
        ChatRequest {
            username: String::from("Armstrong"),
            content: String::from("Armstrong Content")
        }
    ]));
    // sending stream
    let response = client.receive_stream(request).await?.into_inner();
    println!("Response=\n{:?}", response);

    let request = tonic::Request::new(ChatRequest {
        username: String::from("armstrong"),
        content: String::from("Armstrong's Message Content"),
    });

    let mut response = client.send_stream(request).await?.into_inner();

    while let Some(res) = response.message().await? {
        println!("NOTE = {:?}", res);
    }
    Ok(())
}

    // // sending stream
    //     let response = client.receive_stream(request).await?.into_inner();
    //     println!("RESPONSE=\n{}", response.message);
    //     Ok(())
    // }
