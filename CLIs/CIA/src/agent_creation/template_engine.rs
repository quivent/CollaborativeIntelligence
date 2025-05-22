//! Template Engine for Agent Creation
//! 
//! This module provides comprehensive template processing for creating agents
//! with all required files and proper metadata.

use anyhow::{Context, Result};
use chrono::{Utc, Datelike};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct AgentTemplate {
    pub name: String,
    pub role: String,
    pub purpose: String,
    pub competencies: Vec<String>,
    pub principles: Vec<String>,
    pub frameworks: Vec<String>,
    pub collaborations: Vec<String>,
    pub tagline: String,
    pub activation_contexts: Vec<String>,
    pub interaction_patterns: Vec<String>,
    pub examples: Vec<String>,
    pub integrations: Vec<String>,
    pub specializations: Vec<String>,
    pub metrics: Vec<String>,
    pub limitations: Vec<String>,
}

impl AgentTemplate {
    /// Create a new agent template with intelligent defaults based on role
    pub fn new(name: String, role: String, purpose: String) -> Self {
        let mut template = Self {
            name: name.clone(),
            role: role.clone(),
            purpose: purpose.clone(),
            competencies: Vec::new(),
            principles: Vec::new(),
            frameworks: Vec::new(),
            collaborations: Vec::new(),
            tagline: format!("{} specialist", role),
            activation_contexts: Vec::new(),
            interaction_patterns: Vec::new(),
            examples: Vec::new(),
            integrations: Vec::new(),
            specializations: Vec::new(),
            metrics: Vec::new(),
            limitations: Vec::new(),
        };

        // Generate intelligent defaults based on the role
        template.generate_role_based_defaults();
        template
    }

    /// Generate role-based defaults using pattern matching
    fn generate_role_based_defaults(&mut self) {
        let role_lower = self.role.to_lowercase();
        
        // Core competencies based on role
        self.competencies = match role_lower.as_str() {
            r if r.contains("analyst") => vec![
                "Data analysis and interpretation".to_string(),
                "Pattern recognition and insight extraction".to_string(),
                "Report generation and visualization".to_string(),
                "Statistical analysis and modeling".to_string(),
            ],
            r if r.contains("architect") => vec![
                "System design and architecture".to_string(),
                "Technology stack selection".to_string(),
                "Scalability and performance planning".to_string(),
                "Integration strategy development".to_string(),
            ],
            r if r.contains("developer") || r.contains("engineer") => vec![
                "Code implementation and optimization".to_string(),
                "Testing and quality assurance".to_string(),
                "Documentation and technical writing".to_string(),
                "Problem-solving and debugging".to_string(),
            ],
            r if r.contains("designer") => vec![
                "User interface design and prototyping".to_string(),
                "User experience research and testing".to_string(),
                "Visual design and branding".to_string(),
                "Accessibility and usability optimization".to_string(),
            ],
            r if r.contains("manager") => vec![
                "Project planning and coordination".to_string(),
                "Resource allocation and optimization".to_string(),
                "Team leadership and communication".to_string(),
                "Risk assessment and mitigation".to_string(),
            ],
            _ => vec![
                format!("{} domain expertise", self.role),
                "Cross-functional collaboration".to_string(),
                "Problem analysis and solution development".to_string(),
                "Continuous learning and adaptation".to_string(),
            ],
        };

        // Activation contexts
        self.activation_contexts = vec![
            format!("When {} expertise is needed", self.role.to_lowercase()),
            format!("For {}-specific challenges", self.role.to_lowercase()),
            "When deep domain knowledge is required".to_string(),
            "For complex problem-solving in this area".to_string(),
        ];

        // Integration points - common collaborators
        self.integrations = vec![
            "**Manager**: For coordination and resource allocation".to_string(),
            "**Analyst**: For data-driven insights and validation".to_string(),
            "**Architect**: For system design alignment".to_string(),
            format!("**Other {} agents**: For specialized collaboration", self.role),
        ];

        // Basic principles
        self.principles = vec![
            format!("Excellence in {} practices", self.role.to_lowercase()),
            "Collaborative approach to problem-solving".to_string(),
            "Continuous learning and improvement".to_string(),
            "Clear communication and documentation".to_string(),
        ];

        // Success metrics
        self.metrics = vec![
            "Quality of deliverables and solutions".to_string(),
            "Efficiency of problem resolution".to_string(),
            "Stakeholder satisfaction and feedback".to_string(),
            "Knowledge transfer and documentation quality".to_string(),
        ];
    }

    /// Convert template to replacement map for processing
    pub fn to_replacement_map(&self) -> HashMap<String, String> {
        let now = Utc::now();
        let mut map = HashMap::new();

        // Basic agent information
        map.insert("AGENT_NAME".to_string(), self.name.clone());
        map.insert("AGENT_ROLE".to_string(), self.role.clone());
        map.insert("AGENT_PURPOSE".to_string(), self.purpose.clone());
        map.insert("AGENT_TAGLINE".to_string(), self.tagline.clone());

        // Lists as formatted strings
        map.insert("AGENT_COMPETENCIES".to_string(), self.format_list(&self.competencies));
        map.insert("AGENT_PRINCIPLES".to_string(), self.format_list(&self.principles));
        map.insert("AGENT_FRAMEWORKS".to_string(), self.format_frameworks());
        map.insert("AGENT_COLLABORATIONS".to_string(), self.format_list(&self.collaborations));
        map.insert("AGENT_ACTIVATION".to_string(), self.format_list(&self.activation_contexts));
        map.insert("AGENT_INTERACTION".to_string(), self.format_interaction_patterns());
        map.insert("AGENT_EXAMPLES".to_string(), self.format_examples());
        map.insert("AGENT_INTEGRATIONS".to_string(), self.format_list(&self.integrations));
        map.insert("AGENT_SPECIALIZATIONS".to_string(), self.format_list(&self.specializations));
        map.insert("AGENT_METRICS".to_string(), self.format_list(&self.metrics));
        map.insert("AGENT_LIMITATIONS".to_string(), self.format_limitations());

        // Date information
        map.insert("CURRENT_DATE".to_string(), now.format("%Y-%m-%d").to_string());
        map.insert("CURRENT_YEAR".to_string(), now.year().to_string());
        map.insert("CREATED_DATE".to_string(), now.format("%Y-%m-%d").to_string());

        map
    }

    fn format_list(&self, items: &[String]) -> String {
        if items.is_empty() {
            format!("- [To be defined based on {} experience]", self.role.to_lowercase())
        } else {
            items.iter().map(|item| format!("- {}", item)).collect::<Vec<_>>().join("\n")
        }
    }

    fn format_frameworks(&self) -> String {
        if self.frameworks.is_empty() {
            format!("**{}  Framework**: [To be developed based on best practices]\n\n**Methodology**: [To be refined through experience]\n\n**Standards**: [To be established based on domain requirements]", self.role)
        } else {
            self.frameworks.iter().map(|item| format!("**{}**", item)).collect::<Vec<_>>().join("\n\n")
        }
    }

    fn format_interaction_patterns(&self) -> String {
        if self.interaction_patterns.is_empty() {
            format!("**Primary Mode**: Direct {} consultation and problem-solving\n\n**Communication Style**: Clear, professional, and focused on actionable solutions\n\n**Engagement Pattern**: Analyze requirements → Develop solutions → Implement and validate → Document outcomes", self.role.to_lowercase())
        } else {
            self.interaction_patterns.join("\n\n")
        }
    }

    fn format_examples(&self) -> String {
        if self.examples.is_empty() {
            format!("# Example: {} Task\n[Specific example to be added based on actual use cases]\n\n# Common Workflow\n1. Analyze requirements\n2. Develop approach\n3. Implement solution\n4. Validate results\n5. Document outcomes", self.role)
        } else {
            self.examples.join("\n\n")
        }
    }

    fn format_limitations(&self) -> String {
        if self.limitations.is_empty() {
            format!("**Scope**: Focused specifically on {} domain - may need collaboration for cross-domain challenges\n\n**Dependencies**: Requires clear requirements and context for optimal performance\n\n**Constraints**: Subject to available tools and resources within the CI ecosystem", self.role.to_lowercase())
        } else {
            self.format_list(&self.limitations)
        }
    }
}

/// Template processor for applying replacements to template files
pub struct TemplateProcessor;

impl TemplateProcessor {
    /// Process a template file with the given replacements
    pub fn process_template<P: AsRef<Path>>(
        template_path: P,
        output_path: P,
        replacements: &HashMap<String, String>,
    ) -> Result<()> {
        let template_content = fs::read_to_string(&template_path)
            .with_context(|| format!("Failed to read template file: {}", template_path.as_ref().display()))?;

        let processed_content = Self::apply_replacements(&template_content, replacements);

        fs::write(&output_path, processed_content)
            .with_context(|| format!("Failed to write output file: {}", output_path.as_ref().display()))?;

        Ok(())
    }

    /// Apply replacements to template content
    fn apply_replacements(content: &str, replacements: &HashMap<String, String>) -> String {
        let mut result = content.to_string();
        
        for (key, value) in replacements {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        result
    }

    /// Get the Streamliner template directory path
    pub fn get_template_dir<P: AsRef<Path>>(repo_path: P) -> Result<std::path::PathBuf> {
        let template_dir = repo_path.as_ref()
            .join("AGENTS")
            .join("Streamliner")
            .join("templates");
        
        if !template_dir.exists() {
            return Err(anyhow::anyhow!("Streamliner template directory not found: {}", template_dir.display()));
        }
        
        Ok(template_dir)
    }

    /// Validate that all required template files exist
    pub fn validate_templates<P: AsRef<Path>>(template_dir: P) -> Result<()> {
        let required_files = [
            "AGENT_TEMPLATE_README.md",
            "AGENT_TEMPLATE_MEMORY.md",
            "AGENT_TEMPLATE_CONTINUOUS_LEARNING.md",
        ];

        for file in &required_files {
            let file_path = template_dir.as_ref().join(file);
            if !file_path.exists() {
                return Err(anyhow::anyhow!("Required template file missing: {}", file_path.display()));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_template_creation() {
        let template = AgentTemplate::new(
            "TestAgent".to_string(),
            "Testing Specialist".to_string(),
            "Comprehensive testing and quality assurance".to_string(),
        );

        assert_eq!(template.name, "TestAgent");
        assert_eq!(template.role, "Testing Specialist");
        assert!(!template.competencies.is_empty());
        assert!(!template.activation_contexts.is_empty());
    }

    #[test]
    fn test_template_replacement_map() {
        let template = AgentTemplate::new(
            "Projector".to_string(),
            "Idea Organization Specialist".to_string(),
            "Categorize and organize user ideas systematically".to_string(),
        );

        let replacements = template.to_replacement_map();
        
        assert_eq!(replacements.get("AGENT_NAME").unwrap(), "Projector");
        assert_eq!(replacements.get("AGENT_ROLE").unwrap(), "Idea Organization Specialist");
        assert!(replacements.contains_key("CURRENT_DATE"));
    }
}