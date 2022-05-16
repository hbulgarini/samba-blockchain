/* use crate::transaction;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Block<'a> {
    root: &'a str,
    previous_hash: &'a str,
    nonce: u32,
    timestamp: u32,
    transactions: Vec<transaction::Transaction>,
}

impl<'a> Block<'a> {
    pub fn new(
        root: &'a str,
        previous_hash: &'a str,
        nonce: u32,
        timestamp: u32,
        transactions: &Vec<transaction::Transaction>,
    ) -> Block<'a> {
        Block {
            root,
            previous_hash,
            nonce,
            timestamp,
            transactions: transactions.to_vec(),
        }
    }
}
 */
