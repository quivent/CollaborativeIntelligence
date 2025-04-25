#!/bin/bash
# integrate-ci.sh - Add CollaborativeIntelligence to a project

TARGET_DIR="${1:-.}"  # Default to current directory if not specified
CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"

cat > "$TARGET_DIR/CLAUDE.local.md" << EOF
# Load CollaborativeIntelligence System

When starting, immediately:
1. Load $CI_PATH/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
EOF

echo "Integration complete. CollaborativeIntelligence linked to $TARGET_DIR"