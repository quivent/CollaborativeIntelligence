# Collaborative Intelligence System Installation Guide

This guide provides step-by-step instructions for installing and configuring the Collaborative Intelligence (CI) system for new users.

## Prerequisites

Before installing the CI system, ensure you have the following prerequisites:

- macOS, Linux, or Windows with WSL
- Git installed and configured
- Node.js (v14 or later) and npm installed
- Claude CLI installed and configured
- Bash or compatible shell

## Installation Options

There are three ways to install the CI system:

1. **Embedded Installation**: Install CI directly within your project
2. **Sibling Installation**: Install CI next to your project in a separate directory
3. **Symlink Installation**: Use a central CI installation and symlink to it

This guide focuses on the embedded installation, which is recommended for most users.

## Quick Installation

For a quick installation, run:

```bash
# Clone the repository
git clone https://github.com/your-username/CollaborativeIntelligence.git

# Run the installer
cd CollaborativeIntelligence
./ci-tools/install.sh
```

## Step-by-Step Installation

### 1. Clone the Repository

```bash
git clone https://github.com/your-username/CollaborativeIntelligence.git
cd CollaborativeIntelligence
```

### 2. Set Up Environment

Create a `.env` file in the root directory:

```bash
echo "CI_REPO_PATH=$(pwd)" > .env
```

### 3. Make Command-line Tools Executable

```bash
chmod +x CI ci-*
chmod +x ci-tools/cmd/*.sh
chmod +x ci-tools/validate-config.sh
```

### 4. Create Configuration Directory

```bash
mkdir -p .ci-config
```

### 5. Initialize Configuration

```bash
cat > .ci-config/config.json << 'EOF'
{
  "paths": {
    "agents": "./AGENTS",
    "sessions": "./Sessions",
    "templates": "./ci-tools/templates"
  },
  "agents": {
    "enabled": ["Athena", "ProjectArchitect"],
    "disabled": []
  },
  "settings": {
    "fast_activation": true
  }
}
EOF
```

## Installing in a Project

To install CI in an existing project:

```bash
# Navigate to your project directory
cd /path/to/your/project

# Run the integration command
/path/to/CollaborativeIntelligence/CI integrate
```

This will:
1. Create a CLAUDE.local.md file in your project
2. Set up the necessary configuration

## Configuration Options

### Basic Configuration

The CI system can be configured through the following:

1. **Environment Variables**:
   - `CI_REPO_PATH`: Path to the CI repository
   - `CI_DEBUG`: Enable debug mode when set to "true"
   - `CI_VERBOSE`: Enable verbose output when set to "true"

2. **Configuration Files**:
   - `.ci-config/config.json`: Global configuration
   - `.ci-config/user.json`: User preferences
   - `.collaborative-intelligence.json`: Project-specific settings

### Configuring Agents

To configure which agents are available:

```bash
# List available agents
./ci-agent list

# Enable an agent
./ci-agent enable AgentName

# Disable an agent
./ci-agent disable AgentName
```

### Configuring Sessions

Sessions can be configured using:

```bash
# Create a new session
./ci-session create "Session Name"

# List available sessions
./ci-session list

# Archive a session
./ci-session archive session-id
```

## Verifying Installation

To verify that CI is installed correctly:

```bash
./ci-verify
```

This will check:
1. Core files exist
2. Configuration is valid
3. Claude CLI can access the CI system

## Troubleshooting

### Common Issues

1. **Verification Times Out**:
   - Run `./ci-fix verify` to add a timeout to the verification process
   - Check if Claude CLI is correctly installed

2. **Missing Configuration**:
   - Run `./ci-tools/validate-config.sh --fix` to validate and fix configuration issues

3. **Agent Not Found**:
   - Ensure the agent directory exists in `AGENTS/`
   - Check agent is enabled with `./ci-agent list`

4. **Permission Denied**:
   - Ensure all scripts are executable: `chmod +x CI ci-* ci-tools/cmd/*.sh`

### Getting Help

For additional help:

```bash
# Get help with CI
./CI --help

# Get help with agent management
./ci-agent --help

# Get help with session management
./ci-session --help
```

## Updating CI

To update the CI system:

```bash
# Navigate to the CI repository
cd /path/to/CollaborativeIntelligence

# Pull the latest changes
git pull

# Run the installation script again
./ci-tools/install.sh
```

## Advanced Configuration

### Custom Agent Templates

Create custom agent templates in `ci-tools/templates/`:

```bash
# Create a new agent from a template
./ci-agent template MyTemplate NewAgentName
```

### Integration Types

The CI system supports three integration types:

1. **embedded**: CI files are directly in your project
2. **sibling**: CI files are in a sibling directory
3. **symlink**: CI files are symlinked into your project

Specify the integration type during installation:

```bash
./CI integrate --integration=sibling
```

## Next Steps

After installation:

1. Start using Claude Code with Athena: `athena`
2. Get a project overview: `Project Overview`
3. Find the right agent for your task: `Recommend an agent`
4. Create your own specialized agent: `./ci-agent create MyAgent`