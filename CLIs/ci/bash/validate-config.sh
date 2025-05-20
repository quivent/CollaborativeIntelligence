#!/usr/bin/env bash
# validate-config.sh - Configuration validation tool for the CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Import libraries
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LIB_DIR="$SCRIPT_DIR/lib"

source "$LIB_DIR/common.sh"
source "$LIB_DIR/output.sh"
source "$LIB_DIR/config.sh"

# Help text
show_help() {
  echo "Usage: $(basename "$0") [OPTIONS]"
  echo ""
  echo "Configuration validation tool for Collaborative Intelligence system"
  echo ""
  echo "Options:"
  echo "  -v, --verbose       Show detailed validation information"
  echo "  -f, --fix           Fix configuration issues when detected"
  echo "  -c, --config FILE   Use custom configuration file"
  echo "  -h, --help          Show this help message"
  echo ""
  echo "Examples:"
  echo "  $(basename "$0") --verbose"
  echo "  $(basename "$0") --fix"
}

# Main function
main() {
  # Parse command line arguments
  local verbose=false
  local fix=false
  local custom_config=""
  
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -v|--verbose)
        verbose=true
        shift
        ;;
      -f|--fix)
        fix=true
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
      *)
        echo "Unknown option: $1" >&2
        show_help
        exit 1
        ;;
    esac
  done

  # Check for custom config
  if [[ -n "$custom_config" ]]; then
    if [[ ! -f "$custom_config" ]]; then
      print_error "Custom configuration file not found: $custom_config"
      exit 1
    fi
    print_info "Using custom configuration: $custom_config"
    # TODO: Load custom configuration
  fi
  
  # Print banner
  print_box "Validating Collaborative Intelligence Configuration"
  
  # Ensure config directory exists
  if [[ ! -d "$CI_CONFIG_DIR" ]]; then
    print_error "Configuration directory not found: $CI_CONFIG_DIR"
    
    if [[ "$fix" == "true" ]]; then
      print_info "Creating configuration directory"
      mkdir -p "$CI_CONFIG_DIR"
    else
      print_info "Use --fix to create configuration directory"
      exit 1
    fi
  fi
  
  # Check if configuration files exist
  local config_files=(
    "$CI_GLOBAL_CONFIG"
    "$CI_USER_CONFIG"
    "$CI_PROJECT_CONFIG"
  )
  
  local missing_configs=0
  for config_file in "${config_files[@]}"; do
    if [[ ! -f "$config_file" ]]; then
      print_error "Configuration file not found: $config_file"
      missing_configs=$((missing_configs + 1))
    fi
  done
  
  if [[ $missing_configs -gt 0 ]]; then
    if [[ "$fix" == "true" ]]; then
      print_info "Initializing missing configuration files"
      init_config
    else
      print_info "Use --fix to initialize missing configuration files"
      exit 1
    fi
  fi
  
  # Run validation
  if [[ "$verbose" == "true" ]]; then
    validate_config --verbose
    validation_result=$?
  else
    validate_config
    validation_result=$?
  fi
  
  # Print validation result
  if [[ $validation_result -eq 0 ]]; then
    print_success "Configuration validation successful"
  else
    print_error "Configuration validation failed"
    
    if [[ "$fix" == "true" ]]; then
      print_info "Attempting to fix configuration issues"
      
      # Check paths and create if missing
      local agents_dir=$(get_config "agents.paths.base")
      if [[ -n "$agents_dir" && ! -d "$agents_dir" ]]; then
        print_info "Creating agents directory: $agents_dir"
        mkdir -p "$agents_dir"
      fi
      
      local sessions_dir=$(get_config "sessions.paths.base")
      if [[ -n "$sessions_dir" && ! -d "$sessions_dir" ]]; then
        print_info "Creating sessions directory: $sessions_dir"
        mkdir -p "$sessions_dir"
      fi
      
      # Reinitialize configuration in case of corruption
      init_config
      
      # Run validation again
      if [[ "$verbose" == "true" ]]; then
        validate_config --verbose
        validation_result=$?
      else
        validate_config
        validation_result=$?
      fi
      
      if [[ $validation_result -eq 0 ]]; then
        print_success "Configuration issues fixed successfully"
      else
        print_error "Configuration issues could not be fixed automatically"
        print_info "Please check your configuration files manually"
        exit 1
      fi
    else
      print_info "Use --fix to attempt automatic repair of configuration issues"
      exit 1
    fi
  fi
  
  # List configuration paths
  if [[ "$verbose" == "true" ]]; then
    print_section "Configuration Paths"
    echo "Global Config: $CI_GLOBAL_CONFIG"
    echo "User Config: $CI_USER_CONFIG"
    echo "Project Config: $CI_PROJECT_CONFIG"
    
    local agents_dir=$(get_config "agents.paths.base")
    local sessions_dir=$(get_config "sessions.paths.base")
    
    print_section "Directory Paths"
    echo "Agents Directory: $agents_dir"
    echo "Sessions Directory: $sessions_dir"
    
    print_section "Configuration Summary"
    local default_agent=$(get_config "user.preferences.default_agent")
    echo "Default Agent: ${default_agent:-"Athena (default)"}"
    
    local enabled_agents_json=$(get_config "agents.enabled" 2>/dev/null || echo "[]")
    local enabled_agents=($(echo "$enabled_agents_json" | jq -r '.[]' 2>/dev/null || echo ""))
    
    if [[ ${#enabled_agents[@]} -gt 0 ]]; then
      echo "Enabled Agents: $(IFS=,; echo "${enabled_agents[*]}")"
    else
      echo "Enabled Agents: None"
    fi
  fi
  
  return 0
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi