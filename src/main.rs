use futures_util::{SinkExt, StreamExt};
use tokio_tungstenite::{connect_async};
use tokio_tungstenite::tungstenite::Message;

mod domain;
mod infra;
mod config;
mod services;


#[tokio::main]
async fn main() {
    let url = "wss://echo.websocket.org";

    println!("Connecting to, {}", url);
    let (ws_stream  , _) = connect_async(url).await.expect("Failed to connect");


    println!("Connected to Agent Network!");

    let (mut write, mut read) = ws_stream.split();

    let msg = Message::Text("Hello websocket!".into());

    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read message");

        println!("Received a message: {}", message);
    }

    write.send(msg).await.expect("Failed to send message...");

    if let Some(message) = read.next().await{
        let message = message.expect("Failed to read message");

        println!("Received a message: {}", message);
    }
}

