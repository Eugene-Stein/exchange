mod orderbook;
use crate::common::Order;
use log::{debug, error, info};
pub use orderbook::OrderBook;
use std::collections::BinaryHeap;
use std::sync::mpsc::Receiver;

pub fn start(order_channel: Receiver<Order>) {
    info!("starting...");
    let mut btc_usdt_orderbook = OrderBook {
        symbol: String::from("BTC/USDT"),
        base: String::from("BTC"),
        quote: String::from("USDT"),
        sells: BinaryHeap::new(),
        buys: BinaryHeap::new(),
    };
    info!("started.");
    loop {
        let order = order_channel.recv().unwrap();
        info!("get order: {:?}", order);
        btc_usdt_orderbook.put_order(order);
    }
}
