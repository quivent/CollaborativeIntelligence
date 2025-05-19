use crate::error::{AgentCacheError, Result};

/// Update the terminal window title
///
/// # Arguments
///
/// * `agent_name` - Name of the agent to display in the title
///
/// # Returns
///
/// Result indicating success or failure
pub fn update_terminal_title(agent_name: &str) -> Result<()> {
    // For macOS Terminal and iTerm2
    print!("\x1B]0;Agent: {}\x07", agent_name);
    std::io::Write::flush(&mut std::io::stdout())
        .map_err(|e| AgentCacheError::IoError(e))?;
    
    // For Linux terminals
    print!("\x1B]2;Agent: {}\x07", agent_name);
    std::io::Write::flush(&mut std::io::stdout())
        .map_err(|e| AgentCacheError::IoError(e))?;
    
    // For Windows Command Prompt/PowerShell
    #[cfg(target_os = "windows")]
    {
        use std::process::Command;
        let title_cmd = format!("title Agent: {}", agent_name);
        Command::new("cmd")
            .args(&["/C", &title_cmd])
            .output()
            .map_err(|e| AgentCacheError::IoError(e))?;
    }
    
    Ok(())
}

/// Reset the terminal window title
///
/// # Returns
///
/// Result indicating success or failure
pub fn reset_terminal_title() -> Result<()> {
    update_terminal_title("Collaborative Intelligence")
}