#!/usr/bin/env bash
# common.sh - Shared utility functions for the CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Global variables
CI_ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CI_TOOLS_DIR="$CI_ROOT_DIR/ci-tools"
CI_LIB_DIR="$CI_TOOLS_DIR/lib"
CI_CMD_DIR="$CI_TOOLS_DIR/cmd"

# Path handling
#
# Get absolute path from relative path
# Usage: get_absolute_path <path>
get_absolute_path() {
  local path="$1"
  if [[ -d "$path" ]]; then
    (cd "$path" && pwd)
  elif [[ -f "$path" ]]; then
    local dir=$(dirname "$path")
    local file=$(basename "$path")
    echo "$(cd "$dir" && pwd)/$file"
  else
    echo "$path" | sed -e "s#^~#$HOME#"
  fi
}

# Verify that a path exists
# Usage: verify_path_exists <path> [error_message]
verify_path_exists() {
  local path="$1"
  local error_message="${2:-Path does not exist: $path}"
  
  if [[ ! -e "$path" ]]; then
    echo "$error_message" >&2
    return 1
  fi
  return 0
}

# Argument parsing
#
# Parse command line arguments
# Usage: parse_args "$@"
parse_args() {
  # Default values
  export CI_VERBOSE=false
  export CI_DEBUG=false
  
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --verbose|-v)
        export CI_VERBOSE=true
        shift
        ;;
      --debug|-d)
        export CI_DEBUG=true
        shift
        ;;
      --help|-h)
        show_help
        exit 0
        ;;
      *)
        # Store positional args
        if [[ -z "${CI_COMMAND:-}" ]]; then
          export CI_COMMAND="$1"
        elif [[ -z "${CI_SUBCOMMAND:-}" ]]; then
          export CI_SUBCOMMAND="$1"
        else
          export CI_ARGS+=("$1")
        fi
        shift
        ;;
    esac
  done
  
  # Validate required args
  if [[ -z "${CI_COMMAND:-}" ]]; then
    echo "Error: No command specified" >&2
    show_help
    exit 1
  fi
}

# File operations
#
# Create a file with content if it doesn't exist
# Usage: create_file_if_not_exists <file_path> <content>
create_file_if_not_exists() {
  local file_path="$1"
  local content="$2"
  
  if [[ ! -f "$file_path" ]]; then
    mkdir -p "$(dirname "$file_path")"
    echo "$content" > "$file_path"
    return 0
  fi
  return 1
}

# Read a file and return its content
# Usage: read_file <file_path>
read_file() {
  local file_path="$1"
  
  if [[ -f "$file_path" ]]; then
    cat "$file_path"
    return 0
  fi
  echo "Error: File not found: $file_path" >&2
  return 1
}

# Add a line to a file if it doesn't exist
# Usage: add_line_if_not_exists <file_path> <line>
add_line_if_not_exists() {
  local file_path="$1"
  local line="$2"
  
  if [[ ! -f "$file_path" ]]; then
    mkdir -p "$(dirname "$file_path")"
    echo "$line" > "$file_path"
    return 0
  fi
  
  if ! grep -qF "$line" "$file_path"; then
    echo "$line" >> "$file_path"
    return 0
  fi
  return 1
}

# Execute a command and capture its output
# Usage: exec_command <command> [args...]
exec_command() {
  if [[ "$CI_DEBUG" == "true" ]]; then
    echo "Executing: $*" >&2
  fi
  
  "$@"
  return $?
}

# Import a library file
# Usage: import_lib <library_name>
import_lib() {
  local lib_name="$1"
  local lib_path="$CI_LIB_DIR/${lib_name}.sh"
  
  if [[ -f "$lib_path" ]]; then
    source "$lib_path"
    return 0
  fi
  
  echo "Error: Library not found: $lib_name" >&2
  return 1
}

# Display debug information if debug mode is enabled
# Usage: debug_info <message>
debug_info() {
  if [[ "$CI_DEBUG" == "true" ]]; then
    echo "[DEBUG] $*" >&2
  fi
}

# Display verbose information if verbose mode is enabled
# Usage: verbose_info <message>
verbose_info() {
  if [[ "$CI_VERBOSE" == "true" ]]; then
    echo "[INFO] $*" >&2
  fi
}