use transaction::Transaction;

mod block;
mod merkle_tree;
mod transaction;
pub mod util;

fn main() {
    let t1 = Transaction::new("alice".to_string(), "bob".to_string(), 1, 1);
    let t2 = Transaction::new("dave".to_string(), "charlie".to_string(), 10, 2);
    let t3 = Transaction::new("bob".to_string(), "lily".to_string(), 5, 3);
    let t4 = Transaction::new("pato".to_string(), "hety".to_string(), 6, 5);
    let t5 = Transaction::new("jonas".to_string(), "carmen".to_string(), 2, 6);

    let transactions: Vec<Transaction> =
        vec![t1.clone(), t2.clone(), t3.clone(), t4.clone(), t5.clone()];

    let tree = merkle_tree::MerkleTree::create_tree(&transactions);
    let root = tree.root;
    println!("Root {:?}", root);

    /*     let t6 = Transaction::new("hector".to_string(), "carmen".to_string(), 6, 6);

    let mut modifiedTransaction = vec![t1, t2, t6, t4, t5];
    let modified_tree = merkle_tree::MerkleTree::create_tree(&modifiedTransaction);

    assert_eq!(root, modified_tree.root); */
}
