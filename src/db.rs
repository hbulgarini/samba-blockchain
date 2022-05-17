use crate::block::Block;
use crate::transaction::Transaction;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::Seek;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct DBConnection<'a> {
    pub db_file: File,
    pub db_name: String,
    pub new: bool,
}

#[derive(Debug)]
pub struct SambaDB<'a> {
    blockchain: &'a DBConnection<'a>,
    txs: &'a DBConnection<'a>,
}

impl<'a> DBConnection<'a> {
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

    pub fn open<T: Deserialize<'a>>(&mut self) -> (Vec<T>, usize) {
        if self.new {
            let empty: Vec<T> = Vec::new();
            return (empty, 0);
        } else {
            let items: Vec<T> = bincode::deserialize_from(&self.db_file).unwrap();
            let current_id = items.len();
            return (items, current_id);
        };
    }

    pub fn write_to_db<T: Deserialize<'a>>(&mut self, registry: &Vec<T>) {
        let encoded: Vec<u8> = bincode::serialize(&registry).unwrap();
        self.db_file
            .seek(std::io::SeekFrom::Start(0))
            .expect("can't rewind the cursor");
        self.db_file.write_all(&encoded).unwrap();
    }
}

impl<'a> SambaDB<'a> {
    pub fn init_samba() -> SambaDB<'a> {
        let txs = &DBConnection::init_db("txs.db");
        let blockchain = &DBConnection::init_db("samba.db");

        println!("{:?}", blockchain);
        println!("{:?}", txs);

        SambaDB { blockchain, txs }
    }
}
