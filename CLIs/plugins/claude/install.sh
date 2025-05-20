#!/bin/bash
# Install Collaborative Intelligence plugin for Claude CLI

CI_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONFIG_DIR="$HOME/.claude"
CONFIG_FILE="$CONFIG_DIR/config.json"

# Ensure claude config directory exists
mkdir -p "$CONFIG_DIR"

# Check if config file exists
if [ ! -f "$CONFIG_FILE" ]; then
  # Create empty config file
  echo "{}" > "$CONFIG_FILE"
fi

# Make implementation script executable
chmod +x "$CI_PATH/claude-cli-plugin/claude-init-impl.sh"

# Function to update or create config entry
update_config() {
  # Check if jq is installed
  if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not installed. Please install jq first."
    exit 1
  fi

  # Create plugin configuration
  NEW_CONFIG=$(jq --arg path "$CI_PATH" '.plugins."collaborative-intelligence" = {
    "enabled": true,
    "path": $path,
    "commands": {
      "init": {
        "script": "claude-cli-plugin/claude-init-impl.sh",
        "description": "Initialize a new project with Collaborative Intelligence" 
      }
    }
  }' "$CONFIG_FILE")

  # Update the config file
  echo "$NEW_CONFIG" > "$CONFIG_FILE"
}

update_config

echo "✅ Collaborative Intelligence plugin installed for Claude CLI"
echo ""
echo "You can now use: claude init my-project --with-ci"
echo ""
echo "To verify installation:"
echo "  claude plugin list  # Should show collaborative-intelligence"
echo "  claude help init    # Should show the init command options"