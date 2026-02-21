use pzql_ipc::WsCommandEntry;
use pzql_macros::command;

#[command]
fn sync_greet(name: String) -> String {
    format!("Hello, {}!", name)
}

#[command]
async fn async_greet(name: String) -> String {
    format!("Hello async, {}!", name)
}

#[command]
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
fn command_macro_registers_ws_entries() {
    let entries: Vec<&WsCommandEntry> = inventory::iter::<WsCommandEntry>().collect();
    let names: Vec<&str> = entries.iter().map(|e| e.name).collect();

    assert!(names.contains(&"sync_greet"), "sync_greet not registered");
    assert!(names.contains(&"async_greet"), "async_greet not registered");
    assert!(names.contains(&"add"), "add not registered");
}

#[tokio::test]
async fn sync_command_handler_works() {
    let entry = inventory::iter::<WsCommandEntry>()
        .find(|e| e.name == "sync_greet")
        .expect("sync_greet not found");

    let args = serde_json::json!({"name": "Alice"});
    let result = (entry.handler)(args).await.unwrap();
    assert_eq!(result, serde_json::json!("Hello, Alice!"));
}

#[tokio::test]
async fn async_command_handler_works() {
    let entry = inventory::iter::<WsCommandEntry>()
        .find(|e| e.name == "async_greet")
        .expect("async_greet not found");

    let args = serde_json::json!({"name": "Bob"});
    let result = (entry.handler)(args).await.unwrap();
    assert_eq!(result, serde_json::json!("Hello async, Bob!"));
}

#[tokio::test]
async fn multi_arg_command_handler_works() {
    let entry = inventory::iter::<WsCommandEntry>()
        .find(|e| e.name == "add")
        .expect("add not found");

    let args = serde_json::json!({"a": 3, "b": 4});
    let result = (entry.handler)(args).await.unwrap();
    assert_eq!(result, serde_json::json!(7));
}

#[tokio::test]
async fn command_handler_returns_error_on_bad_args() {
    let entry = inventory::iter::<WsCommandEntry>()
        .find(|e| e.name == "sync_greet")
        .expect("sync_greet not found");

    let args = serde_json::json!({"wrong_field": "Alice"});
    let result = (entry.handler)(args).await;
    assert!(result.is_err(), "expected error for bad args");
}
