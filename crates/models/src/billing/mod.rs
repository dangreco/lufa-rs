use std::collections::HashMap;

use serde::Deserialize;

mod card;
pub use card::*;

mod transaction;
pub use transaction::*;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BillingData {
    pub cards: HashMap<usize, Card>,
    pub transactions: Vec<Transaction>,
}
