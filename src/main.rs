extern crate rusqlite;
mod storage;

use std::io;
use std::io::prelude::*;
use std::collections::HashMap;

use storage::Storage;

enum Action {
    Output(String),
    Exit,
    None,
}

type Commands = HashMap<String, fn() -> Action>;

const DB_PATH: &'static str = "~/.notes.db";

fn ls() -> Action {
    let result = "placeholder".to_string();
    Action::Output(result)
}

fn add() -> Action {
    Action::None
}

fn exit() -> Action {
    Action::Exit
}

fn process_command(commands: &Commands, command: String) -> Action {
    match commands.get(&command) {
        None => { return Action::Output("No such command!".to_string()) },
        Some(method) => {
            return method();
        },
    };
}

fn command_loop(commands: &Commands) {
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

                match process_command(commands, input) {
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
    let storage = Storage::new(DB_PATH.to_string()).unwrap();

    commands.insert("ls".to_string(), ls);
    commands.insert("add".to_string(), add);
    commands.insert("exit".to_string(), exit);
    commands.insert("quit".to_string(), exit);

    command_loop(&commands);
}
