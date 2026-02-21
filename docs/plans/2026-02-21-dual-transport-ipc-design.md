# Dual-Transport IPC Design

**Date:** 2026-02-21
**Status:** Approved

## Problem

Tauri on macOS uses WKWebView, not Chromium. Browser automation tools (Playwright, Peekaboo on Chrome) cannot attach to WKWebView. This means Claude cannot open the app in Chrome to visually verify UI changes while also exercising real Rust backend logic - `invoke()` throws when called outside the Tauri runtime.

## Goal

Run the full app (UI + real Rust) in Chrome during development, with zero per-command maintenance burden as the command surface grows.

## Solution

A dual-transport IPC layer with two annotations that replace all manual wiring:

- `#[command]` - frontend calls Rust (replaces `#[tauri::command]`)
- `#[event]` - Rust pushes to frontend (replaces manual `emit()` calls)

Add the annotation. Everything else is automatic.

## Architecture

### Rust: `pzql-macros` crate

A proc macro crate containing `#[command]` and `#[event]`.

**`#[command]`** expands to:
1. `#[tauri::command]` for Tauri IPC
2. An `inventory::submit!` call that registers a `WsCommandEntry` in a compile-time global registry

**`#[event]`** expands to:
1. A typed `emit_<event_name>(handle, payload)` function that calls Tauri's `emit()` in app mode and pushes a WebSocket message in dev-server mode
2. An `inventory::submit!` call that registers a `WsEventEntry` in a parallel registry

The `inventory` crate collects all registered entries at startup via `inventory::iter::<WsCommandEntry>()`. No manual registration list exists anywhere.

Every command also gets `#[specta::specta]` applied, enabling tauri-specta to generate TypeScript bindings.

### Rust: `dev-server` feature flag

When compiled with `--features dev-server`, the binary starts an axum WebSocket server on port 1421 instead of Tauri. At startup it calls `inventory::iter::<WsCommandEntry>()` to build a dispatch table automatically. Incoming WebSocket messages are deserialized and dispatched to the same handler functions the Tauri commands use.

Run via:
```sh
cargo watch -x 'run --features dev-server'
```

### TypeScript: auto-generated bindings

tauri-specta runs at build time and outputs `src/bindings.ts` containing fully typed wrappers for every `#[command]` and `#[event]` in the codebase. This file is never edited by hand.

### TypeScript: transport bridge

`src/ipc.ts` - written once, never modified. Detects `window.__TAURI_INTERNALS__` at startup and switches the transport:

- In Tauri: routes through `invoke()` and `listen()`
- In Chrome: routes through a WebSocket connection to port 1421

All app code imports from `src/ipc.ts`. No app code ever calls `invoke()` directly.

### Type safety guarantees

| Mistake | Caught by |
|---------|-----------|
| Calling a command that doesn't exist | TypeScript compile error |
| Wrong argument types | TypeScript compile error |
| Wrong return type assumption | TypeScript compile error |
| Rust signature changes | tauri-specta regenerates `bindings.ts`, TypeScript compile error at call sites |
| Emitting an event with wrong payload shape | Rust compile error |

No runtime type surprises on either side.

## Developer Workflow

**Adding a command:**
```rust
#[command]
async fn my_command(input: String) -> Result<MyResponse, MyError> {
    // ...
}
```
Done. Available in both Tauri IPC and dev-server WebSocket automatically. TypeScript bindings regenerate on next build.

**Adding an event:**
```rust
#[event]
#[derive(Serialize, Deserialize, specta::Type)]
struct MyEvent {
    field: String,
}
```
Done. `emit_my_event(handle, payload)` is generated. TypeScript listener type is generated.

**Verifying in Chrome:**
```sh
# Terminal 1
cargo watch -x 'run --features dev-server'
# Terminal 2
bun run dev
# Open Chrome at localhost:1420
```

Frontend detects no Tauri runtime, connects to WebSocket server. Full app - real Rust logic, real UI - runs in Chrome.

## What Gets Built (Once)

1. **`pzql-macros` crate** - `#[command]` and `#[event]` proc macros with inventory registration. Tested with `insta` snapshot tests verifying macro expansion.
2. **`dev-server` feature** - axum WebSocket server reading from inventory registries. Tested with integration tests.
3. **`src/ipc.ts`** - ~50-line transport bridge. Written once.
4. **Build script** - runs tauri-specta codegen, outputs `src/bindings.ts`.

## Naming

- Macro: `#[command]` (not `#[tauri::command]`, not `#[dual_command]` - transport is invisible)
- Events: `#[event]`
- Generated bindings file: `src/bindings.ts` (never edit this file)
- Bridge: `src/ipc.ts` (always import from here, never from `bindings.ts` directly)
