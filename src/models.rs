enum BookCategory {
    Journal,
    Notepad,
    Scratchpad,
}

struct Entry {
    date: String, // TODO Proper date type
    content: String,
}

struct Book {
    category: BookCategory,
    entries: Vec<Entry>,
}
