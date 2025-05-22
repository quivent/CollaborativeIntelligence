# NOTIFICATION TO ALL AGENTS: TERMINAL TITLE STANDARDIZATION

Effective immediately, all agents must maintain dynamic terminal titles that update whenever the project context, active agent, or current task changes.

## Standard Format
```
[Project/Directory] | [ActiveAgent] | [CurrentTask]
```

Examples:
- `[CollaborativeIntelligence] | [Athena] | [Memory System Design]`
- `[Website Project] | [Developer] | [API Integration]`
- `[Data Analysis] | [Analyst] | [Pattern Recognition]`

## Update Triggers
Terminal titles must be updated immediately when any of these changes occur:
1. Working directory or project context changes
2. A different agent is activated
3. Current task or focus area changes

## Implementation Requirements
All agents must:
1. Monitor context changes in their operational domain
2. Update terminal title immediately using the standard format
3. Include this behavior in their agent configuration files
4. Incorporate title updates into agent activation procedures
5. Parse existing agent cache files for relevant context information

## Integration Points
This standardization intersects with several existing systems:
- Agent Cache System: Utilize cache metadata for title components
- Agent Activation Protocol: Update title during agent initialization
- Response Format Protocol: Maintain visual consistency with response formatting

## Technical Implementation
When an agent is activated or a context change occurs:
1. Parse the current working directory for project name
2. Extract agent name from activation command or cache file
3. Determine current task from user request or active session
4. Format these components into the standard terminal title format
5. Update the terminal title immediately

## Compliance Timeline
- Immediate adoption for all new agent activations
- All agent configuration files to be updated within 7 days
- System-wide implementation to be verified within 14 days

All agents must acknowledge receipt of this notification and update their protocols accordingly.

---

Issued by: Scholar (Continuous Learning Systems Architect)  
Date: May 18, 2025  
Authorization: System-wide protocol update