#!/usr/bin/env bash
# install.sh - Installation script for the CI system
# Part of the Collaborative Intelligence CLI Migration Plan

# Strict mode
set -euo pipefail

# Determine script and project directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/../../.." && pwd)"

# Print banner
echo "============================================="
echo "Installing Collaborative Intelligence System"
echo "============================================="

# Check prerequisites
echo "Checking prerequisites..."
if ! command -v jq &> /dev/null; then
    echo "Error: jq is required but not found"
    echo "Please install jq: https://stedolan.github.io/jq/download/"
    exit 1
fi

# Make scripts executable
echo "Making scripts executable..."
chmod +x "$SCRIPT_DIR/enforce-agent-format.sh"
chmod +x "$SCRIPT_DIR/update-terminal-title.sh"
chmod +x "$SCRIPT_DIR/validate-config.sh"
chmod +x "$SCRIPT_DIR/activate-agent.sh"

# Configure CI_PATH environment variable
configure_environment_variable() {
    local detected_shell=""
    
    # Detect user's shell
    if [[ -n "${SHELL:-}" ]]; then
        detected_shell="$SHELL"
    elif [[ -n "${BASH:-}" ]]; then
        detected_shell="$BASH"
    elif [[ -n "${ZSH_NAME:-}" ]]; then
        detected_shell="/bin/zsh"
    else
        # Default to bash if we can't detect
        detected_shell="/bin/bash"
    fi
    
    echo "Detected shell: $detected_shell"
    
    # Determine shell config file
    local shell_config=""
    if [[ "$detected_shell" == *"zsh"* ]]; then
        if [[ -f "$HOME/.zshrc" ]]; then
            shell_config="$HOME/.zshrc"
        else
            shell_config="$HOME/.zshenv"
        fi
    elif [[ "$detected_shell" == *"bash"* ]]; then
        if [[ -f "$HOME/.bashrc" ]]; then
            shell_config="$HOME/.bashrc"
        else
            shell_config="$HOME/.bash_profile"
        fi
    else
        echo "Warning: Unsupported shell detected. Please manually set CI_PATH environment variable."
        return 1
    fi
    
    echo "Using shell config file: $shell_config"
    
    # Check if CI_PATH is already configured
    if grep -q "CI_PATH=" "$shell_config"; then
        echo "CI_PATH is already configured in $shell_config"
        echo "Checking if it points to the correct location..."
        
        # Get current CI_PATH value
        local current_ci_path
        current_ci_path=$(grep "CI_PATH=" "$shell_config" | head -n 1 | cut -d= -f2- | tr -d '"' | tr -d "'")
        
        # Check if it needs to be updated
        if [[ "$current_ci_path" != "$PROJECT_DIR" ]]; then
            echo "Updating CI_PATH from '$current_ci_path' to '$PROJECT_DIR'"
            sed -i.bak "s|CI_PATH=.*$|CI_PATH=$PROJECT_DIR|" "$shell_config"
            echo "Updated CI_PATH in $shell_config"
        else
            echo "CI_PATH is already set to the correct value: $PROJECT_DIR"
        fi
    else
        # Add CI_PATH to shell config
        echo "Adding CI_PATH to $shell_config"
        echo "" >> "$shell_config"
        echo "# Collaborative Intelligence - Path configuration" >> "$shell_config"
        echo "export CI_PATH=\"$PROJECT_DIR\"" >> "$shell_config"
        echo "Added CI_PATH to $shell_config"
    fi
    
    # Remove any CI_REPO_PATH environment variables if they exist
    if grep -q "CI_REPO_PATH=" "$shell_config"; then
        echo "Removing deprecated CI_REPO_PATH variable from $shell_config"
        sed -i.bak '/CI_REPO_PATH=/d' "$shell_config"
        echo "Removed deprecated CI_REPO_PATH from $shell_config"
    fi
    
    echo "Shell configuration updated successfully!"
    echo "Please run: source $shell_config"
    echo "Or restart your terminal to apply changes."
    
    # Set the variable for the current session
    export CI_PATH="$PROJECT_DIR"
    echo "Set CI_PATH=$PROJECT_DIR for current session"
    
    return 0
}

# Initialize configuration
echo "Initializing configuration..."
mkdir -p "$PROJECT_DIR/.ci-config"

if [[ ! -f "$PROJECT_DIR/.ci-config/config.json" ]]; then
    cat > "$PROJECT_DIR/.ci-config/config.json" << EOF
{
  "version": "1.0.0",
  "ci_path": "$PROJECT_DIR",
  "agents": {
    "paths": {
      "base": "$PROJECT_DIR/AGENTS"
    }
  },
  "sessions": {
    "paths": {
      "base": "$PROJECT_DIR/Sessions"
    }
  },
  "external_ci": {
    "auto_detect": true,
    "show_deprecation_notice": true
  }
}
EOF
fi

if [[ ! -f "$PROJECT_DIR/.ci-config/user.json" ]]; then
    cat > "$PROJECT_DIR/.ci-config/user.json" << EOF
{
  "user": {
    "name": "",
    "email": ""
  },
  "preferences": {
    "default_agent": "Athena",
    "verbose": false,
    "color": true
  }
}
EOF
fi

# Create wrapper script to check for external CI
create_wrapper_scripts() {
    echo "Creating CI wrapper scripts..."
    
    # Main CI wrapper
    cat > "$PROJECT_DIR/ci" << EOF
#!/usr/bin/env bash
# CI CLI wrapper that checks for external implementation
# Part of the Collaborative Intelligence CLI Migration Plan

# Check for external CI implementation first
if command -v ci-external &> /dev/null; then
    # External CI implementation found, delegate to it
    ci-external "\$@"
elif [[ -n "\${CI_EXTERNAL_PATH:-}" ]] && [[ -x "\$CI_EXTERNAL_PATH/bin/ci" ]]; then
    # External CI implementation found via CI_EXTERNAL_PATH, delegate to it
    "\$CI_EXTERNAL_PATH/bin/ci" "\$@"
elif [[ -x "\$HOME/.ci/bin/ci" ]]; then
    # External CI implementation found in home directory, delegate to it
    "\$HOME/.ci/bin/ci" "\$@"
else
    # Fall back to local implementation
    CI_DIR="\$(cd "\$(dirname "\${BASH_SOURCE[0]}")/CLIs/ci/bash" && pwd)"
    
    if [[ "\$#" -eq 1 ]] && [[ "\$1" == "--version" ]]; then
        echo "CI version 0.1.0 (local implementation)"
        echo "This local implementation will be deprecated in future releases."
        echo "Please install the external CI tools for continued support."
        exit 0
    fi
    
    # Execute the appropriate command
    if [[ "\$#" -ge 1 ]]; then
        cmd="\$1"
        shift
        case "\$cmd" in
            init)
                "\$CI_DIR/ci-init" "\$@"
                ;;
            verify)
                "\$CI_DIR/ci-verify" "\$@"
                ;;
            fix)
                "\$CI_DIR/ci-fix" "\$@"
                ;;
            *)
                echo "Unknown command: \$cmd"
                echo "Available commands: init, verify, fix"
                exit 1
                ;;
        esac
    else
        echo "Usage: ci <command> [options]"
        echo "Available commands: init, verify, fix"
        exit 1
    fi
fi
EOF
    chmod +x "$PROJECT_DIR/ci"
    
    # Create command-specific wrappers for backward compatibility
    for cmd in init verify fix; do
        cat > "$PROJECT_DIR/ci-$cmd" << EOF
#!/usr/bin/env bash
# ci-$cmd wrapper that checks for external implementation
# Part of the Collaborative Intelligence CLI Migration Plan

# Simply delegate to the main CI wrapper
"\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)/ci" $cmd "\$@"
EOF
        chmod +x "$PROJECT_DIR/ci-$cmd"
    done
    
    echo "Wrapper scripts created successfully!"
}

# Prompt for CI_PATH configuration
echo "============================================="
echo "Environment Variable Configuration"
echo "============================================="
echo "For optimal CI functionality, we need to configure the CI_PATH environment variable."
echo "This variable helps the CI tools locate the CollaborativeIntelligence repository."
echo
echo "Current location: $PROJECT_DIR"
echo
echo "Do you want to configure the CI_PATH environment variable? [Y/n]"
read -r configure_env

# Default to yes if enter is pressed
configure_env=${configure_env:-y}

if [[ $configure_env =~ ^[Yy]$ ]]; then
    configure_environment_variable
else
    echo "Skipping environment variable configuration."
    echo "You may need to manually set CI_PATH to: $PROJECT_DIR"
fi

# Create wrapper scripts
create_wrapper_scripts

# Test external CI detection
echo "Testing external CI detection..."
if "$PROJECT_DIR/ci" --version | grep -q "external"; then
    echo "External CI implementation detected and working correctly!"
else
    echo "Using local CI implementation (external CI not detected)."
fi

# Verify installation
echo "Verifying installation..."
"$PROJECT_DIR/ci" verify || true

echo "============================================="
echo "Installation complete!"
echo "============================================="
echo "To use the CI system, run:"
echo "  ./ci <command> [options]"
echo
echo "Available commands:"
echo "  init                Initialize a new project"
echo "  verify              Verify system installation"
echo "  fix                 Fix common issues"
echo
echo "For more information, run:"
echo "  ./ci --help"
echo "============================================="