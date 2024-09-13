
pub mod matcher {
    use std::collections::VecDeque;
    use crate::types::order_types::Order;
    use crate::types::order_types::OrderBook;

    pub fn process_buy_order(buy_order: &mut Order, sell_orders: &mut OrderBook) {
        let mut prices_to_remove = Vec::new();
    
        for (&sell_price, orders_at_price) in sell_orders.range_mut(..=buy_order.price) {
            while let Some(sell_order) = orders_at_price.front_mut() {
                let traded_quantity = buy_order.quantity.min(sell_order.quantity);
    
                // Emit trade message
                println!(
                    "Trade: {} BTC @ {} USD between {} and {}",
                    traded_quantity, sell_price, buy_order.id, sell_order.id
                );
    
                // Reduce quantities
                buy_order.quantity -= traded_quantity;
                sell_order.quantity -= traded_quantity;
    
                // Remove sell order if quantity is zero
                if sell_order.quantity == 0 {
                    orders_at_price.pop_front();
                }
    
                // Break if buy order is fully matched
                if buy_order.quantity == 0 {
                    break;
                }
            }
    
            // Mark price level for removal if no orders remain
            if orders_at_price.is_empty() {
                prices_to_remove.push(sell_price);
            }
    
            // Break if buy order is fully matched
            if buy_order.quantity == 0 {
                break;
            }
        }
    
        // Clean up empty price levels
        for price in prices_to_remove {
            sell_orders.remove(&price);
        }
    }
    
    pub fn register_sell_order(sell_order: Order, sell_orders: &mut OrderBook) {
        sell_orders
            .entry(sell_order.price)
            .or_insert_with(VecDeque::new)
            .push_back(sell_order);
    }
}