use pzql_macros::command;
use tauri_specta::{collect_commands, Builder};

#[cfg(feature = "dev-server")]
mod dev_server;

#[command]
fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = Builder::<tauri::Wry>::new().commands(collect_commands![greet]);

    #[cfg(debug_assertions)]
    builder
        .export(
            specta_typescript::Typescript::default()
                .header("// @ts-nocheck\n// AUTO-GENERATED - do not edit"),
            "../src/bindings.ts",
        )
        .expect("failed to export typescript bindings");

    #[cfg(feature = "dev-server")]
    {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(dev_server::run());
        return;
    }

    #[cfg(not(feature = "dev-server"))]
    {
        tauri::Builder::default()
            .plugin(tauri_plugin_opener::init())
            .invoke_handler(builder.invoke_handler())
            .setup(move |app| {
                builder.mount_events(app);
                Ok(())
            })
            .run(tauri::generate_context!())
            .expect("error while running tauri application");
    }
}
