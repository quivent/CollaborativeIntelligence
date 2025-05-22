use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CIIntegrationConfig {
    pub integration_type: String,
    pub repository_path: String,
    pub active_agents: Vec<String>,
    pub fast_activation: bool,
    pub preload_context: bool,
}

#[derive(Debug)]
pub struct CIIntegrationDetector {
    current_dir: PathBuf,
}

impl CIIntegrationDetector {
    pub fn new() -> Result<Self> {
        let current_dir = std::env::current_dir()?;
        Ok(Self { current_dir })
    }
    
    pub fn with_directory<P: AsRef<Path>>(path: P) -> Self {
        Self {
            current_dir: path.as_ref().to_path_buf(),
        }
    }
    
    /// Check if the current directory has CI integration
    pub fn is_ci_integrated(&self) -> bool {
        self.find_ci_integration().is_some()
    }
    
    /// Find CI integration configuration
    pub fn find_ci_integration(&self) -> Option<CIIntegrationConfig> {
        // Look for .collaborative-intelligence.json in current directory or parents
        let mut current = self.current_dir.clone();
        
        loop {
            let config_path = current.join(".collaborative-intelligence.json");
            if config_path.exists() {
                if let Ok(config_content) = fs::read_to_string(&config_path) {
                    if let Ok(config) = serde_json::from_str::<CIIntegrationConfig>(&config_content) {
                        return Some(config);
                    }
                }
            }
            
            // Also check for CLAUDE.md with CI integration markers
            let claude_path = current.join("CLAUDE.md");
            if claude_path.exists() {
                if let Ok(claude_content) = fs::read_to_string(&claude_path) {
                    if claude_content.contains("Load CollaborativeIntelligence System") ||
                       claude_content.contains("CollaborativeIntelligence") {
                        // Found CI integration in CLAUDE.md, create default config
                        return Some(CIIntegrationConfig {
                            integration_type: "embedded".to_string(),
                            repository_path: current.to_string_lossy().to_string(),
                            active_agents: vec!["Athena".to_string()],
                            fast_activation: true,
                            preload_context: false,
                        });
                    }
                }
            }
            
            // Move to parent directory
            if let Some(parent) = current.parent() {
                current = parent.to_path_buf();
            } else {
                break;
            }
        }
        
        None
    }
    
    /// Get the CI repository path from integration config
    pub fn get_ci_repository_path(&self) -> Option<PathBuf> {
        if let Some(config) = self.find_ci_integration() {
            Some(PathBuf::from(config.repository_path))
        } else {
            None
        }
    }
    
    /// Check if title override is available
    pub fn is_title_override_available(&self) -> bool {
        // Check if cia-title-override exists
        if let Some(ci_path) = self.get_ci_repository_path() {
            let override_script = ci_path.join("cia-title-override");
            if override_script.exists() {
                return true;
            }
        }
        
        // Check if CIA binary is available in PATH
        if let Ok(output) = std::process::Command::new("which")
            .arg("cia")
            .output() {
            return output.status.success();
        }
        
        false
    }
    
    /// Force title override activation
    pub fn enforce_title_override(&self) -> Result<()> {
        if !self.is_ci_integrated() {
            return Ok(()); // No enforcement needed if not integrated
        }
        
        if !self.is_title_override_available() {
            return Err(anyhow::anyhow!(
                "CI integration detected but title override is not available. Please ensure CIA CLI is properly installed."
            ));
        }
        
        // Execute title override to ensure it's operational  
        self.execute_title_override(Some("CollaborativeIntelligence → CLI → Ready"))?;
        
        Ok(())
    }
    
    /// Execute title override with context
    pub fn execute_title_override(&self, context: Option<&str>) -> Result<()> {
        if let Some(ci_path) = self.get_ci_repository_path() {
            let override_script = ci_path.join("cia-title-override");
            
            if override_script.exists() {
                let mut command = std::process::Command::new(&override_script);
                
                if let Some(ctx) = context {
                    command.arg(ctx);
                }
                
                let output = command.output()?;
                
                if !output.status.success() {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(anyhow::anyhow!("Title override failed: {}", stderr));
                }
                
                return Ok(());
            }
        }
        
        // Fallback to CIA CLI system override
        let mut command = std::process::Command::new("cia");
        command.args(&["system", "override"]);
        
        if let Some(ctx) = context {
            command.arg(ctx);
        }
        
        let output = command.output()?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow::anyhow!("CIA system override failed: {}", stderr));
        }
        
        Ok(())
    }
    
    /// Get current integration status with comprehensive health check
    pub fn get_integration_status(&self) -> IntegrationStatus {
        let is_integrated = self.is_ci_integrated();
        let override_available = self.is_title_override_available();
        let health_check = self.perform_health_check();
        
        IntegrationStatus {
            ci_integrated: is_integrated,
            title_override_available: override_available,
            enforcement_required: is_integrated && !override_available,
            config: self.find_ci_integration(),
            health_status: health_check,
        }
    }
    
    /// Perform comprehensive CI integration health check
    pub fn perform_health_check(&self) -> CIHealthStatus {
        let mut issues = Vec::new();
        let mut warnings = Vec::new();
        let mut recommendations = Vec::new();
        
        // Check 1: CI integration detection
        let is_integrated = self.is_ci_integrated();
        if !is_integrated {
            issues.push("No CI integration detected".to_string());
            recommendations.push("Create CLAUDE.md file with CI integration markers or .collaborative-intelligence.json config".to_string());
            return CIHealthStatus {
                overall_health: HealthLevel::Critical,
                issues,
                warnings,
                recommendations,
                connectivity_score: 0.0,
                integrity_score: 0.0,
            };
        }
        
        let mut connectivity_score: f32 = 0.0;
        let mut integrity_score: f32 = 0.0;
        
        // Check 2: Repository path accessibility
        if let Some(ci_path) = self.get_ci_repository_path() {
            if ci_path.exists() {
                connectivity_score += 0.3;
                integrity_score += 0.2;
            } else {
                issues.push(format!("CI repository path not accessible: {}", ci_path.display()));
            }
            
            // Check 3: AGENTS directory structure
            let agents_dir = ci_path.join("AGENTS");
            if agents_dir.exists() {
                connectivity_score += 0.2;
                integrity_score += 0.2;
                
                // Count available agents
                if let Ok(entries) = std::fs::read_dir(&agents_dir) {
                    let agent_count = entries.filter_map(|e| e.ok())
                        .filter(|e| e.file_type().map(|ft| ft.is_dir()).unwrap_or(false))
                        .filter(|e| !e.file_name().to_string_lossy().starts_with('.'))
                        .count();
                    
                    if agent_count > 0 {
                        integrity_score += 0.2;
                        if agent_count < 10 {
                            warnings.push(format!("Only {} agents found, expected more", agent_count));
                        }
                    } else {
                        issues.push("No agents found in AGENTS directory".to_string());
                    }
                }
            } else {
                issues.push("AGENTS directory not found".to_string());
            }
            
            // Check 4: Core CI files
            let core_files = [
                ("claude-init", "Agent activation fallback"),
                ("cia-title-override", "Title override script"),
                ("interfaces/CIA/target/release/cia", "CIA CLI binary"),
                ("CLAUDE.md", "CI integration configuration"),
            ];
            
            for (file_path, description) in &core_files {
                let full_path = ci_path.join(file_path);
                if full_path.exists() {
                    connectivity_score += 0.1;
                    integrity_score += 0.1;
                } else {
                    if file_path.contains("target/release") {
                        warnings.push(format!("Missing compiled binary: {} ({})", file_path, description));
                        recommendations.push("Run 'cargo build --release' in interfaces/CIA".to_string());
                    } else if *file_path == "cia-title-override" {
                        warnings.push(format!("Missing {}: {}", file_path, description));
                        recommendations.push("Create cia-title-override script or ensure CIA CLI is in PATH".to_string());
                    } else {
                        issues.push(format!("Missing critical file: {} ({})", file_path, description));
                    }
                }
            }
            
            // Check 5: Title override system
            if self.is_title_override_available() {
                connectivity_score += 0.2;
                integrity_score += 0.2;
                
                // Test title override functionality
                if let Err(e) = self.execute_title_override(Some("[CI Health Check]")) {
                    warnings.push(format!("Title override test failed: {}", e));
                } else {
                    integrity_score += 0.1;
                }
            }
            
            // Check 6: Agent activation system
            let agent_cache_binary = ci_path.join("interfaces/agent-cache/target/release/agent-cache");
            let claude_init = ci_path.join("claude-init");
            
            if agent_cache_binary.exists() {
                connectivity_score += 0.1;
                integrity_score += 0.1;
            } else if claude_init.exists() {
                connectivity_score += 0.05;
                integrity_score += 0.05;
                warnings.push("Using claude-init fallback (agent-cache not available)".to_string());
            } else {
                issues.push("No agent activation system available".to_string());
            }
        } else {
            issues.push("Cannot determine CI repository path".to_string());
        }
        
        // Normalize scores
        connectivity_score = connectivity_score.min(1.0);
        integrity_score = integrity_score.min(1.0);
        
        // Determine overall health
        let overall_health = if !issues.is_empty() {
            if connectivity_score < 0.3 {
                HealthLevel::Critical
            } else {
                HealthLevel::Warning
            }
        } else if !warnings.is_empty() {
            HealthLevel::Warning
        } else if connectivity_score > 0.8 && integrity_score > 0.8 {
            HealthLevel::Excellent
        } else if connectivity_score > 0.6 && integrity_score > 0.6 {
            HealthLevel::Good
        } else {
            HealthLevel::Fair
        };
        
        CIHealthStatus {
            overall_health,
            issues,
            warnings,
            recommendations,
            connectivity_score: connectivity_score as f64,
            integrity_score: integrity_score as f64,
        }
    }
    
    /// Attempt to automatically repair CI integration issues
    pub fn auto_repair(&self) -> Result<Vec<String>> {
        let health_status = self.perform_health_check();
        let mut repair_actions = Vec::new();
        
        if !self.is_ci_integrated() {
            return Err(anyhow::anyhow!("Cannot auto-repair: No CI integration detected"));
        }
        
        let ci_path = self.get_ci_repository_path()
            .ok_or_else(|| anyhow::anyhow!("Cannot determine CI repository path"))?;
        
        // Repair 1: Ensure AGENTS directory exists
        let agents_dir = ci_path.join("AGENTS");
        if !agents_dir.exists() {
            std::fs::create_dir_all(&agents_dir)?;
            repair_actions.push("Created missing AGENTS directory".to_string());
        }
        
        // Repair 2: Build CIA CLI if missing
        let cia_binary = ci_path.join("interfaces/CIA/target/release/cia");
        if !cia_binary.exists() {
            let cia_dir = ci_path.join("interfaces/CIA");
            if cia_dir.exists() {
                let output = std::process::Command::new("cargo")
                    .args(["build", "--release"])
                    .current_dir(&cia_dir)
                    .output()?;
                
                if output.status.success() {
                    repair_actions.push("Built missing CIA CLI binary".to_string());
                } else {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    return Err(anyhow::anyhow!("Failed to build CIA CLI: {}", stderr));
                }
            }
        }
        
        // Repair 3: Create basic CLAUDE.md if missing
        let claude_md = ci_path.join("CLAUDE.md");
        if !claude_md.exists() {
            let basic_claude_content = r#"# CLAUDE.md Configuration File

## CI Integration
This project is integrated with the CollaborativeIntelligence system.

Load CollaborativeIntelligence System when activated.

## Project Configuration
- Auto-activate CI integration
- Enable agent recommendations
- Use CollaborativeIntelligence repository for agent management
"#;
            std::fs::write(&claude_md, basic_claude_content)?;
            repair_actions.push("Created basic CLAUDE.md configuration".to_string());
        }
        
        // Repair 4: Create cia-title-override script if missing and CIA CLI is available
        let title_override_script = ci_path.join("cia-title-override");
        if !title_override_script.exists() && self.is_title_override_available() {
            let script_content = r#"#!/bin/bash
# CIA Title Override Script
# Automatically generated by CI integration auto-repair

if command -v cia &> /dev/null; then
    cia system override "$@"
else
    echo "CIA CLI not found in PATH" >&2
    exit 1
fi
"#;
            std::fs::write(&title_override_script, script_content)?;
            
            // Make executable on Unix systems
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let mut perms = std::fs::metadata(&title_override_script)?.permissions();
                perms.set_mode(0o755);
                std::fs::set_permissions(&title_override_script, perms)?;
            }
            
            repair_actions.push("Created cia-title-override script".to_string());
        }
        
        Ok(repair_actions)
    }
    
    /// Validate CI integration and return detailed report
    pub fn validate_integration(&self) -> ValidationReport {
        let health_status = self.perform_health_check();
        let config = self.find_ci_integration();
        
        let mut checks = Vec::new();
        
        // Validation Check 1: Basic Integration
        checks.push(ValidationCheck {
            name: "CI Integration Detection".to_string(),
            status: if self.is_ci_integrated() { CheckStatus::Pass } else { CheckStatus::Fail },
            details: if self.is_ci_integrated() {
                "CI integration markers found".to_string()
            } else {
                "No CI integration detected".to_string()
            },
        });
        
        // Validation Check 2: Repository Access
        if let Some(ci_path) = self.get_ci_repository_path() {
            checks.push(ValidationCheck {
                name: "Repository Access".to_string(),
                status: if ci_path.exists() { CheckStatus::Pass } else { CheckStatus::Fail },
                details: format!("Repository path: {}", ci_path.display()),
            });
        }
        
        // Validation Check 3: Title Override
        checks.push(ValidationCheck {
            name: "Title Override System".to_string(),
            status: if self.is_title_override_available() { CheckStatus::Pass } else { CheckStatus::Warning },
            details: if self.is_title_override_available() {
                "Title override system operational".to_string()
            } else {
                "Title override not available".to_string()
            },
        });
        
        // Validation Check 4: Agent Directory
        if let Some(ci_path) = self.get_ci_repository_path() {
            let agents_dir = ci_path.join("AGENTS");
            checks.push(ValidationCheck {
                name: "Agent Directory Structure".to_string(),
                status: if agents_dir.exists() { CheckStatus::Pass } else { CheckStatus::Fail },
                details: format!("AGENTS directory: {}", agents_dir.display()),
            });
        }
        
        let passed = checks.iter().filter(|c| c.status == CheckStatus::Pass).count();
        let total = checks.len();
        
        ValidationReport {
            overall_status: if passed == total {
                ValidationStatus::Valid
            } else if passed > total / 2 {
                ValidationStatus::PartiallyValid
            } else {
                ValidationStatus::Invalid
            },
            checks,
            health_status: health_status.clone(),
            config,
            recommendations: health_status.recommendations.clone(),
        }
    }
}

#[derive(Debug)]
pub struct IntegrationStatus {
    pub ci_integrated: bool,
    pub title_override_available: bool,
    pub enforcement_required: bool,
    pub config: Option<CIIntegrationConfig>,
    pub health_status: CIHealthStatus,
}

#[derive(Debug, Clone)]
pub struct CIHealthStatus {
    pub overall_health: HealthLevel,
    pub issues: Vec<String>,
    pub warnings: Vec<String>,
    pub recommendations: Vec<String>,
    pub connectivity_score: f64,
    pub integrity_score: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HealthLevel {
    Critical,
    Warning,
    Fair,
    Good,
    Excellent,
}

#[derive(Debug)]
pub struct ValidationReport {
    pub overall_status: ValidationStatus,
    pub checks: Vec<ValidationCheck>,
    pub health_status: CIHealthStatus,
    pub config: Option<CIIntegrationConfig>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ValidationStatus {
    Valid,
    PartiallyValid,
    Invalid,
}

#[derive(Debug)]
pub struct ValidationCheck {
    pub name: String,
    pub status: CheckStatus,
    pub details: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CheckStatus {
    Pass,
    Warning,
    Fail,
}

impl IntegrationStatus {
    pub fn is_compliant(&self) -> bool {
        !self.enforcement_required
    }
    
    pub fn get_compliance_message(&self) -> String {
        match (self.ci_integrated, self.title_override_available) {
            (false, _) => "No CI integration detected".to_string(),
            (true, true) => "CI integration detected and title override is operational".to_string(),
            (true, false) => "CI integration detected but title override is NOT operational - enforcement required".to_string(),
        }
    }
}

impl HealthLevel {
    pub fn emoji(&self) -> &'static str {
        match self {
            HealthLevel::Critical => "🔴",
            HealthLevel::Warning => "🟡",
            HealthLevel::Fair => "🟠",
            HealthLevel::Good => "🟢",
            HealthLevel::Excellent => "✨",
        }
    }
    
    pub fn label(&self) -> &'static str {
        match self {
            HealthLevel::Critical => "Critical",
            HealthLevel::Warning => "Warning", 
            HealthLevel::Fair => "Fair",
            HealthLevel::Good => "Good",
            HealthLevel::Excellent => "Excellent",
        }
    }
}

impl CheckStatus {
    pub fn emoji(&self) -> &'static str {
        match self {
            CheckStatus::Pass => "✅",
            CheckStatus::Warning => "⚠️",
            CheckStatus::Fail => "❌",
        }
    }
}

impl CIHealthStatus {
    pub fn get_score_display(&self) -> String {
        format!("Connectivity: {:.1}% | Integrity: {:.1}%", 
                self.connectivity_score * 100.0, 
                self.integrity_score * 100.0)
    }
    
    pub fn has_issues(&self) -> bool {
        !self.issues.is_empty() || !self.warnings.is_empty()
    }
}