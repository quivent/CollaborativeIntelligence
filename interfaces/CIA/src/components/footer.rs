// Footer Component with Documentation Toggle
// Provides access to Matrix Quadrant System documentation and capabilities

use std::collections::HashMap;
use colored::*;

pub struct FooterComponent {
    documentation_visible: bool,
    available_docs: HashMap<String, DocumentationSection>,
}

#[derive(Clone)]
pub struct DocumentationSection {
    pub title: String,
    pub content: String,
    pub examples: Vec<String>,
    pub file_path: Option<String>,
}

impl FooterComponent {
    pub fn new() -> Self {
        let mut component = FooterComponent {
            documentation_visible: false,
            available_docs: HashMap::new(),
        };
        
        component.initialize_documentation();
        component
    }

    fn initialize_documentation(&mut self) {
        // Matrix Quadrant System Documentation
        let matrix_doc = DocumentationSection {
            title: "Matrix Quadrant System".to_string(),
            content: format!(
                "{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
                "📊 MATRIX QUADRANT SYSTEM".cyan().bold(),
                "🏗️ Core Architecture:",
                "  • Flexible QuadrantMatrix structure supporting multiple analysis types",
                "  • 6 predefined matrix types: Performance, Priority, Capability, Risk, Knowledge, Decision",
                "  • Precise positioning system with continuous coordinates (-1.0 to 1.0)",
                "  • Rich metadata support with descriptions, scores, and custom attributes",
                
                "🛠️ CLI Tool Features:",
                "  • Interactive matrix creation with templates",
                "  • Visual ASCII rendering of quadrant layouts",
                "  • Item management (add, move, remove items)",
                "  • JSON import/export for data persistence",
                "  • Detailed analysis views showing item positioning and metadata",
                
                "🎯 Key Features:",
                "  • Quadrant-based categorization with smart positioning",
                "  • Multiple matrix templates for different analysis needs",
                "  • Command-line interface for easy interaction",
                "  • Visual representation with clear quadrant boundaries",
                "  • Scoring and metadata for comprehensive analysis",
                
                "📋 Available Matrix Types:",
                "  • performance - Agent Performance Analysis",
                "  • priority - Task Priority Matrix (Eisenhower)",
                "  • capability - Agent Capability Assessment", 
                "  • risk - Risk Assessment Matrix",
                "  • knowledge - Knowledge Gap Analysis",
                "  • decision - Decision Support Matrix"
            ),
            examples: vec![
                "# Create a priority matrix".to_string(),
                "cargo run --bin matrix_cli create --matrix-type priority --title \"Task Analysis\" --x-label \"Urgency →\" --y-label \"Importance ↑\"".to_string(),
                "".to_string(),
                "# Add items to matrix".to_string(),
                "cargo run --bin matrix_cli add --file matrix.json --id task1 --name \"Critical Bug\" --x=0.9 --y=0.9 --score=9.5".to_string(),
                "".to_string(),
                "# View matrix visualization".to_string(),
                "cargo run --bin matrix_cli show --file matrix.json --detailed".to_string(),
                "".to_string(),
                "# List available templates".to_string(),
                "cargo run --bin matrix_cli templates".to_string(),
            ],
            file_path: Some("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/docs/matrix-system/README.md".to_string()),
        };
        
        self.available_docs.insert("matrix".to_string(), matrix_doc);

        // Security System Documentation
        let security_doc = DocumentationSection {
            title: "Security & Authentication System".to_string(),
            content: format!(
                "{}\n\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}\n{}\n\n{}\n{}\n{}\n{}\n{}",
                "🔐 SECURITY SYSTEM".red().bold(),
                
                "🛡️ Emergency Authentication:",
                "  • Cryptographic signature verification for all agents",
                "  • CI integration with pre-operation security checks",
                "  • Emergency authentication protocols for incident response",
                "  • Complete audit trail of authentication events",
                
                "🚨 Current Status:",
                "  • Emergency signatures active for 35 agents",
                "  • CIA CLI security enforcement implemented",
                "  • User verification tools available",
                "  • Authentication bypass vulnerability resolved",
                "  • System operating in secure emergency mode",
                
                "⚡ Available Commands:",
                "  • cia security verify-agent <name> - Verify agent identity",
                "  • cia security challenge-identity <name> - Challenge agent proof",
                "  • cia security system-status - Show security system status",
                "  • cia security emergency-mode - Show emergency information"
            ),
            examples: vec![
                "# Verify agent signature".to_string(),
                "cia security verify-agent Athena".to_string(),
                "".to_string(),
                "# Challenge agent identity".to_string(), 
                "cia security challenge-identity Fixer".to_string(),
                "".to_string(),
                "# Check system security status".to_string(),
                "cia security system-status".to_string(),
            ],
            file_path: Some("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/core/security/signatures/".to_string()),
        };
        
        self.available_docs.insert("security".to_string(), security_doc);
    }

    pub fn toggle_documentation(&mut self) {
        self.documentation_visible = !self.documentation_visible;
    }

    pub fn show_documentation(&mut self, section: Option<&str>) {
        self.documentation_visible = true;
        
        match section {
            Some(key) => {
                if let Some(doc) = self.available_docs.get(key) {
                    self.render_documentation_section(doc);
                } else {
                    self.show_available_sections();
                }
            },
            None => self.show_all_documentation(),
        }
    }

    pub fn hide_documentation(&mut self) {
        self.documentation_visible = false;
    }

    fn show_all_documentation(&self) {
        println!("\n{}", "📚 COLLABORATIVE INTELLIGENCE DOCUMENTATION".cyan().bold());
        println!("{}", "=".repeat(80));
        
        for (key, doc) in &self.available_docs {
            println!("\n{} {}", "📖".yellow(), doc.title.white().bold());
            println!("{}", "─".repeat(60));
            
            // Show first few lines of content
            let lines: Vec<&str> = doc.content.lines().take(8).collect();
            for line in lines {
                println!("{}", line);
            }
            
            if doc.content.lines().count() > 8 {
                println!("   {} Use 'cia docs {}' for full documentation", "...".dimmed(), key);
            }
            
            // Show first example
            if !doc.examples.is_empty() && !doc.examples[0].trim().is_empty() {
                println!("\n   {} {}", "💡 Quick Example:".green(), doc.examples[1].cyan());
            }
        }
        
        println!("\n{}", "🔧 USAGE COMMANDS:".yellow().bold());
        println!("  {} View specific section", "cia docs <section>".cyan());
        println!("  {} Available sections: {}", "cia docs".cyan(), 
            self.available_docs.keys().map(|k| k.as_str()).collect::<Vec<_>>().join(", ").green());
        println!("  {} Hide documentation", "cia docs hide".cyan());
        println!("  {} Toggle documentation", "cia docs toggle".cyan());
    }

    fn show_available_sections(&self) {
        println!("\n{}", "📚 AVAILABLE DOCUMENTATION SECTIONS".cyan().bold());
        println!("{}", "=".repeat(60));
        
        for (key, doc) in &self.available_docs {
            println!("\n🔸 {} - {}", key.yellow().bold(), doc.title);
            if let Some(path) = &doc.file_path {
                println!("   📁 {}", path.dimmed());
            }
        }
        
        println!("\n💡 Usage: {} to view specific section", "cia docs <section>".cyan());
    }

    fn render_documentation_section(&self, doc: &DocumentationSection) {
        println!("\n{}", doc.title.cyan().bold());
        println!("{}", "=".repeat(doc.title.len() + 10));
        
        println!("{}", doc.content);
        
        if !doc.examples.is_empty() {
            println!("\n{}", "💡 USAGE EXAMPLES:".green().bold());
            println!("{}", "─".repeat(40));
            for example in &doc.examples {
                if example.trim().is_empty() {
                    println!();
                } else if example.starts_with('#') {
                    println!("{}", example.yellow());
                } else {
                    println!("{}", example.cyan());
                }
            }
        }
        
        if let Some(path) = &doc.file_path {
            println!("\n{} {}", "📁 Full Documentation:".blue(), path.dimmed());
        }
    }

    pub fn render_footer(&self) -> String {
        let status_indicator = if self.documentation_visible {
            "📚 DOCS VISIBLE".green()
        } else {
            "📖 docs".blue()
        };
        
        let matrix_status = "🔧 matrix".cyan();
        let security_status = "🔐 secure".green();
        
        format!(
            "\n{}\n{} | {} | {} | {} {}\n{}",
            "─".repeat(80),
            "CollaborativeIntelligence".white().bold(),
            matrix_status,
            security_status,
            status_indicator,
            "| cia docs [section] - View documentation".dimmed(),
            "─".repeat(80)
        )
    }

    pub fn render_compact_footer(&self) -> String {
        let doc_toggle = if self.documentation_visible { "📚" } else { "📖" };
        format!("{} {} | {} | {}", 
            "CI".white().bold(), 
            "🔧".cyan(), 
            "🔐".green(),
            doc_toggle.blue()
        )
    }

    pub fn is_documentation_visible(&self) -> bool {
        self.documentation_visible
    }

    pub fn get_available_sections(&self) -> Vec<String> {
        self.available_docs.keys().cloned().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_component_creation() {
        let footer = FooterComponent::new();
        assert!(!footer.is_documentation_visible());
        assert!(!footer.available_docs.is_empty());
    }

    #[test]
    fn test_documentation_toggle() {
        let mut footer = FooterComponent::new();
        assert!(!footer.is_documentation_visible());
        
        footer.toggle_documentation();
        assert!(footer.is_documentation_visible());
        
        footer.toggle_documentation();
        assert!(!footer.is_documentation_visible());
    }

    #[test]
    fn test_available_sections() {
        let footer = FooterComponent::new();
        let sections = footer.get_available_sections();
        assert!(sections.contains(&"matrix".to_string()));
        assert!(sections.contains(&"security".to_string()));
    }
}