# Project Initializer for Collaborative Intelligence

This document outlines the Project Initializer system, which automates the integration of the Collaborative Intelligence framework into new or existing projects.

## Overview

The Project Initializer provides a standardized way to:
1. Set up the Collaborative Intelligence directory structure
2. Configure agent activation and optimization
3. Establish proper repository relationships
4. Enable fast startup and activation

## Usage

### Initialize a New Project

```bash
# Create basic project with Collaborative Intelligence
mkdir NewProject
cd NewProject
project-init --with-collaborative-intelligence

# Create project with specific configuration
project-init --with-collaborative-intelligence \
  --integration-type=sibling \
  --path=../CollaborativeIntelligence \
  --fast-activation \
  --agents=Athena,Gaia,ProjectArchitect
```

### Add to Existing Project

```bash
# Add Collaborative Intelligence to existing project
cd ExistingProject
project-init --add-collaborative-intelligence

# Add with specific configuration
project-init --add-collaborative-intelligence \
  --integration-type=symlink \
  --path=/path/to/CollaborativeIntelligence \
  --fast-activation
```

## Configuration Options

| Option | Description | Default |
|--------|-------------|---------|
| `--integration-type` | How to integrate (embedded, sibling, symlink) | embedded |
| `--path` | Path to Collaborative Intelligence repo | ./CollaborativeIntelligence |
| `--fast-activation` | Enable Fast Activation System | false |
| `--agents` | Comma-separated list of agents to include | Athena,Master |
| `--preload` | Preload agent context on startup | false |

## Implementation

The Project Initializer:

1. Creates necessary directory structures:
   ```
   ProjectRoot/
   ├── .collaborative-intelligence.json
   ├── CLAUDE.md (with CI integration)
   └── CollaborativeIntelligence/ (or symlink)
   ```

2. Sets up the `.collaborative-intelligence.json` configuration:
   ```json
   {
     "integration_type": "embedded",
     "repository_path": "./CollaborativeIntelligence",
     "active_agents": ["Athena", "Gaia", "ProjectArchitect"],
     "fast_activation": true,
     "preload_context": false
   }
   ```

3. Updates the project's CLAUDE.md with agent activation instructions

4. Creates necessary Claude Code startup configurations

## Repository Integration Models

The initializer supports three integration models:

### 1. Embedded Integration
The Collaborative Intelligence system is embedded directly within the project repository. This approach works best for single projects with tight coupling.

```
ProjectRoot/
└── CollaborativeIntelligence/
    └── AGENTS/
        └── Athena/
```

### 2. Sibling Repository
The Collaborative Intelligence system is maintained as a separate repository and cloned alongside the project.

```
Parent/
├── ProjectRoot/
│   └── [symlink] CollaborativeIntelligence → ../CollaborativeIntelligence
└── CollaborativeIntelligence/
    └── AGENTS/
        └── Athena/
```

### 3. Symlink Configuration
A single instance of the Collaborative Intelligence system is shared across multiple projects through symbolic links.

```
Projects/
├── Project1/
│   └── [symlink] CollaborativeIntelligence → /path/to/CollaborativeIntelligence
├── Project2/
│   └── [symlink] CollaborativeIntelligence → /path/to/CollaborativeIntelligence
└── CollaborativeIntelligence/
    └── AGENTS/
        └── Athena/
```

## Implementation Notes

The Project Initializer is implemented as:

1. A standalone CLI tool
2. A set of shell scripts in the CollaborativeIntelligence repository
3. A command handled by the ProjectArchitect agent

Each approach follows the same core algorithms and produces compatible results.

## Future Enhancements

Planned enhancements include:
- Template-based project creation
- Custom agent selection and configuration
- Integration with CI/CD pipelines
- Automatic updates to shared Collaborative Intelligence repositories
- Configuration version control
- Multi-project synchronization