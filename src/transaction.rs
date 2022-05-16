use crate::util;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Details {
    from: String,
    to: String,
    amount: u32,
    id: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub transaction: Details,
    pub hash: String,
}

impl Details {
    pub fn new(from: String, to: String, amount: u32, id: u32) -> Details {
        Details {
            from,
            to,
            amount,
            id,
        }
    }
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u32, id: u32) -> Transaction {
        let transaction = Details::new(from, to, amount, id);
        let bytes = bincode::serialize(&transaction).unwrap();
        let hash = util::to_hash(&bytes).as_str().to_owned();

        Transaction { transaction, hash }
    }
}