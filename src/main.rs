mod order_matcher;
mod types;

use std::io::{self, BufRead};
use std::collections::BTreeMap;
use std::sync::mpsc::{self, Sender, Receiver};
use std::thread;

use crate::types::order_types::OrderBook;
use crate::types::order_types::Order;
use crate::order_matcher::matcher::process_orders;

fn main() {
    let stdin = io::stdin();
    let (order_tx, order_rx): (Sender<Order>, Receiver<Order>) = mpsc::channel();

    let processor_handle = thread::spawn(move || {
        let mut sell_orders: OrderBook = BTreeMap::new();
        process_orders(order_rx, &mut sell_orders);
    });

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        if line.trim().is_empty() {
            continue;
        }

        let order = match line.parse::<Order>() {
            Ok(order) => order,
            Err(e) => {
                eprintln!("Error parsing order: {}", e);
                continue;
            }
        };

        if let Err(e) = order_tx.send(order) {
            eprintln!("Error sending order to processor: {}", e);
            break;
        }
    }

    drop(order_tx);

    processor_handle.join().expect("Processor thread panicked");
}

