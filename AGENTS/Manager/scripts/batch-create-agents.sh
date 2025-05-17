#!/bin/bash

# Batch Agent Creation Script
# Creates multiple agents from a configuration file
# Usage: ./batch-create-agents.sh <config_file>

set -e

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

# Function to display usage
show_usage() {
    echo "Usage: $0 <config_file>"
    echo ""
    echo "Config file format (JSON):"
    echo '{'
    echo '  "agents": ['
    echo '    {'
    echo '      "name": "AgentName",'
    echo '      "role": "Agent Role",'
    echo '      "description": "Detailed description"'
    echo '    },'
    echo '    ...'
    echo '  ]'
    echo '}'
    echo ""
    echo "Example:"
    echo "  $0 batch-agents.json"
    exit 1
}

# Check arguments
CONFIG_FILE="$1"
if [ -z "$CONFIG_FILE" ]; then
    show_usage
fi

if [ ! -f "$CONFIG_FILE" ]; then
    echo -e "${RED}Error: Config file '$CONFIG_FILE' not found${NC}"
    exit 1
fi

# Set paths
SCRIPT_DIR="$(dirname "$0")"
CREATE_SCRIPT="$SCRIPT_DIR/create-agent.sh"

# Ensure create-agent.sh exists
if [ ! -f "$CREATE_SCRIPT" ]; then
    echo -e "${RED}Error: create-agent.sh not found in $SCRIPT_DIR${NC}"
    exit 1
fi

# Parse JSON config using Python (more reliable than jq)
AGENTS_DATA=$(python3 -c "
import json
import sys

try:
    with open('$CONFIG_FILE', 'r') as f:
        data = json.load(f)
    
    if 'agents' not in data:
        print('ERROR: agents key not found in config', file=sys.stderr)
        sys.exit(1)
    
    for agent in data['agents']:
        if not all(k in agent for k in ['name', 'role']):
            print('ERROR: Each agent must have name and role', file=sys.stderr)
            sys.exit(1)
        
        name = agent['name']
        role = agent['role']
        desc = agent.get('description', role)
        print(f'{name}|{role}|{desc}')
        
except json.JSONDecodeError as e:
    print(f'ERROR: Invalid JSON in config file: {e}', file=sys.stderr)
    sys.exit(1)
except Exception as e:
    print(f'ERROR: {e}', file=sys.stderr)
    sys.exit(1)
")

# Check if parsing was successful
if [ $? -ne 0 ]; then
    echo -e "${RED}Failed to parse config file${NC}"
    exit 1
fi

# Create summary report file
REPORT_FILE="batch-creation-report-$(date +%Y%m%d-%H%M%S).md"
echo "# Batch Agent Creation Report" > "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "**Date**: $(date)" >> "$REPORT_FILE"
echo "**Config File**: $CONFIG_FILE" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "## Agents to Create" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Count agents and list them
AGENT_COUNT=0
while IFS='|' read -r name role description; do
    ((AGENT_COUNT++))
    echo "$AGENT_COUNT. **$name** - $role" >> "$REPORT_FILE"
done <<< "$AGENTS_DATA"

echo "" >> "$REPORT_FILE"
echo "Total agents to create: $AGENT_COUNT" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

# Confirmation prompt
echo -e "${BLUE}About to create $AGENT_COUNT agents:${NC}"
echo "$AGENTS_DATA" | while IFS='|' read -r name role description; do
    echo -e "  - ${GREEN}$name${NC}: $role"
done

echo ""
read -p "Continue with batch creation? (y/N) " -n 1 -r
echo ""

if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "${YELLOW}Batch creation cancelled${NC}"
    exit 0
fi

# Create agents
echo "" >> "$REPORT_FILE"
echo "## Creation Results" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"

SUCCESS_COUNT=0
FAILED_COUNT=0
FAILED_AGENTS=()

while IFS='|' read -r name role description; do
    echo -e "\n${BLUE}Creating agent: $name${NC}"
    echo "### $name" >> "$REPORT_FILE"
    
    if bash "$CREATE_SCRIPT" "$name" "$role" "$description" >> "$REPORT_FILE" 2>&1; then
        echo -e "${GREEN}✓ Successfully created $name${NC}"
        echo "**Status**: Success" >> "$REPORT_FILE"
        ((SUCCESS_COUNT++))
    else
        echo -e "${RED}✗ Failed to create $name${NC}"
        echo "**Status**: Failed" >> "$REPORT_FILE"
        ((FAILED_COUNT++))
        FAILED_AGENTS+=("$name")
    fi
    echo "" >> "$REPORT_FILE"
done <<< "$AGENTS_DATA"

# Summary
echo "" >> "$REPORT_FILE"
echo "## Summary" >> "$REPORT_FILE"
echo "" >> "$REPORT_FILE"
echo "- **Total Attempted**: $AGENT_COUNT" >> "$REPORT_FILE"
echo "- **Successful**: $SUCCESS_COUNT" >> "$REPORT_FILE"
echo "- **Failed**: $FAILED_COUNT" >> "$REPORT_FILE"

if [ ${#FAILED_AGENTS[@]} -gt 0 ]; then
    echo "" >> "$REPORT_FILE"
    echo "### Failed Agents" >> "$REPORT_FILE"
    for agent in "${FAILED_AGENTS[@]}"; do
        echo "- $agent" >> "$REPORT_FILE"
    done
fi

echo "" >> "$REPORT_FILE"
echo "---" >> "$REPORT_FILE"
echo "Report generated on $(date)" >> "$REPORT_FILE"

# Display summary
echo -e "\n${BLUE}Batch Creation Complete${NC}"
echo -e "  ${GREEN}Successful: $SUCCESS_COUNT${NC}"
echo -e "  ${RED}Failed: $FAILED_COUNT${NC}"

if [ ${#FAILED_AGENTS[@]} -gt 0 ]; then
    echo -e "\n${RED}Failed agents:${NC}"
    for agent in "${FAILED_AGENTS[@]}"; do
        echo "  - $agent"
    done
fi

echo -e "\n${YELLOW}Detailed report saved to: $REPORT_FILE${NC}"

# Return non-zero if any failures
[ $FAILED_COUNT -eq 0 ]