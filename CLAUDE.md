# pzql

<EXTREMELY-IMPORTANT>
If you think there is even a 1% chance a skill might apply to what you are doing, you ABSOLUTELY MUST invoke the skill.

IF A SKILL APPLIES TO YOUR TASK, YOU DO NOT HAVE A CHOICE. YOU MUST USE IT.

This is not negotiable. This is not optional. You cannot rationalize your way out of this.
</EXTREMELY-IMPORTANT>

A cross-platform desktop app built with Tauri v2 (React 19 + TypeScript frontend, Rust backend).

## Package Manager
Always use **bun** (not npm/yarn).

## Key Commands
- `bun run dev` - start Vite dev server (port 1420)
- `bun run tauri dev` - run full Tauri app in dev mode
- `bun run build` - type-check + Vite build
- `bun run tauri build` - package desktop app

## Testing
Use **Playwright CLI** to test the app. Run tests with `bunx playwright test`.

## Frontend Stack
- **Vite** - bundler/dev server
- **shadcn/ui** - component library
- **Tailwind CSS** - styling
- **Biome** - linter + formatter (replaces ESLint/Prettier)

Lean on these libraries - do not reinvent the wheel. Use shadcn components and Tailwind utilities instead of writing custom CSS or building UI primitives from scratch. Every line of code is a liability.

## Documentation Lookup
Use the **Context7 MCP** to fetch up-to-date docs for our stack (Tauri v2, React 19, Rust, Tailwind CSS, shadcn/ui, Vite, Playwright, Biome) before writing code that depends on library APIs. Don't guess at APIs from training data - pull current docs via Context7.

## Git
Keep commits atomic - one logical change per commit. Don't bundle unrelated changes together.

## Git Hooks (lefthook)
Pre-commit hooks run automatically via **lefthook** (config in `lefthook.yml`):
- **Biome** - lint + format staged TS/JS/JSON/CSS files
- **cargo fmt** - format staged Rust files
- **cargo clippy** - lint staged Rust files

To install hooks after cloning: `bunx lefthook install`

## Linting & Formatting
- **Frontend (TS/CSS/JSON)**: `bunx @biomejs/biome check --write .`
- **Rust**: `cargo fmt` (formatting) + `cargo clippy` (linting)

## Hooks
- All hook scripts live in `.claude/hooks/<hook-type>/` (e.g. `pre-tool-use/`, `post-tool-use/`, `notification/`).
- Always use a standalone script file - never inline commands in settings JSON.
- One script per concern. Keep scripts focused and name them descriptively.
- Reference scripts in settings via `"$CLAUDE_PROJECT_DIR"/.claude/hooks/<hook-type>/<script>`.

## Architecture
- `src/` - React/TypeScript frontend (Vite)
- `src-tauri/src/` - Rust Tauri backend (commands, plugins)
- IPC via `@tauri-apps/api` - frontend calls Rust with `invoke()`
- New Tauri commands need: Rust fn in `lib.rs` + registered in `generate_handler![]` + TypeScript wrapper in `src/`

## Style
- Never use emojis unless the user explicitly asks for them.
- Never use emdashes. Use hyphens or dashes instead, unless the user explicitly asks for emdashes.

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
