#!/usr/bin/env bash
# config.sh - Configuration management for the CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Import common library if not already imported
if [[ -z "${CI_ROOT_DIR:-}" ]]; then
  source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/common.sh"
fi

# Configuration file paths
CI_CONFIG_DIR="$CI_ROOT_DIR/.ci-config"
CI_GLOBAL_CONFIG="$CI_CONFIG_DIR/config.json"
CI_USER_CONFIG="$CI_CONFIG_DIR/user.json"
CI_PROJECT_CONFIG="$CI_CONFIG_DIR/project.json"

# Ensure config directory exists
mkdir -p "$CI_CONFIG_DIR" 2>/dev/null || true

# Initialize default configuration
# This function creates default configuration files if they don't exist
# Usage: init_config
init_config() {
  # Create global config if it doesn't exist
  if [[ ! -f "$CI_GLOBAL_CONFIG" ]]; then
    cat > "$CI_GLOBAL_CONFIG" << EOF
{
  "version": "1.0.0",
  "agents": {
    "paths": {
      "base": "$CI_ROOT_DIR/AGENTS"
    }
  },
  "sessions": {
    "paths": {
      "base": "$CI_ROOT_DIR/Sessions"
    }
  }
}
EOF
  fi

  # Create user config if it doesn't exist
  if [[ ! -f "$CI_USER_CONFIG" ]]; then
    cat > "$CI_USER_CONFIG" << EOF
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

  # Create project config if it doesn't exist
  if [[ ! -f "$CI_PROJECT_CONFIG" ]]; then
    cat > "$CI_PROJECT_CONFIG" << EOF
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
}

# Load configuration from JSON file
# Usage: load_json <file_path>
load_json() {
  local file_path="$1"
  
  if [[ ! -f "$file_path" ]]; then
    echo "{}" # Return empty JSON if file doesn't exist
    return 0
  fi
  
  cat "$file_path"
}

# Get a value from JSON configuration
# Usage: get_config_value <json> <path>
# Example: get_config_value "$config" "user.name"
get_config_value() {
  local json="$1"
  local path="$2"
  
  # Split path by dots
  IFS='.' read -ra path_parts <<< "$path"
  
  # Construct jq path
  local jq_path=""
  for part in "${path_parts[@]}"; do
    jq_path+=".$part"
  done
  
  # Get value using jq
  local value=$(echo "$json" | jq -r "$jq_path" 2>/dev/null || echo "null")
  
  # Check if value is null or empty
  if [[ "$value" == "null" || -z "$value" ]]; then
    return 1
  fi
  
  echo "$value"
  return 0
}

# Set a value in JSON configuration
# Usage: set_config_value <file_path> <path> <value>
# Example: set_config_value "$CI_USER_CONFIG" "user.name" "John Doe"
set_config_value() {
  local file_path="$1"
  local path="$2"
  local value="$3"
  
  # Split path by dots
  IFS='.' read -ra path_parts <<< "$path"
  
  # Construct jq path
  local jq_path=""
  for part in "${path_parts[@]}"; do
    jq_path+=".$part"
  done
  
  # Load current config
  local config=$(load_json "$file_path")
  
  # Update value
  local updated_config=$(echo "$config" | jq "$jq_path = \"$value\"" 2>/dev/null)
  
  # Check if update was successful
  if [[ $? -ne 0 ]]; then
    echo "Error: Failed to update configuration value at $path" >&2
    return 1
  fi
  
  # Write updated config back to file
  echo "$updated_config" > "$file_path"
  return 0
}

# Load merged configuration from all sources
# Usage: load_config
load_config() {
  # Initialize config directory and files if they don't exist
  init_config
  
  # Load global config
  local global_config=$(load_json "$CI_GLOBAL_CONFIG")
  
  # Load user config
  local user_config=$(load_json "$CI_USER_CONFIG")
  
  # Load project config
  local project_config=$(load_json "$CI_PROJECT_CONFIG")
  
  # Merge configurations
  # Global config is the base, then user config overrides it, then project config
  # This is a simplistic merge - in a real implementation, you would use jq to properly merge the JSON
  echo "{ \"global\": $global_config, \"user\": $user_config, \"project\": $project_config }"
}

# Get a configuration value, trying project, user, then global configs
# Usage: get_config <path>
get_config() {
  local path="$1"
  local config=$(load_config)
  
  # Try project config first
  get_config_value "$(echo "$config" | jq -r '.project')" "$path" 2>/dev/null && return 0
  
  # Then try user config
  get_config_value "$(echo "$config" | jq -r '.user')" "$path" 2>/dev/null && return 0
  
  # Finally, try global config
  get_config_value "$(echo "$config" | jq -r '.global')" "$path" 2>/dev/null && return 0
  
  # Return empty if not found
  echo ""
  return 1
}

# Set a configuration value in the specified scope
# Usage: set_config <scope> <path> <value>
# Scope can be "global", "user", or "project"
set_config() {
  local scope="$1"
  local path="$2"
  local value="$3"
  
  case "$scope" in
    global)
      set_config_value "$CI_GLOBAL_CONFIG" "$path" "$value"
      ;;
    user)
      set_config_value "$CI_USER_CONFIG" "$path" "$value"
      ;;
    project)
      set_config_value "$CI_PROJECT_CONFIG" "$path" "$value"
      ;;
    *)
      echo "Error: Invalid configuration scope: $scope" >&2
      return 1
      ;;
  esac
}

# Validate configuration
# Usage: validate_config [--verbose]
validate_config() {
  local verbose=false
  if [[ "$1" == "--verbose" ]]; then
    verbose=true
  fi
  
  local config=$(load_config)
  local validation_errors=0
  
  # Check for required values
  local required_paths=(
    "global.version"
    "global.agents.paths.base"
    "global.sessions.paths.base"
  )
  
  # Check user preferences
  local preference_paths=(
    "user.preferences.default_agent"
    "user.preferences.color"
    "user.preferences.verbose"
  )
  
  # Check project configuration
  local project_paths=(
    "project.project.name"
    "project.project.version"
    "project.agents.enabled"
    "project.agents.disabled"
  )

  # Validate essential required values
  if [[ "$verbose" == "true" ]]; then
    echo "Validating required configuration values..."
  fi
  
  for path in "${required_paths[@]}"; do
    if ! get_config "$path" >/dev/null; then
      echo "Error: Missing required configuration: $path" >&2
      validation_errors=$((validation_errors + 1))
    elif [[ "$verbose" == "true" ]]; then
      echo "✓ Required config found: $path"
    fi
  done
  
  # Validate additional configuration values
  if [[ "$verbose" == "true" ]]; then
    echo "Validating user preferences..."
  fi
  
  for path in "${preference_paths[@]}"; do
    if ! get_config "$path" >/dev/null; then
      if [[ "$verbose" == "true" ]]; then
        echo "Warning: Missing user preference: $path (will use default)" >&2
      fi
    elif [[ "$verbose" == "true" ]]; then
      echo "✓ User preference found: $path"
    fi
  done
  
  if [[ "$verbose" == "true" ]]; then
    echo "Validating project configuration..."
  fi
  
  for path in "${project_paths[@]}"; do
    if ! get_config "$path" >/dev/null; then
      if [[ "$verbose" == "true" ]]; then
        echo "Warning: Missing project config: $path (will use default)" >&2
      fi
    elif [[ "$verbose" == "true" ]]; then
      echo "✓ Project config found: $path"
    fi
  done
  
  # Validate path existence
  if [[ "$verbose" == "true" ]]; then
    echo "Validating directory paths..."
  fi
  
  # Check if agents directory exists
  local agents_dir=$(get_config "agents.paths.base")
  if [[ -n "$agents_dir" ]]; then
    if [[ ! -d "$agents_dir" ]]; then
      echo "Error: Agents directory does not exist: $agents_dir" >&2
      validation_errors=$((validation_errors + 1))
    elif [[ "$verbose" == "true" ]]; then
      echo "✓ Agents directory exists: $agents_dir"
    fi
  fi
  
  # Check if sessions directory exists
  local sessions_dir=$(get_config "sessions.paths.base")
  if [[ -n "$sessions_dir" ]]; then
    if [[ ! -d "$sessions_dir" ]]; then
      echo "Error: Sessions directory does not exist: $sessions_dir" >&2
      validation_errors=$((validation_errors + 1))
    elif [[ "$verbose" == "true" ]]; then
      echo "✓ Sessions directory exists: $sessions_dir"
    fi
  fi
  
  # Validate version format
  local version=$(get_config "global.version")
  if [[ -n "$version" ]]; then
    if ! [[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
      echo "Warning: Version format should be x.y.z (SemVer)" >&2
      if [[ "$verbose" == "true" ]]; then
        echo "Current version: $version"
      fi
    elif [[ "$verbose" == "true" ]]; then
      echo "✓ Version format valid: $version"
    fi
  fi
  
  # Check enabled agents exist
  local enabled_agents_json=$(get_config "agents.enabled" 2>/dev/null || echo "[]")
  local enabled_agents=($(echo "$enabled_agents_json" | jq -r '.[]' 2>/dev/null || echo ""))
  
  if [[ ${#enabled_agents[@]} -gt 0 ]]; then
    for agent in "${enabled_agents[@]}"; do
      if [[ ! -d "$agents_dir/$agent" ]]; then
        echo "Warning: Enabled agent does not exist: $agent" >&2
      elif [[ "$verbose" == "true" ]]; then
        echo "✓ Enabled agent exists: $agent"
      fi
    done
  fi
  
  # Final status
  if [[ $validation_errors -gt 0 ]]; then
    if [[ "$verbose" == "true" ]]; then
      echo "Configuration validation failed with $validation_errors error(s)"
    fi
    return 1
  else
    if [[ "$verbose" == "true" ]]; then
      echo "Configuration validation successful"
    fi
    return 0
  fi
}