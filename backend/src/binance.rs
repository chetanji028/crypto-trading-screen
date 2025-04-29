use crate::models::{BinanceDepth, BinanceTrade, DataStore, Trade};
use futures::{StreamExt,SinkExt};
use log::{error, info};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use url::Url;
use std::sync::Arc;

pub async fn connect_binance(data_store: Arc<Mutex<DataStore>>) {
    let url = Url::parse("wss://stream.binance.com:9443/ws").unwrap();
    loop {
        match connect_async(url.to_string()).await {
            Ok((ws_stream, _)) => {
                info!("Connected to Binance WebSocket");
                let (mut write, mut read) = ws_stream.split();
                // Send subscription message
                let subscription = r#"{
                    "method": "SUBSCRIBE",
                    "params": ["btcusdt@trade", "btcusdt@depth"],
                    "id": 1
                }"#;
                if let Err(e) = write.send(Message::Text(subscription.to_string())).await {
                    error!("Failed to send subscription: {}", e);
                    continue;
                }
                info!("Subscribed to btcusdt@trade and btcusdt@depth");
                while let Some(message) = read.next().await {
                    match message {
                        Ok(Message::Text(text)) => {
                            let mut store = data_store.lock().await;
                            if let Ok(trade) = serde_json::from_str::<BinanceTrade>(&text) {
                                let trade = Trade {
                                    price: trade.price.parse().unwrap_or(0.0),
                                    quantity: trade.quantity.parse().unwrap_or(0.0),
                                    time: trade.time,
                                };
                                store.add_trade(trade.clone());
                               info!("Processed trade: {:?}", trade);
                            } else if let Ok(depth) = serde_json::from_str::<BinanceDepth>(&text) {
                                let bids: Vec<_> = depth
                                    .bids
                                    .into_iter()
                                    .map(|(p, q)| (p.parse().unwrap_or(0.0), q.parse().unwrap_or(0.0)))
                                    .collect();
                                let asks: Vec<_> = depth
                                    .asks
                                    .into_iter()
                                    .map(|(p, q)| (p.parse().unwrap_or(0.0), q.parse().unwrap_or(0.0)))
                                    .collect();
                                    store.update_order_book(bids.clone(), asks.clone());
                                    info!("Updated order book: {} bids, {} asks", bids.len(), asks.len());
                            } else {
                                info!("Received unhandled message: {}", text);
                            }
                        }
                        Ok(_) => {}
                        Err(e) => {
                            error!("WebSocket error: {}", e);
                            break;
                        }
                    }
                }
            }
            Err(e) => {
                error!("Failed to connect to Binance: {}. Retrying in 5 seconds...", e);
                tokio::time::sleep(std::time::Duration::from_secs(5)).await;
            }
        }
    }
}