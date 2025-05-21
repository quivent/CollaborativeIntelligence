# CI External Repository Structure

## Overview

This document provides a detailed structural design for the external CI repository, outlining the directory organization, file structure, module relationships, and component interactions. It serves as the architectural blueprint for implementing the external CI tool.

## Repository Root Structure

```
ci-tools/
├── .github/                    # GitHub configuration
│   ├── workflows/              # GitHub Actions workflows
│   │   ├── ci.yml              # CI pipeline configuration
│   │   ├── release.yml         # Release automation
│   │   └── test.yml            # Test automation
│   └── ISSUE_TEMPLATE/         # Issue templates
├── .vscode/                    # VS Code configuration
├── docs/                       # Documentation
├── examples/                   # Example projects and usage
├── scripts/                    # Utility scripts
├── src/                        # Source code
├── tests/                      # Test suite
├── .editorconfig               # Editor configuration
├── .gitignore                  # Git ignore configuration
├── Cargo.toml                  # Rust package configuration
├── README.md                   # Project overview
├── CONTRIBUTING.md             # Contribution guidelines
├── LICENSE                     # License information
└── CHANGELOG.md                # Version history
```

## Source Code Organization

The `src/` directory contains the main Rust implementation with the following structure:

```
src/
├── bin/                        # Binary executables
│   └── ci.rs                   # Main CLI entry point
├── cli/                        # CLI infrastructure
│   ├── mod.rs                  # Module exports
│   ├── app.rs                  # CLI application setup
│   └── args.rs                 # Argument parsing
├── commands/                   # Command implementations
│   ├── mod.rs                  # Module exports
│   ├── init.rs                 # Initialize project
│   ├── integrate.rs            # Integrate with project
│   ├── status.rs               # Check status
│   ├── fix.rs                  # Fix issues
│   ├── verify.rs               # Verify setup
│   ├── load.rs                 # Load configurations
│   └── map.rs                  # Map repository
├── config/                     # Configuration management
│   ├── mod.rs                  # Module exports
│   ├── env.rs                  # Environment variables
│   ├── project.rs              # Project configuration
│   └── settings.rs             # User settings
├── core/                       # Core functionality
│   ├── mod.rs                  # Module exports
│   ├── repository.rs           # Repository interactions
│   ├── templates.rs            # Template management
│   └── validation.rs           # Validation utilities
├── error/                      # Error handling
│   ├── mod.rs                  # Module exports
│   ├── types.rs                # Error types
│   └── handling.rs             # Error handling utilities
├── helpers/                    # Utility functions
│   ├── mod.rs                  # Module exports
│   ├── fs.rs                   # File system utilities
│   ├── git.rs                  # Git utilities
│   └── process.rs              # Process utilities
├── integration/                # Project integration
│   ├── mod.rs                  # Module exports
│   ├── detection.rs            # Project detection
│   ├── claude.rs               # Claude integration
│   └── templates.rs            # Template system
├── lib.rs                      # Library exports
└── main.rs                     # Main executable entry
```

## Detailed Component Design

### 1. CLI Application (`src/cli/`)

The CLI application handles argument parsing, command routing, and user interaction.

#### `app.rs`

```rust
use clap::{Parser, Subcommand};
use crate::commands;
use crate::error::Result;

#[derive(Parser)]
#[clap(name = "ci", about = "Collaborative Intelligence CLI")]
pub struct Cli {
    #[clap(subcommand)]
    pub command: Commands,
    
    #[clap(short, long, help = "Verbose output")]
    pub verbose: bool,
    
    #[clap(long, help = "Configuration file path")]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new project
    Init {
        #[clap(help = "Project name")]
        name: String,
        
        #[clap(long, help = "Integration type", default_value = "standalone")]
        integration: String,
        
        #[clap(long, help = "Agents to load")]
        agents: Option<String>,
        
        #[clap(long, help = "Custom CI path")]
        ci_path: Option<String>,
        
        #[clap(long, help = "Fast activation mode")]
        fast: bool,
    },
    // Other commands...
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    
    match &cli.command {
        Commands::Init { name, integration, agents, ci_path, fast } => {
            commands::init::execute(name, integration, agents.as_deref(), ci_path.as_deref(), *fast)
        },
        // Other command handlers...
    }
}
```

### 2. Command Implementations (`src/commands/`)

Each command is implemented in its own module with a clear public API.

#### `init.rs`

```rust
use crate::config::{ProjectConfig, load_config};
use crate::core::templates::TemplateEngine;
use crate::error::{Result, Error};
use crate::helpers::fs::{ensure_directory, write_file};
use crate::integration::claude::setup_claude_integration;
use std::path::Path;

/// Initialize a new project with CI integration
pub fn execute(
    name: &str,
    integration_type: &str,
    agents: Option<&str>,
    ci_path: Option<&str>,
    fast_activation: bool,
) -> Result<()> {
    // Validate inputs
    if name.is_empty() {
        return Err(Error::InvalidInput("Project name cannot be empty".into()));
    }
    
    let valid_integrations = ["standalone", "embedded", "linked"];
    if !valid_integrations.contains(&integration_type) {
        return Err(Error::InvalidInput(format!(
            "Invalid integration type: {}. Must be one of: {}",
            integration_type,
            valid_integrations.join(", ")
        )));
    }
    
    // Create project configuration
    let config = ProjectConfig::new(name, integration_type, agents, ci_path)?;
    
    // Setup project structure
    setup_project_structure(&config)?;
    
    // Setup Claude integration
    setup_claude_integration(&config, fast_activation)?;
    
    println!("Project '{}' initialized successfully with {} integration", name, integration_type);
    
    Ok(())
}

fn setup_project_structure(config: &ProjectConfig) -> Result<()> {
    // Create project directory if it doesn't exist
    ensure_directory(&config.project_path)?;
    
    // Generate CLAUDE.md file
    let template_engine = TemplateEngine::new();
    let claude_md_content = template_engine.render("CLAUDE.md", &config)?;
    
    write_file(Path::new(&config.project_path).join("CLAUDE.md"), &claude_md_content)?;
    
    // Create other necessary files based on integration type
    match config.integration_type.as_str() {
        "standalone" => setup_standalone_structure(config)?,
        "embedded" => setup_embedded_structure(config)?,
        "linked" => setup_linked_structure(config)?,
        _ => unreachable!(), // We've already validated the integration type
    }
    
    Ok(())
}

// Implementation of setup functions for different integration types...
```

### 3. Configuration Management (`src/config/`)

Handles loading, parsing, and managing configuration from various sources.

#### `project.rs`

```rust
use serde::{Deserialize, Serialize};
use crate::error::{Result, Error};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub integration_type: String,
    pub agents: Vec<String>,
    pub project_path: String,
    pub ci_path: Option<String>,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

impl ProjectConfig {
    pub fn new(
        name: &str,
        integration_type: &str,
        agents: Option<&str>,
        ci_path: Option<&str>,
    ) -> Result<Self> {
        let agents = match agents {
            Some(a) => a.split(',').map(|s| s.trim().to_string()).collect(),
            None => vec!["Athena".to_string()], // Default agent
        };
        
        let project_path = std::env::current_dir()
            .map_err(|e| Error::Io(format!("Failed to get current directory: {}", e)))?
            .to_string_lossy()
            .to_string();
        
        Ok(Self {
            name: name.to_string(),
            integration_type: integration_type.to_string(),
            agents,
            project_path,
            ci_path: ci_path.map(|s| s.to_string()),
            created_at: chrono::Utc::now(),
        })
    }
    
    pub fn load(path: &Path) -> Result<Self> {
        let content = fs::read_to_string(path)
            .map_err(|e| Error::Io(format!("Failed to read config file: {}", e)))?;
        
        serde_json::from_str(&content)
            .map_err(|e| Error::Parse(format!("Failed to parse config file: {}", e)))
    }
    
    pub fn save(&self, path: &Path) -> Result<()> {
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| Error::Serialize(format!("Failed to serialize config: {}", e)))?;
        
        fs::write(path, content)
            .map_err(|e| Error::Io(format!("Failed to write config file: {}", e)))?;
        
        Ok(())
    }
}
```

### 4. Core Functionality (`src/core/`)

Implements the core business logic of the CI tool.

#### `repository.rs`

```rust
use std::path::{Path, PathBuf};
use crate::error::{Result, Error};
use crate::helpers::git;

/// Represents a repository with CI integration
pub struct Repository {
    path: PathBuf,
    is_git_repo: bool,
}

impl Repository {
    /// Create a new repository instance
    pub fn new(path: &Path) -> Result<Self> {
        let abs_path = if path.is_absolute() {
            path.to_path_buf()
        } else {
            std::env::current_dir()
                .map_err(|e| Error::Io(format!("Failed to get current directory: {}", e)))?
                .join(path)
        };
        
        if !abs_path.exists() {
            return Err(Error::NotFound(format!("Repository path does not exist: {}", abs_path.display())));
        }
        
        let is_git_repo = git::is_git_repository(&abs_path)?;
        
        Ok(Self {
            path: abs_path,
            is_git_repo,
        })
    }
    
    /// Get the repository path
    pub fn path(&self) -> &Path {
        &self.path
    }
    
    /// Check if the repository is a git repository
    pub fn is_git_repo(&self) -> bool {
        self.is_git_repo
    }
    
    /// Check if the repository has CI integration
    pub fn has_ci_integration(&self) -> Result<bool> {
        let claude_md_path = self.path.join("CLAUDE.md");
        Ok(claude_md_path.exists())
    }
    
    /// Get the CI configuration path
    pub fn ci_config_path(&self) -> PathBuf {
        self.path.join(".ci").join("config.json")
    }
    
    /// Initialize CI in this repository
    pub fn initialize_ci(&self, config: &crate::config::ProjectConfig) -> Result<()> {
        // Implementation...
        Ok(())
    }
}
```

### 5. Error Handling (`src/error/`)

Comprehensive error handling system for the CI tool.

#### `types.rs`

```rust
use std::fmt;

#[derive(Debug)]
pub enum Error {
    /// Input validation errors
    InvalidInput(String),
    
    /// IO errors
    Io(String),
    
    /// Not found errors
    NotFound(String),
    
    /// Configuration errors
    Config(String),
    
    /// Parse errors
    Parse(String),
    
    /// Serialization errors
    Serialize(String),
    
    /// Git errors
    Git(String),
    
    /// Template errors
    Template(String),
    
    /// External command errors
    Command(String),
    
    /// Other errors
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            Error::Io(msg) => write!(f, "IO error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
            Error::Config(msg) => write!(f, "Configuration error: {}", msg),
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Serialize(msg) => write!(f, "Serialization error: {}", msg),
            Error::Git(msg) => write!(f, "Git error: {}", msg),
            Error::Template(msg) => write!(f, "Template error: {}", msg),
            Error::Command(msg) => write!(f, "Command error: {}", msg),
            Error::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
```

### 6. Helpers (`src/helpers/`)

Utility functions for common operations.

#### `fs.rs`

```rust
use std::fs;
use std::path::{Path, PathBuf};
use crate::error::{Result, Error};

/// Ensure a directory exists, creating it if necessary
pub fn ensure_directory(path: &str) -> Result<()> {
    let path = Path::new(path);
    
    if !path.exists() {
        fs::create_dir_all(path)
            .map_err(|e| Error::Io(format!("Failed to create directory {}: {}", path.display(), e)))?;
    } else if !path.is_dir() {
        return Err(Error::InvalidInput(format!("{} exists but is not a directory", path.display())));
    }
    
    Ok(())
}

/// Write content to a file, creating parent directories if necessary
pub fn write_file(path: impl AsRef<Path>, content: &str) -> Result<()> {
    let path = path.as_ref();
    
    // Ensure parent directory exists
    if let Some(parent) = path.parent() {
        ensure_directory(&parent.to_string_lossy())?;
    }
    
    fs::write(path, content)
        .map_err(|e| Error::Io(format!("Failed to write to {}: {}", path.display(), e)))?;
    
    Ok(())
}

/// Read a file to string
pub fn read_file(path: impl AsRef<Path>) -> Result<String> {
    let path = path.as_ref();
    
    fs::read_to_string(path)
        .map_err(|e| Error::Io(format!("Failed to read from {}: {}", path.display(), e)))
}

/// Check if a file exists
pub fn file_exists(path: impl AsRef<Path>) -> bool {
    path.as_ref().exists() && path.as_ref().is_file()
}

/// Find a file in a directory tree, searching up from the given path
pub fn find_file_up(start_dir: &Path, filename: &str) -> Option<PathBuf> {
    let mut current_dir = start_dir.to_path_buf();
    
    loop {
        let file_path = current_dir.join(filename);
        if file_path.exists() && file_path.is_file() {
            return Some(file_path);
        }
        
        if !current_dir.pop() {
            return None;
        }
    }
}
```

### 7. Integration (`src/integration/`)

Handles integration with external systems and projects.

#### `claude.rs`

```rust
use crate::config::ProjectConfig;
use crate::core::templates::TemplateEngine;
use crate::error::{Result, Error};
use crate::helpers::fs::write_file;
use std::path::Path;

/// Setup Claude integration for a project
pub fn setup_claude_integration(config: &ProjectConfig, fast_activation: bool) -> Result<()> {
    // Generate CLAUDE.md file
    let template_engine = TemplateEngine::new();
    let claude_md_content = template_engine.render("CLAUDE.md", &config)?;
    
    let claude_md_path = Path::new(&config.project_path).join("CLAUDE.md");
    write_file(&claude_md_path, &claude_md_content)?;
    
    // Generate CLAUDE.local.md file if needed
    if config.integration_type != "standalone" {
        let claude_local_md_content = template_engine.render("CLAUDE.local.md", &config)?;
        let claude_local_md_path = Path::new(&config.project_path).join("CLAUDE.local.md");
        write_file(&claude_local_md_path, &claude_local_md_content)?;
    }
    
    // Generate activation script if needed
    if !fast_activation {
        let activation_script_content = template_engine.render("activation.sh", &config)?;
        let activation_script_path = Path::new(&config.project_path).join(".ci").join("activation.sh");
        write_file(&activation_script_path, &activation_script_content)?;
        
        // Make the script executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = std::fs::metadata(&activation_script_path)
                .map_err(|e| Error::Io(format!("Failed to get permissions: {}", e)))?
                .permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(&activation_script_path, perms)
                .map_err(|e| Error::Io(format!("Failed to set permissions: {}", e)))?;
        }
    }
    
    Ok(())
}
```

## Test Suite Organization

The `tests/` directory is organized to thoroughly test the CI tool's functionality:

```
tests/
├── commands/                   # Command tests
│   ├── init_test.rs            # Test init command
│   ├── integrate_test.rs       # Test integrate command
│   ├── status_test.rs          # Test status command
│   └── verify_test.rs          # Test verify command
├── config/                     # Configuration tests
│   ├── env_test.rs             # Test environment variables
│   └── project_test.rs         # Test project configuration
├── core/                       # Core functionality tests
│   ├── repository_test.rs      # Test repository handling
│   └── templates_test.rs       # Test template engine
├── fixtures/                   # Test fixtures
│   ├── projects/               # Example project structures
│   └── templates/              # Template test cases
├── helpers/                    # Helper function tests
│   ├── fs_test.rs              # Test file system utilities
│   └── git_test.rs             # Test git utilities
├── integration/                # Integration tests
│   └── claude_test.rs          # Test Claude integration
├── common/                     # Common test utilities
│   ├── mod.rs                  # Module exports
│   └── fixtures.rs             # Fixture management
└── integration_tests.rs        # End-to-end integration tests
```

## Package Management

The `Cargo.toml` file defines the package structure and dependencies:

```toml
[package]
name = "ci-tools"
version = "0.1.0"
edition = "2021"
authors = ["Collaborative Intelligence Team"]
description = "CLI tool for Collaborative Intelligence integration"
repository = "https://github.com/anthropics/ci-tools"
license = "MIT"
readme = "README.md"

[dependencies]
clap = { version = "4.3", features = ["derive"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
anyhow = "1.0"
thiserror = "1.0"
chrono = { version = "0.4", features = ["serde"] }
log = "0.4"
env_logger = "0.10"
dirs = "5.0"
handlebars = "4.3"
git2 = "0.17"
toml = "0.7"
path-absolutize = "3.1"
tokio = { version = "1.28", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
tempfile = "3.6"
similar = "2.2"
colored = "2.0"

[dev-dependencies]
assert_cmd = "2.0"
predicates = "3.0"
tempfile = "3.6"
test-case = "3.1"
mockall = "0.11"

[[bin]]
name = "ci"
path = "src/bin/ci.rs"

[lib]
name = "ci_tools"
path = "src/lib.rs"
```

## Documentation Organization

The `docs/` directory contains comprehensive documentation:

```
docs/
├── api/                        # API documentation
│   ├── commands.md             # Command reference
│   ├── config.md               # Configuration reference
│   └── core.md                 # Core functionality
├── guides/                     # User guides
│   ├── getting-started.md      # Getting started guide
│   ├── project-integration.md  # Project integration guide
│   └── troubleshooting.md      # Troubleshooting guide
├── development/                # Development documentation
│   ├── architecture.md         # Architectural overview
│   ├── testing.md              # Testing guidelines
│   └── contributing.md         # Contribution workflow
├── examples/                   # Example documentation
│   ├── standalone.md           # Standalone integration
│   ├── embedded.md             # Embedded integration
│   └── linked.md               # Linked integration
└── README.md                   # Documentation overview
```

## Module Dependencies

Here's a diagram of the key module dependencies:

```
bin/ci.rs → cli/app.rs → commands/* → config, core, helpers
                       → error/types.rs
                       → integration/*

commands/* → config/* → error/types.rs
          → core/* → helpers/*
          → integration/*
          → error/types.rs

core/* → helpers/* → error/types.rs
      → error/types.rs

integration/* → core/*
             → config/*
             → helpers/*
             → error/types.rs
```

## Implementation Timeline

The implementation will follow this timeline:

1. **Week 1: Foundation**
   - Repository setup
   - Basic CLI structure
   - Error handling system
   - Core utilities

2. **Week 2: Core Functionality**
   - Command implementations
   - Configuration management
   - Template system
   - Basic integration

3. **Week 3: Testing and Refinement**
   - Test suite implementation
   - Documentation
   - Error handling refinement
   - Performance optimization

4. **Week 4: Polish and Release**
   - Integration tests
   - Documentation finalization
   - CI/CD pipeline setup
   - Release preparation