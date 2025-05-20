#!/usr/bin/env bash
# session.sh - Session management command for the CI system
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
CMD_NAME="session"
CMD_DESCRIPTION="Manage interaction sessions in the Collaborative Intelligence system"
CMD_USAGE="ci session <subcommand> [options]"
CMD_OPTIONS="
  -c, --config FILE    Use custom configuration file
  -a, --agent AGENT    Specify agent for the session
"

# Subcommands and their descriptions
SUBCOMMANDS=(
  "list:List available sessions"
  "info:Show detailed information about a session"
  "create:Create a new session"
  "archive:Archive a session"
  "export:Export a session to markdown or JSON"
  "import:Import a session from markdown or JSON"
)

# Show command-specific help
show_help() {
  cat << EOF
$CMD_DESCRIPTION

Usage: $CMD_USAGE

Subcommands:
  list                     List all available sessions
  info <session_id>        Show detailed information about a session
  create <session_name>    Create a new session
  archive <session_id>     Archive a session
  export <session_id>      Export a session to markdown or JSON
  import <file>            Import a session from markdown or JSON

Options:$CMD_OPTIONS

Examples:
  ci session list                       List all available sessions
  ci session list --agent Athena        List sessions for Athena agent
  ci session info ABC123                Show details about session ABC123
  ci session create "Feature Planning"  Create a new session
  ci session archive ABC123             Archive session ABC123
  ci session export ABC123              Export session ABC123 to markdown
  ci session import session.md          Import a session from markdown
  ci session --help                     Show this help message
EOF
}

# Get the sessions directory
get_sessions_dir() {
  local sessions_dir=$(get_config "sessions.paths.base")
  
  if [[ -z "$sessions_dir" ]]; then
    print_error "Sessions directory not configured"
    print_info "Run 'ci fix' to repair configuration"
    exit 1
  fi
  
  if [[ ! -d "$sessions_dir" ]]; then
    print_error "Sessions directory not found: $sessions_dir"
    print_info "Run 'ci fix' to create the directory"
    exit 1
  fi
  
  echo "$sessions_dir"
}

# Get the agent sessions directory
get_agent_sessions_dir() {
  local agent_name="$1"
  local agents_dir=$(get_config "agents.paths.base")
  
  if [[ -z "$agents_dir" ]]; then
    print_error "Agents directory not configured"
    print_info "Run 'ci fix' to repair configuration"
    exit 1
  fi
  
  local agent_dir="$agents_dir/$agent_name"
  
  if [[ ! -d "$agent_dir" ]]; then
    print_error "Agent not found: $agent_name"
    print_info "Use 'ci agent list' to see available agents"
    exit 1
  fi
  
  local sessions_dir="$agent_dir/Sessions"
  
  if [[ ! -d "$sessions_dir" ]]; then
    mkdir -p "$sessions_dir"
  fi
  
  echo "$sessions_dir"
}

# Get a session directory
get_session_dir() {
  local session_id="$1"
  local agent="${2:-}"
  
  if [[ -n "$agent" ]]; then
    # Look for session in agent-specific sessions
    local agent_sessions_dir=$(get_agent_sessions_dir "$agent")
    if [[ -d "$agent_sessions_dir/$session_id" ]]; then
      echo "$agent_sessions_dir/$session_id"
      return 0
    fi
  else
    # Look for session in all agent directories
    local agents_dir=$(get_config "agents.paths.base")
    if [[ -n "$agents_dir" && -d "$agents_dir" ]]; then
      while IFS= read -r -d '' agent_dir; do
        local agent_name=$(basename "$agent_dir")
        local agent_sessions_dir="$agent_dir/Sessions"
        
        if [[ -d "$agent_sessions_dir/$session_id" ]]; then
          echo "$agent_sessions_dir/$session_id"
          return 0
        fi
      done < <(find "$agents_dir" -mindepth 1 -maxdepth 1 -type d -print0)
    fi
  
    # Look for session in system-wide sessions
    local sessions_dir=$(get_sessions_dir)
    if [[ -d "$sessions_dir/$session_id" ]]; then
      echo "$sessions_dir/$session_id"
      return 0
    fi
  fi
  
  return 1
}

# Generate a session ID
generate_session_id() {
  local prefix="$1"
  local timestamp=$(date "+%Y%m%d%H%M%S")
  local random=$(head /dev/urandom | tr -dc 'a-z0-9' | head -c 4)
  
  echo "${prefix}${timestamp}-${random}"
}

# List available sessions
list_sessions() {
  local agent="${1:-}"
  
  if [[ -n "$agent" ]]; then
    print_box "Sessions for Agent: $agent"
    local sessions_dir=$(get_agent_sessions_dir "$agent")
  else
    print_box "Available Sessions"
    local sessions_dir=$(get_sessions_dir)
  fi
  
  # Get all session directories
  local session_dirs=()
  while IFS= read -r -d '' dir; do
    if [[ -d "$dir" && -f "$dir/README.md" ]]; then
      session_dirs+=("$dir")
    fi
  done < <(find "$sessions_dir" -mindepth 1 -maxdepth 1 -type d -print0)
  
  # Also search in agent sessions if not already searching for a specific agent
  if [[ -z "$agent" ]]; then
    local agents_dir=$(get_config "agents.paths.base")
    if [[ -n "$agents_dir" && -d "$agents_dir" ]]; then
      while IFS= read -r -d '' agent_dir; do
        local agent_name=$(basename "$agent_dir")
        local agent_sessions_dir="$agent_dir/Sessions"
        
        if [[ -d "$agent_sessions_dir" ]]; then
          while IFS= read -r -d '' dir; do
            if [[ -d "$dir" && -f "$dir/README.md" ]]; then
              session_dirs+=("$dir")
            fi
          done < <(find "$agent_sessions_dir" -mindepth 1 -maxdepth 1 -type d -print0)
        fi
      done < <(find "$agents_dir" -mindepth 1 -maxdepth 1 -type d -print0)
    fi
  fi
  
  if [[ ${#session_dirs[@]} -eq 0 ]]; then
    print_info "No sessions found"
    print_info "Run 'ci session create <name>' to create a new session"
    return 0
  fi
  
  # Sort sessions by modification time (most recent first)
  readarray -t sorted_sessions < <(for dir in "${session_dirs[@]}"; do
    echo "$(stat -f "%m" "$dir" 2>/dev/null || stat --format "%Y" "$dir") $dir"
  done | sort -rn | cut -d' ' -f2-)
  
  print_section "Recent Sessions"
  local count=0
  for dir in "${sorted_sessions[@]}"; do
    local session_id=$(basename "$dir")
    local readme="$dir/README.md"
    local metadata="$dir/metadata.json"
    
    # Get session title from README.md (first heading)
    local title="$session_id"
    if [[ -f "$readme" ]]; then
      local first_heading=$(grep "^# " "$readme" | head -n 1)
      if [[ -n "$first_heading" ]]; then
        title="${first_heading#\# }"
      fi
    fi
    
    # Get session date from metadata.json or use file modification time
    local date=""
    if [[ -f "$metadata" ]]; then
      date=$(jq -r '.created_at // empty' "$metadata" 2>/dev/null)
    fi
    
    if [[ -z "$date" ]]; then
      date=$(date -r "$dir" "+%Y-%m-%d %H:%M:%S" 2>/dev/null || stat --format "%y" "$dir" | cut -d. -f1)
    fi
    
    # Determine agent from path
    local session_agent=""
    if [[ "$dir" == *"/AGENTS/"* ]]; then
      session_agent=$(echo "$dir" | grep -o "/AGENTS/[^/]*/Sessions/" | cut -d'/' -f3)
    fi
    
    # Print session information
    local session_info="$title"
    if [[ -n "$session_agent" ]]; then
      session_info+=" [Agent: $session_agent]"
    fi
    print_list_item "$session_info"
    echo "  ├── ID: $session_id"
    echo "  └── Date: $date"
    
    count=$((count + 1))
    if [[ $count -ge 10 && ${#sorted_sessions[@]} -gt 10 ]]; then
      echo "  ... and $((${#sorted_sessions[@]} - 10)) more sessions"
      break
    fi
  done
  
  echo ""
  print_info "Total sessions: ${#sorted_sessions[@]}"
  print_info "Use 'ci session info <id>' for detailed information"
  
  return 0
}

# Show detailed information about a session
show_session_info() {
  local session_id="$1"
  local agent="${2:-}"
  
  # Get session directory
  local session_dir=$(get_session_dir "$session_id" "$agent")
  
  if [[ -z "$session_dir" || ! -d "$session_dir" ]]; then
    print_error "Session not found: $session_id"
    if [[ -n "$agent" ]]; then
      print_info "Use 'ci session list --agent $agent' to see available sessions"
    else
      print_info "Use 'ci session list' to see available sessions"
    fi
    exit 1
  fi
  
  print_box "Session Information: $session_id"
  
  # Session files
  local readme="$session_dir/README.md"
  local metadata="$session_dir/metadata.json"
  
  # Determine agent from path
  local session_agent=""
  if [[ "$session_dir" == *"/AGENTS/"* ]]; then
    session_agent=$(echo "$session_dir" | grep -o "/AGENTS/[^/]*/Sessions/" | cut -d'/' -f3)
  fi
  
  # Session status
  if [[ -n "$session_agent" ]]; then
    print_section "Agent"
    print_info "This session belongs to the $session_agent agent"
  fi
  
  # Print session path
  print_section "Location"
  echo "Path: $session_dir"
  
  # Check README.md
  if [[ -f "$readme" ]]; then
    # Extract session title (first heading)
    local title=$(grep "^# " "$readme" | head -n 1)
    if [[ -n "$title" ]]; then
      print_section "Title"
      echo "${title#\# }"
    fi
    
    # Extract session description (content after title and before first section)
    print_section "Description"
    local description=$(awk '/^# / {p=1; next} /^##/ {p=0} p {print}' "$readme" | sed '/^$/d' | head -n 5)
    if [[ -n "$description" ]]; then
      echo "$description"
      if [[ $(wc -l <<< "$description") -ge 5 ]]; then
        echo "... (see $readme for full description)"
      fi
    else
      echo "No description available in README.md"
    fi
  else
    print_warning "No README.md found for this session"
  fi
  
  # Check metadata.json
  print_section "Metadata"
  if [[ -f "$metadata" ]]; then
    # Extract creation date
    local created_at=$(jq -r '.created_at // empty' "$metadata" 2>/dev/null)
    if [[ -n "$created_at" ]]; then
      echo "Created: $created_at"
    else
      echo "Created: Unknown"
    fi
    
    # Extract topics
    local topics=$(jq -r '.topics // empty | join(", ")' "$metadata" 2>/dev/null)
    if [[ -n "$topics" ]]; then
      echo "Topics: $topics"
    fi
    
    # Extract other metadata
    local keys=$(jq -r 'keys[] | select(. != "created_at" and . != "topics")' "$metadata" 2>/dev/null)
    if [[ -n "$keys" ]]; then
      while read -r key; do
        local value=$(jq -r ".$key" "$metadata" 2>/dev/null)
        if [[ -n "$value" && "$value" != "null" ]]; then
          # If value is an array, join with commas
          if [[ "$value" == \[* ]]; then
            value=$(jq -r ".$key | join(\", \")" "$metadata" 2>/dev/null)
          fi
          echo "$key: $value"
        fi
      done <<< "$keys"
    fi
  else
    echo "No metadata.json found for this session"
    
    # Show file modification time as fallback
    local mod_time=$(date -r "$session_dir" "+%Y-%m-%d %H:%M:%S" 2>/dev/null || stat --format "%y" "$session_dir" | cut -d. -f1)
    echo "Last modified: $mod_time"
  fi
  
  # List files in the session
  print_section "Files"
  local files=()
  while IFS= read -r -d '' file; do
    files+=("$(basename "$file")")
  done < <(find "$session_dir" -maxdepth 1 -type f -print0)
  
  if [[ ${#files[@]} -eq 0 ]]; then
    echo "No files found in this session"
  else
    for file in "${files[@]}"; do
      local file_size=$(du -h "$session_dir/$file" | cut -f1)
      echo "- $file ($file_size)"
    done
  fi
  
  return 0
}

# Create a new session
create_session() {
  local session_name="$1"
  local agent="${2:-}"
  
  # Generate a session ID based on the name
  local slug=$(echo "$session_name" | tr '[:upper:]' '[:lower:]' | tr -cd '[:alnum:]' | tr ' ' '-')
  local session_id=$(generate_session_id "${slug:0:20}-")
  
  # Determine the sessions directory
  if [[ -n "$agent" ]]; then
    local sessions_dir=$(get_agent_sessions_dir "$agent")
  else
    local sessions_dir=$(get_sessions_dir)
  fi
  
  local session_dir="$sessions_dir/$session_id"
  
  print_box "Creating Session: $session_name"
  
  # Create session directory
  print_step 1 3 "Creating session directory"
  mkdir -p "$session_dir"
  print_success "Created session directory: $session_dir"
  
  # Create README.md
  print_step 2 3 "Creating session documentation"
  cat > "$session_dir/README.md" << EOF
# $session_name

Session created on $(date "+%Y-%m-%d at %H:%M:%S").

## Description

Add a description of this session here.

## Summary

Summarize the key points and outcomes of this session.

## Participants

- User
EOF

  if [[ -n "$agent" ]]; then
    echo "- $agent" >> "$session_dir/README.md"
  else
    echo "- Collaborative Intelligence System" >> "$session_dir/README.md"
  fi
  
  print_success "Created README.md documentation"
  
  # Create metadata.json
  print_step 3 3 "Creating session metadata"
  cat > "$session_dir/metadata.json" << EOF
{
  "created_at": "$(date -u "+%Y-%m-%dT%H:%M:%SZ")",
  "title": "$session_name",
  "topics": [],
  "status": "active"
EOF

  if [[ -n "$agent" ]]; then
    cat >> "$session_dir/metadata.json" << EOF
,
  "agent": "$agent"
EOF
  fi

  echo "}" >> "$session_dir/metadata.json"
  
  print_success "Created metadata.json file"
  
  # Final message
  if [[ -n "$agent" ]]; then
    print_success "Session $session_id created successfully for agent $agent"
  else
    print_success "Session $session_id created successfully"
  fi
  
  print_info "Access the session with:"
  echo "  ci session info $session_id"
  
  return 0
}

# Archive a session
archive_session() {
  local session_id="$1"
  local agent="${2:-}"
  
  # Get session directory
  local session_dir=$(get_session_dir "$session_id" "$agent")
  
  if [[ -z "$session_dir" || ! -d "$session_dir" ]]; then
    print_error "Session not found: $session_id"
    if [[ -n "$agent" ]]; then
      print_info "Use 'ci session list --agent $agent' to see available sessions"
    else
      print_info "Use 'ci session list' to see available sessions"
    fi
    exit 1
  fi
  
  print_box "Archiving Session: $session_id"
  
  # Check if it's already archived
  local metadata="$session_dir/metadata.json"
  if [[ -f "$metadata" ]]; then
    local status=$(jq -r '.status // empty' "$metadata" 2>/dev/null)
    if [[ "$status" == "archived" ]]; then
      print_info "Session is already archived"
      return 0
    fi
  fi
  
  # Create archives directory
  local sessions_dir=$(dirname "$session_dir")
  local archives_dir="$sessions_dir/Archives"
  mkdir -p "$archives_dir"
  
  # Move the session to archives
  local archive_dir="$archives_dir/$session_id"
  
  # Update metadata.json if exists, otherwise create it
  if [[ -f "$metadata" ]]; then
    # Update status in metadata.json
    jq '.status = "archived" | .archived_at = "'"$(date -u "+%Y-%m-%dT%H:%M:%SZ")"'"' "$metadata" > "$metadata.tmp"
    mv "$metadata.tmp" "$metadata"
  else
    # Create metadata.json with archived status
    cat > "$session_dir/metadata.json" << EOF
{
  "created_at": "unknown",
  "archived_at": "$(date -u "+%Y-%m-%dT%H:%M:%SZ")",
  "status": "archived"
}
EOF
  fi
  
  # Move the session
  mv "$session_dir" "$archive_dir"
  
  print_success "Session $session_id has been archived"
  print_info "The session has been moved to: $archive_dir"
  
  return 0
}

# Export a session to markdown or JSON
export_session() {
  local session_id="$1"
  local format="${2:-markdown}"
  local agent="${3:-}"
  
  # Get session directory
  local session_dir=$(get_session_dir "$session_id" "$agent")
  
  if [[ -z "$session_dir" || ! -d "$session_dir" ]]; then
    print_error "Session not found: $session_id"
    if [[ -n "$agent" ]]; then
      print_info "Use 'ci session list --agent $agent' to see available sessions"
    else
      print_info "Use 'ci session list' to see available sessions"
    fi
    exit 1
  fi
  
  print_box "Exporting Session: $session_id"
  
  # Determine output file
  local output_dir="."
  local output_file=""
  
  case "$format" in
    markdown|md)
      output_file="$output_dir/$session_id.md"
      # Export to markdown
      print_step 1 2 "Exporting to markdown: $output_file"
      
      # Start with README.md content
      if [[ -f "$session_dir/README.md" ]]; then
        cp "$session_dir/README.md" "$output_file"
      else
        echo "# Session: $session_id" > "$output_file"
        echo "" >> "$output_file"
        echo "Exported on $(date)" >> "$output_file"
        echo "" >> "$output_file"
      fi
      
      # Add metadata
      echo "" >> "$output_file"
      echo "## Metadata" >> "$output_file"
      echo "" >> "$output_file"
      
      if [[ -f "$session_dir/metadata.json" ]]; then
        echo "```json" >> "$output_file"
        cat "$session_dir/metadata.json" >> "$output_file"
        echo "```" >> "$output_file"
        echo "" >> "$output_file"
      fi
      
      # Add other markdown files
      find "$session_dir" -maxdepth 1 -name "*.md" -not -name "README.md" | while read -r md_file; do
        echo "" >> "$output_file"
        echo "## $(basename "$md_file" .md)" >> "$output_file"
        echo "" >> "$output_file"
        cat "$md_file" >> "$output_file"
        echo "" >> "$output_file"
      done
      ;;
      
    json)
      output_file="$output_dir/$session_id.json"
      # Export to JSON
      print_step 1 2 "Exporting to JSON: $output_file"
      
      # Create a JSON object with session data
      echo "{" > "$output_file"
      echo "  \"session_id\": \"$session_id\"," >> "$output_file"
      echo "  \"exported_at\": \"$(date -u "+%Y-%m-%dT%H:%M:%SZ")\"," >> "$output_file"
      
      # Add metadata if available
      if [[ -f "$session_dir/metadata.json" ]]; then
        echo "  \"metadata\": $(cat "$session_dir/metadata.json")," >> "$output_file"
      fi
      
      # Add README content
      if [[ -f "$session_dir/README.md" ]]; then
        echo "  \"readme\": $(jq -Rs . < "$session_dir/README.md")," >> "$output_file"
      fi
      
      # Add other files
      echo "  \"files\": {" >> "$output_file"
      first_file=true
      find "$session_dir" -maxdepth 1 -type f -not -name "README.md" -not -name "metadata.json" | while read -r file; do
        file_name=$(basename "$file")
        if [[ "$first_file" == "true" ]]; then
          first_file=false
        else
          echo "," >> "$output_file"
        fi
        echo -n "    \"$file_name\": $(jq -Rs . < "$file")" >> "$output_file"
      done
      echo "" >> "$output_file"
      echo "  }" >> "$output_file"
      echo "}" >> "$output_file"
      ;;
      
    *)
      print_error "Unsupported export format: $format"
      print_info "Supported formats: markdown, json"
      exit 1
      ;;
  esac
  
  print_step 2 2 "Finalizing export"
  
  # Check if export was successful
  if [[ -f "$output_file" ]]; then
    print_success "Session $session_id exported successfully"
    print_info "Exported to: $output_file"
  else
    print_error "Failed to export session"
    exit 1
  fi
  
  return 0
}

# Import a session from markdown or JSON
import_session() {
  local input_file="$1"
  local agent="${2:-}"
  
  # Check if input file exists
  if [[ ! -f "$input_file" ]]; then
    print_error "Input file not found: $input_file"
    exit 1
  fi
  
  print_box "Importing Session from: $(basename "$input_file")"
  
  # Determine import format based on file extension
  local format=""
  if [[ "$input_file" == *.md || "$input_file" == *.markdown ]]; then
    format="markdown"
  elif [[ "$input_file" == *.json ]]; then
    format="json"
  else
    print_error "Unsupported file format: $(basename "$input_file")"
    print_info "Supported formats: .md, .markdown, .json"
    exit 1
  fi
  
  # Determine the sessions directory
  if [[ -n "$agent" ]]; then
    local sessions_dir=$(get_agent_sessions_dir "$agent")
  else
    local sessions_dir=$(get_sessions_dir)
  fi
  
  # Generate a session ID
  local base_name=$(basename "$input_file" | sed 's/\.[^.]*$//')
  local session_id=$(generate_session_id "import-${base_name:0:15}-")
  local session_dir="$sessions_dir/$session_id"
  
  print_step 1 3 "Creating session directory"
  mkdir -p "$session_dir"
  print_success "Created session directory: $session_dir"
  
  print_step 2 3 "Importing content"
  case "$format" in
    markdown)
      # Import from markdown
      # Extract title from markdown
      local title=$(grep "^# " "$input_file" | head -n 1 | sed 's/^# //')
      if [[ -z "$title" ]]; then
        title="Imported Session"
      fi
      
      # Create README.md
      cp "$input_file" "$session_dir/README.md"
      
      # Create metadata.json
      cat > "$session_dir/metadata.json" << EOF
{
  "created_at": "$(date -u "+%Y-%m-%dT%H:%M:%SZ")",
  "imported_at": "$(date -u "+%Y-%m-%dT%H:%M:%SZ")",
  "title": "$title",
  "source": "$(basename "$input_file")",
  "status": "active"
EOF

      if [[ -n "$agent" ]]; then
        cat >> "$session_dir/metadata.json" << EOF
,
  "agent": "$agent"
EOF
      fi

      echo "}" >> "$session_dir/metadata.json"
      ;;
      
    json)
      # Import from JSON
      # Extract metadata, readme, and files from JSON
      
      # Extract README.md content if available
      jq -r '.readme // empty' "$input_file" > "$session_dir/README.md"
      
      # Extract metadata if available
      jq -r '.metadata // empty' "$input_file" > "$session_dir/metadata.json.tmp"
      if [[ -s "$session_dir/metadata.json.tmp" ]]; then
        # Update imported_at field
        jq '. + {"imported_at": "'"$(date -u "+%Y-%m-%dT%H:%M:%SZ")"'"}' "$session_dir/metadata.json.tmp" > "$session_dir/metadata.json"
        rm "$session_dir/metadata.json.tmp"
      else
        # Create basic metadata
        cat > "$session_dir/metadata.json" << EOF
{
  "created_at": "$(date -u "+%Y-%m-%dT%H:%M:%SZ")",
  "imported_at": "$(date -u "+%Y-%m-%dT%H:%M:%SZ")",
  "source": "$(basename "$input_file")",
  "status": "active"
EOF

        if [[ -n "$agent" ]]; then
          cat >> "$session_dir/metadata.json" << EOF
,
  "agent": "$agent"
EOF
        fi

        echo "}" >> "$session_dir/metadata.json"
      fi
      
      # Extract other files
      jq -r '.files | keys[]' "$input_file" 2>/dev/null | while read -r key; do
        jq -r ".files[\"$key\"]" "$input_file" > "$session_dir/$key"
      done
      ;;
  esac
  
  print_step 3 3 "Finalizing import"
  
  # Check if README.md was created
  if [[ ! -f "$session_dir/README.md" ]]; then
    cat > "$session_dir/README.md" << EOF
# Imported Session

Session imported from $(basename "$input_file") on $(date).

## Description

This session was imported from an external source.
EOF
  fi
  
  print_success "Session imported successfully with ID: $session_id"
  print_info "Access the session with:"
  echo "  ci session info $session_id"
  
  return 0
}

# Main function
main() {
  # Parse command arguments
  local subcommand=""
  local session_id=""
  local agent=""
  local custom_config=""
  local format="markdown"
  
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
      -a|--agent)
        agent="$2"
        shift 2
        ;;
      -c|--config)
        custom_config="$2"
        shift 2
        ;;
      --format)
        format="$2"
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
        # Store positional argument (session_id)
        if [[ -z "$session_id" ]]; then
          session_id="$1"
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
      list_sessions "$agent"
      ;;
    info)
      if [[ -z "$session_id" ]]; then
        print_error "Session ID is required for 'info' subcommand"
        print_info "Usage: ci session info <session_id>"
        exit 1
      fi
      show_session_info "$session_id" "$agent"
      ;;
    create)
      if [[ -z "$session_id" ]]; then
        print_error "Session name is required for 'create' subcommand"
        print_info "Usage: ci session create <session_name>"
        exit 1
      fi
      create_session "$session_id" "$agent"
      ;;
    archive)
      if [[ -z "$session_id" ]]; then
        print_error "Session ID is required for 'archive' subcommand"
        print_info "Usage: ci session archive <session_id>"
        exit 1
      fi
      archive_session "$session_id" "$agent"
      ;;
    export)
      if [[ -z "$session_id" ]]; then
        print_error "Session ID is required for 'export' subcommand"
        print_info "Usage: ci session export <session_id> [format]"
        exit 1
      fi
      export_session "$session_id" "$format" "$agent"
      ;;
    import)
      if [[ -z "$session_id" ]]; then
        print_error "Input file is required for 'import' subcommand"
        print_info "Usage: ci session import <file>"
        exit 1
      fi
      import_session "$session_id" "$agent"
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