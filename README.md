Main branch includes simplest implementation I could build.
The other branch includes an extension to write orders to a queue and read from that.
- Using this pattern means the system could be extended to handle different asset classes
- Extending to use crossbeam would allow other consumers such as queues for different assets and/ or checking that order ids are unique


Other sensible features listed on this repo: https://github.com/dgtony/orderbook-rs/blob/master/src/engine/order_queues.rs
