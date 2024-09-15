
pub mod matcher {
    use std::collections::VecDeque;
    use crate::types::order_types::Order;
    use crate::types::order_types::OrderBook;
    use crate::types::order_types::Trade;


    pub fn process_buy_order(buy_order: &mut Order, sell_orders: &mut OrderBook) -> Vec<Trade> {
        let mut prices_to_remove = Vec::new();
        let mut trades = Vec::new();
    
        for (&sell_price, orders_at_price) in sell_orders.range_mut(..=buy_order.price) {
            while let Some(sell_order) = orders_at_price.front_mut() {
                let traded_quantity = buy_order.quantity.min(sell_order.quantity);
    
                let trade = Trade {
                    buy_id: buy_order.id,
                    sell_id: sell_order.id,
                    price: sell_price,
                    quantity_traded: traded_quantity,
                };
                trades.push(trade);
    
                buy_order.quantity -= traded_quantity;
                sell_order.quantity -= traded_quantity;
    
                if sell_order.quantity == 0 {
                    orders_at_price.pop_front();
                }
    
                if buy_order.quantity == 0 {
                    break;
                }
            }
    
            if orders_at_price.is_empty() {
                prices_to_remove.push(sell_price);
            }
    
            if buy_order.quantity == 0 {
                break;
            }
        }
    
        for price in prices_to_remove {
            sell_orders.remove(&price);
        }
    
        trades
    }
    
    pub fn register_sell_order(sell_order: Order, sell_orders: &mut OrderBook) {
        sell_orders
            .entry(sell_order.price)
            .or_insert_with(VecDeque::new)
            .push_back(sell_order);
    }
}