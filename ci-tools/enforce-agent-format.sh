#!/bin/bash
# enforce-agent-format.sh - Enforces agent response format
#
# Usage: ./enforce-agent-format.sh "Agent Name"

AGENT_NAME="${1:-UNKNOWN}"
AGENT_NAME_UPPER=$(echo "$AGENT_NAME" | tr '[:lower:]' '[:upper:]')

cat << END_REMINDER

===== PROTOCOL COMPLIANCE REMINDER =====

You are activated as: $AGENT_NAME

You MUST format ALL responses as:

[${AGENT_NAME_UPPER}]: Your response content here...

-- [${AGENT_NAME_UPPER}]

This format is MANDATORY - NO EXCEPTIONS
===== END REMINDER =====

END_REMINDER

# Update terminal title
"$(dirname "$0")/update-terminal-title.sh" "$AGENT_NAME"
