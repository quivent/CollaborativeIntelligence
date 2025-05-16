# Collaborative Intelligence Quick Start Guide

## Instant Access

| Type this in terminal | To get this |
|----------------------|-------------|
| `Athena` | Your memory & learning systems expert (✦ PRIMARY AGENT) |
| `Recommend an agent` | Find the perfect agent for your specific task |
| `Project Overview` | See what's in this repository right now |
| `Detailed Analysis` | Get comprehensive codebase documentation |

Simply type one of these phrases to instantly activate:
```
$ claude
> Project Overview
```

## Creating New Projects with CI

Create a new project with Collaborative Intelligence:
```
CI init my-project
cd my-project
claude
```

## Adding CI to Existing Projects

Add Collaborative Intelligence to any existing project:
```
cd existing-project
CI integrate
claude
```

You can also specify additional options:
```
CI integrate --agents=Athena,TheFixer,Optimizer
```

## Core Functions

- **Athena**: Knowledge architecture and learning systems
- **Agent Recommender**: Matches your task to the right expert
- **Project Overviewer**: Instant repository assessment
- **Code Cartographer**: Deep codebase analysis with documentation

## Key Features

- **Natural Commands**: Use human language phrases
- **Automatic Execution**: Overview runs immediately when requested
- **Memory Continuity**: Agents remember previous interactions
- **Session Organization**: Work stored in structured directories
- **Agent Identification**: All responses prefixed with [AGENT_NAME] format

## Need Help?

- Type `Recommend an agent` and describe your task
- Ask any agent "which agent should handle [task]?"
- Check agent Sessions directories for similar examples

## Common Commands

| Command | Function |
|---------|----------|
| `help` | Show available commands for current agent |
| `switch:[agent/command]` | Switch to a different agent |
| `agents:list` | Show all available agents |
| `memory:status` | Check what the current agent remembers |

## Repository Routing Prefixes

| Prefix | Function |
|---------|----------|
| `_` | Routes command to home repository (CollaborativeIntelligence/) |
| `-` | Routes command to current working directory |

Examples:
- `_Write this to NOTES.md` → Writes to CollaborativeIntelligence/NOTES.md
- `-Write this to README.md` → Writes to current project's README.md

---

*This guide updates automatically based on project context.*
*Last updated: 2025-05-14*

---

Athena: Updated QuickStart guide with more human-friendly command phrases and added instructions for the new 'CI integrate' command.