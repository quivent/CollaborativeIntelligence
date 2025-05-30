// Documentation Commands for CIA CLI
// Provides access to system documentation and Matrix Quadrant capabilities

use crate::error::Result;
use crate::components::FooterComponent;
use colored::*;

static mut FOOTER_COMPONENT: Option<FooterComponent> = None;

fn get_footer_component() -> &'static mut FooterComponent {
    unsafe {
        if FOOTER_COMPONENT.is_none() {
            FOOTER_COMPONENT = Some(FooterComponent::new());
        }
        FOOTER_COMPONENT.as_mut().unwrap()
    }
}

/// Show all available documentation
pub fn show_all() -> Result<()> {
    let footer = get_footer_component();
    footer.show_documentation(None);
    Ok(())
}

/// Show specific documentation section
pub fn show_section(section: &str) -> Result<()> {
    let footer = get_footer_component();
    footer.show_documentation(Some(section));
    Ok(())
}

/// Toggle documentation visibility
pub fn toggle() -> Result<()> {
    let footer = get_footer_component();
    footer.toggle_documentation();
    
    if footer.is_documentation_visible() {
        println!("{}", "📚 Documentation now visible".green());
        footer.show_documentation(None);
    } else {
        println!("{}", "📖 Documentation hidden".blue());
    }
    
    Ok(())
}

/// Hide documentation
pub fn hide() -> Result<()> {
    let footer = get_footer_component();
    footer.hide_documentation();
    println!("{}", "📖 Documentation hidden".blue());
    Ok(())
}

/// Show available documentation sections
pub fn list_sections() -> Result<()> {
    let footer = get_footer_component();
    let sections = footer.get_available_sections();
    
    println!("{}", "📚 AVAILABLE DOCUMENTATION SECTIONS".cyan().bold());
    println!("{}", "=".repeat(50));
    
    for section in sections {
        match section.as_str() {
            "matrix" => println!("🔧 {} - Matrix Quadrant System", section.yellow().bold()),
            "security" => println!("🔐 {} - Security & Authentication", section.yellow().bold()),
            _ => println!("📖 {}", section.yellow().bold()),
        }
    }
    
    println!("\n💡 Usage:");
    println!("  {} - View specific section", "cia docs <section>".cyan());
    println!("  {} - Show all documentation", "cia docs".cyan());
    println!("  {} - Toggle documentation", "cia docs toggle".cyan());
    
    Ok(())
}

/// Render footer for other commands
pub fn render_footer() -> String {
    let footer = get_footer_component();
    footer.render_footer()
}

/// Render compact footer for other commands
pub fn render_compact_footer() -> String {
    let footer = get_footer_component();
    footer.render_compact_footer()
}

/// Check if documentation is currently visible
pub fn is_documentation_visible() -> bool {
    let footer = get_footer_component();
    footer.is_documentation_visible()
}

/// Quick access to Matrix Quadrant System documentation
pub fn matrix_help() -> Result<()> {
    println!("{}", "🔧 MATRIX QUADRANT SYSTEM - QUICK REFERENCE".cyan().bold());
    println!("{}", "=".repeat(60));
    
    println!("\n{}", "📊 Create Matrix:".green().bold());
    println!("  cargo run --bin matrix_cli create \\");
    println!("    --matrix-type {} \\", "priority".yellow());
    println!("    --title {} \\", "\"Task Analysis\"".yellow());
    println!("    --x-label {} \\", "\"Urgency →\"".yellow());
    println!("    --y-label {} \\", "\"Importance ↑\"".yellow());
    println!("    --output {}", "matrix.json".yellow());
    
    println!("\n{}", "➕ Add Items:".green().bold());
    println!("  cargo run --bin matrix_cli add \\");
    println!("    --file {} \\", "matrix.json".yellow());
    println!("    --id {} --name {} \\", "task1".yellow(), "\"Critical Bug\"".yellow());
    println!("    --x={} --y={} --score={}", "0.9".yellow(), "0.9".yellow(), "9.5".yellow());
    
    println!("\n{}", "👁️ View Matrix:".green().bold());
    println!("  cargo run --bin matrix_cli show --file {} {}", "matrix.json".yellow(), "--detailed".yellow());
    
    println!("\n{}", "📋 Available Templates:".green().bold());
    println!("  cargo run --bin matrix_cli {}", "templates".yellow());
    
    println!("\n{}", "🎯 Matrix Types:".blue().bold());
    println!("  • {} - Agent Performance Analysis", "performance".cyan());
    println!("  • {} - Task Priority Matrix (Eisenhower)", "priority".cyan());
    println!("  • {} - Agent Capability Assessment", "capability".cyan());
    println!("  • {} - Risk Assessment Matrix", "risk".cyan());
    println!("  • {} - Knowledge Gap Analysis", "knowledge".cyan());
    println!("  • {} - Decision Support Matrix", "decision".cyan());
    
    println!("\n💡 For full documentation: {}", "cia docs matrix".green());
    
    Ok(())
}

/// Quick access to Security System documentation
pub fn security_help() -> Result<()> {
    println!("{}", "🔐 SECURITY SYSTEM - QUICK REFERENCE".red().bold());
    println!("{}", "=".repeat(60));
    
    println!("\n{}", "🛡️ Verify Agent:".green().bold());
    println!("  cia security verify-agent {}", "Athena".yellow());
    
    println!("\n{}", "🎯 Challenge Identity:".green().bold());
    println!("  cia security challenge-identity {}", "Fixer".yellow());
    
    println!("\n{}", "📊 System Status:".green().bold());
    println!("  cia security {}", "system-status".yellow());
    
    println!("\n{}", "🚨 Emergency Mode:".green().bold());
    println!("  cia security {}", "emergency-mode".yellow());
    
    println!("\n{}", "📋 Generate Report:".green().bold());
    println!("  cia security {}", "report".yellow());
    
    println!("\n{}", "🧪 Test Enforcement:".green().bold());
    println!("  cia security test-enforcement {}", "AgentName".yellow());
    
    println!("\n{}", "ℹ️ Current Status:".blue().bold());
    println!("  • Emergency signatures active for 35 agents");
    println!("  • CIA CLI security enforcement implemented");
    println!("  • System operating in secure emergency mode");
    
    println!("\n💡 For full documentation: {}", "cia docs security".green());
    
    Ok(())
}