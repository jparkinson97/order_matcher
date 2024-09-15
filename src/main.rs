mod order_matcher;
mod types;

use std::io::{self, BufRead};
use std::collections::BTreeMap;

use crate::types::order_types::OrderBook;
use crate::types::order_types::OrderType;
use crate::types::order_types::Order;
use crate::order_matcher::matcher::register_sell_order;
use crate::order_matcher::matcher::process_buy_order;


fn main() {
    let stdin = io::stdin();
    let mut sell_orders: OrderBook = BTreeMap::new();

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            continue;
        }

        let mut order = match line.parse::<Order>() {
            Ok(order) => order,
            Err(e) => {
                eprintln!("Error parsing order: {}", e);
                continue;
            }
        };

        match order.order_type {
            OrderType::Sell => {
                register_sell_order(order, &mut sell_orders);
            }
            OrderType::Buy => {
                let trades = process_buy_order(&mut order, &mut sell_orders);
                for trade in trades {
                    println!(
                        "Trade: {} BTC @ {} USD between {} and {}",
                        trade.quantity_traded, trade.price, trade.buy_id, trade.sell_id
                    );
                }
            }
        }
    }
}
