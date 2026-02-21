#!/usr/bin/env bash
# Block git worktree commands. Work on the main branch instead.

set -euo pipefail

input="$(cat)"
tool_name="$(echo "$input" | jq -r '.tool_name')"

case "$tool_name" in
  Bash)
    command="$(echo "$input" | jq -r '.tool_input.command // empty')"
    ;;
  EnterWorktree)
    echo '{"decision":"block","reason":"Do not use worktrees. Work directly on the main branch."}'
    exit 0
    ;;
  *)
    exit 0
    ;;
esac

if echo "$command" | grep -qiE 'git\s+worktree'; then
  echo '{"decision":"block","reason":"Do not use git worktree commands. Work directly on the main branch."}'
  exit 0
fi
