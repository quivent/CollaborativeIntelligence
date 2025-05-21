//! Terminal utility functions for TL CLI
//!
//! This module provides utility functions for terminal output and formatting.
//! These functions are maintained for backwards compatibility but aren't
//! currently used in the codebase. New code should use helpers::terminal::TerminalHelpers.

use colored::*;

// Allow dead code warnings for backward compatibility functions
#[allow(dead_code)]

/// Legacy compatibility wrapper around TerminalHelpers::set_terminal_title
pub fn update_terminal_title(title: &str) {
    crate::helpers::terminal::TerminalHelpers::set_terminal_title(title);
}

#[allow(dead_code)]
/// Legacy compatibility wrapper around TerminalHelpers::print_section
pub fn print_section(title: &str) {
    println!("\n{}", format!("=== {} ===", title).green().bold());
}

#[allow(dead_code)]
/// Legacy compatibility wrapper around TerminalHelpers::print_success
pub fn print_success(message: &str) {
    println!("{} {}", "SUCCESS:".green().bold(), message);
}

#[allow(dead_code)]
/// Legacy compatibility wrapper around TerminalHelpers::print_error
pub fn print_error(message: &str) {
    eprintln!("{} {}", "ERROR:".red().bold(), message);
}

#[allow(dead_code)]
/// Legacy compatibility wrapper around TerminalHelpers::print_warning
pub fn print_warning(message: &str) {
    println!("{} {}", "WARNING:".yellow().bold(), message);
}

#[allow(dead_code)]
/// Legacy compatibility wrapper around TerminalHelpers::print_info
pub fn print_info(message: &str) {
    println!("{} {}", "INFO:".blue().bold(), message);
}