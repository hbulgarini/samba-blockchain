use crate::transaction::Transaction;
use crate::util;
use std::format;

#[derive(Debug)]
pub struct MerkleTree {
    pub root: String,
}

fn show_short_hash(hash: &String) -> String {
    let first = &hash[..5];
    let len = hash.len();
    let last = &hash[len - 5..len];
    format!("{}...{}", first, last)
}

fn show_tree_array(hashes: &Vec<String>) {
    let formated_hashes: Vec<String> = hashes.iter().map(|hash| show_short_hash(&hash)).collect();
    println!("{:?}", formated_hashes);
}

impl MerkleTree {
    pub fn create_tree(transactions: &Vec<Transaction>) -> MerkleTree {
        let hashes: Vec<String> = transactions
            .iter()
            .map(|transaction| transaction.hash.clone())
            .collect();

        let mut tree: Vec<Vec<String>> = Vec::new();

        let mut index_tree = 0;
        tree.push(hashes);

        loop {
            println!("*******************");
            println!(
                "Tree at index {}: {:?}",
                index_tree,
                show_tree_array(&tree[index_tree])
            );
            let mut level = Vec::new();
            let mut index = 0;

            loop {
                let mut hash: String = "".to_string();
                if index < tree[index_tree].len() - 1 && index % 2 == 0 {
                    let left = index;
                    let right = index + 1;

                    let to_hash = tree[index_tree][left].as_str().to_owned()
                        + tree[index_tree][right].as_str();
                    let binary = bincode::serialize(&to_hash).unwrap();
                    hash = util::to_hash(&binary);
                    println!("Index {}", left);
                    println!("Index {}", right);
                    println!(
                        "Index {}+{}: {} + {}: {}",
                        left,
                        right,
                        show_short_hash(&tree[index_tree][left]),
                        show_short_hash(&tree[index_tree][right]),
                        show_short_hash(&hash)
                    );
                } else {
                    hash = tree[index_tree][index].clone();
                    println!("Index {}: {}", index, show_short_hash(&hash));
                }

                level.push(hash);
                // level.rotate_right(1);
                index += 2;

                if index > tree[index_tree].len() - 1 {
                    tree.push(level);
                    index_tree += 1;
                    break;
                }
            }

            if tree[index_tree].len() <= 1 {
                println!(
                    "Tree at index {}: {:?}",
                    index_tree,
                    show_tree_array(&tree[index_tree])
                );
                break;
            }
        }

        MerkleTree {
            root: tree[index_tree][0].clone(),
        }
    }
}
