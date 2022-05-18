use serde::de::DeserializeOwned;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::Seek;
use std::{fs::File, io::Write};

#[derive(Debug)]
pub struct DBConnection {
    pub db_file: File,
    pub db_name: String,
    pub new: bool,
}

pub type AccountsList = HashMap<String, u32>;

#[derive(Debug)]
pub struct AccountsDB {
    pub db_accounts: DBConnection,
    pub accounts: AccountsList,
}

#[derive(Debug)]
pub struct SambaDB {
    pub blockchain: DBConnection,
    pub txs: DBConnection,
    pub accounts_db: AccountsDB,
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

    pub fn open<T: DeserializeOwned + Serialize>(&mut self) -> (Vec<T>, usize) {
        if self.new {
            let empty: Vec<T> = Vec::new();
            return (empty, 0);
        } else {
            let items: Vec<T> = bincode::deserialize_from(&self.db_file).unwrap();
            let current_id = items.len();
            return (items, current_id);
        };
    }

    pub fn write_to_db<T: DeserializeOwned + Serialize>(&mut self, registry: &Vec<T>) {
        let encoded: Vec<u8> = bincode::serialize(&registry).unwrap();
        self.db_file
            .seek(std::io::SeekFrom::Start(0))
            .expect("can't rewind the cursor");
        self.db_file.write_all(&encoded).unwrap();
    }

    pub fn clear_db(&mut self) {
        let empty = {};
        let encoded: Vec<u8> = bincode::serialize(&empty).unwrap();
        self.db_file
            .seek(std::io::SeekFrom::Start(0))
            .expect("can't rewind the cursor");
        self.db_file.write_all(&encoded).unwrap();
    }
}

fn read_accounts(db_accounts: &DBConnection) -> AccountsList {
    if db_accounts.new {
        let mut genesis: AccountsList = AccountsList::new();
        genesis.insert("genesis".to_string(), 100000000);
        return genesis;
    } else {
        let accounts: AccountsList = bincode::deserialize_from(&db_accounts.db_file).unwrap();
        return accounts;
    };
}

impl SambaDB {
    pub fn init_samba() -> SambaDB {
        let txs = DBConnection::init_db("txs.db");
        let blockchain = DBConnection::init_db("samba.db");
        let db_accounts = DBConnection::init_db("accounts.db");
        let accounts = read_accounts(&db_accounts);

        let accounts_db = AccountsDB {
            db_accounts,
            accounts,
        };

        SambaDB {
            blockchain,
            txs,
            accounts_db,
        }
    }
}
