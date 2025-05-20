#!/usr/bin/env bash
# verify.sh - System verification command for the CI system
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
CMD_NAME="verify"
CMD_DESCRIPTION="Verify the Collaborative Intelligence system installation"
CMD_USAGE="ci verify [options]"
CMD_OPTIONS="
  -f, --fix            Attempt to fix issues automatically
  -c, --config FILE    Use custom configuration file
  --skip-agents        Skip agent verification
  --skip-sessions      Skip session verification
"

# Show command-specific help
show_help() {
  cat << EOF
$CMD_DESCRIPTION

Usage: $CMD_USAGE

Options:$CMD_OPTIONS

Examples:
  ci verify             Verify system installation
  ci verify --fix       Verify and fix issues
  ci verify --help      Show this help message
EOF
}

# Verify configuration
verify_configuration() {
  print_step 1 4 "Verifying configuration"
  start_progress "Checking configuration files"
  
  local issues=0
  
  # Check if config directory exists
  if [[ ! -d "$CI_CONFIG_DIR" ]]; then
    stop_progress "failure"
    print_error "Configuration directory not found: $CI_CONFIG_DIR"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      print_info "Creating configuration directory"
      mkdir -p "$CI_CONFIG_DIR"
    fi
  else
    stop_progress "success"
    print_success "Configuration directory found: $CI_CONFIG_DIR"
  fi
  
  # Check configuration files
  local config_files=(
    "$CI_GLOBAL_CONFIG"
    "$CI_USER_CONFIG"
    "$CI_PROJECT_CONFIG"
  )
  
  for config_file in "${config_files[@]}"; do
    start_progress "Checking $(basename "$config_file")"
    
    if [[ ! -f "$config_file" ]]; then
      stop_progress "failure"
      print_error "Configuration file not found: $config_file"
      issues=$((issues + 1))
      
      if [[ "${CI_FIX:-false}" == "true" ]]; then
        print_info "Initializing configuration"
        init_config
      fi
    else
      # Validate JSON syntax
      if ! jq . "$config_file" > /dev/null 2>&1; then
        stop_progress "failure"
        print_error "Invalid JSON in configuration file: $config_file"
        issues=$((issues + 1))
        
        if [[ "${CI_FIX:-false}" == "true" ]]; then
          print_warning "Cannot automatically fix invalid JSON, manual intervention required"
        fi
      else
        stop_progress "success"
        print_success "Configuration file valid: $(basename "$config_file")"
      fi
    fi
  done
  
  # Validate configuration values
  start_progress "Validating configuration values"
  
  if ! validate_config > /dev/null 2>&1; then
    stop_progress "failure"
    print_error "Configuration validation failed"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      print_info "Reinitializing configuration with default values"
      init_config
    fi
  else
    stop_progress "success"
    print_success "Configuration values validated"
  fi
  
  # Return the number of issues found
  return $issues
}

# Verify agent structure
verify_agents() {
  if [[ "${CI_SKIP_AGENTS:-false}" == "true" ]]; then
    print_info "Skipping agent verification"
    return 0
  fi
  
  print_step 2 4 "Verifying agent structure"
  
  local issues=0
  local agents_dir=$(get_config "agents.paths.base")
  
  # Check if agents directory exists
  start_progress "Checking agents directory"
  
  if [[ -z "$agents_dir" ]]; then
    stop_progress "failure"
    print_error "Agents directory not configured"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      agents_dir="$CI_ROOT_DIR/AGENTS"
      print_info "Setting default agents directory: $agents_dir"
      set_config "global" "agents.paths.base" "$agents_dir"
    fi
  fi
  
  if [[ ! -d "$agents_dir" ]]; then
    stop_progress "failure"
    print_error "Agents directory not found: $agents_dir"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      print_info "Creating agents directory"
      mkdir -p "$agents_dir"
    fi
  else
    stop_progress "success"
    print_success "Agents directory found: $agents_dir"
  fi
  
  # Check for Athena agent (primary agent)
  start_progress "Checking for Athena agent"
  
  if [[ ! -d "$agents_dir/Athena" ]]; then
    stop_progress "failure"
    print_error "Athena agent not found"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      print_info "Creating Athena agent structure"
      mkdir -p "$agents_dir/Athena/Sessions"
      
      # Create Athena README
      cat > "$agents_dir/Athena/README.md" << EOF
# Athena

Athena is the primary agent in the Collaborative Intelligence system, specializing in memory architecture and learning systems.

## Capabilities

- Memory management and optimization
- Knowledge integration across sessions
- Learning system coordination
- Project-wide understanding
EOF
    fi
  else
    stop_progress "success"
    print_success "Athena agent found"
  fi
  
  # Return the number of issues found
  return $issues
}

# Verify session structure
verify_sessions() {
  if [[ "${CI_SKIP_SESSIONS:-false}" == "true" ]]; then
    print_info "Skipping session verification"
    return 0
  fi
  
  print_step 3 4 "Verifying session structure"
  
  local issues=0
  local sessions_dir=$(get_config "sessions.paths.base")
  
  # Check if sessions directory exists
  start_progress "Checking sessions directory"
  
  if [[ -z "$sessions_dir" ]]; then
    stop_progress "failure"
    print_error "Sessions directory not configured"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      sessions_dir="$CI_ROOT_DIR/Sessions"
      print_info "Setting default sessions directory: $sessions_dir"
      set_config "global" "sessions.paths.base" "$sessions_dir"
    fi
  fi
  
  if [[ ! -d "$sessions_dir" ]]; then
    stop_progress "failure"
    print_error "Sessions directory not found: $sessions_dir"
    issues=$((issues + 1))
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      print_info "Creating sessions directory"
      mkdir -p "$sessions_dir"
      
      # Create Sessions README
      cat > "$sessions_dir/README.md" << EOF
# Sessions

This directory contains records of interaction sessions with the Collaborative Intelligence system.

Each session is stored in its own directory with relevant metadata and conversation history.
EOF
    fi
  else
    stop_progress "success"
    print_success "Sessions directory found: $sessions_dir"
  fi
  
  # Return the number of issues found
  return $issues
}

# Verify system dependencies
verify_dependencies() {
  print_step 4 4 "Verifying system dependencies"
  
  local issues=0
  local dependencies=(
    "jq"
    "git"
  )
  
  for dep in "${dependencies[@]}"; do
    start_progress "Checking for $dep"
    
    if ! command -v "$dep" > /dev/null 2>&1; then
      stop_progress "failure"
      print_error "Dependency not found: $dep"
      issues=$((issues + 1))
      
      if [[ "${CI_FIX:-false}" == "true" ]]; then
        print_warning "Cannot automatically install dependencies, please install manually"
        case "$dep" in
          jq)
            print_info "Install jq: https://stedolan.github.io/jq/download/"
            ;;
          git)
            print_info "Install git: https://git-scm.com/downloads"
            ;;
          *)
            print_info "Install $dep using your package manager"
            ;;
        esac
      fi
    else
      stop_progress "success"
      print_success "Dependency found: $dep"
    fi
  done
  
  # Return the number of issues found
  return $issues
}

# Main function
main() {
  # Parse command arguments
  export CI_FIX="false"
  export CI_SKIP_AGENTS="false"
  export CI_SKIP_SESSIONS="false"
  local custom_config=""
  
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -f|--fix)
        export CI_FIX="true"
        shift
        ;;
      -c|--config)
        custom_config="$2"
        shift 2
        ;;
      --skip-agents)
        export CI_SKIP_AGENTS="true"
        shift
        ;;
      --skip-sessions)
        export CI_SKIP_SESSIONS="true"
        shift
        ;;
      -h|--help)
        show_help
        exit 0
        ;;
      *)
        print_error "Unknown option: $1"
        show_help
        exit 1
        ;;
    esac
  done
  
  # Print banner
  print_box "Verifying Collaborative Intelligence System"
  
  # Check for custom config
  if [[ -n "$custom_config" ]]; then
    if [[ ! -f "$custom_config" ]]; then
      print_error "Custom configuration file not found: $custom_config"
      exit 1
    fi
    print_info "Using custom configuration: $custom_config"
    # TODO: Load custom configuration
  fi
  
  # Track total issues
  local total_issues=0
  
  # Run verification steps
  verify_configuration
  total_issues=$((total_issues + $?))
  
  verify_agents
  total_issues=$((total_issues + $?))
  
  verify_sessions
  total_issues=$((total_issues + $?))
  
  verify_dependencies
  total_issues=$((total_issues + $?))
  
  # Final summary
  print_divider
  
  if [[ $total_issues -eq 0 ]]; then
    print_success "Verification complete: No issues found"
    exit 0
  else
    print_error "Verification complete: Found $total_issues issue(s)"
    
    if [[ "${CI_FIX:-false}" == "true" ]]; then
      print_info "Attempted to fix issues automatically"
      print_info "Run verification again to check if issues were resolved"
    else
      print_info "Use --fix option to attempt automatic issue resolution"
    fi
    
    exit 1
  fi
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi