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
- `bun run app dev` - run full app in dev mode (native window + Vite + WS server for Chrome)
- `bun run app build` - package desktop app
- `bun run build` - type-check + Vite build (used by Tauri internally)

## Testing

Verify UI and backend changes by running the app in Chrome via the dev-server. This exercises real Rust logic through the WebSocket bridge - not just a screenshot of the WKWebView.

### Dev verification workflow

```sh
# Everything in one command
bun run app dev

# Native window opens automatically.
# For Chrome testing, also open http://localhost:1420
```

In debug builds, the Tauri app spawns a WebSocket server on port 1421 alongside the native window. The frontend detects no Tauri runtime in Chrome and routes all IPC calls over WebSocket. Both Chrome and the native window work simultaneously.

For standalone backend-only iteration (faster Rust compile, no native window):
```sh
cd src-tauri && cargo run --features dev-server
# Open http://localhost:1420 (start Vite separately with: vite)
```

### Visual Verification
Use the Playwright CLI (`npx playwright`) to automate Chrome for visual verification. Save screenshots to `.screenshots/` in the project root - never to `/tmp` or the desktop.

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

**Conventional commits** - all commit messages follow the format `<type>[scope][!]: <description>`. The `/commit` skill has the full reference. The critical rule: any commit that changes public API, IPC contracts, config formats, or behavior that downstream code depends on **must** have `!` after the type (e.g. `feat!:`, `refactor(ipc)!:`) and optionally a `BREAKING CHANGE:` footer explaining migration steps.

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
- `src-tauri/` - Cargo workspace root
  - `src-tauri/src/` - Rust Tauri backend (commands, dev-server)
  - `src-tauri/crates/pzql-ipc/` - shared IPC types (WsCommandEntry, WsEventEntry)
  - `src-tauri/crates/pzql-macros/` - proc macros (`#[command]`, `#[event]`)
- IPC via dual-transport layer - frontend calls Rust through `src/ipc.ts`

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

## IPC: Commands and Events

All frontend-backend communication uses the dual-transport IPC layer. **Never call `invoke()` or `emit()` directly.**

### Adding a command (frontend calls Rust)

1. Define the function in `src-tauri/src/` with `#[command]`:
   ```rust
   use pzql_macros::command;

   #[command]
   async fn my_command(arg: String) -> Result<String, String> {
       Ok(format!("got: {arg}"))
   }
   ```
2. Add to `collect_commands![]` in `src-tauri/src/lib.rs` (one line - for Tauri IPC + TS types).
3. Add one line to `src/ws-bindings.ts` - TypeScript will not compile if you forget.
4. Run `bun run app dev` once to regenerate `src/bindings.ts`.
5. Call via `commands.myCommand(arg)` imported from `src/ipc.ts`.

### Adding an event (Rust pushes to frontend)

1. Define the struct with `#[event]`:
   ```rust
   use pzql_macros::event;

   #[event]
   #[derive(Serialize, Deserialize, specta::Type)]
   struct MyEvent { field: String }
   ```
2. Emit: `MyEvent { field: "value".into() }.emit(&app_handle)?`
3. Listen in TypeScript via `events` from `src/ipc.ts`.

### Dev verification in Chrome

```sh
bun run app dev
# Open http://localhost:1420 in Chrome
```

Full app with real Rust backend runs in Chrome. Claude uses Playwright on Chrome to verify.

### Key files

| File | Edit? | Purpose |
|------|-------|---------|
| `src/bindings.ts` | Never | Auto-generated by tauri-specta |
| `src/ws-bindings.ts` | Per new command (1 line) | WS transport - TypeScript enforced |
| `src/ws-client.ts` | Never | WebSocket transport layer |
| `src/ipc.ts` | Never | Transport bridge |
