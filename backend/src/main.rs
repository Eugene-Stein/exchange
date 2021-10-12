use exchange::common::{Order, Side};
use exchange::{match_engine, order_server};
use log::{debug, error, info};
use rust_decimal_macros::dec;
use std::sync::mpsc::sync_channel;
use std::thread;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;
fn main() {
    env_logger::init();
    info!("starting...");
    let (order_channel_transmit, order_channel_receive) = sync_channel(5);
    let match_engine_thread = thread::spawn(move || match_engine::start(order_channel_receive));
    let order_server_thread = thread::spawn(move || order_server::start(order_channel_transmit));
    info!("started.");
    order_server_thread.join().unwrap();
    match_engine_thread.join().unwrap();
    info!("exit");
}
