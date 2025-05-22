#!/bin/bash
# Implementation of the 'claude init' command for Collaborative Intelligence
# This script is called by the Claude CLI when 'claude init' is invoked with the --with-ci flag

# Default values
CI_PATH="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
INTEGRATION_TYPE="embedded"
FAST_ACTIVATION=true
AGENTS="Athena,ProjectArchitect"
WITH_CI=false

# Process standard claude CLI arguments
PROJECT_NAME=$1
shift

# Process remaining arguments
for arg in "$@"; do
  case $arg in
    --with-ci)
      WITH_CI=true
      ;;
    --integration=*)
      INTEGRATION_TYPE="${arg#*=}"
      ;;
    --agents=*)
      AGENTS="${arg#*=}"
      ;;
    --no-fast-activation)
      FAST_ACTIVATION=false
      ;;
    --ci-path=*)
      CI_PATH="${arg#*=}"
      ;;
  esac
done

# Only proceed with CI integration if --with-ci flag is present
if [ "$WITH_CI" = false ]; then
  # Just create a basic project without CI integration
  mkdir -p "$PROJECT_NAME"
  cd "$PROJECT_NAME" || exit 1
  echo "Project $PROJECT_NAME created (without CI integration)"
  exit 0
fi

echo "Initializing project: $PROJECT_NAME with Collaborative Intelligence"
echo "CollaborativeIntelligence path: $CI_PATH"
echo "Integration type: $INTEGRATION_TYPE"
echo "Fast activation: $FAST_ACTIVATION"
echo "Agents: $AGENTS"

# Create project directory
mkdir -p "$PROJECT_NAME"
cd "$PROJECT_NAME" || { echo "Failed to create project directory"; exit 1; }

# Create CLAUDE.md with CI integration
cat > CLAUDE.md << EOF
# Project: $PROJECT_NAME
# Created: $(date)

# Load CollaborativeIntelligence System
When starting, immediately:
1. Load $CI_PATH/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system

# Project-specific configuration can be added below
EOF

# Create CI configuration
cat > .collaborative-intelligence.json << EOF
{
  "integration_type": "$INTEGRATION_TYPE",
  "repository_path": "$CI_PATH",
  "active_agents": ["${AGENTS//,/\",\"}"],
  "fast_activation": $FAST_ACTIVATION,
  "preload_context": false
}
EOF

# Initialize git repository
git init
echo "CLAUDE.local.md" >> .gitignore
echo ".collaborative-intelligence.json" >> .gitignore

# Initialize with basic project structure
mkdir -p src docs tests

# Create a README.md
cat > README.md << EOF
# $PROJECT_NAME

This project was initialized with the Collaborative Intelligence system.

## Quick Start

To interact with the CI system, start Claude Code in this directory:

\`\`\`bash
claude
\`\`\`

Then use one of these commands:
- \`Athena\` - Memory & learning systems expert
- \`Recommend an agent\` - Find the right specialist
- \`Project Overview\` - See what's in this repository
- \`Detailed Analysis\` - Get comprehensive documentation
EOF

echo "✅ Project $PROJECT_NAME initialized with Collaborative Intelligence"
echo ""
echo "To get started:"
echo "  cd $PROJECT_NAME"
echo "  claude"
echo ""
echo "Then try commands like 'Athena' or 'Recommend an agent'"