# Agent Memory Permissions System

## Overview

This document describes the granular permission system for agent memory management in Claude Code. It implements the principle that only specific agents can update their own memory files, while maintaining collaborative access where appropriate.

## Permission Model

### MEMORY.md Files
- **Who can edit**: Any Agent + Manager
- **Rationale**: Core memory files require collaborative oversight and management authority
- **Pattern**: `edit:/path/to/CollaborativeIntelligence/AGENTS/*/MEMORY.md`

### ContinuousLearning.md Files  
- **Who can edit**: Only the specific agent themselves
- **Rationale**: Learning files are personal to each agent and should not be modified by others
- **Pattern**: `edit:/path/to/AGENTS/{AgentName}/ContinuousLearning.md:{AgentName}`

### Working Files
- **Who can edit**: Any agent can edit their own working files
- **Pattern**: `edit:/path/to/AGENTS/*/working_*.md`

### Session Directories
- **Who can edit**: Any agent can update their own session directories
- **Pattern**: `edit:/path/to/AGENTS/*/Sessions/**/*`

## Implementation

### Step 1: Copy Configuration
```bash
cp agent-memory-permissions.json ~/.claude/settings.json
```

### Step 2: Merge with Existing Settings
If you have existing Claude Code settings, merge the permissions section:

```json
{
  "existing_settings": "...",
  "permissions": {
    "allow": [
      // Copy the allow array from agent-memory-permissions.json
    ]
  }
}
```

### Step 3: Restart Claude Code
Restart Claude Code to apply the new permissions.

## Usage

### Autonomous Memory Updates
With these permissions enabled:

1. **Agent Session Learning**: Agents automatically capture learning insights
2. **Memory Evolution**: Core memory files can be updated by agents or managers
3. **Private Learning**: Each agent's continuous learning remains private
4. **Session Management**: Agents maintain their own session histories

### Permission Syntax
Claude Code permissions use the format:
- `edit:filepath` - Any authenticated user can edit
- `edit:filepath:agent_name` - Only specified agent can edit
- `edit:filepath_pattern` - Pattern matching for multiple files

## Agent Coverage

The permission file covers **82 active agents** with proper continuous learning file permissions, including:

- Standard agents (Developer, Architect, Manager, etc.)
- Specialized agents (AssemblyMaster, Cryptographer, HardwareArchitect, etc.)  
- Business agents (CFO, BusinessDevelopmentHead, etc.)
- Alternative naming patterns (continuous_learning.md, CONTINUOUS_LEARNING.md, etc.)

## Security Boundaries

### Protected Operations
- Only specific agents can modify their own learning files
- Manager retains oversight authority for all MEMORY.md files
- Working files remain agent-private

### Collaborative Operations  
- MEMORY.md files allow collaborative updates
- Session directories enable cross-agent reference
- Working files support agent autonomy

## Troubleshooting

### Permission Denied Errors
- Verify agent name matches directory name exactly
- Check file path accuracy in configuration
- Confirm Claude Code has reloaded settings

### Learning Updates Not Working
- Ensure agent is using exact naming: `ContinuousLearning.md`
- Check for alternative naming patterns in agent directory
- Verify agent identity matches permission specification

## Benefits

1. **Autonomous Learning**: Agents can evolve their knowledge without friction
2. **Memory Integrity**: Core memory protected but accessible for management
3. **Privacy Boundaries**: Learning files remain agent-specific
4. **Scalable Architecture**: Easy to add new agents with consistent permissions
5. **Collaborative Oversight**: Manager maintains cross-agent memory authority

## Future Enhancements

- Dynamic permission generation based on agent registry
- Role-based permission groups (e.g., "architect_agents", "business_agents")
- Audit logging for memory modifications
- Automatic permission validation on agent creation

---

**Implementation Status**: Ready for deployment
**Coverage**: 82 agents with granular memory permissions
**Security Model**: Agent-private learning, collaborative memory management