use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    root: String,
    previous_hash: String,
    timestamp: SystemTime,
    transactions: Vec<Transaction>,
}

impl Block {
    pub fn new(
        root: String,
        previous_hash: String,
        timestamp: SystemTime,
        transactions: &Vec<Transaction>,
    ) -> Block {
        Block {
            root,
            previous_hash,
            timestamp,
            transactions: transactions.to_vec(),
        }
    }
}
