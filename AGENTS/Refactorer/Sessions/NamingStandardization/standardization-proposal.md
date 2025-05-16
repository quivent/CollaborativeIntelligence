# Agent File Structure Standardization Proposal

## Executive Summary
This proposal outlines a comprehensive standardization approach for the AGENTS directory structure, focusing on consistent naming conventions, file organization, and content structure across all agents.

## Current State Analysis

### Issues Identified
1. **Memory File Inconsistencies**
   - Multiple naming patterns: `MEMORY.md`, `{Agent}_memory.md`, `The{Agent}_memory.md`
   - Duplicate files in same directory
   - Lingering backup files (`.bak`)

2. **Agent Name Legacy Issues**
   - Remnants from previous naming conventions
   - Incomplete cleanup after agent renames
   - Inconsistent references across system

3. **Structure Variations**
   - Not all agents have ContinuousLearning.md
   - Sessions directory organization varies
   - Missing standardized documentation

## Proposed Standards

### 1. Directory Structure
```
AGENTS/
└── {AgentName}/
    ├── README.md              # Agent overview and capabilities
    ├── MEMORY.md              # Memory architecture (single file)
    ├── ContinuousLearning.md  # Extracted learnings
    └── Sessions/              # Work session records
        ├── README.md          # Session index and guidelines
        └── {SessionName}/     # Individual session folders
            ├── README.md      # Session summary
            └── metadata.json  # Session metadata
```

### 2. File Naming Conventions
- **Memory Files**: Single `MEMORY.md` per agent (uppercase)
- **No Prefixes**: Remove agent name prefixes from memory files
- **No Backups**: Clean `.bak` files regularly
- **Consistent Capitalization**: Use standard file names across all agents

### 3. Content Structure Standards

#### MEMORY.md Format
```markdown
# {AgentName} Memory Architecture

## Long-Term Memory: Core Identity

### Foundational Purpose
[Core purpose and role description]

### Guiding Principles
[Key principles that guide the agent]

### Core Frameworks
[Established methodologies and approaches]

## Short-Term Memory: Current State

### Active Context
[Current focus and activities]

### Recent Insights
[Latest learnings and observations]

### Operational Status
[Current capabilities and limitations]
```

#### README.md Format
```markdown
# {AgentName}

## Core Identity
[Brief description of agent's role and purpose]

## Primary Responsibilities
[List of key responsibilities]

## Expertise Areas
[Domains of specialization]

## Collaboration Guidelines
[How to work with this agent]

## Activation Protocol
[How to invoke this agent]
```

### 4. Implementation Phases

#### Phase 1: Critical Cleanup (Immediate)
- Remove duplicate memory files
- Delete `.bak` files
- Fix legacy naming remnants

#### Phase 2: Structure Standardization (Short-term)
- Ensure all agents have required files
- Standardize content formats
- Update templates in Streamliner

#### Phase 3: Validation & Enforcement (Long-term)
- Create validation scripts
- Add pre-commit hooks
- Update contribution guidelines

## Benefits
1. **Consistency**: Uniform structure across all agents
2. **Maintainability**: Easier to update and manage
3. **Discoverability**: Clear, predictable file locations
4. **Integration**: Smoother agent interactions
5. **Onboarding**: Clearer for new contributors

## Risk Mitigation
- Create backups before major changes
- Test changes on subset of agents first
- Document all changes made
- Ensure no functionality is lost

## Success Metrics
- 100% agents follow standard structure
- No duplicate memory files
- All templates updated
- Validation scripts in place
- Documentation updated

## Timeline
- Week 1: Critical cleanup
- Week 2: Structure standardization
- Week 3: Validation implementation
- Week 4: Documentation and training

## Approval Required From
- System architects
- Active contributors
- Repository maintainers

---
*Prepared by Refactorer*
*Date: 2025-05-16*