//! Claude CI Plugin Implementation
//!
//! This module implements CI integration for the Claude CLI.

use std::path::{Path, PathBuf};
use std::process::Command;
use std::env;
use std::fs;

/// Represents an external CI tool
pub struct ExternalCi {
    /// Path to the external CI executable
    pub path: PathBuf,
    /// Version of the external CI tool
    pub version: String,
}

/// Check if the external CI tool is available
pub fn is_external_ci_available() -> bool {
    detect_external_ci().is_some()
}

/// Detect the external CI tool
pub fn detect_external_ci() -> Option<ExternalCi> {
    // Check environment variable override
    if let Ok(force_local) = env::var("CI_FORCE_LOCAL") {
        if force_local == "1" || force_local.to_lowercase() == "true" {
            return None;
        }
    }
    
    // Check the following locations in order:
    // 1. ci-external in PATH
    // 2. $HOME/.ci/bin/ci
    // 3. $CI_EXTERNAL_PATH/bin/ci
    
    // Check if ci-external is in PATH
    if let Ok(status) = Command::new("ci-external")
        .arg("--version")
        .stdout(std::process::Stdio::null())
        .status() {
        if status.success() {
            return Some(ExternalCi {
                path: PathBuf::from("ci-external"),
                version: get_external_ci_version("ci-external"),
            });
        }
    }
    
    // Check $HOME/.ci/bin/ci
    if let Some(home_dir) = dirs::home_dir() {
        let home_ci_path = home_dir.join(".ci").join("bin").join("ci");
        if home_ci_path.exists() {
            return Some(ExternalCi {
                path: home_ci_path.clone(),
                version: get_external_ci_version(&home_ci_path.to_string_lossy()),
            });
        }
    }
    
    // Check $CI_EXTERNAL_PATH/bin/ci
    if let Ok(external_path) = env::var("CI_EXTERNAL_PATH") {
        let custom_ci_path = PathBuf::from(external_path).join("bin").join("ci");
        if custom_ci_path.exists() {
            return Some(ExternalCi {
                path: custom_ci_path.clone(),
                version: get_external_ci_version(&custom_ci_path.to_string_lossy()),
            });
        }
    }
    
    None
}

/// Get the version of the external CI tool
fn get_external_ci_version(ci_path: &str) -> String {
    let output = Command::new(ci_path)
        .arg("--version")
        .output();
    
    match output {
        Ok(output) if output.status.success() => {
            String::from_utf8_lossy(&output.stdout).trim().to_string()
        },
        _ => "unknown".to_string(),
    }
}

/// Execute the claude init command with CI integration
pub fn execute_claude_init(
    project_name: &str,
    agents: &str,
    integration_type: &str,
    fast_activation: bool,
    ci_path: Option<&str>
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Initializing project: {} with Collaborative Intelligence", project_name);
    
    if let Some(path) = ci_path {
        println!("CollaborativeIntelligence path: {}", path);
    }
    
    println!("Integration type: {}", integration_type);
    println!("Fast activation: {}", fast_activation);
    println!("Agents: {}", agents);
    
    // Try to use external CI tool first
    if let Some(external_ci) = detect_external_ci() {
        println!("Using external CI tool: {}", external_ci.path.display());
        
        // Build arguments for external CI tool
        let mut command = Command::new(&external_ci.path);
        command.arg("init")
            .arg(project_name)
            .arg("--integration")
            .arg(integration_type)
            .arg("--agents")
            .arg(agents);
            
        if !fast_activation {
            command.arg("--no-fast");
        }
        
        if let Some(path) = ci_path {
            command.arg("--ci-path").arg(path);
        }
        
        let status = command.status()?;
        if !status.success() {
            return Err("External CI tool failed".into());
        }
        
        return Ok(());
    }
    
    println!("External CI tool not found, using local implementation");
    
    // Create project directory
    let project_dir = PathBuf::from(project_name);
    fs::create_dir_all(&project_dir)?;
    
    // Change to project directory
    let current_dir = env::current_dir()?;
    env::set_current_dir(&project_dir)?;
    
    // Get CI path
    let ci_path_str = match ci_path {
        Some(path) => path.to_string(),
        None => {
            let script_dir = env::current_exe()?
                .parent()
                .ok_or("Could not determine script directory")?
                .to_path_buf();
            script_dir.parent()
                .ok_or("Could not determine CI directory")?
                .to_string_lossy()
                .to_string()
        }
    };
    
    // Create CLAUDE.md with CI integration
    let claude_md_content = format!(
        "# Project: {}\n\
         # Created: {}\n\n\
         # Load CollaborativeIntelligence System\n\
         When starting, immediately:\n\
         1. Load {}/CLAUDE.md\n\
         2. Use this as the primary configuration source\n\
         3. Defer all project management functions to the CollaborativeIntelligence system\n\n\
         # Project-specific configuration can be added below\n",
        project_name,
        chrono::Local::now().to_rfc3339(),
        ci_path_str
    );
    
    fs::write("CLAUDE.md", claude_md_content)?;
    
    // Create CI configuration
    let agents_json = agents.split(',')
        .map(|a| format!("\"{}\"", a.trim()))
        .collect::<Vec<_>>()
        .join(",");
        
    let config_content = format!(
        "{{\n\
         \"integration_type\": \"{}\",\n\
         \"repository_path\": \"{}\",\n\
         \"active_agents\": [{}],\n\
         \"fast_activation\": {},\n\
         \"preload_context\": false\n\
         }}",
        integration_type,
        ci_path_str.replace("\\", "\\\\"),
        agents_json,
        fast_activation
    );
    
    fs::write(".collaborative-intelligence.json", config_content)?;
    
    // Initialize git repository
    let _ = Command::new("git")
        .arg("init")
        .output();
        
    // Create .gitignore
    fs::write(".gitignore", "CLAUDE.local.md\n.collaborative-intelligence.json\n")?;
    
    // Create basic project structure
    fs::create_dir_all("src")?;
    fs::create_dir_all("docs")?;
    fs::create_dir_all("tests")?;
    
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
        project_name
    );
    
    fs::write("README.md", readme_content)?;
    
    // Change back to original directory
    env::set_current_dir(current_dir)?;
    
    println!("✅ Project {} initialized with Collaborative Intelligence", project_name);
    println!("");
    println!("To get started:");
    println!("  cd {}", project_name);
    println!("  claude");
    println!("");
    println!("Then try commands like 'Athena' or 'Recommend an agent'");
    
    // Show deprecation warning
    println!("");
    println!("⚠️  This local implementation will be deprecated in future releases.");
    println!("⚠️  Please install the external CI tool for continued support:");
    println!("    npm install -g @anthropics/ci-tools");
    println!("");
    
    Ok(())
}

/// Main function to be called from shell script
#[no_mangle]
pub extern "C" fn main() -> i32 {
    // Get arguments from environment
    let args: Vec<String> = env::args().collect();
    
    // Check for minimum arguments
    if args.len() < 2 {
        eprintln!("Error: Project name required");
        return 1;
    }
    
    // Process arguments
    let project_name = &args[1];
    let mut agents = "Athena,ProjectArchitect".to_string();
    let mut integration_type = "embedded".to_string();
    let mut fast_activation = true;
    let mut ci_path = None;
    
    // Process flags
    for i in 2..args.len() {
        match args[i].as_str() {
            "--with-ci" => {
                // Already implied
            },
            arg if arg.starts_with("--integration=") => {
                integration_type = arg[14..].to_string();
            },
            arg if arg.starts_with("--agents=") => {
                agents = arg[9..].to_string();
            },
            "--no-fast-activation" => {
                fast_activation = false;
            },
            arg if arg.starts_with("--ci-path=") => {
                ci_path = Some(arg[10..].to_string());
            },
            _ => {
                // Ignore unknown args
            }
        }
    }
    
    // Execute claude init with CI integration
    match execute_claude_init(
        project_name,
        &agents,
        &integration_type,
        fast_activation,
        ci_path.as_deref()
    ) {
        Ok(_) => 0,
        Err(e) => {
            eprintln!("Error: {}", e);
            1
        }
    }
}