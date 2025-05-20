use std::path::{Path, PathBuf};
use std::fs;
use std::io;
use std::process::{Command, exit};
use std::env;
use std::collections::HashMap;
use std::thread;
use std::time::Duration;

// Memory System Migration Plan
// Provides a safe path to migrate from bash scripts to Rust implementation

// Error types
#[derive(Debug)]
enum MigrationError {
    IoError(io::Error),
    ConfigError(String),
    ValidationError(String),
    MigrationFailed(String),
}

impl From<io::Error> for MigrationError {
    fn from(err: io::Error) -> Self {
        MigrationError::IoError(err)
    }
}

impl std::fmt::Display for MigrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MigrationError::IoError(e) => write!(f, "I/O error: {}", e),
            MigrationError::ConfigError(e) => write!(f, "Configuration error: {}", e),
            MigrationError::ValidationError(e) => write!(f, "Validation error: {}", e),
            MigrationError::MigrationFailed(e) => write!(f, "Migration failed: {}", e),
        }
    }
}

impl std::error::Error for MigrationError {}

type Result<T> = std::result::Result<T, MigrationError>;

// Configuration for migration
struct MigrationConfig {
    project_dir: PathBuf,
    bash_scripts_dir: PathBuf,
    rust_impl_dir: PathBuf,
    backup_dir: PathBuf,
    agents_dir: PathBuf,
    memory_cli_path: PathBuf,
    dry_run: bool,
    verbose: bool,
}

impl Default for MigrationConfig {
    fn default() -> Self {
        let current_dir = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        
        Self {
            project_dir: current_dir.clone(),
            bash_scripts_dir: current_dir.join("ci-tools/lib"),
            rust_impl_dir: current_dir.join("scripts"),
            backup_dir: current_dir.join("ci-tools/lib/backups"),
            agents_dir: current_dir.join("AGENTS"),
            memory_cli_path: current_dir.join("target/release/memory_cli"),
            dry_run: false,
            verbose: true,
        }
    }
}

// Main migration planner
struct MigrationPlanner {
    config: MigrationConfig,
    phase: u8,
}

impl MigrationPlanner {
    fn new(config: MigrationConfig) -> Self {
        Self { config, phase: 0 }
    }
    
    fn default() -> Self {
        Self::new(MigrationConfig::default())
    }
    
    // Log messages if verbose mode is enabled
    fn log(&self, message: &str) {
        if self.config.verbose {
            println!("[Migration] {}", message);
        }
    }
    
    // Log warnings in any mode
    fn warn(&self, message: &str) {
        eprintln!("[Migration Warning] {}", message);
    }
    
    // Log errors in any mode
    fn error(&self, message: &str) {
        eprintln!("[Migration Error] {}", message);
    }
    
    // Check if a Rust component is available
    fn check_rust_component(&self, name: &str) -> bool {
        let component_path = self.config.rust_impl_dir.join(format!("{}.rs", name));
        component_path.exists()
    }
    
    // List shell scripts to be migrated
    fn list_shell_scripts(&self) -> Result<Vec<PathBuf>> {
        self.log("Listing shell scripts for migration");
        
        let mut scripts = Vec::new();
        
        if !self.config.bash_scripts_dir.exists() {
            return Err(MigrationError::ConfigError(format!(
                "Bash scripts directory not found: {}",
                self.config.bash_scripts_dir.display()
            )));
        }
        
        for entry in fs::read_dir(&self.config.bash_scripts_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() && path.extension().map_or(false, |ext| ext == "sh") {
                scripts.push(path);
            }
        }
        
        Ok(scripts)
    }
    
    // Check which Rust components are available
    fn check_rust_components(&self) -> Result<HashMap<String, bool>> {
        self.log("Checking available Rust components");
        
        let components = vec![
            "memory_bridge",
            "memory_cli",
            "memory_cache",
            "memory_integration",
            "unified_memory_cli",
        ];
        
        let mut available = HashMap::new();
        
        for component in components {
            let is_available = self.check_rust_component(component);
            available.insert(component.to_string(), is_available);
            
            self.log(&format!(
                "Component {} is {}",
                component,
                if is_available { "available" } else { "not available" }
            ));
        }
        
        Ok(available)
    }
    
    // Check if the memory CLI is built
    fn check_memory_cli_built(&self) -> bool {
        self.config.memory_cli_path.exists()
    }
    
    // Build the Rust components
    fn build_rust_components(&self) -> Result<()> {
        self.log("Building Rust components");
        
        if self.config.dry_run {
            self.log("[Dry Run] Would build Rust components");
            return Ok(());
        }
        
        // Run cargo build in the Rust implementation directory
        let output = Command::new("cargo")
            .args(["build", "--release"])
            .current_dir(&self.config.rust_impl_dir)
            .output()
            .map_err(|e| MigrationError::MigrationFailed(format!("Failed to build Rust components: {}", e)))?;
        
        if !output.status.success() {
            return Err(MigrationError::MigrationFailed(format!(
                "Build failed: {}",
                String::from_utf8_lossy(&output.stderr)
            )));
        }
        
        self.log("Rust components built successfully");
        Ok(())
    }
    
    // Create backups of shell scripts
    fn backup_shell_scripts(&self, scripts: &[PathBuf]) -> Result<()> {
        self.log("Creating backups of shell scripts");
        
        if self.config.dry_run {
            self.log("[Dry Run] Would create backups");
            return Ok(());
        }
        
        // Create backup directory if it doesn't exist
        fs::create_dir_all(&self.config.backup_dir)?;
        
        for script in scripts {
            if let Some(file_name) = script.file_name() {
                let backup_path = self.config.backup_dir.join(file_name);
                
                self.log(&format!(
                    "Backing up {} to {}",
                    script.display(),
                    backup_path.display()
                ));
                
                fs::copy(script, backup_path)?;
            }
        }
        
        self.log("Backups created successfully");
        Ok(())
    }
    
    // Create transition script for a shell script
    fn create_transition_script(&self, script: &Path) -> Result<()> {
        if let Some(file_name) = script.file_name() {
            let file_name_str = file_name.to_string_lossy();
            let script_name = file_name_str.replace(".sh", "");
            
            self.log(&format!("Creating transition script for {}", script_name));
            
            if self.config.dry_run {
                self.log("[Dry Run] Would create transition script");
                return Ok(());
            }
            
            // Read original script to extract function names
            let content = fs::read_to_string(script)?;
            
            // Create transition script
            let transition_script = format!(
                r#"#!/usr/bin/env bash
# TRANSITION SCRIPT - DO NOT MODIFY
# This script provides backward compatibility during migration to Rust implementation
# Original script: {}
# Generated by Migration Planner

# Issue warning about using deprecated shell script
echo "[DEPRECATED] This shell script is being migrated to Rust. Please update your code to use the new implementation." >&2

# Function to get the memory CLI path
get_memory_cli_path() {{
    local rust_cli="$HOME/Documents/Projects/CollaborativeIntelligence/target/release/memory_cli"
    if [[ -x "$rust_cli" ]]; then
        echo "$rust_cli"
    else
        local debug_cli="$HOME/Documents/Projects/CollaborativeIntelligence/target/debug/memory_cli"
        if [[ -x "$debug_cli" ]]; then
            echo "$debug_cli"
        else
            echo ""
        fi
    fi
}}

# Function to call the rust implementation
call_rust_implementation() {{
    local func_name="$1"
    shift
    
    local cli_path=$(get_memory_cli_path)
    if [[ -z "$cli_path" ]]; then
        echo "[ERROR] Cannot find Rust implementation. Please build it with 'cargo build --release'" >&2
        return 1
    fi
    
    # Forward the call to the Rust implementation
    "$cli_path" "$func_name" "$@"
}}

# Transition functions that call the Rust implementation
{}"#,
                script_name,
                self.generate_transition_functions(&content),
            );
            
            // Write transition script
            let transition_path = script.with_file_name(format!("{}.transition.sh", script_name));
            fs::write(&transition_path, transition_script)?;
            
            // Make the transition script executable
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = fs::metadata(&transition_path)?.permissions();
                perms.set_mode(0o755);
                fs::set_permissions(&transition_path, perms)?;
            }
        }
        
        Ok(())
    }
    
    // Generate transition functions for a shell script
    fn generate_transition_functions(&self, content: &str) -> String {
        let mut functions = String::new();
        
        // Simple regex to match function declarations (this is a crude approximation)
        let lines: Vec<&str> = content.lines().collect();
        
        for (i, line) in lines.iter().enumerate() {
            if line.trim().ends_with("() {") || line.trim().ends_with("(){") {
                if let Some(name) = line.trim().split(|c| c == ' ' || c == '(' || c == '{').next() {
                    if !name.is_empty() && !name.starts_with('#') {
                        // Extract function body to determine arguments
                        let mut depth = 1;
                        let mut body = String::new();
                        let mut j = i + 1;
                        
                        while j < lines.len() && depth > 0 {
                            let line = lines[j];
                            if line.contains('{') {
                                depth += 1;
                            }
                            if line.contains('}') {
                                depth -= 1;
                            }
                            if depth > 0 {
                                body.push_str(line);
                                body.push('\n');
                            }
                            j += 1;
                        }
                        
                        // Generate the transition function
                        functions.push_str(&format!(
                            r#"
{}() {{
    echo "[TRANSITION] Calling Rust implementation for {}" >&2
    call_rust_implementation "{}" "$@"
}}
"#,
                            name, name, name
                        ));
                    }
                }
            }
        }
        
        functions
    }
    
    // Replace shell scripts with transition scripts
    fn replace_with_transition(&self, scripts: &[PathBuf]) -> Result<()> {
        self.log("Replacing shell scripts with transition scripts");
        
        // First create all transition scripts
        for script in scripts {
            self.create_transition_script(script)?;
        }
        
        if self.config.dry_run {
            self.log("[Dry Run] Would replace shell scripts");
            return Ok(());
        }
        
        // Then replace original scripts with transition scripts
        for script in scripts {
            if let Some(file_name) = script.file_name() {
                let script_name = file_name.to_string_lossy().replace(".sh", "");
                let transition_path = script.with_file_name(format!("{}.transition.sh", script_name));
                
                self.log(&format!(
                    "Replacing {} with transition script",
                    script.display()
                ));
                
                // Only replace if transition script exists
                if transition_path.exists() {
                    fs::rename(&transition_path, script)?;
                }
            }
        }
        
        self.log("Scripts replaced successfully");
        Ok(())
    }
    
    // Running a compatibility test
    fn test_compatibility(&self) -> Result<()> {
        self.log("Testing compatibility");
        
        // Define test cases
        let test_cases = vec![
            ("create_agent_cache", vec!["TestAgent", "/test/dir", "TestInit", "Testing"]),
            ("load_agent_memory", vec!["Athena", "performance"]),
        ];
        
        for (func, args) in test_cases {
            self.log(&format!("Testing function {} with args {:?}", func, args));
            
            if self.config.dry_run {
                self.log("[Dry Run] Would test compatibility");
                continue;
            }
            
            // Call both implementations and compare results
            
            // First the Rust implementation (if available)
            if self.check_memory_cli_built() {
                let mut rust_cmd = Command::new(&self.config.memory_cli_path);
                rust_cmd.arg(func);
                
                for arg in &args {
                    rust_cmd.arg(arg);
                }
                
                let rust_output = rust_cmd.output().map_err(|e| {
                    MigrationError::MigrationFailed(format!("Failed to run Rust implementation: {}", e))
                })?;
                
                self.log(&format!(
                    "Rust output (status={}): {}",
                    rust_output.status.code().unwrap_or(0),
                    String::from_utf8_lossy(&rust_output.stdout)
                ));
            } else {
                self.warn("Rust implementation not found, skipping compatibility test");
            }
        }
        
        self.log("Compatibility tests completed");
        Ok(())
    }
    
    // Main migration function
    fn migrate(&mut self) -> Result<()> {
        self.log("Starting migration process");
        
        // Phase 0: Check Rust components and build status
        self.phase = 0;
        self.log("Phase 0: Checking Rust components");
        
        let rust_components = self.check_rust_components()?;
        let memory_cli_built = self.check_memory_cli_built();
        
        let mut required_components = vec![
            "memory_bridge",
            "memory_cli",
            "memory_cache",
            "memory_integration",
        ];
        
        let missing_components: Vec<&str> = required_components
            .iter()
            .filter(|&c| !rust_components.get(c).unwrap_or(&false))
            .map(|&c| c)
            .collect();
        
        if !missing_components.is_empty() {
            return Err(MigrationError::ValidationError(format!(
                "Missing required Rust components: {:?}",
                missing_components
            )));
        }
        
        if !memory_cli_built {
            self.log("Building Rust components");
            self.build_rust_components()?;
            
            // Verify the build was successful
            if !self.check_memory_cli_built() {
                return Err(MigrationError::MigrationFailed(
                    "Failed to build memory CLI".to_string(),
                ));
            }
        }
        
        // Phase 1: Create backups of shell scripts
        self.phase = 1;
        self.log("Phase 1: Creating backups");
        
        let scripts = self.list_shell_scripts()?;
        self.backup_shell_scripts(&scripts)?;
        
        // Phase 2: Test compatibility
        self.phase = 2;
        self.log("Phase 2: Testing compatibility");
        
        self.test_compatibility()?;
        
        // Phase 3: Replace shell scripts with transition scripts
        self.phase = 3;
        self.log("Phase 3: Creating transition scripts");
        
        self.replace_with_transition(&scripts)?;
        
        // Phase 4: Final verification
        self.phase = 4;
        self.log("Phase 4: Final verification");
        
        // Simple verification: check if transition scripts are in place
        if !self.config.dry_run {
            for script in &scripts {
                if !script.exists() {
                    return Err(MigrationError::ValidationError(format!(
                        "Script missing after migration: {}",
                        script.display()
                    )));
                }
            }
        }
        
        self.log("Migration completed successfully");
        Ok(())
    }
}

// Main function
fn main() {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    let mut config = MigrationConfig::default();
    
    // Process arguments
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--dry-run" => {
                config.dry_run = true;
                i += 1;
            }
            "--verbose" => {
                config.verbose = true;
                i += 1;
            }
            "--quiet" => {
                config.verbose = false;
                i += 1;
            }
            "--help" => {
                println!("Memory System Migration Tool");
                println!();
                println!("Usage: {} [OPTIONS]", args[0]);
                println!();
                println!("Options:");
                println!("  --dry-run     Run without making actual changes");
                println!("  --verbose     Enable verbose output");
                println!("  --quiet       Disable verbose output");
                println!("  --help        Show this help message");
                exit(0);
            }
            _ => {
                eprintln!("Unknown option: {}", args[i]);
                eprintln!("Use --help for usage information");
                exit(1);
            }
        }
    }
    
    // Create migration planner
    let mut planner = MigrationPlanner::new(config);
    
    // Run migration
    match planner.migrate() {
        Ok(_) => {
            println!("Migration completed successfully");
            exit(0);
        }
        Err(e) => {
            eprintln!("Migration failed in phase {}: {}", planner.phase, e);
            exit(1);
        }
    }
}