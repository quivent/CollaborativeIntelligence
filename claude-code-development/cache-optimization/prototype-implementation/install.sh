#!/bin/bash

# Claude Code Cache System Installer
# This script installs the Claude Code cache wrapper with offline/cached mode

set -e

echo -e "\033[1;36m[INSTALLER]\033[0m Installing Claude Code Cache System..."

# Define paths
INSTALL_DIR="$HOME/.local/bin"
CACHE_DIR="$HOME/.claude/cache"
CONFIG_DIR="$HOME/.claude"
SESSION_FILE="$CONFIG_DIR/session.json"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Create necessary directories
mkdir -p "$INSTALL_DIR"
mkdir -p "$CACHE_DIR"
mkdir -p "$CONFIG_DIR"

# Copy files to install directory
cp "$SCRIPT_DIR/claude-cache" "$INSTALL_DIR/"
cp "$SCRIPT_DIR/claude-cache-module.js" "$INSTALL_DIR/"
cp "$SCRIPT_DIR/athena" "$INSTALL_DIR/" 2>/dev/null || echo -e "\033[1;33m[WARNING]\033[0m Athena activation script not found, skipping"

# Make files executable
chmod +x "$INSTALL_DIR/claude-cache"
chmod +x "$INSTALL_DIR/athena" 2>/dev/null || true

# Check if original claude command exists
if ! command -v claude &> /dev/null; then
    echo -e "\033[1;31m[ERROR]\033[0m Claude Code CLI not found. Please install it first:"
    echo "npm install -g @anthropic-ai/claude-code"
    exit 1
fi

# Create a symlink to the original Claude CLI
ORIGINAL_CLAUDE=$(which claude)
echo -e "\033[1;36m[INSTALLER]\033[0m Found original Claude CLI at: $ORIGINAL_CLAUDE"

# Create wrapper for 'claude' command
echo -e "\033[1;36m[INSTALLER]\033[0m Creating 'claude' wrapper..."
cat > "$INSTALL_DIR/claude" <<EOF
#!/bin/bash
# Claude wrapper that redirects to claude-cache
"$INSTALL_DIR/claude-cache" "\$@"
EOF
chmod +x "$INSTALL_DIR/claude"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
    echo -e "\033[1;36m[INSTALLER]\033[0m Adding $INSTALL_DIR to your PATH..."
    
    # Detect shell
    SHELL_NAME=$(basename "$SHELL")
    
    if [[ "$SHELL_NAME" == "bash" ]]; then
        SHELL_RC="$HOME/.bashrc"
    elif [[ "$SHELL_NAME" == "zsh" ]]; then
        SHELL_RC="$HOME/.zshrc"
    else
        echo -e "\033[1;33m[WARNING]\033[0m Unsupported shell. Please add the following to your shell configuration file:"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\""
        echo ""
    fi
    
    if [[ -n "$SHELL_RC" ]]; then
        echo "" >> "$SHELL_RC"
        echo "# Added by Claude Code Cache installer" >> "$SHELL_RC"
        echo "export PATH=\"$INSTALL_DIR:\$PATH\"" >> "$SHELL_RC"
        echo -e "\033[1;36m[INSTALLER]\033[0m Added to $SHELL_RC. Please restart your terminal or run: source $SHELL_RC"
    fi
fi

# Create initial config if it doesn't exist
CONFIG_FILE="$CONFIG_DIR/config.json"
if [[ ! -f "$CONFIG_FILE" ]]; then
    echo "{}" > "$CONFIG_FILE"
fi

# Add cache configuration to config file
node -e "
const fs = require('fs');
const configPath = '$CONFIG_FILE';
let config = {};
try {
  config = JSON.parse(fs.readFileSync(configPath, 'utf8'));
} catch (e) {
  config = {};
}
config.cache = config.cache || {
  enabled: true,
  ttl: 3600000,
  maxSize: 104857600,
  persistOnExit: true,
  validateOnStartup: true
};
fs.writeFileSync(configPath, JSON.stringify(config, null, 2));
"

# Create session file if it doesn't exist
if [[ ! -f "$SESSION_FILE" ]]; then
    echo '{"mode":"cached"}' > "$SESSION_FILE"
    echo -e "\033[1;36m[INSTALLER]\033[0m Created session file with default cached mode"
fi

echo -e "\033[1;32m[SUCCESS]\033[0m Installation complete!"
echo ""
echo -e "\033[1;36m[USAGE]\033[0m Basic commands:"
echo "  claude \"What does this file do?\"       - Use Claude in cached mode (default)"
echo "  claude --query \"What does this do?\"   - Force online mode for one query"
echo "  claude !default                        - Switch to online mode permanently"
echo "  claude !cached                         - Switch back to cached mode"
echo "  claude athena                          - Shortcut for switching to cached mode"
echo "  athena                                 - Activate Athena agent with zero token usage"
echo "  athena \"your question\"                - Ask Athena directly with minimal token usage"
echo "  claude !status                         - Show current mode and cache status"
echo ""
echo -e "\033[1;36m[USAGE]\033[0m Cache management:"
echo "  claude cache stats                     - Show cache statistics"
echo "  claude cache clear                     - Clear cache"
echo "  claude cache enable                    - Enable caching"
echo "  claude cache disable                   - Disable caching"
echo ""
echo -e "\033[1;33m[NOTE]\033[0m You may need to restart your terminal or run: source $SHELL_RC"
echo -e "\033[1;33m[NOTE]\033[0m Claude starts in cached mode. Use !default to switch to online mode."
echo -e "\033[1;33m[NOTE]\033[0m To troubleshoot issues with cached mode, use !default to return to online mode."