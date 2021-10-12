use exchange::common::*;
use exchange::match_engine::OrderBook;
use log::{debug, error, info};
use rust_decimal_macros::dec;
use std::collections::BinaryHeap;
fn main() {
    info!("started.");
    let mut btc_usdt_orderbook = OrderBook {
        symbol: String::from("BTC/USDT"),
        base: String::from("BTC"),
        quote: String::from("USDT"),
        sells: BinaryHeap::new(),
        buys: BinaryHeap::new(),
    };
    let buy_order = Order {
        side: SideVal::Buy,
        price: dec!(49000.56),
        amount: dec!(1.7),
    };
    btc_usdt_orderbook.buy(buy_order);
    let sell_order = Order {
        side: SideVal::Sell,
        price: dec!(49000),
        amount: dec!(2),
    };
    btc_usdt_orderbook.sell(sell_order);
}
