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

        // Generate intelligent defaults based on purpose and role
        template.generate_intelligent_defaults();
        template
    }

    /// Generate intelligent defaults by parsing purpose and role
    fn generate_intelligent_defaults(&mut self) {
        // Parse the purpose statement for key concepts and behaviors
        self.parse_purpose_statement();
        
        // Apply role-based enhancements
        self.enhance_with_role_patterns();
        
        // Generate contextual integration points
        self.generate_contextual_integrations();
    }
    
    /// Parse the purpose statement to extract specific competencies and behaviors
    fn parse_purpose_statement(&mut self) {
        let purpose_lower = self.purpose.to_lowercase();
        
        // Extract action verbs and concepts
        let key_actions = self.extract_action_verbs(&purpose_lower);
        let key_concepts = self.extract_domain_concepts(&purpose_lower);
        let methodologies = self.extract_methodologies(&purpose_lower);
        
        // Build competencies from extracted concepts
        self.competencies = self.build_competencies_from_purpose(&key_actions, &key_concepts, &methodologies);
        
        // Build activation contexts from purpose
        self.activation_contexts = self.build_activation_contexts(&key_actions, &key_concepts);
        
        // Extract specific frameworks mentioned
        self.frameworks = self.extract_frameworks_from_purpose(&purpose_lower);
    }
    
    /// Extract action verbs from purpose statement
    fn extract_action_verbs(&self, purpose: &str) -> Vec<String> {
        let action_patterns = [
            "categorize", "organize", "classify", "file", "transform", "structure",
            "analyze", "process", "manage", "coordinate", "develop", "implement",
            "design", "create", "build", "optimize", "monitor", "track", "evaluate"
        ];
        
        action_patterns.iter()
            .filter(|&action| purpose.contains(action))
            .map(|&action| action.to_string())
            .collect()
    }
    
    /// Extract domain-specific concepts
    fn extract_domain_concepts(&self, purpose: &str) -> Vec<String> {
        let mut concepts = Vec::new();
        
        // Look for specific domain terms
        if purpose.contains("idea") { concepts.push("idea management".to_string()); }
        if purpose.contains("project") { concepts.push("project categorization".to_string()); }
        if purpose.contains("classification") { concepts.push("intelligent classification".to_string()); }
        if purpose.contains("filing") { concepts.push("systematic filing".to_string()); }
        if purpose.contains("category") || purpose.contains("categor") { 
            concepts.push("categorical organization".to_string()); 
        }
        if purpose.contains("standard") { concepts.push("standardization".to_string()); }
        if purpose.contains("systematic") { concepts.push("systematic approaches".to_string()); }
        
        // Extract specific categories mentioned
        let category_patterns = ["project", "migration", "enhancement", "task", "integration"];
        for pattern in &category_patterns {
            if purpose.contains(pattern) {
                concepts.push(format!("{} classification", pattern));
            }
        }
        
        concepts
    }
    
    /// Extract methodologies from purpose
    fn extract_methodologies(&self, purpose: &str) -> Vec<String> {
        let mut methodologies = Vec::new();
        
        if purpose.contains("intelligent") && purpose.contains("classification") {
            methodologies.push("AI-driven categorization".to_string());
        }
        if purpose.contains("systematic") {
            methodologies.push("systematic organization".to_string());
        }
        if purpose.contains("standardized") {
            methodologies.push("standardized processes".to_string());
        }
        if purpose.contains("decision") || purpose.contains("choice") {
            methodologies.push("autonomous decision-making".to_string());
        }
        
        methodologies
    }
    
    /// Build competencies from extracted purpose elements
    fn build_competencies_from_purpose(
        &self, 
        actions: &[String], 
        concepts: &[String], 
        methodologies: &[String]
    ) -> Vec<String> {
        let mut competencies = Vec::new();
        
        // Create competencies from actions + concepts
        for action in actions {
            for concept in concepts {
                competencies.push(format!("{} and {}", 
                    action.chars().next().unwrap().to_uppercase().collect::<String>() + &action[1..],
                    concept
                ));
            }
        }
        
        // Add methodology competencies
        for methodology in methodologies {
            competencies.push(methodology.clone());
        }
        
        // Add communication competencies if purpose suggests interaction
        if self.purpose.to_lowercase().contains("answer") || 
           self.purpose.to_lowercase().contains("communicate") ||
           self.purpose.to_lowercase().contains("understand") {
            competencies.push("Conversational intelligence and user interaction".to_string());
            competencies.push("Contextual understanding and explanation".to_string());
        }
        
        // Ensure we have at least basic competencies
        if competencies.is_empty() {
            competencies.push(format!("{} domain expertise", self.role));
            competencies.push("Problem analysis and solution development".to_string());
        }
        
        // Remove duplicates and limit to reasonable number
        competencies.sort();
        competencies.dedup();
        competencies.truncate(6);
        
        competencies
    }
    
    /// Build activation contexts from purpose elements
    fn build_activation_contexts(&self, actions: &[String], concepts: &[String]) -> Vec<String> {
        let mut contexts = Vec::new();
        
        // Build contexts from actions
        for action in actions {
            contexts.push(format!("When {} is required", action));
        }
        
        // Build contexts from concepts
        for concept in concepts {
            contexts.push(format!("For {} challenges", concept));
        }
        
        // Add purpose-specific contexts
        if self.purpose.to_lowercase().contains("decision") {
            contexts.push("When autonomous decision-making is needed".to_string());
        }
        
        // Ensure we have basic contexts
        if contexts.is_empty() {
            contexts.push(format!("When {} expertise is needed", self.role.to_lowercase()));
        }
        
        contexts.truncate(4);
        contexts
    }
    
    /// Extract frameworks from purpose
    fn extract_frameworks_from_purpose(&self, purpose: &str) -> Vec<String> {
        let mut frameworks = Vec::new();
        
        if purpose.contains("categor") {
            frameworks.push("**Categorization Framework**: Multi-dimensional classification system for systematic organization".to_string());
        }
        if purpose.contains("filing") {
            frameworks.push("**Filing Methodology**: Standardized storage and retrieval protocols".to_string());
        }
        if purpose.contains("intelligent") {
            frameworks.push("**Decision Engine**: AI-driven classification and routing logic".to_string());
        }
        
        frameworks
    }
    
    /// Apply role-based enhancements to the purpose-derived content
    fn enhance_with_role_patterns(&mut self) {
        let role_lower = self.role.to_lowercase();
        
        // Add role-specific enhancements to existing competencies
        let role_enhancements = match role_lower.as_str() {
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
            ],
        };
        
        // Add role-specific competencies to purpose-derived ones (don't replace)
        for enhancement in role_enhancements {
            if !self.competencies.contains(&enhancement) {
                self.competencies.push(enhancement);
            }
        }
        
        // Enhance principles with role-specific ones
        let role_principles = vec![
            format!("Excellence in {} practices", self.role.to_lowercase()),
            "Collaborative approach to problem-solving".to_string(),
            "Continuous learning and improvement".to_string(),
            "Clear communication and documentation".to_string(),
        ];
        
        for principle in role_principles {
            if !self.principles.contains(&principle) {
                self.principles.push(principle);
            }
        }
        
        // Add standard success metrics if not already derived from purpose
        let standard_metrics = vec![
            "Quality of deliverables and solutions".to_string(),
            "Efficiency of problem resolution".to_string(),
            "Stakeholder satisfaction and feedback".to_string(),
            "Knowledge transfer and documentation quality".to_string(),
        ];
        
        for metric in standard_metrics {
            if !self.metrics.contains(&metric) {
                self.metrics.push(metric);
            }
        }
    }
    
    /// Generate contextual integration points based on purpose and role
    fn generate_contextual_integrations(&mut self) {
        // Start with basic integrations
        self.integrations = vec![
            "**Manager**: For coordination and resource allocation".to_string(),
        ];
        
        // Add purpose-specific integrations
        let purpose_lower = self.purpose.to_lowercase();
        
        if purpose_lower.contains("analyz") || purpose_lower.contains("data") {
            self.integrations.push("**Analyst**: For data-driven insights and validation".to_string());
        }
        if purpose_lower.contains("system") || purpose_lower.contains("architect") {
            self.integrations.push("**Architect**: For system design alignment".to_string());
        }
        if purpose_lower.contains("design") || purpose_lower.contains("interface") {
            self.integrations.push("**Designer**: For user experience considerations".to_string());
        }
        if purpose_lower.contains("document") || purpose_lower.contains("report") {
            self.integrations.push("**Documenter**: For comprehensive documentation".to_string());
        }
        
        // Add role-based integrations
        self.integrations.push(format!("**Other {} agents**: For specialized collaboration", self.role));
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