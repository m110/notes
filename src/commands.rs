use storage::Storage;
use editor::text_from_editor;

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
    storage.add_entry(text_from_editor());

    Action::Output("Entry added".to_string())
}

pub fn exit(_: &mut Storage) -> Action {
    Action::Exit
}
