#!/bin/bash
# Install Collaborative Intelligence tools

# Determine paths
CI_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INSTALL_DIR="${INSTALL_DIR:-$HOME/.local/bin}"

# Create install directory if it doesn't exist
mkdir -p "$INSTALL_DIR"

# Make script executable
chmod +x "$CI_DIR/ci-tools/ci-init"

# Create wrapper script with environment variable
cat > "$INSTALL_DIR/ci-init" << EOF
#!/bin/bash
# Wrapper for ci-init with pre-configured CI repository path

export CI_REPO_PATH="$CI_DIR"
exec "$CI_DIR/ci-tools/ci-init" "\$@"
EOF

# Make wrapper executable
chmod +x "$INSTALL_DIR/ci-init"

# Check if install directory is in PATH
if [[ ":$PATH:" != *":$INSTALL_DIR:"* ]]; then
  echo "Adding $INSTALL_DIR to PATH in your shell configuration"
  
  # Determine shell
  if [[ "$SHELL" == */zsh ]]; then
    SHELL_CONFIG="$HOME/.zshrc"
  elif [[ "$SHELL" == */bash ]]; then
    SHELL_CONFIG="$HOME/.bashrc"
    # Check for macOS
    if [[ "$(uname)" == "Darwin" ]]; then
      SHELL_CONFIG="$HOME/.bash_profile"
    fi
  else
    SHELL_CONFIG=""
  fi
  
  # Add to PATH if we identified the shell config
  if [[ -n "$SHELL_CONFIG" ]]; then
    echo 'export PATH="$PATH:'"$INSTALL_DIR"'"' >> "$SHELL_CONFIG"
    echo "Added $INSTALL_DIR to PATH in $SHELL_CONFIG"
    echo "Please restart your terminal or run: source $SHELL_CONFIG"
  else
    echo "Could not identify shell configuration file"
    echo "Please manually add $INSTALL_DIR to your PATH"
  fi
fi

echo "✅ Collaborative Intelligence tools installed successfully"
echo ""
echo "You can now use: ci-init my-project [options]"
echo ""
echo "To verify installation:"
echo "  which ci-init  # Should show path to the installed script"
echo "  ci-init --help # Should show help text"