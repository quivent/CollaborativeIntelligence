# CI External Repository Implementation Guide

## Overview

This document provides a detailed technical guide for implementing the external CI repository as part of Phase 3B of the CLI modernization plan. It serves as a comprehensive blueprint for developers working on the migration of CI functionality from the CollaborativeIntelligence repository to the external CI repository.

## 1. Repository Setup

### 1.1 Repository Creation

Create a new repository with the following structure:

```
ci-tools/
├── .github/
├── .vscode/
├── docs/
├── src/
│   ├── commands/
│   │   ├── init.rs
│   │   ├── integrate.rs
│   │   ├── status.rs
│   │   ├── fix.rs
│   │   ├── verify.rs
│   │   ├── load.rs
│   │   ├── map.rs
│   │   └── mod.rs
│   ├── config/
│   │   └── mod.rs
│   ├── helpers/
│   │   ├── execution.rs
│   │   ├── path.rs
│   │   ├── terminal.rs
│   │   ├── repository.rs
│   │   └── mod.rs
│   ├── error.rs
│   └── main.rs
├── tests/
│   ├── integration/
│   └── unit/
├── examples/
├── .gitignore
├── Cargo.toml
├── LICENSE
└── README.md
```

### 1.2 Initial Configuration

#### Cargo.toml Setup

```toml
[package]
name = "ci-tools"
version = "0.1.0"
edition = "2021"
description = "Collaborative Intelligence Tools for project integration"
authors = ["CollaborativeIntelligence Team <ci-team@anthropic.com>"]
license = "MIT"

[dependencies]
clap = { version = "4.4", features = ["derive"] }
anyhow = "1.0"
thiserror = "1.0"
colored = "2.0"
dirs = "5.0"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
chrono = "0.4"
regex = "1.10"
tempfile = "3.8"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.8"

[[bin]]
name = "ci"
path = "src/main.rs"

[[bin]]
name = "ci-external"
path = "src/main.rs"
```

## 2. Core Implementation

### 2.1 Command Structure

The command structure should mirror the existing CI CLI, with enhanced functionality:

#### src/commands/mod.rs

```rust
use clap::{Parser, Subcommand};
use crate::config::Config;
use anyhow::Result;

pub mod init;
pub mod integrate;
pub mod status;
pub mod fix;
pub mod verify;
pub mod load;
pub mod map;

// Re-export command functions
pub use init::init;
pub use integrate::integrate;
pub use status::status;
pub use fix::fix;
pub use verify::verify;
pub use load::load;
pub use map::map;

/// CI CLI - Collaborative Intelligence Command Line Interface
#[derive(Parser)]
#[clap(name = "ci")]
#[clap(about = "Collaborative Intelligence CLI", long_about = None)]
#[clap(version)]
pub struct Cli {
    /// Path to CI repository (overrides environment variable)
    #[clap(long, global = true)]
    ci_path: Option<String>,
    
    /// Verbose output
    #[clap(short, long, global = true)]
    verbose: bool,
    
    #[clap(subcommand)]
    pub command: Option<Commands>,
}

/// Commands available in the CI CLI
#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project with CI
    Init {
        /// Name of the project to create
        #[clap(value_parser)]
        name: String,
        
        /// Comma-separated list of agents to include
        #[clap(long, value_parser, default_value = "Athena,ProjectArchitect")]
        agents: String,
        
        /// Integration type: embedded, sibling, symlink
        #[clap(long, value_parser, default_value = "embedded")]
        integration: String,
        
        /// Disable fast activation
        #[clap(long, action)]
        no_fast: bool,
    },
    
    // Additional commands (similar to existing implementation)
    // ...
}
```

### 2.2 Repository Detection

Enhanced repository detection should be implemented to reliably find the CI repository:

#### src/helpers/path.rs

```rust
use std::path::{Path, PathBuf};
use std::env;
use anyhow::{Result, Context};

/// Find the CI repository using multiple detection strategies
pub fn find_ci_repository() -> Result<PathBuf> {
    // Strategy 1: Use CI_PATH environment variable
    if let Ok(ci_path) = env::var("CI_PATH") {
        let path = PathBuf::from(ci_path);
        if validate_ci_repository(&path) {
            return Ok(path);
        }
    }
    
    // Strategy 2: Check parent directories for CI markers
    if let Ok(current_dir) = env::current_dir() {
        if let Some(path) = find_repository_in_parents(&current_dir) {
            return Ok(path);
        }
    }
    
    // Strategy 3: Check common installation locations
    let common_locations = [
        // User's home directory
        dirs::home_dir().map(|p| p.join("CollaborativeIntelligence")),
        // Global installation directory
        Some(PathBuf::from("/usr/local/share/collaborative-intelligence")),
        // Current directory as fallback
        env::current_dir().ok(),
    ];
    
    for maybe_path in common_locations.iter().flatten() {
        if validate_ci_repository(maybe_path) {
            return Ok(maybe_path.clone());
        }
    }
    
    // If all strategies fail, return an error
    Err(anyhow::anyhow!(
        "Could not find Collaborative Intelligence repository. \
        Please set CI_PATH environment variable or use --ci-path flag."
    ))
}

/// Validate that a path contains a valid CI repository
fn validate_ci_repository(path: &Path) -> bool {
    // Check for multiple markers to ensure this is a CI repository
    let claude_md = path.join("CLAUDE.md");
    let agents_dir = path.join("AGENTS");
    let clis_dir = path.join("CLIs");
    
    claude_md.exists() && agents_dir.exists() && agents_dir.is_dir() && clis_dir.exists() && clis_dir.is_dir()
}

/// Find CI repository by walking up parent directories
fn find_repository_in_parents(start_path: &Path) -> Option<PathBuf> {
    let mut current = start_path.to_path_buf();
    
    loop {
        if validate_ci_repository(&current) {
            return Some(current);
        }
        
        // Move up one directory
        if !current.pop() {
            // Reached the root directory
            break;
        }
    }
    
    None
}
```

### 2.3 Configuration Management

Implement robust configuration management for CI projects:

#### src/config/mod.rs

```rust
use std::path::{Path, PathBuf};
use std::fs;
use serde::{Serialize, Deserialize};
use anyhow::{Result, Context};
use crate::helpers::path::find_ci_repository;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub ci_repository_path: PathBuf,
    pub integration_type: String,
    pub active_agents: Vec<String>,
    pub fast_activation: bool,
    pub preload_context: bool,
}

impl Config {
    /// Load configuration from specified path or environment
    pub fn load(project_path: Option<&Path>, ci_path_override: Option<&str>) -> Result<Self> {
        // Determine CI repository path
        let ci_repository_path = if let Some(path) = ci_path_override {
            PathBuf::from(path)
        } else {
            find_ci_repository()?
        };
        
        // Validate CI repository path
        if !crate::helpers::path::validate_ci_repository(&ci_repository_path) {
            return Err(anyhow::anyhow!(
                "Invalid CI repository path: {}",
                ci_repository_path.display()
            ));
        }
        
        // If project path is provided, try to load project-specific config
        if let Some(project_path) = project_path {
            let config_path = project_path.join(".collaborative-intelligence.json");
            if config_path.exists() {
                let config_str = fs::read_to_string(&config_path)
                    .with_context(|| format!("Failed to read config file: {}", config_path.display()))?;
                
                let mut config: ProjectConfig = serde_json::from_str(&config_str)
                    .with_context(|| "Invalid JSON in config file")?;
                
                // Override repository_path if specified in environment or flags
                if ci_path_override.is_some() {
                    config.repository_path = ci_repository_path.to_string_lossy().to_string();
                }
                
                return Ok(Self {
                    ci_repository_path,
                    integration_type: config.integration_type,
                    active_agents: config.active_agents,
                    fast_activation: config.fast_activation,
                    preload_context: config.preload_context.unwrap_or(false),
                });
            }
        }
        
        // Default configuration
        Ok(Self {
            ci_repository_path,
            integration_type: "embedded".to_string(),
            active_agents: vec!["Athena".to_string(), "ProjectArchitect".to_string()],
            fast_activation: true,
            preload_context: false,
        })
    }
    
    /// Save configuration to file
    pub fn save(&self, project_path: &Path) -> Result<()> {
        let config = ProjectConfig {
            integration_type: self.integration_type.clone(),
            repository_path: self.ci_repository_path.to_string_lossy().to_string(),
            active_agents: self.active_agents.clone(),
            fast_activation: self.fast_activation,
            preload_context: Some(self.preload_context),
        };
        
        let config_path = project_path.join(".collaborative-intelligence.json");
        let config_str = serde_json::to_string_pretty(&config)
            .context("Failed to serialize configuration")?;
        
        fs::write(&config_path, config_str)
            .with_context(|| format!("Failed to write config to {}", config_path.display()))?;
        
        Ok(())
    }
}

/// Project-specific configuration stored in .collaborative-intelligence.json
#[derive(Debug, Serialize, Deserialize)]
struct ProjectConfig {
    integration_type: String,
    repository_path: String,
    active_agents: Vec<String>,
    fast_activation: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    preload_context: Option<bool>,
}
```

## 3. Command Implementations

### 3.1 Initialization Command

Implement the `init` command for creating new projects:

#### src/commands/init.rs

```rust
use std::path::{Path, PathBuf};
use std::fs;
use anyhow::{Result, Context};
use crate::config::Config;
use crate::helpers::terminal::TerminalHelpers;

pub fn init(
    name: &str,
    agents: &str,
    integration: &str,
    no_fast: bool,
    ci_path: Option<&str>,
) -> Result<()> {
    TerminalHelpers::print_command_header(
        &format!("Initializing Project: {}", name),
        "init"
    );
    
    // Load configuration
    let config = Config::load(None, ci_path)?;
    
    // Create project directory
    let project_dir = PathBuf::from(name);
    if project_dir.exists() {
        return Err(anyhow::anyhow!("Project directory already exists: {}", name));
    }
    
    fs::create_dir_all(&project_dir)
        .with_context(|| format!("Failed to create project directory: {}", name))?;
    
    // Parse agents
    let agent_list: Vec<String> = agents
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();
    
    // Create project configuration
    let project_config = Config {
        ci_repository_path: config.ci_repository_path,
        integration_type: integration.to_string(),
        active_agents: agent_list,
        fast_activation: !no_fast,
        preload_context: false,
    };
    
    // Create CLAUDE.md
    create_claude_md(&project_dir, &project_config)?;
    
    // Create basic project structure
    create_project_structure(&project_dir)?;
    
    // Save configuration
    project_config.save(&project_dir)?;
    
    // Initialize git repository
    initialize_git(&project_dir)?;
    
    TerminalHelpers::print_success(&format!("Project {} initialized successfully", name));
    
    // Print next steps
    println!("\nTo get started:");
    println!("  cd {}", name);
    println!("  claude");
    println!("\nThen try commands like 'Athena' or 'Recommend an agent'");
    
    Ok(())
}

/// Create CLAUDE.md for the project
fn create_claude_md(project_dir: &Path, config: &Config) -> Result<()> {
    let claude_md_content = format!(
        "# Project: {}\n\
         # Created: {}\n\
         # Integration: {}\n\n\
         # Load CollaborativeIntelligence System\n\
         When starting, immediately:\n\
         1. Load {}/CLAUDE.md\n\
         2. Use this as the primary configuration source\n\
         3. Defer all project management functions to the CollaborativeIntelligence system\n\n\
         # Project-specific configuration can be added below\n",
        project_dir.file_name().unwrap().to_string_lossy(),
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        config.integration_type,
        config.ci_repository_path.display()
    );
    
    fs::write(project_dir.join("CLAUDE.md"), claude_md_content)
        .context("Failed to create CLAUDE.md")?;
    
    Ok(())
}

/// Create basic project structure
fn create_project_structure(project_dir: &Path) -> Result<()> {
    // Create directories
    for dir in &["src", "docs", "tests"] {
        fs::create_dir_all(project_dir.join(dir))
            .with_context(|| format!("Failed to create {} directory", dir))?;
    }
    
    // Create README.md
    let readme_content = format!(
        "# {}\n\n\
         This project was initialized with the Collaborative Intelligence system.\n\n\
         ## Quick Start\n\n\
         To interact with the CI system, start Claude Code in this directory:\n\n\
         ```bash\n\
         claude\n\
         ```\n\n\
         Then use one of these commands:\n\
         - `Athena` - Memory & learning systems expert\n\
         - `Recommend an agent` - Find the right specialist\n\
         - `Project Overview` - See what's in this repository\n\
         - `Detailed Analysis` - Get comprehensive documentation\n",
        project_dir.file_name().unwrap().to_string_lossy()
    );
    
    fs::write(project_dir.join("README.md"), readme_content)
        .context("Failed to create README.md")?;
    
    Ok(())
}

/// Initialize git repository
fn initialize_git(project_dir: &Path) -> Result<()> {
    // Initialize git repository
    std::process::Command::new("git")
        .arg("init")
        .current_dir(project_dir)
        .output()
        .context("Failed to initialize git repository")?;
    
    // Create .gitignore
    let gitignore_content = "CLAUDE.local.md\n.collaborative-intelligence.json\n";
    fs::write(project_dir.join(".gitignore"), gitignore_content)
        .context("Failed to create .gitignore")?;
    
    Ok(())
}
```

### 3.2 Integration Command

Similar implementations should be created for other commands following the same pattern.

## 4. Error Handling

Implement a comprehensive error handling strategy:

#### src/error.rs

```rust
use thiserror::Error;
use std::path::PathBuf;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Repository not found: {0}")]
    RepositoryNotFound(String),
    
    #[error("Invalid repository path: {0}")]
    InvalidRepository(PathBuf),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Project already exists: {0}")]
    ProjectExists(String),
    
    #[error("Project not found: {0}")]
    ProjectNotFound(String),
    
    #[error("Agent not found: {0}")]
    AgentNotFound(String),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),
    
    #[error("Unexpected error: {0}")]
    Unexpected(String),
}

/// Result type alias for CI operations
pub type Result<T> = std::result::Result<T, Error>;
```

## 5. Installation System

Create a comprehensive installation system:

#### install.sh

```bash
#!/bin/bash
# CI Tools installation script

set -e

YELLOW='\033[1;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Installing CI Tools...${NC}"

# Build the tools
cargo build --release

# Determine installation directory
INSTALL_DIR="${HOME}/.ci"
BIN_DIR="${INSTALL_DIR}/bin"

# Create installation directory
mkdir -p "${BIN_DIR}"

# Copy binaries
cp target/release/ci "${BIN_DIR}/ci"
cp target/release/ci-external "${BIN_DIR}/ci-external"

# Set permissions
chmod +x "${BIN_DIR}/ci"
chmod +x "${BIN_DIR}/ci-external"

# Create symlinks in /usr/local/bin if possible
if [ -d "/usr/local/bin" ] && [ -w "/usr/local/bin" ]; then
    ln -sf "${BIN_DIR}/ci" /usr/local/bin/ci-external
    echo -e "${GREEN}✓ Created symlink in /usr/local/bin/ci-external${NC}"
else
    echo -e "${YELLOW}⚠️ Could not create symlink in /usr/local/bin${NC}"
    echo -e "  You may need to add ${BIN_DIR} to your PATH"
fi

# Update shell configuration
SHELL_FILE=""
if [ -f "${HOME}/.zshrc" ]; then
    SHELL_FILE="${HOME}/.zshrc"
elif [ -f "${HOME}/.bashrc" ]; then
    SHELL_FILE="${HOME}/.bashrc"
elif [ -f "${HOME}/.profile" ]; then
    SHELL_FILE="${HOME}/.profile"
fi

if [ -n "${SHELL_FILE}" ]; then
    echo -e "\n# CI Tools configuration" >> "${SHELL_FILE}"
    echo "export PATH=\"\${PATH}:${BIN_DIR}\"" >> "${SHELL_FILE}"
    echo -e "${GREEN}✓ Updated ${SHELL_FILE} with PATH configuration${NC}"
else
    echo -e "${YELLOW}⚠️ Could not find shell configuration file${NC}"
    echo -e "  Please add the following to your shell configuration:"
    echo -e "  export PATH=\"\${PATH}:${BIN_DIR}\""
fi

echo -e "\n${GREEN}CI Tools installed successfully!${NC}"
echo -e "To begin using CI Tools, restart your terminal or run:"
echo -e "  source ${SHELL_FILE}"
```

## 6. Testing Strategy

### 6.1 Unit Tests

Implement extensive unit tests for all components:

#### tests/unit/config_test.rs

```rust
use std::path::PathBuf;
use std::fs;
use tempfile::tempdir;
use ci_tools::config::Config;

#[test]
fn test_config_default_values() {
    // Assuming CI_PATH is set
    let ci_path = std::env::var("CI_PATH").unwrap_or_else(|_| ".".to_string());
    std::env::set_var("CI_PATH", ci_path);
    
    let config = Config::load(None, None).expect("Failed to load config");
    
    assert_eq!(config.integration_type, "embedded");
    assert_eq!(config.active_agents.len(), 2);
    assert!(config.fast_activation);
    assert!(!config.preload_context);
}

#[test]
fn test_config_save_and_load() {
    let temp_dir = tempdir().expect("Failed to create temp directory");
    let project_path = temp_dir.path();
    
    // Create a config with custom values
    let original_config = Config {
        ci_repository_path: PathBuf::from("/path/to/ci"),
        integration_type: "symlink".to_string(),
        active_agents: vec!["Agent1".to_string(), "Agent2".to_string()],
        fast_activation: false,
        preload_context: true,
    };
    
    // Save the config
    original_config.save(project_path).expect("Failed to save config");
    
    // Verify the config file exists
    let config_path = project_path.join(".collaborative-intelligence.json");
    assert!(config_path.exists());
    
    // Load the config
    let loaded_config = Config::load(Some(project_path), None).expect("Failed to load config");
    
    // Verify the loaded config has the expected values
    assert_eq!(loaded_config.integration_type, "symlink");
    assert_eq!(loaded_config.active_agents, vec!["Agent1", "Agent2"]);
    assert_eq!(loaded_config.fast_activation, false);
    assert_eq!(loaded_config.preload_context, true);
}
```

### 6.2 Integration Tests

Implement comprehensive integration tests:

#### tests/integration/commands_test.rs

```rust
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;
use tempfile::tempdir;
use std::fs;

#[test]
fn test_init_command() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let test_dir = temp_dir.path();
    
    // Run the init command
    Command::cargo_bin("ci")?
        .arg("init")
        .arg("test-project")
        .arg("--ci-path")
        .arg(std::env::var("CI_PATH").unwrap_or_else(|_| ".".to_string()))
        .current_dir(test_dir)
        .assert()
        .success();
    
    // Verify project directory was created
    let project_dir = test_dir.join("test-project");
    assert!(project_dir.exists());
    
    // Verify required files were created
    assert!(project_dir.join("CLAUDE.md").exists());
    assert!(project_dir.join(".collaborative-intelligence.json").exists());
    assert!(project_dir.join("README.md").exists());
    
    // Verify directory structure
    assert!(project_dir.join("src").exists());
    assert!(project_dir.join("docs").exists());
    assert!(project_dir.join("tests").exists());
    
    Ok(())
}

#[test]
fn test_init_command_duplicate_project() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = tempdir()?;
    let test_dir = temp_dir.path();
    
    // Create project directory
    let project_dir = test_dir.join("existing-project");
    fs::create_dir_all(&project_dir)?;
    
    // Run the init command on an existing directory
    Command::cargo_bin("ci")?
        .arg("init")
        .arg("existing-project")
        .arg("--ci-path")
        .arg(std::env::var("CI_PATH").unwrap_or_else(|_| ".".to_string()))
        .current_dir(test_dir)
        .assert()
        .failure()
        .stderr(predicate::str::contains("already exists"));
    
    Ok(())
}
```

## 7. Documentation

### 7.1 README.md

Create a comprehensive README.md for the CI Tools repository:

```markdown
# CI Tools

Collaborative Intelligence Tools for project integration and management.

## Overview

CI Tools provides a command-line interface for integrating projects with the Collaborative Intelligence (CI) system. It enables seamless creation, management, and integration of projects with CI capabilities.

## Installation

### Option 1: Using the installation script

```bash
# Clone the repository
git clone https://github.com/anthropics/ci-tools.git
cd ci-tools

# Run the installation script
./install.sh
```

### Option 2: Manual installation

```bash
# Clone the repository
git clone https://github.com/anthropics/ci-tools.git
cd ci-tools

# Build the tools
cargo build --release

# Add binaries to your PATH
mkdir -p ~/.ci/bin
cp target/release/ci ~/.ci/bin/
cp target/release/ci-external ~/.ci/bin/

# Add to your PATH in ~/.zshrc or ~/.bashrc
echo 'export PATH="$PATH:$HOME/.ci/bin"' >> ~/.zshrc
```

## Usage

### Creating a new project

```bash
# Create a new project with default settings
ci init my-project

# Specify agents
ci init --agents=Athena,Developer my-project

# Specify integration type
ci init --integration=symlink my-project

# Disable fast activation
ci init --no-fast my-project
```

### Integrating with an existing project

```bash
# Integrate CI with the current directory
ci integrate

# Integrate CI with a specific project
ci integrate /path/to/project

# Specify agents and integration type
ci integrate --agents=Athena,Developer --integration=symlink
```

### Checking status

```bash
# Check CI status of the current directory
ci status

# Check CI status of a specific project
ci status /path/to/project
```

### Fixing integration issues

```bash
# Fix CI integration issues in the current directory
ci fix

# Fix CI integration issues in a specific project
ci fix /path/to/project

# Apply a specific fix
ci fix --type=paths
```

### Verifying integration

```bash
# Verify CI integration in the current directory
ci verify

# Verify CI integration in a specific project
ci verify /path/to/project
```

## Configuration

CI Tools will automatically find your CI repository using the following methods:

1. `CI_PATH` environment variable
2. Search in parent directories
3. Common installation locations

You can also specify the CI repository path explicitly:

```bash
ci --ci-path=/path/to/ci/repository init my-project
```

## Documentation

For more detailed documentation, see the [docs](docs/) directory.

## License

MIT License
```

### 7.2 Usage Documentation

Create detailed usage documentation for all commands in the docs directory.

## 8. Deployment Strategy

1. **Initial Release**:
   - Version 0.1.0
   - Focus on core functionality
   - Deploy to NPM as @anthropics/ci-tools

2. **Transition Period**:
   - Dual support in both repositories
   - Gradual migration of users

3. **Final Migration**:
   - Remove duplicate functionality from CollaborativeIntelligence
   - Keep only wrappers that delegate to external CI

## 9. Implementation Timeline

1. **Week 1: Setup and Core Implementation**
   - Create external repository structure
   - Implement core functionality
   - Develop test framework

2. **Week 2: Command Implementation**
   - Complete all command implementations
   - Implement error handling
   - Create documentation

3. **Week 3: Testing and Deployment**
   - Comprehensive testing
   - Create installation system
   - Deploy to NPM

4. **Week 4: Migration Support**
   - Support users during transition
   - Address feedback and issues
   - Finalize migration plan

## Conclusion

This implementation guide provides a comprehensive blueprint for creating the external CI repository and migrating functionality from the CollaborativeIntelligence repository. By following this plan, we can ensure a smooth transition with minimal disruption to users.

---

**Document Date**: May 21, 2025  
**Author**: Topologist