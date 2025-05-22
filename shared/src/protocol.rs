/// Shared protocol instructions module
/// Ensures consistency across all agent loading methods

/// Display standardized agent protocol instructions
/// This function provides the canonical protocol instructions that all
/// agent loading methods should display to ensure consistency
pub fn display_agent_protocol_instructions(agent_name: &str) {
    println!();
    println!("ℹ 📋 Agent Protocol Instructions:");
    
    println!("  • All responses must follow the standardized format:");
    println!("    [{}]: Your response content...", agent_name.to_uppercase());
    println!();
    println!("    -- [{}]", agent_name.to_uppercase());
    println!();
    
    println!("  • This format provides:");
    println!("    - Clear agent attribution");
    println!("    - Consistent user experience");
    println!("    - Improved conversation tracking");
    println!("    - Visual response boundaries");
    println!();
    
    println!("ℹ See docs/protocols/response-format-protocol.md for complete details");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_name_formatting() {
        // This would test the function if we could capture stdout
        // For now, just ensure it doesn't panic
        display_agent_protocol_instructions("scholar");
        display_agent_protocol_instructions("REFACTORER");
        display_agent_protocol_instructions("test_agent");
    }
}