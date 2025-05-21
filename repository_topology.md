# 🌳 CollaborativeIntelligence Repository Topology

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
│   ├── 🧭 Topologist/                 # Repository structure agent (CURRENT)
│   │   ├── 📝 CommitNotification.md   # Recently updated
│   │   ├── 📝 ContinuousLearning.md
│   │   ├── 📚 MEMORY.md
│   │   └── 📁 Sessions/
│   │       └── 📝 CLIReorganization/  # New session documenting this work
│   ├── 🔄 Refactorer/                 # Code refactoring agent
│   │   └── 📝 metadata.json           # Recently updated
│   └── ... (40+ other agents)
│
├── 📁 CLIs/                           # 🆕 New centralized CLI directory
│   ├── 📝 README.md                   # CLI overview documentation 
│   ├── 📝 MIGRATION_PLAN.md           # CLI consolidation strategy
│   ├── 📝 CHERRY_PICKING_GUIDE.md     # Code integration guidance
│   ├── 📝 CLI_SEPARATION.md           # External vs internal CLI plan
│   ├── 📝 NAMING_FIXES.md             # CIR to CI naming standards
│   │
│   ├── 📁 ci/                         # Main CI system
│   │   ├── 📁 bash/                   # Original implementation
│   │   │   ├── 📜 activate-agent.sh   # Moved from ci-tools/
│   │   │   ├── 📜 ci-init             # Moved from ci-tools/
│   │   │   ├── 📜 ci-fix              # New file
│   │   │   ├── 📜 ci-verify           # New file 
│   │   │   ├── 📁 cmd/
│   │   │   ├── 📁 lib/
│   │   │   └── 📁 test/
│   │   └── 📁 rust/                   # Rust implementation
│   │       ├── 📝 COMPARISON.md       # Compares implementation approaches
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
├── 📝 CLAUDE.md                       # Core configuration (recently updated)
├── 📝 .collaborative-intelligence.json # System config (recently updated)
├── 📝 .gitignore                      # Updated with new patterns
│
├── 📁 ci-tools/                       # ⚠️ Old CLI directory (files moved to CLIs/ci/bash/)
│
├── 📁 ci-rust-minimal/                # Minimal Rust CLI implementation
│   ├── 📝 COMPARISON.md
│   ├── 📦 Cargo.toml
│   └── 📁 src/
│
├── 📁 scripts/                        # Core system scripts
│   ├── 📁 knowledge/
│   ├── 📁 legacy/
│   ├── 📜 memory_cli.rs
│   ├── 📜 memory_loader.rs
│   └── ... (other scripts)
│
├── 📁 docs/                           # System documentation
│   ├── 📁 agent-system/
│   ├── 📁 core-concepts/
│   ├── 📁 protocols/
│   └── ... (other documentation)
│
├── 📁 .projects/                      # External projects with symbolic links
│   ├── 🔗 test-ci-project → /Users/joshkornreich/Documents/Projects/CI/test-ci-project
│   └── 🔗 unnamed_project → /Users/joshkornreich/Documents/Projects/CI
│
└── ... (other system components)
```

## 🎨 Legend

- 📁 Blue - Directories
- 📝 Green - Documentation files
- 📜 Yellow - Script files
- 📦 Purple - Package configuration files
- 🔗 Cyan - Symbolic links
- 🆕 Red - New components
- ⚠️ Orange - Deprecated components

## 🔄 Current Work Context

The repository is undergoing a significant restructuring of CLI tools with:

1. **Centralization**: Moving tools from scattered locations into `CLIs/` directory
2. **Organization**: Creating a logical component structure (ci, memory, agent-cache, etc.)
3. **Documentation**: Adding migration plans and implementation guides
4. **Naming**: Standardizing on "CI" instead of "CIR" throughout the codebase

## 📋 Next Steps in CLI Migration Plan

1. **Phase 1** ✓ (in progress): Directory structure reorganization
2. **Phase 2**: Core functionality alignment
3. **Phase 3**: Cherry-picking advanced features
4. **Phase 4**: Complete Rust migration
5. **Phase 5**: Cleanup and deprecation

## 🏢 Architecture Overview

The system is being structured with separation between:

- **External CLI (CI)**: For integration with other projects (planned to live in `../CI`)
- **Internal CLI (TL)**: For maintenance of this repository (in `CLIs/`)

The repository is now configured in standalone mode, which means it doesn't depend on
an external CollaborativeIntelligence repository.