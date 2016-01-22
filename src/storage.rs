use rusqlite::Connection;

use std::env;

pub struct Storage {
    connection: Connection,
}

impl Storage {
    pub fn new(db_path: String) -> Result<Storage, String> {
        let db_path = expand_home_dir(db_path);

        match Connection::open(db_path) {
            Ok(connection) => Ok(Storage { connection: connection}),
            Err(_) => Err("SQLite connection failed".to_string()),
        }
    }
}

fn expand_home_dir(path: String) -> String {
    match env::home_dir() {
        Some(home_dir) => path.replace("~", home_dir.to_str().unwrap()),
        None => path,
    }
}
