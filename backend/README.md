 
# Crypto Trading Backend

Rust backend for the crypto trading screen, connecting to Binance WebSocket and serving data to the frontend.

## Setup
1. Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
2. Navigate to `backend/`: `cd backend`
3. Build :   cargo build
4. Run :      cargo run

## Dependencies
- tokio: Async runtime
- tokio-tungstenite: WebSocket support
- serde: JSON serialization
- log, env_logger: Logging