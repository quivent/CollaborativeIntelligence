use std::path::{Path, PathBuf};
use std::error::Error;
use std::fs;
use std::time::SystemTime;
use std::env;
use std::process::exit;
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json;

// Import memory bridge
mod memory_bridge;
use memory_bridge::{MemoryBridge, MemoryBridgeConfig, MemoryMode, MemoryBridgeError};

// Re-export other memory system components
mod agent_cache_system;
mod memory_checksum_cache;
mod memory_segment_parser;
mod memory_loader;
mod memory_updater;

// Output formats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Json,
    Text,
    Markdown,
}

impl OutputFormat {
    fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => OutputFormat::Json,
            "text" => OutputFormat::Text,
            "markdown" => OutputFormat::Markdown,
            _ => OutputFormat::Json,
        }
    }
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

    fn to_string(&self, format: OutputFormat) -> String {
        match format {
            OutputFormat::Json => {
                serde_json::to_string_pretty(self).unwrap_or_else(|_| "{}".to_string())
            }
            OutputFormat::Text => {
                if self.success {
                    format!("SUCCESS: {}", self.message)
                } else {
                    format!("ERROR: {}", self.error.as_ref().unwrap_or(&"Unknown error".to_string()))
                }
            }
            OutputFormat::Markdown => {
                if self.success {
                    format!("# Success\n\n{}\n\n", self.message)
                } else {
                    format!("# Error\n\n{}\n\n", self.error.as_ref().unwrap_or(&"Unknown error".to_string()))
                }
            }
        }
    }
}

struct CliApp {
    bridge: MemoryBridge,
    format: OutputFormat,
    project_dir: PathBuf,
}

impl CliApp {
    fn new(project_dir: &Path, format: OutputFormat, debug: bool) -> Self {
        let config = MemoryBridgeConfig {
            memory_cli_path: project_dir.join("target/release/memory_cli"),
            cache_dir: project_dir.to_path_buf(),
            db_path: project_dir.join("memory.db.json"),
            agents_dir: project_dir.join("AGENTS"),
            mode: MemoryMode::Dual,
            segmentation_enabled: true,
            segment_size: 4096,
            segment_overlap: 256,
            debug,
        };

        let bridge = MemoryBridge::new(config);

        Self {
            bridge,
            format,
            project_dir: project_dir.to_path_buf(),
        }
    }

    fn handle_cache(&self, agent_name: &str, directory: &str, initializer: &str, intent: &str) -> String {
        match self.bridge.create_agent_cache(agent_name, directory, initializer, intent) {
            Ok(entry_id) => {
                let response = CliResponse::success(
                    entry_id.clone(),
                    &format!("Cache entry created for agent {}", agent_name),
                );
                
                match self.format {
                    OutputFormat::Markdown => {
                        format!(
                            "# Agent Cache Created\n\n## Agent: {}\n## Intent: {}\n\nCache entry ID: {}",
                            agent_name, intent, entry_id
                        )
                    }
                    _ => response.to_string(self.format),
                }
            }
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_load(&self, agent_name: &str, context: Option<&str>, format: Option<&str>) -> String {
        let output_format = format.map(OutputFormat::from_str).unwrap_or(self.format);
        
        match self.bridge.load_agent_memory(agent_name, context, Some(&output_format.to_string().to_lowercase())) {
            Ok(memory) => memory,
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_validate(&self, agent_name: &str) -> String {
        match self.bridge.validate_agent_memory(agent_name, Some(&self.format.to_string().to_lowercase())) {
            Ok(result) => result,
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_status(&self) -> String {
        match self.bridge.get_memory_system_status(Some(&self.format.to_string().to_lowercase())) {
            Ok(result) => result,
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_migrate(&self, agent_name: Option<&str>, force: bool) -> String {
        match self.bridge.migrate_agent_memory(agent_name, force, Some(&self.format.to_string().to_lowercase())) {
            Ok(result) => result,
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_set_mode(&mut self, mode: &str) -> String {
        match self.bridge.set_memory_mode(mode) {
            Ok(_) => {
                let response = CliResponse::success(
                    format!("Memory mode set to: {}", mode),
                    &format!("Memory mode set to: {}", mode),
                );
                response.to_string(self.format)
            }
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_set_segmentation(&mut self, enabled: bool, size: Option<usize>, overlap: Option<usize>) -> String {
        match self.bridge.set_memory_segmentation(enabled, size, overlap) {
            Ok(_) => {
                let size_str = size.map_or("default".to_string(), |s| s.to_string());
                let overlap_str = overlap.map_or("default".to_string(), |o| o.to_string());
                
                let response = CliResponse::success(
                    format!("Segmentation: enabled={}, size={}, overlap={}", enabled, size_str, overlap_str),
                    &format!("Memory segmentation settings updated"),
                );
                response.to_string(self.format)
            }
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn handle_init_db(&self, force: bool) -> String {
        match self.bridge.init_database(force, Some(&self.format.to_string().to_lowercase())) {
            Ok(result) => result,
            Err(e) => {
                CliResponse::<String>::error(&format!("{}", e)).to_string(self.format)
            }
        }
    }

    fn print_usage() {
        println!("Memory CLI - Unified Memory System for Collaborative Intelligence");
        println!("");
        println!("USAGE:");
        println!("  memory_cli [OPTIONS] <COMMAND> [ARGS]");
        println!("");
        println!("OPTIONS:");
        println!("  -f, --format <FORMAT>    Output format: json, text, markdown [default: json]");
        println!("  -d, --debug              Enable debug output");
        println!("  -h, --help               Print help information");
        println!("");
        println!("COMMANDS:");
        println!("  cache                    Create or update an agent cache entry");
        println!("  load                     Load agent memory");
        println!("  validate                 Validate agent memory files");
        println!("  status                   Show agent cache status");
        println!("  migrate                  Migrate from old cache format to new format");
        println!("  set-mode                 Set memory operation mode (legacy, db, dual)");
        println!("  set-segmentation         Configure memory segmentation settings");
        println!("  init-db                  Initialize the database");
        println!("");
        println!("Run 'memory_cli <COMMAND> --help' for more information on a specific command.");
    }

    fn print_command_help(command: &str) {
        match command {
            "cache" => {
                println!("Create or update an agent cache entry");
                println!("");
                println!("USAGE:");
                println!("  memory_cli cache --agent <NAME> --directory <PATH> --initializer <NAME> --intent <TEXT>");
                println!("");
                println!("ARGS:");
                println!("  --agent <NAME>         Agent name");
                println!("  --directory <PATH>     Working directory");
                println!("  --initializer <NAME>   Initializer name");
                println!("  --intent <TEXT>        Intent description");
            }
            "load" => {
                println!("Load agent memory");
                println!("");
                println!("USAGE:");
                println!("  memory_cli load --agent <NAME> [--context <TEXT>]");
                println!("");
                println!("ARGS:");
                println!("  --agent <NAME>         Agent name");
                println!("  --context <TEXT>       Context for memory loading");
            }
            "validate" => {
                println!("Validate agent memory files");
                println!("");
                println!("USAGE:");
                println!("  memory_cli validate --agent <NAME>");
                println!("");
                println!("ARGS:");
                println!("  --agent <NAME>         Agent name");
            }
            "status" => {
                println!("Show agent cache status");
                println!("");
                println!("USAGE:");
                println!("  memory_cli status");
            }
            "migrate" => {
                println!("Migrate from old cache format to new format");
                println!("");
                println!("USAGE:");
                println!("  memory_cli migrate [--agent <NAME>] [--force]");
                println!("");
                println!("ARGS:");
                println!("  --agent <NAME>         Agent to migrate (all if not specified)");
                println!("  --force                Force migration even if it may cause data loss");
            }
            "set-mode" => {
                println!("Set memory operation mode");
                println!("");
                println!("USAGE:");
                println!("  memory_cli set-mode <MODE>");
                println!("");
                println!("ARGS:");
                println!("  <MODE>                 Mode: legacy, db, dual");
            }
            "set-segmentation" => {
                println!("Configure memory segmentation settings");
                println!("");
                println!("USAGE:");
                println!("  memory_cli set-segmentation <ENABLED> [--size <SIZE>] [--overlap <OVERLAP>]");
                println!("");
                println!("ARGS:");
                println!("  <ENABLED>              true or false");
                println!("  --size <SIZE>          Segment size (bytes)");
                println!("  --overlap <OVERLAP>    Segment overlap (bytes)");
            }
            "init-db" => {
                println!("Initialize the database");
                println!("");
                println!("USAGE:");
                println!("  memory_cli init-db [--force]");
                println!("");
                println!("ARGS:");
                println!("  --force                Force initialization even if database exists");
            }
            _ => {
                Self::print_usage();
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        CliApp::print_usage();
        exit(1);
    }

    // Parse global options
    let mut format = OutputFormat::Json;
    let mut debug = false;
    let mut command_index = 1;

    // Process global flags
    while command_index < args.len() {
        if args[command_index] == "-f" || args[command_index] == "--format" {
            if command_index + 1 < args.len() {
                format = match args[command_index + 1].as_str() {
                    "json" => OutputFormat::Json,
                    "text" => OutputFormat::Text,
                    "markdown" => OutputFormat::Markdown,
                    _ => {
                        eprintln!("ERROR: Invalid format: {}", args[command_index + 1]);
                        exit(1);
                    }
                };
                command_index += 2;
            } else {
                eprintln!("ERROR: Missing format value");
                exit(1);
            }
        } else if args[command_index] == "-d" || args[command_index] == "--debug" {
            debug = true;
            command_index += 1;
        } else if args[command_index] == "-h" || args[command_index] == "--help" {
            CliApp::print_usage();
            exit(0);
        } else {
            break;
        }
    }

    if command_index >= args.len() {
        CliApp::print_usage();
        exit(1);
    }

    // Get command and initialize CLI
    let command = &args[command_index];
    command_index += 1;

    // Check for command-specific help
    if command_index < args.len() && (args[command_index] == "-h" || args[command_index] == "--help") {
        CliApp::print_command_help(command);
        exit(0);
    }

    // Initialize current directory
    let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut app = CliApp::new(&current_dir, format, debug);

    // Process command
    let result = match command.as_str() {
        "cache" => {
            let mut agent_name = String::new();
            let mut directory = String::new();
            let mut initializer = String::new();
            let mut intent = String::new();

            let mut i = command_index;
            while i < args.len() {
                match args[i].as_str() {
                    "--agent" | "-a" => {
                        if i + 1 < args.len() {
                            agent_name = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing agent name");
                            exit(1);
                        }
                    }
                    "--directory" | "-d" => {
                        if i + 1 < args.len() {
                            directory = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing directory");
                            exit(1);
                        }
                    }
                    "--initializer" | "-i" => {
                        if i + 1 < args.len() {
                            initializer = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing initializer");
                            exit(1);
                        }
                    }
                    "--intent" | "-t" => {
                        if i + 1 < args.len() {
                            intent = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing intent");
                            exit(1);
                        }
                    }
                    _ => {
                        eprintln!("ERROR: Unknown argument: {}", args[i]);
                        exit(1);
                    }
                }
            }

            if agent_name.is_empty() || directory.is_empty() || initializer.is_empty() || intent.is_empty() {
                eprintln!("ERROR: Missing required arguments");
                CliApp::print_command_help("cache");
                exit(1);
            }

            app.handle_cache(&agent_name, &directory, &initializer, &intent)
        }
        "load" => {
            let mut agent_name = String::new();
            let mut context = None;
            let mut format_override = None;

            let mut i = command_index;
            while i < args.len() {
                match args[i].as_str() {
                    "--agent" | "-a" => {
                        if i + 1 < args.len() {
                            agent_name = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing agent name");
                            exit(1);
                        }
                    }
                    "--context" | "-c" => {
                        if i + 1 < args.len() {
                            context = Some(args[i + 1].clone());
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing context");
                            exit(1);
                        }
                    }
                    "--format" | "-f" => {
                        if i + 1 < args.len() {
                            format_override = Some(args[i + 1].clone());
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing format");
                            exit(1);
                        }
                    }
                    _ => {
                        eprintln!("ERROR: Unknown argument: {}", args[i]);
                        exit(1);
                    }
                }
            }

            if agent_name.is_empty() {
                eprintln!("ERROR: Missing required arguments");
                CliApp::print_command_help("load");
                exit(1);
            }

            app.handle_load(&agent_name, context.as_deref(), format_override.as_deref())
        }
        "validate" => {
            let mut agent_name = String::new();

            let mut i = command_index;
            while i < args.len() {
                match args[i].as_str() {
                    "--agent" | "-a" => {
                        if i + 1 < args.len() {
                            agent_name = args[i + 1].clone();
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing agent name");
                            exit(1);
                        }
                    }
                    _ => {
                        eprintln!("ERROR: Unknown argument: {}", args[i]);
                        exit(1);
                    }
                }
            }

            if agent_name.is_empty() {
                eprintln!("ERROR: Missing required arguments");
                CliApp::print_command_help("validate");
                exit(1);
            }

            app.handle_validate(&agent_name)
        }
        "status" => {
            app.handle_status()
        }
        "migrate" => {
            let mut agent_name = None;
            let mut force = false;

            let mut i = command_index;
            while i < args.len() {
                match args[i].as_str() {
                    "--agent" | "-a" => {
                        if i + 1 < args.len() {
                            agent_name = Some(args[i + 1].clone());
                            i += 2;
                        } else {
                            eprintln!("ERROR: Missing agent name");
                            exit(1);
                        }
                    }
                    "--force" | "-f" => {
                        force = true;
                        i += 1;
                    }
                    _ => {
                        eprintln!("ERROR: Unknown argument: {}", args[i]);
                        exit(1);
                    }
                }
            }

            app.handle_migrate(agent_name.as_deref(), force)
        }
        "set-mode" => {
            if command_index >= args.len() {
                eprintln!("ERROR: Missing mode argument");
                CliApp::print_command_help("set-mode");
                exit(1);
            }

            let mode = &args[command_index];
            app.handle_set_mode(mode)
        }
        "set-segmentation" => {
            if command_index >= args.len() {
                eprintln!("ERROR: Missing enabled argument");
                CliApp::print_command_help("set-segmentation");
                exit(1);
            }

            let enabled = match args[command_index].as_str() {
                "true" => true,
                "false" => false,
                _ => {
                    eprintln!("ERROR: Invalid enabled value (must be 'true' or 'false')");
                    exit(1);
                }
            };
            command_index += 1;

            let mut size = None;
            let mut overlap = None;

            let mut i = command_index;
            while i < args.len() {
                match args[i].as_str() {
                    "--size" => {
                        if i + 1 < args.len() {
                            match args[i + 1].parse::<usize>() {
                                Ok(val) => {
                                    size = Some(val);
                                    i += 2;
                                }
                                Err(_) => {
                                    eprintln!("ERROR: Invalid size value");
                                    exit(1);
                                }
                            }
                        } else {
                            eprintln!("ERROR: Missing size value");
                            exit(1);
                        }
                    }
                    "--overlap" => {
                        if i + 1 < args.len() {
                            match args[i + 1].parse::<usize>() {
                                Ok(val) => {
                                    overlap = Some(val);
                                    i += 2;
                                }
                                Err(_) => {
                                    eprintln!("ERROR: Invalid overlap value");
                                    exit(1);
                                }
                            }
                        } else {
                            eprintln!("ERROR: Missing overlap value");
                            exit(1);
                        }
                    }
                    _ => {
                        eprintln!("ERROR: Unknown argument: {}", args[i]);
                        exit(1);
                    }
                }
            }

            app.handle_set_segmentation(enabled, size, overlap)
        }
        "init-db" => {
            let mut force = false;

            let mut i = command_index;
            while i < args.len() {
                match args[i].as_str() {
                    "--force" | "-f" => {
                        force = true;
                        i += 1;
                    }
                    _ => {
                        eprintln!("ERROR: Unknown argument: {}", args[i]);
                        exit(1);
                    }
                }
            }

            app.handle_init_db(force)
        }
        "help" => {
            if command_index < args.len() {
                CliApp::print_command_help(&args[command_index]);
            } else {
                CliApp::print_usage();
            }
            exit(0);
        }
        _ => {
            eprintln!("ERROR: Unknown command: {}", command);
            CliApp::print_usage();
            exit(1);
        }
    };

    println!("{}", result);
}