// Emergency Security Monitoring System for AUTH_BYPASS_001
// Real-time detection and alerting for authentication bypass attempts

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use crate::audit_logger::{AuditLogger, RiskLevel};
use chrono::Utc;

#[derive(Debug, Clone)]
pub struct SecurityMetrics {
    pub auth_attempts: u64,
    pub failed_operations: u64,
    pub risk_acceptances: u64,
    pub identity_challenges: u64,
    pub emergency_overrides: u64,
    pub last_incident: Option<Instant>,
    pub system_health: SystemHealth,
}

#[derive(Debug, Clone)]
pub enum SystemHealth {
    Critical,      // Active security incident
    Degraded,      // Some security functions compromised
    Monitoring,    // Security monitoring active
    Operational,   // Normal security operations
}

#[derive(Debug, Clone)]
pub struct SecurityAlert {
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: Instant,
    pub requires_action: bool,
    pub escalation_level: u8,
}

#[derive(Debug, Clone)]
pub enum AlertSeverity {
    Emergency,    // Immediate action required
    Critical,     // Urgent attention needed
    Warning,      // Monitoring required
    Info,         // Informational
}

pub struct SecurityMonitor {
    metrics: Arc<Mutex<SecurityMetrics>>,
    alerts: Arc<Mutex<Vec<SecurityAlert>>>,
    monitoring: bool,
    audit_logger: Arc<Mutex<AuditLogger>>,
}

impl SecurityMonitor {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let audit_logger = Arc::new(Mutex::new(AuditLogger::new("security_monitor.log")?));
        
        let monitor = SecurityMonitor {
            metrics: Arc::new(Mutex::new(SecurityMetrics {
                auth_attempts: 0,
                failed_operations: 0,
                risk_acceptances: 0,
                identity_challenges: 0,
                emergency_overrides: 0,
                last_incident: None,
                system_health: SystemHealth::Critical, // Start in critical due to AUTH_BYPASS_001
            })),
            alerts: Arc::new(Mutex::new(Vec::new())),
            monitoring: false,
            audit_logger,
        };
        
        // Generate initial critical alert
        monitor.generate_alert(
            AlertSeverity::Emergency,
            "AUTH_BYPASS_001: Agent authentication system compromised - All agents unverified",
            true,
            5
        )?;
        
        Ok(monitor)
    }

    /// Start continuous security monitoring
    pub fn start_monitoring(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.monitoring {
            return Ok(());
        }
        
        self.monitoring = true;
        let metrics = Arc::clone(&self.metrics);
        let alerts = Arc::clone(&self.alerts);
        let audit_logger = Arc::clone(&self.audit_logger);
        
        thread::spawn(move || {
            loop {
                thread::sleep(Duration::from_secs(30)); // Monitor every 30 seconds
                
                if let Ok(mut logger) = audit_logger.lock() {
                    let _ = logger.log_security_compromise(
                        "Security monitoring heartbeat - AUTH_BYPASS_001 active",
                        RiskLevel::High
                    );
                }
                
                // Check for critical thresholds
                if let Ok(metrics_guard) = metrics.lock() {
                    let current_metrics = metrics_guard.clone();
                    drop(metrics_guard);
                    
                    // Alert if too many failed operations
                    if current_metrics.failed_operations > 10 {
                        if let Ok(mut alerts_guard) = alerts.lock() {
                            alerts_guard.push(SecurityAlert {
                                severity: AlertSeverity::Critical,
                                message: format!("High failure rate detected: {} failed operations", current_metrics.failed_operations),
                                timestamp: Instant::now(),
                                requires_action: true,
                                escalation_level: 3,
                            });
                        }
                    }
                    
                    // Alert if no recent activity (possible system freeze)
                    if let Some(last_incident) = current_metrics.last_incident {
                        if last_incident.elapsed() > Duration::from_secs(300) { // 5 minutes
                            if let Ok(mut alerts_guard) = alerts.lock() {
                                alerts_guard.push(SecurityAlert {
                                    severity: AlertSeverity::Warning,
                                    message: "No security events in 5+ minutes - possible system inactivity".to_string(),
                                    timestamp: Instant::now(),
                                    requires_action: false,
                                    escalation_level: 1,
                                });
                            }
                        }
                    }
                }
            }
        });
        
        Ok(())
    }

    /// Record authentication attempt
    pub fn record_auth_attempt(&self, agent_name: &str, success: bool, user_decision: &str) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.auth_attempts += 1;
            metrics.last_incident = Some(Instant::now());
            
            if success {
                metrics.risk_acceptances += 1;
            } else {
                metrics.failed_operations += 1;
            }
        }
        
        // Log to audit system
        {
            let mut logger = self.audit_logger.lock().unwrap();
            logger.log_authentication_attempt(agent_name, success, user_decision)?;
        }
        
        // Generate alert for each authentication attempt
        let alert_message = if success {
            format!("User accepted security risk for agent {} - Operation proceeded without verification", agent_name)
        } else {
            format!("User blocked unverified agent {} - Security enforcement successful", agent_name)
        };
        
        self.generate_alert(
            if success { AlertSeverity::Warning } else { AlertSeverity::Info },
            &alert_message,
            success, // Require action only if risk was accepted
            if success { 2 } else { 0 }
        )?;
        
        Ok(())
    }

    /// Record identity challenge
    pub fn record_identity_challenge(&self, agent_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.identity_challenges += 1;
            metrics.last_incident = Some(Instant::now());
        }
        
        {
            let mut logger = self.audit_logger.lock().unwrap();
            logger.log_identity_challenge(agent_name, "USER_INITIATED")?;
        }
        
        self.generate_alert(
            AlertSeverity::Info,
            &format!("Identity challenge issued to agent {}", agent_name),
            false,
            0
        )?;
        
        Ok(())
    }

    /// Record emergency override
    pub fn record_emergency_override(&self, reason: &str, authorized_by: &str) -> Result<(), Box<dyn std::error::Error>> {
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.emergency_overrides += 1;
            metrics.last_incident = Some(Instant::now());
            metrics.system_health = SystemHealth::Critical; // Override escalates to critical
        }
        
        {
            let mut logger = self.audit_logger.lock().unwrap();
            logger.log_emergency_override(reason, authorized_by)?;
        }
        
        self.generate_alert(
            AlertSeverity::Emergency,
            &format!("EMERGENCY OVERRIDE: {} (By: {})", reason, authorized_by),
            true,
            5
        )?;
        
        Ok(())
    }

    /// Generate security alert
    fn generate_alert(&self, severity: AlertSeverity, message: &str, requires_action: bool, escalation_level: u8) -> Result<(), Box<dyn std::error::Error>> {
        let alert = SecurityAlert {
            severity,
            message: message.to_string(),
            timestamp: Instant::now(),
            requires_action,
            escalation_level,
        };
        
        // Add to alerts queue
        {
            let mut alerts = self.alerts.lock().unwrap();
            alerts.push(alert.clone());
            
            // Keep only last 100 alerts
            if alerts.len() > 100 {
                let drain_count = alerts.len() - 100;
                alerts.drain(0..drain_count);
            }
        }
        
        // Print immediate alert to console
        let severity_emoji = match alert.severity {
            AlertSeverity::Emergency => "🚨",
            AlertSeverity::Critical => "🔴",
            AlertSeverity::Warning => "🟡",
            AlertSeverity::Info => "🔵",
        };
        
        eprintln!("{} SECURITY ALERT: {}", severity_emoji, alert.message);
        
        if alert.requires_action {
            eprintln!("⚡ ACTION REQUIRED - Escalation Level: {}", alert.escalation_level);
        }
        
        Ok(())
    }

    /// Get current security metrics
    pub fn get_metrics(&self) -> SecurityMetrics {
        self.metrics.lock().unwrap().clone()
    }

    /// Get recent alerts
    pub fn get_recent_alerts(&self, limit: usize) -> Vec<SecurityAlert> {
        let alerts = self.alerts.lock().unwrap();
        alerts.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }

    /// Get system health status
    pub fn get_system_health(&self) -> SystemHealth {
        self.metrics.lock().unwrap().system_health.clone()
    }

    /// Generate comprehensive security report
    pub fn generate_monitoring_report(&self) -> Result<String, Box<dyn std::error::Error>> {
        let metrics = self.get_metrics();
        let recent_alerts = self.get_recent_alerts(10);
        
        let health_status = match metrics.system_health {
            SystemHealth::Critical => "🔴 CRITICAL",
            SystemHealth::Degraded => "🟡 DEGRADED",
            SystemHealth::Monitoring => "🟠 MONITORING",
            SystemHealth::Operational => "🟢 OPERATIONAL",
        };
        
        let mut report = format!(
            "SECURITY MONITORING REPORT\n\
            Generated: {}\n\
            System Health: {}\n\
            \n\
            SECURITY METRICS:\n\
            Authentication Attempts: {}\n\
            Failed Operations: {}\n\
            Risk Acceptances: {}\n\
            Identity Challenges: {}\n\
            Emergency Overrides: {}\n\
            \n\
            INCIDENT STATUS:\n\
            Active Incident: AUTH_BYPASS_001\n\
            Last Activity: {}\n\
            \n\
            RECENT ALERTS:\n",
            Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
            health_status,
            metrics.auth_attempts,
            metrics.failed_operations,
            metrics.risk_acceptances,
            metrics.identity_challenges,
            metrics.emergency_overrides,
            if let Some(last) = metrics.last_incident {
                format!("{:.1} seconds ago", last.elapsed().as_secs_f64())
            } else {
                "No recent activity".to_string()
            }
        );
        
        for (i, alert) in recent_alerts.iter().enumerate() {
            let severity_text = match alert.severity {
                AlertSeverity::Emergency => "EMERGENCY",
                AlertSeverity::Critical => "CRITICAL",
                AlertSeverity::Warning => "WARNING",
                AlertSeverity::Info => "INFO",
            };
            
            report.push_str(&format!(
                "  {}. [{}] {} ({}s ago)\n",
                i + 1,
                severity_text,
                alert.message,
                alert.timestamp.elapsed().as_secs()
            ));
        }
        
        // Add audit log summary
        let audit_logger = self.audit_logger.lock().unwrap();
        let audit_summary = audit_logger.generate_summary_report()?;
        report.push_str("\n");
        report.push_str(&audit_summary);
        
        Ok(report)
    }
}

/// Initialize global security monitoring
pub fn initialize_security_monitoring() -> Result<SecurityMonitor, Box<dyn std::error::Error>> {
    let mut monitor = SecurityMonitor::new()?;
    monitor.start_monitoring()?;
    
    eprintln!("🛡️ Security monitoring system initialized for AUTH_BYPASS_001");
    eprintln!("🔴 CRITICAL: All agent operations require manual risk acceptance");
    
    Ok(monitor)
}