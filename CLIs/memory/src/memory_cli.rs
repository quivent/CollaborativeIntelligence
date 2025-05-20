use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs;
use std::time::SystemTime;
use clap::{App, Arg, SubCommand, AppSettings};
use serde::{Serialize, Deserialize};
use serde_json;

// Import from our crate
mod agent_cache_system;
mod memory_checksum_cache;
mod memory_segment_parser;
mod memory_loader;
mod memory_updater;

use agent_cache_system::{AgentCacheManager, AgentCacheEntry, AgentStatus};
use memory_checksum_cache::{ChecksumCache, ChecksumCacheConfig};
use memory_segment_parser::{MemorySegmentParser, MemorySegment};
use memory_loader::MemoryLoader;

// CLI error type
#[derive(Debug)]
enum CliError {
    IoError(std::io::Error),
    SerdeError(serde_json::Error),
    ConfigError(String),
    ValidationError(String),
    MigrationError(String),
}

impl From<std::io::Error> for CliError {
    fn from(err: std::io::Error) -> Self {
        CliError::IoError(err)
    }
}

impl From<serde_json::Error> for CliError {
    fn from(err: serde_json::Error) -> Self {
        CliError::SerdeError(err)
    }
}

impl From<Box<dyn Error>> for CliError {
    fn from(err: Box<dyn Error>) -> Self {
        CliError::ValidationError(err.to_string())
    }
}

// Standard result type for CLI operations
type CliResult<T> = Result<T, CliError>;

// Output formats
#[derive(Debug, Clone, Copy)]
enum OutputFormat {
    Json,
    Text,
    Markdown,
}

// Response wrapper for consistent output
#[derive(Debug, Serialize, Deserialize)]
struct CliResponse<T> {
    success: bool,
    message: String,
    data: Option<T>,
    error: Option<String>,
    timestamp: u64,
}

impl<T: Serialize> CliResponse<T> {
    fn success(data: T, message: &str) -> Self {
        CliResponse {
            success: true,
            message: message.to_string(),
            data: Some(data),
            error: None,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn error(error: &str) -> Self {
        CliResponse {
            success: false,
            message: "Operation failed".to_string(),
            data: None,
            error: Some(error.to_string()),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }

    fn to_json(&self) -> CliResult<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }

    fn to_text(&self) -> String {
        if self.success {
            format!("SUCCESS: {}", self.message)
        } else {
            format!("ERROR: {}", self.error.as_ref().unwrap_or(&"Unknown error".to_string()))
        }
    }
}

// Main CLI handler
struct MemoryCli {
    cache_dir: PathBuf,
    format: OutputFormat,
    cache_manager: Option<AgentCacheManager>,
    checksum_cache: Option<ChecksumCache>,
    memory_loader: Option<MemoryLoader>,
}

impl MemoryCli {
    // Initialize CLI handler
    fn new(cache_dir: &Path, format: OutputFormat) -> Self {
        MemoryCli {
            cache_dir: cache_dir.to_path_buf(),
            format,
            cache_manager: None,
            checksum_cache: None,
            memory_loader: None,
        }
    }

    // Initialize the cache manager
    fn init_cache_manager(&mut self) -> CliResult<&mut AgentCacheManager> {
        if self.cache_manager.is_none() {
            let cache_dir = self.cache_dir.join("CACHE/SESSIONS");
            fs::create_dir_all(&cache_dir)?;
            
            let manager = AgentCacheManager::new(&cache_dir, 60)
                .map_err(|e| CliError::ConfigError(format!("Failed to initialize cache manager: {}", e)))?;
            
            self.cache_manager = Some(manager);
        }
        
        Ok(self.cache_manager.as_mut().unwrap())
    }

    // Initialize the checksum cache
    fn init_checksum_cache(&mut self) -> CliResult<&mut ChecksumCache> {
        if self.checksum_cache.is_none() {
            let config = ChecksumCacheConfig {
                cache_dir: self.cache_dir.join("CACHE/CHECKSUMS"),
                validation_interval_seconds: 60,
                enable_file_watcher: false,
                file_watcher_debounce_ms: 500,
                live_update: false,
                verbose_logging: false,
            };
            
            let cache = ChecksumCache::new(config)
                .map_err(|e| CliError::ConfigError(format!("Failed to initialize checksum cache: {}", e)))?;
            
            self.checksum_cache = Some(cache);
        }
        
        Ok(self.checksum_cache.as_mut().unwrap())
    }

    // Initialize the memory loader
    fn init_memory_loader(&mut self) -> CliResult<&mut MemoryLoader> {
        if self.memory_loader.is_none() {
            let loader = MemoryLoader::new(&self.cache_dir)
                .map_err(|e| CliError::ConfigError(format!("Failed to initialize memory loader: {}", e)))?;
            
            self.memory_loader = Some(loader);
        }
        
        Ok(self.memory_loader.as_mut().unwrap())
    }

    // Handle the 'cache' subcommand
    fn handle_cache(&mut self, agent_name: &str, directory: &str, initializer: &str, intent: &str) -> CliResult<String> {
        let manager = self.init_cache_manager()?;
        
        let entry_id = manager.create_agent_cache(agent_name, directory, initializer, intent)
            .map_err(|e| CliError::ValidationError(format!("Failed to create agent cache: {}", e)))?;
        
        manager.force_persist()
            .map_err(|e| CliError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e.to_string())))?;
        
        let response = CliResponse::success(entry_id, &format!("Cache entry created for agent {}", agent_name));
        
        match self.format {
            OutputFormat::Json => response.to_json(),
            OutputFormat::Text => Ok(response.to_text()),
            OutputFormat::Markdown => {
                let text = format!(
                    "# Agent Cache Created\n\n## Agent: {}\n## Intent: {}\n\nCache entry ID: {}",
                    agent_name, intent, entry_id
                );
                Ok(text)
            }
        }
    }

    // Handle the 'load' subcommand
    fn handle_load(&mut self, agent_name: &str, context: Option<&str>) -> CliResult<String> {
        let loader = self.init_memory_loader()?;
        
        let segments = loader.load_agent_memory(agent_name, context.map(|s| s.to_string()))
            .map_err(|e| CliError::ValidationError(format!("Failed to load agent memory: {}", e)))?;
        
        let response = CliResponse::success(segments.len(), &format!("Loaded {} memory segments for agent {}", segments.len(), agent_name));
        
        match self.format {
            OutputFormat::Json => {
                // Create a more detailed response with the actual segments
                let detailed_response = CliResponse::success(segments, &format!("Loaded memory for agent {}", agent_name));
                detailed_response.to_json()
            },
            OutputFormat::Text => Ok(response.to_text()),
            OutputFormat::Markdown => {
                let mut text = format!("# Agent Memory: {}\n\n", agent_name);
                
                // Group segments by type
                let mut core_segments = Vec::new();
                let mut learning_segments = Vec::new();
                let mut context_segments = Vec::new();
                
                for segment in &segments {
                    if segment.source_file.contains("MEMORY.md") {
                        core_segments.push(segment);
                    } else if segment.source_file.contains("ContinuousLearning.md") {
                        learning_segments.push(segment);
                    } else {
                        context_segments.push(segment);
                    }
                }
                
                // Core memory segments
                text.push_str("## Core Memory\n\n");
                for segment in core_segments {
                    text.push_str(&format!("### {}\n\n{}\n\n", segment.id, segment.content));
                }
                
                // Learning segments
                text.push_str("## Continuous Learning\n\n");
                for segment in learning_segments {
                    text.push_str(&format!("### {}\n\n{}\n\n", segment.id, segment.content));
                }
                
                // Context segments (if any)
                if !context_segments.is_empty() {
                    text.push_str("## Context-Specific Memory\n\n");
                    for segment in context_segments {
                        text.push_str(&format!("### {}\n\n{}\n\n", segment.id, segment.content));
                    }
                }
                
                Ok(text)
            }
        }
    }

    // Handle the 'validate' subcommand
    fn handle_validate(&mut self, agent_name: &str) -> CliResult<String> {
        let cache = self.init_checksum_cache()?;
        
        let base_dir = self.cache_dir.join("AGENTS");
        let changes = cache.validate_agent_memory(agent_name, &base_dir)
            .map_err(|e| CliError::ValidationError(format!("Failed to validate agent memory: {}", e)))?;
        
        let total_changes = changes.values().map(|v| v.len()).sum::<usize>();
        
        let response = CliResponse::success(
            total_changes,
            &format!("Validated memory files for agent {}, found {} changes", agent_name, total_changes)
        );
        
        match self.format {
            OutputFormat::Json => {
                // Create a more detailed response with the actual changes
                let detailed_response = CliResponse::success(
                    changes.keys().map(|k| k.to_string_lossy().to_string()).collect::<Vec<String>>(),
                    &format!("Validated memory files for agent {}", agent_name)
                );
                detailed_response.to_json()
            },
            OutputFormat::Text => Ok(response.to_text()),
            OutputFormat::Markdown => {
                let mut text = format!("# Memory Validation: {}\n\n", agent_name);
                
                if total_changes == 0 {
                    text.push_str("No changes detected in memory files.\n");
                } else {
                    text.push_str(&format!("Detected {} changes in {} files:\n\n", total_changes, changes.len()));
                    
                    for (file_path, file_changes) in &changes {
                        text.push_str(&format!("## {}\n\n", file_path.file_name().unwrap().to_string_lossy()));
                        
                        for (segment, change_type) in file_changes {
                            text.push_str(&format!("- **{}**: {} ({})\n", change_type, segment.id, segment.domains.join(", ")));
                        }
                        
                        text.push_str("\n");
                    }
                }
                
                Ok(text)
            }
        }
    }

    // Handle the 'status' subcommand
    fn handle_status(&mut self) -> CliResult<String> {
        let manager = self.init_cache_manager()?;
        
        let active_agents = manager.get_active_agents();
        
        let response = CliResponse::success(
            active_agents.clone(),
            &format!("Found {} active agents", active_agents.len())
        );
        
        match self.format {
            OutputFormat::Json => response.to_json(),
            OutputFormat::Text => Ok(response.to_text()),
            OutputFormat::Markdown => {
                let mut text = "# Agent Cache Status\n\n";
                
                if active_agents.is_empty() {
                    text.push_str("No active agents found.\n");
                } else {
                    text.push_str(&format!("## Active Agents ({})\n\n", active_agents.len()));
                    
                    for agent in &active_agents {
                        if let Some(entry) = manager.get_entry_for_agent(agent) {
                            text.push_str(&format!("### {}\n\n", agent));
                            text.push_str(&format!("- **Working Directory**: {}\n", entry.directory));
                            text.push_str(&format!("- **Initializer**: {}\n", entry.initializer));
                            text.push_str(&format!("- **Intent**: {}\n\n", entry.intent));
                        } else {
                            text.push_str(&format!("### {}\n\n", agent));
                            text.push_str("No detailed information available.\n\n");
                        }
                    }
                }
                
                Ok(text)
            }
        }
    }

    // Handle the 'migrate' subcommand
    fn handle_migrate(&mut self, force: bool) -> CliResult<String> {
        // This is a placeholder for the migration logic
        // In a real implementation, this would:
        // 1. Scan for existing cache files
        // 2. Convert them to the new database format
        // 3. Validate the migration
        
        let response = CliResponse::success(
            0,
            "Migration not implemented yet"
        );
        
        match self.format {
            OutputFormat::Json => response.to_json(),
            OutputFormat::Text => Ok(response.to_text()),
            OutputFormat::Markdown => {
                let text = "# Migration Status\n\nMigration functionality is not implemented yet.\n";
                Ok(text)
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // Define the command-line interface
    let matches = App::new("Memory CLI")
        .version("0.1.0")
        .author("Athena & Developer")
        .about("CLI for the Collaborative Intelligence memory system")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(
            Arg::with_name("format")
                .short("f")
                .long("format")
                .value_name("FORMAT")
                .help("Output format (json, text, markdown)")
                .takes_value(true)
                .possible_values(&["json", "text", "markdown"])
                .default_value("json")
        )
        .arg(
            Arg::with_name("cache-dir")
                .short("c")
                .long("cache-dir")
                .value_name("DIR")
                .help("Cache directory path")
                .takes_value(true)
                .default_value("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence")
        )
        .subcommand(
            SubCommand::with_name("cache")
                .about("Create or update an agent cache entry")
                .arg(
                    Arg::with_name("agent")
                        .short("a")
                        .long("agent")
                        .value_name("NAME")
                        .help("Agent name")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("directory")
                        .short("d")
                        .long("directory")
                        .value_name("PATH")
                        .help("Working directory")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("initializer")
                        .short("i")
                        .long("initializer")
                        .value_name("NAME")
                        .help("Initializer name")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("intent")
                        .short("t")
                        .long("intent")
                        .value_name("TEXT")
                        .help("Intent description")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("load")
                .about("Load agent memory")
                .arg(
                    Arg::with_name("agent")
                        .short("a")
                        .long("agent")
                        .value_name("NAME")
                        .help("Agent name")
                        .required(true)
                        .takes_value(true)
                )
                .arg(
                    Arg::with_name("context")
                        .short("x")
                        .long("context")
                        .value_name("TEXT")
                        .help("Context for memory loading")
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("validate")
                .about("Validate agent memory files")
                .arg(
                    Arg::with_name("agent")
                        .short("a")
                        .long("agent")
                        .value_name("NAME")
                        .help("Agent name")
                        .required(true)
                        .takes_value(true)
                )
        )
        .subcommand(
            SubCommand::with_name("status")
                .about("Show agent cache status")
        )
        .subcommand(
            SubCommand::with_name("migrate")
                .about("Migrate from old cache format to new format")
                .arg(
                    Arg::with_name("force")
                        .short("f")
                        .long("force")
                        .help("Force migration even if it may cause data loss")
                )
        )
        .get_matches();

    // Get common options
    let format = match matches.value_of("format").unwrap() {
        "json" => OutputFormat::Json,
        "text" => OutputFormat::Text,
        "markdown" => OutputFormat::Markdown,
        _ => OutputFormat::Json,
    };
    
    let cache_dir = matches.value_of("cache-dir").unwrap();
    
    // Initialize CLI handler
    let mut cli = MemoryCli::new(Path::new(cache_dir), format);
    
    // Process subcommands
    let result = match matches.subcommand() {
        ("cache", Some(sub_m)) => {
            let agent = sub_m.value_of("agent").unwrap();
            let directory = sub_m.value_of("directory").unwrap();
            let initializer = sub_m.value_of("initializer").unwrap();
            let intent = sub_m.value_of("intent").unwrap();
            
            cli.handle_cache(agent, directory, initializer, intent)
        },
        ("load", Some(sub_m)) => {
            let agent = sub_m.value_of("agent").unwrap();
            let context = sub_m.value_of("context");
            
            cli.handle_load(agent, context)
        },
        ("validate", Some(sub_m)) => {
            let agent = sub_m.value_of("agent").unwrap();
            
            cli.handle_validate(agent)
        },
        ("status", Some(_)) => {
            cli.handle_status()
        },
        ("migrate", Some(sub_m)) => {
            let force = sub_m.is_present("force");
            
            cli.handle_migrate(force)
        },
        _ => unreachable!(),
    };
    
    // Print the result
    match result {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        },
        Err(e) => {
            let error_response = match e {
                CliError::IoError(e) => CliResponse::<()>::error(&format!("I/O error: {}", e)),
                CliError::SerdeError(e) => CliResponse::<()>::error(&format!("Serialization error: {}", e)),
                CliError::ConfigError(e) => CliResponse::<()>::error(&format!("Configuration error: {}", e)),
                CliError::ValidationError(e) => CliResponse::<()>::error(&format!("Validation error: {}", e)),
                CliError::MigrationError(e) => CliResponse::<()>::error(&format!("Migration error: {}", e)),
            };
            
            match format {
                OutputFormat::Json => println!("{}", error_response.to_json().unwrap_or_else(|_| "{}".to_string())),
                OutputFormat::Text => println!("{}", error_response.to_text()),
                OutputFormat::Markdown => println!("# Error\n\n{}", error_response.error.unwrap_or_else(|| "Unknown error".to_string())),
            }
            
            Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "CLI operation failed")))
        }
    }
}