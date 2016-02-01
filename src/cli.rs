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
            let prompt = match self.current_book {
                Some(book) => self.storage.books()[book].title.clone(),
                None => String::new(),
            };

            print!("{}> ", prompt);
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
        // TODO: Split on all whitespace
        let mut args = command.split(" ").collect::<Vec<_>>();
        let command = args.remove(0);

        match command {
            "ls" => self.ls(args),
            "add" => self.add(args),
            "exit" => self.exit(args),
            "quit" => self.exit(args),
            _ => Action::Output("No such command!".to_string()),
        }
    }

    fn ls(&mut self, _: Vec<&str>) -> Action {
        let mut result = String::new();

        for book in self.storage.books() {
            result.push_str(&book.title);
        }

        Action::Output(result)
    }

    fn add(&mut self, _: Vec<&str>) -> Action {
        let date = match time::strftime(TIME_FORMAT, &time::now()) {
            Ok(date) => date,
            Err(_) => String::from("(unknown)"),
        };

        let content = text_from_editor();

        let entry = format!("\n{}\n\n{}", date, content);

        self.storage.add_entry(entry);

        Action::Output("Entry added".to_string())
    }

    fn exit(&mut self, _: Vec<&str>) -> Action {
        Action::Exit
    }
}
