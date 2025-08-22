#!/bin/bash
# Single-purpose script to add the missing Manager import
# Created to fix the issue with sed in the main script

ENV_RS_PATH="/Users/joshkornreich/Documents/Projects/Terminals/Claude/src-tauri/src/env.rs"

# Check if the file exists
if [ ! -f "$ENV_RS_PATH" ]; then
  echo "Error: env.rs not found"
  exit 1
fi

# Check if import already exists
if grep -q "use tauri::Manager;" "$ENV_RS_PATH"; then
  echo "tauri::Manager already imported"
  exit 0
fi

# Create a temporary file with the new content
TEMP_FILE=$(mktemp)

# Add the import after line 7 (after the path import)
awk 'NR==7{print; print "use tauri::Manager;"} NR!=7{print}' "$ENV_RS_PATH" > "$TEMP_FILE"

# Replace the original file
mv "$TEMP_FILE" "$ENV_RS_PATH"

echo "Successfully added tauri::Manager import"