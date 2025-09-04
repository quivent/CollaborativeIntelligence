# CollaborativeIntelligence Agent Loading Scripts

This directory contains scripts for loading and managing agents from the CollaborativeIntelligence system.

## Available Scripts

### load-topologist.sh

**Purpose**: Load the complete memory architecture of the Repository Topologist agent for use with Claude Code.

**Usage**:
```bash
# Basic usage
./scripts/load-topologist.sh

# Load quietly and pipe to Claude Code
./scripts/load-topologist.sh -q | claude code

# Use custom project path
./scripts/load-topologist.sh -p /path/to/CollaborativeIntelligence

# Show help
./scripts/load-topologist.sh -h
```

**What it loads**:
- **Core memory files**: MEMORY.md, ContinuousLearning.md, README.md
- **Context files**: CommitNotification.md, Communication.md, protocols, TOOLKIT_INDEX.md
- **JavaScript tools**: initialization.js, repository_change_analyzer.js, sequential_commit_protocol.js
- **Support files**: change_report_template.md, Topologist_memory.md
- **Recent session information**: Latest session from Sessions/ directory
- **Topologist CLI tool**: Specialized Rust CLI tool specification
- **CI CLI integration context**: Main CollaborativeIntelligence CLI system
- **Agent activation summary**: Complete capabilities overview

**Features**:
- Path validation and error checking
- Colored output for better readability
- Quiet mode for piping to Claude Code
- Configurable project paths
- Comprehensive error handling

## Discovery Process Documentation

The process used to discover and document the Topologist agent memory involved:

1. **Project Structure Analysis**: 
   - Located CollaborativeIntelligence at `/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence`
   - Identified AGENTS directory with individual agent subdirectories

2. **Agent Discovery**:
   - Used `Grep` to search for "Topolog" across the AGENTS directory
   - Found Topologist agent at `AGENTS/Topologist/`

3. **Memory Architecture Analysis**:
   - **MEMORY.md**: Core identity, critical protocols, knowledge domains
   - **ContinuousLearning.md**: Learning framework with incident-based refinement
   - **README.md**: Operational guidelines and current repository status

4. **System Integration Discovery**:
   - Found CI CLI at `cli/` directory with comprehensive agent management
   - Identified Sessions/ directory for historical context
   - Located tools and protocols for repository management

5. **Key Insights Captured**:
   - Enhanced learning architecture with three-tier triggers
   - Strict repository boundary enforcement
   - Sequential commit protocols with mandatory checklists
   - Cross-agent learning patterns
   - JavaScript-based repository analysis tools
   - Specialized Topologist CLI tool in Rust
   - Automated initialization with protocol enforcement
   - Comprehensive change impact scoring system

## Creating Additional Agent Loaders

To create similar loading scripts for other agents:

1. **Follow the template pattern** from `load-topologist.sh`
2. **Identify core memory files** for the target agent
3. **Locate agent-specific context** (sessions, tools, protocols)
4. **Adapt the loading logic** for agent-specific requirements
5. **Test with both direct execution and Claude Code piping**

## Integration with CI CLI

The loading scripts complement the existing CI CLI system:

- **CI CLI**: `ci load <agent>` - Official agent loading through the CI tool
- **Loading Scripts**: Direct shell scripts for detailed memory analysis
- **Use Cases**: 
  - CI CLI for regular agent activation
  - Loading scripts for deep memory examination and debugging

## Script Maintenance

- **Path Configuration**: Update `CI_PROJECT_PATH` if project location changes
- **Memory Files**: Add new core files to the `memory_files` array
- **Context Files**: Update `context_files` array for new protocols or documentation
- **Error Handling**: Maintain robust path checking and user feedback