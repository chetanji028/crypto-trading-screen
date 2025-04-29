 
# Crypto Trading Screen

A desktop application replicating a professional crypto trading interface, with a Rust backend for low-latency WebSocket processing and an Electron + React frontend for a rich UI.

## Features
- Live BTC/USDT price feed via Binance WebSocket
- Candlestick chart (using Recharts)
- Order book with top 10 bids and asks
- Recent trade history (last 100 trades)
- Ticker panel for other coins (bonus)
- Buy button (mock functionality)
- Desktop notifications for large trades (>10 BTC, bonus)

## Setup
1. **Backend**:
   - Navigate to `backend/`: `cd backend`
   - Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
   - Build: `cargo build`
    Run: `cargo run`
2. **Frontend**:
   - Navigate to `frontend/`: `cd frontend`
   - Install Node.js: https://nodejs.org/
   - Install dependencies: `npm install`
   - Run: `npm run dev`

## Architecture
- **Backend**: Rust with Tokio and tokio-tungstenite for WebSocket handling. Connects to Binance API, maintains in-memory data, and serves a local WebSocket at `ws://127.0.0.1:8080`.
- **Frontend**: Electron + React, using Recharts for charting. Connects to the backend WebSocket for live updates.

## Notes
- Ensure the backend is running before starting the frontend.
- The UI is styled to resemble modern trading apps (dark theme, clean layout).
- The candlestick chart uses a simplified area chart for demonstration; a proper candlestick library like `lightweight-charts` can be used for production.