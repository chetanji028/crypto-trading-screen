 
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Trade {
    pub price: f64,
    pub quantity: f64,
    pub time: u64,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderBook {
    pub bids: Vec<(f64, f64)>, // (price, quantity)
    pub asks: Vec<(f64, f64)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceTrade {
    #[serde(rename = "p")]
    pub price: String,
    #[serde(rename = "q")]
    pub quantity: String,
    #[serde(rename = "T")]
    pub time: u64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BinanceDepth {
    #[serde(rename = "b")]
    pub bids: Vec<(String, String)>,
    #[serde(rename = "a")]
    pub asks: Vec<(String, String)>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPayload {
    pub trades: Vec<Trade>,
    pub order_book: OrderBook,
}

pub struct DataStore {
    pub trades: VecDeque<Trade>,
    pub order_book: OrderBook,
}

impl DataStore {
    pub fn new() -> Self {
        DataStore {
            trades: VecDeque::with_capacity(100),
            order_book: OrderBook {
                bids: vec![],
                asks: vec![],
            },
        }
    }

    pub fn add_trade(&mut self, trade: Trade) {
        if self.trades.len() >= 100 {
            self.trades.pop_front();
        }
        self.trades.push_back(trade);
    }

    pub fn update_order_book(&mut self, bids: Vec<(f64, f64)>, asks: Vec<(f64, f64)>) {
        self.order_book.bids = bids.into_iter().take(10).collect();
        self.order_book.asks = asks.into_iter().take(10).collect();
    }

    pub fn to_payload(&self) -> DataPayload {
        DataPayload {
            trades: self.trades.iter().cloned().collect(),
            order_book: self.order_book.clone(),
        }
    }
}