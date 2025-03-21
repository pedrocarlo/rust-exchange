use std::collections::BinaryHeap;
use std::{cmp::Reverse, collections::HashMap};

use crate::{Order, OrderBook, OrderId, OrderSide, Price, Volume};

/// Price Time Priority Order Book Implementation
#[derive(Default)]
struct PTP {
    buy_book: HashMap<OrderId, Order>,
    sell_book: HashMap<OrderId, Order>,
    queue_map: HashMap<(Price, OrderSide), &'static Order>,
    volume_map: HashMap<(Price, OrderSide), Volume>,
    ask_prices: BinaryHeap<OrderId>,
    bid_prices: BinaryHeap<OrderId>,
}

impl OrderBook for PTP {
    fn place_order(&mut self, order: Order) -> bool {
        let (same_book, opposite_book) = match order.side {
            OrderSide::Buy => (&mut self.ask_prices, &mut self.bid_prices),
            OrderSide::Sell => (&mut self.bid_prices, &mut self.ask_prices),
        };

        let mut other_order = opposite_book.peek_mut();

        if other_order.is_none() {
            return false;
        }

        // while order.volume > 0 {}

        // if matches!(order.side, OrderSide::Buy) {
        //     let _ = self.buy_book.insert(order.price, order);
        // } else {
        //     let _ = self.sell_book.insert(order.price, order);
        // }

        true
    }

    fn cancel_order(&mut self, order_id: OrderId) {}

    fn best_ask(&self) -> Option<&Order> {
        match self.ask_prices.peek() {
            Some(id) => self.sell_book.get(id),
            None => None,
        }
    }

    fn best_bid(&self) -> Option<&Order> {
        match self.bid_prices.peek() {
            Some(id) => self.buy_book.get(id),
            None => None,
        }
    }
}
