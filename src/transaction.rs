use crate::{
    db::{AccountsList, DBConnection, SambaDB},
    util,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Details {
    pub from: String,
    pub to: String,
    pub amount: u32,
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
        println!("from: {:?}", account_balance);
        match account_balance {
            Some(account_balance) => account_balance > &amount,
            None => false,
        }
    }
}

pub struct Transactions {
    pub transactions: Vec<Transaction>,
}

impl Transactions {
    pub fn read(db: &mut SambaDB) -> Self {
        let (txs, _id) = db.txs.open();
        Self { transactions: txs }
    }
    pub fn add_tx(&mut self, from: String, to: String, amount: u32, db: &mut SambaDB) {
        let len = db.accounts_db.accounts.len();
        let tx = Transaction::new(from.to_string(), to.to_string(), amount, len + 1);
        let from_new_balance = db.accounts_db.accounts.get(&tx.transaction.from).unwrap() - amount;
        let to_new_balance = db.accounts_db.accounts.get(&tx.transaction.to).unwrap() + amount;

        db.accounts_db
            .accounts
            .insert(tx.transaction.from.to_string(), from_new_balance);

        db.accounts_db
            .accounts
            .insert(tx.transaction.to.to_string(), to_new_balance);

        db.accounts_db
            .db_accounts
            .write_to_db(&db.accounts_db.accounts);

        self.transactions.push(tx);
    }
}
