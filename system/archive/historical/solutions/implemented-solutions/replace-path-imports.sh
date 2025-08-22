#!/bin/bash
# Single-purpose script to replace the path imports
# Created as a separate script to ensure precise implementation

ENV_RS_PATH="/Users/joshkornreich/Documents/Projects/Terminals/Claude/src-tauri/src/env.rs"

# Check if the file exists
if [ ! -f "$ENV_RS_PATH" ]; then
  echo "Error: env.rs not found"
  exit 1
fi

# Create a temporary file with the new content
TEMP_FILE=$(mktemp)

# Replace the path import line
sed 's/use tauri::api::path:{app_dir, resource_dir};/use tauri::api::path:{app_config_dir, app_data_dir, resource_dir};/' "$ENV_RS_PATH" > "$TEMP_FILE"

# Replace the original file
mv "$TEMP_FILE" "$ENV_RS_PATH"

echo "Successfully updated path imports"

# Now replace app_dir calls with app_data_dir
TEMP_FILE=$(mktemp)

# Replace all occurrences of app_dir( with app_data_dir(
sed 's/app_dir(/app_data_dir(/g' "$ENV_RS_PATH" > "$TEMP_FILE"

# Replace the original file
mv "$TEMP_FILE" "$ENV_RS_PATH"

echo "Successfully replaced app_dir calls with app_data_dir"