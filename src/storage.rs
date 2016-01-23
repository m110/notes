use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;

pub struct Storage {
    file: File,
}

impl Storage {
    pub fn new(path: String) -> Result<Storage, String> {
        let path = expand_home_dir(path);

        let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path);

        match file {
            Ok(file) => Ok(Storage { file: file }),
            Err(_) => Err("Open failed".to_string()),
        }
    }

    pub fn entries(&mut self) -> Vec<String> {
        self.file.seek(SeekFrom::Start(0)).unwrap();

        let mut result = vec![];
        let reader = BufReader::new(&self.file);

        for line in reader.lines() {
            result.push(line.unwrap());
        }

        return result;
    }

    pub fn add_entry(&mut self, content: String) {
        self.file.write_all(content.as_bytes()).unwrap();
    }
}

fn expand_home_dir(path: String) -> String {
    match env::home_dir() {
        Some(home_dir) => path.replace("~", home_dir.to_str().unwrap()),
        None => path,
    }
}
