#!/bin/bash
# CI command wrapper template for external repository integration
# This script delegates to the external CI tool when available,
# and falls back to the local implementation when needed.

# Command name (replaced during instantiation)
COMMAND_NAME="command"

# Function to check if external CI tool is installed
check_external_ci() {
  if command -v ci-external &> /dev/null; then
    return 0  # External CI tool is available
  elif [ -x "$HOME/.ci/bin/ci" ]; then
    return 0  # External CI tool is available in home directory
  elif [ -n "$CI_EXTERNAL_PATH" ] && [ -x "$CI_EXTERNAL_PATH/bin/ci" ]; then
    return 0  # External CI tool is available at custom path
  else
    return 1  # External CI tool is not available
  fi
}

# Function to get path to external CI tool
get_external_ci_path() {
  if command -v ci-external &> /dev/null; then
    echo "ci-external"
  elif [ -x "$HOME/.ci/bin/ci" ]; then
    echo "$HOME/.ci/bin/ci"
  elif [ -n "$CI_EXTERNAL_PATH" ] && [ -x "$CI_EXTERNAL_PATH/bin/ci" ]; then
    echo "$CI_EXTERNAL_PATH/bin/ci"
  else
    echo ""  # External CI tool not found
  fi
}

# Check for help flag - we handle this directly to avoid passing to both systems
if [[ "$1" == "--help" || "$1" == "-h" ]]; then
  cat << EOF
Usage: ${COMMAND_NAME} [OPTIONS] [ARGUMENTS]

This is a wrapper command that delegates to the external CI tool if available,
or falls back to the local implementation if necessary.

External CI tool status: $(check_external_ci && echo "Available" || echo "Not installed")

For more detailed help, run the command without arguments.
EOF
  exit 0
fi

# Try to use external CI tool
if check_external_ci; then
  CI_EXTERNAL=$(get_external_ci_path)
  
  # If this is a legacy wrapper, show notice only once per day
  NOTICE_FILE="$HOME/.ci_migration_notice_${COMMAND_NAME}"
  if [ ! -f "$NOTICE_FILE" ] || [ "$(find "$NOTICE_FILE" -mtime +1)" ]; then
    echo "ℹ️  Using external CI tool: $CI_EXTERNAL"
    echo "ℹ️  See https://github.com/anthropics/ci-tools for more information"
    
    # Touch notice file to update timestamp
    touch "$NOTICE_FILE" 2>/dev/null || true
  fi
  
  # Execute external CI tool with all arguments
  exec "$CI_EXTERNAL" "$COMMAND_NAME" "$@"
else
  # Fall back to local implementation
  # Source appropriate local script based on command
  SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
  
  case "$COMMAND_NAME" in
    init)
      source "$SCRIPT_DIR/CLIs/ci/bash/cmd/init.sh"
      ;;
    fix)
      source "$SCRIPT_DIR/CLIs/ci/bash/cmd/fix.sh"
      ;;
    verify)
      source "$SCRIPT_DIR/CLIs/ci/bash/cmd/verify.sh"
      ;;
    *)
      echo "Error: Unknown command '$COMMAND_NAME'"
      exit 1
      ;;
  esac
  
  # Show deprecation warning
  echo ""
  echo "⚠️  This local implementation will be deprecated in future releases."
  echo "⚠️  Please install the external CI tool for continued support:"
  echo "    npm install -g @anthropics/ci-tools"
  echo ""
fi