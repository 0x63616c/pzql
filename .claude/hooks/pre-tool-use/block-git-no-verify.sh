#!/usr/bin/env bash
# Block --no-verify flag in git commands.

set -euo pipefail

input="$(cat)"
tool_name="$(echo "$input" | jq -r '.tool_name')"

if [ "$tool_name" != "Bash" ]; then
  exit 0
fi

command="$(echo "$input" | jq -r '.tool_input.command // empty')"

if echo "$command" | grep -qE '\bgit\b' && echo "$command" | grep -qE '\-\-no-verify'; then
  echo '{"decision":"block","reason":"Do not use --no-verify. Run the commit with hooks enabled and fix any issues they flag."}'
  exit 0
fi
