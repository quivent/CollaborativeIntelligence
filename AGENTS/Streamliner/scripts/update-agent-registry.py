#!/usr/bin/env python3
"""
Update AGENTS.md and AGENT_INDEX.md with new agent information
"""

import sys
import re
from datetime import datetime

def insert_agent_to_agents_md(file_path, agent_name, agent_role, description):
    """Insert agent entry into AGENTS.md in the appropriate section"""
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Find the Development Agents section
    dev_section_pattern = r'(## Development Agents\s*\n)'
    match = re.search(dev_section_pattern, content)
    
    if not match:
        print("Warning: Could not find Development Agents section")
        return False
    
    # Create the new agent entry
    agent_entry = f"""
### {agent_name} - {agent_role}
- **Role**: {agent_role}
- **Expertise**: [TO BE FILLED]
- **Focus**: {description}
- **Perspective**: "[TO BE FILLED]"
- **Primary Responsibilities**:
  - [TO BE FILLED]
- **Operational Guidelines**:
  - [TO BE FILLED]
- **Activation Protocol**:
  - Direct invocation: `{agent_name}`
"""
    
    # Insert after the section header
    insert_pos = match.end()
    new_content = content[:insert_pos] + agent_entry + content[insert_pos:]
    
    with open(file_path, 'w') as f:
        f.write(new_content)
    
    return True

def update_agent_index(file_path, agent_name, agent_role, description):
    """Update AGENT_INDEX.md with new agent"""
    with open(file_path, 'r') as f:
        lines = f.readlines()
    
    # Create new entry
    new_entry = f"| {agent_name} | {agent_role} | {description} |\n"
    
    # Find insert position (alphabetically)
    insert_pos = len(lines)
    for i, line in enumerate(lines):
        if line.startswith('|') and agent_name < line.split('|')[1].strip():
            insert_pos = i
            break
    
    lines.insert(insert_pos, new_entry)
    
    with open(file_path, 'w') as f:
        f.writelines(lines)
    
    return True

if __name__ == "__main__":
    if len(sys.argv) < 4:
        print("Usage: update-agent-registry.py <agent_name> <agent_role> <description>")
        sys.exit(1)
    
    agent_name = sys.argv[1]
    agent_role = sys.argv[2]
    description = sys.argv[3]
    
    base_dir = "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
    
    # Update AGENTS.md
    agents_file = f"{base_dir}/AGENTS.md"
    if insert_agent_to_agents_md(agents_file, agent_name, agent_role, description):
        print(f"Updated {agents_file}")
    
    # Update AGENT_INDEX.md
    index_file = f"{base_dir}/AGENT_INDEX.md"
    if update_agent_index(index_file, agent_name, agent_role, description):
        print(f"Updated {index_file}")