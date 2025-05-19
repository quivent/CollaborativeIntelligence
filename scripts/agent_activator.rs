use std::env;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use clap::{App, Arg, ArgMatches, SubCommand};
use colored::*;
use glob::glob;

// Import from our memory system crate
use crate::memory_segment_parser::{MemorySegment, MemorySegmentParser, MemoryType, MemoryTier};
use crate::memory_loader::{MemoryLoader, MemoryLoaderConfig};
use crate::memory_activation_model::MemoryActivationModel;
use crate::memory_updater::{MemoryUpdater, MemoryUpdaterConfig};
use crate::memory_checksum_cache::{ChecksumCache, ChecksumCacheConfig};

// Agent activation configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentActivatorConfig {
    pub agents_dir: PathBuf,
    pub cache_dir: PathBuf,
    pub memory_optimization: bool,
    pub parallel_loading: bool,
    pub update_terminal_title: bool,
    pub enforce_protocol: bool,
    pub verbose: bool,
}

impl Default for AgentActivatorConfig {
    fn default() -> Self {
        let project_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let root_dir = project_dir.clone();
        
        AgentActivatorConfig {
            agents_dir: root_dir.join("AGENTS"),
            cache_dir: root_dir.join("CACHE"),
            memory_optimization: true,
            parallel_loading: true,
            update_terminal_title: true,
            enforce_protocol: true,
            verbose: false,
        }
    }
}

// Result of agent activation
#[derive(Debug, Clone)]
pub struct ActivationResult {
    pub agent_name: String,
    pub memory_segments: Vec<MemorySegment>,
    pub context: Option<String>,
    pub task: String,
    pub activation_time_ms: u64,
    pub success: bool,
    pub error_message: Option<String>,
}

// Agent activator implementation
pub struct AgentActivator {
    config: AgentActivatorConfig,
    memory_loader: Arc<Mutex<MemoryLoader>>,
    checksum_cache: Arc<Mutex<ChecksumCache>>,
    activation_model: Arc<Mutex<MemoryActivationModel>>,
}

impl AgentActivator {
    // Create a new agent activator
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Self::with_config(AgentActivatorConfig::default())
    }
    
    // Create a new agent activator with custom configuration
    pub fn with_config(config: AgentActivatorConfig) -> Result<Self, Box<dyn Error>> {
        // Initialize memory loader
        let loader_config = MemoryLoaderConfig {
            base_agents_dir: config.agents_dir.clone(),
            cache_dir: config.cache_dir.join("MEMORY"),
            parallel_loading: config.parallel_loading,
            cache_enabled: config.memory_optimization,
            preload_identity_segments: true,
            auto_refresh_interval_mins: 60,
            context_based_loading: true,
            verbose_logging: config.verbose,
            timeout_seconds: 30,
        };
        
        let mut memory_loader = MemoryLoader::new(loader_config)?;
        let activation_model = memory_loader.get_activation_model();
        
        // Initialize checksum cache
        let cache_config = ChecksumCacheConfig {
            cache_dir: config.cache_dir.join("CHECKSUMS"),
            validation_interval_seconds: 60,
            enable_file_watcher: true,
            file_watcher_debounce_ms: 500,
            live_update: true,
            verbose_logging: config.verbose,
        };
        
        let mut checksum_cache = ChecksumCache::new(cache_config)?;
        
        // Connect components
        let loader_arc = Arc::new(Mutex::new(memory_loader));
        let cache_arc = Arc::new(Mutex::new(checksum_cache));
        
        // Start file watcher for memory files
        if let Ok(mut cache) = cache_arc.lock() {
            cache.connect_memory_system(loader_arc.clone(), activation_model.clone());
            if let Err(e) = cache.start_file_watcher(&config.agents_dir) {
                eprintln!("Warning: Failed to start file watcher: {}", e);
            }
        }
        
        Ok(AgentActivator {
            config,
            memory_loader: loader_arc,
            checksum_cache: cache_arc,
            activation_model,
        })
    }
    
    // Activate an agent
    pub fn activate_agent(&self, agent_name: &str, context: Option<&str>, task: &str) -> Result<ActivationResult, Box<dyn Error>> {
        let start_time = std::time::Instant::now();
        
        // Check if agent exists
        let agent_dir = self.config.agents_dir.join(agent_name);
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent not found: {}", agent_name).into());
        }
        
        // Update terminal title if enabled
        if self.config.update_terminal_title {
            self.update_terminal_title(agent_name, task)?;
        }
        
        // Enforce agent protocol if enabled
        if self.config.enforce_protocol {
            self.enforce_agent_protocol(agent_name)?;
        }
        
        // Load agent memory
        if self.config.verbose {
            println!("Loading memory for agent: {}", agent_name);
        }
        
        let context_vec = context.map(|ctx| vec![ctx.to_string()]);
        
        let memory_segments = if let Ok(mut loader) = self.memory_loader.lock() {
            match loader.load_agent_memory(agent_name, context_vec) {
                Ok(segments) => segments,
                Err(e) => {
                    return Err(format!("Failed to load agent memory: {}", e).into());
                }
            }
        } else {
            return Err("Failed to acquire lock on memory loader".into());
        };
        
        // Create activation result
        let activation_time_ms = start_time.elapsed().as_millis() as u64;
        
        let result = ActivationResult {
            agent_name: agent_name.to_string(),
            memory_segments,
            context: context.map(|ctx| ctx.to_string()),
            task: task.to_string(),
            activation_time_ms,
            success: true,
            error_message: None,
        };
        
        if self.config.verbose {
            println!("Agent {} activated in {}ms", agent_name, activation_time_ms);
        }
        
        Ok(result)
    }
    
    // Show activation info to CLI
    pub fn show_activation_info(&self, result: &ActivationResult) {
        println!("{}", "╔═══════════════════════════════════════════════════╗".blue());
        println!("{} {:<45} {}", "║".blue(), format!("Agent: {}", result.agent_name).green().bold(), "║".blue());
        println!("{}", "╠═══════════════════════════════════════════════════╣".blue());
        
        // Show task
        println!("{} {:<45} {}", "║".blue(), format!("Task: {}", result.task), "║".blue());
        
        // Show context if provided
        if let Some(ctx) = &result.context {
            println!("{} {:<45} {}", "║".blue(), format!("Context: {}", ctx), "║".blue());
        }
        
        // Show activation time
        println!("{} {:<45} {}", "║".blue(), format!("Activation time: {}ms", result.activation_time_ms), "║".blue());
        
        // Show memory statistics
        println!("{} {:<45} {}", "║".blue(), format!("Memory segments: {}", result.memory_segments.len()), "║".blue());
        
        // Count segments by type
        let mut segment_counts = std::collections::HashMap::new();
        for segment in &result.memory_segments {
            *segment_counts.entry(segment.memory_type).or_insert(0) += 1;
        }
        
        for (memory_type, count) in segment_counts {
            println!("{} {:<45} {}", "║".blue(), format!("  {:?}: {}", memory_type, count), "║".blue());
        }
        
        println!("{}", "╚═══════════════════════════════════════════════════╝".blue());
        
        // Protocol reminder
        if self.config.enforce_protocol {
            println!("\n{}", "Protocol Reminder:".yellow().bold());
            println!("  1. Begin your response with: [{}]:", result.agent_name.bold());
            println!("  2. End your response with: -- [{}]", result.agent_name.bold());
            println!("  3. Maintain the agent's personality and capabilities");
            println!("  4. Respond as {} would to the user's query", result.agent_name.bold());
        }
        
        // Show a prompt
        println!("\n{} is now active and ready.", result.agent_name.green().bold());
    }
    
    // Output formatted memory for Claude
    pub fn output_memory_for_claude(&self, result: &ActivationResult) -> Result<(), Box<dyn Error>> {
        // Output header marker to indicate agent activation
        println!("@[AGENT_ACTIVATION:{}]", result.agent_name);
        
        // First output all Tier 1 segments (critical identity)
        for segment in &result.memory_segments {
            if segment.tier == MemoryTier::Tier1 {
                println!("{}",  segment.content);
                println!("\n---\n");
            }
        }
        
        // Then output remaining segments in order of importance
        let mut remaining_segments: Vec<&MemorySegment> = result.memory_segments.iter()
            .filter(|s| s.tier != MemoryTier::Tier1)
            .collect();
        
        // Sort by tier and type
        remaining_segments.sort_by(|a, b| {
            if a.tier != b.tier {
                a.tier.cmp(&b.tier)
            } else {
                // Order by memory type for same tier
                // Priority: Procedural > Semantic > Relational > Episodic
                let type_priority = |memory_type: MemoryType| -> u8 {
                    match memory_type {
                        MemoryType::Procedural => 0,
                        MemoryType::Semantic => 1,
                        MemoryType::Relational => 2,
                        MemoryType::Episodic => 3,
                        _ => 4,
                    }
                };
                
                type_priority(a.memory_type).cmp(&type_priority(b.memory_type))
            }
        });
        
        // Output remaining segments
        for segment in remaining_segments {
            println!("{}",  segment.content);
            println!("\n---\n");
        }
        
        // Output footer marker
        println!("@[END_AGENT_ACTIVATION]");
        
        Ok(())
    }
    
    // Update terminal window title
    fn update_terminal_title(&self, agent_name: &str, task: &str) -> Result<(), Box<dyn Error>> {
        // For macOS Terminal and iTerm2
        print!("\x1B]0;Agent: {} - {}\x07", agent_name, task);
        
        // For Linux terminals
        print!("\x1B]2;Agent: {} - {}\x07", agent_name, task);
        
        // For Windows Command Prompt/PowerShell
        #[cfg(target_os = "windows")]
        {
            let title_cmd = format!("title Agent: {} - {}", agent_name, task);
            Command::new("cmd")
                .args(&["/C", &title_cmd])
                .output()?;
        }
        
        Ok(())
    }
    
    // Enforce agent protocol
    fn enforce_agent_protocol(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        // Simply remind about protocol - no need to execute a script
        if self.config.verbose {
            println!("Agent protocol reminder for: {}", agent_name);
            println!("1. Begin your response with: [{}]:", agent_name);
            println!("2. End your response with: -- [{}]", agent_name);
            println!("3. Maintain the agent's personality and capabilities");
            println!("4. Respond as {} would to the user's query", agent_name);
        }
        
        Ok(())
    }
    
    // List available agents
    pub fn list_agents(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let mut agents = Vec::new();
        
        // Check if agents directory exists
        if !self.config.agents_dir.exists() {
            return Err(format!("Agents directory not found: {}", self.config.agents_dir.display()).into());
        }
        
        // List agent directories
        for entry in fs::read_dir(&self.config.agents_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                let readme_path = path.join("README.md");
                let memory_path = path.join("MEMORY.md");
                
                // Check if directory contains agent files
                if readme_path.exists() || memory_path.exists() {
                    let agent_name = path.file_name()
                        .ok_or("Invalid agent directory name")?
                        .to_string_lossy()
                        .to_string();
                    
                    agents.push(agent_name);
                }
            }
        }
        
        // Sort agents alphabetically
        agents.sort();
        
        Ok(agents)
    }
    
    // Show agent information
    pub fn show_agent_info(&self, agent_name: &str) -> Result<(), Box<dyn Error>> {
        let agent_dir = self.config.agents_dir.join(agent_name);
        
        // Check if agent exists
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Err(format!("Agent not found: {}", agent_name).into());
        }
        
        println!("{}", "╔═══════════════════════════════════════════════════╗".blue());
        println!("{} {:<45} {}", "║".blue(), format!("Agent Information: {}", agent_name).green().bold(), "║".blue());
        println!("{}", "╠═══════════════════════════════════════════════════╣".blue());
        
        // Show agent path
        println!("{} {:<45} {}", "║".blue(), format!("Path: {}", agent_dir.display()), "║".blue());
        
        // Check README.md
        let readme_path = agent_dir.join("README.md");
        if readme_path.exists() {
            let readme_content = fs::read_to_string(&readme_path)?;
            
            // Extract agent description (simple algorithm)
            let mut description = String::new();
            let mut in_description = false;
            
            for line in readme_content.lines() {
                if line.starts_with("# ") {
                    in_description = true;
                    continue;
                }
                
                if in_description && line.starts_with("## ") {
                    break;
                }
                
                if in_description && !line.trim().is_empty() {
                    description.push_str(line);
                    description.push('\n');
                }
            }
            
            if !description.is_empty() {
                println!("{}", "╠═══════════════════════════════════════════════════╣".blue());
                println!("{} {:<45} {}", "║".blue(), "Description:".bold(), "║".blue());
                
                // Truncate and format description
                let desc_lines: Vec<&str> = description.lines().collect();
                for (i, line) in desc_lines.iter().take(3).enumerate() {
                    if i == 2 && desc_lines.len() > 3 {
                        println!("{} {:<45} {}", "║".blue(), "  ... (see README.md for full description)", "║".blue());
                    } else {
                        let formatted_line = if line.len() > 43 {
                            format!("  {}...", &line[0..40])
                        } else {
                            format!("  {}", line)
                        };
                        
                        println!("{} {:<45} {}", "║".blue(), formatted_line, "║".blue());
                    }
                }
            }
        }
        
        // Check memory files
        let memory_path = agent_dir.join("MEMORY.md");
        let learning_path = agent_dir.join("ContinuousLearning.md");
        
        println!("{}", "╠═══════════════════════════════════════════════════╣".blue());
        println!("{} {:<45} {}", "║".blue(), "Memory:".bold(), "║".blue());
        
        if memory_path.exists() {
            let memory_content = fs::read_to_string(&memory_path)?;
            let line_count = memory_content.lines().count();
            println!("{} {:<45} {}", "║".blue(), format!("  MEMORY.md: {} lines", line_count), "║".blue());
        } else {
            println!("{} {:<45} {}", "║".blue(), "  MEMORY.md: Not found", "║".blue());
        }
        
        if learning_path.exists() {
            let learning_content = fs::read_to_string(&learning_path)?;
            let line_count = learning_content.lines().count();
            println!("{} {:<45} {}", "║".blue(), format!("  ContinuousLearning.md: {} lines", line_count), "║".blue());
        } else {
            println!("{} {:<45} {}", "║".blue(), "  ContinuousLearning.md: Not found", "║".blue());
        }
        
        // Check sessions directory
        let sessions_dir = agent_dir.join("Sessions");
        
        println!("{}", "╠═══════════════════════════════════════════════════╣".blue());
        println!("{} {:<45} {}", "║".blue(), "Sessions:".bold(), "║".blue());
        
        if sessions_dir.exists() && sessions_dir.is_dir() {
            let session_count = fs::read_dir(&sessions_dir)?.filter(|e| e.is_ok() && e.as_ref().unwrap().path().is_dir()).count();
            println!("{} {:<45} {}", "║".blue(), format!("  Count: {}", session_count), "║".blue());
            
            // List recent sessions (up to 5)
            if session_count > 0 {
                let mut sessions: Vec<PathBuf> = fs::read_dir(&sessions_dir)?
                    .filter_map(|e| e.ok().map(|entry| entry.path()))
                    .filter(|p| p.is_dir())
                    .collect();
                
                // Sort by modification time (most recent first)
                sessions.sort_by(|a, b| {
                    let a_time = fs::metadata(a).and_then(|m| m.modified()).unwrap_or_else(|_| SystemTime::now());
                    let b_time = fs::metadata(b).and_then(|m| m.modified()).unwrap_or_else(|_| SystemTime::now());
                    b_time.cmp(&a_time)
                });
                
                println!("{} {:<45} {}", "║".blue(), "  Recent sessions:", "║".blue());
                
                for (i, session) in sessions.iter().take(5).enumerate() {
                    let session_name = session.file_name().unwrap_or_default().to_string_lossy();
                    println!("{} {:<45} {}", "║".blue(), format!("    {}: {}", i + 1, session_name), "║".blue());
                }
                
                if session_count > 5 {
                    println!("{} {:<45} {}", "║".blue(), format!("    ... and {} more", session_count - 5), "║".blue());
                }
            }
        } else {
            println!("{} {:<45} {}", "║".blue(), "  Sessions directory not found", "║".blue());
        }
        
        println!("{}", "╚═══════════════════════════════════════════════════╝".blue());
        
        Ok(())
    }
    
    // Show command help
    pub fn show_help() {
        println!("{}", "Agent Activator - Optimized agent activation for Collaborative Intelligence".bold());
        println!();
        println!("USAGE:");
        println!("    agent_activator [SUBCOMMAND] [OPTIONS]");
        println!();
        println!("SUBCOMMANDS:");
        println!("    activate    Activate an agent for the current session");
        println!("    list        List available agents");
        println!("    info        Show detailed information about an agent");
        println!("    help        Show this help message");
        println!();
        println!("OPTIONS:");
        println!("    --agents-dir <PATH>       Set custom agents directory");
        println!("    --cache-dir <PATH>        Set custom cache directory");
        println!("    --no-memory-optimization  Disable memory optimization");
        println!("    --no-parallel-loading     Disable parallel memory loading");
        println!("    --no-terminal-title       Disable terminal title updates");
        println!("    --no-protocol             Disable agent protocol enforcement");
        println!("    -v, --verbose             Enable verbose output");
        println!("    -h, --help                Show this help message");
        println!();
        println!("EXAMPLES:");
        println!("    agent_activator activate Athena");
        println!("    agent_activator activate Debugger code");
        println!("    agent_activator list");
        println!("    agent_activator info Athena");
    }
}

// Command-line interface
pub fn main() -> Result<(), Box<dyn Error>> {
    // Parse command-line arguments
    let matches = App::new("Agent Activator")
        .version("1.0.0")
        .author("Collaborative Intelligence Team")
        .about("Optimized agent activation for Collaborative Intelligence")
        .arg(Arg::with_name("agents-dir")
            .long("agents-dir")
            .value_name("PATH")
            .help("Set custom agents directory")
            .takes_value(true))
        .arg(Arg::with_name("cache-dir")
            .long("cache-dir")
            .value_name("PATH")
            .help("Set custom cache directory")
            .takes_value(true))
        .arg(Arg::with_name("no-memory-optimization")
            .long("no-memory-optimization")
            .help("Disable memory optimization"))
        .arg(Arg::with_name("no-parallel-loading")
            .long("no-parallel-loading")
            .help("Disable parallel memory loading"))
        .arg(Arg::with_name("no-terminal-title")
            .long("no-terminal-title")
            .help("Disable terminal title updates"))
        .arg(Arg::with_name("no-protocol")
            .long("no-protocol")
            .help("Disable agent protocol enforcement"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable verbose output"))
        .subcommand(SubCommand::with_name("activate")
            .about("Activate an agent for the current session")
            .arg(Arg::with_name("AGENT")
                .help("Agent name to activate")
                .required(true)
                .index(1))
            .arg(Arg::with_name("CONTEXT")
                .help("Optional context for activation")
                .index(2)))
        .subcommand(SubCommand::with_name("list")
            .about("List available agents"))
        .subcommand(SubCommand::with_name("info")
            .about("Show detailed information about an agent")
            .arg(Arg::with_name("AGENT")
                .help("Agent name to show information about")
                .required(true)
                .index(1)))
        .get_matches();
    
    // Create configuration from command-line arguments
    let mut config = AgentActivatorConfig::default();
    
    // Override config with command-line arguments
    if let Some(agents_dir) = matches.value_of("agents-dir") {
        config.agents_dir = PathBuf::from(agents_dir);
    }
    
    if let Some(cache_dir) = matches.value_of("cache-dir") {
        config.cache_dir = PathBuf::from(cache_dir);
    }
    
    if matches.is_present("no-memory-optimization") {
        config.memory_optimization = false;
    }
    
    if matches.is_present("no-parallel-loading") {
        config.parallel_loading = false;
    }
    
    if matches.is_present("no-terminal-title") {
        config.update_terminal_title = false;
    }
    
    if matches.is_present("no-protocol") {
        config.enforce_protocol = false;
    }
    
    if matches.is_present("verbose") {
        config.verbose = true;
    }
    
    // Create agent activator
    let activator = AgentActivator::with_config(config)?;
    
    // Process subcommands
    match matches.subcommand() {
        ("activate", Some(sub_matches)) => {
            let agent_name = sub_matches.value_of("AGENT").unwrap();
            let context = sub_matches.value_of("CONTEXT");
            let task = context.unwrap_or("General Task");
            
            match activator.activate_agent(agent_name, context, task) {
                Ok(result) => {
                    activator.show_activation_info(&result);
                    activator.output_memory_for_claude(&result)?;
                },
                Err(e) => {
                    eprintln!("Error: Failed to activate agent {}: {}", agent_name, e);
                    return Err(e);
                }
            }
        },
        ("list", _) => {
            match activator.list_agents() {
                Ok(agents) => {
                    println!("{}", "Available Agents:".bold());
                    for (i, agent) in agents.iter().enumerate() {
                        println!("  {}. {}", i + 1, agent);
                    }
                    println!("\nTotal agents: {}", agents.len());
                },
                Err(e) => {
                    eprintln!("Error: Failed to list agents: {}", e);
                    return Err(e);
                }
            }
        },
        ("info", Some(sub_matches)) => {
            let agent_name = sub_matches.value_of("AGENT").unwrap();
            
            if let Err(e) = activator.show_agent_info(agent_name) {
                eprintln!("Error: Failed to show agent information: {}", e);
                return Err(e);
            }
        },
        _ => {
            // Show help by default
            AgentActivator::show_help();
        }
    }
    
    Ok(())
}