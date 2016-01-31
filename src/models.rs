use std::str::FromStr;

pub enum BookCategory {
    Journal,
    Notepad,
    Scratchpad,
}

pub struct Entry {
    pub date: String, // TODO Proper date type
    pub content: String,
}

pub struct Book {
    pub title: String,
    category: BookCategory,
    pub entries: Vec<Entry>,
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
            category: category.parse::<BookCategory>().expect("Unknown book category"),
            entries: vec![],
        }
    }

    pub fn add_entry(&mut self, entry: Entry) {
        self.entries.push(entry)
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

    pub fn title(&self) -> &String {
        &self.content
    }
}
