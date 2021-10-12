extern crate serde;
use super::Order;
use rust_decimal::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Event(u8);
pub mod EventVal {
    use super::Event;
    pub const Fullfilled: Event = Event(0);
    pub const Canceled: Event = Event(1);
    pub const Rejected: Event = Event(2);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderEvent {
    pub order: Order,
    pub event: Event,
    pub timestamp: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tick {
    // 开盘价，必要字段
    open: Decimal,
    // 收盘价，必要字段
    close: Decimal,
    // 最高价，必要字段
    high: Decimal,
    // 最低价，必要字段
    low: Decimal,
    // 成交量，非必须字段
    volume: Decimal,
    // 成交额，非必须字段，如果需要展示技术指标'EMV'和'AVP'，则需要为该字段填充数据。
    turnover: Decimal,
    // 时间戳，毫秒级别，必要字段
    timestamp: u32,
}
