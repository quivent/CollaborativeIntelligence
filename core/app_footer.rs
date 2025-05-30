// Application Footer Component with Documentation Toggle
// Provides help and feature documentation for the local application

use colored::*;
use std::collections::HashMap;

pub struct AppFooter {
    docs_visible: bool,
    current_context: String,
    features: HashMap<String, FeatureDoc>,
}

#[derive(Clone)]
pub struct FeatureDoc {
    pub name: String,
    pub description: String,
    pub usage: Vec<String>,
    pub examples: Vec<String>,
    pub status: String,
}

impl AppFooter {
    pub fn new() -> Self {
        let mut footer = AppFooter {
            docs_visible: false,
            current_context: "CollaborativeIntelligence → Athena → Application".to_string(),
            features: HashMap::new(),
        };
        footer.initialize_features();
        footer
    }

    fn initialize_features(&mut self) {
        // Matrix Quadrant System
        let matrix_feature = FeatureDoc {
            name: "Matrix Quadrant System".to_string(),
            description: "Complete quadrant-based analysis system with 6 matrix types for strategic decision-making and categorization.".to_string(),
            usage: vec![
                "Create matrices with performance, priority, capability, risk, knowledge, or decision types".to_string(),
                "Position items on continuous coordinates (-1.0 to 1.0) across quadrants".to_string(),
                "Add metadata, scores, and descriptions to items".to_string(),
                "Export/import via JSON for data persistence".to_string(),
                "Visual ASCII rendering with quadrant layouts".to_string(),
            ],
            examples: vec![
                "// Create priority matrix".to_string(),
                "let mut matrix = MatrixTemplates::priority_matrix();".to_string(),
                "".to_string(),
                "// Add item to quadrant".to_string(),
                "let item = MatrixItem::new(\"task1\".to_string(), \"Critical Fix\".to_string(), 0.9, 0.9)".to_string(),
                "    .with_description(\"Security vulnerability\".to_string())".to_string(),
                "    .with_score(9.8);".to_string(),
                "matrix.add_item(item);".to_string(),
                "".to_string(),
                "// Render visualization".to_string(),
                "println!(\"{}\", matrix.render_ascii());".to_string(),
            ],
            status: "✅ OPERATIONAL".to_string(),
        };
        self.features.insert("matrix".to_string(), matrix_feature);

        // Security System
        let security_feature = FeatureDoc {
            name: "Security Authentication System".to_string(),
            description: "Emergency authentication system with cryptographic signatures for all agents and operations.".to_string(),
            usage: vec![
                "Verify agent identity with cryptographic signatures".to_string(),
                "Emergency mode authentication for 35 agents".to_string(),
                "Pre-operation security checks and enforcement".to_string(),
                "Audit logging for all authentication attempts".to_string(),
                "Challenge/response protocols for identity verification".to_string(),
            ],
            examples: vec![
                "// Verify agent signature".to_string(),
                "let manager = SignatureManager::new();".to_string(),
                "match manager.verify_agent(\"Athena\") {".to_string(),
                "    Ok(true) => println!(\"✅ AUTHENTICATED\"),".to_string(),
                "    _ => println!(\"❌ AUTHENTICATION FAILED\"),".to_string(),
                "}".to_string(),
                "".to_string(),
                "// Get system status".to_string(),
                "println!(\"{}\", manager.get_system_status());".to_string(),
            ],
            status: "🟢 SECURE".to_string(),
        };
        self.features.insert("security".to_string(), security_feature);

        // Terminal Management
        let terminal_feature = FeatureDoc {
            name: "Terminal Title Management".to_string(),
            description: "Dynamic terminal title formatting with context awareness and protocol compliance.".to_string(),
            usage: vec![
                "Update terminal title with current context".to_string(),
                "Format: Project → Agent → Task".to_string(),
                "Arrow syntax (→) instead of pipes".to_string(),
                "Real-time context updates".to_string(),
                "Protocol-compliant formatting".to_string(),
            ],
            examples: vec![
                "// Set terminal title".to_string(),
                "echo -n \"\\033]0;CollaborativeIntelligence → Athena → Task\\007\"".to_string(),
                "".to_string(),
                "// Current format".to_string(),
                "CollaborativeIntelligence → Athena → Critical Security Response".to_string(),
            ],
            status: "✅ COMPLIANT".to_string(),
        };
        self.features.insert("terminal".to_string(), terminal_feature);
    }

    pub fn toggle_docs(&mut self) {
        self.docs_visible = !self.docs_visible;
    }

    pub fn show_docs(&mut self) {
        self.docs_visible = true;
    }

    pub fn hide_docs(&mut self) {
        self.docs_visible = false;
    }

    pub fn render(&self) -> String {
        let doc_indicator = if self.docs_visible { "📚 DOCS" } else { "📖 docs" };
        
        format!(
            "\n{}\n{} | {} | {} | {} {}\n{}{}",
            "─".repeat(80),
            self.current_context.white().bold(),
            "🔧 matrix".cyan(),
            "🔐 secure".green(),
            doc_indicator.blue(),
            "| help - toggle documentation".dimmed(),
            "─".repeat(80),
            if self.docs_visible { self.render_documentation() } else { String::new() }
        )
    }

    fn render_documentation(&self) -> String {
        let mut output = String::new();
        
        output.push_str(&format!("\n{}", "📚 APPLICATION DOCUMENTATION".cyan().bold()));
        output.push_str(&format!("\n{}", "=".repeat(80)));
        
        // Feature overview
        output.push_str(&format!("\n\n{}", "🎯 AVAILABLE FEATURES:".green().bold()));
        for (key, feature) in &self.features {
            output.push_str(&format!(
                "\n  {} {} - {} {}",
                match key.as_str() {
                    "matrix" => "🔧",
                    "security" => "🔐", 
                    "terminal" => "📺",
                    _ => "📖"
                },
                feature.name.yellow().bold(),
                feature.description,
                feature.status.green()
            ));
        }

        // Quick examples
        output.push_str(&format!("\n\n{}", "💡 QUICK EXAMPLES:".blue().bold()));
        
        if let Some(matrix_feature) = self.features.get("matrix") {
            output.push_str(&format!("\n\n{}", "🔧 Matrix System:".cyan().bold()));
            for example in &matrix_feature.examples[0..5] {
                if example.trim().is_empty() {
                    output.push('\n');
                } else if example.starts_with("//") {
                    output.push_str(&format!("\n  {}", example.green()));
                } else {
                    output.push_str(&format!("\n  {}", example.cyan()));
                }
            }
        }

        if let Some(security_feature) = self.features.get("security") {
            output.push_str(&format!("\n\n{}", "🔐 Security System:".red().bold()));
            for example in &security_feature.examples[0..4] {
                if example.trim().is_empty() {
                    output.push('\n');
                } else if example.starts_with("//") {
                    output.push_str(&format!("\n  {}", example.green()));
                } else {
                    output.push_str(&format!("\n  {}", example.cyan()));
                }
            }
        }

        // Usage commands
        output.push_str(&format!("\n\n{}", "🚀 USAGE COMMANDS:".yellow().bold()));
        output.push_str(&format!("\n  {} - Toggle this documentation", "help".cyan()));
        output.push_str(&format!("\n  {} - Show specific feature help", "help <feature>".cyan()));
        output.push_str(&format!("\n  {} - Available: matrix, security, terminal", "Features".dimmed()));
        
        // System status
        output.push_str(&format!("\n\n{}", "📊 CURRENT STATUS:".blue().bold()));
        output.push_str(&format!("\n  🔧 Matrix System: {}", "OPERATIONAL".green()));
        output.push_str(&format!("\n  🔐 Security System: {}", "SECURE (Emergency Mode)".green()));
        output.push_str(&format!("\n  📺 Terminal Management: {}", "COMPLIANT".green()));
        output.push_str(&format!("\n  📚 Documentation: {}", if self.docs_visible { "VISIBLE" } else { "HIDDEN" }.blue()));

        output.push_str(&format!("\n{}", "─".repeat(80)));
        output
    }

    pub fn show_feature_help(&self, feature_name: &str) -> String {
        if let Some(feature) = self.features.get(feature_name) {
            let mut output = String::new();
            
            output.push_str(&format!("{} {}", 
                match feature_name {
                    "matrix" => "🔧",
                    "security" => "🔐",
                    "terminal" => "📺",
                    _ => "📖"
                }, 
                feature.name.cyan().bold()
            ));
            output.push_str(&format!("\n{}", "=".repeat(60)));
            
            output.push_str(&format!("\n\n{}", feature.description));
            output.push_str(&format!("\nStatus: {}", feature.status));
            
            output.push_str(&format!("\n\n{}", "📋 CAPABILITIES:".green().bold()));
            for usage in &feature.usage {
                output.push_str(&format!("\n  • {}", usage));
            }
            
            output.push_str(&format!("\n\n{}", "💡 EXAMPLES:".blue().bold()));
            for example in &feature.examples {
                if example.trim().is_empty() {
                    output.push('\n');
                } else if example.starts_with("//") {
                    output.push_str(&format!("\n{}", example.green()));
                } else {
                    output.push_str(&format!("\n{}", example.cyan()));
                }
            }
            
            output
        } else {
            format!("❌ Feature '{}' not found. Available: {}", 
                feature_name, 
                self.features.keys().cloned().collect::<Vec<_>>().join(", ")
            )
        }
    }

    pub fn update_context(&mut self, new_context: String) {
        self.current_context = new_context;
    }

    pub fn is_docs_visible(&self) -> bool {
        self.docs_visible
    }

    pub fn get_available_features(&self) -> Vec<String> {
        self.features.keys().cloned().collect()
    }

    pub fn handle_command(&mut self, command: &str) -> String {
        match command.trim() {
            "help" => {
                self.toggle_docs();
                if self.docs_visible {
                    "📚 Documentation now visible".to_string()
                } else {
                    "📖 Documentation hidden".to_string()
                }
            },
            cmd if cmd.starts_with("help ") => {
                let feature = cmd.strip_prefix("help ").unwrap();
                self.show_feature_help(feature)
            },
            "show" => {
                self.show_docs();
                "📚 Documentation shown".to_string()
            },
            "hide" => {
                self.hide_docs();
                "📖 Documentation hidden".to_string()
            },
            _ => "Available commands: help, help <feature>, show, hide".to_string()
        }
    }
}

// Global footer instance for application use
static mut APP_FOOTER: Option<AppFooter> = None;

pub fn get_app_footer() -> &'static mut AppFooter {
    unsafe {
        if APP_FOOTER.is_none() {
            APP_FOOTER = Some(AppFooter::new());
        }
        APP_FOOTER.as_mut().unwrap()
    }
}

// Convenience functions for easy use
pub fn render_footer() -> String {
    get_app_footer().render()
}

pub fn toggle_docs() -> String {
    get_app_footer().handle_command("help")
}

pub fn show_feature_help(feature: &str) -> String {
    get_app_footer().show_feature_help(feature)
}

pub fn update_context(context: String) {
    get_app_footer().update_context(context);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_creation() {
        let footer = AppFooter::new();
        assert!(!footer.is_docs_visible());
        assert_eq!(footer.get_available_features().len(), 3);
    }

    #[test]
    fn test_documentation_toggle() {
        let mut footer = AppFooter::new();
        assert!(!footer.is_docs_visible());
        
        footer.toggle_docs();
        assert!(footer.is_docs_visible());
        
        footer.toggle_docs();
        assert!(!footer.is_docs_visible());
    }

    #[test]
    fn test_feature_help() {
        let footer = AppFooter::new();
        let help = footer.show_feature_help("matrix");
        assert!(help.contains("Matrix Quadrant System"));
        assert!(help.contains("CAPABILITIES"));
        assert!(help.contains("EXAMPLES"));
    }

    #[test]
    fn test_command_handling() {
        let mut footer = AppFooter::new();
        
        let response = footer.handle_command("help");
        assert!(response.contains("Documentation"));
        
        let response = footer.handle_command("help matrix");
        assert!(response.contains("Matrix Quadrant System"));
        
        let response = footer.handle_command("invalid");
        assert!(response.contains("Available commands"));
    }
}