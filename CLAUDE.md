# pzql

A cross-platform desktop app built with Tauri v2 (React 19 + TypeScript frontend, Rust backend).

## Package Manager
Always use **bun** (not npm/yarn).

## Key Commands
- `bun run dev` — start Vite dev server (port 1420)
- `bun run tauri dev` — run full Tauri app in dev mode
- `bun run build` — type-check + Vite build
- `bun run tauri build` — package desktop app

## Architecture
- `src/` — React/TypeScript frontend (Vite)
- `src-tauri/src/` — Rust Tauri backend (commands, plugins)
- IPC via `@tauri-apps/api` — frontend calls Rust with `invoke()`
- New Tauri commands need: Rust fn in `lib.rs` + registered in `generate_handler![]` + TypeScript wrapper in `src/`

## Tauri Commands Pattern
Rust:
```rust
#[tauri::command]
fn my_command(arg: &str) -> String { ... }
// Register in run(): .invoke_handler(tauri::generate_handler![my_command])
```
TypeScript:
```ts
import { invoke } from "@tauri-apps/api/core";
await invoke<string>("my_command", { arg: "value" });
```
