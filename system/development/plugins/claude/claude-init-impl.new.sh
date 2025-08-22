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

echo "Initializing project: $PROJECT_NAME with Collaborative Intelligence"
echo "CollaborativeIntelligence path: $CI_PATH"
echo "Integration type: $INTEGRATION_TYPE"
echo "Fast activation: $FAST_ACTIVATION"
echo "Agents: $AGENTS"

# Try to use external CI tool first
if check_external_ci; then
  CI_EXTERNAL=$(get_external_ci_path)
  echo "Using external CI tool: $CI_EXTERNAL"
  
  # Build arguments for external CI tool
  CI_ARGS=("init" "$PROJECT_NAME" "--integration=$INTEGRATION_TYPE" "--agents=$AGENTS")
  if [ "$FAST_ACTIVATION" = false ]; then
    CI_ARGS+=("--no-fast-activation")
  fi
  if [ -n "$CI_PATH" ]; then
    CI_ARGS+=("--ci-path=$CI_PATH")
  fi
  
  # Execute external CI tool
  "$CI_EXTERNAL" "${CI_ARGS[@]}"
  exit $?
fi

# Fall back to local implementation if external CI is not available
echo "External CI tool not found, using local implementation"

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

# Show deprecation warning
echo ""
echo "⚠️  This local implementation will be deprecated in future releases."
echo "⚠️  Please install the external CI tool for continued support:"
echo "    npm install -g @anthropics/ci-tools"
echo ""