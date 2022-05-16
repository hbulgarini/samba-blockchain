use std::env;
use std::time::SystemTime;
use transaction::Transaction;

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
                        let (txs, id) = db.open_txs();
                        println!("txs: {:?}", txs);

                        let amount: u32 = amount.parse().expect("Invalid u32 id");
                        let tx = Transaction::new(from.to_string(), to.to_string(), amount, id + 1);
                        let mut transactions = txs.clone();
                        println!("Transactions: {:?}", transactions);
                        transactions.push(tx);
                        let tree = merkle_tree::MerkleTree::create_tree(&transactions);
                        let root = tree.root;
                        let timestamp = SystemTime::now();

                        println!("Root {:?}", root);
                        if (transactions.len() >= 10) {
                            let block =
                                block::Block::new(root, "1".to_string(), timestamp, &transactions);
                            println!("New block minted {:?}", block);
                            let blocks = vec![block];
                            db.write_to_blockchain_db(&blocks);
                        }

                        // println!("${:?}", block);
                        db.write_to_txs_db(&transactions)
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

    /*     let t6 = Transaction::new("hector".to_string(), "carmen".to_string(), 6, 6);

    let mut modifiedTransaction = vec![t1, t2, t6, t4, t5];
    let modified_tree = merkle_tree::MerkleTree::create_tree(&modifiedTransaction);

    assert_eq!(root, modified_tree.root); */
}
