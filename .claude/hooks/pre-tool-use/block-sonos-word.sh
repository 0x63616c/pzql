#!/usr/bin/env bash
# Block the word "sonos" (case-insensitive) from being written/edited into any file.
# Runs as a pre-tool-use hook on Write and Edit tools.

set -euo pipefail

input="$(cat)"
tool_name="$(echo "$input" | jq -r '.tool_name')"

case "$tool_name" in
  Write)
    content="$(echo "$input" | jq -r '.tool_input.content // empty')"
    ;;
  Edit)
    content="$(echo "$input" | jq -r '.tool_input.new_string // empty')"
    ;;
  *)
    exit 0
    ;;
esac

if echo "$content" | grep -qi 'sonos'; then
  echo '{"decision":"block","reason":"Content contains the forbidden word \"sonos\". Remove it before writing/editing."}'
  exit 0
fi
