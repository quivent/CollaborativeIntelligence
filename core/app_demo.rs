// Application Demo with Footer Component
// Demonstrates the footer component with documentation toggle functionality

mod app_footer;

use app_footer::*;
use colored::*;
use std::io::{self, Write};

fn main() {
    println!("{}", "🚀 APPLICATION WITH FOOTER COMPONENT DEMO".cyan().bold());
    println!("{}", "=".repeat(60));
    
    // Update context
    update_context("CollaborativeIntelligence → Athena → Application Demo".to_string());
    
    println!("\n{}", "📋 Available Commands:".green().bold());
    println!("  {} - Toggle documentation", "help".cyan());
    println!("  {} - Show Matrix system help", "help matrix".cyan());
    println!("  {} - Show Security system help", "help security".cyan());
    println!("  {} - Show Terminal management help", "help terminal".cyan());
    println!("  {} - Show all documentation", "show".cyan());
    println!("  {} - Hide documentation", "hide".cyan());
    println!("  {} - Exit demo", "exit".cyan());
    
    // Show initial footer
    println!("{}", render_footer());
    
    // Interactive loop
    loop {
        print!("\n💡 Enter command: ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();
        
        if command == "exit" {
            println!("{}", "👋 Goodbye!".green());
            break;
        }
        
        // Handle footer commands
        let response = get_app_footer().handle_command(command);
        
        if !response.is_empty() && !response.contains("Available commands") {
            println!("{}", response.blue());
        } else if response.contains("Available commands") {
            println!("{}", response.yellow());
        }
        
        // Always show updated footer
        println!("{}", render_footer());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_footer_integration() {
        // Test that footer renders without errors
        let footer_output = render_footer();
        assert!(footer_output.contains("CollaborativeIntelligence"));
        assert!(footer_output.contains("matrix"));
        assert!(footer_output.contains("secure"));
    }

    #[test]
    fn test_documentation_toggle() {
        // Test documentation toggle functionality
        let response = toggle_docs();
        assert!(response.contains("Documentation"));
    }

    #[test]
    fn test_context_update() {
        // Test context updating
        update_context("Test Context".to_string());
        let footer_output = render_footer();
        assert!(footer_output.contains("Test Context"));
    }
}