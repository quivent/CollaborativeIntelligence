# Agent Guide System

The Agent Guide helps you find the right specialized agent for your specific task.

## How It Works

When you type `AgentGuide` in the terminal, you'll receive:

1. A brief questionnaire about your current task
2. Personalized agent recommendations based on your answers
3. One-click activation links for recommended agents
4. Relevant examples from previous sessions

## Core Agents

### Primary System Agents

| Agent | Specialty | Best For |
|-------|-----------|----------|
| **Athena** | Memory architecture & learning systems | Knowledge organization, system design |
| **Gaia** | Integration & environmental awareness | Holistic project understanding |
| **Hermes** | Communication & information flow | Message systems, data transfer |

### Project Development Agents

| Agent | Specialty | Best For |
|-------|-----------|----------|
| **Architect** | System design & component relationships | Technical specifications, architecture |
| **Debugger** | Error identification & resolution | Crash fixes, performance issues |
| **UI_UX_Specialist** | Interface design & interaction | Visual refinement, user experience |
| **TaskPlanner** | Feature implementation & requirements | Planning, work breakdown |
| **RepositoryTopologist** | Source control & branch strategy | Git operations, commit management |

## Agent Matching

The Agent Guide matches your task characteristics to agent specialties:

- **Problem Type**: Bug, feature, refactor, documentation
- **Domain Area**: UI, data, performance, architecture
- **Task Scope**: Quick fix, medium change, major feature
- **Project Context**: Current priorities, recent work

## Example Usage

```
$ claude
> AgentGuide

What are you working on today?
> Fixing a UI freeze in the DataExplorer component

Based on your task, I recommend:
1. Debugger (95% match) - Expert at thread handling & UI issues
2. UI_UX_Specialist (80% match) - Familiar with DataExplorer component
3. Architect (70% match) - Can address structural performance issues

[Activate Debugger]  [Activate UI_UX_Specialist]  [Activate Architect]

Related sessions:
- Debugger/Sessions/DataExplorerThreadingFix/
- UI_UX_Specialist/Sessions/DataExplorerUIDebugging/
```

## Command Reference

| Command | Function |
|---------|----------|
| `AgentGuide` | Start interactive agent recommendation |
| `AgentGuide:task "description"` | Quick recommendation for specific task |
| `AgentGuide:list` | Show all agents with detailed capabilities |
| `AgentGuide:recent` | Show recently active agents and their work |

---

*The Agent Guide system continuously learns from project context and task outcomes to improve recommendations.*