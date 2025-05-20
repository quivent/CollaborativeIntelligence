#!/usr/bin/env bash
# fix.sh - Automated issue resolution for the CI system
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
CMD_NAME="fix"
CMD_DESCRIPTION="Fix common issues in the Collaborative Intelligence system"
CMD_USAGE="ci fix [options] [issue_type]"
CMD_OPTIONS="
  -a, --all            Fix all detected issues
  -c, --config FILE    Use custom configuration file
  --dry-run            Show what would be fixed without making changes
  --force              Force fix even if potentially destructive
"

# Show command-specific help
show_help() {
  cat << EOF
$CMD_DESCRIPTION

Usage: $CMD_USAGE

Options:$CMD_OPTIONS

Issue Types:
  config               Fix configuration issues
  structure            Fix directory structure issues
  permissions          Fix file permission issues
  dependencies         Check and install missing dependencies

Examples:
  ci fix --all         Fix all detected issues
  ci fix config        Fix only configuration issues
  ci fix --dry-run     Show what would be fixed without making changes
  ci fix --help        Show this help message
EOF
}

# Fix configuration issues
fix_configuration() {
  print_step 1 4 "Fixing configuration issues"
  
  local fixed=0
  local issues=0
  
  # Check if config directory exists
  start_progress "Checking configuration directory"
  
  if [[ ! -d "$CI_CONFIG_DIR" ]]; then
    stop_progress "failure"
    print_error "Configuration directory not found: $CI_CONFIG_DIR"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Creating configuration directory"
      mkdir -p "$CI_CONFIG_DIR"
      fixed=$((fixed + 1))
    else
      print_info "Would create configuration directory: $CI_CONFIG_DIR"
    fi
  else
    stop_progress "success"
  fi
  
  # Reinitialize configuration files
  start_progress "Checking configuration files"
  
  if [[ ! -f "$CI_GLOBAL_CONFIG" || ! -f "$CI_USER_CONFIG" || ! -f "$CI_PROJECT_CONFIG" ]]; then
    stop_progress "warning"
    print_warning "One or more configuration files missing"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Initializing configuration files"
      init_config
      fixed=$((fixed + 1))
    else
      print_info "Would initialize configuration files"
    fi
  else
    stop_progress "success"
  fi
  
  # Validate JSON syntax in config files
  local config_files=(
    "$CI_GLOBAL_CONFIG"
    "$CI_USER_CONFIG"
    "$CI_PROJECT_CONFIG"
  )
  
  for config_file in "${config_files[@]}"; do
    if [[ -f "$config_file" ]]; then
      start_progress "Validating $(basename "$config_file")"
      
      if ! jq . "$config_file" > /dev/null 2>&1; then
        stop_progress "failure"
        print_error "Invalid JSON in $(basename "$config_file")"
        issues=$((issues + 1))
        
        if [[ "${CI_FORCE:-false}" == "true" && "${CI_DRY_RUN:-false}" == "false" ]]; then
          print_warning "Backing up corrupted file to ${config_file}.bak"
          cp "$config_file" "${config_file}.bak"
          
          print_info "Reinitializing configuration file"
          # This is a simplistic approach - in a real implementation, you would handle each file separately
          init_config
          fixed=$((fixed + 1))
        elif [[ "${CI_DRY_RUN:-false}" == "true" ]]; then
          print_info "Would reinitialize configuration file (requires --force)"
        else
          print_info "Use --force to reinitialize corrupted configuration files"
        fi
      else
        stop_progress "success"
      fi
    fi
  done
  
  # Return number of fixed issues
  if [[ "$fixed" -gt 0 ]]; then
    print_success "Fixed $fixed configuration issue(s)"
  else
    print_info "No configuration issues fixed"
  fi
  
  return $fixed
}

# Fix directory structure issues
fix_structure() {
  print_step 2 4 "Fixing directory structure issues"
  
  local fixed=0
  local issues=0
  
  # Check if agents directory exists
  local agents_dir=$(get_config "agents.paths.base")
  start_progress "Checking agents directory"
  
  if [[ -z "$agents_dir" ]]; then
    stop_progress "failure"
    print_error "Agents directory not configured"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      agents_dir="$CI_ROOT_DIR/AGENTS"
      print_info "Setting default agents directory: $agents_dir"
      set_config "global" "agents.paths.base" "$agents_dir"
      fixed=$((fixed + 1))
    else
      print_info "Would set default agents directory: $CI_ROOT_DIR/AGENTS"
    fi
  fi
  
  if [[ ! -d "$agents_dir" ]]; then
    stop_progress "failure"
    print_error "Agents directory not found: $agents_dir"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Creating agents directory"
      mkdir -p "$agents_dir"
      fixed=$((fixed + 1))
    else
      print_info "Would create agents directory: $agents_dir"
    fi
  else
    stop_progress "success"
  fi
  
  # Check if Athena agent exists
  start_progress "Checking Athena agent"
  
  if [[ ! -d "$agents_dir/Athena" ]]; then
    stop_progress "failure"
    print_error "Athena agent not found"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
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
      fixed=$((fixed + 1))
    else
      print_info "Would create Athena agent structure"
    fi
  else
    stop_progress "success"
  fi
  
  # Check if sessions directory exists
  local sessions_dir=$(get_config "sessions.paths.base")
  start_progress "Checking sessions directory"
  
  if [[ -z "$sessions_dir" ]]; then
    stop_progress "failure"
    print_error "Sessions directory not configured"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      sessions_dir="$CI_ROOT_DIR/Sessions"
      print_info "Setting default sessions directory: $sessions_dir"
      set_config "global" "sessions.paths.base" "$sessions_dir"
      fixed=$((fixed + 1))
    else
      print_info "Would set default sessions directory: $CI_ROOT_DIR/Sessions"
    fi
  fi
  
  if [[ ! -d "$sessions_dir" ]]; then
    stop_progress "failure"
    print_error "Sessions directory not found: $sessions_dir"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Creating sessions directory"
      mkdir -p "$sessions_dir"
      
      # Create Sessions README
      cat > "$sessions_dir/README.md" << EOF
# Sessions

This directory contains records of interaction sessions with the Collaborative Intelligence system.

Each session is stored in its own directory with relevant metadata and conversation history.
EOF
      fixed=$((fixed + 1))
    else
      print_info "Would create sessions directory: $sessions_dir"
    fi
  else
    stop_progress "success"
  fi
  
  # Return number of fixed issues
  if [[ "$fixed" -gt 0 ]]; then
    print_success "Fixed $fixed directory structure issue(s)"
  else
    print_info "No directory structure issues fixed"
  fi
  
  return $fixed
}

# Fix file permission issues
fix_permissions() {
  print_step 3 4 "Fixing file permission issues"
  
  local fixed=0
  local issues=0
  
  # Check if main script is executable
  start_progress "Checking main script permissions"
  
  local main_script="$CI_TOOLS_DIR/CI.new"
  if [[ ! -x "$main_script" ]]; then
    stop_progress "failure"
    print_error "Main script not executable: $main_script"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Making main script executable"
      chmod +x "$main_script"
      fixed=$((fixed + 1))
    else
      print_info "Would make main script executable: $main_script"
    fi
  else
    stop_progress "success"
  fi
  
  # Check if command modules are executable
  start_progress "Checking command module permissions"
  
  local cmd_files=("$CI_CMD_DIR"/*.sh)
  local cmd_issues=0
  
  for cmd_file in "${cmd_files[@]}"; do
    if [[ ! -x "$cmd_file" ]]; then
      cmd_issues=$((cmd_issues + 1))
      issues=$((issues + 1))
    fi
  done
  
  if [[ "$cmd_issues" -gt 0 ]]; then
    stop_progress "failure"
    print_error "$cmd_issues command module(s) not executable"
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Making command modules executable"
      chmod +x "$CI_CMD_DIR"/*.sh
      fixed=$((fixed + 1))
    else
      print_info "Would make command modules executable"
    fi
  else
    stop_progress "success"
  fi
  
  # Check if wrapper script is executable
  start_progress "Checking wrapper script permissions"
  
  local wrapper_script="$CI_ROOT_DIR/CI_wrapper.sh"
  if [[ -f "$wrapper_script" && ! -x "$wrapper_script" ]]; then
    stop_progress "failure"
    print_error "Wrapper script not executable: $wrapper_script"
    issues=$((issues + 1))
    
    if [[ "${CI_DRY_RUN:-false}" == "false" ]]; then
      print_info "Making wrapper script executable"
      chmod +x "$wrapper_script"
      fixed=$((fixed + 1))
    else
      print_info "Would make wrapper script executable: $wrapper_script"
    fi
  else
    stop_progress "success"
  fi
  
  # Return number of fixed issues
  if [[ "$fixed" -gt 0 ]]; then
    print_success "Fixed $fixed permission issue(s)"
  else
    print_info "No permission issues fixed"
  fi
  
  return $fixed
}

# Check dependencies
check_dependencies() {
  print_step 4 4 "Checking dependencies"
  
  local issues=0
  local fixed=0
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
      
      print_warning "Cannot automatically install dependencies"
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
    else
      stop_progress "success"
    fi
  done
  
  # Return number of fixed issues
  if [[ "$fixed" -gt 0 ]]; then
    print_success "Fixed $fixed dependency issue(s)"
  else
    print_info "No dependency issues fixed"
  fi
  
  return $fixed
}

# Main function
main() {
  # Parse command arguments
  export CI_DRY_RUN="false"
  export CI_FORCE="false"
  export CI_FIX_ALL="false"
  local fix_config="false"
  local fix_structure="false"
  local fix_permissions="false"
  local check_deps="false"
  local custom_config=""
  
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -a|--all)
        export CI_FIX_ALL="true"
        fix_config="true"
        fix_structure="true"
        fix_permissions="true"
        check_deps="true"
        shift
        ;;
      --dry-run)
        export CI_DRY_RUN="true"
        shift
        ;;
      --force)
        export CI_FORCE="true"
        shift
        ;;
      -c|--config)
        custom_config="$2"
        shift 2
        ;;
      -h|--help)
        show_help
        exit 0
        ;;
      config)
        fix_config="true"
        shift
        ;;
      structure)
        fix_structure="true"
        shift
        ;;
      permissions)
        fix_permissions="true"
        shift
        ;;
      dependencies)
        check_deps="true"
        shift
        ;;
      -*)
        print_error "Unknown option: $1"
        show_help
        exit 1
        ;;
      *)
        print_error "Unknown issue type: $1"
        show_help
        exit 1
        ;;
    esac
  done
  
  # If no specific issue type is selected, default to all
  if [[ "$fix_config" == "false" && "$fix_structure" == "false" && "$fix_permissions" == "false" && "$check_deps" == "false" ]]; then
    export CI_FIX_ALL="true"
    fix_config="true"
    fix_structure="true"
    fix_permissions="true"
    check_deps="true"
  fi
  
  # Print banner
  if [[ "${CI_DRY_RUN:-false}" == "true" ]]; then
    print_box "Dry Run: Would Fix Collaborative Intelligence System Issues"
  else
    print_box "Fixing Collaborative Intelligence System Issues"
  fi
  
  # Check for custom config
  if [[ -n "$custom_config" ]]; then
    if [[ ! -f "$custom_config" ]]; then
      print_error "Custom configuration file not found: $custom_config"
      exit 1
    fi
    print_info "Using custom configuration: $custom_config"
    # TODO: Load custom configuration
  fi
  
  # Track total fixed issues
  local total_fixed=0
  
  # Run fix steps
  if [[ "$fix_config" == "true" ]]; then
    fix_configuration
    total_fixed=$((total_fixed + $?))
  fi
  
  if [[ "$fix_structure" == "true" ]]; then
    fix_structure
    total_fixed=$((total_fixed + $?))
  fi
  
  if [[ "$fix_permissions" == "true" ]]; then
    fix_permissions
    total_fixed=$((total_fixed + $?))
  fi
  
  if [[ "$check_deps" == "true" ]]; then
    check_dependencies
    total_fixed=$((total_fixed + $?))
  fi
  
  # Final summary
  print_divider
  
  if [[ "${CI_DRY_RUN:-false}" == "true" ]]; then
    if [[ $total_fixed -eq 0 ]]; then
      print_success "No issues found that would need fixing"
    else
      print_info "Would fix $total_fixed issue(s)"
      print_info "Run without --dry-run to apply fixes"
    fi
  else
    if [[ $total_fixed -eq 0 ]]; then
      print_success "No issues found, system is already in good state"
    else
      print_success "Fixed $total_fixed issue(s)"
    fi
  fi
  
  return 0
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi