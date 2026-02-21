# Unified Dev Workflow Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Single-command dev workflow (`bun run app dev`) that runs the native window, Vite, and WebSocket server simultaneously.

**Architecture:** Make axum/tokio/futures-util non-optional deps. Gate the dev_server module with `#[cfg(debug_assertions)]` so it compiles in debug but is eliminated in release. Spawn the WS server as a background task in Tauri's setup hook. Keep the `dev-server` feature for standalone mode.

**Tech Stack:** Tauri v2, axum, tokio, Rust cfg attributes

---

### Task 1: Make WS deps non-optional in Cargo.toml

**Files:**
- Modify: `src-tauri/Cargo.toml`

**Step 1: Edit Cargo.toml**

Move axum, tokio, futures-util from optional to always-on. Keep the `dev-server` feature but change it to just a flag (no dep activation).

```toml
[dependencies]
tauri = { version = "2", features = [] }
tauri-plugin-opener = "2"
serde = { version = "1", features = ["derive"] }
serde_json = "1"
pzql-ipc = { path = "crates/pzql-ipc" }
pzql-macros = { path = "crates/pzql-macros" }
inventory = "0.3"
specta = "=2.0.0-rc.22"
tauri-specta = { version = "=2.0.0-rc.21", features = ["derive", "typescript"] }
specta-typescript = "0.0.9"
axum = { version = "0.8", features = ["ws"] }
tokio = { version = "1", features = ["full"] }
futures-util = "0.3"

[features]
dev-server = []
```

Remove the separate `[dependencies.axum]`, `[dependencies.tokio]`, `[dependencies.futures-util]` sections and the `[dev-dependencies]` tokio entry (now redundant).

**Step 2: Verify it compiles**

Run: `cd src-tauri && cargo check`
Expected: compiles without errors

**Step 3: Commit**

Message: `chore: make WS deps non-optional for debug dev server`

---

### Task 2: Gate dev_server module with debug_assertions

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Change the cfg on the dev_server module**

Replace:
```rust
#[cfg(feature = "dev-server")]
mod dev_server;
```

With:
```rust
#[cfg(any(feature = "dev-server", debug_assertions))]
mod dev_server;
```

**Step 2: Verify it compiles**

Run: `cd src-tauri && cargo check`
Expected: compiles - dev_server module is now included in debug builds

**Step 3: Commit**

Message: `refactor: compile dev_server module in all debug builds`

---

### Task 3: Spawn WS server in Tauri setup hook

**Files:**
- Modify: `src-tauri/src/lib.rs`

**Step 1: Rewrite the run() function**

Replace the entire `run()` function body (after the specta export block) with:

```rust
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

            #[cfg(debug_assertions)]
            tauri::async_runtime::spawn(dev_server::run());

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
```

This keeps `cargo run --features dev-server` working as standalone mode, while `tauri dev` (no feature flag) spawns the WS server alongside the native window.

**Step 2: Verify standalone mode still works**

Run: `cd src-tauri && cargo run --features dev-server`
Expected: prints "dev-server listening on ws://127.0.0.1:1421"

**Step 3: Commit**

Message: `feat: spawn WS dev server alongside Tauri in debug builds`

---

### Task 4: Clean up package.json scripts

**Files:**
- Modify: `package.json`

**Step 1: Replace scripts**

```json
"scripts": {
    "app": "tauri",
    "build": "tsc && vite build",
    "prepare": "lefthook install"
}
```

Removes `dev` and `preview`. Renames `tauri` to `app`.

**Step 2: Commit**

Message: `chore: clean up scripts - remove unused, rename tauri to app`

---

### Task 5: Update tauri.conf.json beforeDevCommand

**Files:**
- Modify: `src-tauri/tauri.conf.json`

**Step 1: Change beforeDevCommand**

Replace `"bun run dev"` with `"vite"`.

**Step 2: Commit**

Message: `chore: inline vite in beforeDevCommand`

---

### Task 6: Update CLAUDE.md

**Files:**
- Modify: `CLAUDE.md`

**Step 1: Update Key Commands section**

Replace:
```
- `bun run dev` - start Vite dev server (port 1420)
- `bun run tauri dev` - run full Tauri app in dev mode
- `bun run build` - type-check + Vite build
- `bun run tauri build` - package desktop app
```

With:
```
- `bun run app dev` - run full app in dev mode (native window + Vite + WS server for Chrome)
- `bun run app build` - package desktop app
- `bun run build` - type-check + Vite build (used by Tauri internally)
```

**Step 2: Update Testing / Dev verification workflow section**

Replace the multi-terminal workflow with:
```sh
# Everything in one command
bun run app dev

# Native window opens automatically.
# For Chrome testing, also open http://localhost:1420
```

And add a note about standalone mode:
```sh
# Standalone backend-only mode (faster Rust iteration, no native window)
cd src-tauri && cargo run --features dev-server
# Then open http://localhost:1420 (Vite must be running separately)
```

**Step 3: Update the Dev verification in Chrome section under IPC**

Replace the two-terminal workflow with the single command.

**Step 4: Update the Adding a command step 4**

Change `bun run tauri dev` to `bun run app dev`.

**Step 5: Commit**

Message: `docs: update CLAUDE.md for unified dev workflow`

---

### Task 7: Verify everything works end-to-end

**Step 1: Run `bun run app dev`**

Expected: Vite starts on 1420, native window opens, WS server listens on 1421.

**Step 2: Open Chrome at http://localhost:1420**

Expected: App loads, IPC calls work via WebSocket.

**Step 3: Test native window**

Expected: App loads, IPC calls work via Tauri IPC.
