use anyhow::{Context, Result};
use clap::{Arg, ArgMatches, Command};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::{Command as StdCommand, Stdio};
use std::io::{self, Write};
use std::thread;
use std::time::{Duration, Instant};

use crate::errors::CIError;
use crate::helpers::path::get_ci_root;

#[derive(Debug, Serialize, Deserialize)]
struct SessionInfo {
    id: String,
    status: String,
    lane: Option<String>,
    registered_at: String,
    last_heartbeat: String,
    hostname: Option<String>,
    tasks_completed: u64,
    tasks_reserved: u64,
    heartbeat_count: u64,
    duration_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemStatus {
    system_health: SystemHealth,
    session_counts: HashMap<String, u64>,
    lane_assignments: HashMap<String, u64>,
    coordination: CoordinationInfo,
    metadata: SystemMetadata,
    last_updated: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemHealth {
    status: String,
    active_sessions: u64,
    max_concurrent: u64,
    utilization_percent: u64,
    timeout_rate: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct CoordinationInfo {
    active_sessions: u64,
    total_sessions_registered: u64,
    session_timeouts: u64,
    last_cleanup: String,
    heartbeat_interval_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct SystemMetadata {
    max_concurrent_sessions: u64,
    session_timeout_minutes: u64,
    auto_cleanup_enabled: bool,
    last_update: String,
}

pub fn create_command() -> Command {
    Command::new("dashboard")
        .about("Launch coordination dashboard")
        .arg(
            Arg::new("web")
                .help("Launch web dashboard instead of terminal")
                .long("web")
                .action(clap::ArgAction::SetTrue)
        )
        .arg(
            Arg::new("port")
                .help("Port for web dashboard")
                .long("port")
                .short('p')
                .value_name("PORT")
                .value_parser(clap::value_parser!(u16))
                .default_value("8080")
        )
        .arg(
            Arg::new("refresh")
                .help("Refresh interval in seconds for terminal dashboard")
                .long("refresh")
                .short('r')
                .value_name("SECONDS")
                .value_parser(clap::value_parser!(u64))
                .default_value("5")
        )
        .arg(
            Arg::new("view")
                .help("Dashboard view mode")
                .long("view")
                .short('v')
                .value_name("VIEW")
                .value_parser(["system", "sessions", "tasks", "lanes"])
                .default_value("system")
        )
        .arg(
            Arg::new("export")
                .help("Export dashboard data to file")
                .long("export")
                .short('e')
                .value_name("FILE")
        )
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    let web = matches.get_flag("web");
    let port = *matches.get_one::<u16>("port").unwrap();
    let refresh_interval = *matches.get_one::<u64>("refresh").unwrap();
    let view_mode = matches.get_one::<String>("view").unwrap();
    let export_file = matches.get_one::<String>("export");

    if let Some(export_path) = export_file {
        return export_dashboard_data(export_path, view_mode);
    }

    if web {
        launch_web_dashboard(port)
    } else {
        launch_terminal_dashboard(refresh_interval, view_mode)
    }
}

fn get_coordination_script_path() -> Result<PathBuf> {
    let ci_root = get_ci_root()?;
    let script_path = ci_root.join("BRAIN").join("coordination").join("session_coordinator.py");
    
    if !script_path.exists() {
        return Err(CIError::NotFound(format!(
            "Coordination script not found at {}. Make sure BRAIN is properly set up.",
            script_path.display()
        )).into());
    }
    
    Ok(script_path)
}

fn run_coordination_script(args: Vec<String>) -> Result<String> {
    let script_path = get_coordination_script_path()?;
    
    let output = StdCommand::new("python3")
        .arg(&script_path)
        .args(&args)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .with_context(|| "Failed to execute coordination script")?;
    
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(CIError::Command(format!(
            "Coordination script failed: {}",
            stderr
        )).into());
    }
    
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn launch_terminal_dashboard(refresh_interval: u64, view_mode: &str) -> Result<()> {
    println!("{}", "🖥️  Coordination Dashboard".magenta().bold());
    println!("{}", "=".repeat(60).magenta());
    println!("View Mode: {} | Refresh: {}s | Press Ctrl+C to exit", view_mode.cyan(), refresh_interval);
    println!("{}", "=".repeat(60).magenta());

    let mut last_update = Instant::now();
    let refresh_duration = Duration::from_secs(refresh_interval);

    loop {
        if last_update.elapsed() >= refresh_duration {
            // Clear screen
            print!("\x1b[2J\x1b[H");
            io::stdout().flush().unwrap();

            // Display header
            println!("{}", "🖥️  Coordination Dashboard".magenta().bold());
            println!("{}", "=".repeat(60).magenta());
            println!("View: {} | Refresh: {}s | Time: {}", 
                view_mode.cyan(), 
                refresh_interval, 
                chrono::Utc::now().format("%H:%M:%S").to_string().yellow()
            );
            println!("{}", "=".repeat(60).magenta());
            println!();

            match display_dashboard_view(view_mode) {
                Ok(_) => {},
                Err(e) => {
                    eprintln!("{} Dashboard error: {}", "✗".red(), e);
                }
            }

            last_update = Instant::now();
        }

        thread::sleep(Duration::from_millis(100));
    }
}

fn display_dashboard_view(view_mode: &str) -> Result<()> {
    match view_mode {
        "system" => display_system_overview(),
        "sessions" => display_sessions_view(),
        "tasks" => display_tasks_view(),
        "lanes" => display_lanes_view(),
        _ => display_system_overview(),
    }
}

fn display_system_overview() -> Result<()> {
    let args = vec!["status".to_string(), "--system".to_string(), "--format".to_string(), "json".to_string()];
    let output = run_coordination_script(args)?;
    let status: SystemStatus = serde_json::from_str(&output)?;

    // System Health Section
    println!("{}", "🏥 SYSTEM HEALTH".green().bold());
    println!("{}", "─".repeat(40).dimmed());
    
    let health_color = match status.system_health.status.as_str() {
        "healthy" => colored::Color::Green,
        "warning" => colored::Color::Yellow,
        _ => colored::Color::Red,
    };
    
    println!("Status: {}", status.system_health.status.to_uppercase().color(health_color).bold());
    
    // Progress bar for utilization
    let utilization = status.system_health.utilization_percent;
    let bar_width = 30;
    let filled = (utilization as f64 / 100.0 * bar_width as f64) as usize;
    let bar = format!("[{}{}] {}%", 
        "█".repeat(filled).green(), 
        "░".repeat(bar_width - filled).dimmed(), 
        utilization
    );
    
    println!("Active Sessions: {}/{} {}", 
        status.system_health.active_sessions.to_string().cyan().bold(),
        status.system_health.max_concurrent.to_string().dimmed(),
        bar
    );
    
    println!("Timeout Rate: {:.1}%", (status.system_health.timeout_rate * 100.0));
    println!();

    // Session Statistics
    println!("{}", "📊 SESSION STATISTICS".blue().bold());
    println!("{}", "─".repeat(40).dimmed());
    
    for (status_type, count) in &status.session_counts {
        let color = match status_type.as_str() {
            "active" => colored::Color::Green,
            "ended" => colored::Color::Blue,
            "timed_out" => colored::Color::Red,
            _ => colored::Color::White,
        };
        println!("{}: {}", status_type.to_uppercase(), count.to_string().color(color).bold());
    }
    println!();

    // Lane Assignments
    if !status.lane_assignments.is_empty() {
        println!("{}", "🛤️  LANE ASSIGNMENTS".yellow().bold());
        println!("{}", "─".repeat(40).dimmed());
        
        for (lane, count) in &status.lane_assignments {
            println!("{}: {} sessions", lane.cyan(), count.to_string().white().bold());
        }
        println!();
    }

    // Performance Metrics
    println!("{}", "⚡ PERFORMANCE METRICS".magenta().bold());
    println!("{}", "─".repeat(40).dimmed());
    println!("Total Registered: {}", status.coordination.total_sessions_registered.to_string().cyan().bold());
    println!("Session Timeouts: {}", status.coordination.session_timeouts.to_string().red().bold());
    println!("Heartbeat Interval: {}s", status.coordination.heartbeat_interval_seconds);
    println!("Last Cleanup: {}", format_timestamp(&status.coordination.last_cleanup));

    Ok(())
}

fn display_sessions_view() -> Result<()> {
    let args = vec!["status".to_string(), "--all".to_string(), "--format".to_string(), "json".to_string()];
    let output = run_coordination_script(args)?;
    let sessions: Vec<SessionInfo> = serde_json::from_str(&output)?;

    println!("{}", "👥 ACTIVE SESSIONS".green().bold());
    println!("{}", "─".repeat(80).dimmed());
    
    if sessions.is_empty() {
        println!("No sessions currently active.");
        return Ok(());
    }

    // Table header
    println!("{:<25} {:<8} {:<10} {:<12} {:<8} {:<8}", 
        "SESSION ID".dimmed(),
        "STATUS".dimmed(),
        "LANE".dimmed(), 
        "DURATION".dimmed(),
        "TASKS".dimmed(),
        "BEATS".dimmed()
    );
    println!("{}", "─".repeat(80).dimmed());

    for session in sessions.iter().take(10) {
        let status_color = match session.status.as_str() {
            "active" => colored::Color::Green,
            "ended" => colored::Color::Blue,
            "timed_out" => colored::Color::Red,
            _ => colored::Color::White,
        };

        let session_id = if session.id.len() > 24 {
            format!("{}...", &session.id[..21])
        } else {
            session.id.clone()
        };

        let duration_min = session.duration_seconds / 60;
        let tasks = format!("{}/{}", session.tasks_completed, session.tasks_reserved);

        println!("{:<25} {:<8} {:<10} {:<12} {:<8} {:<8}", 
            session_id.cyan(),
            session.status.color(status_color).bold(),
            session.lane.as_deref().unwrap_or("None").yellow(),
            format!("{}m", duration_min).white(),
            tasks.white(),
            session.heartbeat_count.to_string().white()
        );
    }

    if sessions.len() > 10 {
        println!("\n... and {} more sessions", sessions.len() - 10);
    }

    Ok(())
}

fn display_tasks_view() -> Result<()> {
    let args = vec!["status".to_string(), "--all".to_string(), "--format".to_string(), "json".to_string()];
    let output = run_coordination_script(args)?;
    let sessions: Vec<SessionInfo> = serde_json::from_str(&output)?;

    println!("{}", "📋 TASK COORDINATION".yellow().bold());
    println!("{}", "─".repeat(60).dimmed());

    let mut total_completed = 0;
    let mut total_reserved = 0;
    let mut tasks_by_lane: HashMap<String, (u64, u64)> = HashMap::new();

    for session in &sessions {
        total_completed += session.tasks_completed;
        total_reserved += session.tasks_reserved;

        if let Some(lane) = &session.lane {
            let entry = tasks_by_lane.entry(lane.clone()).or_insert((0, 0));
            entry.0 += session.tasks_completed;
            entry.1 += session.tasks_reserved;
        }
    }

    // Overall task metrics
    println!("Total Completed: {}", total_completed.to_string().green().bold());
    println!("Total Reserved: {}", total_reserved.to_string().yellow().bold());
    println!("Active Sessions: {}", sessions.iter().filter(|s| s.status == "active").count().to_string().cyan().bold());
    println!();

    // Tasks by lane
    if !tasks_by_lane.is_empty() {
        println!("{}", "Tasks by Lane:".white().bold());
        for (lane, (completed, reserved)) in &tasks_by_lane {
            println!("  {}: {} completed, {} reserved", 
                lane.cyan(), 
                completed.to_string().green(), 
                reserved.to_string().yellow()
            );
        }
        println!();
    }

    // Most active sessions
    let mut active_sessions: Vec<_> = sessions.iter()
        .filter(|s| s.status == "active")
        .collect();
    active_sessions.sort_by_key(|s| s.tasks_completed + s.tasks_reserved);
    active_sessions.reverse();

    if !active_sessions.is_empty() {
        println!("{}", "Most Active Sessions:".white().bold());
        for session in active_sessions.iter().take(5) {
            let session_id = if session.id.len() > 30 {
                format!("{}...", &session.id[..27])
            } else {
                session.id.clone()
            };
            
            println!("  {}: {} completed, {} reserved", 
                session_id.cyan(),
                session.tasks_completed.to_string().green(),
                session.tasks_reserved.to_string().yellow()
            );
        }
    }

    Ok(())
}

fn display_lanes_view() -> Result<()> {
    let args = vec!["status".to_string(), "--system".to_string(), "--format".to_string(), "json".to_string()];
    let output = run_coordination_script(args)?;
    let status: SystemStatus = serde_json::from_str(&output)?;

    println!("{}", "🛤️  LANE MANAGEMENT".yellow().bold());
    println!("{}", "─".repeat(50).dimmed());

    if status.lane_assignments.is_empty() {
        println!("No lane assignments currently active.");
        return Ok(());
    }

    // Visual representation of lane utilization
    let max_sessions = status.lane_assignments.values().max().unwrap_or(&0);
    
    for (lane, count) in &status.lane_assignments {
        let bar_width = 20;
        let filled = if *max_sessions > 0 {
            (*count as f64 / *max_sessions as f64 * bar_width as f64) as usize
        } else {
            0
        };
        
        let bar = format!("[{}{}]", 
            "█".repeat(filled).green(), 
            "░".repeat(bar_width - filled).dimmed()
        );
        
        println!("{:<12} {} {} sessions", 
            lane.cyan().bold(),
            bar,
            count.to_string().white().bold()
        );
    }
    println!();

    // Get sessions for lane details
    let session_args = vec!["status".to_string(), "--all".to_string(), "--format".to_string(), "json".to_string()];
    let session_output = run_coordination_script(session_args)?;
    let sessions: Vec<SessionInfo> = serde_json::from_str(&session_output)?;

    // Lane activity details
    println!("{}", "Lane Activity Details:".white().bold());
    for (lane, _) in &status.lane_assignments {
        let lane_sessions: Vec<_> = sessions.iter()
            .filter(|s| s.lane.as_deref() == Some(lane))
            .collect();
        
        let active_count = lane_sessions.iter().filter(|s| s.status == "active").count();
        let total_completed: u64 = lane_sessions.iter().map(|s| s.tasks_completed).sum();
        let total_reserved: u64 = lane_sessions.iter().map(|s| s.tasks_reserved).sum();
        
        println!("  {}: {} active, {} completed, {} reserved", 
            lane.cyan(),
            active_count.to_string().green(),
            total_completed.to_string().blue(),
            total_reserved.to_string().yellow()
        );
    }

    Ok(())
}

fn launch_web_dashboard(_port: u16) -> Result<()> {
    println!("{}", "🌐 Web Dashboard".magenta().bold());
    println!("Web dashboard functionality coming soon!");
    println!("For now, use the terminal dashboard: {}", "ci dashboard".cyan());
    Ok(())
}

fn export_dashboard_data(export_path: &str, view_mode: &str) -> Result<()> {
    println!("{}", "📊 Exporting Dashboard Data".magenta().bold());
    println!("View: {} | File: {}", view_mode.cyan(), export_path.yellow());
    println!();

    let data = match view_mode {
        "system" => {
            let args = vec!["status".to_string(), "--system".to_string(), "--format".to_string(), "json".to_string()];
            run_coordination_script(args)?
        },
        "sessions" => {
            let args = vec!["status".to_string(), "--all".to_string(), "--format".to_string(), "json".to_string()];
            run_coordination_script(args)?
        },
        "tasks" => {
            let args = vec!["status".to_string(), "--all".to_string(), "--format".to_string(), "json".to_string()];
            let output = run_coordination_script(args)?;
            let sessions: Vec<SessionInfo> = serde_json::from_str(&output)?;
            
            let total_completed: u64 = sessions.iter().map(|s| s.tasks_completed).sum();
            let total_reserved: u64 = sessions.iter().map(|s| s.tasks_reserved).sum();
            let active_sessions = sessions.iter().filter(|s| s.status == "active").count();
            
            serde_json::to_string_pretty(&serde_json::json!({
                "total_completed": total_completed,
                "total_reserved": total_reserved,
                "active_sessions": active_sessions,
                "sessions": sessions
            }))?
        },
        "lanes" => {
            let args = vec!["status".to_string(), "--system".to_string(), "--format".to_string(), "json".to_string()];
            let output = run_coordination_script(args)?;
            let status: SystemStatus = serde_json::from_str(&output)?;
            serde_json::to_string_pretty(&status.lane_assignments)?
        },
        _ => return Err(CIError::InvalidInput(format!("Unknown view mode: {}", view_mode)).into()),
    };

    fs::write(export_path, data)?;
    println!("{} Dashboard data exported to {}", "✓".green(), export_path.green());

    Ok(())
}

fn format_timestamp(timestamp: &str) -> String {
    match chrono::DateTime::parse_from_rfc3339(timestamp) {
        Ok(dt) => dt.format("%H:%M:%S").to_string(),
        Err(_) => timestamp.to_string(),
    }
}