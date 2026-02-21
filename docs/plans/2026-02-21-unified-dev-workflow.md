# Unified Dev Workflow

## Problem

The dev setup requires managing multiple terminals and knowing which commands to run for different scenarios. `bun run tauri dev` doesn't start the WebSocket server, so Chrome and the native app can't run simultaneously. `bun run dev` (Vite only) is never run manually but exists as a script.

## Design

### Script cleanup

Remove unused scripts, rename `tauri` to `app`:

```json
"scripts": {
    "app": "tauri",
    "build": "tsc && vite build",
    "prepare": "lefthook install"
}
```

Change `beforeDevCommand` in `tauri.conf.json` from `"bun run dev"` to `"vite"`.

### Dual-transport in debug builds

In `lib.rs`, spawn the WebSocket dev server on a background tokio task inside the Tauri `setup` hook when building in debug mode. This runs alongside the native Tauri app, so both transports work simultaneously:

- Native window uses Tauri IPC
- Chrome on localhost:1420 connects via WebSocket to port 1421
- The frontend auto-detects which transport to use (already implemented in `src/ipc.ts`)

The `dev-server` cargo feature stays for standalone backend-only iteration (`cargo run --features dev-server`), useful for fast Rust-only changes without compiling all of Tauri.

### Resulting workflows

| Command | What it does |
|---------|-------------|
| `bun run app dev` | Starts Vite + native window + WS server. Open Chrome at localhost:1420 for browser testing. |
| `bun run app build` | Production bundle |
| `cargo run --features dev-server` | Standalone WS server for fast backend iteration (open localhost:1420 manually) |

### CLAUDE.md updates

Update all command references to use the new names.
