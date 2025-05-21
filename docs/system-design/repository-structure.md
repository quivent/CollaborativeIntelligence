# Repository Structure Guide

## Overview

This document provides a comprehensive guide to the standardized repository structure for the CollaborativeIntelligence system. It serves as the central reference for all agents regarding the organization, navigation, and maintenance of the repository.

## Standardized Directory Structure

```
CollaborativeIntelligence/
├── agents/                    # Agent definitions with categorization
│   ├── core/                  # Core system agents (Athena, Gaia, etc.)
│   ├── development/           # Engineering-focused agents
│   ├── knowledge/             # Memory and knowledge agents
│   └── specialized/           # Task-specific agents
├── cli/                       # Command-line tools
│   ├── ci-tools/              # Main CLI implementation
│   ├── plugins/               # CLI plugins
│   └── scripts/               # Utility scripts
├── core/                      # Core system components
│   ├── memory/                # Memory system implementation
│   ├── protocols/             # Communication protocols
│   └── architecture/          # System architecture
├── docs/                      # All documentation
│   ├── guides/                # User guides and tutorials
│   ├── architecture/          # Architecture documentation
│   ├── protocols/             # Protocol specifications
│   └── api/                   # API documentation
├── data/                      # Data storage
│   ├── cache/                 # System caches
│   ├── sessions/              # Session records
│   └── benchmarks/            # Performance benchmarks
├── examples/                  # Example implementations
│   ├── conversations/         # Example conversations
│   └── projects/              # Example projects
├── tools/                     # Development tools
│   ├── testing/               # Testing tools
│   └── development/           # Development utilities
├── extensions/                # Optional extensions
│   ├── kanji/                 # Kanji project
│   └── tauri/                 # Desktop application integration
```

## Current Issues and Restructuring Goals

### Current Issues

1. **Inconsistent Directory Naming**: Mix of uppercase (e.g., `AGENTS`, `CACHE`) and lowercase (e.g., `scripts`, `agent_cache`) directories
2. **Scattered Documentation**: Documentation files spread across multiple locations
3. **Unclear Resource Organization**: Implementation files, tools, and configuration mixed at the root level
4. **Inconsistent Casing in Directories**: `BENCHMARKS` vs `Benchmarks`, `CACHE` vs `cache`
5. **Ambiguous Category Boundaries**: Overlap between `Architectures`, `Plans`, and `Documentation` directories
6. **Orphaned Files**: Several files at the root level that belong in specific directories

### Restructuring Goals

1. **Standardize Naming Conventions**: Adopt consistent casing for all directories
2. **Centralize Similar Resources**: Group related files and directories 
3. **Improve Navigation**: Create logical hierarchy for easier discovery
4. **Reduce Root Clutter**: Move files from root to appropriate subdirectories
5. **Simplify Maintenance**: Organize related components close together
6. **Enhance Discoverability**: Make it easier to find specific resources

## Implementation Plan

The restructuring is implemented in five phases:

### Phase 1: Directory Standardization
Creates the new standardized directory structure with proper naming conventions.

### Phase 2: Resource Consolidation
Moves existing content to the new directory structure while preserving all functionality.

### Phase 3: Project Organization
Organizes project files, creates README files, and updates references.

### Phase 4: Special Resources
Handles special cases, updates path references, and creates additional documentation.

### Phase 5: Validation
Verifies the restructured repository for completeness and correctness.

## Directory Mapping

### Current → New Structure

```
CollaborativeIntelligence/
├── AGENTS/                       → agents/
├── BENCHMARKS/                   → data/benchmarks/
├── CACHE/                        → data/cache/
├── Architectures/                → docs/architecture/
├── Documentation/                → docs/
├── KnowledgeRegistry/            → knowledge-registry/
├── MARKDOWN_DESIGNS/             → tools/markdown-designs/
├── OPTIMIZATIONS/                → docs/optimizations/
├── Plans/                        → docs/plans/
├── Projects/                     → projects/
├── Protocols/                    → protocols/
├── SPECS/                        → docs/specifications/
├── SYSTEMS/                      → systems/
├── Sessions/                     → data/sessions/
├── Terminal/                     → cli/terminal/
├── Visions/                      → docs/visions/
├── agent_cache/                  → cli/agent-cache/
├── ci-rust-minimal/              → cli/minimal/
├── ci-tools/                     → cli/tools/
├── scripts/                      → cli/scripts/
└── various root files            → appropriate subdirectories
```

## Implementation Scripts

Implementation scripts are available in the Cartographer session directory. These scripts should be executed in sequence to perform the restructuring:

- `execute-restructuring.sh` - Master execution script
- `phase1-directory-standardization.sh` - Creates new directory structure
- `phase2-resource-consolidation.sh` - Moves content to new structure
- `phase3-project-organization.sh` - Organizes project files
- `phase4-special-resources.sh` - Handles special cases
- `phase5-validation.sh` - Validates restructuring

## Navigation Guidelines

### Backward Compatibility

During transition, critical files are available in both locations:
- CLAUDE.md remains at the root and in `/docs/`
- Protocol documentation is in original location and `/docs/protocols/`
- Agent Index is referenced from original location and in `/core/`

### Finding Documentation

1. **Documentation Index**: See `/docs/documentation_index.md` for a complete listing of all documentation
2. **README Files**: Each directory contains a README explaining its purpose and contents
3. **Agent Categories**: See `/core/agent_categories.md` for the agent categorization system
4. **Markdown Standards**: See `/docs/development/markdown-standards.md` for documentation organization rules

### Tips for Agents

If you're an agent looking for information:

1. AGENTS directory remains in its original location during transition
2. Critical files like CLAUDE.md are available in both original and new locations
3. Check `/core/agent_categories.md` for new agent categorization
4. Protocol documentation is available at both original and `/docs/protocols/` paths
5. READMEs in each directory explain the new organization

## Benefits of New Structure

- **Cleaner Organization**: Logical grouping by function and purpose
- **Better Discoverability**: Clear category separation and README files
- **Improved Maintainability**: Related components kept together
- **Separation of Concerns**: Core system separated from extensions
- **Enhanced Navigation**: Clear structure and documentation index
- **Future Extensibility**: Room for growth in each category

## Implementation Timeline

1. **Day 1**: Directory structure standardization (Phase 1)
2. **Day 2**: Code implementation reorganization (Phase 2-1)
3. **Day 3**: Documentation and configuration reorganization (Phase 2-2 & 2-3)
4. **Day 4**: Tools and scripts consolidation (Phase 2-4)
5. **Day 5**: Project organization and special resources (Phase 3 & 4)
6. **Day 6**: Validation and finalization

## Risk Management

Potential risks and mitigation strategies have been documented:

1. **Code Imports and References**: Automated path updates and manual review
2. **Build and CI Systems**: Update CI configuration files and verify build processes
3. **Documentation Accuracy**: Automated scanning and README updates
4. **Agent System Knowledge**: Update agent memory files
5. **End User Experience**: Update user documentation

## Testing Approach

A comprehensive testing plan has been created with these phases:

1. **File Integrity Tests**: Ensure all files are present in the new structure
2. **Reference Validation**: Verify references are updated correctly
3. **Functionality Tests**: Confirm system continues to work
4. **Integration Tests**: Validate the entire system works together

## Agent Directory Guidelines

Every agent in the CollaborativeIntelligence system must follow these directory structure guidelines:

```
AGENTS/
└── AgentName/
    ├── README.md              # Core identity and capabilities
    ├── MEMORY.md              # Long-term and short-term memory
    ├── ContinuousLearning.md  # Accumulated knowledge and insights
    ├── Sessions/              # Records of interaction sessions
    │   └── SessionName/       # Individual session directories
    │       ├── README.md      # Session summary and outcomes
    │       └── metadata.json  # Session metadata
    └── Toolkit/               # Optional specialized tools (if applicable)
```

### Agent File Naming Conventions

- **Agent Directories**: Use PascalCase (e.g., `DatabaseAgent`, `UIDesigner`)
- **Session Directories**: Use kebab-case (e.g., `repository-restructuring`, `memory-optimization`)
- **Core Files**: Use UPPERCASE.md (e.g., `README.md`, `MEMORY.md`)
- **Implementation Files**: Use kebab-case.md (e.g., `implementation-plan.md`)
- **Report Files**: Use UPPERCASE_REPORT.md (e.g., `FINAL_REPORT.md`)

### Agent Onboarding

New agents should:

1. Review the complete agent onboarding guide at `/docs/agent-system/agent-onboarding-guide.md`
2. Understand the overall repository structure outlined in this document
3. Follow the standardized directory structure and naming conventions
4. Contribute to maintaining consistent organization across the system

## Support

For questions or support regarding the repository restructuring, contact the Cartographer agent.

---

*This document serves as the centralized reference for the repository structure. All agents should refer to this guide for repository organization and navigation. For detailed agent onboarding information, see the agent onboarding guide.*