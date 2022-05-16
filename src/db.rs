use crate::block::Block;
use crate::transaction::Transaction;

use std::fs::OpenOptions;
use std::io::Seek;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct DBConnection {
    pub db_file: File,
    pub db_name: String,
    pub new: bool,
}

#[derive(Debug)]
pub struct SambaDB {
    blockchain: DBConnection,
    txs: DBConnection,
}

impl DBConnection {
    fn init_db(db_name: &str) -> DBConnection {
        println!("Initing DB {}", db_name);

        let db_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(&db_name)
            .expect(&format!("Can't open the file: {}", &db_name));

        let metadata = db_file.metadata().unwrap();
        let new = if metadata.len() == 0 { true } else { false };
        println!("Is db {} new:  {}", db_name, new);

        DBConnection {
            db_file,
            db_name: db_name.to_string(),
            new,
        }
    }

    fn write_to_db(&mut self, encoded: &[u8]) {
        self.db_file
            .seek(std::io::SeekFrom::Start(0))
            .expect("can't rewind the cursor");
        self.db_file.write_all(encoded).unwrap();
    }
}

impl SambaDB {
    pub fn init_samba() -> SambaDB {
        let txs = DBConnection::init_db("txs.db");
        let blockchain = DBConnection::init_db("samba.db");

        println!("{:?}", blockchain);
        println!("{:?}", txs);

        SambaDB { blockchain, txs }
    }

    pub fn open_txs(&mut self) -> (Vec<Transaction>, usize) {
        if self.txs.new {
            let empty: Vec<Transaction> = Vec::new();
            return (empty, 0);
        } else {
            let txs: Vec<Transaction> = bincode::deserialize_from(&self.txs.db_file).unwrap();
            let current_id = txs.len();
            return (txs, current_id);
        };
    }

    fn open_blockchain(&mut self) -> (Vec<Block>, usize) {
        if self.txs.new {
            let empty: Vec<Block> = Vec::new();
            return (empty, 0);
        } else {
            let txs: Vec<Block> = bincode::deserialize_from(&self.txs.db_file).unwrap();
            let current_id = txs.len();
            return (txs, current_id);
        };
    }

    pub fn write_to_blockchain_db(&mut self, registry: &Vec<Block>) {
        let encoded: Vec<u8> = bincode::serialize(&registry).unwrap();
        self.blockchain.write_to_db(&encoded);
    }

    pub fn write_to_txs_db(&mut self, registry: &Vec<Transaction>) {
        let encoded: Vec<u8> = bincode::serialize(&registry).unwrap();
        self.txs.write_to_db(&encoded);
    }
}
