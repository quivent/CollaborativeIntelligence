// Emergency Audit Logging System for AUTH_BYPASS_001
// Critical security monitoring and incident tracking

use std::fs::{File, OpenOptions};
use std::io::{Write, BufWriter};
use std::path::Path;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub timestamp: DateTime<Utc>,
    pub event_type: SecurityEventType,
    pub agent_name: Option<String>,
    pub user_action: Option<String>,
    pub ip_address: Option<String>,
    pub success: bool,
    pub risk_level: RiskLevel,
    pub description: String,
    pub incident_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEventType {
    AuthenticationAttempt,
    IdentityChallenge,
    RiskAcceptance,
    OperationBlocked,
    SecurityBypass,
    EmergencyOverride,
    SystemCompromise,
    UserVerification,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

pub struct AuditLogger {
    log_file: String,
    incident_id: String,
    buffer: BufWriter<File>,
}

impl AuditLogger {
    pub fn new(log_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let path = Path::new(log_file);
        
        // Create directory if it doesn't exist
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;
            
        let buffer = BufWriter::new(file);
        
        Ok(AuditLogger {
            log_file: log_file.to_string(),
            incident_id: "AUTH_BYPASS_001".to_string(),
            buffer,
        })
    }

    /// Log a security event with full audit trail
    pub fn log_event(&mut self, event: SecurityEvent) -> Result<(), Box<dyn std::error::Error>> {
        let log_entry = format!(
            "[{}] {} | {} | {} | {} | {}\n",
            event.timestamp.format("%Y-%m-%d %H:%M:%S UTC"),
            self.format_risk_level(&event.risk_level),
            self.format_event_type(&event.event_type),
            event.agent_name.unwrap_or_else(|| "UNKNOWN".to_string()),
            if event.success { "SUCCESS" } else { "FAILED" },
            event.description
        );
        
        self.buffer.write_all(log_entry.as_bytes())?;
        self.buffer.flush()?;
        
        // Also log to stderr for immediate visibility
        eprintln!("🔒 AUDIT: {}", log_entry.trim());
        
        Ok(())
    }

    /// Log authentication attempt
    pub fn log_authentication_attempt(&mut self, agent_name: &str, success: bool, user_action: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::AuthenticationAttempt,
            agent_name: Some(agent_name.to_string()),
            user_action: Some(user_action.to_string()),
            ip_address: None,
            success,
            risk_level: if success { RiskLevel::High } else { RiskLevel::Critical },
            description: format!(
                "Agent {} authentication attempt: {} (User: {})", 
                agent_name, 
                if success { "Risk accepted by user" } else { "Blocked by user" },
                user_action
            ),
            incident_id: self.incident_id.clone(),
        };
        
        self.log_event(event)
    }

    /// Log identity challenge
    pub fn log_identity_challenge(&mut self, agent_name: &str, challenge_type: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::IdentityChallenge,
            agent_name: Some(agent_name.to_string()),
            user_action: None,
            ip_address: None,
            success: true,
            risk_level: RiskLevel::Info,
            description: format!("Identity challenge issued to {} (Type: {})", agent_name, challenge_type),
            incident_id: self.incident_id.clone(),
        };
        
        self.log_event(event)
    }

    /// Log security bypass or compromise
    pub fn log_security_compromise(&mut self, description: &str, risk_level: RiskLevel) -> Result<(), Box<dyn std::error::Error>> {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::SystemCompromise,
            agent_name: None,
            user_action: None,
            ip_address: None,
            success: false,
            risk_level,
            description: description.to_string(),
            incident_id: self.incident_id.clone(),
        };
        
        self.log_event(event)
    }

    /// Log emergency override
    pub fn log_emergency_override(&mut self, reason: &str, authorized_by: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event = SecurityEvent {
            timestamp: Utc::now(),
            event_type: SecurityEventType::EmergencyOverride,
            agent_name: None,
            user_action: Some(authorized_by.to_string()),
            ip_address: None,
            success: true,
            risk_level: RiskLevel::Critical,
            description: format!("Emergency override: {} (Authorized by: {})", reason, authorized_by),
            incident_id: self.incident_id.clone(),
        };
        
        self.log_event(event)
    }

    /// Generate security summary report
    pub fn generate_summary_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(&self.log_file)?;
        let lines = content.lines().collect::<Vec<_>>();
        
        let total_events = lines.len();
        let critical_events = lines.iter().filter(|line| line.contains("CRITICAL")).count();
        let failed_events = lines.iter().filter(|line| line.contains("FAILED")).count();
        let auth_attempts = lines.iter().filter(|line| line.contains("AuthenticationAttempt")).count();
        
        let report = format!(
            "SECURITY AUDIT SUMMARY REPORT\n\
            Incident: {}\n\
            Report Generated: {}\n\
            Log File: {}\n\
            \n\
            EVENT STATISTICS:\n\
            Total Events: {}\n\
            Critical Events: {}\n\
            Failed Operations: {}\n\
            Authentication Attempts: {}\n\
            \n\
            RISK ANALYSIS:\n\
            System Status: {}\n\
            Primary Risk: {}\n\
            Mitigation Status: {}\n\
            \n\
            RECENT EVENTS (Last 10):\n\
            {}",
            self.incident_id,
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            self.log_file,
            total_events,
            critical_events,
            failed_events,
            auth_attempts,
            if critical_events > 0 { "COMPROMISED" } else { "MONITORING" },
            "Agent authentication bypass vulnerability",
            "Emergency authentication system deployed",
            lines.iter()
                .rev()
                .take(10)
                .map(|line| format!("  {}", line))
                .collect::<Vec<_>>()
                .join("\n")
        );
        
        Ok(report)
    }

    fn format_risk_level(&self, level: &RiskLevel) -> &str {
        match level {
            RiskLevel::Critical => "🔴 CRITICAL",
            RiskLevel::High => "🟠 HIGH",
            RiskLevel::Medium => "🟡 MEDIUM",
            RiskLevel::Low => "🟢 LOW",
            RiskLevel::Info => "🔵 INFO",
        }
    }

    fn format_event_type(&self, event_type: &SecurityEventType) -> &str {
        match event_type {
            SecurityEventType::AuthenticationAttempt => "AUTH_ATTEMPT",
            SecurityEventType::IdentityChallenge => "IDENTITY_CHALLENGE",
            SecurityEventType::RiskAcceptance => "RISK_ACCEPTED",
            SecurityEventType::OperationBlocked => "OPERATION_BLOCKED",
            SecurityEventType::SecurityBypass => "SECURITY_BYPASS",
            SecurityEventType::EmergencyOverride => "EMERGENCY_OVERRIDE",
            SecurityEventType::SystemCompromise => "SYSTEM_COMPROMISE",
            SecurityEventType::UserVerification => "USER_VERIFICATION",
        }
    }
}

impl Drop for AuditLogger {
    fn drop(&mut self) {
        let _ = self.buffer.flush();
    }
}

/// Global audit logger instance
pub fn initialize_audit_logging() -> Result<AuditLogger, Box<dyn std::error::Error>> {
    let log_file = "security_audit.log";
    let mut logger = AuditLogger::new(log_file)?;
    
    // Log system initialization
    logger.log_security_compromise(
        "Emergency audit logging system initialized for AUTH_BYPASS_001",
        RiskLevel::Critical
    )?;
    
    Ok(logger)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_audit_logger_creation() {
        let temp_file = NamedTempFile::new().unwrap();
        let logger = AuditLogger::new(temp_file.path().to_str().unwrap());
        assert!(logger.is_ok());
    }

    #[test]
    fn test_authentication_logging() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut logger = AuditLogger::new(temp_file.path().to_str().unwrap()).unwrap();
        
        let result = logger.log_authentication_attempt("TestAgent", false, "DENY_RISK");
        assert!(result.is_ok());
    }

    #[test]
    fn test_summary_report_generation() {
        let temp_file = NamedTempFile::new().unwrap();
        let mut logger = AuditLogger::new(temp_file.path().to_str().unwrap()).unwrap();
        
        logger.log_authentication_attempt("TestAgent", false, "DENY_RISK").unwrap();
        let report = logger.generate_summary_report();
        assert!(report.is_ok());
        assert!(report.unwrap().contains("SECURITY AUDIT SUMMARY"));
    }
}