use pzql_ipc::WsEventEntry;
use pzql_macros::event;
use serde::{Deserialize, Serialize};

#[event]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct FileChanged {
    path: String,
}

#[event]
#[derive(Serialize, Deserialize, Clone, Debug)]
struct QueryProgress {
    total: u64,
    completed: u64,
}

#[test]
fn event_macro_registers_ws_entries() {
    let entries: Vec<&WsEventEntry> = inventory::iter::<WsEventEntry>().collect();
    let names: Vec<&str> = entries.iter().map(|e| e.name).collect();

    assert!(
        names.contains(&"file_changed"),
        "file_changed not registered"
    );
    assert!(
        names.contains(&"query_progress"),
        "query_progress not registered"
    );
}

#[test]
fn event_struct_is_serializable() {
    let event = FileChanged {
        path: "/tmp/test.sql".to_string(),
    };
    let json = serde_json::to_value(&event).unwrap();
    assert_eq!(json["path"], "/tmp/test.sql");
}

#[test]
fn event_name_is_snake_case() {
    let entries: Vec<&WsEventEntry> = inventory::iter::<WsEventEntry>().collect();
    let entry = entries
        .iter()
        .find(|e| e.name == "query_progress")
        .expect("query_progress not found");
    assert_eq!(entry.name, "query_progress");
}
