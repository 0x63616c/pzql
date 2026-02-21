#!/usr/bin/env bash
# Block git commit commands that contain Co-Authored-By trailers.

set -euo pipefail

input="$(cat)"
tool_name="$(echo "$input" | jq -r '.tool_name')"

if [ "$tool_name" != "Bash" ]; then
  exit 0
fi

command="$(echo "$input" | jq -r '.tool_input.command // empty')"

if echo "$command" | grep -qE '\bgit\b.*\bcommit\b' && echo "$command" | grep -qiE 'co-authored-by'; then
  echo '{"decision":"block","reason":"Do not include Co-Authored-By trailers in commits. The attribution setting is disabled."}'
  exit 0
fi
