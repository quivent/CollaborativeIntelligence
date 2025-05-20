use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::env;
use std::process::Command;
use std::time::SystemTime;

// Memory Integration System
// Provides a standardized way to integrate memory loading into the Collaborative Intelligence ecosystem

// Error handling
#[derive(Debug)]
pub enum MemoryIntegrationError {
    IoError(io::Error),
    ConfigError(String),
    RuntimeError(String),
}

impl From<io::Error> for MemoryIntegrationError {
    fn from(err: io::Error) -> Self {
        MemoryIntegrationError::IoError(err)
    }
}

impl std::fmt::Display for MemoryIntegrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MemoryIntegrationError::IoError(e) => write!(f, "I/O error: {}", e),
            MemoryIntegrationError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            MemoryIntegrationError::RuntimeError(e) => write!(f, "Runtime error: {}", e),
        }
    }
}

impl std::error::Error for MemoryIntegrationError {}

type Result<T> = std::result::Result<T, MemoryIntegrationError>;

// Configuration for memory integration
pub struct MemoryIntegrationConfig {
    pub project_dir: PathBuf,
    pub memory_cli_path: PathBuf,
    pub debug: bool,
}

impl Default for MemoryIntegrationConfig {
    fn default() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        Self {
            project_dir: current_dir.clone(),
            memory_cli_path: current_dir.join("target/release/memory_cli"),
            debug: false,
        }
    }
}

// Memory Integration Service
pub struct MemoryIntegration {
    config: MemoryIntegrationConfig,
}

impl MemoryIntegration {
    pub fn new(config: MemoryIntegrationConfig) -> Self {
        Self { config }
    }

    pub fn default() -> Self {
        Self::new(MemoryIntegrationConfig::default())
    }

    // Debug logging
    fn debug_log(&self, message: &str) {
        if self.config.debug {
            eprintln!("[Memory Integration Debug] {}", message);
        }
    }

    // Helper to check if CLI is available
    fn check_cli_available(&self) -> Result<PathBuf> {
        if self.config.memory_cli_path.exists() {
            return Ok(self.config.memory_cli_path.clone());
        }

        // Check for debug build
        let debug_path = self.config.project_dir.join("target/debug/memory_cli");
        if debug_path.exists() {
            self.debug_log("Using debug build of memory CLI");
            return Ok(debug_path);
        }

        Err(MemoryIntegrationError::ConfigError(
            "Memory CLI not found".to_string(),
        ))
    }

    // Update terminal title with agent name
    pub fn update_terminal_title(&self, agent_name: &str, task: &str) -> Result<()> {
        self.debug_log(&format!("Updating terminal title for agent: {}", agent_name));
        
        // Create title string
        let project_name = self.config.project_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "Project".to_string());

        let title = format!("{} - {} - {}", project_name, task, agent_name);
        
        // ANSI escape sequences for various terminal types
        print!("\x1B]0;{}\x07", title); // Linux/macOS terminals
        print!("\x1B]2;{}\x07", title); // Alternative for some terminals
        
        // For Windows terminals, we'd use the `title` command, but
        // since we're avoiding shell commands per project requirements, 
        // we'll implement a Rust alternative if needed
        
        Ok(())
    }

    // Create agent cache for tracking activations
    pub fn create_agent_cache(&self, 
                             agent_name: &str, 
                             directory: &str, 
                             initializer: &str, 
                             intent: &str) -> Result<String> {
        self.debug_log(&format!("Creating agent cache for agent: {}", agent_name));
        
        // Try to use the memory CLI
        match self.check_cli_available() {
            Ok(cli_path) => {
                let output = Command::new(cli_path)
                    .args([
                        "cache",
                        "--agent", agent_name,
                        "--directory", directory,
                        "--initializer", initializer,
                        "--intent", intent,
                        "--format", "json",
                    ])
                    .output()
                    .map_err(|e| MemoryIntegrationError::RuntimeError(format!("Failed to execute memory CLI: {}", e)))?;

                if output.status.success() {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(result)
                } else {
                    Err(MemoryIntegrationError::RuntimeError(format!(
                        "CLI command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )))
                }
            }
            Err(_) => {
                // Fallback to direct implementation if CLI not available
                self.debug_log("CLI not available, using direct implementation");
                
                // Create cache directory
                let cache_dir = self.config.project_dir.join("CACHE/SESSIONS");
                fs::create_dir_all(&cache_dir)?;
                
                // Create unique ID for the cache entry
                let timestamp = SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .map_err(|e| MemoryIntegrationError::RuntimeError(e.to_string()))?
                    .as_secs();
                
                // Sanitize directory for filename
                let sanitized_dir = directory.replace('/', "_");
                
                let entry_id = format!(
                    "{}_{}_{}_{}", 
                    agent_name.replace(' ', "_"), 
                    sanitized_dir, 
                    initializer.replace(' ', "_"), 
                    timestamp
                );
                
                let cache_file = cache_dir.join(format!("{}.md", entry_id));
                
                // Create cache file content
                let content = format!(
                    r#"# Agent Activation Cache

## Metadata
- **Agent:** {}
- **Working Directory:** {}
- **Initializer:** {}
- **Timestamp:** {}

## Intent
{}

## Status
Active

This file was automatically generated by the Agent Cache System and should be immediately integrated into the agent's short-term memory upon activation.
"#,
                    agent_name,
                    directory,
                    initializer,
                    chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                    intent
                );
                
                // Write the cache file
                fs::write(cache_file, content)?;
                
                // Also update terminal title 
                self.update_terminal_title(agent_name, intent)?;
                
                Ok(entry_id)
            }
        }
    }

    // Load agent memory
    pub fn load_agent_memory(&self, agent_name: &str, context: Option<&str>) -> Result<String> {
        self.debug_log(&format!("Loading memory for agent: {}", agent_name));
        
        // Try to use the memory CLI
        match self.check_cli_available() {
            Ok(cli_path) => {
                let mut command = Command::new(cli_path);
                command.args(["load", "--agent", agent_name, "--format", "markdown"]);
                
                if let Some(ctx) = context {
                    command.args(["--context", ctx]);
                }
                
                let output = command
                    .output()
                    .map_err(|e| MemoryIntegrationError::RuntimeError(format!("Failed to execute memory CLI: {}", e)))?;

                if output.status.success() {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    Ok(result)
                } else {
                    Err(MemoryIntegrationError::RuntimeError(format!(
                        "CLI command failed: {}",
                        String::from_utf8_lossy(&output.stderr)
                    )))
                }
            }
            Err(_) => {
                // Fallback to direct implementation if CLI not available
                self.debug_log("CLI not available, using direct implementation");
                
                // Construct paths to memory files
                let agent_dir = self.config.project_dir.join("AGENTS").join(agent_name);
                let memory_file = agent_dir.join("MEMORY.md");
                let learning_file = agent_dir.join("ContinuousLearning.md");
                
                // Check if agent directory exists
                if !agent_dir.exists() {
                    return Err(MemoryIntegrationError::ConfigError(format!(
                        "Agent not found: {}",
                        agent_name
                    )));
                }
                
                // Prepare result
                let mut result = String::new();
                
                // Read memory file if it exists
                if memory_file.exists() {
                    let content = fs::read_to_string(memory_file)?;
                    result.push_str(&content);
                    result.push_str("\n\n# Continuous Learning\n\n");
                }
                
                // Read continuous learning file if it exists
                if learning_file.exists() {
                    let content = fs::read_to_string(learning_file)?;
                    result.push_str(&content);
                }
                
                // If context is provided, look for context-specific files
                if let Some(ctx) = context {
                    let sessions_dir = agent_dir.join("Sessions");
                    if sessions_dir.exists() {
                        let mut context_files = Vec::new();
                        
                        // Find context-related files
                        for entry in fs::read_dir(sessions_dir)? {
                            let entry = entry?;
                            let path = entry.path();
                            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                                if let Some(file_name) = path.file_name() {
                                    let file_name_str = file_name.to_string_lossy().to_lowercase();
                                    if file_name_str.contains(&ctx.to_lowercase()) {
                                        context_files.push(path);
                                    }
                                }
                            }
                        }
                        
                        // Add context files to result
                        if !context_files.is_empty() {
                            result.push_str(&format!("\n\n# Context-Specific Memory: {}\n\n", ctx));
                            
                            for file in context_files {
                                if let Some(file_name) = file.file_name() {
                                    result.push_str(&format!(
                                        "## {}\n\n",
                                        file_name.to_string_lossy().replace(".md", "")
                                    ));
                                }
                                let content = fs::read_to_string(file)?;
                                result.push_str(&content);
                                result.push_str("\n\n");
                            }
                        }
                    }
                }
                
                Ok(result)
            }
        }
    }

    // Fully load an agent
    pub fn activate_agent(&self, agent_name: &str, context: Option<&str>, task: &str) -> Result<String> {
        self.debug_log(&format!("Activating agent: {}", agent_name));
        
        // Update terminal title
        self.update_terminal_title(agent_name, task)?;
        
        // Create cache entry
        let current_dir = env::current_dir()
            .map_err(|e| MemoryIntegrationError::RuntimeError(format!("Failed to get current directory: {}", e)))?;
        
        self.create_agent_cache(
            agent_name, 
            &current_dir.to_string_lossy(), 
            "MemoryIntegration", 
            task
        )?;
        
        // Load agent memory
        self.load_agent_memory(agent_name, context)
    }
}

// Main function for testing/demo
pub fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("Usage: memory_integration <agent_name> [context] [task]");
        return;
    }
    
    let agent_name = &args[1];
    let context = if args.len() > 2 { Some(args[2].as_str()) } else { None };
    let task = if args.len() > 3 { &args[3] } else { "General Task" };
    
    // Initialize with default configuration
    let integration = MemoryIntegration::default();
    
    // Activate agent
    match integration.activate_agent(agent_name, context, task) {
        Ok(memory) => {
            println!("{}", memory);
        }
        Err(e) => {
            eprintln!("Error activating agent: {}", e);
        }
    }
}