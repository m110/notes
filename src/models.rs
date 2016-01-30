use std::str::FromStr;

pub enum BookCategory {
    Journal,
    Notepad,
    Scratchpad,
}

pub struct Entry {
    date: String, // TODO Proper date type
    content: String,
}

pub struct Book {
    title: String,
    category: BookCategory,
    entries: Vec<Entry>,
}

impl FromStr for BookCategory {
    type Err = ();

    fn from_str(s: &str) -> Result<BookCategory, ()> {
        match s {
            "journal" => Ok(BookCategory::Journal),
            "notepad" => Ok(BookCategory::Notepad),
            "scratchpad" => Ok(BookCategory::Scratchpad),
            _ => Err(()),
        }
    }
}

impl Book {
    pub fn new(title: &str, category: &str) -> Book {
        Book {
            title: String::from(title),
            category: category.parse::<BookCategory>().unwrap(),
            entries: vec![],
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry)
    }

    pub fn entries(self) -> Vec<Entry> {
        self.entries
    }
}

impl Entry {
    pub fn new() -> Entry {
        Entry {
            date: String::new(),
            content: String::new(),
        }
    }

    pub fn set_content(&mut self, content: &str) {
        self.content = String::from(content);
    }

    pub fn content(self) -> String {
        self.content
    }
}
