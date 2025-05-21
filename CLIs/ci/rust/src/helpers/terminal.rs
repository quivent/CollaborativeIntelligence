//! Terminal output formatting for CI CLI
//!
//! This module provides helper functions for terminal output formatting,
//! including colored text, symbols, and structured output formats.

use colored::*;
use std::io::{self, Write};

/// Theme colors for UI elements
pub struct ThemeColors;

#[allow(dead_code)]
impl ThemeColors {
    pub const PRIMARY: &'static str = "blue";      // Primary UI elements
    pub const SECONDARY: &'static str = "cyan";    // Secondary UI elements
    pub const ACCENT: &'static str = "magenta";    // Accent elements
    pub const SUCCESS: &'static str = "green";     // Success messages
    pub const WARNING: &'static str = "yellow";    // Warning messages
    pub const ERROR: &'static str = "red";         // Error messages
    pub const INFO: &'static str = "white";        // Informational messages
    
    // Command category colors
    pub const INIT: &'static str = "cyan";         // Init commands
    pub const INTEGRATE: &'static str = "blue";    // Integrate commands
    pub const STATUS: &'static str = "green";      // Status commands
    pub const FIX: &'static str = "yellow";        // Fix commands
    pub const VERIFY: &'static str = "magenta";    // Verify commands
    pub const LOAD: &'static str = "white";        // Load commands
}

/// Symbol definitions for CI CLI
pub struct Symbols;

#[allow(dead_code)]
impl Symbols {
    // Status symbols
    pub const SUCCESS: &'static str = "✓";
    pub const ERROR: &'static str = "✗";
    pub const WARNING: &'static str = "!";
    pub const INFO: &'static str = "i";
    
    // Command category symbols
    pub const INIT: &'static str = "I";
    pub const INTEGRATE: &'static str = "+";
    pub const STATUS: &'static str = "?";
    pub const FIX: &'static str = "F";
    pub const VERIFY: &'static str = "V";
    pub const LOAD: &'static str = "L";
    
    // UI elements
    pub const BULLET: &'static str = "•";
    pub const ARROW: &'static str = "->";
    pub const CHECKMARK: &'static str = "✓";
    pub const CROSS: &'static str = "×";
    pub const STAR: &'static str = "*";
    pub const DOT: &'static str = "·";
    
    // Box drawing characters
    pub const TOP_LEFT: &'static str = "+";
    pub const TOP_RIGHT: &'static str = "+";
    pub const BOTTOM_LEFT: &'static str = "+";
    pub const BOTTOM_RIGHT: &'static str = "+";
    pub const HORIZONTAL: &'static str = "-";
    pub const VERTICAL: &'static str = "|";
    pub const T_DOWN: &'static str = "+";
    pub const T_UP: &'static str = "+";
    pub const T_RIGHT: &'static str = "+";
    pub const T_LEFT: &'static str = "+";
    pub const CROSS_SYMBOL: &'static str = "+";
}

/// Helper functions for terminal output formatting
pub struct TerminalHelpers;

#[allow(dead_code)]
impl TerminalHelpers {
    /// Set the terminal window title
    pub fn set_terminal_title(title: &str) {
        print!("\x1B]0;CI: {}\x07", title);
        io::stdout().flush().unwrap_or_default();
    }
    
    /// Apply color to text based on color name
    fn apply_color(text: &str, color: &str) -> ColoredString {
        match color {
            ThemeColors::PRIMARY => text.blue(),
            ThemeColors::SECONDARY => text.cyan(),
            ThemeColors::ACCENT => text.magenta(),
            ThemeColors::SUCCESS => text.green(),
            ThemeColors::WARNING => text.yellow(),
            ThemeColors::ERROR => text.red(),
            "white" => text.white(),
            _ => text.normal(),
        }
    }
    
    /// Print a section header with formatting
    pub fn print_section(title: &str, color: &str) {
        println!("\n{}", Self::styled_header(title, color));
    }
    
    /// Create a styled header with custom formatting
    pub fn styled_header(title: &str, color: &str) -> ColoredString {
        let header = format!("=== {} ===", title);
        Self::apply_color(&header, color).bold()
    }
    
    /// Print a success message with check mark
    pub fn print_success(message: &str) {
        println!("{} {}", Symbols::SUCCESS.green().bold(), message.green());
    }
    
    /// Print an error message with cross
    pub fn print_error(message: &str) {
        eprintln!("{} {}", Symbols::ERROR.red().bold(), message.red());
    }
    
    /// Print a warning message with warning symbol
    pub fn print_warning(message: &str) {
        println!("{} {}", Symbols::WARNING.yellow().bold(), message.yellow());
    }
    
    /// Print an info message with info symbol
    pub fn print_info(message: &str) {
        println!("{} {}", Symbols::INFO.blue().bold(), message);
    }
    
    /// Print a status message with bullet point
    pub fn print_status(message: &str) {
        println!("{} {}", Symbols::BULLET.cyan().bold(), message);
    }
    
    /// Print a step message with number and total
    pub fn print_step(step: usize, total: usize, message: &str) {
        println!(
            "{} Step {} of {}: {}", 
            Symbols::ARROW.blue().bold(),
            step.to_string().blue().bold(), 
            total.to_string().blue().bold(),
            message
        );
    }
    
    /// Print a command header with category and emoji
    pub fn print_command_header(title: &str, category: &str) {
        let (symbol, category_color) = match category {
            "init" => (Symbols::INIT, ThemeColors::INIT),
            "integrate" => (Symbols::INTEGRATE, ThemeColors::INTEGRATE),
            "status" => (Symbols::STATUS, ThemeColors::STATUS),
            "fix" => (Symbols::FIX, ThemeColors::FIX),
            "verify" => (Symbols::VERIFY, ThemeColors::VERIFY),
            "load" => (Symbols::LOAD, ThemeColors::LOAD),
            _ => (Symbols::INFO, ThemeColors::PRIMARY),
        };
        
        let category_name = match category {
            "init" => "Project Initialization",
            "integrate" => "Integration",
            "status" => "Status Check",
            "fix" => "Problem Fixing",
            "verify" => "Verification",
            "load" => "Memory Loading",
            _ => "CI Management",
        };
        
        // Create the header box
        let box_width = category_name.len() + 4;
        
        // Print top border
        println!("{}{}{}", 
            Self::apply_color(Symbols::TOP_LEFT, category_color),
            Self::apply_color(&Symbols::HORIZONTAL.repeat(box_width + 8), category_color),
            Self::apply_color(Symbols::TOP_RIGHT, category_color)
        );
        
        // Print title line
        println!("{}  {} {}  {}", 
            Self::apply_color(Symbols::VERTICAL, category_color),
            Self::apply_color(symbol, category_color).bold(),
            Self::apply_color(category_name, category_color).bold(),
            Self::apply_color(Symbols::VERTICAL, category_color)
        );
        
        // Print bottom border
        println!("{}{}{}", 
            Self::apply_color(Symbols::BOTTOM_LEFT, category_color),
            Self::apply_color(&Symbols::HORIZONTAL.repeat(box_width + 8), category_color),
            Self::apply_color(Symbols::BOTTOM_RIGHT, category_color)
        );
        
        // Print the section title
        println!("\n     {}     \n", title.bold());
    }
    
    /// Print a divider line
    pub fn print_divider(color: &str) {
        println!("{}", Self::apply_color(&Symbols::HORIZONTAL.repeat(50), color));
    }
    
    /// Show a spinner for the specified duration in milliseconds
    pub fn show_spinner(message: &str, duration_ms: u64) {
        let spinner_chars = ["/", "-", "\\", "|"];
        let start_time = std::time::Instant::now();
        let duration = std::time::Duration::from_millis(duration_ms);
        
        print!("{} {} ", spinner_chars[0].cyan(), message);
        io::stdout().flush().unwrap_or_default();
        
        let mut i = 0;
        while start_time.elapsed() < duration {
            std::thread::sleep(std::time::Duration::from_millis(100));
            print!("\r{} {} ", spinner_chars[i % spinner_chars.len()].cyan(), message);
            io::stdout().flush().unwrap_or_default();
            i += 1;
        }
        
        println!("\r{} {} ", Symbols::SUCCESS.green(), message);
    }
    
    /// Print help message for CI CLI
    pub fn print_help_message() {
        // Banner width
        let box_width = 70;
        
        // Create header box
        let top_border = format!(
            "{}{}{}",
            Symbols::TOP_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::TOP_RIGHT
        );
        
        let title_line = format!(
            "{}{}{}{}",
            Symbols::VERTICAL,
            " ".repeat(33),
            "CI CLI".bold(),
            " ".repeat(32) + Symbols::VERTICAL,
        );
        
        let subtitle_line = format!(
            "{}{}{}{}",
            Symbols::VERTICAL,
            " ".repeat(17),
            "Collaborative Intelligence CLI Tool".italic(),
            " ".repeat(17) + Symbols::VERTICAL,
        );
        
        let bottom_border = format!(
            "{}{}{}",
            Symbols::BOTTOM_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::BOTTOM_RIGHT
        );
        
        println!("{}", top_border.blue());
        println!("{}", title_line.blue());
        println!("{}", subtitle_line.blue());
        println!("{}", bottom_border.blue());
        
        println!("\nAVAILABLE COMMANDS");
        println!("{}", Symbols::HORIZONTAL.repeat(40).blue());
        println!();
        
        // Init Commands
        println!("{} {}", Symbols::INIT.cyan(), "Project Initialization:".cyan().bold());
        println!("  • {} {}", "ci init PROJECT_NAME".white().bold(), "- Initialize a new project with CI");
        println!();
        
        // Integrate Commands
        println!("{} {}", Symbols::INTEGRATE.blue(), "Integration Commands:".blue().bold());
        println!("  • {} {}", "ci integrate [PATH]".white().bold(), "- Integrate CI into an existing project");
        println!();
        
        // Status Commands
        println!("{} {}", Symbols::STATUS.green(), "Status Commands:".green().bold());
        println!("  • {} {}", "ci status [PATH]".white().bold(), "- Check CI integration status");
        println!();
        
        // Fix Commands
        println!("{} {}", Symbols::FIX.yellow(), "Fix Commands:".yellow().bold());
        println!("  • {} {}", "ci fix [PATH] [TYPE]".white().bold(), "- Fix CI integration issues");
        println!();
        
        // Verify Commands
        println!("{} {}", Symbols::VERIFY.magenta(), "Verify Commands:".magenta().bold());
        println!("  • {} {}", "ci verify [PATH]".white().bold(), "- Verify CI integration is working");
        println!();
        
        // Load Commands
        println!("{} {}", Symbols::LOAD.white(), "Memory Commands:".white().bold());
        println!("  • {} {}", "ci load AGENT [--context=CONTEXT]".white().bold(), "- Load agent memory");
        println!();
        
        // General Options
        println!("{} {}", Symbols::INFO.blue(), "General Options:".blue().bold());
        println!("  • {} {}", "ci --help".white().bold(), "- Show this help message");
        println!("  • {} {}", "ci --version".white().bold(), "- Show version information");
        println!("  • {} {}", "ci map".white().bold(), "- Display visual map of CLI commands");
        println!();
        
        // Footer
        let footer = format!(
            "{}{}{}",
            Symbols::BOTTOM_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::BOTTOM_RIGHT
        );
        println!("{}", footer.blue());
    }
    
    /// Prompt user for confirmation with styled prompt
    pub fn prompt_confirmation(message: &str) -> bool {
        print!(
            "{} {} (y/n): ", 
            Self::apply_color(Symbols::ARROW, ThemeColors::PRIMARY).bold(),
            message
        );
        io::stdout().flush().unwrap_or_default();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap_or_default();
        
        response.trim().to_lowercase() == "y" || response.trim().to_lowercase() == "yes"
    }
    
    /// Print a styled prompt for user input
    pub fn prompt_input(message: &str) -> String {
        print!(
            "{} {}: ", 
            Self::apply_color(Symbols::ARROW, ThemeColors::PRIMARY).bold(),
            message
        );
        io::stdout().flush().unwrap_or_default();
        
        let mut response = String::new();
        io::stdin().read_line(&mut response).unwrap_or_default();
        
        response.trim().to_string()
    }
}