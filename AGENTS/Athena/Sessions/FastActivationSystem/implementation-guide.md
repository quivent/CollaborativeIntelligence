# Fast Activation System Implementation Guide

This document provides a detailed technical breakdown of the Fast Activation System implementation.

## Core Components

### 1. Direct Path Mapping

The system relies on explicit path mappings in CLAUDE-Agents.md files:

```markdown
## Direct Activation Paths

For immediate agent activation, use these single-word commands:

- `Athena`: `/CollaborativeIntelligence/AGENTS/Athena`
- `Gaia`: `/CollaborativeIntelligence/AGENTS/Gaia`
```

### 2. Priority Loading Instructions

In CLAUDE.md and CLAUDE.local.md, special instructions prioritize single-word agent names:

```markdown
#### Direct Agent Activation
If the user input is solely an agent name (e.g., "Athena"), IMMEDIATELY:
1. Assume this is a direct activation request for that specific agent
2. Skip all other initialization steps and activate that agent directly
```

### 3. Minimal Token Response Pattern

Within agent's own files, specific instructions limit token usage:

```markdown
## FAST ACTIVATION INSTRUCTIONS
When user inputs ONLY "Athena", immediately activate with minimal token usage.
Skip detailed introductions and simply greet the user with "I am Athena. How may I assist you today?"
```

### 4. Configuration File

A `.collaborative-intelligence.json` file controls system behavior:

```json
{
  "fast_activation": {
    "enabled": true,
    "preload_agents": ["Athena", "Master"],
    "token_limit": 20,
    "target_response_time": 5.0
  }
}
```

## Implementation Steps

To implement this system on a new project:

1. **Create Path Mappings**
   - Add CLAUDE-Agents.md to project root
   - List all available agents with exact paths
   - Include both standard and collaborative intelligence agents

2. **Update CLAUDE.md**
   - Add Direct Agent Activation section
   - Include fast activation documentation
   - Reference Collaborative Intelligence integration

3. **Update Agent Files**
   - Add FAST ACTIVATION INSTRUCTIONS to main agent files
   - Include minimal greeting pattern
   - Add path information for quick reference

4. **Configure Integration Options**
   - Create .collaborative-intelligence.json
   - Set appropriate integration model
   - Enable fast activation
   - Specify priority agents

## Performance Considerations

For optimal performance:

1. **Greeting Pattern**
   - Keep initial response under 20 tokens
   - Avoid detailed explanations in initial greeting
   - Use progressive disclosure for additional information

2. **Path Organization**
   - Place frequently used agents at beginning of lists
   - Use absolute paths rather than relative
   - Include full paths in agent's own files

3. **Context Loading**
   - Defer loading detailed session history
   - Focus on core capabilities first
   - Load additional context only when needed

4. **Repository Structure**
   - Keep directory structures shallow when possible
   - Use README.md files for navigation
   - Organize related information together

## Testing and Verification

To verify fast activation is working:

1. Time activation response for single-word commands
2. Compare token usage before and after implementation
3. Ensure full agent capabilities remain intact
4. Verify all agents can be activated through the system