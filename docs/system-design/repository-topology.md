# CollaborativeIntelligence Repository Topology

## Overview

This document provides a detailed map of the CollaborativeIntelligence repository structure, illustrating the organization of directories, files, and key components. It serves as a reference for understanding the repository layout and ongoing structural improvements.

## Repository Structure

```
CollaborativeIntelligence/
├── 📁 AGENTS/                         # Agent system components
│   ├── 🧠 Athena/                     # Primary intelligence agent
│   │   ├── 📝 ContinuousLearning.md
│   │   ├── 📚 MEMORY.md
│   │   └── 📁 Sessions/
│   ├── 🏛️ Architect/                  # System architecture agent
│   │   └── 📁 Sessions/
│   ├── 🛠️ Developer/                  # Development agent
│   ├── 🧭 Topologist/                 # Repository structure agent
│   │   ├── 📝 CommitNotification.md
│   │   ├── 📝 ContinuousLearning.md
│   │   ├── 📚 MEMORY.md
│   │   └── 📁 Sessions/
│   │       └── 📝 CLIReorganization/
│   ├── 🔄 Refactorer/                 # Code refactoring agent
│   │   └── 📝 metadata.json
│   └── ... (40+ other agents)
│
├── 📁 CLIs/                           # Centralized CLI directory
│   ├── 📝 README.md                   # CLI overview documentation 
│   ├── 📝 MIGRATION_PLAN.md           # CLI consolidation strategy
│   ├── 📝 CHERRY_PICKING_GUIDE.md     # Code integration guidance
│   ├── 📝 CLI_SEPARATION.md           # External vs internal CLI plan
│   ├── 📝 NAMING_FIXES.md             # CIR to CI naming standards
│   │
│   ├── 📁 ci/                         # Main CI system
│   │   ├── 📁 bash/                   # Original implementation
│   │   │   ├── 📜 activate-agent.sh
│   │   │   ├── 📜 ci-init
│   │   │   ├── 📜 ci-fix
│   │   │   ├── 📜 ci-verify
│   │   │   ├── 📁 cmd/
│   │   │   ├── 📁 lib/
│   │   │   └── 📁 test/
│   │   └── 📁 rust/                   # Rust implementation
│   │       ├── 📝 COMPARISON.md
│   │       ├── 📦 Cargo.toml
│   │       ├── 📁 src/
│   │       └── 📁 legacy/
│   │
│   ├── 📁 memory/                     # Memory management CLI
│   │   ├── 📜 build_memory_cli.sh
│   │   ├── 📁 schemas/
│   │   └── 📁 src/
│   │       ├── 📜 memory_cli.rs
│   │       ├── 📜 memory_checksum_cache.rs
│   │       ├── 📜 memory_loader.rs
│   │       └── ... (other memory components)
│   │
│   ├── 📁 agent-cache/                # Agent caching system
│   │   └── 📁 src/
│   │       └── 📜 main.rs
│   │
│   ├── 📁 knowledge/                  # Knowledge registry CLI
│   │   └── 📁 src/
│   │       └── 📜 search_cli.sh
│   │
│   ├── 📁 plugins/                    # CLI plugins
│   │   └── 📁 claude/                 # Claude CLI integration
│   │       ├── 📜 claude-init-impl.sh
│   │       └── 📜 install.sh
│   │
│   └── 📁 docs/                       # CLI documentation
│       ├── 📁 usage/
│       ├── 📁 development/
│       └── 📁 examples/
│
├── 📝 CLAUDE.md                       # Core configuration
├── 📝 .collaborative-intelligence.json # System config
├── 📝 .gitignore
│
├── 📁 docs/                           # System documentation
│   ├── 📁 agent-system/
│   ├── 📁 core-concepts/
│   ├── 📁 protocols/
│   └── ... (other documentation)
│
├── 📁 scripts/                        # Core system scripts
│   ├── 📁 knowledge/
│   ├── 📁 legacy/
│   ├── 📜 memory_cli.rs
│   ├── 📜 memory_loader.rs
│   └── ... (other scripts)
│
└── ... (other system components)
```

## Legend

- 📁 Directories
- 📝 Documentation files
- 📜 Script files
- 📦 Package configuration files
- 🔗 Symbolic links

## Architecture Overview

The system is structured with separation between:

- **External CLI (CI)**: For integration with other projects (planned to live in `../CI`)
- **Internal CLI (TL)**: For maintenance of this repository (in `CLIs/`)

The repository is configured in standalone mode, which means it doesn't depend on an external CollaborativeIntelligence repository.

## CLI Migration Plan

The repository is undergoing a significant restructuring of CLI tools with:

1. **Phase 1**: Directory structure reorganization
   - Centralizing tools in the `CLIs/` directory
   - Creating a logical component structure

2. **Phase 2**: Core functionality alignment
   - Standardizing interfaces
   - Ensuring consistent behavior

3. **Phase 3**: Cherry-picking advanced features
   - Integrating best features from all implementations
   - Ensuring backward compatibility

4. **Phase 4**: Complete Rust migration
   - Moving from Bash to Rust implementation
   - Performance and maintainability improvements

5. **Phase 5**: Cleanup and deprecation
   - Removing deprecated components
   - Finalizing documentation

## Directory Structure Standards

This repository follows these directory naming conventions:

1. **Agent Directories**: PascalCase (e.g., `Athena/`, `Topologist/`)
2. **System Directories**: lowercase (e.g., `docs/`, `scripts/`)
3. **Component Directories**: kebab-case (e.g., `agent-cache/`)

## Integration Points

The repository topology is designed to facilitate:

1. **Agent Collaboration**: Structured agent directories with standard files
2. **CLI Tools**: Centralized in the `CLIs/` directory
3. **Documentation**: Centralized in the `docs/` directory
4. **Scripts**: Core system scripts in the `scripts/` directory

## Version History

| Version | Date | Description | Author |
|---------|------|-------------|--------|
| 1.0 | 2025-05-21 | Initial repository topology documentation | Topologist |

---

*Originally created as repository_topology.md by Topologist - Repository Structure and Version Control Specialist*