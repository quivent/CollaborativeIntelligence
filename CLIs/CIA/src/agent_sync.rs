use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub name: String,
    pub role: String,
    pub description: String,
    pub focus: Option<String>,
}

#[derive(Debug)]
pub struct AgentSyncTool {
    base_dir: String,
    agents_file: String,
    agent_index_file: String,
}

impl AgentSyncTool {
    pub fn new() -> Self {
        let base_dir = "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence".to_string();
        Self {
            agents_file: format!("{}/docs/agent-system/agents-overview.md", base_dir),
            agent_index_file: format!("{}/docs/agent-system/agent-index.md", base_dir),
            base_dir,
        }
    }

    pub fn with_base_dir(base_dir: String) -> Self {
        Self {
            agents_file: format!("{}/docs/agent-system/agents-overview.md", base_dir),
            agent_index_file: format!("{}/docs/agent-system/agent-index.md", base_dir),
            base_dir,
        }
    }

    pub fn parse_agents_file(&self) -> Result<Vec<Agent>> {
        let content = fs::read_to_string(&self.agents_file)
            .with_context(|| format!("Failed to read agents file: {}", self.agents_file))?;

        let mut agents = Vec::new();
        let agent_regex = Regex::new(r"(?m)^##\s+(.+?)$").unwrap();
        let role_regex = Regex::new(r"(?m)^\*\*Role\*\*:\s*(.+?)$").unwrap();
        let description_regex = Regex::new(r"(?m)^\*\*Description\*\*:\s*(.+?)$").unwrap();
        let focus_regex = Regex::new(r"(?m)^\*\*Focus\*\*:\s*(.+?)$").unwrap();

        let sections: Vec<&str> = agent_regex.split(&content).collect();
        let agent_names: Vec<_> = agent_regex
            .find_iter(&content)
            .map(|m| m.as_str().trim_start_matches("## ").trim())
            .collect();

        for (i, &section) in sections.iter().enumerate().skip(1) {
            if let Some(name) = agent_names.get(i - 1) {
                let role = role_regex
                    .find(section)
                    .map(|m| {
                        m.as_str()
                            .trim_start_matches("**Role**:")
                            .trim()
                            .to_string()
                    })
                    .unwrap_or_else(|| "Not specified".to_string());

                let description = description_regex
                    .find(section)
                    .map(|m| {
                        m.as_str()
                            .trim_start_matches("**Description**:")
                            .trim()
                            .to_string()
                    })
                    .unwrap_or_else(|| "No description available".to_string());

                let focus = focus_regex.find(section).map(|m| {
                    m.as_str()
                        .trim_start_matches("**Focus**:")
                        .trim()
                        .to_string()
                });

                agents.push(Agent {
                    name: name.to_string(),
                    role,
                    description,
                    focus,
                });
            }
        }

        Ok(agents)
    }

    pub fn parse_agent_index(&self) -> Result<HashMap<String, Agent>> {
        if !Path::new(&self.agent_index_file).exists() {
            return Ok(HashMap::new());
        }

        let content = fs::read_to_string(&self.agent_index_file)
            .with_context(|| format!("Failed to read agent index: {}", self.agent_index_file))?;

        let mut index_agents = HashMap::new();
        let line_regex = Regex::new(r"(?m)^\|\s*\*\*(.+?)\*\*\s*\|\s*(.+?)\s*\|\s*(.+?)\s*\|").unwrap();

        for cap in line_regex.captures_iter(&content) {
            let name = cap[1].trim().to_string();
            let role = cap[2].trim().to_string();
            let description = cap[3].trim().to_string();

            index_agents.insert(
                name.clone(),
                Agent {
                    name,
                    role,
                    description,
                    focus: None,
                },
            );
        }

        Ok(index_agents)
    }

    pub fn generate_agent_index(&self, agents: &[Agent]) -> Result<String> {
        let mut content = String::new();
        content.push_str("# Agent Index\n\n");
        content.push_str("Quick reference for all available agents in the Collaborative Intelligence system.\n\n");
        content.push_str("| Agent | Role | Description |\n");
        content.push_str("|-------|------|-------------|\n");

        for agent in agents {
            content.push_str(&format!(
                "| **{}** | {} | {} |\n",
                agent.name, agent.role, agent.description
            ));
        }

        content.push_str("\n---\n\n");
        content.push_str(&format!(
            "*Last updated: {}*\n",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        ));
        content.push_str("*Generated automatically by CIA agent sync tool*\n");

        Ok(content)
    }

    pub fn sync_indices(&self) -> Result<()> {
        println!("🔍 Analyzing agents-overview.md...");
        let agents = self.parse_agents_file()?;
        println!("  - Found {} agents", agents.len());
        println!("  - Located in {}", self.agents_file);

        println!("\n🔄 Checking current agent-index.md...");
        let index_agents = self.parse_agent_index()?;
        println!("  - Found {} agents in index", index_agents.len());
        println!("  - Located in {}", self.agent_index_file);

        let discrepancies = self.check_discrepancies(&agents, &index_agents);
        
        if discrepancies.is_empty() {
            println!("\n✓ agents-overview.md and agent-index.md are synchronized");
            return Ok(());
        }

        println!("\n⚠️  Found discrepancies:");
        for discrepancy in &discrepancies {
            println!("  - {}", discrepancy);
        }

        println!("\n📝 Updating agent-index.md...");
        let new_content = self.generate_agent_index(&agents)?;
        fs::write(&self.agent_index_file, new_content)
            .with_context(|| format!("Failed to write agent index: {}", self.agent_index_file))?;

        println!("✓ agent-index.md successfully synchronized with agents-overview.md");
        println!("  - agents-overview.md: {} agents", agents.len());
        println!("  - agent-index.md: {} agents", agents.len());

        Ok(())
    }

    pub fn check_discrepancies(&self, agents: &[Agent], index_agents: &HashMap<String, Agent>) -> Vec<String> {
        let mut discrepancies = Vec::new();
        
        let agent_names: std::collections::HashSet<String> = 
            agents.iter().map(|a| a.name.clone()).collect();
        let index_names: std::collections::HashSet<String> = 
            index_agents.keys().cloned().collect();

        let missing_in_index: Vec<_> = agent_names.difference(&index_names).collect();
        let extra_in_index: Vec<_> = index_names.difference(&agent_names).collect();

        if !missing_in_index.is_empty() {
            discrepancies.push(format!(
                "Missing in agent-index.md: {}", 
                missing_in_index.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }

        if !extra_in_index.is_empty() {
            discrepancies.push(format!(
                "Extra in agent-index.md: {}", 
                extra_in_index.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")
            ));
        }

        // Check for role/description mismatches
        for agent in agents {
            if let Some(index_agent) = index_agents.get(&agent.name) {
                if agent.role != index_agent.role {
                    discrepancies.push(format!(
                        "Role mismatch for {}: '{}' vs '{}'", 
                        agent.name, agent.role, index_agent.role
                    ));
                }
            }
        }

        discrepancies
    }


    pub fn add_agent(&self, name: &str, role: &str, description: &str) -> Result<()> {
        // Add to agents-overview.md
        let mut agents_content = fs::read_to_string(&self.agents_file)
            .with_context(|| format!("Failed to read agents file: {}", self.agents_file))?;

        let new_agent_section = format!(
            "\n## {}\n\n**Role**: {}\n**Description**: {}\n\n",
            name, role, description
        );

        agents_content.push_str(&new_agent_section);
        fs::write(&self.agents_file, agents_content)
            .with_context(|| format!("Failed to write agents file: {}", self.agents_file))?;

        // Sync the index
        self.sync_indices()?;

        println!("✓ Added agent '{}' to both files", name);
        Ok(())
    }
}

// Backwards compatibility functions
pub fn sync_agent_indices() -> Result<()> {
    let tool = AgentSyncTool::new();
    tool.sync_indices()
}

pub fn add_agent(name: &str, role: &str, description: &str) -> Result<()> {
    let tool = AgentSyncTool::new();
    tool.add_agent(name, role, description)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_agent_sync_tool_creation() {
        let tool = AgentSyncTool::new();
        assert!(tool.agents_file.contains("docs/agent-system/agents-overview.md"));
        assert!(tool.agent_index_file.contains("docs/agent-system/agent-index.md"));
    }

    #[test]
    fn test_generate_agent_index() {
        let tool = AgentSyncTool::new();
        let agents = vec![
            Agent {
                name: "TestAgent".to_string(),
                role: "Tester".to_string(),
                description: "Test description".to_string(),
                focus: None,
            }
        ];

        let content = tool.generate_agent_index(&agents).unwrap();
        assert!(content.contains("TestAgent"));
        assert!(content.contains("Tester"));
        assert!(content.contains("Test description"));
    }
}