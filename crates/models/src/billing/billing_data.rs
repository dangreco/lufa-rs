use serde::Deserialize;
use std::collections::HashMap;

use super::{Card, Transaction};

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct BillingData {
    pub cards: HashMap<usize, Card>,
    pub transactions: Vec<Transaction>,
}
