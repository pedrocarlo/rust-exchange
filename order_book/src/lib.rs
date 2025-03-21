pub(crate) mod price_time_priority;

fn generate_random_number() -> i64 {
    let mut buf = [0u8; 8];
    getrandom::fill(&mut buf).unwrap();
    i64::from_ne_bytes(buf)
}

pub type OrderId = u128;
pub type Price = ordered_float::OrderedFloat<f64>;
pub type Volume = u64;

#[derive(Debug, PartialEq, Eq)]
pub struct Timestamp(i64);

#[derive(Debug, PartialEq, Eq)]
pub struct Order {
    order_id: OrderId, // for now use u128, but later find a determinisitic way to generate uuids using our own rng seed
    /// Nanosecond precision timestamp
    timestamp: Timestamp,
    side: OrderSide,
    price: Price, // TODO wrap this value in a safe newtype struct to have some guarentees in decimal palce
    volume: Volume,
    client_id: u64,
}

// TODO implement new types here for specific ordering priorities

impl PartialOrd for Order {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.price.partial_cmp(&other.price)
    }
}

impl Ord for Order {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.price.cmp(&other.price)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum OrderSide {
    Buy,
    Sell,
}

pub trait OrderBook {
    /// Places an order and tries to execute it against another order,
    /// else adds to the order book
    fn place_order(&mut self, order: Order) -> bool;
    /// Attempts to cancel the order
    fn cancel_order(&mut self, order_id: OrderId); // TODO maybe have some signal to show that it was executed
    /// Best asking price
    fn best_ask(&self) -> Option<&Order>;
    /// Best bidding price  
    fn best_bid(&self) -> Option<&Order>;
}
