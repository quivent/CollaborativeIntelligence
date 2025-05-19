#!/bin/bash
# Scheduled Knowledge Distribution Workflow
# This script is designed to be run as a scheduled task (e.g., via cron)
# to regularly process and distribute knowledge throughout the system.

# Configuration
ROOT_DIR="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
KNOWLEDGE_DIR="$ROOT_DIR/scripts/knowledge"
LOG_DIR="$ROOT_DIR/KnowledgeRegistry/logs"
THRESHOLD=0.7

# Ensure log directory exists
mkdir -p "$LOG_DIR"

# Create timestamped log file
TIMESTAMP=$(date +"%Y%m%d_%H%M%S")
LOG_FILE="$LOG_DIR/workflow_$TIMESTAMP.log"

# Log start
echo "Knowledge Distribution Workflow Started: $(date)" > "$LOG_FILE"
echo "Root Directory: $ROOT_DIR" >> "$LOG_FILE"
echo "Threshold: $THRESHOLD" >> "$LOG_FILE"
echo "----------------------------------------" >> "$LOG_FILE"

# Run the workflow
cd "$ROOT_DIR" || { echo "Failed to change to root directory"; exit 1; }

echo "Executing knowledge workflow script..." >> "$LOG_FILE"
"$KNOWLEDGE_DIR/knowledge_workflow.py" --dir "$ROOT_DIR" --threshold "$THRESHOLD" >> "$LOG_FILE" 2>&1
WORKFLOW_STATUS=$?

# Log completion status
echo "----------------------------------------" >> "$LOG_FILE"
if [ $WORKFLOW_STATUS -eq 0 ]; then
    echo "Workflow completed successfully: $(date)" >> "$LOG_FILE"
else
    echo "Workflow completed with errors (status $WORKFLOW_STATUS): $(date)" >> "$LOG_FILE"
fi

# Create a link to the latest log
ln -sf "$LOG_FILE" "$LOG_DIR/latest_workflow.log"

# Exit with the workflow status
exit $WORKFLOW_STATUS