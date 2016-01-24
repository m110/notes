use time;
use storage::Storage;
use editor::text_from_editor;

const TIME_FORMAT: &'static str = "%d-%m-%Y %H:%M:%S";

pub enum Action {
    Output(String),
    Exit,
    None,
}

pub fn ls(storage: &mut Storage) -> Action {
    let entries = storage.entries()[..].join("\n");
    Action::Output(entries)
}

pub fn add(storage: &mut Storage) -> Action {
    let date = match time::strftime(TIME_FORMAT, &time::now()) {
        Ok(date) => date,
        Err(_) => String::from("(unknown)"),
    };

    let content = text_from_editor();

    let entry = format!("\n{}\n\n{}", date, content);

    storage.add_entry(entry);

    Action::Output("Entry added".to_string())
}

pub fn exit(_: &mut Storage) -> Action {
    Action::Exit
}
