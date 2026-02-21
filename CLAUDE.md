# pzql

A cross-platform desktop app built with Tauri v2 (React 19 + TypeScript frontend, Rust backend).

## Package Manager
Always use **bun** (not npm/yarn).

## Key Commands
- `bun run dev` — start Vite dev server (port 1420)
- `bun run tauri dev` — run full Tauri app in dev mode
- `bun run build` — type-check + Vite build
- `bun run tauri build` — package desktop app

## Testing
Use **Playwright CLI** to test the app. Run tests with `bunx playwright test`.

## Frontend Stack
- **Vite** — bundler/dev server
- **shadcn/ui** — component library
- **Tailwind CSS** — styling
- **Biome** — linter + formatter (replaces ESLint/Prettier)

Lean on these libraries — do not reinvent the wheel. Use shadcn components and Tailwind utilities instead of writing custom CSS or building UI primitives from scratch. Every line of code is a liability.

## Linting & Formatting
- **Frontend (TS/CSS/JSON)**: `bunx @biomejs/biome check --write .`
- **Rust**: `cargo fmt` (formatting) + `cargo clippy` (linting)

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
