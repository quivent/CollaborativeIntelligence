use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, exit};
use anyhow::{Result, Context, anyhow};

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return Ok(());
    }
    
    match args[1].as_str() {
        "init" => init_command(&args[2..])?,
        "integrate" => integrate_command(&args[2..])?,
        "status" => status_command(&args[2..])?,
        "fix" => fix_command(&args[2..])?,
        "verify" => verify_command(&args[2..])?,
        "load" => load_command(&args[2..])?,
        "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            exit(1);
        }
    }
    
    Ok(())
}

fn print_help() {
    println!("Usage: ci COMMAND [OPTIONS]");
    println!("");
    println!("Collaborative Intelligence command-line tool");
    println!("");
    println!("Commands:");
    println!("  init PROJECT_NAME    Initialize a project with Collaborative Intelligence");
    println!("  integrate [PATH]     Integrate CI into an existing project (default: current directory)");
    println!("  status [PATH]        Check CI integration status of a project (default: current directory)");
    println!("  fix [PATH] [TYPE]    Repair CI integration issues (default: current directory)");
    println!("  verify [PATH]        Verify CI integration is working properly (default: current directory)");
    println!("  load AGENT [--context=CONTEXT] [--path=PATH]  Load agent memory");
    println!("");
    println!("Options for 'init' and 'integrate':");
    println!("  --agents=LIST         Comma-separated list of agents (default: Athena,ProjectArchitect)");
    println!("  --integration=TYPE    Integration type: embedded, sibling, symlink (default: embedded)");
    println!("  --no-fast             Disable fast activation");
    println!("  --ci-path=PATH        Path to CI repository");
}

fn init_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        eprintln!("Error: Project name required");
        eprintln!("Usage: ci init PROJECT_NAME [OPTIONS]");
        exit(1);
    }
    
    let project_name = &args[0];
    let ci_path = get_ci_path()?;
    let agents = "Athena,ProjectArchitect"; // Default agents
    let integration_type = "embedded"; // Default integration type
    let fast_activation = true; // Default fast activation
    
    println!("Creating project: {}", project_name);
    println!("CI path: {}", ci_path.display());
    println!("Integration: {}", integration_type);
    println!("Agents: {}", agents);
    println!("Fast activation: {}", fast_activation);
    
    // Create project directory
    fs::create_dir_all(project_name)
        .with_context(|| format!("Failed to create project directory: {}", project_name))?;
    
    // Create CLAUDE.md with CI integration
    let claude_md_content = format!(
r#"# Project: {}
# Created: {}

# Load CollaborativeIntelligence System
When starting, immediately:
1. Load {}/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
"#, 
        project_name, 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), 
        ci_path.display()
    );
    
    let claude_md_path = Path::new(project_name).join("CLAUDE.md");
    fs::write(&claude_md_path, claude_md_content)
        .with_context(|| format!("Failed to write CLAUDE.md file: {}", claude_md_path.display()))?;
    
    // Create CI configuration
    // Convert comma-separated list to proper JSON array
    let agents_json = format!("[\"{}\"]", agents.replace(',', "\",\""));
    
    let config_content = format!(
r#"{{
  "integration_type": "{}",
  "repository_path": "{}",
  "active_agents": {},
  "fast_activation": {},
  "repository_context": "project",
  "home_repository_path": "{}",
  "allowed_operations": ["memory_read", "agent_consult", "project_development"]
}}"#, 
        integration_type, 
        ci_path.display(),
        agents_json,
        fast_activation,
        ci_path.display()
    );
    
    let config_path = Path::new(project_name).join(".collaborative-intelligence.json");
    fs::write(&config_path, config_content)
        .with_context(|| format!("Failed to write configuration file: {}", config_path.display()))?;
    
    // Initialize with basic structure
    fs::create_dir_all(Path::new(project_name).join("src"))?;
    fs::create_dir_all(Path::new(project_name).join("docs"))?;
    fs::create_dir_all(Path::new(project_name).join("tests"))?;
    
    // Try to initialize git repository
    let _ = Command::new("git")
        .args(&["init"])
        .current_dir(project_name)
        .output();
    
    // Create .gitignore
    let gitignore_content = r#".collaborative-intelligence.json
.env
"#;
    
    fs::write(Path::new(project_name).join(".gitignore"), gitignore_content)
        .with_context(|| format!("Failed to write .gitignore file"))?;
    
    // Create .env file with CI_REPO_PATH
    let env_content = format!("CI_REPO_PATH={}", ci_path.display());
    fs::write(Path::new(project_name).join(".env"), env_content)
        .with_context(|| format!("Failed to write .env file"))?;
    
    println!("✓ Project {} created with CI integration", project_name);
    println!("");
    println!("To get started:");
    println!("  cd {}", project_name);
    println!("  claude");
    println!("");
    println!("Then try: 'Athena' or 'Recommend an agent'");
    
    Ok(())
}

fn integrate_command(args: &[String]) -> Result<()> {
    let target_dir = if !args.is_empty() && !args[0].starts_with("--") {
        &args[0]
    } else {
        "."
    };
    
    let ci_path = get_ci_path()?;
    let agents = "Athena,ProjectArchitect"; // Default agents
    let integration_type = "embedded"; // Default integration type
    let fast_activation = true; // Default fast activation
    
    // Get absolute path to target directory
    let target_path = PathBuf::from(target_dir);
    if !target_path.exists() {
        return Err(anyhow!("Error: Target directory '{}' does not exist", target_dir));
    }
    
    // Get project name from directory
    let project_name = target_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");
    
    println!("Integrating CI into project: {}", project_name);
    println!("Target directory: {}", target_dir);
    println!("CI path: {}", ci_path.display());
    println!("Integration: {}", integration_type);
    println!("Agents: {}", agents);
    println!("Fast activation: {}", fast_activation);
    
    // Create CLAUDE.local.md with CI integration
    let claude_md_content = format!(
r#"# Project: {}
# Integrated: {}

# Load CollaborativeIntelligence System
When starting, immediately:
1. Load {}/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
"#, 
        project_name, 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), 
        ci_path.display()
    );
    
    let claude_md_path = target_path.join("CLAUDE.local.md");
    fs::write(&claude_md_path, claude_md_content)
        .with_context(|| format!("Failed to write CLAUDE.local.md file: {}", claude_md_path.display()))?;
    
    // Create CI configuration
    // Convert comma-separated list to proper JSON array
    let agents_json = format!("[\"{}\"]", agents.replace(',', "\",\""));
    
    let config_content = format!(
r#"{{
  "integration_type": "{}",
  "repository_path": "{}",
  "active_agents": {},
  "fast_activation": {},
  "repository_context": "project",
  "home_repository_path": "{}",
  "allowed_operations": ["memory_read", "agent_consult", "project_development"]
}}"#, 
        integration_type, 
        ci_path.display(),
        agents_json,
        fast_activation,
        ci_path.display()
    );
    
    let config_path = target_path.join(".collaborative-intelligence.json");
    fs::write(&config_path, config_content)
        .with_context(|| format!("Failed to write configuration file: {}", config_path.display()))?;
    
    // Add to gitignore if it exists
    let gitignore_path = target_path.join(".gitignore");
    if gitignore_path.exists() {
        let gitignore_content = fs::read_to_string(&gitignore_path)?;
        if !gitignore_content.contains(".collaborative-intelligence.json") {
            let new_content = format!("{}
.collaborative-intelligence.json", gitignore_content);
            fs::write(&gitignore_path, new_content)?;
        }
    } else {
        fs::write(&gitignore_path, ".collaborative-intelligence.json
")?;
    }
    
    // Create or update .env file with CI_REPO_PATH
    let env_path = target_path.join(".env");
    let env_content = format!("CI_REPO_PATH={}", ci_path.display());
    
    if env_path.exists() {
        let existing_env = fs::read_to_string(&env_path)?;
        if existing_env.contains("CI_REPO_PATH=") {
            // Replace existing CI_REPO_PATH
            let new_env = existing_env.lines()
                .map(|line| {
                    if line.starts_with("CI_REPO_PATH=") {
                        &env_content
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(&env_path, new_env)?;
        } else {
            // Add CI_REPO_PATH to existing .env file
            let new_env = format!("{}
{}", existing_env, env_content);
            fs::write(&env_path, new_env)?;
        }
    } else {
        // Create new .env file with CI_REPO_PATH
        fs::write(&env_path, env_content)?;
        
        // Add .env to gitignore if it's not already there
        if gitignore_path.exists() {
            let gitignore_content = fs::read_to_string(&gitignore_path)?;
            if !gitignore_content.contains(".env") {
                let new_content = format!("{}
.env", gitignore_content);
                fs::write(&gitignore_path, new_content)?;
            }
        }
    }
    
    println!("✓ CollaborativeIntelligence integrated into project: {}", project_name);
    println!("");
    println!("To get started:");
    println!("  cd {}", target_dir);
    println!("  claude");
    println!("");
    println!("Then try: 'Athena' or 'Recommend an agent'");
    
    Ok(())
}

fn status_command(args: &[String]) -> Result<()> {
    let target_dir = if !args.is_empty() && !args[0].starts_with("--") {
        &args[0]
    } else {
        "."
    };
    
    // Get absolute path to target directory
    let target_path = PathBuf::from(target_dir);
    if !target_path.exists() {
        return Err(anyhow!("Error: Target directory '{}' does not exist", target_dir));
    }
    
    // Get project name from directory
    let project_name = target_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");
    
    println!("Checking CI integration for project: {}", project_name);
    println!("Directory: {}", target_dir);
    
    // Check for CI integration files
    let config_path = target_path.join(".collaborative-intelligence.json");
    let claude_md_path = target_path.join("CLAUDE.md");
    let claude_local_md_path = target_path.join("CLAUDE.local.md");
    
    // Initialize status flags
    let config_exists = config_path.exists();
    let claude_exists = claude_md_path.exists() || claude_local_md_path.exists();
    
    // Display status
    println!("");
    println!("CI Integration Status:");
    println!("======================");
    
    if config_exists && claude_exists {
        // Read config file to get CI path
        let config_content = fs::read_to_string(&config_path)?;
        let config_ci_path = extract_path_from_json(&config_content, "repository_path")
            .unwrap_or_else(|| "Unknown".to_string());
        let config_agents = extract_agents_from_json(&config_content)
            .unwrap_or_else(|| "Unknown".to_string());
        let config_integration_type = extract_from_json(&config_content, "integration_type")
            .unwrap_or_else(|| "Unknown".to_string());
        let config_fast = extract_from_json(&config_content, "fast_activation")
            .unwrap_or_else(|| "Unknown".to_string());
        
        println!("✓ CI is integrated with this project");
        println!("");
        println!("Configuration File: {}", config_path.file_name().unwrap().to_string_lossy());
        println!("  - Repository Path: {}", config_ci_path);
        println!("  - Integration Type: {}", config_integration_type);
        println!("  - Active Agents: {}", config_agents);
        println!("  - Fast Activation: {}", config_fast);
        println!("");
        
        let claude_file = if claude_md_path.exists() {
            "CLAUDE.md"
        } else {
            "CLAUDE.local.md"
        };
        
        println!("Claude Configuration: {}", claude_file);
        
        // Verify the CI repository path exists
        let ci_path = Path::new(&config_ci_path);
        if ci_path.exists() {
            println!("  - CI Repository: ✓ Found");
        } else {
            println!("  - CI Repository: ✗ Not found at {}", config_ci_path);
        }
        
        // Check if the CI repository has a CLAUDE.md file
        let ci_claude_md = ci_path.join("CLAUDE.md");
        if ci_claude_md.exists() {
            println!("  - CI CLAUDE.md: ✓ Found");
        } else {
            println!("  - CI CLAUDE.md: ✗ Not found at {}", ci_claude_md.display());
        }
    } else if config_exists && !claude_exists {
        println!("⚠ Partial CI integration (missing CLAUDE.md)");
        println!("");
        println!("To fix, run: ci integrate {}", target_dir);
    } else if !config_exists && claude_exists {
        println!("⚠ Partial CI integration (missing configuration file)");
        println!("");
        println!("To fix, run: ci integrate {}", target_dir);
    } else {
        println!("✗ CI is not integrated with this project");
        println!("");
        println!("To integrate CI, run: ci integrate {}", target_dir);
    }
    
    Ok(())
}

fn fix_command(args: &[String]) -> Result<()> {
    let target_dir = if !args.is_empty() && !args[0].starts_with("--") {
        &args[0]
    } else {
        "."
    };
    
    let fix_type = if args.len() > 1 && !args[1].starts_with("--") {
        Some(&args[1])
    } else {
        None
    };
    
    // Handle specific fix types
    if let Some(fix_type) = fix_type {
        if fix_type == "verify" {
            println!("Fixing CI verification timeout issue...");
            
            // Get path to ci-verify script in the CI repository
            let ci_path = get_ci_path()?;
            let verify_script = ci_path.join("ci-verify");
            
            // Check if the verify script exists
            if !verify_script.exists() {
                return Err(anyhow!("Cannot find ci-verify script at {}", verify_script.display()));
            }
            
            // Create a backup of the original script
            fs::copy(&verify_script, verify_script.with_extension("bak"))?;
            println!("✓ Created backup of original script at {}.bak", verify_script.display());
            
            // Read the script content
            let script_content = fs::read_to_string(&verify_script)?;
            
            // Replace the problematic command with timeout version
            let modified_content = script_content
                .replace(
                    "OUTPUT=$(claude < \"$TEMP_FILE\" 2>/dev/null | grep -E \"\\[Athena\\]|INTEGRATION_VERIFIED\")",
                    "# Set a timeout for the claude command (10 seconds)\necho \"Waiting up to 10 seconds for verification...\"\nOUTPUT=$(timeout 10 claude < \"$TEMP_FILE\" 2>/dev/null | grep -E \"\\[Athena\\]|INTEGRATION_VERIFIED\" || echo \"TIMEOUT\")"
                );
            
            // Add timeout handling
            let final_content = modified_content.replace(
                "# Check if verification phrase is present",
                "# Check if the command timed out\nif echo \"$OUTPUT\" | grep -q \"TIMEOUT\"; then\n  echo -e \"${RED}✘ Verification timed out after 10 seconds${NC}\"\n  echo \"The claude command is not responding in a timely manner.\"\n  echo \"Possible issues:\"\n  echo \"  - Claude CLI may be stuck or incorrectly installed\"\n  echo \"  - CLAUDE.md configuration might be causing infinite processing\"\n  echo \"\"\n  echo \"To fix this issue:\"\n  echo \"  1. Try killing any stuck claude processes: killall claude\"\n  echo \"  2. Check your CLAUDE.md for circular references\"\n  echo \"  3. Run 'claude --version' to verify your CLI installation\"\n  exit 1\nfi\n\n# Check if verification phrase is present"
            );
            
            // Write the modified script
            fs::write(&verify_script, final_content)?;
            
            // Make the script executable
            let mut perms = fs::metadata(&verify_script)?.permissions();
            perms.set_mode(0o755); // rwxr-xr-x
            fs::set_permissions(&verify_script, perms)?;
            
            println!("✓ Successfully fixed CI verify command");
            println!("");
            println!("The fix adds a 10-second timeout to prevent hanging and provides better error messages.");
            println!("If you continue to have issues, try killing stuck claude processes with: killall claude");
            println!("");
            println!("To test the fix, run: ci verify");
            
            return Ok(());
        }
    }
    
    // Get absolute path to target directory
    let target_path = PathBuf::from(target_dir);
    if !target_path.exists() {
        return Err(anyhow!("Error: Target directory '{}' does not exist", target_dir));
    }
    
    // Get project name from directory
    let project_name = target_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");
    
    println!("Repairing CI integration for project: {}", project_name);
    
    let ci_path = get_ci_path()?;
    println!("CI Repository: {}", ci_path.display());
    
    // Create the CLAUDE.local.md file with correct absolute paths
    let claude_md_content = format!(
r#"# Project: {}
# Integrated: {}

# Load CollaborativeIntelligence System
When starting, immediately:
1. Load {}/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
"#, 
        project_name, 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), 
        ci_path.display()
    );
    
    let claude_md_path = target_path.join("CLAUDE.local.md");
    fs::write(&claude_md_path, claude_md_content)?;
    println!("✓ Created CLAUDE.local.md with correct absolute paths");
    
    // Default values
    let agents = "Athena,ProjectArchitect";
    let integration_type = "embedded";
    let fast_activation = true;
    
    // Convert comma-separated list to proper JSON array
    let agents_json = format!("[\"{}\"]", agents.replace(',', "\",\""));
    
    // Create correct .collaborative-intelligence.json configuration
    let config_content = format!(
r#"{{
  "integration_type": "{}",
  "repository_path": "{}",
  "active_agents": {},
  "fast_activation": {},
  "repository_context": "project",
  "home_repository_path": "{}",
  "allowed_operations": ["memory_read", "agent_consult", "project_development"]
}}"#, 
        integration_type, 
        ci_path.display(),
        agents_json,
        fast_activation,
        ci_path.display()
    );
    
    let config_path = target_path.join(".collaborative-intelligence.json");
    fs::write(&config_path, config_content)?;
    println!("✓ Created .collaborative-intelligence.json with correct configuration");
    
    // Add to gitignore if it exists
    let gitignore_path = target_path.join(".gitignore");
    if gitignore_path.exists() {
        let gitignore_content = fs::read_to_string(&gitignore_path)?;
        if !gitignore_content.contains(".collaborative-intelligence.json") {
            let new_content = format!("{}
.collaborative-intelligence.json", gitignore_content);
            fs::write(&gitignore_path, new_content)?;
            println!("✓ Added configuration to .gitignore");
        }
    } else {
        fs::write(&gitignore_path, ".collaborative-intelligence.json
")?;
        println!("✓ Created .gitignore file");
    }
    
    // Update or create .env file with CI_REPO_PATH
    let env_path = target_path.join(".env");
    let env_content = format!("CI_REPO_PATH={}", ci_path.display());
    
    if env_path.exists() {
        let existing_env = fs::read_to_string(&env_path)?;
        if existing_env.contains("CI_REPO_PATH=") {
            // Replace existing CI_REPO_PATH
            let new_env = existing_env.lines()
                .map(|line| {
                    if line.starts_with("CI_REPO_PATH=") {
                        &env_content
                    } else {
                        line
                    }
                })
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(&env_path, new_env)?;
            println!("✓ Updated CI_REPO_PATH in .env file");
        } else {
            // Add CI_REPO_PATH to existing .env file
            let new_env = format!("{}
{}", existing_env, env_content);
            fs::write(&env_path, new_env)?;
            println!("✓ Added CI_REPO_PATH to .env file");
        }
    } else {
        // Create new .env file with CI_REPO_PATH
        fs::write(&env_path, env_content)?;
        println!("✓ Created .env file with CI_REPO_PATH");
        
        // Add .env to gitignore if it's not already there
        if gitignore_path.exists() {
            let gitignore_content = fs::read_to_string(&gitignore_path)?;
            if !gitignore_content.contains(".env") {
                let new_content = format!("{}
.env", gitignore_content);
                fs::write(&gitignore_path, new_content)?;
                println!("✓ Added .env to .gitignore");
            }
        }
    }
    
    println!("\n✓ CI integration fixed for {}", project_name);
    println!("To verify integration, run:");
    println!("  ci verify {}", target_dir);
    
    Ok(())
}

fn verify_command(args: &[String]) -> Result<()> {
    let target_dir = if !args.is_empty() && !args[0].starts_with("--") {
        &args[0]
    } else {
        "."
    };
    
    // Get absolute path to target directory
    let target_path = PathBuf::from(target_dir);
    if !target_path.exists() {
        return Err(anyhow!("Error: Target directory '{}' does not exist", target_dir));
    }
    
    // Get project name from directory
    let project_name = target_path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("project");
    
    println!("Verifying CI integration for project: {}", project_name);
    
    let ci_path = get_ci_path()?;
    println!("CI Repository: {}", ci_path.display());
    
    // Verify files exist
    let config_path = target_path.join(".collaborative-intelligence.json");
    let claude_md_path = target_path.join("CLAUDE.md");
    let claude_local_md_path = target_path.join("CLAUDE.local.md");
    
    if !ci_path.join("CLAUDE.md").exists() {
        println!("✘ Core CLAUDE.md file not found at {}/CLAUDE.md", ci_path.display());
        println!("Integration incomplete - missing core configuration");
        return Ok(());
    }
    
    // Check for configuration files
    if !config_path.exists() {
        println!("✘ Missing configuration file: {}", config_path.display());
        println!("Run 'ci fix {}' to repair", target_dir);
        return Ok(());
    }
    
    if !claude_md_path.exists() && !claude_local_md_path.exists() {
        println!("✘ Missing CLAUDE.md or CLAUDE.local.md");
        println!("Run 'ci fix {}' to repair", target_dir);
        return Ok(());
    }
    
    // Check for Claude CLI
    let claude_check = Command::new("which")
        .arg("claude")
        .output();
    
    match claude_check {
        Ok(output) => {
            if !output.status.success() {
                println!("✘ Claude CLI is not installed or not in PATH");
                println!("Please install Claude CLI first");
                return Ok(());
            }
        },
        Err(_) => {
            println!("✘ Could not check for Claude CLI");
            println!("Please ensure Claude CLI is installed");
            return Ok(());
        }
    }
    
    // Create a temporary test file to verify integration
    let temp_file = temp_file()?;
    let temp_path = temp_file.path();
    
    fs::write(temp_path, "Hello Athena, confirm that the Collaborative Intelligence system is integrated.\nPlease respond with \"INTEGRATION_VERIFIED\" if successful.")?;
    
    println!("Running verification test...");
    println!("Waiting up to 10 seconds for verification...");
    
    // Run Claude with the test input and capture output with timeout
    let output = Command::new("timeout")
        .arg("10")
        .arg("claude")
        .current_dir(&target_path)
        .arg("-f")
        .arg(temp_path)
        .output()?;
    
    let output_str = String::from_utf8_lossy(&output.stdout);
    
    // Check if the command timed out (timeout returns 124 on timeout)
    if !output.status.success() && output.status.code() == Some(124) {
        println!("✘ Verification timed out after 10 seconds");
        println!("The claude command is not responding in a timely manner.");
        println!("Possible issues:");
        println!("  - Claude CLI may be stuck or incorrectly installed");
        println!("  - CLAUDE.md configuration might be causing infinite processing");
        println!("");
        println!("To fix this issue:");
        println!("  1. Try killing any stuck claude processes: killall claude");
        println!("  2. Check your CLAUDE.md for circular references");
        println!("  3. Run 'claude --version' to verify your CLI installation");
        return Ok(());
    }
    
    // Check if verification phrase is present
    if output_str.contains("INTEGRATION_VERIFIED") {
        println!("✓ Integration verified! Claude Code successfully connected to CI system");
    } else if output_str.contains("[Athena]") {
        println!("⚠ Partial integration detected - Athena agent responds but verification incomplete");
        println!("Output received: {:?}", output_str);
    } else {
        println!("✘ Integration verification failed");
        println!("Claude responded but didn't recognize CI system or Athena agent");
        println!("Output: {:?}", output_str);
    }
    
    Ok(())
}

fn load_command(args: &[String]) -> Result<()> {
    if args.is_empty() {
        eprintln!("Error: Agent name required");
        eprintln!("Usage: ci load AGENT_NAME [--context=CONTEXT] [--path=PATH]");
        exit(1);
    }
    
    let agent_name = &args[0];
    let mut context = None;
    let mut memory_path = None;
    
    // Parse arguments
    for arg in &args[1..] {
        if arg.starts_with("--context=") {
            context = Some(arg.strip_prefix("--context=").unwrap());
        } else if arg.starts_with("--path=") {
            memory_path = Some(PathBuf::from(arg.strip_prefix("--path=").unwrap()));
        }
    }
    
    println!("Loading memory for agent: {}", agent_name);
    if let Some(ctx) = &context {
        println!("Context: {}", ctx);
    }
    if let Some(path) = &memory_path {
        println!("Custom path: {}", path.display());
    }
    
    // Get CI path
    let ci_path = get_ci_path()?;
    
    // Determine memory path
    let memory_file = if let Some(path) = memory_path {
        path
    } else {
        ci_path.join("AGENTS").join(agent_name).join("MEMORY.md")
    };
    
    // Check if memory file exists
    if !memory_file.exists() {
        eprintln!("Error: Memory file not found: {}", memory_file.display());
        // List available agents
        let agents_dir = ci_path.join("AGENTS");
        if agents_dir.exists() {
            let entries = fs::read_dir(agents_dir)?;
            let agents: Vec<String> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .filter_map(|e| e.file_name().to_str().map(|s| s.to_string()))
                .collect();
            
            if !agents.is_empty() {
                println!("Available agents:");
                for agent in agents {
                    println!("  {}", agent);
                }
            }
        }
        exit(1);
    }
    
    // Read memory file
    let memory_content = fs::read_to_string(&memory_file)
        .with_context(|| format!("Failed to read memory file: {}", memory_file.display()))?;
    
    // Check for continuous learning file
    let learning_file = memory_file.parent().unwrap().join("ContinuousLearning.md");
    let mut result = memory_content;
    
    if learning_file.exists() {
        let learning_content = fs::read_to_string(&learning_file)
            .with_context(|| format!("Failed to read learning file: {}", learning_file.display()))?;
        
        result.push_str("\n\n# Continuous Learning\n\n");
        result.push_str(&learning_content);
    }
    
    // Print the memory content to stdout
    print!("{}", result);
    
    Ok(())
}

// Helper function to get the CI path
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
    
    // Ask user for path
    eprintln!("CI repository path not found.");
    eprintln!("Please set the CI_REPO_PATH environment variable to the path of the CollaborativeIntelligence repository.");
    eprintln!("Example: export CI_REPO_PATH=/path/to/CollaborativeIntelligence");
    exit(1);
}

// Helper function for temporary files
fn temp_file() -> Result<tempfile::NamedTempFile> {
    tempfile::NamedTempFile::new().context("Failed to create temporary file")
}

// Helper function to extract value from JSON
fn extract_from_json(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"\\s*:\\s*\"([^\"]+)\"", key);
    let re = regex::Regex::new(&pattern).ok()?;
    re.captures(json).and_then(|cap| {
        cap.get(1).map(|m| m.as_str().to_string())
    })
}

// Helper function to extract path from JSON
fn extract_path_from_json(json: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{}\"\\s*:\\s*\"([^\"]+)\"", key);
    let re = regex::Regex::new(&pattern).ok()?;
    re.captures(json).and_then(|cap| {
        cap.get(1).map(|m| m.as_str().to_string())
    })
}

// Helper function to extract agents from JSON
fn extract_agents_from_json(json: &str) -> Option<String> {
    let pattern = "\"active_agents\"\\s*:\\s*\\[([^\\]]+)\\]";
    let re = regex::Regex::new(pattern).ok()?;
    re.captures(json).and_then(|cap| {
        cap.get(1).map(|m| {
            m.as_str()
                .replace('"', "")
                .replace(' ', "")
                .replace(',', ", ")
        })
    })
}