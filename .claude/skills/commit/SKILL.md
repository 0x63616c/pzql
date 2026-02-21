---
name: commit
description: ALWAYS use when the user wants to commit their changes
user_invocable: true
---

Commit staged and unstaged changes using conventional commits. If changes seem unrelated, create separate commits per change.

## Conventional Commit Format

```
<type>[optional scope][!]: <description>

[optional body]

[optional footer(s)]
```

### Types

| Type | When to use |
|------|-------------|
| `feat` | New feature or capability |
| `fix` | Bug fix |
| `refactor` | Code change that neither fixes a bug nor adds a feature |
| `style` | Formatting, linting fixes, no logic change |
| `docs` | Documentation only |
| `ci` | CI/CD config and scripts |
| `chore` | Build process, tooling, deps, anything that doesn't touch app code |
| `perf` | Performance improvement |
| `test` | Adding or fixing tests |

### Breaking Changes

**Any commit that changes public API, IPC contracts, config formats, or behavior that downstream code depends on MUST be marked as breaking.**

Two ways to mark a breaking change (use both together for clarity):

1. **`!` after the type/scope**: `feat!: redesign IPC protocol` or `feat(ipc)!: redesign protocol`
2. **`BREAKING CHANGE:` footer**: explains what broke and how to migrate

```
feat!: replace JSON IPC with MessagePack

BREAKING CHANGE: All IPC messages now use MessagePack instead of JSON.
Update ws-bindings.ts call signatures accordingly.
```

The `!` is the minimum - always include it. The footer is for migration details when the breakage isn't obvious from the description.

Examples of breaking changes:
- Renaming or removing a Tauri command
- Changing a command's argument or return type
- Changing the IPC protocol or message format
- Removing or renaming a frontend route
- Changing config file format or location
- Removing a public export

Examples of NON-breaking changes:
- Adding a new command (additive)
- Adding an optional parameter with a default
- Internal refactors that don't change external behavior
- Bug fixes (even if they change incorrect behavior)
