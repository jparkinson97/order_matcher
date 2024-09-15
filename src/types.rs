pub mod order_types{
    use std::collections::{BTreeMap, VecDeque};
    use std::str::FromStr;

    pub enum OrderType {
        Buy, 
        Sell,
    }

    pub struct Order {
        pub id: usize, 
        pub order_type: OrderType, 
        pub price: u32, 
        pub quantity: u32,
    }

   pub struct Trade {
       pub buy_id: usize, 
       pub sell_id: usize, 
       pub price: u32,
       pub quantity_traded: u32,
   }

    pub type OrderBook = BTreeMap<u32, VecDeque<Order>>;

    impl FromStr for Order {
        type Err = String;
    
        fn from_str(line: &str) -> Result<Self, Self::Err> {
            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() != 2 {
                return Err("Invalid order format".into());
            }
    
            let id = parts[0].parse::<usize>().map_err(|_| "Invalid order ID")?;
    
            let tokens: Vec<&str> = parts[1].split_whitespace().collect();
            if tokens.len() != 6 {
                return Err("Invalid order format".into());
            }
    
            let order_type = match tokens[0] {
                "Buy" => OrderType::Buy,
                "Sell" => OrderType::Sell,
                _ => return Err("Invalid order type".into()),
            };
    
            let quantity = tokens[1].parse::<u32>().map_err(|_| "Invalid quantity")?;
    
            let price_str = tokens[4].replace(",", "");
            let price = price_str.parse::<u32>().map_err(|_| "Invalid price")?;
    
            Ok(Order {
                id,
                order_type,
                price,
                quantity,
            })
        }
    }
}