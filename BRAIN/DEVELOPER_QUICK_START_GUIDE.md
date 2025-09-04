# CollaborativeIntelligence: Developer Quick Start Guide
**Get Up and Running with the Multi-Agent Collaborative Intelligence Platform in 30 Minutes**

## 🚀 Quick Start Overview

This guide will get you from zero to a fully functional CollaborativeIntelligence development environment in 30 minutes. You'll have agents running, coordination systems working, and be ready to start developing or using the platform.

**What You'll Accomplish:**
- ✅ Complete system installation and configuration
- ✅ Agent system activated with multiple agents
- ✅ CLI tools fully functional
- ✅ Coordination system operational
- ✅ BRAIN knowledge system accessible
- ✅ First multi-agent workflow executed

## 📋 Prerequisites Check

Before starting, ensure you have these tools installed:

### Required Tools
```bash
# Check if tools are installed
rust --version    # Should be 1.70+ 
sqlite3 --version # Should be 3.35+
git --version     # Should be 2.30+
python3 --version # Should be 3.8+
```

### Installation Commands (if needed)
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install SQLite3 (macOS)
brew install sqlite3

# Install SQLite3 (Ubuntu/Debian)
sudo apt-get install sqlite3 libsqlite3-dev

# Install Python 3 (if not available)
# macOS: brew install python3
# Ubuntu: sudo apt-get install python3 python3-pip
```

## ⚡ 30-Minute Setup

### Step 1: Repository Setup (5 minutes)

```bash
# Clone the repository
cd ~/Development  # or your preferred directory
git clone https://github.com/username/CollaborativeIntelligence.git
cd CollaborativeIntelligence

# Set up environment variables
export CI_REPO_PATH="$(pwd)"
export CI_DB_PATH="$(pwd)/db"

# Add to your shell profile for persistence
echo "export CI_REPO_PATH=\"$(pwd)\"" >> ~/.bashrc
echo "export CI_DB_PATH=\"$(pwd)/db\"" >> ~/.bashrc

# Source the profile (or restart terminal)
source ~/.bashrc
```

### Step 2: Core System Installation (10 minutes)

```bash
# Build and install CLI system
cd cli
cargo build --release

# Install CLI globally
cargo install --path .

# Verify installation
ci --version
# Should output: CI - Collaborative Intelligence CLI v1.0

# Test basic functionality
ci agents
# Should list available agents

# Build database management system
cd ../interfaces/db
cargo build --release
cargo install --path .

# Initialize database
ci-db init
# Should create database and show success message
```

### Step 3: System Verification (5 minutes)

```bash
# Check system status
ci status
# Should show system is operational

# Check database status
ci-db status
# Should show database is initialized and healthy

# List available agents
ci agents --details
# Should show detailed agent information

# Test agent loading
ci load Athena
# Should output agent memory and configuration
```

### Step 4: Coordination System Setup (5 minutes)

```bash
# Navigate to coordination system
cd ../../BRAIN/coordination

# Make scripts executable
chmod +x quick_start.sh

# Initialize coordination system
./quick_start.sh init
# Should create coordination files and show success

# Test coordination system
./quick_start.sh status
# Should show coordination system status
```

### Step 5: First Multi-Agent Workflow (5 minutes)

```bash
# Start a development session
./quick_start.sh start development "Architect,Developer,Tester"

# Check session status
./quick_start.sh status
# Should show active session with assigned lane

# Test task coordination (optional)
python3 task_checkin_checkout.py available
# Should show available tasks

# End session
./quick_start.sh end
```

## 🧪 Verification Tests

Run these tests to ensure everything is working correctly:

### Basic System Tests
```bash
# Test 1: Agent system functionality
ci agents --stats
# Should show agent usage statistics

# Test 2: Database operations
ci-db agents list
ci-db sessions list
# Should show database contents

# Test 3: BRAIN knowledge access
ci query "memory architecture"
# Should search BRAIN knowledge base

# Test 4: Coordination system
cd BRAIN/coordination
./quick_start.sh status
python3 session_coordinator.py status --system
# Should show coordination system health
```

### Advanced Integration Tests
```bash
# Test 5: Multi-agent loading
ci load "Architect,Developer,Designer"
# Should load multiple agents simultaneously

# Test 6: Session management
python3 session_coordinator.py register --lane testing
python3 session_coordinator.py status --active
python3 session_coordinator.py heartbeat --session $SESSION_ID
# Should manage session lifecycle

# Test 7: Task coordination
python3 task_checkin_checkout.py status --system
# Should show task system status
```

## 🔧 Development Environment Setup

### IDE Configuration

#### VS Code Setup
```bash
# Install Rust extension
code --install-extension rust-lang.rust-analyzer

# Create workspace settings
mkdir .vscode
cat > .vscode/settings.json << EOF
{
    "rust-analyzer.cargo.allFeatures": true,
    "rust-analyzer.checkOnSave.command": "clippy",
    "files.watcherExclude": {
        "**/target/**": true
    }
}
EOF
```

#### Environment Variables
```bash
# Development environment variables
cat >> .env << EOF
CI_REPO_PATH=$(pwd)
CI_DB_PATH=$(pwd)/db
CI_DEBUG=true
CI_LOG_LEVEL=debug
RUST_LOG=debug
BRAIN_SESSION_ID=dev-session-$(date +%s)
EOF

# Load environment
source .env
```

### Development Tools Setup

#### Pre-commit Hooks
```bash
# Install pre-commit tools
pip3 install pre-commit

# Set up hooks
cat > .pre-commit-config.yaml << EOF
repos:
  - repo: https://github.com/psf/black
    rev: 22.3.0
    hooks:
      - id: black
        language_version: python3
  - repo: local
    hooks:
      - id: cargo-fmt
        name: cargo fmt
        entry: cargo fmt
        language: system
        types: [rust]
      - id: cargo-clippy
        name: cargo clippy
        entry: cargo clippy -- -D warnings
        language: system
        types: [rust]
EOF

pre-commit install
```

#### Debugging Setup
```bash
# Install debugging tools
cargo install cargo-watch
cargo install cargo-expand

# Set up debug configuration
mkdir -p .vscode
cat > .vscode/launch.json << EOF
{
    "version": "0.2.0",
    "configurations": [
        {
            "type": "lldb",
            "request": "launch",
            "name": "Debug CI",
            "program": "\${workspaceFolder}/target/debug/ci",
            "args": ["agents"],
            "cwd": "\${workspaceFolder}"
        }
    ]
}
EOF
```

## 👥 Team Development Setup

### Multi-Developer Configuration

#### Setting Up Development Lanes
```bash
# Initialize development lanes for team
cd BRAIN/coordination

# Create team-specific lanes
python3 lane_manager.py create --lane team-alpha --description "Alpha team development"
python3 lane_manager.py create --lane team-beta --description "Beta team development"
python3 lane_manager.py create --lane integration --description "Integration testing"

# Assign team members to lanes
python3 lane_manager.py assign-session --session dev-alice --lane team-alpha
python3 lane_manager.py assign-session --session dev-bob --lane team-beta
```

#### Shared Development Database
```bash
# Set up shared database for team (optional)
export CI_SHARED_DB_PATH="/shared/ci/database"
ci-db init --shared --path $CI_SHARED_DB_PATH

# Configure team access
ci-db permissions set --role developer --access read,write
ci-db permissions set --role lead --access all
```

### Team Workflow Example
```bash
# Developer workflow example
# Developer Alice starts work on feature
./quick_start.sh start team-alpha "Architect,Developer"
./quick_start.sh reserve feature-authentication Architect

# Developer Bob works on different feature  
./quick_start.sh start team-beta "Developer,Tester"
./quick_start.sh reserve feature-dashboard Developer

# Integration lead coordinates
./quick_start.sh start integration "Integrator,Tester"
python3 lane_manager.py check-deps --lane integration
```

## 🎯 Common Development Tasks

### Working with Agents

#### Creating a New Agent
```bash
# Create new agent directory
mkdir AGENTS/MyCustomAgent

# Generate agent template
ci generate agent --name MyCustomAgent --domain custom

# Edit agent configuration
vi AGENTS/MyCustomAgent/metadata.json
vi AGENTS/MyCustomAgent/MyCustomAgent_memory.md

# Register agent
ci-db agents add MyCustomAgent AGENTS/MyCustomAgent

# Test agent
ci load MyCustomAgent
```

#### Agent Development Workflow
```bash
# Edit agent memory
vi AGENTS/Athena/Athena_memory.md

# Update agent learning
vi AGENTS/Athena/ContinuousLearning.md

# Test changes
ci load Athena --force

# Update usage statistics
ci-db agents update Athena --usage-count +1
```

### Working with BRAIN Knowledge

#### Submitting Knowledge
```bash
# Create knowledge file
cat > my_knowledge.md << EOF
# New Development Pattern

This pattern improves agent coordination by...

## Implementation

1. Step one
2. Step two
3. Step three

## Benefits

- Improved performance
- Better reliability
EOF

# Submit to BRAIN
ci submit my_knowledge.md --category patterns
```

#### Querying BRAIN
```bash
# Search for information
ci query "agent memory architecture"
ci query --domain protocols "coordination"
ci query --recent "database optimization"

# Browse BRAIN structure
ci brain --structure
ci brain --recent
```

### Database Operations

#### Managing Agent Data
```bash
# View all agents
ci-db agents list --detailed

# Add learning entry
ci-db learning add Athena "Discovered new optimization pattern for memory loading"

# View learning history
ci-db learning list --agent Athena --recent 10

# Export agent data
ci-db agents export Athena agent_data.json
```

#### Session Management
```bash
# Create named session
ci-db sessions create "Feature Development Session"

# Track session activity
ci-db sessions update $SESSION_ID --notes "Working on authentication system"

# View session history
ci-db sessions list --recent 5 --detailed
```

## 🔍 Debugging and Troubleshooting

### Common Issues and Solutions

#### Issue: Agent Loading Fails
```bash
# Debug agent loading
ci load Athena --debug --verbose

# Check agent configuration
ci agents --validate Athena

# Verify file permissions
ls -la AGENTS/Athena/
chmod 644 AGENTS/Athena/*.md

# Clear agent cache
ci cache clear --agents
```

#### Issue: Database Connection Problems
```bash
# Check database status
ci-db health

# Verify database file permissions
ls -la $CI_DB_PATH/

# Reinitialize if corrupted
ci-db backup current_db.backup
ci-db init --force
ci-db restore current_db.backup
```

#### Issue: Coordination System Problems
```bash
# Check coordination files
cd BRAIN/coordination
ls -la ../*.json

# Validate coordination data
python3 session_coordinator.py status --system --verbose

# Reset coordination system
./quick_start.sh reset
./quick_start.sh init
```

### Debug Mode Operations

#### Enable Debug Logging
```bash
# Set debug environment
export CI_DEBUG=true
export RUST_LOG=debug
export CI_LOG_LEVEL=debug

# Run commands with debug output
ci load Athena 2>&1 | tee debug.log

# View coordination debug info
python3 session_coordinator.py status --debug
```

#### Performance Debugging
```bash
# Profile agent loading
time ci load "Architect,Developer,Designer"

# Monitor database performance
ci-db analyze --performance

# Check system resources
ci diagnose --system --resources
```

## 📈 Development Best Practices

### Code Quality

#### Rust Development
```bash
# Format code
cargo fmt

# Check for issues
cargo clippy -- -D warnings

# Run tests
cargo test --all

# Check security
cargo audit
```

#### Python Scripts
```bash
# Format Python coordination scripts
cd BRAIN/coordination
black *.py

# Check Python code quality
pylint *.py
flake8 *.py
```

### Testing Practices

#### Running Tests
```bash
# Run Rust tests
cargo test --workspace

# Run integration tests
cargo test --test integration

# Test CLI functionality
cd cli && cargo test
```

#### Performance Testing
```bash
# Benchmark agent loading
ci benchmark --agents --iterations 100

# Test coordination system performance
cd BRAIN/coordination
python3 load_test.py --sessions 10 --duration 60
```

### Documentation

#### Update Documentation
```bash
# Generate API documentation
cargo doc --open

# Update README files
# Edit relevant README.md files

# Test documentation examples
# Verify all code examples work
```

## 🚀 Next Steps

### Beginner Path (First Week)
1. **Day 1-2**: Get familiar with basic CLI commands and agent loading
2. **Day 3-4**: Learn coordination system and session management
3. **Day 5-7**: Practice submitting knowledge and using BRAIN system

### Intermediate Path (First Month)
1. **Week 1**: Master all CLI tools and coordination workflows
2. **Week 2**: Create custom agents and modify existing ones
3. **Week 3**: Work with database operations and performance tuning
4. **Week 4**: Implement custom integration and automation scripts

### Advanced Path (First Quarter)
1. **Month 1**: Become proficient with all system components
2. **Month 2**: Contribute to core system development
3. **Month 3**: Design and implement major features or improvements

### Learning Resources

#### Essential Reading
- [System Specification](SPECIFICATION.md) - Complete technical specification
- [Architecture Guide](TECHNICAL_ARCHITECTURE_GUIDE.md) - Deep technical details
- [Usage Guide](USAGE.md) - Comprehensive user manual
- [Multi-Agent Orchestration](MULTI_AGENT_DEVELOPMENT_ORCHESTRATION.md) - Coordination patterns

#### Practical Guides
- [CLI Documentation](../cli/docs/documentation_index.md) - Complete CLI reference
- [Coordination System](coordination/README.md) - Multi-agent coordination
- [Agent Development](../AGENTS/README.md) - Creating custom agents

#### Community Resources
- **GitHub Issues**: Report bugs and request features
- **BRAIN Knowledge Base**: Accumulated system wisdom
- **Agent Network**: Learn from specialized agents
- **Development Sessions**: Join coordination workflows

## 📞 Getting Help

### Self-Service Resources
```bash
# Built-in help
ci help
ci help agents
ci-db help

# System diagnostics
ci diagnose --full
ci health --verbose

# Documentation access
ci docs --local
```

### Community Support
- **Documentation**: This guide and linked references
- **Examples**: Working code examples in repository
- **Issues**: GitHub issue tracker for bug reports
- **Discussions**: Community discussions for questions

### Professional Support
- **Implementation Consulting**: Custom development assistance
- **Training Programs**: Structured learning paths
- **Enterprise Support**: Production deployment help

---

## ✅ Setup Verification Checklist

Mark each item as complete:

### Basic Setup
- [ ] Repository cloned and environment variables set
- [ ] Rust and dependencies installed
- [ ] CLI tools built and installed globally
- [ ] Database initialized and operational
- [ ] Agent system functional (can load agents)

### Coordination System  
- [ ] Coordination scripts executable and functional
- [ ] Can start and manage development sessions
- [ ] Task coordination system operational
- [ ] Session management working

### Development Environment
- [ ] IDE configured with appropriate extensions
- [ ] Debug environment set up
- [ ] Pre-commit hooks installed (optional)
- [ ] Team coordination configured (if applicable)

### Verification Tests
- [ ] All basic system tests pass
- [ ] Agent loading and management works
- [ ] Database operations functional
- [ ] BRAIN knowledge system accessible
- [ ] Multi-agent workflows operational

### Ready for Development
- [ ] Can create and modify agents
- [ ] Can submit and query BRAIN knowledge
- [ ] Can coordinate multi-agent sessions
- [ ] Familiar with debugging tools and procedures

---

**Congratulations!** 🎉 You now have a fully functional CollaborativeIntelligence development environment. You're ready to start building with the platform, contributing to development, or using it for collaborative AI workflows.

**What's Next?**
- Explore the [Usage Guide](USAGE.md) for advanced features
- Read the [Technical Architecture Guide](TECHNICAL_ARCHITECTURE_GUIDE.md) for deep understanding
- Join coordination sessions and start collaborating with agents
- Consider contributing new agents or system improvements

---

*Developer Quick Start Guide Version: 1.0*  
*System Compatibility: CollaborativeIntelligence v1.0+*  
*Last Updated: 2025-09-02*  
*Average Setup Time: 30 minutes*  
*Success Rate: 95%+ with prerequisites met*