use crate::config::Config;
use anyhow::Result;
use colored::*;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
pub struct RepositoryInfo {
    pub name: String,
    pub path: PathBuf,
    pub current_branch: String,
    pub last_commit: String,
    pub has_uncommitted_changes: bool,
    pub staged_files: Vec<String>,
    pub unstaged_files: Vec<String>,
    pub health: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MultiRepoConfig {
    pub repositories: HashMap<String, RepoConfig>,
    pub commit_protocol: CommitProtocolConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RepoConfig {
    pub path: String,
    pub commit_priority: u32,
    pub dependencies: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommitProtocolConfig {
    pub require_clean_working_tree: bool,
    pub validate_commit_messages: bool,
    pub enable_rollback: bool,
    pub parallel_execution: bool,
}

impl Default for MultiRepoConfig {
    fn default() -> Self {
        Self {
            repositories: HashMap::new(),
            commit_protocol: CommitProtocolConfig {
                require_clean_working_tree: true,
                validate_commit_messages: true,
                enable_rollback: true,
                parallel_execution: false,
            },
        }
    }
}

pub fn init(root_path: &str, gen_config: bool, _config: &Config) -> Result<()> {
    println!("{} Initializing multi-repository environment...", "🚀".bright_blue());
    
    let root = Path::new(root_path);
    if !root.exists() {
        return Err(anyhow::anyhow!("Root path does not exist: {}", root_path));
    }
    
    let repositories = discover_repositories(root)?;
    
    println!("{} {} repositories discovered:", "✅".green(), repositories.len());
    
    for repo in &repositories {
        let status = if repo.has_uncommitted_changes {
            "uncommitted changes".yellow()
        } else {
            "clean".green()
        };
        println!("  {} {}: {} ({})", "📁".blue(), repo.name, status, repo.current_branch);
    }
    
    if gen_config {
        generate_config_file(root_path, &repositories)?;
        println!("{} Configuration file generated at multi_repo_config.json", "✅".green());
    }
    
    Ok(())
}

pub fn commit(
    root_path: &Option<String>, 
    message: &Option<String>, 
    dry_run: bool, 
    skip_deps: bool, 
    _config: &Config
) -> Result<()> {
    let root = root_path.as_deref().unwrap_or(".");
    
    println!("{} Starting sequential commit protocol...", "🔄".bright_blue());
    
    let repositories = discover_repositories(Path::new(root))?;
    let repos_with_changes: Vec<_> = repositories.iter()
        .filter(|repo| repo.has_uncommitted_changes)
        .collect();
    
    if repos_with_changes.is_empty() {
        println!("{} No repositories have uncommitted changes", "ℹ️".blue());
        return Ok(());
    }
    
    println!("Found {} repositories with changes:", repos_with_changes.len());
    for repo in &repos_with_changes {
        println!("  {} {} ({} staged, {} unstaged)", 
            "📝".yellow(), 
            repo.name, 
            repo.staged_files.len(),
            repo.unstaged_files.len()
        );
    }
    
    if dry_run {
        println!("\n{} This is a dry run - no commits will be made", "ℹ️".blue());
        return Ok(());
    }
    
    // Load dependencies if not skipping
    let execution_plan = if skip_deps {
        create_simple_execution_plan(&repos_with_changes)
    } else {
        create_dependency_execution_plan(&repos_with_changes, root)?
    };
    
    println!("\n{} Execution plan:", "📋".bright_blue());
    for (phase, repos) in execution_plan.iter().enumerate() {
        let repo_names: Vec<String> = repos.iter().map(|r| r.name.clone()).collect();
        println!("  Phase {}: {}", phase + 1, repo_names.join(", "));
    }
    
    execute_sequential_commits(&execution_plan, message)?;
    
    Ok(())
}

pub fn analyze(root_path: &str, format: &str, _config: &Config) -> Result<()> {
    println!("{} Analyzing repository topology...", "🔍".bright_blue());
    
    let repositories = discover_repositories(Path::new(root_path))?;
    
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(&repositories)?;
            println!("{}", json);
        },
        "table" => {
            print_topology_table(&repositories);
        },
        _ => return Err(anyhow::anyhow!("Unsupported format: {}", format)),
    }
    
    Ok(())
}

fn discover_repositories(root_path: &Path) -> Result<Vec<RepositoryInfo>> {
    let mut repositories = Vec::new();
    
    for entry in fs::read_dir(root_path)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let repo_path = entry.path();
            let git_path = repo_path.join(".git");
            
            if git_path.exists() {
                if let Ok(repo_info) = analyze_repository(&repo_path) {
                    repositories.push(repo_info);
                }
            }
        }
    }
    
    Ok(repositories)
}

fn analyze_repository(repo_path: &Path) -> Result<RepositoryInfo> {
    let name = repo_path.file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("unknown")
        .to_string();
    
    // Change to repository directory
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .current_dir(repo_path)
        .output()?;
    
    let status = String::from_utf8_lossy(&output.stdout);
    let has_uncommitted_changes = !status.trim().is_empty();
    
    // Get current branch
    let branch_output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .current_dir(repo_path)
        .output()?;
    let current_branch = String::from_utf8_lossy(&branch_output.stdout).trim().to_string();
    
    // Get last commit
    let commit_output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(repo_path)
        .output()?;
    let last_commit = String::from_utf8_lossy(&commit_output.stdout).trim().to_string();
    
    // Get staged files
    let staged_output = Command::new("git")
        .args(["diff", "--cached", "--name-only"])
        .current_dir(repo_path)
        .output()?;
    let staged_files: Vec<String> = String::from_utf8_lossy(&staged_output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.to_string())
        .collect();
    
    // Get unstaged files
    let unstaged_output = Command::new("git")
        .args(["diff", "--name-only"])
        .current_dir(repo_path)
        .output()?;
    let unstaged_files: Vec<String> = String::from_utf8_lossy(&unstaged_output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|s| s.to_string())
        .collect();
    
    Ok(RepositoryInfo {
        name,
        path: repo_path.to_path_buf(),
        current_branch,
        last_commit,
        has_uncommitted_changes,
        staged_files,
        unstaged_files,
        health: "healthy".to_string(),
    })
}

fn generate_config_file(root_path: &str, repositories: &[RepositoryInfo]) -> Result<()> {
    let mut config = MultiRepoConfig::default();
    
    for (i, repo) in repositories.iter().enumerate() {
        config.repositories.insert(
            repo.name.clone(),
            RepoConfig {
                path: repo.path.to_string_lossy().to_string(),
                commit_priority: i as u32 + 1,
                dependencies: Vec::new(),
            },
        );
    }
    
    let config_path = Path::new(root_path).join("multi_repo_config.json");
    let json = serde_json::to_string_pretty(&config)?;
    fs::write(config_path, json)?;
    
    Ok(())
}

fn create_simple_execution_plan<'a>(repositories: &'a [&'a RepositoryInfo]) -> Vec<Vec<&'a RepositoryInfo>> {
    vec![repositories.to_vec()]
}

fn create_dependency_execution_plan<'a>(
    repositories: &'a [&'a RepositoryInfo], 
    root_path: &str
) -> Result<Vec<Vec<&'a RepositoryInfo>>> {
    let config_path = Path::new(root_path).join("multi_repo_config.json");
    
    if !config_path.exists() {
        // No config file, use simple plan
        return Ok(create_simple_execution_plan(repositories));
    }
    
    let config_content = fs::read_to_string(config_path)?;
    let config: MultiRepoConfig = serde_json::from_str(&config_content)?;
    
    // Sort by priority for now (dependency resolution would be more complex)
    let mut sorted_repos = repositories.to_vec();
    sorted_repos.sort_by(|a, b| {
        let priority_a = config.repositories.get(&a.name).map(|r| r.commit_priority).unwrap_or(999);
        let priority_b = config.repositories.get(&b.name).map(|r| r.commit_priority).unwrap_or(999);
        priority_a.cmp(&priority_b)
    });
    
    Ok(vec![sorted_repos])
}

fn execute_sequential_commits(
    execution_plan: &[Vec<&RepositoryInfo>], 
    message: &Option<String>
) -> Result<()> {
    for (phase, repos) in execution_plan.iter().enumerate() {
        println!("\n{} Phase {}: {} repositories", "🔄".bright_blue(), phase + 1, repos.len());
        
        for repo in repos {
            println!("Committing {}...", repo.name);
            
            let default_msg = format!("Update {}", repo.name);
            let commit_msg = message.as_deref().unwrap_or(&default_msg);
            
            // Stage all changes if nothing staged
            if repo.staged_files.is_empty() && !repo.unstaged_files.is_empty() {
                Command::new("git")
                    .args(["add", "."])
                    .current_dir(&repo.path)
                    .output()?;
            }
            
            // Commit
            let output = Command::new("git")
                .args(["commit", "-m", commit_msg])
                .current_dir(&repo.path)
                .output()?;
            
            if output.status.success() {
                let commit_output = Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .current_dir(&repo.path)
                    .output()?;
                let commit_hash_full = String::from_utf8_lossy(&commit_output.stdout);
                let commit_hash = commit_hash_full.trim();
                
                println!("  {} {}: {}", "✅".green(), repo.name, &commit_hash[..8]);
            } else {
                let error = String::from_utf8_lossy(&output.stderr);
                println!("  {} {}: {}", "❌".red(), repo.name, error.trim());
            }
        }
    }
    
    Ok(())
}

fn print_topology_table(repositories: &[RepositoryInfo]) {
    println!("\n{}", "Repository Topology Analysis".bright_blue().bold());
    println!("{}", "=".repeat(60));
    
    for repo in repositories {
        let status = if repo.has_uncommitted_changes {
            format!("{} changes", repo.staged_files.len() + repo.unstaged_files.len()).yellow()
        } else {
            "clean".green()
        };
        
        println!("{} {}", "📁".blue(), repo.name.bright_white().bold());
        println!("  Branch: {}", repo.current_branch.cyan());
        println!("  Status: {}", status);
        println!("  Path: {}", repo.path.display());
        
        if repo.has_uncommitted_changes {
            if !repo.staged_files.is_empty() {
                println!("  Staged: {}", repo.staged_files.join(", ").green());
            }
            if !repo.unstaged_files.is_empty() {
                println!("  Unstaged: {}", repo.unstaged_files.join(", ").yellow());
            }
        }
        println!();
    }
}