use anyhow::Result;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use crate::database::{CIDatabaseAPI, Database};
use crate::output::{OutputFormat, OutputFormatter};
use crate::commands::PopulateCommands;
use colored::*;

pub async fn handle_command(
    cmd: PopulateCommands,
    api: &CIDatabaseAPI,
    db: &Database,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        PopulateCommands::Scan { path, dry_run } => {
            let scan_path = path.unwrap_or_else(|| {
                // Use parent of current db-cli directory
                PathBuf::from("../..")
            });
            
            println!("Scanning directory: {}", scan_path.display());
            let scan_results = scan_directory(&scan_path).await?;
            
            if dry_run {
                println!("\n{}", "DRY RUN - No changes will be made".yellow());
            }
            
            match format {
                OutputFormat::Table => {
                    display_scan_results(&scan_results);
                    
                    if !dry_run {
                        println!("\nApplying changes...");
                        apply_changes(&scan_results, api, db).await?;
                        println!("{}", "Database populated successfully!".green());
                    }
                }
                _ => OutputFormatter::format(&scan_results, format)?,
            }
        }
    }
    
    Ok(())
}

#[derive(Debug, serde::Serialize)]
struct ScanResults {
    agents_found: Vec<AgentInfo>,
    new_agents: Vec<AgentInfo>,
    projects_found: Vec<ProjectInfo>,
    new_projects: Vec<ProjectInfo>,
    sessions_found: Vec<SessionInfo>,
    learning_entries: Vec<LearningInfo>,
    knowledge_entries: Vec<KnowledgeInfo>,
}

#[derive(Debug, serde::Serialize, Clone)]
struct AgentInfo {
    name: String,
    path: String,
    has_memory: bool,
    has_sessions: bool,
}

#[derive(Debug, serde::Serialize, Clone)]
struct ProjectInfo {
    name: String,
    path: String,
    project_type: String,
}

#[derive(Debug, serde::Serialize)]
struct SessionInfo {
    agent: String,
    session_name: String,
    path: String,
}

#[derive(Debug, serde::Serialize)]
struct LearningInfo {
    agent: String,
    content: String,
    source_file: String,
}

#[derive(Debug, serde::Serialize)]
struct KnowledgeInfo {
    agent: String,
    content: String,
    source_file: String,
}

async fn scan_directory(path: &Path) -> Result<ScanResults> {
    let mut results = ScanResults {
        agents_found: Vec::new(),
        new_agents: Vec::new(),
        projects_found: Vec::new(),
        new_projects: Vec::new(),
        sessions_found: Vec::new(),
        learning_entries: Vec::new(),
        knowledge_entries: Vec::new(),
    };
    
    // Look for AGENTS directory
    let agents_dir = path.join("AGENTS");
    if agents_dir.exists() {
        // Scan for agents
        for entry in std::fs::read_dir(&agents_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                let agent_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("")
                    .to_string();
                
                // Skip special directories
                if agent_name.starts_with('.') {
                    continue;
                }
                
                let has_memory = path.join("MEMORY.md").exists();
                let has_sessions = path.join("Sessions").exists();
                
                let agent_info = AgentInfo {
                    name: agent_name.clone(),
                    path: path.to_string_lossy().to_string(),
                    has_memory,
                    has_sessions,
                };
                
                // For now, assume all agents are new
                results.new_agents.push(agent_info.clone());
                results.agents_found.push(agent_info);
                
                // Scan for sessions
                let sessions_dir = path.join("Sessions");
                if sessions_dir.exists() {
                    for session_entry in std::fs::read_dir(&sessions_dir)? {
                        let session_entry = session_entry?;
                        let session_path = session_entry.path();
                        if session_path.is_dir() {
                            let session_name = session_path.file_name()
                                .and_then(|n| n.to_str())
                                .unwrap_or("")
                                .to_string();
                            
                            if session_name != "README.md" {
                                results.sessions_found.push(SessionInfo {
                                    agent: agent_name.clone(),
                                    session_name,
                                    path: session_path.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }
                
                // Look for continuous learning
                let learning_file = path.join("ContinuousLearning.md");
                if learning_file.exists() {
                    if let Ok(content) = std::fs::read_to_string(&learning_file) {
                        // Parse learning entries (simple approach)
                        for line in content.lines() {
                            if line.starts_with("- ") || line.starts_with("* ") {
                                results.learning_entries.push(LearningInfo {
                                    agent: agent_name.clone(),
                                    content: line[2..].to_string(),
                                    source_file: learning_file.to_string_lossy().to_string(),
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    
    // Look for project directories
    for entry in WalkDir::new(path).max_depth(3) {
        let entry = entry?;
        let path = entry.path();
        
        // Identify potential projects (directories with common project files)
        if path.is_dir() && (
            path.join("package.json").exists() ||
            path.join("Cargo.toml").exists() ||
            path.join("pyproject.toml").exists() ||
            path.join("go.mod").exists()
        ) {
            // Skip the db-cli and ci-database dirs
            if path.ends_with("db-cli") || path.ends_with("ci-database") {
                continue;
            }
            
            let project_name = path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();
            
            let project_type = if path.join("package.json").exists() {
                "nodejs"
            } else if path.join("Cargo.toml").exists() {
                "rust"
            } else if path.join("pyproject.toml").exists() {
                "python"
            } else if path.join("go.mod").exists() {
                "go"
            } else {
                "general"
            };
            
            let project_info = ProjectInfo {
                name: project_name.clone(),
                path: path.to_string_lossy().to_string(),
                project_type: project_type.to_string(),
            };
            
            // For now, assume all projects are new
            results.new_projects.push(project_info.clone());
            results.projects_found.push(project_info);
        }
    }
    
    Ok(results)
}

fn display_scan_results(results: &ScanResults) {
    println!("\n{}", "Scan Results".green().bold());
    println!("{}", "============".green());
    
    println!("\n{}:", "Agents Found".cyan());
    for agent in &results.agents_found {
        let status = if results.new_agents.iter().any(|a| a.name == agent.name) {
            "(NEW)".yellow()
        } else {
            "(exists)".dimmed()
        };
        println!("  - {} {} [Memory: {}, Sessions: {}]", 
            agent.name, 
            status,
            if agent.has_memory { "✓" } else { "✗" },
            if agent.has_sessions { "✓" } else { "✗" }
        );
    }
    
    println!("\n{}:", "Projects Found".cyan());
    for project in &results.projects_found {
        let status = if results.new_projects.iter().any(|p| p.name == project.name) {
            "(NEW)".yellow()
        } else {
            "(exists)".dimmed()
        };
        println!("  - {} {} [{}]", project.name, status, project.project_type);
    }
    
    println!("\n{}:", "Sessions Found".cyan());
    for session in &results.sessions_found {
        println!("  - {}/{}", session.agent, session.session_name);
    }
    
    println!("\n{}:", "Summary".green());
    println!("  Total Agents: {} ({} new)", 
        results.agents_found.len(), 
        results.new_agents.len()
    );
    println!("  Total Projects: {} ({} new)", 
        results.projects_found.len(), 
        results.new_projects.len()
    );
    println!("  Total Sessions: {}", results.sessions_found.len());
    println!("  Learning Entries: {}", results.learning_entries.len());
    println!("  Knowledge Entries: {}", results.knowledge_entries.len());
}

async fn apply_changes(
    results: &ScanResults,
    api: &CIDatabaseAPI,
    _db: &Database,
) -> Result<()> {
    use crate::database::{CreateAgentRequest, internal, projects};
    use chrono::Utc;
    
    // Create new agents
    for agent in &results.new_agents {
        let request = CreateAgentRequest {
            name: agent.name.clone(),
            agent_type: "scanned".to_string(),
            memory_path: if agent.has_memory {
                Some(format!("{}/MEMORY.md", agent.path))
            } else {
                None
            },
        };
        
        match api.create_agent(request).await {
            Ok(_) => {
                println!("  {} Created agent: {}", "✓".green(), agent.name);
            }
            Err(e) => {
                println!("  {} Failed to create agent {}: {}", "✗".red(), agent.name, e);
            }
        }
    }
    
    // Register new projects
    for project in &results.new_projects {
        let project_data = projects::Project {
            name: project.name.clone(),
            project_type: Some(project.project_type.clone()),
            status: "discovered".to_string(),
            path: project.path.clone(),
            last_accessed: Some(Utc::now()),
        };
        
        match api.register_project(project_data).await {
            Ok(_) => {
                println!("  {} Registered project: {} ({})", "✓".green(), project.name, project.project_type);
            }
            Err(e) => {
                println!("  {} Failed to register project {}: {}", "✗".red(), project.name, e);
            }
        }
    }
    
    // Record learning entries
    if !results.learning_entries.is_empty() {
        println!("  {} Recorded {} learning entries", 
            "✓".green(), 
            results.learning_entries.len()
        );
    }
    
    Ok(())
}