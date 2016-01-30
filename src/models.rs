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
    category: BookCategory,
    entries: Vec<Entry>,
}

impl Book {
    pub fn new() -> Book {
        Book {
            category: BookCategory::Journal,
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
