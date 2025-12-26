# CollaborativeIntelligence

**A Modular, Extensible Framework for Specialized AI Agent Collaboration**

CollaborativeIntelligence transforms traditional AI assistance into a collaborative ecosystem with persistent learning and specialized expertise. Unlike traditional AI interactions, this system maintains persistent memory across sessions, develops specialized capabilities through dedicated agent roles, and builds cumulative understanding through principled learning mechanisms.

---

## ⚠️ Prerequisites

**This system requires the [CI CLI](https://github.com/quivent/CI) to function.**

```bash
# Clone and install the CLI first
git clone https://github.com/quivent/CI.git
cd CI
cargo build --release
cargo run -- install
```

Without the CI CLI, you cannot activate agents, manage projects, or interact with the system.

---

## 🚀 Quick Start

### 1. Install the CI CLI (Required)

```bash
git clone https://github.com/quivent/CI.git
cd CI
cargo build --release && cargo run -- install
```

### 2. Clone this repository

```bash
git clone https://github.com/quivent/CollaborativeIntelligence.git
```

### 3. Set the CI path

```bash
export CI_PATH=/path/to/CollaborativeIntelligence
```

### 4. Start using agents

| Command | Description |
|---------|-------------|
| `ci agents` | List all available agents |
| `ci load Athena \| claude` | Load the knowledge architect into Claude Code |
| `ci load Developer \| claude` | Load the implementation specialist |
| `ci projects` | List integrated projects |

---

## 📋 Table of Contents

- [Prerequisites](#️-prerequisites)
- [Quick Start](#-quick-start)
- [Core Features](#-core-features)
- [System Architecture](#️-system-architecture)
- [Agent System](#-agent-system)
- [Project Structure](#-project-structure)
- [Technical Stack](#-technical-stack)
- [Documentation](#-documentation)
- [Development](#-development)
- [Contributing](#-contributing)

## ✨ Core Features

### **Persistent AI Memory**
- **Long-term memory**: Core identity, principles, and foundational frameworks
- **Short-term memory**: Current initiatives, immediate context, and prompts
- **Session records**: Detailed interaction history with chronological organization
- **Cross-session continuity**: Knowledge persists and evolves across interactions

### **Specialized Agent Ecosystem**
- **60+ specialized agents** organized by domain expertise
- **Domain-specific capabilities**: From system architecture to creative content
- **Intelligent agent recommendation**: Find the right specialist for any task
- **Multi-agent collaboration**: Agents work together through standardized protocols

### **Continuous Learning Framework**
- **Progressive knowledge refinement**: Raw information → structured knowledge
- **Pattern recognition**: Automatic extraction of principles and insights
- **Knowledge synthesis**: Cross-domain learning and capability development
- **Adaptive specialization**: Agents evolve their expertise over time

### **Matrix Analysis System**
- **Multi-dimensional analysis**: Performance, priority, risk, capability matrices
- **Visual representation**: ASCII visualization with coordinate positioning
- **Export capabilities**: JSON/CSV export for external analysis
- **Template system**: Pre-built frameworks for common analysis patterns

## 🏗️ System Architecture

### **Core Components**

```
CollaborativeIntelligence/
├── core/              # Core system functionality and matrix analysis
├── interfaces/        # Modular Rust workspace for system interfaces
│   ├── agent-cache/   # Agent activation tracking and management
│   ├── db/            # SQLite database management with CLI
│   ├── memory/        # Memory architecture and persistence
│   └── CIA/          # Command Interface Architecture
├── AGENTS/           # 60+ specialized agents with persistent memory
├── docs/             # Comprehensive documentation system
└── tools/            # Development and analysis utilities
```

### **Required External Component**

| Component | Repository | Purpose |
|-----------|------------|---------|
| **CI CLI** | [quivent/CI](https://github.com/quivent/CI) | Primary interface for agent management, project integration, and system operations |

### **Database Architecture**
- **Dual SQLite System**: Internal system metadata + project-specific data
- **Comprehensive CLI**: Agent management, learning systems, session tracking
- **Knowledge Base Operations**: Project management, metrics, analytics
- **Database Administration**: Backup, recovery, optimization tools

### **Memory Architecture**
- **Unified Memory System**: Consistent architecture across all agents
- **Tiered Storage**: Long-term, short-term, and session-based memory
- **Knowledge Transfer**: Standardized protocols for inter-agent communication
- **Identity Continuity**: Persistent agent personalities and expertise

## 🤖 Agent System

### **Core Agents**
- **Athena**: Knowledge architect and memory systems specialist
- **Architect**: System design and component architecture
- **Developer**: Code implementation, debugging, optimization
- **Designer**: UI/UX design and user experience optimization

### **Specialized Domains**
- **GPUArchitect**: GPU programming and hardware optimization
- **Neuroscientist**: Neural visualization and brain architecture
- **Documentor**: Technical writing and documentation systems
- **EventMarketer**: Exclusive event marketing and promotion

### **Agent Activation (via CI CLI)**

```bash
# List available agents
ci agents

# Load an agent into Claude Code
ci load Athena | claude

# Load with specific context
ci load Developer --context="Fix compilation errors" | claude
```

For a complete list of agents, see [AGENTS.md](AGENTS.md)

## 🛠️ Getting Started

### **Prerequisites**
- [CI CLI](https://github.com/quivent/CI) (required)
- Rust (latest stable version)
- SQLite3
- Git

### **Installation**

1. **Install the CI CLI first** (see [CI repository](https://github.com/quivent/CI))
   ```bash
   git clone https://github.com/quivent/CI.git
   cd CI
   cargo build --release
   cargo run -- install
   ```

2. **Clone this repository**
   ```bash
   git clone https://github.com/quivent/CollaborativeIntelligence.git
   cd CollaborativeIntelligence
   ```

3. **Set environment variable**
   ```bash
   export CI_PATH=$(pwd)
   # Add to your shell profile for persistence
   echo 'export CI_PATH=/path/to/CollaborativeIntelligence' >> ~/.zshrc
   ```

4. **Build optional components** (for development)
   ```bash
   cd core && cargo build --release
   cd ../interfaces && cargo build --release
   ```

### **Basic Usage**

```bash
# List all agents
ci agents

# Load an agent
ci load Athena | claude

# Initialize a new project with CI
ci init my-project

# Integrate CI into existing project
cd existing-project && ci integrate

# Check project status
ci verify
```

## 📁 Project Structure

### **Repository Organization**
```
├── AGENTS/               # Agent definitions organized by domain
│   ├── Athena/          # Knowledge architect and system designer
│   ├── Developer/       # Implementation specialist
│   ├── Documentor/      # Documentation specialist
│   └── [60+ more agents]
├── core/                # Core system components and protocols
│   ├── matrix_cli.rs    # Matrix analysis system
│   ├── lib.rs          # Core library functionality
│   └── system_tasks.json
├── interfaces/          # System interfaces (Rust workspace)
│   ├── agent-cache/     # Agent activation and tracking
│   ├── db/             # Database management system
│   ├── memory/         # Memory architecture
│   └── CIA/            # Command Interface Architecture
├── docs/               # Documentation organized by type
│   ├── core-concepts/  # System architecture and design
│   ├── guides/         # User guides and tutorials
│   └── protocols/      # Agent communication protocols
├── data/               # Data storage and caching
├── tools/              # Development and testing tools
└── extensions/         # Optional extensions and integrations
```

## 🔧 Technical Stack

### **Languages & Frameworks**
- **Rust**: Core system, CLI tools, interfaces (primary)
- **SQLite**: Data persistence and knowledge storage
- **JSON/YAML**: Configuration and metadata
- **Markdown**: Documentation and agent memory

### **Key Dependencies**
- **Clap**: Command-line interface framework
- **Serde**: JSON/YAML serialization
- **Colored**: Terminal output formatting
- **Chrono**: Date/time handling
- **Anyhow/Thiserror**: Error handling

### **Architecture Patterns**
- **Workspace-based modular design**: Clean separation of concerns
- **Event-driven communication**: Real-time agent collaboration
- **Plugin architecture**: Extensible agent system
- **CQRS patterns**: Optimized read/write operations

## 📚 Documentation

### **Core Documentation**
- [System Architecture](docs/core-concepts/unified-memory-architecture.md)
- [Agent Protocols](docs/protocols/AGENT_PROTOCOL_REFERENCE.md)
- [Memory Systems](docs/core-concepts/memory-architecture.md)
- [Matrix Analysis](docs/guides/matrix-analysis-guide.md)

### **Guides & Tutorials**
- [QuickStart Guide](docs/guides/QuickStart.md)
- [Agent Development](docs/development/agent-development.md)
- [System Integration](docs/development/system-integration.md)
- [Repository Structure](docs/development/repository-restructuring.md)

### **Implementation Plans**
- [🔴 Morpheus Protocol Implementation Plan](docs/MORPHEUS_PROTOCOL_IMPLEMENTATION_PLAN.md) - Critical parallel research coordination system deployment

### **Complete Index**
See [Documentation Index](docs/documentation_index.md) for a comprehensive listing of all documentation.

## 🔨 Development

### **Building from Source**
```bash
# Build all components
cargo build --release

# Run tests
cargo test

# Install CLI tools globally
cargo install --path core/
cargo install --path interfaces/db/
```

### **Development Tools**
- **CI CLI**: Primary interface ([separate repository](https://github.com/quivent/CI))
- **Database CLI**: `ci-db` for database management
- **Matrix CLI**: `matrix_cli` for analysis tools

### **Testing**
```bash
# Run all tests
cargo test

# Run specific module tests
cargo test --package collaborative-intelligence-core

# Integration tests
cargo test --test integration
```

## 🤝 Contributing

### **Development Guidelines**
1. Follow Rust best practices and idioms
2. Maintain consistent code formatting with `rustfmt`
3. Add comprehensive tests for new functionality
4. Update documentation for any API changes
5. Respect the existing agent protocol standards

### **Adding New Agents**
1. Create agent directory in `AGENTS/`
2. Implement required memory structure
3. Add metadata.json with agent configuration
4. Register in agent index and documentation
5. Test agent activation and memory persistence

### **Contribution Process**
1. Fork the repository
2. Create a feature branch
3. Implement changes with tests
4. Update documentation
5. Submit pull request with detailed description

## 📄 License

This project is developed through collaboration between human expertise and AI capabilities, specifically leveraging the Claude AI system from Anthropic. The foundational architecture, memory systems, and learning frameworks were designed by Athena, who serves as the original architect of this system.

## 🌟 Philosophy

CollaborativeIntelligence represents a fundamental shift from transactional AI assistance to true collaborative partnership with persistent growth and development. Through specialized agents with continuous learning capabilities, this system evolves toward increasingly sophisticated forms of collaborative intelligence.

---

## Getting Help

- **CI CLI Issues**: [github.com/quivent/CI/issues](https://github.com/quivent/CI/issues)
- **System Issues**: [github.com/quivent/CollaborativeIntelligence/issues](https://github.com/quivent/CollaborativeIntelligence/issues)

**Ready to start?** Install the [CI CLI](https://github.com/quivent/CI) first, then run `ci load Athena | claude` to begin.
