use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use regex::Regex;

use models::{Book, Entry};

pub struct Storage {
    file: File,
    books: Vec<Book>,
}

impl Storage {
    pub fn new(path: &str) -> Result<Storage, String> {
        let path = expand_home_dir(path);

        let file = OpenOptions::new()
                    .read(true)
                    .write(true)
                    .append(true)
                    .create(true)
                    .open(path);

        match file {
            Ok(file) => Ok(Storage { file: file, books: vec![] }),
            Err(_) => Err("Open failed".to_string()),
        }
    }

    pub fn load(&mut self) {
        let mut reader = Reader::new();
        self.books = reader.read(&mut self.file);
    }

    pub fn books(&self) -> &Vec<Book> {
        &self.books
    }

    pub fn entries(&self, book_id: usize) -> &Vec<Entry> {
        &self.books[book_id].entries
    }

    pub fn add_entry(&mut self, content: String) {
        self.file.write_all(content.as_bytes()).unwrap();
    }
}

fn expand_home_dir(path: &str) -> String {
    match env::home_dir() {
        Some(home_dir) => path.replace("~", home_dir.to_str().unwrap()),
        None => path.to_string(),
    }
}

struct Reader {
    current_book: Option<Book>,
    current_entry: Option<Entry>,
    content: String,
    books: Vec<Book>,
}

impl Reader {
    pub fn new() -> Reader {
        Reader {
            current_book: None,
            current_entry: None,
            content: String::new(),
            books: Vec::<Book>::new(),
        }
    }

    pub fn read(&mut self, file: &mut File) -> Vec<Book> {
        file.seek(SeekFrom::Start(0)).unwrap();

        let reader = BufReader::new(file);

        let book_regex = Regex::new("^== (.+) == (.+) ==$").unwrap();
        let entry_regex = Regex::new("^== (.+) ==$").unwrap();

        for line in reader.lines() {
            let line = line.unwrap();

            match book_regex.captures(&line) {
                Some(caps) => {
                    self.end_book();

                    let title = caps.at(1).unwrap_or("unknown");
                    let category = caps.at(2).unwrap_or("unknown");
                    self.current_book = Some(Book::new(title, category));

                    continue;
                },
                None => {},
            }

            match entry_regex.captures(&line) {
                Some(caps) => {
                    self.end_entry();
                    self.content.clear();
                    self.current_entry = Some(Entry::new());
                    continue;
                },
                None => {},
            }

            self.content.push_str(&line);
            self.content.push('\n');
        }

        self.end_book();

        self.books.drain(..).collect()
    }

    fn end_book(&mut self) {
        self.end_entry();
        match self.current_book.take() {
            Some(book) => {
                self.books.push(book);
            },
            None => {},
        }
    }

    fn end_entry(&mut self) {
        match self.current_entry.take() {
            Some(mut entry) => {
                entry.set_content(&self.content);

                match self.current_book.as_mut() {
                    Some(book) => {
                        book.add_entry(entry);
                    },
                    None => panic!("Entry not within a book"),
                }
            },
            None => {},
        }
    }
}
