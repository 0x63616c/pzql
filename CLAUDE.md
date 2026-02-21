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
Use **Peekaboo** (MCP server) to test the running app. After making UI changes, verify them visually by controlling the app - take screenshots, click elements, type text, and confirm things work as expected. Don't just write code and hope it works - see it.

### Peekaboo Usage (MCP)
Key tools:
- `see` - capture a screenshot with annotated element IDs. Always call this first to understand the UI state.
- `click` - click elements by ID (`on: "B1"`), text query (`query: "Submit"`), or coordinates (`coords: "100,200"`).
- `type` - type text into the focused element or a specific element (`on: "T1"`).
- `scroll` - scroll on an element or at the current mouse position.
- `hotkey` - press keyboard shortcuts (e.g. Cmd+S, Cmd+W).
- `app` - launch, quit, switch, or focus applications.
- `window` - focus, move, resize, close, or maximize windows.
- `menu` - list or click native menu bar items (e.g. `path: "File > Save"`).
- `dialog` - interact with system dialogs (click buttons, input text, handle file panels).
- `clipboard` - read/write clipboard contents.
- `image` - capture screenshots without annotation.

### Testing Workflow
1. Start the app with `bun run tauri dev`.
2. Use Peekaboo `see` (with `app_target: "pzql"`) to capture the app window and get element IDs.
3. Interact with elements using `click`, `type`, `scroll`, etc.
4. Verify results with another `see` capture.

### Screenshots
Always save screenshots to `.screenshots/` in the project root (e.g. `path: "/Users/calum/code/github.com/0x63616c/pzql/.screenshots/my-screenshot.png"`). Never save to `/tmp` or the desktop.

## Library Selection Philosophy
Pick libraries that are **typesafe, have good guardrails, and are hard for LLMs to get wrong**. When an LLM makes a mistake, it should be obvious - caught by the type checker, not hidden at runtime. Prefer small API surfaces over flexible-but-footgunny ones. This applies to every dependency choice.

## Frontend Stack
- **Vite** - bundler/dev server
- **shadcn/ui** - component library
- **Tailwind CSS** - styling
- **Heroicons** - icon library (outline + solid variants, by Tailwind team)
- **Biome** - linter + formatter (replaces ESLint/Prettier)
- **Zustand** - state management (tiny API, excellent TypeScript inference, hard to misuse)
- **TanStack Router** - routing (fully typesafe routes, params, and search params - compile-time errors for wrong paths)
Lean on these libraries - do not reinvent the wheel. Use shadcn components and Tailwind utilities instead of writing custom CSS or building UI primitives from scratch. Every line of code is a liability.

## Documentation Lookup
Use the **Context7 MCP** to fetch up-to-date docs for our stack (Tauri v2, React 19, Rust, Tailwind CSS, shadcn/ui, Vite, Biome) before writing code that depends on library APIs. Don't guess at APIs from training data - pull current docs via Context7.

## Git
Keep commits atomic - one logical change per commit. Don't bundle unrelated changes together.

**Always use the `/commit` skill to commit.** No exceptions - not for one-liners, quick fixes, small changes, urgent commits, or any other rationalization. Never run `git commit` directly. Always invoke the commit skill.

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

## Theming
Supports multiple themes (not just light/dark). All themes live in `src/themes/`, one file per theme.

- A TypeScript `Theme` interface is the **single source of truth** for all design tokens - colors, border radius, shadows, and anything else that varies between themes.
- Each theme is an object satisfying that interface. Missing a token = compile error.
- A small utility converts a theme object to CSS custom properties at runtime.
- shadcn/ui and Tailwind consume the CSS variables as normal (`bg-background`, `text-foreground`, etc.).
- **Rule**: if it changes between themes, it's a token in the `Theme` interface. If it doesn't change, it's a Tailwind utility or a constant. Don't tokenize everything - just what varies.
- **Never hardcode colors.** No `bg-red-500`, `text-blue-300`, `bg-[#ff0000]`, `text-[rgb(...)]`, or inline `color`/`background` styles. Always use theme tokens: `bg-background`, `text-foreground`, `bg-primary`, `text-muted-foreground`, `border-border`, etc. The only exception is `transparent`, `inherit`, and `currentColor`.

## Parallel Work - MANDATORY

<EXTREMELY-IMPORTANT>
When ANY skill or workflow involves parallel work, multiple independent tasks, or subagents:

**DO NOT USE:**
- `Task` tool with `subagent_type` to spawn standalone subagents
- Fire-and-forget `Task` calls running in background
- Multiple independent `Task` invocations as a substitute for coordination

**ALWAYS USE Agent Teams instead:**
1. `TeamCreate` - create a team first
2. `TaskCreate` / `TaskList` / `TaskUpdate` - manage work items in the team's shared task list
3. `Task` tool WITH `team_name` and `name` parameters - spawn teammates that join the team
4. `SendMessage` - coordinate between teammates

This applies to ALL superpowers skills that dispatch work in parallel, including but not limited to `dispatching-parallel-agents` and `subagent-driven-development`. No exceptions. If the skill says "spawn subagents" or "dispatch agents", use Agent Teams.
</EXTREMELY-IMPORTANT>

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
