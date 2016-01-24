use rand;
use rand::Rng;
use std::process::Command;
use std::io::prelude::*;
use std::fs::File;

const TEMP_FILE_LENGTH: usize = 16;

pub fn text_from_editor() -> String {
    let temp_file = temp_file();

    let status = Command::new(editor())
                         .arg(temp_file.clone())
                         .status();

    match status {
        Ok(status) => {
            if !status.success() {
                return String::new();
            }

            let mut file = File::open(temp_file).unwrap();
            let mut buffer = String::new();

            file.read_to_string(&mut buffer).unwrap();

            buffer
        },
        Err(_) => String::new()
    }
}

/// TODO Return proper editor from env / config
fn editor() -> String {
    return "vim".to_string();
}

/// TODO Ensure real unique names
fn temp_file() -> String {
    let result = String::from("/tmp/");
    let file = rand::thread_rng()
                    .gen_ascii_chars()
                    .take(TEMP_FILE_LENGTH)
                    .collect::<String>();

    result + &file
}

