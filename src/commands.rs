use storage::Storage;

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
    let entry = "Placeholder entry\n".to_string();
    storage.add_entry(entry);

    Action::Output("Entry added".to_string())
}

pub fn exit(_: &mut Storage) -> Action {
    Action::Exit
}
