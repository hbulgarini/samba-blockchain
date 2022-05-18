use crate::{db::AccountsList, util};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Details {
    from: String,
    to: String,
    amount: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub transaction: Details,
    id: usize,
    pub hash: String,
}

impl Details {
    pub fn new(from: String, to: String, amount: u32) -> Details {
        Details { from, to, amount }
    }
}

impl Transaction {
    pub fn new(from: String, to: String, amount: u32, id: usize) -> Transaction {
        let transaction = Details::new(from, to, amount);
        let bytes = bincode::serialize(&transaction).unwrap();
        let hash = util::to_hash(&bytes).as_str().to_owned();

        Transaction {
            transaction,
            hash,
            id,
        }
    }
    pub fn validate_transaction(from: &String, amount: &u32, accounts: &AccountsList) -> bool {
        let account_balance = accounts.get(&from.to_string());
        match account_balance {
            // The division was valid
            Some(account_balance) => account_balance > &amount,
            None => false,
        }
        // The division was invalid
    }
}
