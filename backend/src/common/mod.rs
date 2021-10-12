use rust_decimal::prelude::*;
use std::cmp::Ordering;
extern crate serde;
use serde_derive::{Deserialize, Serialize};

pub mod events;
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Side(u8);
pub mod SideVal {
    use super::Side;
    pub const Buy: Side = Side(0);
    pub const Sell: Side = Side(1);
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Order {
    pub side: Side,
    pub amount: Decimal,
    pub price: Decimal,
}
impl Ord for Order {
    fn cmp(&self, other: &Self) -> Ordering {
        self.price.cmp(&other.price)
    }
}

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Order {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for Order {}
