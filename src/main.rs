extern crate rand;
extern crate regex;
extern crate time;

mod cli;
mod editor;
mod models;
mod storage;

use cli::Cli;

fn main() {
    let mut cli = Cli::new();
    cli.run();
}
