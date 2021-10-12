use crate::common::SideVal;
use crate::common::{Order, Side};
use log::info;
use rust_decimal::prelude::*;
use std::cmp::Ordering;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

pub struct OrderBook {
    pub symbol: String,
    pub base: String,
    pub quote: String,
    pub sells: BinaryHeap<Reverse<Order>>, // min heap, need reverse wrap
    pub buys: BinaryHeap<Order>,           // max heap
}

impl OrderBook {
    pub fn buy(&mut self, mut order: Order) {
        loop {
            if let Some(sell) = self.sells.peek() {
                if order.price >= sell.0.price {
                    if let Some(mut sell) = self.sells.pop() {
                        if sell.0.amount >= order.amount {
                            info!(
                                "buy {} {} with price {}",
                                order.amount, self.base, order.price
                            );
                            sell.0.amount -= order.amount;
                            self.sells.push(sell);
                            return;
                        } else {
                            info!(
                                "buy {} {} with price {}",
                                sell.0.amount, self.base, order.price
                            );
                            order.amount -= sell.0.amount;
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.buys.push(order)
    }
    pub fn sell(&mut self, mut order: Order) {
        loop {
            if let Some(buy) = self.buys.peek() {
                if buy.price >= order.price {
                    if let Some(mut buy) = self.buys.pop() {
                        if buy.amount >= order.amount {
                            info!(
                                "sell {} {} with price {}",
                                order.amount, self.base, buy.price
                            );
                            buy.amount -= order.amount;
                            self.buys.push(buy);
                            return;
                        } else {
                            info!("sell {} {} with price {}", buy.amount, self.base, buy.price);
                            order.amount -= buy.amount;
                            break;
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        self.sells.push(Reverse(order))
    }
    pub fn put_order(&mut self, order: Order) {
        match order.side {
            SideVal::Buy => self.buy(order),
            SideVal::Sell => self.sell(order),
            _ => (),
        }
    }
}
