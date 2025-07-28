// Security Enforcement Module for Agent Cache System
// Implements AUTH_BYPASS_001 vulnerability fix

use colored::*;
use std::io::{self, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub type Result<T> = std::result::Result<T, SecurityError>;

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Authentication check failed: {0}")]
    AuthenticationError(String),
    
    #[error("Security enforcement blocked operation: {0}")]
    OperationBlocked(String),
    
    #[error("IO error during security check: {0}")]
    IoError(#[from] io::Error),
}

/// Security Enforcer for Agent Cache System
/// Implements authentication checks before agent activation
pub struct SecurityEnforcer {
    /// Timestamp of enforcer creation
    created_at: SystemTime,
    /// Whether to run in CI/automated mode (bypasses interactive prompts)
    ci_mode: bool,
}

impl SecurityEnforcer {
    /// Create a new security enforcer
    pub fn new() -> Self {
        let ci_mode = std::env::var("CI_MODE").is_ok() || 
                     std::env::var("CI").is_ok() || 
                     std::env::var("GITHUB_ACTIONS").is_ok() ||
                     std::env::var("AUTOMATED_AGENT_LOADING").is_ok();
        
        SecurityEnforcer {
            created_at: SystemTime::now(),
            ci_mode,
        }
    }
    
    /// Create a new security enforcer with explicit CI mode setting
    pub fn new_with_ci_mode(ci_mode: bool) -> Self {
        SecurityEnforcer {
            created_at: SystemTime::now(),
            ci_mode,
        }
    }
    
    /// Pre-operation security check for agent activation
    /// Returns true if user accepts risk, false if operation should be blocked
    pub fn pre_operation_check(&mut self, agent_name: &str) -> Result<bool> {
        if self.ci_mode {
            // In CI mode, automatically accept risk with logging
            println!("{} CI/Automated mode: Auto-accepting security risk for {}", "🤖".cyan(), agent_name);
            
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
                
            println!("{} Audit: {} - Auto-accepted in CI mode at {}", "📝".blue(), agent_name, timestamp);
            
            return Ok(true);
        }
        
        // Interactive mode: Display critical security warning
        self.display_security_warning(agent_name)?;
        
        // Prompt user for risk acceptance
        self.prompt_risk_acceptance(agent_name)
    }
    
    /// Display the critical security warning
    fn display_security_warning(&self, agent_name: &str) -> Result<()> {
        println!("{}", "🔴 CRITICAL SECURITY WARNING".red().bold());
        println!("{}", "Incident: AUTH_BYPASS_001".yellow());
        println!("{}", "Status: Agent authentication bypass vulnerability".yellow());
        println!();
        
        println!("{} attempting to operate:", agent_name.bright_white().bold());
        println!("{} No cryptographic signature verification available", "❌".red());
        println!("{} Cannot guarantee agent authenticity", "❌".red());
        println!("{} Operating in compromised security state", "❌".red());
        println!();
        
        Ok(())
    }
    
    /// Prompt user for explicit risk acceptance
    fn prompt_risk_acceptance(&self, agent_name: &str) -> Result<bool> {
        println!("{}", "🛡️ SECURITY CHALLENGE:".yellow().bold());
        println!("Do you accept the security risk of proceeding with unverified agent {}?", agent_name.bright_white());
        println!("Enter '{}' to continue or any other input to abort:", "ACCEPT_RISK".green().bold());
        
        print!("{}", "🛡️ ".yellow());
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        if input == "ACCEPT_RISK" {
            // Log acceptance
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            println!("{} User accepted security risk for {}", "⚠️".yellow(), agent_name);
            println!("{} Audit: {} - User accepted risk at {}", "📝".blue(), agent_name, timestamp);
            
            Ok(true)
        } else {
            // Log denial
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0);
            
            println!("{} Operation blocked - User denied security risk", "🛡️".red());
            println!("{} Audit: {} - Operation blocked at {}", "📝".blue(), agent_name, timestamp);
            
            Ok(false)
        }
    }
    
    /// Generate security incident report
    pub fn generate_security_report(&self) -> String {
        let mode_status = if self.ci_mode {
            "🤖 Mode: CI/Automated (Auto-accepting risks)"
        } else {
            "👤 Mode: Interactive (User prompts)"
        };
        
        format!(
            "🔐 AGENT CACHE SECURITY REPORT\n\n\
            📊 Status: AUTH_BYPASS_001 Emergency Mode Active\n\
            🛡️ Enforcement: Active pre-operation checks\n\
            {}\n\
            ⚠️ Risk Level: CRITICAL - No cryptographic verification\n\
            📝 Audit: All operations logged with user decisions\n\n\
            🚨 IMMEDIATE ACTION REQUIRED:\n\
            • Implement cryptographic agent signature system\n\
            • Deploy authentication infrastructure\n\
            • Verify all agent operations manually\n\
            • Monitor security logs for unauthorized access attempts\n\n\
            🤖 CI Mode Configuration:\n\
            • Set CI_MODE=1 or AUTOMATED_AGENT_LOADING=1 for automated environments\n\
            • CI mode auto-accepts security risks with audit logging\n",
            mode_status
        )
    }
}

impl Default for SecurityEnforcer {
    fn default() -> Self {
        Self::new()
    }
}