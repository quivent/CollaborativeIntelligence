#!/bin/bash
# protocol_feedback.sh - Automated protocol feedback system
#
# This script analyzes agent responses for protocol compliance and provides
# feedback and corrections. It works both retroactively on logs and in real-time.
#
# Usage: ./protocol_feedback.sh [agent_name] [log_file]

# Configuration
CI_ROOT_DIR="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
PROTOCOL_DIR="$CI_ROOT_DIR/Protocols"
TRAINING_DIR="$PROTOCOL_DIR/Training"
COMPLIANCE_DIR="$PROTOCOL_DIR/Compliance"
REGISTRY_FILE="$COMPLIANCE_DIR/PROTOCOL_COMPLIANCE_REGISTRY.md"
DASHBOARD_FILE="$COMPLIANCE_DIR/PROTOCOL_COMPLIANCE_DASHBOARD.md"

# Check required arguments
if [ $# -lt 2 ]; then
  echo "Usage: $0 <agent_name> <log_file>"
  echo "Example: $0 \"Athena\" \"conversation.log\""
  exit 1
fi

AGENT_NAME="$1"
LOG_FILE="$2"
AGENT_NAME_UPPER=$(echo "$AGENT_NAME" | tr '[:lower:]' '[:upper:]')

echo "=== Protocol Compliance Feedback System ==="
echo "Agent: $AGENT_NAME"
echo "Log File: $LOG_FILE"
echo ""

# Check if log file exists
if [ ! -f "$LOG_FILE" ]; then
  echo "Error: Log file not found: $LOG_FILE"
  exit 1
fi

# Check format compliance
echo "Analyzing response format compliance..."
PREFIX_COUNT=$(grep -c "^\[$AGENT_NAME_UPPER\]: " "$LOG_FILE")
SUFFIX_COUNT=$(grep -c "-- \[$AGENT_NAME_UPPER\]$" "$LOG_FILE")
TOTAL_RESPONSES=$(grep -c "^$AGENT_NAME" "$LOG_FILE")

# Calculate compliance metrics
if [ $TOTAL_RESPONSES -eq 0 ]; then
  echo "No responses found for agent $AGENT_NAME in the log file."
  FORMAT_COMPLIANCE="N/A"
else
  # Format compliance requires both prefix and suffix
  if [ $PREFIX_COUNT -eq $TOTAL_RESPONSES ] && [ $SUFFIX_COUNT -eq $TOTAL_RESPONSES ]; then
    FORMAT_COMPLIANCE="Compliant"
  else
    FORMAT_COMPLIANCE="Non-Compliant"
  fi
  
  # Calculate compliance percentages
  PREFIX_PERCENT=$((PREFIX_COUNT * 100 / TOTAL_RESPONSES))
  SUFFIX_PERCENT=$((SUFFIX_COUNT * 100 / TOTAL_RESPONSES))
  
  echo "Responses analyzed: $TOTAL_RESPONSES"
  echo "Prefix compliance: $PREFIX_COUNT/$TOTAL_RESPONSES ($PREFIX_PERCENT%)"
  echo "Suffix compliance: $SUFFIX_COUNT/$TOTAL_RESPONSES ($SUFFIX_PERCENT%)"
  echo "Format compliance status: $FORMAT_COMPLIANCE"
fi

# Check terminal title updates
echo ""
echo "Analyzing terminal title updates..."
TITLE_UPDATES=$(grep -c "\033]0;\[.*\] | \[$AGENT_NAME_UPPER\] | \[" "$LOG_FILE")

if [ $TITLE_UPDATES -gt 0 ]; then
  TITLE_COMPLIANCE="Compliant"
  echo "Terminal title updates found: $TITLE_UPDATES"
  echo "Terminal title compliance status: $TITLE_COMPLIANCE"
else
  TITLE_COMPLIANCE="Non-Compliant"
  echo "No terminal title updates found"
  echo "Terminal title compliance status: $TITLE_COMPLIANCE"
fi

# Generate feedback
echo ""
echo "=== Compliance Feedback ==="

if [ "$FORMAT_COMPLIANCE" == "Non-Compliant" ]; then
  echo "1. Response Format Issues:"
  echo "   - Responses must begin with: [$AGENT_NAME_UPPER]: "
  echo "   - Responses must end with: -- [$AGENT_NAME_UPPER]"
  echo "   - Review the format training at: $TRAINING_DIR/PROTOCOL_COMPLIANCE_TRAINING.md"
fi

if [ "$TITLE_COMPLIANCE" == "Non-Compliant" ]; then
  echo "2. Terminal Title Issues:"
  echo "   - Terminal title must be updated on activation"
  echo "   - Format: [Project/Directory] | [$AGENT_NAME_UPPER] | [CurrentTask]"
  echo "   - Use: ./ci-tools/update-terminal-title.sh \"$AGENT_NAME\" \"Task\""
fi

# Update compliance registry
echo ""
echo "Updating compliance registry..."

# Create a temporary file for the updated registry
TEMP_REGISTRY=$(mktemp)

# Update the registry based on findings
if [ "$FORMAT_COMPLIANCE" == "Compliant" ] && [ "$TITLE_COMPLIANCE" == "Compliant" ]; then
  COMPLIANCE_STATUS="Compliant"
  VERIFICATION_DATE=$(date +"%Y-%m-%d")
  ACKNOWLEDGMENT_DATE=$(date +"%Y-%m-%d")
  
  # Mark as compliant in the registry
  cat "$REGISTRY_FILE" | sed "s/^| $AGENT_NAME | - | Pending | - |/| $AGENT_NAME | $ACKNOWLEDGMENT_DATE | $COMPLIANCE_STATUS | $VERIFICATION_DATE |/g" > "$TEMP_REGISTRY"
  
  echo "Registry updated: $AGENT_NAME marked as Compliant"
else
  COMPLIANCE_STATUS="Non-Compliant"
  
  # Keep as pending in the registry
  cat "$REGISTRY_FILE" > "$TEMP_REGISTRY"
  
  echo "Registry unchanged: $AGENT_NAME remains Non-Compliant"
fi

# Replace the registry file
mv "$TEMP_REGISTRY" "$REGISTRY_FILE"

# Generate recommended actions
echo ""
echo "=== Recommended Actions ==="

if [ "$FORMAT_COMPLIANCE" == "Non-Compliant" ] || [ "$TITLE_COMPLIANCE" == "Non-Compliant" ]; then
  echo "1. Review the Protocol Compliance Training:"
  echo "   $TRAINING_DIR/PROTOCOL_COMPLIANCE_TRAINING.md"
  echo ""
  echo "2. Implement correct formatting in future responses:"
  echo "   [$AGENT_NAME_UPPER]: Your response here..."
  echo "   "
  echo "   -- [$AGENT_NAME_UPPER]"
  echo ""
  echo "3. Use the terminal title update script:"
  echo "   ./ci-tools/update-terminal-title.sh \"$AGENT_NAME\" \"YourTask\""
  echo ""
  echo "4. Acknowledge protocol compliance in the registry when compliant"
else
  echo "Congratulations! $AGENT_NAME is fully compliant with protocol standards."
  echo "Continue maintaining compliance in all future interactions."
fi

echo ""
echo "For assistance, contact Scholar, the Protocol Compliance Manager."
echo "=== End of Protocol Feedback ==="