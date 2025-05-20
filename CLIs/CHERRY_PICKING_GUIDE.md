# Cherry-Picking Guide for CLI Implementation

This guide identifies specific code elements from the CollaborativeIntelligence repository that should be cherry-picked into the main ../CI implementation. It includes styling elements, core functionality, and components from both active and deprecated implementations.

## Terminal Styling and Output Formatting

```rust
// From: CollaborativeIntelligence/CLIs/ci/bash/lib/output.sh (to be converted to Rust)

// ANSI color constants
const RED: &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const YELLOW: &str = "\u{001b}[33m";
const BLUE: &str = "\u{001b}[34m";
const MAGENTA: &str = "\u{001b}[35m";
const CYAN: &str = "\u{001b}[36m";
const BOLD: &str = "\u{001b}[1m";
const RESET: &str = "\u{001b}[0m";

// Output formatting functions
pub fn print_success(message: &str) {
    if is_interactive_terminal() {
        println!("{}✓ {}{}", GREEN, message, RESET);
    } else {
        println!("SUCCESS: {}", message);
    }
}

pub fn print_error(message: &str) {
    if is_interactive_terminal() {
        eprintln!("{}✗ {}{}", RED, message, RESET);
    } else {
        eprintln!("ERROR: {}", message);
    }
}

pub fn print_warning(message: &str) {
    if is_interactive_terminal() {
        println!("{}⚠ {}{}", YELLOW, message, RESET);
    } else {
        println!("WARNING: {}", message);
    }
}

pub fn print_step(step: usize, total: usize, message: &str) {
    if is_interactive_terminal() {
        println!("{}[STEP {}/{}]{} {}", BOLD, step, total, RESET, message);
    } else {
        println!("[STEP {}/{}] {}", step, total, message);
    }
}

// Agent formatting
pub fn format_agent_response(agent_name: &str, message: &str) -> String {
    let agent_upper = agent_name.to_uppercase();
    format!("[{}]: {}\n\n-- [{}]", agent_upper, message, agent_upper)
}

// Terminal title management
pub fn update_terminal_title(title: &str) {
    if is_interactive_terminal() {
        print!("\u{001b}]0;{}\u{0007}", title);
    }
}

// Terminal type detection
fn is_interactive_terminal() -> bool {
    std::env::var("TERM").is_ok() && atty::is(atty::Stream::Stdout)
}
```

## Memory Management System

### 1. Memory Segment Parser

```rust
// From: CollaborativeIntelligence/scripts/memory_segment_parser.rs

pub struct MemorySegment {
    pub id: String,
    pub content: String,
    pub domains: Vec<String>,
    pub source_file: PathBuf,
    pub last_modified: SystemTime,
}

pub struct MemorySegmentParser {
    pub segment_regex: Regex,
    pub domain_regex: Regex,
}

impl MemorySegmentParser {
    pub fn new() -> Self {
        // Pattern for Markdown headers with optional ID and domains
        let segment_regex = Regex::new(r"(?m)^#+\s+(.*?)(?:\s+\{#([\w-]+)(?:\s+domains=([\w,\s-]+))?\})?$").unwrap();
        let domain_regex = Regex::new(r"[\w-]+").unwrap();
        
        MemorySegmentParser {
            segment_regex,
            domain_regex,
        }
    }
    
    pub fn parse_file(&self, file_path: &Path) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        // Implementation details
    }
}
```

### 2. Checksum Cache System

```rust
// From: CollaborativeIntelligence/scripts/memory_checksum_cache.rs

pub struct ChecksumCache {
    cache_dir: PathBuf,
    checksums: HashMap<PathBuf, HashMap<String, String>>,
    validation_interval: Duration,
    last_validation: SystemTime,
    // Additional fields
}

impl ChecksumCache {
    pub fn new(config: ChecksumCacheConfig) -> Result<Self, Box<dyn Error>> {
        // Implementation details
    }
    
    pub fn validate_agent_memory(&mut self, agent_name: &str, base_dir: &Path) -> Result<HashMap<PathBuf, Vec<(MemorySegment, ChangeType)>>, Box<dyn Error>> {
        // Implementation details
    }
}
```

### 3. Memory Loader

```rust
// From: CollaborativeIntelligence/scripts/memory_loader.rs

pub struct MemoryLoader {
    base_dir: PathBuf,
    parser: MemorySegmentParser,
    // Additional fields
}

impl MemoryLoader {
    pub fn new(base_dir: &Path) -> Result<Self, Box<dyn Error>> {
        // Implementation details
    }
    
    pub fn load_agent_memory(&self, agent_name: &str, context: Option<String>) -> Result<Vec<MemorySegment>, Box<dyn Error>> {
        // Implementation details
    }
}
```

## Agent Cache System

```rust
// From: CollaborativeIntelligence/agent_cache/src/cache_manager.rs

pub struct AgentCacheManager {
    cache_dir: PathBuf,
    entries: HashMap<String, AgentCacheEntry>,
    persistence_interval: Duration,
    last_persist: Instant,
}

impl AgentCacheManager {
    pub fn new(cache_dir: &Path, persistence_interval_seconds: u64) -> Result<Self, AgentCacheError> {
        // Implementation details
    }
    
    pub fn create_agent_cache(&mut self, agent_name: &str, directory: &str, initializer: &str, intent: &str) -> Result<String, AgentCacheError> {
        // Implementation details
    }
    
    pub fn get_active_agents(&self) -> Vec<String> {
        // Implementation details
    }
}
```

## Knowledge Registry CLI

```rust
// From: CollaborativeIntelligence/KnowledgeRegistry/search_cli.sh (to be converted to Rust)

// Proposed Rust Structure:

pub struct KnowledgeSearch {
    registry_dir: PathBuf,
    index_path: PathBuf,
}

impl KnowledgeSearch {
    pub fn new(registry_dir: &Path) -> Self {
        // Implementation details
    }
    
    pub fn search_by_query(&self, query: &str) -> Result<Vec<KnowledgeItem>, Box<dyn Error>> {
        // Implementation details
    }
    
    pub fn search_by_category(&self, category: &str) -> Result<Vec<KnowledgeItem>, Box<dyn Error>> {
        // Implementation details
    }
    
    pub fn search_by_tag(&self, tag: &str) -> Result<Vec<KnowledgeItem>, Box<dyn Error>> {
        // Implementation details
    }
    
    pub fn get_recent(&self, count: usize) -> Result<Vec<KnowledgeItem>, Box<dyn Error>> {
        // Implementation details
    }
}
```

## Legacy Elements to Preserve

Several valuable components exist in deprecated or legacy code that should be preserved:

```rust
// From: CollaborativeIntelligence/ci-rust-minimal/legacy/CI.refactored

// Command handler pattern for consistent error handling
fn run_command<F>(command: &str, handler: F) -> Result<(), Box<dyn Error>>
where
    F: FnOnce() -> Result<(), Box<dyn Error>>,
{
    match handler() {
        Ok(_) => {
            println!("✓ Command '{}' completed successfully", command);
            Ok(())
        }
        Err(e) => {
            eprintln!("✗ Command '{}' failed: {}", command, e);
            Err(e)
        }
    }
}

// Legacy command adapter
fn handle_legacy_command(args: &[String]) -> Result<(), Box<dyn Error>> {
    let legacy_script = find_legacy_script(&args[0])?;
    let status = Command::new(legacy_script)
        .args(&args[1..])
        .status()?;
    
    if status.success() {
        Ok(())
    } else {
        Err(anyhow!("Legacy command failed with status: {}", status))
    }
}
```

```bash
# From: scripts/legacy/memory-loader.sh

# Fast memory loading with caching
load_agent_memory() {
  local AGENT_NAME="$1"
  local CACHE_FILE="${CACHE_DIR}/${AGENT_NAME}_memory.cache"
  
  # Check if cache exists and is recent
  if [[ -f "$CACHE_FILE" && $(find "$CACHE_FILE" -mmin -60) ]]; then
    cat "$CACHE_FILE"
    return 0
  fi
  
  # Cache doesn't exist or is stale, regenerate
  mkdir -p "$CACHE_DIR"
  parse_memory_files "$AGENT_NAME" > "$CACHE_FILE"
  cat "$CACHE_FILE"
}
```

## Integration Patterns

### 1. Command Registration

```rust
// Proposed integration in ../CI/src/main.rs

fn register_commands(app: clap::Command) -> clap::Command {
    app.subcommand(
        Command::new("intelligence")
            .about("Intelligence system commands")
            .subcommand(
                Command::new("memory")
                    .about("Memory management commands")
                    .subcommand(
                        Command::new("load")
                            .about("Load agent memory")
                            .arg(arg!(-a --agent <NAME> "Agent name").required(true))
                            .arg(arg!(-c --context <CONTEXT> "Optional context"))
                    )
            )
            .subcommand(
                Command::new("agent")
                    .about("Agent management commands")
                    .subcommand(
                        Command::new("activate")
                            .about("Activate an agent")
                            .arg(arg!(-n --name <NAME> "Agent name").required(true))
                            .arg(arg!(-d --directory <DIR> "Working directory").required(true))
                            .arg(arg!(-i --intent <INTENT> "Intent description"))
                    )
            )
            .subcommand(
                Command::new("knowledge")
                    .about("Knowledge registry commands")
                    .subcommand(
                        Command::new("search")
                            .about("Search knowledge registry")
                            .arg(arg!(-q --query <QUERY> "Search query"))
                            .arg(arg!(-c --category <CATEGORY> "Category filter"))
                            .arg(arg!(-t --tag <TAG> "Tag filter"))
                    )
            )
    )
}
```

### 2. Command Handlers

```rust
// Proposed integration in ../CI/src/handlers/intelligence.rs

pub fn handle_memory_load(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = args.get_one::<String>("agent").unwrap();
    let context = args.get_one::<String>("context").map(|s| s.as_str());
    
    let memory_loader = MemoryLoader::new(&get_ci_path()?)?;
    let segments = memory_loader.load_agent_memory(agent_name, context.map(|s| s.to_string()))?;
    
    // Format and display the segments
    
    Ok(())
}

pub fn handle_agent_activate(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let agent_name = args.get_one::<String>("name").unwrap();
    let directory = args.get_one::<String>("directory").unwrap();
    let intent = args.get_one::<String>("intent").unwrap_or(&"No specific intent provided".to_string());
    
    let mut cache_manager = AgentCacheManager::new(&get_cache_dir()?, 60)?;
    let id = cache_manager.create_agent_cache(agent_name, directory, "CI", intent)?;
    
    println!("✓ Agent {} activated (ID: {})", agent_name, id);
    
    Ok(())
}

pub fn handle_knowledge_search(args: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let query = args.get_one::<String>("query");
    let category = args.get_one::<String>("category");
    let tag = args.get_one::<String>("tag");
    
    let knowledge_search = KnowledgeSearch::new(&get_registry_dir()?);
    
    let results = if let Some(q) = query {
        knowledge_search.search_by_query(q)?
    } else if let Some(c) = category {
        knowledge_search.search_by_category(c)?
    } else if let Some(t) = tag {
        knowledge_search.search_by_tag(t)?
    } else {
        knowledge_search.get_recent(5)?
    };
    
    // Format and display the results
    
    Ok(())
}
```

## Configuration Integration

```rust
// From: CollaborativeIntelligence/ci-rust-minimal/src/main.rs

fn get_ci_path() -> Result<PathBuf> {
    // Try to get CI path from environment variable
    if let Ok(path) = env::var("CI_REPO_PATH") {
        let ci_path = PathBuf::from(path);
        if ci_path.exists() {
            return Ok(ci_path);
        }
    }
    
    // Try to find in common locations
    let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
    let common_paths = vec![
        home_dir.join("Documents/Projects/CollaborativeIntelligence"),
        home_dir.join("Projects/CollaborativeIntelligence"),
        home_dir.join("CollaborativeIntelligence"),
        PathBuf::from("/usr/local/share/CollaborativeIntelligence"),
    ];
    
    for path in common_paths {
        if path.exists() && path.join("CLAUDE.md").exists() {
            return Ok(path);
        }
    }
    
    // Failed to find the CI path
    Err(anyhow!("CI repository path not found"))
}
```

## Implementation Steps

1. Copy these code snippets into the ../CI codebase
2. Integrate them with the existing command structure
3. Ensure proper error handling and consistent user experience
4. Test the integration to verify functionality
5. Update documentation to reflect the new capabilities

By following this guide, you can effectively cherry-pick the most valuable components from the CollaborativeIntelligence repository into the main ../CI implementation.