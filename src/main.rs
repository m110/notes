extern crate rand;
extern crate time;

mod commands;
mod editor;
mod models;
mod storage;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

use storage::Storage;
use commands::Action;
use commands::{add,ls,exit};

type Commands = HashMap<String, fn(storage: &mut Storage) -> Action>;

const DB_PATH: &'static str = "~/.notes";

fn process_command(commands: &Commands, storage: &mut Storage, command: String) -> Action {
    match commands.get(&command) {
        None => { return Action::Output("No such command!".to_string()) },
        Some(method) => {
            return method(storage);
        },
    };
}

fn command_loop(commands: &Commands, storage: &mut Storage) {
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

                match process_command(commands, storage, input) {
                    Action::Output(output) => println!("{}", output),
                    Action::Exit => break,
                    Action::None => (),
                }
            },
        }
    }

    println!("Bye!");
}

fn main() {
    let mut commands: Commands = HashMap::new();
    let mut storage = Storage::new(DB_PATH.to_string()).unwrap();

    commands.insert("ls".to_string(), ls);
    commands.insert("add".to_string(), add);
    commands.insert("exit".to_string(), exit);
    commands.insert("quit".to_string(), exit);

    command_loop(&commands, &mut storage);
}
