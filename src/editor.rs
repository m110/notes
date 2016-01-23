use rand;
use rand::Rng;
use std::process::Command;

const TEMP_FILE_LENGTH: usize = 16;

pub fn text_from_editor() -> String {
    let status = Command::new(editor())
                        .arg(temp_file())
                        .status();

    match status {
        Ok(status) => {
            if !status.success() {
                return String::new();
            }

            // TODO Return file's content
            return String::new();
        },
        Err(_) => return String::new(),
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

