#!/bin/bash
# claude-integrity.sh - Maintains integrity of critical CLAUDE.md configuration
# Part of the Collaborative Intelligence system

# Strict mode
set -euo pipefail

# Get directory where this script is located
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CI_ROOT_DIR="$(cd "$CI_TOOLS_DIR/.." && pwd)"

# Import common libraries if they exist
if [[ -f "$SCRIPT_DIR/common.sh" ]]; then
  source "$SCRIPT_DIR/common.sh"
fi

if [[ -f "$SCRIPT_DIR/output.sh" ]]; then
  source "$SCRIPT_DIR/output.sh"
fi

# Define color codes if they don't exist
GREEN="${GREEN:-\033[0;32m}"
RED="${RED:-\033[0;31m}"
YELLOW="${YELLOW:-\033[1;33m}"
NC="${NC:-\033[0m}" # No Color

# Main function to check and restore CLAUDE.md integrity
function verify_claude_integrity() {
  local ci_path="${1:-$CI_ROOT_DIR}"
  local claude_md="$ci_path/CLAUDE.md"
  local claude_backup="$ci_path/.CLAUDE.md.backup"
  local claude_protected="$ci_path/CLAUDE.md.protected"
  
  # Function to print status messages
  function print_status() {
    local status="$1"
    local message="$2"
    
    if type print_success &>/dev/null && [[ "$status" == "success" ]]; then
      print_success "$message"
    elif type print_error &>/dev/null && [[ "$status" == "error" ]]; then
      print_error "$message"
    elif type print_warning &>/dev/null && [[ "$status" == "warning" ]]; then
      print_warning "$message"
    else
      if [[ "$status" == "success" ]]; then
        echo -e "${GREEN}✓ $message${NC}"
      elif [[ "$status" == "error" ]]; then
        echo -e "${RED}✘ $message${NC}"
      elif [[ "$status" == "warning" ]]; then
        echo -e "${YELLOW}⚠ $message${NC}"
      else
        echo "$message"
      fi
    fi
  }
  
  # Check if CLAUDE.md exists
  if [[ ! -f "$claude_md" ]]; then
    print_status "error" "CLAUDE.md not found at $claude_md"
    
    # Try to restore from backup
    if [[ -f "$claude_backup" ]]; then
      print_status "warning" "Attempting to restore CLAUDE.md from backup..."
      cp "$claude_backup" "$claude_md"
      print_status "success" "CLAUDE.md restored from backup"
    else
      print_status "error" "No backup file found. Cannot restore CLAUDE.md"
      return 1
    fi
  fi
  
  # Create backup if it doesn't exist
  if [[ ! -f "$claude_backup" ]]; then
    print_status "info" "Creating backup of CLAUDE.md..."
    cp "$claude_md" "$claude_backup"
    chmod 444 "$claude_backup"
    print_status "success" "Backup created at $claude_backup"
  fi
  
  # Create protected file if it doesn't exist
  if [[ ! -f "$claude_protected" ]]; then
    print_status "info" "Creating protected CLAUDE.md version..."
    # Extract critical sections or use a template
    {
      echo "# Critical Protected Instructions - DO NOT MODIFY"
      echo ""
      echo "## Agent Activation and Response Format"
      echo ""
      echo "### Activating Agents"
      echo ""
      echo "The system recognizes multiple formats for activating agents:"
      echo ""
      echo "1. **Direct Name**: Simply type the agent's name at the beginning of your message:"
      echo "   \`\`\`"
      echo "   Athena"
      echo "   \`\`\`"
      echo ""
      echo "2. **Agent:NAME Format**: Use the Agent: prefix followed by the agent name:"
      echo "   \`\`\`"
      echo "   Agent:Athena"
      echo "   \`\`\`"
      echo ""
      echo "3. **Bracketed Format**: Use the agent name in square brackets:"
      echo "   \`\`\`"
      echo "   [Athena]"
      echo "   \`\`\`"
      echo ""
      echo "When you see any of these patterns at the beginning of a message, immediately:"
      echo "1. Parse the agent name from the pattern"
      echo "2. Switch to that named agent's context and capabilities"
      echo "3. Process the rest of the message (if any) as that agent"
      echo "4. Format your response with the proper agent name prefix in square brackets"
      echo ""
      echo "### Agent Response Format"
      echo ""
      echo "All agents should prefix their responses with their agent name in square brackets:"
      echo ""
      echo "\`\`\`"
      echo "[AGENT_NAME]: Response content goes here..."
      echo "\`\`\`"
    } > "$claude_protected"
    chmod 444 "$claude_protected"
    print_status "success" "Protected file created at $claude_protected"
  fi
  
  # Verify critical sections exist in CLAUDE.md
  local missing_sections=false
  
  # Check for agent activation section
  if ! grep -q "Agent Activation and Response Format" "$claude_md"; then
    print_status "warning" "CLAUDE.md is missing 'Agent Activation and Response Format' section"
    missing_sections=true
  fi
  
  # Check for activating agents section
  if ! grep -q "Activating Agents" "$claude_md"; then
    print_status "warning" "CLAUDE.md is missing 'Activating Agents' section"
    missing_sections=true
  fi
  
  # Check for agent response format section
  if ! grep -q "Agent Response Format" "$claude_md"; then
    print_status "warning" "CLAUDE.md is missing 'Agent Response Format' section"
    missing_sections=true
  fi
  
  # Restore if missing critical sections
  if [[ "$missing_sections" == "true" ]]; then
    print_status "warning" "CLAUDE.md is missing critical sections, attempting to restore..."
    
    # Try to restore from backup
    if [[ -f "$claude_backup" ]]; then
      if grep -q "Agent Activation and Response Format" "$claude_backup"; then
        cp "$claude_backup" "$claude_md"
        print_status "success" "CLAUDE.md restored from backup"
      else
        print_status "error" "Backup is also missing critical sections"
        
        # Try to merge protected content into CLAUDE.md
        print_status "warning" "Attempting to merge protected content into CLAUDE.md..."
        
        # Create a temporary file for merging
        local temp_file=$(mktemp)
        
        # Prepend the critical sections from protected file
        cat "$claude_protected" > "$temp_file"
        echo "" >> "$temp_file"
        echo "# Original CLAUDE.md content below" >> "$temp_file"
        echo "" >> "$temp_file"
        cat "$claude_md" >> "$temp_file"
        
        # Replace CLAUDE.md with the merged content
        mv "$temp_file" "$claude_md"
        print_status "success" "CLAUDE.md repaired with critical sections"
      fi
    else
      print_status "error" "No valid backup found"
      return 1
    fi
  else
    print_status "success" "CLAUDE.md integrity verified"
    
    # Check if backup should be updated
    if ! cmp -s "$claude_md" "$claude_backup"; then
      # Compare timestamps
      local claude_time=$(stat -f "%m" "$claude_md")
      local backup_time=$(stat -f "%m" "$claude_backup")
      
      # Only update backup if CLAUDE.md is newer and has all critical sections
      if [[ $claude_time -gt $backup_time ]]; then
        print_status "info" "Updating CLAUDE.md backup with newer version..."
        cp "$claude_md" "$claude_backup"
        chmod 444 "$claude_backup"
        print_status "success" "Backup updated"
      fi
    fi
  fi
  
  return 0
}

# Run the function if this script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  CI_PATH="${1:-$CI_ROOT_DIR}"
  verify_claude_integrity "$CI_PATH"
fi