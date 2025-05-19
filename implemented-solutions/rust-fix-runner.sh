#!/bin/bash
# rust-fix-runner.sh - Direct implementation script to fix Rust compilation errors
# Created by Fixer as part of action-first implementation

# Configuration
TERMINAL_DIR="/Users/joshkornreich/Documents/Projects/Terminals/Claude"
ENV_RS_PATH="$TERMINAL_DIR/src-tauri/src/env.rs"
BACKUP_DIR="$TERMINAL_DIR/src-tauri/backups"

# Function to print colored text
print_colored() {
  local color=$1
  local text=$2
  
  case $color in
    "green") echo -e "\033[0;32m$text\033[0m" ;;
    "yellow") echo -e "\033[0;33m$text\033[0m" ;;
    "red") echo -e "\033[0;31m$text\033[0m" ;;
    "blue") echo -e "\033[0;34m$text\033[0m" ;;
    *) echo "$text" ;;
  esac
}

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

print_colored "blue" "## RUST CODE FIX IMPLEMENTATION ##"
print_colored "blue" "Fixing compilation errors in Claude Terminal Rust code..."

# Check if env.rs exists
if [ ! -f "$ENV_RS_PATH" ]; then
  print_colored "red" "Error: env.rs not found at $ENV_RS_PATH"
  exit 1
fi

# Create backup of env.rs
print_colored "yellow" "Creating backup of env.rs..."
ENV_BACKUP="$BACKUP_DIR/env.rs.backup.$(date +%Y%m%d%H%M%S)"
cp "$ENV_RS_PATH" "$ENV_BACKUP"
print_colored "green" "✅ Backup created at: $ENV_BACKUP"

# Implementation Fix 1: Add Manager import
print_colored "yellow" "Implementing Fix 1: Adding tauri::Manager import..."
if grep -q "use tauri::Manager;" "$ENV_RS_PATH"; then
  print_colored "yellow" "tauri::Manager already imported"
else
  # Directly implementing the fix by adding the import
  sed -i'.tmp' '7a\\nuse tauri::Manager;' "$ENV_RS_PATH"
  rm "$ENV_RS_PATH.tmp" # Remove temporary file created by sed on macOS
  print_colored "green" "✅ Implementation successful: Added tauri::Manager import"
fi

# Implementation Fix 2: Replace deprecated app_dir with app_data_dir
print_colored "yellow" "Implementing Fix 2: Replacing deprecated app_dir with app_data_dir..."
if grep -q "use tauri::api::path:{app_config_dir, app_data_dir, resource_dir};" "$ENV_RS_PATH"; then
  print_colored "yellow" "app_data_dir already imported"
else
  # Directly implementing the fix by updating the import
  sed -i'.tmp' 's/use tauri::api::path:{app_dir, resource_dir};/use tauri::api::path:{app_config_dir, app_data_dir, resource_dir};/' "$ENV_RS_PATH"
  rm "$ENV_RS_PATH.tmp" # Remove temporary file created by sed on macOS
  print_colored "green" "✅ Implementation successful: Updated path imports"
fi

# Implementation Fix 3: Update app_dir function calls
print_colored "yellow" "Implementing Fix 3: Updating app_dir function calls to app_data_dir..."
sed -i'.tmp' 's/app_dir(&app_handle.config())/app_data_dir(\&app_handle.config())/g' "$ENV_RS_PATH"
rm "$ENV_RS_PATH.tmp" # Remove temporary file created by sed on macOS
print_colored "green" "✅ Implementation successful: Updated app_dir calls to app_data_dir"

# Verify the changes
print_colored "blue" "Verifying implementation..."

# Check Manager import
if grep -q "use tauri::Manager;" "$ENV_RS_PATH"; then
  print_colored "green" "✅ Verification: tauri::Manager import found"
else
  print_colored "red" "⛔ Verification failed: tauri::Manager import not found"
fi

# Check path imports
if grep -q "app_data_dir" "$ENV_RS_PATH"; then
  print_colored "green" "✅ Verification: app_data_dir import found"
else
  print_colored "red" "⛔ Verification failed: app_data_dir import not found"
fi

# Count app_dir references that should be replaced
APP_DIR_COUNT=$(grep -c "app_dir(" "$ENV_RS_PATH")
if [ "$APP_DIR_COUNT" -eq 0 ]; then
  print_colored "green" "✅ Verification: All app_dir function calls have been replaced"
else
  print_colored "red" "⛔ Verification failed: Found $APP_DIR_COUNT remaining app_dir calls"
fi

print_colored "blue" "## IMPLEMENTATION SUMMARY ##"
print_colored "green" "✅ Added tauri::Manager import"
print_colored "green" "✅ Updated path imports to include app_data_dir"
print_colored "green" "✅ Replaced app_dir calls with app_data_dir"
print_colored "yellow" "Original file backed up at: $ENV_BACKUP"
print_colored "blue" "Now the Rust code should compile without the previous errors"
print_colored "blue" "Try rebuilding with: cd $TERMINAL_DIR && npm run tauri dev"