#!/usr/bin/env bash
# install.sh - Installation script for the refactored CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Determine script and project directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"

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
chmod +x "$SCRIPT_DIR/CI.new"
chmod +x "$SCRIPT_DIR/cmd"/*.sh

# Create wrapper script
echo "Creating wrapper script..."
cat > "$PROJECT_DIR/CI" << EOF
#!/usr/bin/env bash
# This file is auto-generated. Do not edit directly.
# Use the refactored CI system in ci-tools/

exec "\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)/ci-tools/CI.new" "\$@"
EOF

chmod +x "$PROJECT_DIR/CI"

# Create compatibility links
echo "Creating compatibility links..."

# ci-init
cat > "$PROJECT_DIR/ci-init" << EOF
#!/usr/bin/env bash
# This file is auto-generated. Do not edit directly.
# Use the refactored CI system in ci-tools/

exec "\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)/ci-tools/CI.new" init "\$@"
EOF

chmod +x "$PROJECT_DIR/ci-init"

# ci-verify
cat > "$PROJECT_DIR/ci-verify" << EOF
#!/usr/bin/env bash
# This file is auto-generated. Do not edit directly.
# Use the refactored CI system in ci-tools/

exec "\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)/ci-tools/CI.new" verify "\$@"
EOF

chmod +x "$PROJECT_DIR/ci-verify"

# ci-fix
cat > "$PROJECT_DIR/ci-fix" << EOF
#!/usr/bin/env bash
# This file is auto-generated. Do not edit directly.
# Use the refactored CI system in ci-tools/

exec "\$(cd "\$(dirname "\${BASH_SOURCE[0]}")" && pwd)/ci-tools/CI.new" fix "\$@"
EOF

chmod +x "$PROJECT_DIR/ci-fix"

# Initialize configuration
echo "Initializing configuration..."
mkdir -p "$PROJECT_DIR/.ci-config"

if [[ ! -f "$PROJECT_DIR/.ci-config/config.json" ]]; then
    cat > "$PROJECT_DIR/.ci-config/config.json" << EOF
{
  "version": "1.0.0",
  "agents": {
    "paths": {
      "base": "$PROJECT_DIR/AGENTS"
    }
  },
  "sessions": {
    "paths": {
      "base": "$PROJECT_DIR/Sessions"
    }
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

if [[ ! -f "$PROJECT_DIR/.ci-config/project.json" ]]; then
    cat > "$PROJECT_DIR/.ci-config/project.json" << EOF
{
  "project": {
    "name": "CollaborativeIntelligence",
    "version": "1.0.0",
    "description": "Collaborative Intelligence System"
  },
  "agents": {
    "enabled": [],
    "disabled": []
  }
}
EOF
fi

# Verify installation
echo "Verifying installation..."
"$PROJECT_DIR/CI" verify

echo "============================================="
echo "Installation complete!"
echo "============================================="
echo "To use the CI system, run:"
echo "  ./CI <command> [options]"
echo
echo "Available commands:"
echo "  init                Initialize a new project"
echo "  verify              Verify system installation"
echo "  fix                 Fix common issues"
echo
echo "For more information, run:"
echo "  ./CI --help"
echo "============================================="