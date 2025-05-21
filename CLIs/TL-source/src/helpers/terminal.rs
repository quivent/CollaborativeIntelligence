//! Terminal output helpers for TL CLI
//!
//! This module provides helpers for terminal output formatting and styling.
//! Implements the visual design system for the TL CLI.

use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

/// Color theme definition for TL CLI
pub struct ThemeColors;

impl ThemeColors {
    // Primary colors
    pub const PRIMARY: &'static str = "blue";        // Main brand color
    pub const SECONDARY: &'static str = "cyan";      // Secondary brand color
    pub const ACCENT: &'static str = "magenta";      // Accent color for highlights
    
    // Status colors
    pub const SUCCESS: &'static str = "green";       // Success messages (#00C853)
    pub const WARNING: &'static str = "yellow";      // Warning messages (#FFD600)
    pub const ERROR: &'static str = "red";           // Error messages (#FF3D00)
    pub const INFO: &'static str = "blue";           // Info messages (#2979FF)
    
    // Command category colors
    pub const AGENT: &'static str = "cyan";          // Agent management commands
    pub const SYSTEM: &'static str = "blue";         // System commands
    pub const MEMORY: &'static str = "magenta";      // Memory operations
    pub const KNOWLEDGE: &'static str = "green";     // Knowledge management
    pub const DEBUG: &'static str = "yellow";        // Debug and diagnostic commands
}

/// Symbol definitions for TL CLI
pub struct Symbols;

#[allow(dead_code)]
impl Symbols {
    // Status symbols
    pub const SUCCESS: &'static str = "✓";
    pub const ERROR: &'static str = "✗";
    pub const WARNING: &'static str = "⚠";
    pub const INFO: &'static str = "ℹ";
    
    // Command category symbols
    pub const AGENT: &'static str = "👤";
    pub const SYSTEM: &'static str = "🔧";
    pub const MEMORY: &'static str = "🧠";
    pub const KNOWLEDGE: &'static str = "📚";
    pub const DEBUG: &'static str = "🔍";
    
    // UI elements
    pub const BULLET: &'static str = "•";
    pub const ARROW: &'static str = "→";
    pub const CHECKMARK: &'static str = "✓";
    pub const CROSS: &'static str = "✗";
    pub const STAR: &'static str = "★";
    pub const DOT: &'static str = "·";
    
    // Box drawing characters
    pub const TOP_LEFT: &'static str = "┌";
    pub const TOP_RIGHT: &'static str = "┐";
    pub const BOTTOM_LEFT: &'static str = "└";
    pub const BOTTOM_RIGHT: &'static str = "┘";
    pub const HORIZONTAL: &'static str = "─";
    pub const VERTICAL: &'static str = "│";
    pub const T_DOWN: &'static str = "┬";
    pub const T_UP: &'static str = "┴";
    pub const T_RIGHT: &'static str = "├";
    pub const T_LEFT: &'static str = "┤";
    pub const CROSS_SYMBOL: &'static str = "┼";
    
    // Progress indicators
    pub const LOADING_FRAMES: [&'static str; 8] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"];
}

/// Helper functions for terminal output formatting
pub struct TerminalHelpers;

#[allow(dead_code)]
impl TerminalHelpers {
    /// Set the terminal window title
    pub fn set_terminal_title(title: &str) {
        print!("\x1B]0;TL: {}\x07", title);
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
        println!("{} {}", Symbols::INFO.blue().bold(), message.blue());
    }
    
    /// Print a status message with bullet point
    pub fn print_status(message: &str) {
        println!("  {} {}", Symbols::BULLET.cyan(), message);
    }
    
    /// Print a standard command header with category and emoji
    pub fn print_command_header(title: &str, category_type: &str) {
        // Determine category colors and symbols
        let (emoji, category, color) = match category_type.to_lowercase().as_str() {
            "agent" => (Symbols::AGENT, "Agent Management", ThemeColors::AGENT),
            "system" => (Symbols::SYSTEM, "System Operations", ThemeColors::SYSTEM),
            "memory" => (Symbols::MEMORY, "Memory Management", ThemeColors::MEMORY),
            "knowledge" => (Symbols::KNOWLEDGE, "Knowledge Base", ThemeColors::KNOWLEDGE),
            "debug" => (Symbols::DEBUG, "Debug & Diagnostics", ThemeColors::DEBUG),
            _ => (Symbols::INFO, category_type, ThemeColors::PRIMARY),
        };
        
        Self::print_fancy_header(title, emoji, category, color);
    }
    
    /// Print a fancy styled header box
    pub fn print_fancy_header(title: &str, emoji: &str, category: &str, color: &str) {
        // Calculate width based on the longest string in header or title
        let content_width = std::cmp::max(
            category.len() + emoji.len() + 4, // +4 for spacing
            title.len()
        );
        let box_width = content_width + 8; // Add padding
        
        // Create box elements with the right width
        let top_border = format!(
            "{}{}{}", 
            Symbols::TOP_LEFT, 
            Symbols::HORIZONTAL.repeat(box_width), 
            Symbols::TOP_RIGHT
        );
        
        let header = format!(
            "{}  {}  {}{}",
            Symbols::VERTICAL,
            emoji,
            category,
            " ".repeat(box_width - category.len() - emoji.len() - 2)
        );
        
        let bottom_border = format!(
            "{}{}{}",
            Symbols::BOTTOM_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::BOTTOM_RIGHT
        );
        
        // Print the header with appropriate color
        println!("{}", Self::apply_color(&top_border, color));
        println!("{}", Self::apply_color(&header, color).bold());
        println!("{}", Self::apply_color(&bottom_border, color));
        println!();
        
        // Print the title with the same color and center it
        let padding = (box_width - title.len()) / 2;
        let title_line = format!("{}{}{}", " ".repeat(padding), title, " ".repeat(padding));
        println!("{}", Self::apply_color(&title_line, color).bold());
        println!();
    }
    
    /// Print a divider line with box drawing characters
    pub fn print_divider(color: &str) {
        let line = Symbols::HORIZONTAL.repeat(70);
        println!("{}", Self::apply_color(&line, color));
        println!();
    }
    
    /// Print a step message with number and total
    pub fn print_step(step: usize, total: usize, message: &str) {
        let step_indicator = format!("[{}/{}]", step, total);
        println!(
            "{} {} {}", 
            Self::apply_color(&step_indicator, ThemeColors::INFO),
            Self::apply_color(Symbols::ARROW, ThemeColors::ACCENT),
            message
        );
        println!("{}", Self::apply_color(&Symbols::HORIZONTAL.repeat(60), ThemeColors::INFO));
    }
    
    /// Display a spinner animation with a message for long-running operations
    pub fn show_spinner(message: &str, duration_ms: u64) {
        print!("\r"); // Move to the beginning of the line
        
        for i in 0..((duration_ms / 100) as usize) {
            let frame = Symbols::LOADING_FRAMES[i % Symbols::LOADING_FRAMES.len()];
            print!("\r{} {} ", frame.cyan(), message);
            io::stdout().flush().unwrap_or_default();
            thread::sleep(Duration::from_millis(100));
        }
        
        // Clear the spinner when done
        print!("\r{}\r", " ".repeat(message.len() + 4));
        io::stdout().flush().unwrap_or_default();
    }
    
    /// Print a table with headers and rows
    pub fn print_table(headers: &[&str], rows: &[Vec<String>]) {
        if headers.is_empty() || rows.is_empty() {
            return;
        }
        
        // Calculate column widths based on content
        let mut col_widths = vec![0; headers.len()];
        
        // Check header lengths
        for (i, &header) in headers.iter().enumerate() {
            if i < col_widths.len() {
                col_widths[i] = header.len();
            }
        }
        
        // Check row content lengths
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    col_widths[i] = std::cmp::max(col_widths[i], cell.len());
                }
            }
        }
        
        // Add padding to column widths
        for width in &mut col_widths {
            *width += 2; // Add 2 spaces of padding
        }
        
        // Print table top border
        Self::print_table_border(&col_widths, true, ThemeColors::INFO);
        
        // Print headers
        print!("{} ", Self::apply_color(Symbols::VERTICAL, ThemeColors::INFO));
        for (i, &header) in headers.iter().enumerate() {
            if i < col_widths.len() {
                let padded = format!(" {}{} ", header, " ".repeat(col_widths[i] - header.len() - 2));
                print!("{}", Self::apply_color(&padded, ThemeColors::INFO).bold());
                print!("{} ", Self::apply_color(Symbols::VERTICAL, ThemeColors::INFO));
            }
        }
        println!();
        
        // Print header separator
        Self::print_table_separator(&col_widths, ThemeColors::INFO);
        
        // Print rows
        for row in rows {
            print!("{} ", Self::apply_color(Symbols::VERTICAL, ThemeColors::INFO));
            for (i, cell) in row.iter().enumerate() {
                if i < col_widths.len() {
                    let padded = format!(" {}{} ", cell, " ".repeat(col_widths[i] - cell.len() - 2));
                    print!("{}", padded);
                    print!("{} ", Self::apply_color(Symbols::VERTICAL, ThemeColors::INFO));
                }
            }
            println!();
        }
        
        // Print table bottom border
        Self::print_table_border(&col_widths, false, ThemeColors::INFO);
        println!();
    }
    
    /// Print table top or bottom border
    fn print_table_border(col_widths: &[usize], is_top: bool, color: &str) {
        let (left, cross, right) = if is_top {
            (Symbols::TOP_LEFT, Symbols::T_DOWN, Symbols::TOP_RIGHT)
        } else {
            (Symbols::BOTTOM_LEFT, Symbols::T_UP, Symbols::BOTTOM_RIGHT)
        };
        
        print!("{}", Self::apply_color(left, color));
        for (i, &width) in col_widths.iter().enumerate() {
            print!("{}", Self::apply_color(&Symbols::HORIZONTAL.repeat(width), color));
            if i < col_widths.len() - 1 {
                print!("{}", Self::apply_color(cross, color));
            }
        }
        println!("{}", Self::apply_color(right, color));
    }
    
    /// Print table column separator
    fn print_table_separator(col_widths: &[usize], color: &str) {
        print!("{}", Self::apply_color(Symbols::T_RIGHT, color));
        for (i, &width) in col_widths.iter().enumerate() {
            print!("{}", Self::apply_color(&Symbols::HORIZONTAL.repeat(width), color));
            if i < col_widths.len() - 1 {
                print!("{}", Self::apply_color(Symbols::CROSS_SYMBOL, color));
            }
        }
        println!("{}", Self::apply_color(Symbols::T_LEFT, color));
    }
    
    /// Print a styled list with bullets
    pub fn print_list(items: &[String], color: &str) {
        for item in items {
            println!(
                "  {} {}", 
                Self::apply_color(Symbols::BULLET, color).bold(),
                item
            );
        }
        println!();
    }
    
    /// Print a result block with title
    pub fn print_result_block(title: &str, content: &str, color: &str) {
        // Calculate width based on the longest line in title or content
        let content_lines: Vec<&str> = content.lines().collect();
        let mut max_width = title.len();
        
        for line in &content_lines {
            max_width = std::cmp::max(max_width, line.len());
        }
        
        let box_width = max_width + 4; // Add padding
        
        // Create box elements
        let top_border = format!(
            "{}{}{}", 
            Symbols::TOP_LEFT, 
            Symbols::HORIZONTAL.repeat(box_width), 
            Symbols::TOP_RIGHT
        );
        
        let title_line = format!(
            "{}  {}{}  {}",
            Symbols::VERTICAL,
            title,
            " ".repeat(box_width - title.len() - 4),
            Symbols::VERTICAL
        );
        
        let separator = format!(
            "{}{}{}",
            Symbols::T_RIGHT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::T_LEFT
        );
        
        let bottom_border = format!(
            "{}{}{}",
            Symbols::BOTTOM_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::BOTTOM_RIGHT
        );
        
        // Print the block with appropriate color
        println!("{}", Self::apply_color(&top_border, color));
        println!("{}", Self::apply_color(&title_line, color).bold());
        println!("{}", Self::apply_color(&separator, color));
        
        // Print content lines
        for line in content_lines {
            let padding = box_width - line.len();
            let formatted_line = format!(
                "{}  {}{}  {}", 
                Symbols::VERTICAL, 
                line, 
                " ".repeat(padding - 4),
                Symbols::VERTICAL
            );
            println!("{}", Self::apply_color(&formatted_line, color));
        }
        
        println!("{}", Self::apply_color(&bottom_border, color));
        println!();
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
    
    /// Create a selection menu and return selected index
    pub fn selection_menu(title: &str, options: &[String]) -> usize {
        println!(
            "{} {}", 
            Self::apply_color(Symbols::ARROW, ThemeColors::PRIMARY).bold(),
            title
        );
        println!();
        
        // Print options
        for (i, option) in options.iter().enumerate() {
            println!(
                "  {}. {}", 
                Self::apply_color(&(i + 1).to_string(), ThemeColors::PRIMARY).bold(),
                option
            );
        }
        
        println!();
        let selection = Self::prompt_input("Enter selection number");
        
        // Parse selection
        match selection.parse::<usize>() {
            Ok(n) if n > 0 && n <= options.len() => n - 1,
            _ => {
                Self::print_error("Invalid selection, using default (1)");
                0
            }
        }
    }
    
    /// Print a styled help message for the main TL CLI
    pub fn print_help_message() {
        // Banner and title
        let title = "TL CLI";
        let subtitle = "Teaching/Learning CLI for CollaborativeIntelligence";
        let box_width = 70;
        
        // Create title box
        let top_border = format!(
            "{}{}{}",
            Symbols::TOP_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::TOP_RIGHT
        );
        
        let bottom_border = format!(
            "{}{}{}",
            Symbols::BOTTOM_LEFT,
            Symbols::HORIZONTAL.repeat(box_width),
            Symbols::BOTTOM_RIGHT
        );
        
        // Print title box
        println!("{}", top_border.blue());
        
        // Center the title
        let title_padding = (box_width - title.len()) / 2;
        let title_line = format!("{}{}{}",
            Symbols::VERTICAL,
            " ".repeat(title_padding),
            title
        );
        let title_spaces = box_width - title.len() - title_padding;
        println!("{}{}{}", 
            title_line.blue().bold(), 
            " ".repeat(title_spaces), 
            Symbols::VERTICAL.blue()
        );
        
        // Center the subtitle
        let subtitle_padding = (box_width - subtitle.len()) / 2;
        let subtitle_line = format!("{}{}{}",
            Symbols::VERTICAL,
            " ".repeat(subtitle_padding),
            subtitle
        );
        let subtitle_spaces = box_width - subtitle.len() - subtitle_padding;
        println!("{}{}{}", 
            subtitle_line.blue(), 
            " ".repeat(subtitle_spaces), 
            Symbols::VERTICAL.blue()
        );
        
        println!("{}", bottom_border.blue());
        println!();
        
        // Available Commands
        println!("{}", "AVAILABLE COMMANDS".blue().bold());
        println!("{}", Symbols::HORIZONTAL.repeat(40).blue());
        println!();
        
        // Agent Commands
        println!("{} {}", Symbols::AGENT.cyan(), "Agent Management:".cyan().bold());
        println!("  • {} {}", "TL agent list".white().bold(), "- List all available agents");
        println!("  • {} {}", "TL agent activate <name>".white().bold(), "- Activate an agent");
        println!("  • {} {}", "TL agent create <name> [category]".white().bold(), "- Create a new agent");
        println!();
        
        // Memory Commands
        println!("{} {}", Symbols::MEMORY.magenta(), "Memory Operations:".magenta().bold());
        println!("  • {} {}", "TL memory load <agent>".white().bold(), "- Load agent memory");
        println!("  • {} {}", "TL memory optimize <agent>".white().bold(), "- Optimize agent memory");
        println!("  • {} {}", "TL memory cache <agent>".white().bold(), "- Cache agent memory");
        println!();
        
        // Knowledge Commands
        println!("{} {}", Symbols::KNOWLEDGE.green(), "Knowledge Management:".green().bold());
        println!("  • {} {}", "TL knowledge search <query> [category]".white().bold(), "- Search knowledge registry");
        println!("  • {} {}", "TL knowledge index".white().bold(), "- Update knowledge registry indexes");
        println!("  • {} {}", "TL knowledge distribute [agent]".white().bold(), "- Distribute knowledge");
        println!();
        
        // Debug Commands
        println!("{} {}", Symbols::DEBUG.yellow(), "Debug Utilities:".yellow().bold());
        println!("  • {} {}", "TL debug paths".white().bold(), "- Print repository path information");
        println!("  • {} {}", "TL debug config".white().bold(), "- Check system configuration");
        println!("  • {} {}", "TL debug format".white().bold(), "- Test terminal formatting");
        println!("  • {} {}", "TL debug map".white().bold(), "- Display visual map of CLI commands");
        println!();
        
        // System Commands
        println!("{} {}", Symbols::SYSTEM.blue(), "System Maintenance:".blue().bold());
        println!("  • {} {}", "TL system clean [--cache]".white().bold(), "- Clean temporary files");
        println!("  • {} {}", "TL system verify".white().bold(), "- Verify repository structure");
        println!("  • {} {}", "TL system title [title]".white().bold(), "- Update terminal title");
        println!();
        
        // Help and Version
        println!("{} {}", Symbols::INFO.blue(), "General Options:".blue().bold());
        println!("  • {} {}", "TL --help".white().bold(), "- Show this help message");
        println!("  • {} {}", "TL --version".white().bold(), "- Show version information");
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
}