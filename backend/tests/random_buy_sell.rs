#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {
    use exchange::common::{Order, Side};
    use exchange::match_engine::OrderBook;
    use rand::Rng;
    use rust_decimal::prelude::*;
    use rust_decimal_macros::dec;
    use std::collections::BinaryHeap;
    use test::Bencher;

    #[bench]
    fn bench_random(b: &mut Bencher) {
        let mut rng = rand::thread_rng();
        let mut orderbook = OrderBook {
            symbol: String::from("BTC/USDT"),
            base: String::from("BTC"),
            quote: String::from("USDT"),
            sells: BinaryHeap::new(),
            buys: BinaryHeap::new(),
        };
        b.iter(|| {
            (0..1000000).fold(0, |old, new| {
                let order = Order {
                    side: SideVal::Buy,
                    price: Decimal::new(rng.gen_range(1..10000), 2),
                    amount: Decimal::new(rng.gen_range(1..1000), 2),
                };
                orderbook.put_order(order);
                orderbook.buys.len()
            })
        });
    }

    #[test]
    fn it_works() {
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
}
