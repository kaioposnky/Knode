use std::time;
use tokio_tungstenite::connect_async;
use crate::Message;
use tokio::net::TcpStream;
use tokio_tungstenite::WebSocketStream;
use tokio_tungstenite::MaybeTlsStream;
use futures::stream::SplitStream;
use futures_util::stream::SplitSink;
use futures_util::{SinkExt, StreamExt};

struct WsClient {
    url: String,
    writer:  SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>,
    reader:  SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>
}

impl WsClient {
    pub async fn new(url: String) -> Self {
        let connection = Self::create_websocket_connection(&url).await;

        let (writer, reader) = connection.split();

        WsClient {
            url: url,
            reader: reader,
            writer: writer
        }
    }

    async fn create_websocket_connection(url: &str) -> WebSocketStream<MaybeTlsStream<TcpStream>> {
        loop {
            match connect_async(url).await{
                Ok((connection, _)) => {
                    return connection;
                }

                Err(e) => {
                    match e {
                        tokio_tungstenite::tungstenite::Error::Io(_) => {
                            let _ = tokio::time::sleep(time::Duration::from_secs(2));
                            continue;
                        }

                        tokio_tungstenite::tungstenite::Error::Http(http_err) => {
                            if http_err.status().is_server_error() {
                                let _ = tokio::time::sleep(time::Duration::from_secs(5));
                                continue;
                            }
                        }

                        other_error => panic!("Erro fatal na conexão! {:?}", other_error)
                    }
                }
            }
        }
    }

    pub async fn refresh_connection(&mut self){
        let connection = Self::create_websocket_connection(&self.url).await;

        let (writer, reader) = connection.split();

        self.writer = writer;
        self.reader = reader;
    }

    pub async fn send_message(&mut self, message: Message) {
        self.writer.send(message).await.expect("Failed to send message!");
    }

    pub async fn send_binary(&mut self, binary: Vec<u8>){
        let message = Message::Binary(binary);
        self.send_message(message).await;
    }

    async fn read_message(&mut self) -> Option<Result<String, tokio_tungstenite::tungstenite::Error>>{
        let next = self.reader.next().await;

        match next {

            Some(Ok(msg)) => {
                return Some(Ok(msg.to_string()));
            }

            Some(Err(e)) => {
                return Some(Err(e));
            }

            None => {
                return None;
            }
        }
    }
}