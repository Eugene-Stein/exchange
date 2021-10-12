extern crate rmp_serde as rmps;
extern crate serde;
extern crate serde_derive;

use bytes::Bytes;
use rmps::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, io::Cursor, sync::mpsc::SyncSender};

use crate::common::Order;

pub fn handle(order_channel: &SyncSender<Order>, bin: Bytes) {
    let cur = Cursor::new(bin);
    let mut des = Deserializer::new(cur);
    let order: Order = Deserialize::deserialize(&mut des).unwrap();
    order_channel.send(order).unwrap();
}
