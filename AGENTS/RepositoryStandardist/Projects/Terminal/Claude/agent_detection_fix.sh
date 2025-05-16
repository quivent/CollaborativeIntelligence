#!/bin/bash
# Agent detection fix for the agent.sh command module

# Function to find the agent.sh file
find_agent_script() {
  local ci_repo_path="${CI_REPO_PATH:-/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence}"
  local agent_script="$ci_repo_path/ci-tools/cmd/agent.sh"
  
  if [[ -f "$agent_script" ]]; then
    echo "$agent_script"
    return 0
  else
    echo "Agent script not found at $agent_script" >&2
    return 1
  fi
}

# Main function
main() {
  # Find the agent.sh script
  local agent_script=$(find_agent_script)
  if [[ $? -ne 0 ]]; then
    echo "Error finding agent script" >&2
    exit 1
  fi
  
  echo "Found agent script at $agent_script"
  echo "Creating backup before modification..."
  
  # Create backup
  cp "$agent_script" "${agent_script}.bak"
  echo "Backup created at ${agent_script}.bak"
  
  # Fix the agent detection logic to include all agent directories
  # (not just those with README.md files)
  echo "Updating agent detection logic..."
  
  # Use sed to update the file
  sed -i.fix1 '94,98s/if \[\[ -d "$dir" \&\& -f "$dir\/README.md" \]\]; then/if \[\[ -d "$dir" \]\]; then/' "$agent_script"
  
  echo "Agent detection logic updated"
  echo "The script will now detect all agent directories, not just those with README.md files"
  echo
  echo "To test the fix, run:"
  echo "  chmod +x $agent_script"
  echo "  /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/ci-agent list"
  echo
  echo "The agent count should now show all 18 agents"
}

# Run the main function
main