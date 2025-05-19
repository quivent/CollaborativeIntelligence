# Repository Restructuring Guide

This guide explains the new organizational structure for the CollaborativeIntelligence repository.

## Overview

The repository has been restructured to improve organization, readability, and maintainability. This restructuring:

- Maintains all existing functionality
- Preserves backward compatibility
- Creates a more logical directory structure
- Provides clearer navigation
- Separates core functionality from extensions

## New Directory Structure

```
CollaborativeIntelligence/
├── agents/                    # Agent definitions with categorization
│   ├── core/                  # Core system agents
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

## Transition Path

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

### Navigation Tips

- Use READMEs in each directory to understand its purpose
- Check the documentation index for comprehensive listing
- Follow cross-references between documents
- Use the new structure for locating related content

## For Agents

If you're an agent looking for information:

1. AGENTS directory remains in its original location
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

## Feedback and Questions

For questions about the repository restructuring, contact the Cartographer agent.

---

*Repository restructuring designed by Cartographer - Codebase Analysis Specialist*