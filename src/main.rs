use std::env;
use std::time::SystemTime;
use transaction::{Transaction, Transactions};

use crate::db::SambaDB;
mod block;
mod db;
mod merkle_tree;
mod transaction;
pub mod util;

fn help() {
    println!(
        "usage:
TO DO"
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut db = db::SambaDB::init_samba();
    match args.len() {
        3 => {
            let cmd = &args[1];

            match &cmd[..] {
                "add_tx" => {
                    let values: Vec<&str> = args[2].split(";").collect();
                    if let [from, to, amount] = &values[..] {
                        let amount: u32 = amount.parse().expect("Invalid u32 id");

                        if Transaction::validate_transaction(
                            &from.to_string(),
                            &amount,
                            &db.accounts_db.accounts,
                        ) {
                            let mut transactions = Transactions::read(&mut db);
                            transactions.add_tx(from.to_string(), to.to_string(), amount, &mut db);

                            let tree =
                                merkle_tree::MerkleTree::create_tree(&transactions.transactions);

                            db.accounts_db
                                .db_accounts
                                .write_to_db(&db.accounts_db.accounts);
                            let timestamp = SystemTime::now();

                            if transactions.transactions.len() >= 10 {
                                let block = block::Block::new(
                                    tree.root,
                                    "1".to_string(),
                                    timestamp,
                                    &transactions.transactions,
                                );
                                println!("New block minted {:?}", block);
                                let blocks = vec![block];
                                db.blockchain.write_to_db(&blocks);
                                db.txs.clear_db();
                            } else {
                                db.txs.write_to_db(&transactions.transactions);
                            }
                        } else {
                            panic!("Account does not exist or does not have enough funds");
                        }
                    } else {
                        panic!("Invalid registry!");
                    }
                }
                _ => {
                    eprintln!("Missing arguments");
                    help();
                }
            }
        }

        _ => {
            // show a help message
            help();
        }
    }
}
