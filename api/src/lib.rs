// Static API Library - No server required
// Can be called directly by external tools

pub mod content_service;
pub mod document_registry;
pub mod error;

pub use content_service::ContentService;
pub use document_registry::{DocumentRegistry, DocumentType};
pub use error::{ApiError, ApiResult};

use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::path::PathBuf;
use colored::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentInfo {
    pub name: String,
    pub path: String,
    pub status: String,
    pub has_readme: bool,
    pub has_metadata: bool,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentMetadata {
    pub path: String,
    pub relative_path: String,
    pub last_modified: DateTime<Utc>,
    pub size: u64,
}

/// Static API for CollaborativeIntelligence content
/// No server required - direct function calls
pub struct CollaborativeIntelligenceApi {
    content_service: ContentService,
}

impl CollaborativeIntelligenceApi {
    /// Create new API instance
    pub fn new(base_path: PathBuf) -> ApiResult<Self> {
        let content_service = ContentService::new(base_path)?;
        Ok(CollaborativeIntelligenceApi { content_service })
    }

    /// Get formatted agents list (close to original CI design)
    pub async fn get_agents_formatted(&self) -> ApiResult<String> {
        let agents = self.content_service.get_agent_list().await?;
        
        let active_agents: Vec<_> = agents.iter().filter(|a| a.status == "active").collect();
        let inactive_agents: Vec<_> = agents.iter().filter(|a| a.status == "inactive").collect();
        
        let mut output = Vec::new();
        
        // Match original CI formatting exactly
        output.push(format!("{}\n", "Available Collaborative Intelligence Agents".bold()));
        output.push(format!("{}\n", "========================================".bold()));
        output.push("\n".to_string());
        
        if agents.is_empty() {
            output.push("No agents found.\n".to_string());
            return Ok(output.join(""));
        }
        
        // Print blue divider line (matching CommandHelpers::print_divider("blue"))
        output.push(format!("{}\n", "═".repeat(70).blue()));
        output.push("\n".to_string());
        
        // Sort agents alphabetically (like original)
        let mut sorted_agents = active_agents.clone();
        sorted_agents.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        
        output.push(format!("{}\n", "Available Agents:".bold()));
        output.push("\n".to_string());
        
        for agent in sorted_agents {
            let status_icon = if agent.has_readme { "•" } else { "○" };
            let agent_name = agent.name.cyan().bold();
            output.push(format!("{} {}", status_icon.green().bold(), agent_name));
            if let Some(ref desc) = agent.description {
                output.push(format!(" - {}", desc.white()));
            }
            output.push("\n".to_string());
        }
        
        if !inactive_agents.is_empty() {
            output.push(format!("\n{}\n", "Inactive Agents:".bold()));
            for agent in &inactive_agents {
                let agent_name = agent.name.cyan().bold();
                output.push(format!("{} {} - {}\n", "○".red().bold(), agent_name, "No README found".white()));
            }
        }
        
        output.push("\n".to_string());
        output.push(format!("{}\n", "To use agents in your project:".yellow().bold()));
        output.push(format!("  {}\n", "ci init <project-name> --agents=Agent1,Agent2".white()));
        output.push(format!("  {}\n", "ci integrate --agents=Agent1,Agent2".white()));
        output.push("\n".to_string());
        output.push(format!("{}\n", "For more information on a specific agent:".yellow().bold()));
        output.push(format!("  {}\n", "ci load <agent-name>".white()));
        output.push(format!("  {}\n", "ci load <agent-name> --context=<context>".white()));
        
        Ok(output.join(""))
    }
    
    /// Categorize agents for better organization
    fn categorize_agent(&self, agent: &AgentInfo) -> &'static str {
        match agent.name.as_str() {
            "Athena" | "Sage" | "Memory" | "Mnemosyne" | "SageKeeper" => "Core",
            "Developer" | "Engineer" | "Architect" | "Rustist" | "Debugger" | "Tester" | "Refactorer" => "Development",
            "Database" | "Analyst" | "Benchmarker" | "Researcher" | "Comparitor" => "Data & Analysis",
            "Designer" | "UI" | "UX" | "Visualist" => "UI/UX",
            "Automator" | "Fixer" | "Optimizer" | "Basher" | "CLIA" | "Topologist" => "System",
            _ => "Other"
        }
    }

    /// Get raw agent list (for programmatic use)
    pub async fn get_agents_list(&self) -> ApiResult<Vec<AgentInfo>> {
        self.content_service.get_agent_list().await
    }

    /// Get specific agent info
    pub async fn get_agent_info(&self, agent_name: &str) -> ApiResult<String> {
        self.content_service.get_agent_readme(agent_name).await
    }

    /// Get document content by type
    pub async fn get_document(&self, doc_type: DocumentType) -> ApiResult<(String, DocumentMetadata)> {
        self.content_service.get_document_content(doc_type).await
    }
}

/// Convenience function for external tools (synchronous)
pub async fn get_agents_content(ci_repo_path: PathBuf) -> Result<String, Box<dyn std::error::Error>> {
    let api = CollaborativeIntelligenceApi::new(ci_repo_path)?;
    let content = api.get_agents_formatted().await?;
    Ok(content)
}