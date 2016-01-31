use time;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

use editor::text_from_editor;
use storage::Storage;

const DB_PATH: &'static str = "~/.notes";
const TIME_FORMAT: &'static str = "%d-%m-%Y %H:%M:%S";

pub enum Action {
    Output(String),
    Exit,
    None,
}

pub struct Cli {
    current_book: Option<usize>,
    storage: Storage,
}

impl Cli {
    pub fn new() -> Cli {
        let mut cli = Cli {
            current_book: None,
            storage: Storage::new(DB_PATH).unwrap(),
        };

        cli.storage.load();

        cli
    }

    pub fn run(&mut self) {
        loop {
            print!("notes> ");
            io::stdout().flush().expect("Flush failed");

            let mut input = String::new();
            match io::stdin().read_line(&mut input) {
                Err(_) => continue,
                Ok(_) => {
                    // EoF reached
                    if input.is_empty() {
                        println!("");
                        break;
                    }

                    input = input.trim().to_string();
                    if input.is_empty() {
                        continue;
                    }

                    match self.process_command(&input) {
                        Action::Output(output) => println!("{}", output),
                        Action::Exit => break,
                        Action::None => (),
                    }
                },
            }
        }

        println!("Bye!");
    }

    fn process_command(&mut self, command: &str) -> Action {
        match command {
            "ls" => self.ls(),
            "add" => self.add(),
            "exit" => self.exit(),
            "quit" => self.exit(),
            _ => Action::Output("No such command!".to_string()),
        }
    }

    fn ls(&mut self) -> Action {
        let mut result = String::new();

        for book in self.storage.books() {
            result.push_str(&book.title);
        }

        Action::Output(result)
    }

    fn add(&mut self) -> Action {
        let date = match time::strftime(TIME_FORMAT, &time::now()) {
            Ok(date) => date,
            Err(_) => String::from("(unknown)"),
        };

        let content = text_from_editor();

        let entry = format!("\n{}\n\n{}", date, content);

        self.storage.add_entry(entry);

        Action::Output("Entry added".to_string())
    }

    fn exit(&mut self) -> Action {
        Action::Exit
    }
}
