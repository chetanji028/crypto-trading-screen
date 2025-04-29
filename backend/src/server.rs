 
use crate::models::DataStore;
use futures:: SinkExt;
use log::{error, info};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;


pub async fn start_server(data_store: Arc<Mutex<DataStore>>) {
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("Local WebSocket server running on ws://127.0.0.1:8080");

    while let Ok((stream, _)) = listener.accept().await {
        let data_store = data_store.clone();
        tokio::spawn(async move {
            match accept_async(stream).await {
                Ok(mut ws_stream) => {
                    info!("New frontend connection established");
                    loop {
                        let payload = {
                            let store = data_store.lock().await;
                            store.to_payload()
                        };
                        let message = serde_json::to_string(&payload).unwrap();
                        if ws_stream.send(Message::Text(message)).await.is_err() {
                            error!("Failed to send message to frontend");
                            break;
                        }
                        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                    }
                }
                Err(e) => {
                    error!("Failed to accept WebSocket connection: {}", e);
                }
            }
        });
    }
}