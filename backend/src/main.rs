use crypto_trading_backend::{binance::connect_binance, models::DataStore, server::start_server};
use std::sync::Arc;
use tokio::sync::Mutex;
use env_logger;
use log::{info, debug, error};
use std::env; 

#[tokio::main]
async fn main() {
    
    if env::var("RUST_LOG").is_err() {
        env::set_var("RUST_LOG", "info");
    }

    env_logger::init();

    debug!("This is a debug message");
    info!("This is an info message");
    error!("This is an error message");

    let data_store = Arc::new(Mutex::new(DataStore::new()));

    let data_store_clone = data_store.clone();
    tokio::spawn(async move {
        connect_binance(data_store_clone).await;
    });

    start_server(data_store).await;
}
