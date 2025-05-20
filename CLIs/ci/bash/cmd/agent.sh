#!/usr/bin/env bash
# agent.sh - Agent management command for the CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Import libraries
CI_CMD_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$CI_CMD_DIR/.." && pwd)"
CI_LIB_DIR="$CI_TOOLS_DIR/lib"
CI_ROOT_DIR="$(cd "$CI_TOOLS_DIR/.." && pwd)"

source "$CI_LIB_DIR/common.sh"
source "$CI_LIB_DIR/output.sh"
source "$CI_LIB_DIR/config.sh"

# Check for memory optimization system
if [[ -f "$CI_LIB_DIR/rust-memory-bridge.sh" ]]; then
  source "$CI_LIB_DIR/rust-memory-bridge.sh"
  MEMORY_OPTIMIZATION_AVAILABLE=true
else
  MEMORY_OPTIMIZATION_AVAILABLE=false
fi

# Command metadata
CMD_NAME="agent"
CMD_DESCRIPTION="Manage agents in the Collaborative Intelligence system"
CMD_USAGE="ci agent <subcommand> [options]"
CMD_OPTIONS="
  -c, --config FILE    Use custom configuration file
"

# Subcommands and their descriptions
SUBCOMMANDS=(
  "list:List available agents"
  "info:Show detailed information about an agent"
  "create:Create a new agent"
  "enable:Enable an agent in the current project"
  "disable:Disable an agent in the current project"
  "template:Create an agent from a template"
  "activate:Activate an agent for the current session"
)

# Show command-specific help
show_help() {
  cat << EOF
$CMD_DESCRIPTION

Usage: $CMD_USAGE

Subcommands:
  list                 List all available agents
  info <agent_name>    Show detailed information about an agent
  create <agent_name>  Create a new agent
  enable <agent_name>  Enable an agent in the current project
  disable <agent_name> Disable an agent in the current project
  template <template>  Create an agent from a template
  activate <agent_name> [context] Activate an agent for the current session

Options:$CMD_OPTIONS

Examples:
  ci agent list                    List all available agents
  ci agent info Athena             Show details about Athena agent
  ci agent create MyAgent          Create a new agent named MyAgent
  ci agent enable Optimizer        Enable Optimizer agent in this project
  ci agent disable Debugger        Disable Debugger agent in this project
  ci agent template DataScientist  Create a new agent from DataScientist template
  ci agent activate Athena         Activate Athena agent for the current session
  ci agent activate Debugger code  Activate Debugger with 'code' context
  ci agent --help                  Show this help message
EOF
}

# Get the agents directory
get_agents_dir() {
  local agents_dir=$(get_config "agents.paths.base")
  
  if [[ -z "$agents_dir" ]]; then
    print_error "Agents directory not configured"
    print_info "Run 'ci fix' to repair configuration"
    exit 1
  fi
  
  if [[ ! -d "$agents_dir" ]]; then
    print_error "Agents directory not found: $agents_dir"
    print_info "Run 'ci fix' to create the directory"
    exit 1
  fi
  
  echo "$agents_dir"
}

# List available agents
list_agents() {
  local agents_dir=$(get_agents_dir)
  
  print_box "Available Agents"
  
  # Count enabled and disabled agents
  local enabled_agents=($(get_config "agents.enabled" 2>/dev/null || echo "[]" | jq -r '.[]' 2>/dev/null || echo ""))
  local disabled_agents=($(get_config "agents.disabled" 2>/dev/null || echo "[]" | jq -r '.[]' 2>/dev/null || echo ""))
  
  # Get all agent directories
  local agent_dirs=()
  while IFS= read -r -d '' dir; do
    if [[ -d "$dir" && -f "$dir/README.md" ]]; then
      agent_dirs+=("$dir")
    fi
  done < <(find "$agents_dir" -mindepth 1 -maxdepth 1 -type d -print0)
  
  if [[ ${#agent_dirs[@]} -eq 0 ]]; then
    print_info "No agents found in $agents_dir"
    print_info "Run 'ci agent create <n>' to create a new agent"
    return 0
  fi
  
  print_section "Agent Status"
  for dir in "${agent_dirs[@]}"; do
    local agent_name=$(basename "$dir")
    local readme="$dir/README.md"
    local memory="$dir/MEMORY.md"
    local sessions_dir="$dir/Sessions"
    
    # Determine agent status
    local status="available"
    if [[ " ${enabled_agents[*]} " == *" $agent_name "* ]]; then
      status="enabled"
    elif [[ " ${disabled_agents[*]} " == *" $agent_name "* ]]; then
      status="disabled"
    fi
    
    # Get agent description from README.md (first line after the title)
    local description=""
    if [[ -f "$readme" ]]; then
      description=$(grep -v "^#" "$readme" | grep -v "^$" | head -n 1 | sed 's/^[[:space:]]*//')
    fi
    
    # Truncate description if too long
    if [[ ${#description} -gt 60 ]]; then
      description="${description:0:57}..."
    fi
    
    # Print agent information
    case "$status" in
      enabled)
        print_list_item "$agent_name" "success"
        ;;
      disabled)
        print_list_item "$agent_name" "skipped"
        ;;
      *)
        print_list_item "$agent_name"
        ;;
    esac
    
    if [[ -n "$description" ]]; then
      echo "  └── $description"
    fi
  done
  
  echo ""
  print_info "Total agents: ${#agent_dirs[@]}"
  print_info "Use 'ci agent info <n>' for detailed information"
  
  return 0
}

# Show detailed information about an agent
show_agent_info() {
  local agent_name="$1"
  local agents_dir=$(get_agents_dir)
  local agent_dir="$agents_dir/$agent_name"
  
  # Check if agent exists
  if [[ ! -d "$agent_dir" ]]; then
    print_error "Agent not found: $agent_name"
    print_info "Use 'ci agent list' to see available agents"
    exit 1
  fi
  
  print_box "Agent Information: $agent_name"
  
  # Agent files
  local readme="$agent_dir/README.md"
  local memory="$agent_dir/MEMORY.md"
  local learning="$agent_dir/ContinuousLearning.md"
  local sessions_dir="$agent_dir/Sessions"
  
  # Agent status
  local enabled_agents=($(get_config "agents.enabled" 2>/dev/null || echo "[]" | jq -r '.[]' 2>/dev/null || echo ""))
  local disabled_agents=($(get_config "agents.disabled" 2>/dev/null || echo "[]" | jq -r '.[]' 2>/dev/null || echo ""))
  
  local status="available"
  if [[ " ${enabled_agents[*]} " == *" $agent_name "* ]]; then
    status="enabled"
  elif [[ " ${disabled_agents[*]} " == *" $agent_name "* ]]; then
    status="disabled"
  fi
  
  # Print agent status
  print_section "Status"
  case "$status" in
    enabled)
      print_success "Agent is enabled in this project"
      ;;
    disabled)
      print_warning "Agent is disabled in this project"
      ;;
    *)
      print_info "Agent is available but not configured for this project"
      ;;
  esac
  
  # Print agent path
  echo "Path: $agent_dir"
  
  # Check README.md
  if [[ -f "$readme" ]]; then
    # Extract agent description (all content after the title and before the first section)
    print_section "Description"
    local description=$(awk '/^# / {p=1; next} /^##/ {p=0} p {print}' "$readme" | sed '/^$/d' | head -n 5)
    echo "$description"
    if [[ $(wc -l <<< "$description") -ge 5 ]]; then
      echo "... (see $readme for full description)"
    fi
  else
    print_warning "No README.md found for this agent"
  fi
  
  # Check memory file
  print_section "Memory"
  if [[ -f "$memory" ]]; then
    echo "Memory file: $memory"
    local memory_size=$(wc -l < "$memory")
    echo "Memory size: $memory_size lines"
  else
    print_warning "No MEMORY.md file found"
  fi
  
  # Check continuous learning file
  print_section "Learning"
  if [[ -f "$learning" ]]; then
    echo "Learning file: $learning"
    local learning_size=$(wc -l < "$learning")
    echo "Learning entries: $learning_size lines"
  else
    print_warning "No ContinuousLearning.md file found"
  fi
  
  # Check sessions
  print_section "Sessions"
  if [[ -d "$sessions_dir" ]]; then
    local session_count=$(find "$sessions_dir" -mindepth 1 -maxdepth 1 -type d | wc -l)
    echo "Session count: $session_count"
    
    # List recent sessions
    if [[ $session_count -gt 0 ]]; then
      echo "Recent sessions:"
      find "$sessions_dir" -mindepth 1 -maxdepth 1 -type d -exec basename {} \; | sort | tail -n 5 | while read -r session; do
        echo "  - $session"
      done
      
      if [[ $session_count -gt 5 ]]; then
        echo "  ... and $(($session_count - 5)) more"
      fi
    else
      echo "No sessions found"
    fi
  else
    print_warning "No Sessions directory found"
  fi
  
  return 0
}

# Create a new agent
create_agent() {
  local agent_name="$1"
  local agents_dir=$(get_agents_dir)
  local agent_dir="$agents_dir/$agent_name"
  
  # Check if agent already exists
  if [[ -d "$agent_dir" ]]; then
    print_error "Agent already exists: $agent_name"
    print_info "Use 'ci agent info $agent_name' to view agent details"
    exit 1
  fi
  
  print_box "Creating Agent: $agent_name"
  
  # Create agent directory structure
  print_step 1 3 "Creating agent directory structure"
  mkdir -p "$agent_dir/Sessions"
  print_success "Created agent directory: $agent_dir"
  
  # Create README.md
  print_step 2 3 "Creating agent documentation"
  cat > "$agent_dir/README.md" << EOF
# $agent_name

This agent is part of the Collaborative Intelligence system.

## Capabilities

- Add agent capabilities here

## Usage

To use this agent, type \`$agent_name\` in a Claude Code session.

## Sessions

Session records are stored in the Sessions directory.
EOF
  print_success "Created README.md documentation"
  
  # Create MEMORY.md
  cat > "$agent_dir/MEMORY.md" << EOF
# $agent_name Memory

This file stores the long-term memory for the $agent_name agent.

## Core Knowledge

- Agent Name: $agent_name
- Creation Date: $(date)
- Primary Function: [Define primary function]

## System Integration

The $agent_name agent is part of the Collaborative Intelligence ecosystem and follows the standard agent communication protocols.

## Expertise

[Define areas of expertise]
EOF
  print_success "Created MEMORY.md file"
  
  # Create ContinuousLearning.md
  cat > "$agent_dir/ContinuousLearning.md" << EOF
# $agent_name Continuous Learning

This file documents the learning progress of the $agent_name agent.

## Learning Record

### $(date +"%Y-%m-%d")

- Agent created
- Initial memory structure established
- Basic capabilities defined
EOF
  print_success "Created ContinuousLearning.md file"
  
  # Final step
  print_step 3 3 "Finalizing agent creation"
  
  # Ask if agent should be enabled
  if [[ -t 0 && -t 1 ]]; then
    read -p "Do you want to enable this agent in the current project? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      enable_agent "$agent_name" "quiet"
    fi
  fi
  
  print_success "Agent $agent_name created successfully"
  print_info "To enable this agent in your project, run:"
  echo "  ci agent enable $agent_name"
  print_info "To use this agent, type '$agent_name' in a Claude Code session"
  
  return 0
}

# Enable an agent in the current project
enable_agent() {
  local agent_name="$1"
  local quiet="${2:-}"
  local agents_dir=$(get_agents_dir)
  local agent_dir="$agents_dir/$agent_name"
  
  # Check if agent exists
  if [[ ! -d "$agent_dir" ]]; then
    print_error "Agent not found: $agent_name"
    print_info "Use 'ci agent list' to see available agents"
    exit 1
  fi
  
  if [[ "$quiet" != "quiet" ]]; then
    print_box "Enabling Agent: $agent_name"
  fi
  
  # Get current enabled and disabled agents
  local enabled_json=$(get_config "agents.enabled" 2>/dev/null || echo "[]")
  local disabled_json=$(get_config "agents.disabled" 2>/dev/null || echo "[]")
  
  # Convert JSON arrays to bash arrays
  local enabled_agents=($(echo "$enabled_json" | jq -r '.[]' 2>/dev/null || echo ""))
  local disabled_agents=($(echo "$disabled_json" | jq -r '.[]' 2>/dev/null || echo ""))
  
  # Check if agent is already enabled
  if [[ " ${enabled_agents[*]} " == *" $agent_name "* ]]; then
    if [[ "$quiet" != "quiet" ]]; then
      print_info "Agent $agent_name is already enabled"
    fi
    return 0
  fi
  
  # Add agent to enabled agents
  enabled_agents+=("$agent_name")
  
  # Remove agent from disabled agents (if present)
  local new_disabled_agents=()
  for agent in "${disabled_agents[@]}"; do
    if [[ "$agent" != "$agent_name" ]]; then
      new_disabled_agents+=("$agent")
    fi
  done
  disabled_agents=("${new_disabled_agents[@]}")
  
  # Convert bash arrays back to JSON
  local new_enabled_json=$(printf '"%s",' "${enabled_agents[@]}" | sed 's/,$//')
  local new_disabled_json=$(printf '"%s",' "${disabled_agents[@]}" | sed 's/,$//')
  
  # Update configuration
  set_config "project" "agents.enabled" "[${new_enabled_json}]"
  set_config "project" "agents.disabled" "[${new_disabled_json}]"
  
  if [[ "$quiet" != "quiet" ]]; then
    print_success "Agent $agent_name has been enabled"
    print_info "The agent will now be available in this project"
    print_info "To use this agent, type '$agent_name' in a Claude Code session"
  fi
  
  return 0
}

# Disable an agent in the current project
disable_agent() {
  local agent_name="$1"
  local agents_dir=$(get_agents_dir)
  local agent_dir="$agents_dir/$agent_name"
  
  # Check if agent exists
  if [[ ! -d "$agent_dir" ]]; then
    print_error "Agent not found: $agent_name"
    print_info "Use 'ci agent list' to see available agents"
    exit 1
  fi
  
  print_box "Disabling Agent: $agent_name"
  
  # Get current enabled and disabled agents
  local enabled_json=$(get_config "agents.enabled" 2>/dev/null || echo "[]")
  local disabled_json=$(get_config "agents.disabled" 2>/dev/null || echo "[]")
  
  # Convert JSON arrays to bash arrays
  local enabled_agents=($(echo "$enabled_json" | jq -r '.[]' 2>/dev/null || echo ""))
  local disabled_agents=($(echo "$disabled_json" | jq -r '.[]' 2>/dev/null || echo ""))
  
  # Check if agent is already disabled
  if [[ " ${disabled_agents[*]} " == *" $agent_name "* ]]; then
    print_info "Agent $agent_name is already disabled"
    return 0
  fi
  
  # Add agent to disabled agents
  disabled_agents+=("$agent_name")
  
  # Remove agent from enabled agents (if present)
  local new_enabled_agents=()
  for agent in "${enabled_agents[@]}"; do
    if [[ "$agent" != "$agent_name" ]]; then
      new_enabled_agents+=("$agent")
    fi
  done
  enabled_agents=("${new_enabled_agents[@]}")
  
  # Convert bash arrays back to JSON
  local new_enabled_json=$(printf '"%s",' "${enabled_agents[@]}" | sed 's/,$//')
  local new_disabled_json=$(printf '"%s",' "${disabled_agents[@]}" | sed 's/,$//')
  
  # Update configuration
  set_config "project" "agents.enabled" "[${new_enabled_json}]"
  set_config "project" "agents.disabled" "[${new_disabled_json}]"
  
  print_success "Agent $agent_name has been disabled"
  print_info "The agent will not be active in this project"
  print_info "To re-enable this agent, run: ci agent enable $agent_name"
  
  return 0
}

# Create an agent from a template
create_agent_from_template() {
  local template="$1"
  local agents_dir=$(get_agents_dir)
  
  # Check if the second argument is provided for agent name
  local agent_name=""
  if [[ $# -ge 2 ]]; then
    agent_name="$2"
  fi
  
  # Templates directory
  local templates_dir="$CI_TOOLS_DIR/templates/agents"
  
  # Check if templates directory exists
  if [[ ! -d "$templates_dir" ]]; then
    print_error "Templates directory not found: $templates_dir"
    print_info "This feature requires agent templates to be installed"
    exit 1
  fi
  
  # Check if template exists
  local template_dir="$templates_dir/$template"
  if [[ ! -d "$template_dir" ]]; then
    print_error "Template not found: $template"
    print_info "Available templates:"
    
    # List available templates
    local found_templates=false
    while IFS= read -r -d '' dir; do
      print_list_item "$(basename "$dir")"
      found_templates=true
    done < <(find "$templates_dir" -mindepth 1 -maxdepth 1 -type d -print0)
    
    if [[ "$found_templates" != "true" ]]; then
      print_info "No templates found in $templates_dir"
    fi
    
    exit 1
  fi
  
  # If agent name not provided, prompt for it
  if [[ -z "$agent_name" && -t 0 && -t 1 ]]; then
    read -p "Enter name for the new agent: " agent_name
  fi
  
  # Validate agent name
  if [[ -z "$agent_name" ]]; then
    print_error "Agent name is required"
    print_info "Usage: ci agent template $template <agent_name>"
    exit 1
  fi
  
  # Check if agent already exists
  local agent_dir="$agents_dir/$agent_name"
  if [[ -d "$agent_dir" ]]; then
    print_error "Agent already exists: $agent_name"
    print_info "Use 'ci agent info $agent_name' to view agent details"
    exit 1
  fi
  
  print_box "Creating Agent from Template: $template -> $agent_name"
  
  # Create agent directory
  print_step 1 3 "Creating agent directory structure"
  mkdir -p "$agent_dir/Sessions"
  print_success "Created agent directory: $agent_dir"
  
  # Copy template files
  print_step 2 3 "Copying template files"
  cp -r "$template_dir"/* "$agent_dir/" 2>/dev/null || true
  
  # Replace template placeholders with agent name
  find "$agent_dir" -type f -exec sed -i "s/TEMPLATE_NAME/$agent_name/g" {} \; 2>/dev/null || true
  find "$agent_dir" -type f -exec sed -i "s/TEMPLATE_DATE/$(date)/g" {} \; 2>/dev/null || true
  
  print_success "Applied template: $template"
  
  # Final step
  print_step 3 3 "Finalizing agent creation"
  
  # Ask if agent should be enabled
  if [[ -t 0 && -t 1 ]]; then
    read -p "Do you want to enable this agent in the current project? (y/n) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
      enable_agent "$agent_name" "quiet"
    fi
  fi
  
  print_success "Agent $agent_name created successfully from template $template"
  print_info "To enable this agent in your project, run:"
  echo "  ci agent enable $agent_name"
  print_info "To use this agent, type '$agent_name' in a Claude Code session"
  
  return 0
}

# Debug logging function
debug_log() {
  if [[ "${MEMORY_DEBUG:-false}" == "true" ]]; then
    echo "[DEBUG] $1" >&2
  fi
}

# Activate an agent for the current session with optimized memory loading
activate_agent() {
  local agent_name="$1"
  local context="${2:-}"
  local agents_dir=$(get_agents_dir)
  local agent_dir="$agents_dir/$agent_name"
  
  # Check if agent exists
  if [[ ! -d "$agent_dir" ]]; then
    print_error "Agent not found: $agent_name"
    print_info "Use 'ci agent list' to see available agents"
    exit 1
  fi
  
  print_box "Activating Agent: $agent_name"
  
  # Agent files
  local readme="$agent_dir/README.md"
  local memory="$agent_dir/MEMORY.md"
  local learning="$agent_dir/ContinuousLearning.md"
  
  # Check if agent is enabled
  local enabled_agents=($(get_config "agents.enabled" 2>/dev/null || echo "[]" | jq -r '.[]' 2>/dev/null || echo ""))
  local is_enabled=false
  
  for agent in "${enabled_agents[@]}"; do
    if [[ "$agent" == "$agent_name" ]]; then
      is_enabled=true
      break
    fi
  done
  
  if [[ "$is_enabled" != "true" ]]; then
    print_warning "Agent $agent_name is not enabled in this project"
    print_info "To enable this agent, run: ci agent enable $agent_name"
    # Continue anyway since the user specifically asked to activate it
  fi
  
  # Create agent cache entry (for all activation methods)
  if [[ "$MEMORY_OPTIMIZATION_AVAILABLE" == "true" ]]; then
    debug_log "Creating optimized agent cache entry"
    create_optimized_agent_cache "$agent_name" "$(pwd)" "AgentCommand" "Activated via agent command"
  fi
  
  # Load agent memory with optimizations
  print_step 1 3 "Loading agent memory"
  
  # Try optimized memory loading first
  local memory_loaded=false
  if [[ "$MEMORY_OPTIMIZATION_AVAILABLE" == "true" ]] && type load_optimized_agent_memory &>/dev/null; then
    debug_log "Using optimized memory loading for agent: $agent_name"
    if [[ -n "$context" ]]; then
      print_info "Context: $context"
    fi
    
    # Output marker to indicate agent activation in Claude Code
    echo "@[AGENT_ACTIVATION:$agent_name]"
    
    # Try to load memory with optimized loader
    if load_optimized_agent_memory "$agent_name" "$context"; then
      debug_log "Successfully loaded optimized memory for agent $agent_name"
      memory_loaded=true
    else
      debug_log "Optimized memory loading failed, falling back to legacy"
    fi
  fi
  
  # Fall back to legacy memory loading if necessary
  if [[ "$memory_loaded" == "false" ]]; then
    if type load_agent_memory &>/dev/null; then
      print_info "Using standard memory loading for agent: $agent_name"
      if [[ -n "$context" ]]; then
        print_info "Context: $context"
      fi
      
      # Output marker to indicate agent activation in Claude Code
      echo "@[AGENT_ACTIVATION:$agent_name]"
      
      # Load memory with context awareness
      load_agent_memory "$agent_name" "$context"
    else
      print_warning "Memory loading function not available"
      print_info "Using basic memory loading"
      
      # Output marker to indicate agent activation in Claude Code
      echo "@[AGENT_ACTIVATION:$agent_name]"
      
      # Basic memory loading (fallback)
      cat "$memory" 2>/dev/null || echo "# $agent_name Memory (Empty)"
      echo -e "\n\n# Continuous Learning\n"
      cat "$learning" 2>/dev/null || echo "# $agent_name Continuous Learning (Empty)"
    fi
  fi
  
  print_step 2 3 "Setting agent context"
  # Output information about the activated agent
  print_info "Agent activated: $agent_name"
  if [[ -n "$context" ]]; then
    print_info "Context: $context"
  fi
  
  print_step 3 3 "Agent ready"
  print_success "Agent $agent_name is now active"
  print_info "You can interact with this agent directly in Claude Code"
  
  return 0
}

# Main function
main() {
  # Parse command arguments
  local subcommand=""
  local agent_name=""
  local custom_config=""
  
  # Check for at least one argument (subcommand)
  if [[ $# -eq 0 ]]; then
    print_error "No subcommand provided"
    show_help
    exit 1
  fi
  
  # Get subcommand
  subcommand="$1"
  shift
  
  # Parse options and arguments
  while [[ $# -gt 0 ]]; do
    case "$1" in
      -c|--config)
        custom_config="$2"
        shift 2
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
        # Store positional argument (agent name)
        if [[ -z "$agent_name" ]]; then
          agent_name="$1"
        else
          print_error "Too many arguments"
          show_help
          exit 1
        fi
        shift
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
  
  # Execute subcommand
  case "$subcommand" in
    list)
      list_agents
      ;;
    info)
      if [[ -z "$agent_name" ]]; then
        print_error "Agent name is required for 'info' subcommand"
        print_info "Usage: ci agent info <agent_name>"
        exit 1
      fi
      show_agent_info "$agent_name"
      ;;
    create)
      if [[ -z "$agent_name" ]]; then
        print_error "Agent name is required for 'create' subcommand"
        print_info "Usage: ci agent create <agent_name>"
        exit 1
      fi
      create_agent "$agent_name"
      ;;
    enable)
      if [[ -z "$agent_name" ]]; then
        print_error "Agent name is required for 'enable' subcommand"
        print_info "Usage: ci agent enable <agent_name>"
        exit 1
      fi
      enable_agent "$agent_name"
      ;;
    disable)
      if [[ -z "$agent_name" ]]; then
        print_error "Agent name is required for 'disable' subcommand"
        print_info "Usage: ci agent disable <agent_name>"
        exit 1
      fi
      disable_agent "$agent_name"
      ;;
    template)
      if [[ -z "$agent_name" ]]; then
        print_error "Template name is required for 'template' subcommand"
        print_info "Usage: ci agent template <template> [agent_name]"
        exit 1
      fi
      create_agent_from_template "$agent_name" "$2"
      ;;
    activate)
      if [[ -z "$agent_name" ]]; then
        print_error "Agent name is required for 'activate' subcommand"
        print_info "Usage: ci agent activate <agent_name> [context]"
        exit 1
      fi
      activate_agent "$agent_name" "$2"
      ;;
    --help|-h)
      show_help
      exit 0
      ;;
    *)
      print_error "Unknown subcommand: $subcommand"
      print_info "Available subcommands:"
      for cmd in "${SUBCOMMANDS[@]}"; do
        local cmd_name="${cmd%%:*}"
        local cmd_desc="${cmd#*:}"
        printf "  %-20s %s\n" "$cmd_name" "$cmd_desc"
      done
      exit 1
      ;;
  esac
  
  return 0
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  main "$@"
fi