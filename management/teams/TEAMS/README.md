# CollaborativeIntelligence Teams Framework

## Overview

The TEAMS directory contains project-based team configurations where agents work together with context-specific roles, responsibilities, and shared project knowledge. Unlike the base AGENTS directory which contains individual agent capabilities, TEAMS captures collaborative working relationships and project-specific contexts.

## Team Structure Philosophy

### Base Agent + Project Context = Team Member
Each team member consists of:
- **Base Agent Personality**: Core capabilities and expertise from the AGENTS directory
- **Project Context**: Role within the specific team, current tasks, team relationships
- **Shared Knowledge**: Project-specific information, decisions, and working memory
- **Team Dynamics**: Communication patterns and collaboration history with other team members

## Directory Organization

```
TEAMS/
├── README.md (this file)
├── TEMPLATES/
│   ├── PROJECT_TEMPLATE/
│   ├── TEAM_MEMBER_TEMPLATE.md
│   └── TEAM_MANAGEMENT_TEMPLATE.md
├── ACTIVE_PROJECTS/
│   └── [Individual project directories]
├── ARCHIVED_PROJECTS/
│   └── [Completed project directories]
└── TEAM_MANAGEMENT/
    ├── TEAM_STANDARDS.md
    ├── PROJECT_LIFECYCLE.md
    └── PERFORMANCE_METRICS.md
```

## Project Team Categories

### 1. **Active Development Teams**
- Currently executing projects
- Real-time coordination and progress tracking
- Dynamic team composition based on project needs

### 2. **Research & Innovation Teams**
- Experimental projects and proof-of-concepts
- Cross-disciplinary collaboration
- Knowledge discovery and advancement

### 3. **Operational Teams**
- Ongoing system maintenance and support
- Process optimization and automation
- Quality assurance and monitoring

### 4. **Strategic Teams**
- Long-term planning and architecture
- Technology evaluation and adoption
- Business development and partnerships

## Team Formation Process

1. **Project Definition**: Establish project scope, objectives, and requirements
2. **Agent Selection**: Choose agents based on required expertise and availability
3. **Context Creation**: Develop project-specific context for each team member
4. **Team Initialization**: Establish communication protocols and working relationships
5. **Project Execution**: Coordinate team activities and track progress
6. **Knowledge Capture**: Document learnings and outcomes for future teams

## Integration with Base Agents

Team configurations extend base agent capabilities without modifying the core agent definitions. This allows:
- **Parallel Team Membership**: Agents can participate in multiple teams simultaneously
- **Context Switching**: Agents adapt behavior based on current team context
- **Knowledge Transfer**: Learnings from team projects enhance base agent capabilities
- **Scalable Organization**: Teams can be formed, modified, and dissolved dynamically

## Quality Standards

All team configurations must include:
- Clear project objectives and success criteria
- Defined roles and responsibilities for each team member
- Communication protocols and coordination mechanisms
- Progress tracking and performance metrics
- Knowledge management and documentation standards
- Risk management and issue escalation procedures

---

**Created**: 2025-08-10  
**Manager**: Manager Agent  
**Purpose**: Establish project-based team management framework for CollaborativeIntelligence system