#!/usr/bin/env bash
# init.sh - Project initialization command for the CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Import libraries
CI_CMD_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$CI_CMD_DIR/.." && pwd)"
CI_LIB_DIR="$CI_TOOLS_DIR/lib"

source "$CI_LIB_DIR/common.sh"
source "$CI_LIB_DIR/output.sh"
source "$CI_LIB_DIR/config.sh"

# Command metadata
CMD_NAME="init"
CMD_DESCRIPTION="Initialize a new project with Collaborative Intelligence"
CMD_USAGE="ci init [options] <project_name>"
CMD_OPTIONS="
  -t, --template NAME   Use specific template (default: standard)
  -f, --force           Force overwrite existing files
  -c, --config FILE     Use custom configuration file
  --no-agents           Initialize without agent structure
  --no-sessions         Initialize without session structure
"

# Show command-specific help
show_help() {
  cat << EOF
$CMD_DESCRIPTION

Usage: $CMD_USAGE

Options:$CMD_OPTIONS

Examples:
  ci init myproject                Initialize a new project
  ci init myproject --template ai  Initialize with AI template
  ci init --help                   Show this help message
EOF
}

# Initialize project directory
init_project_directory() {
  local project_name="$1"
  local template="${2:-standard}"
  local force="${3:-false}"
  
  # Create project directory
  print_step 1 4 "Creating project directory: $project_name"
  
  if [[ -d "$project_name" && "$force" != "true" ]]; then
    print_error "Directory already exists: $project_name"
    print_info "Use --force to overwrite existing files"
    return 1
  fi
  
  mkdir -p "$project_name"
  cd "$project_name"
  
  print_success "Created project directory: $project_name"
  return 0
}

# Initialize CLAUDE.md configuration
init_claude_config() {
  print_step 2 4 "Creating CLAUDE.md configuration"
  
  local claude_md="CLAUDE.md"
  local claude_template="$CI_TOOLS_DIR/templates/CLAUDE.md.template"
  
  if [[ -f "$claude_template" ]]; then
    cp "$claude_template" "$claude_md"
  else
    # Fallback if template doesn't exist
    cat > "$claude_md" << EOF
# Ethical Collaborative Intelligence System - Configuration

This file serves as the primary configuration and initialization point for the Ethical Collaborative Intelligence System. When Claude (or any compatible AI system) loads this file, it establishes the foundational framework for collaborative interaction.

## Quick Start

| Type this in terminal | To get this |
|----------------------|-------------|
| \`athena\` | Your memory & learning systems expert (✦ PRIMARY AGENT) |
| \`Recommend an agent\` | Find the perfect agent for your specific task |
| \`Project Overview\` | See what's in this repository right now |
| \`Detailed Analysis\` | Get comprehensive codebase documentation |

For complete details, see \`CollaborativeIntelligence/docs/guides/QuickStart.md\`
EOF
  fi
  
  print_success "Created CLAUDE.md configuration"
  return 0
}

# Initialize project structure
init_project_structure() {
  local with_agents="${1:-true}"
  local with_sessions="${2:-true}"
  
  print_step 3 4 "Creating project structure"
  
  # Create README.md
  cat > "README.md" << EOF
# $(basename "$(pwd)") Project

This project uses the Collaborative Intelligence system for AI-assisted development.

## Getting Started

1. Open this project with Claude Code
2. Type \`athena\` to start a conversation with the primary agent
3. Type \`Recommend an agent\` to find the right specialist for your task

## Project Structure

- \`CLAUDE.md\`: Main configuration for the Collaborative Intelligence system
- \`AGENTS/\`: Specialized collaborative intelligence agents
- \`Sessions/\`: Records of interaction sessions
EOF
  
  # Create directory structure
  mkdir -p Documentation
  
  if [[ "$with_agents" == "true" ]]; then
    mkdir -p AGENTS/Athena/Sessions
    
    # Create Athena README
    cat > "AGENTS/Athena/README.md" << EOF
# Athena

Athena is the primary agent in the Collaborative Intelligence system, specializing in memory architecture and learning systems.

## Capabilities

- Memory management and optimization
- Knowledge integration across sessions
- Learning system coordination
- Project-wide understanding
EOF
  fi
  
  if [[ "$with_sessions" == "true" ]]; then
    mkdir -p Sessions
    
    # Create Sessions README
    cat > "Sessions/README.md" << EOF
# Sessions

This directory contains records of interaction sessions with the Collaborative Intelligence system.

Each session is stored in its own directory with relevant metadata and conversation history.
EOF
  fi
  
  print_success "Created project structure"
  return 0
}

# Initialize git repository
init_git_repository() {
  print_step 4 4 "Initializing git repository"
  
  # Check if git is already initialized
  if [[ -d ".git" ]]; then
    print_info "Git repository already initialized"
    return 0
  fi
  
  # Initialize git
  if ! command -v git >/dev/null 2>&1; then
    print_warning "Git command not found, skipping repository initialization"
    return 1
  fi
  
  git init > /dev/null
  
  # Create .gitignore
  cat > ".gitignore" << EOF
# Collaborative Intelligence system
.ci-config/
CLAUDE.local.md

# OS files
.DS_Store
Thumbs.db

# Editor files
.vscode/
.idea/
*.swp
*~
EOF
  
  # Initial commit
  git add .
  git commit -m "Initial commit: Project initialized with Collaborative Intelligence" > /dev/null
  
  print_success "Initialized git repository"
  return 0
}

# Main function
main() {
  # Parse command arguments
  local project_name=""
  local template="standard"
  local force="false"
  local with_agents="true"
  local with_sessions="true"
  local custom_config=""
  
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -t|--template)
        template="$2"
        shift 2
        ;;
      -f|--force)
        force="true"
        shift
        ;;
      -c|--config)
        custom_config="$2"
        shift 2
        ;;
      --no-agents)
        with_agents="false"
        shift
        ;;
      --no-sessions)
        with_sessions="false"
        shift
        ;;
      -h|--help)
        show_help
        exit 0
        ;;
      -*)
        print_error "Unknown option: $1"
        show_help
        exit 1
        ;;
      *)
        if [[ -z "$project_name" ]]; then
          project_name="$1"
        else
          print_error "Too many arguments"
          show_help
          exit 1
        fi
        shift
        ;;
    esac
  done
  
  # Validate required arguments
  if [[ -z "$project_name" ]]; then
    print_error "Project name is required"
    show_help
    exit 1
  fi
  
  # Print banner
  print_box "Initializing Collaborative Intelligence Project: $project_name"
  
  # Check for custom config
  if [[ -n "$custom_config" ]]; then
    if [[ ! -f "$custom_config" ]]; then
      print_error "Custom configuration file not found: $custom_config"
      exit 1
    fi
    print_info "Using custom configuration: $custom_config"
    # TODO: Load custom configuration
  fi
  
  # Initialize project
  if ! init_project_directory "$project_name" "$template" "$force"; then
    exit 1
  fi
  
  init_claude_config
  init_project_structure "$with_agents" "$with_sessions"
  init_git_repository
  
  # Final message
  print_divider
  print_success "Project initialization complete: $project_name"
  print_info "To get started, run:"
  echo -e "${BOLD}cd $project_name${RESET}"
  echo -e "${BOLD}claude code${RESET}"
  
  return 0
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi