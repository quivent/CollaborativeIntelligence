#!/bin/bash
# Script to test if Tauri runs correctly after configuration fix

TAURI_PROJECT="/Users/joshkornreich/Documents/Projects/Terminals/Claude"
LOG_FILE="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/tauri-test-script/tauri-test.log"

echo "=== Testing Tauri build at $(date) ===" > "$LOG_FILE"
echo "Changing to directory: $TAURI_PROJECT" >> "$LOG_FILE"

cd "$TAURI_PROJECT" || {
  echo "ERROR: Failed to change directory to $TAURI_PROJECT" >> "$LOG_FILE"
  exit 1
}

echo "Running npm run tauri dev (with timeout of 10 seconds)..." >> "$LOG_FILE"
# Using a PID-based timeout approach since timeout command may not be available
{
  npm run tauri dev 2>&1 & NPM_PID=$!
  sleep 10
  if kill -0 $NPM_PID 2>/dev/null; then
    kill $NPM_PID 2>/dev/null
    echo "Command execution stopped after 10 seconds"
  fi
  wait $NPM_PID 2>/dev/null || echo "Command exited with error or was terminated"
} >> "$LOG_FILE"

echo "=== Build finished at $(date) ===" >> "$LOG_FILE"
echo "Checking for error messages..." >> "$LOG_FILE"

# Check for common error messages
if grep -i "error" "$LOG_FILE" | grep -v "node_modules" | grep -v "deprecation"; then
  echo "ERRORS FOUND in the build process!" >> "$LOG_FILE"
else
  echo "No obvious errors found in output" >> "$LOG_FILE"
fi

# Check specifically for the devProtocol error
if grep -i "devProtocol" "$LOG_FILE" | grep -i "error"; then
  echo "The devProtocol error is STILL PRESENT!" >> "$LOG_FILE"
else
  echo "The devProtocol error appears to be RESOLVED" >> "$LOG_FILE"
fi

echo "=== Test completed ===" >> "$LOG_FILE"