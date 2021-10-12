extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use exchange::common::{
    events::{Event, EventVal, OrderEvent},
    Order, Side, SideVal,
};
use rmps::{Deserializer, Serializer};
use rust_decimal_macros::dec;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor};

fn main() {
    let mut buf = Vec::new();
    let buy = Order {
        side: SideVal::Buy,
        amount: dec!(3.4),
        price: dec!(49000),
    };
    let event = OrderEvent {
        order: buy,
        event: EventVal::Fullfilled,
        timestamp: 1234,
    };
    event.serialize(&mut Serializer::new(&mut buf)).unwrap();
    println!("{:?}", buf);
    let cur = Cursor::new(&buf[..]);
    let mut des = Deserializer::new(cur);
    let recovered: OrderEvent = Deserialize::deserialize(&mut des).unwrap();
    println!("{:?}", recovered);
    buf.clear();

    // Orders
    // let buy = Order {
    //     side: SideVal::Buy,
    //     amount: dec!(3.4),
    //     price: dec!(49000),
    // };
    // let sell = Order {
    //     side: SideVal::Sell,
    //     amount: dec!(3.4),
    //     price: dec!(49000),
    // };

    // buy.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // println!("{:?}", buf);
    // let cur = Cursor::new(&buf[..]);
    // let mut des = Deserializer::new(cur);
    // let buy_recover: Order = Deserialize::deserialize(&mut des).unwrap();
    // println!("{:?}", buy_recover);
    // buf.clear();
    // sell.serialize(&mut Serializer::new(&mut buf)).unwrap();
    // println!("{:?}", buf);
    // let cur = Cursor::new(&buf[..]);
    // let mut des = Deserializer::new(cur);
    // let sell_recover: Order = Deserialize::deserialize(&mut des).unwrap();
    // println!("{:?}", sell_recover);
}
