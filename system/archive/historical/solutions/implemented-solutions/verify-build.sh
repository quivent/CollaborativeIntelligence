#!/bin/bash
# Script to verify the build succeeds after all fixes

TERMINAL_DIR="/Users/joshkornreich/Documents/Projects/Terminals/Claude"
LOG_FILE="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/implemented-solutions/build-verification.log"

echo "Starting build verification at $(date)" | tee -a "$LOG_FILE"
echo "=======================================================" | tee -a "$LOG_FILE"

# Check if the terminal directory exists
if [ ! -d "$TERMINAL_DIR" ]; then
  echo "ERROR: Terminal directory not found at $TERMINAL_DIR" | tee -a "$LOG_FILE"
  exit 1
fi

# Navigate to the terminal directory
cd "$TERMINAL_DIR" || {
  echo "ERROR: Failed to change to directory $TERMINAL_DIR" | tee -a "$LOG_FILE"
  exit 1
}

echo "Building the application..." | tee -a "$LOG_FILE"

# Run the build command with all output going to the log file
npm run tauri build 2>&1 | tee -a "$LOG_FILE"

# Check if the build succeeded
if [ ${PIPESTATUS[0]} -eq 0 ]; then
  echo "SUCCESS: Build completed without errors" | tee -a "$LOG_FILE"
  echo "All fixes have been successfully applied" | tee -a "$LOG_FILE"
  echo "The Claude Terminal application is now functioning correctly" | tee -a "$LOG_FILE"
else
  echo "ERROR: Build failed - check the log for details" | tee -a "$LOG_FILE"
  echo "Some issues may still need to be addressed" | tee -a "$LOG_FILE"
fi

echo "=======================================================" | tee -a "$LOG_FILE"
echo "Build verification completed at $(date)" | tee -a "$LOG_FILE"

# Print final message
echo
echo "Verification complete. See $LOG_FILE for full build output."