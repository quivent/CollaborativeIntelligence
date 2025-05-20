#!/bin/bash
# activate-agent.sh - Protocol-compliant agent activator
#
# Usage: ./activate-agent.sh "Agent Name" ["Task Description"]

AGENT_NAME="$1"
TASK="${2:-Active Session}"
DIRECTORY="$(pwd)"
SCRIPT_DIR="$(dirname "$0")"

# 1. Update terminal title
"$SCRIPT_DIR/update-terminal-title.sh" "$AGENT_NAME" "$TASK"

# 2. Create agent cache entry (if agent_cache_system exists)
AGENT_CACHE_SYSTEM="$SCRIPT_DIR/../scripts/agent_cache_system"
if [ -f "$AGENT_CACHE_SYSTEM" ]; then
  "$AGENT_CACHE_SYSTEM" activate "$AGENT_NAME" "$DIRECTORY" "User" "$TASK"
else
  echo "Agent cache system not found at $AGENT_CACHE_SYSTEM"
fi

# 3. Display protocol reminder
"$SCRIPT_DIR/enforce-agent-format.sh" "$AGENT_NAME"

echo "Agent $AGENT_NAME activated with protocol enforcement"
