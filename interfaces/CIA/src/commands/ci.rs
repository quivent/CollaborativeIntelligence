use crate::config::Config;
use crate::ci_integration::{CIIntegrationDetector, IntegrationStatus, HealthLevel, CheckStatus};
use anyhow::Result;
use colored::*;
use std::process::Command;
use std::path::PathBuf;

pub fn agents(config: &Config) -> Result<()> {
    println!("{} Checking CI agents status...", "🤖".bright_blue());
    
    let detector = CIIntegrationDetector::new()?;
    let status = detector.get_integration_status();
    
    // Display integration status
    print_integration_status(&status);
    
    if !status.ci_integrated {
        println!("\n{} No CI integration detected. To enable CI integration:", "ℹ️".blue());
        println!("  1. Ensure you're in a CollaborativeIntelligence integrated project");
        println!("  2. Check for CLAUDE.md with CI integration markers");
        println!("  3. Or create .collaborative-intelligence.json config file");
        return Ok(());
    }
    
    // Get CI repository path
    if let Some(ci_path) = detector.get_ci_repository_path() {
        println!("\n{} CI Repository: {}", "📁".blue(), ci_path.display());
        
        // List available agents
        list_available_agents(&ci_path)?;
        
        // Check agent activation system
        check_agent_activation_system(&ci_path)?;
        
        // Verify title override system
        verify_title_override_system(&detector)?;
        
    } else {
        return Err(anyhow::anyhow!("CI integration detected but cannot determine repository path"));
    }
    
    Ok(())
}

pub fn status(config: &Config) -> Result<()> {
    println!("{} CI Integration Status", "📊".bright_blue().bold());
    
    let detector = CIIntegrationDetector::new()?;
    let status = detector.get_integration_status();
    
    print_integration_status(&status);
    
    if let Some(config) = &status.config {
        println!("\n{} Configuration Details:", "⚙️".bright_blue());
        println!("  Integration Type: {}", config.integration_type.cyan());
        println!("  Repository Path: {}", config.repository_path.cyan());
        println!("  Active Agents: {}", config.active_agents.join(", ").cyan());
        println!("  Fast Activation: {}", if config.fast_activation { "✅" } else { "❌" });
        println!("  Preload Context: {}", if config.preload_context { "✅" } else { "❌" });
    }
    
    Ok(())
}

pub fn connect(config: &Config) -> Result<()> {
    println!("{} Establishing CI connection...", "🔗".bright_blue());
    
    let detector = CIIntegrationDetector::new()?;
    
    if !detector.is_ci_integrated() {
        return Err(anyhow::anyhow!("No CI integration detected in current directory"));
    }
    
    // Test connection to CI repository
    if let Some(ci_path) = detector.get_ci_repository_path() {
        println!("{} CI Repository found: {}", "✅".green(), ci_path.display());
        
        // Verify repository structure
        let agents_dir = ci_path.join("AGENTS");
        if agents_dir.exists() {
            println!("{} AGENTS directory accessible", "✅".green());
        } else {
            return Err(anyhow::anyhow!("AGENTS directory not found at: {}", agents_dir.display()));
        }
        
        // Test title override system
        println!("{} Testing title override system...", "🧪".yellow());
        if let Err(e) = detector.execute_title_override(Some("CI Connection Test")) {
            println!("{} Title override test failed: {}", "⚠️".yellow(), e);
        } else {
            println!("{} Title override system operational", "✅".green());
        }
        
        println!("\n{} CI connection established successfully", "🎉".green().bold());
        
    } else {
        return Err(anyhow::anyhow!("Cannot determine CI repository path"));
    }
    
    Ok(())
}

pub fn activate(agent_name: &str, config: &Config) -> Result<()> {
    println!("{} Activating CI agent: {}", "🚀".bright_blue(), agent_name.bright_white().bold());
    
    // CRITICAL: Enforce authentication for CI agent activation (AUTH_BYPASS_001 fix)
    let mut security_enforcer = crate::security::SecurityEnforcer::new();
    
    println!("{}", "🔒 SECURITY ENFORCEMENT: Validating CI agent authentication...".yellow().bold());
    match security_enforcer.pre_operation_check(&format!("CI_{}", agent_name)) {
        Ok(true) => {
            println!("{}", "✅ Security check passed - User accepted risk for unverified CI agent".green());
        },
        Ok(false) => {
            println!("{}", "🛡️ CI agent activation blocked by security enforcement".red());
            return Err(anyhow::anyhow!("Security enforcement blocked CI agent activation: {}", agent_name));
        },
        Err(e) => {
            println!("{} Security check failed: {}", "❌".red(), e);
            return Err(anyhow::anyhow!("Security check failed for CI agent {}: {}", agent_name, e));
        }
    }
    
    let detector = CIIntegrationDetector::new()?;
    
    if !detector.is_ci_integrated() {
        return Err(anyhow::anyhow!("No CI integration detected. Cannot activate CI agents."));
    }
    
    let ci_path = detector.get_ci_repository_path()
        .ok_or_else(|| anyhow::anyhow!("Cannot determine CI repository path"))?;
    
    // Check if agent exists
    let agent_path = ci_path.join("AGENTS").join(agent_name);
    if !agent_path.exists() {
        return Err(anyhow::anyhow!("Agent '{}' not found in CI repository", agent_name));
    }
    
    println!("{} Agent found: {}", "✅".green(), agent_path.display());
    
    // Use agent-cache interface for activation
    let agent_cache_binary = ci_path.join("interfaces").join("agent-cache").join("target").join("release").join("agent-cache");
    
    if agent_cache_binary.exists() {
        println!("{} Using agent-cache interface for activation...", "🔄".yellow());
        
        let output = Command::new(&agent_cache_binary)
            .args(&["activate", agent_name])
            .current_dir(&ci_path)
            .output()?;
        
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
            println!("{} Agent '{}' activated successfully via agent-cache", "🎉".green().bold(), agent_name);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("{} Agent activation failed. Try refreshing the cache:", "💡".bright_yellow());
            println!("  {} Build agent-cache: cd interfaces/agent-cache && cargo build --release", "🔧".cyan());
            println!("  {} Clean cache: rm -rf CACHE/SESSIONS/*", "🧹".cyan());
            return Err(anyhow::anyhow!("Agent activation failed: {}", stderr));
        }
    } else {
        // Fallback to direct activation
        println!("{} agent-cache not found, using direct activation...", "⚠️".yellow());
        
        // Use claude-init or similar fallback mechanism
        let claude_init = ci_path.join("claude-init");
        if claude_init.exists() {
            let output = Command::new(&claude_init)
                .arg(agent_name)
                .current_dir(&ci_path)
                .output()?;
            
            if output.status.success() {
                println!("{} Agent '{}' activated successfully via claude-init", "🎉".green().bold(), agent_name);
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                return Err(anyhow::anyhow!("Agent activation failed: {}", stderr));
            }
        } else {
            return Err(anyhow::anyhow!("No agent activation system found. Please ensure agent-cache or claude-init is available."));
        }
    }
    
    Ok(())
}

fn print_integration_status(status: &IntegrationStatus) {
    println!("Status: {}", status.get_compliance_message());
    
    let integration_status = if status.ci_integrated { "✅".green() } else { "❌".red() };
    let override_status = if status.title_override_available { "✅".green() } else { "❌".red() };
    let compliance_status = if status.is_compliant() { "✅".green() } else { "⚠️".yellow() };
    
    println!("  CI Integrated: {}", integration_status);
    println!("  Title Override: {}", override_status);
    println!("  Compliance: {}", compliance_status);
    
    if status.enforcement_required {
        println!("\n{} {} Enforcement required: Install CIA CLI or ensure cia-title-override script is available", 
                "⚠️".yellow().bold(), "Warning:".yellow().bold());
    }
}

fn list_available_agents(ci_path: &PathBuf) -> Result<()> {
    println!("\n{} Available Agents:", "🤖".bright_blue().bold());
    
    let agents_dir = ci_path.join("AGENTS");
    if !agents_dir.exists() {
        return Err(anyhow::anyhow!("AGENTS directory not found"));
    }
    
    let mut agents = Vec::new();
    for entry in std::fs::read_dir(&agents_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let agent_name = entry.file_name().to_string_lossy().to_string();
            // Skip system files and metadata
            if !agent_name.starts_with('.') && !agent_name.ends_with(".md") {
                agents.push(agent_name);
            }
        }
    }
    
    agents.sort();
    
    for (i, agent) in agents.iter().enumerate() {
        let agent_path = agents_dir.join(agent);
        let readme_path = agent_path.join("README.md");
        
        if readme_path.exists() {
            println!("  {}. {} {}", (i + 1).to_string().bright_blue(), "🤖".blue(), agent.bright_white());
        } else {
            println!("  {}. {} {} {}", (i + 1).to_string().bright_blue(), "⚠️".yellow(), agent.bright_white(), "(incomplete)".yellow());
        }
    }
    
    if agents.is_empty() {
        println!("  No agents found in AGENTS directory");
    } else {
        println!("\n{} Use 'cia ci activate <agent-name>' to activate an agent", "💡".bright_yellow());
    }
    
    Ok(())
}

fn check_agent_activation_system(ci_path: &PathBuf) -> Result<()> {
    println!("\n{} Agent Activation System:", "⚙️".bright_blue().bold());
    
    // Check for agent-cache binary
    let agent_cache_binary = ci_path.join("interfaces").join("agent-cache").join("target").join("release").join("agent-cache");
    if agent_cache_binary.exists() {
        println!("  {} agent-cache binary available", "✅".green());
    } else {
        println!("  {} agent-cache binary not found", "⚠️".yellow());
    }
    
    // Check for claude-init fallback
    let claude_init = ci_path.join("claude-init");
    if claude_init.exists() {
        println!("  {} claude-init fallback available", "✅".green());
    } else {
        println!("  {} claude-init fallback not found", "⚠️".yellow());
    }
    
    Ok(())
}

fn verify_title_override_system(detector: &CIIntegrationDetector) -> Result<()> {
    println!("\n{} Title Override System:", "🏷️".bright_blue().bold());
    
    if detector.is_title_override_available() {
        println!("  {} Title override system available", "✅".green());
        
        // Test the system
        if let Err(e) = detector.execute_title_override(Some("CI System Check")) {
            println!("  {} Test execution failed: {}", "⚠️".yellow(), e);
        } else {
            println!("  {} Test execution successful", "✅".green());
        }
    } else {
        println!("  {} Title override system not available", "❌".red());
        println!("    Install CIA CLI or ensure cia-title-override script exists");
    }
    
    Ok(())
}

pub fn health(_config: &Config) -> Result<()> {
    println!("{} Performing CI Integration Health Check...", "🏥".bright_blue().bold());
    
    let detector = CIIntegrationDetector::new()?;
    let health_status = detector.perform_health_check();
    
    // Display overall health
    println!("\n{} Overall Health: {} {}", 
        "📊".bright_blue(), 
        health_status.overall_health.emoji(), 
        health_status.overall_health.label().bright_white().bold()
    );
    
    println!("{} {}", "📈".blue(), health_status.get_score_display().cyan());
    
    // Display issues
    if !health_status.issues.is_empty() {
        println!("\n{} Critical Issues:", "🔴".red().bold());
        for issue in &health_status.issues {
            println!("  {} {}", "❌".red(), issue);
        }
    }
    
    // Display warnings
    if !health_status.warnings.is_empty() {
        println!("\n{} Warnings:", "🟡".yellow().bold());
        for warning in &health_status.warnings {
            println!("  {} {}", "⚠️".yellow(), warning);
        }
    }
    
    // Display recommendations
    if !health_status.recommendations.is_empty() {
        println!("\n{} Recommendations:", "💡".bright_yellow().bold());
        for recommendation in &health_status.recommendations {
            println!("  {} {}", "💡".yellow(), recommendation);
        }
    }
    
    // Suggest repair if needed
    if health_status.has_issues() {
        println!("\n{} {} Run 'cia ci repair' to automatically fix detected issues", 
            "🔧".bright_blue().bold(), "Tip:".bright_blue().bold());
    }
    
    Ok(())
}

pub fn validate(_config: &Config) -> Result<()> {
    println!("{} Validating CI Integration...", "🔍".bright_blue().bold());
    
    let detector = CIIntegrationDetector::new()?;
    let validation_report = detector.validate_integration();
    
    // Display overall validation status
    let status_emoji = match validation_report.overall_status {
        crate::ci_integration::ValidationStatus::Valid => "✅",
        crate::ci_integration::ValidationStatus::PartiallyValid => "⚠️",
        crate::ci_integration::ValidationStatus::Invalid => "❌",
    };
    
    println!("\n{} Validation Status: {} {:?}", 
        "📋".bright_blue(), 
        status_emoji, 
        validation_report.overall_status
    );
    
    // Display individual checks
    println!("\n{} Validation Checks:", "🔍".bright_blue().bold());
    for check in &validation_report.checks {
        println!("  {} {} - {}", 
            check.status.emoji(), 
            check.name.bright_white(), 
            check.details.cyan()
        );
    }
    
    // Display health summary
    let health = &validation_report.health_status;
    println!("\n{} Health Summary:", "🏥".bright_blue().bold());
    println!("  {} Overall: {} {}", 
        health.overall_health.emoji(), 
        health.overall_health.label(), 
        health.get_score_display().cyan()
    );
    
    if !validation_report.recommendations.is_empty() {
        println!("\n{} Recommendations:", "💡".bright_yellow().bold());
        for recommendation in &validation_report.recommendations {
            println!("  {} {}", "💡".yellow(), recommendation);
        }
    }
    
    Ok(())
}

pub fn repair(dry_run: bool, _config: &Config) -> Result<()> {
    println!("{} CI Integration Auto-Repair{}", 
        "🔧".bright_blue().bold(),
        if dry_run { " (Dry Run)" } else { "" }
    );
    
    let detector = CIIntegrationDetector::new()?;
    
    if !detector.is_ci_integrated() {
        return Err(anyhow::anyhow!("Cannot perform auto-repair: No CI integration detected"));
    }
    
    // Show current health status
    let health_status = detector.perform_health_check();
    println!("\n{} Current Health: {} {}", 
        "📊".blue(), 
        health_status.overall_health.emoji(), 
        health_status.overall_health.label()
    );
    
    if !health_status.has_issues() {
        println!("\n{} No issues detected - CI integration is healthy!", "✅".green().bold());
        return Ok(());
    }
    
    if dry_run {
        println!("\n{} Dry Run - Analyzing potential repairs:", "🔍".yellow().bold());
        
        // Simulate repair actions
        match detector.auto_repair() {
            Ok(actions) => {
                if actions.is_empty() {
                    println!("  {} No automatic repairs available", "ℹ️".blue());
                } else {
                    println!("  {} Potential repairs:", "🔧".blue());
                    for action in actions {
                        println!("    {} {}", "🔧".cyan(), action);
                    }
                }
            },
            Err(e) => {
                println!("  {} Auto-repair analysis failed: {}", "❌".red(), e);
            }
        }
    } else {
        println!("\n{} Attempting automatic repairs...", "🔧".bright_blue().bold());
        
        match detector.auto_repair() {
            Ok(actions) => {
                if actions.is_empty() {
                    println!("  {} No automatic repairs were needed", "✅".green());
                } else {
                    println!("  {} Applied repairs:", "✅".green().bold());
                    for action in actions {
                        println!("    {} {}", "✅".green(), action);
                    }
                    
                    // Re-check health after repairs
                    println!("\n{} Verifying repairs...", "🔍".blue());
                    let new_health = detector.perform_health_check();
                    println!("  {} New Health Status: {} {}", 
                        "📊".blue(), 
                        new_health.overall_health.emoji(), 
                        new_health.overall_health.label()
                    );
                    
                    if new_health.overall_health == HealthLevel::Good || new_health.overall_health == HealthLevel::Excellent {
                        println!("\n{} CI integration successfully repaired!", "🎉".green().bold());
                    } else if !new_health.has_issues() {
                        println!("\n{} CI integration partially repaired - manual intervention may be needed", "⚠️".yellow());
                    } else {
                        println!("\n{} Some issues remain - manual intervention required", "⚠️".yellow());
                    }
                }
            },
            Err(e) => {
                println!("  {} Auto-repair failed: {}", "❌".red(), e);
                println!("  {} Manual intervention required", "🔧".yellow());
            }
        }
    }
    
    Ok(())
}

pub fn monitor(interval: u64, _config: &Config) -> Result<()> {
    println!("{} Starting CI Integration Monitor (interval: {}s)", 
        "👁️".bright_blue().bold(), interval);
    
    let detector = CIIntegrationDetector::new()?;
    let mut last_health: Option<HealthLevel> = None;
    
    println!("{} Press Ctrl+C to stop monitoring\n", "ℹ️".blue());
    
    loop {
        let health_status = detector.perform_health_check();
        let timestamp = chrono::Utc::now().format("%H:%M:%S");
        
        // Only display if health status changed or there are new issues
        let should_display = match &last_health {
            None => true,
            Some(prev) => prev != &health_status.overall_health || health_status.has_issues(),
        };
        
        if should_display {
            println!("[{}] {} Health: {} {} | {}", 
                timestamp.to_string().bright_black(),
                "📊".blue(),
                health_status.overall_health.emoji(), 
                health_status.overall_health.label(),
                health_status.get_score_display().cyan()
            );
            
            if health_status.has_issues() {
                for issue in &health_status.issues {
                    println!("  {} {}", "❌".red(), issue);
                }
                for warning in &health_status.warnings {
                    println!("  {} {}", "⚠️".yellow(), warning);
                }
            }
        }
        
        last_health = Some(health_status.overall_health);
        
        std::thread::sleep(std::time::Duration::from_secs(interval));
    }
}