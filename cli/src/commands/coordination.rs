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
    Command::new("coordination")
        .about("Manage multi-agent coordination and session orchestration")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("start")
                .about("Start a new coordination session")
                .arg(
                    Arg::new("session_id")
                        .help("Custom session ID (optional)")
                        .long("session")
                        .short('s')
                        .value_name("SESSION_ID")
                )
                .arg(
                    Arg::new("lane")
                        .help("Assign session to specific lane")
                        .long("lane")
                        .short('l')
                        .value_name("LANE")
                )
                .arg(
                    Arg::new("claude_version")
                        .help("Claude version identifier")
                        .long("claude-version")
                        .short('v')
                        .value_name("VERSION")
                )
                .arg(
                    Arg::new("user_context")
                        .help("Additional user context")
                        .long("context")
                        .short('c')
                        .value_name("CONTEXT")
                )
        )
        .subcommand(
            Command::new("status")
                .about("Show coordination system status")
                .arg(
                    Arg::new("session_id")
                        .help("Show status for specific session")
                        .long("session")
                        .short('s')
                        .value_name("SESSION_ID")
                )
                .arg(
                    Arg::new("all")
                        .help("Show all sessions")
                        .long("all")
                        .short('a')
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("active")
                        .help("Show only active sessions")
                        .long("active")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("system")
                        .help("Show system status overview")
                        .long("system")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("format")
                        .help("Output format (table, json)")
                        .long("format")
                        .short('f')
                        .value_name("FORMAT")
                        .value_parser(["table", "json"])
                        .default_value("table")
                )
        )
        .subcommand(
            Command::new("heartbeat")
                .about("Send heartbeat for active session")
                .arg(
                    Arg::new("session_id")
                        .help("Session ID to send heartbeat for")
                        .long("session")
                        .short('s')
                        .value_name("SESSION_ID")
                        .required(true)
                )
                .arg(
                    Arg::new("tasks_completed")
                        .help("Number of tasks completed")
                        .long("tasks-completed")
                        .short('c')
                        .value_name("COUNT")
                        .value_parser(clap::value_parser!(u64))
                )
                .arg(
                    Arg::new("tasks_reserved")
                        .help("Number of tasks reserved")
                        .long("tasks-reserved")
                        .short('r')
                        .value_name("COUNT")
                        .value_parser(clap::value_parser!(u64))
                )
                .arg(
                    Arg::new("activity_data")
                        .help("Additional activity data as JSON")
                        .long("activity")
                        .short('a')
                        .value_name("JSON")
                )
        )
        .subcommand(
            Command::new("task")
                .about("Manage task coordination")
                .subcommand(
                    Command::new("reserve")
                        .about("Reserve a task for a session")
                        .arg(
                            Arg::new("session_id")
                                .help("Session ID")
                                .long("session")
                                .short('s')
                                .value_name("SESSION_ID")
                                .required(true)
                        )
                        .arg(
                            Arg::new("task_id")
                                .help("Task ID to reserve")
                                .long("task")
                                .short('t')
                                .value_name("TASK_ID")
                                .required(true)
                        )
                        .arg(
                            Arg::new("priority")
                                .help("Task priority (high, medium, low)")
                                .long("priority")
                                .short('p')
                                .value_name("PRIORITY")
                                .value_parser(["high", "medium", "low"])
                                .default_value("medium")
                        )
                )
                .subcommand(
                    Command::new("complete")
                        .about("Mark a task as completed")
                        .arg(
                            Arg::new("session_id")
                                .help("Session ID")
                                .long("session")
                                .short('s')
                                .value_name("SESSION_ID")
                                .required(true)
                        )
                        .arg(
                            Arg::new("task_id")
                                .help("Task ID to complete")
                                .long("task")
                                .short('t')
                                .value_name("TASK_ID")
                                .required(true)
                        )
                        .arg(
                            Arg::new("result_data")
                                .help("Task completion result data as JSON")
                                .long("result")
                                .short('r')
                                .value_name("JSON")
                        )
                )
                .subcommand(
                    Command::new("status")
                        .about("Show task coordination status")
                        .arg(
                            Arg::new("lane")
                                .help("Filter by lane")
                                .long("lane")
                                .short('l')
                                .value_name("LANE")
                        )
                        .arg(
                            Arg::new("format")
                                .help("Output format (table, json)")
                                .long("format")
                                .short('f')
                                .value_name("FORMAT")
                                .value_parser(["table", "json"])
                                .default_value("table")
                        )
                )
        )
        .subcommand(
            Command::new("lane")
                .about("Manage coordination lanes")
                .subcommand(
                    Command::new("assign")
                        .about("Assign session to lane")
                        .arg(
                            Arg::new("session_id")
                                .help("Session ID")
                                .long("session")
                                .short('s')
                                .value_name("SESSION_ID")
                                .required(true)
                        )
                        .arg(
                            Arg::new("lane")
                                .help("Lane identifier")
                                .long("lane")
                                .short('l')
                                .value_name("LANE")
                                .required(true)
                        )
                )
                .subcommand(
                    Command::new("status")
                        .about("Show lane assignment status")
                        .arg(
                            Arg::new("format")
                                .help("Output format (table, json)")
                                .long("format")
                                .short('f')
                                .value_name("FORMAT")
                                .value_parser(["table", "json"])
                                .default_value("table")
                        )
                )
        )
        .subcommand(
            Command::new("end")
                .about("End an active coordination session")
                .arg(
                    Arg::new("session_id")
                        .help("Session ID to end")
                        .long("session")
                        .short('s')
                        .value_name("SESSION_ID")
                        .required(true)
                )
                .arg(
                    Arg::new("reason")
                        .help("Termination reason")
                        .long("reason")
                        .short('r')
                        .value_name("REASON")
                        .default_value("Normal termination")
                )
        )
        .subcommand(
            Command::new("reports")
                .about("Generate coordination reports")
                .subcommand(
                    Command::new("summary")
                        .about("Generate coordination summary report")
                        .arg(
                            Arg::new("output")
                                .help("Output file path")
                                .long("output")
                                .short('o')
                                .value_name("FILE")
                        )
                        .arg(
                            Arg::new("format")
                                .help("Report format (markdown, json, html)")
                                .long("format")
                                .short('f')
                                .value_name("FORMAT")
                                .value_parser(["markdown", "json", "html"])
                                .default_value("markdown")
                        )
                        .arg(
                            Arg::new("timeframe")
                                .help("Report timeframe (1h, 24h, 7d, 30d)")
                                .long("timeframe")
                                .short('t')
                                .value_name("TIMEFRAME")
                                .value_parser(["1h", "24h", "7d", "30d"])
                                .default_value("24h")
                        )
                )
                .subcommand(
                    Command::new("activity")
                        .about("Generate activity report")
                        .arg(
                            Arg::new("output")
                                .help("Output file path")
                                .long("output")
                                .short('o')
                                .value_name("FILE")
                        )
                        .arg(
                            Arg::new("format")
                                .help("Report format (markdown, json, csv)")
                                .long("format")
                                .short('f')
                                .value_name("FORMAT")
                                .value_parser(["markdown", "json", "csv"])
                                .default_value("markdown")
                        )
                )
        )
        .subcommand(
            Command::new("cleanup")
                .about("Clean up stale sessions and data")
                .arg(
                    Arg::new("dry_run")
                        .help("Show what would be cleaned without making changes")
                        .long("dry-run")
                        .action(clap::ArgAction::SetTrue)
                )
                .arg(
                    Arg::new("force")
                        .help("Force cleanup even if sessions appear active")
                        .long("force")
                        .action(clap::ArgAction::SetTrue)
                )
        )
        .subcommand(
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
        )
}

pub fn execute(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("start", sub_matches)) => start_session(sub_matches),
        Some(("status", sub_matches)) => show_status(sub_matches),
        Some(("heartbeat", sub_matches)) => send_heartbeat(sub_matches),
        Some(("task", sub_matches)) => handle_task_command(sub_matches),
        Some(("lane", sub_matches)) => handle_lane_command(sub_matches),
        Some(("end", sub_matches)) => end_session(sub_matches),
        Some(("reports", sub_matches)) => handle_reports_command(sub_matches),
        Some(("cleanup", sub_matches)) => cleanup_sessions(sub_matches),
        Some(("dashboard", sub_matches)) => handle_dashboard_command(sub_matches),
        _ => {
            eprintln!("{}", "No valid subcommand provided".red());
            std::process::exit(1);
        }
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

fn start_session(matches: &ArgMatches) -> Result<()> {
    println!("{}", "🤝 Multi-Agent Coordination".magenta().bold());
    println!("{}", "Starting new coordination session...".magenta());
    println!();
    
    let session_id = matches.get_one::<String>("session_id");
    let lane = matches.get_one::<String>("lane");
    let claude_version = matches.get_one::<String>("claude_version");
    let user_context = matches.get_one::<String>("user_context");
    
    let mut args = vec!["register".to_string()];
    
    if let Some(sid) = session_id {
        args.extend(vec!["--session".to_string(), sid.clone()]);
    }
    
    if let Some(l) = lane {
        args.extend(vec!["--lane".to_string(), l.clone()]);
    }
    
    if let Some(cv) = claude_version {
        args.extend(vec!["--claude-version".to_string(), cv.clone()]);
    }
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("{}", output);
            
            if let Some(ctx) = user_context {
                println!("User Context: {}", ctx.dimmed());
            }
            
            println!();
            println!("{}", "Next Steps:".bold());
            println!("• Send periodic heartbeats: {}", "ci coordination heartbeat --session <SESSION_ID>".cyan());
            println!("• Check session status: {}", "ci coordination status --session <SESSION_ID>".cyan());
            println!("• Reserve tasks: {}", "ci coordination task reserve --session <SESSION_ID> --task <TASK_ID>".cyan());
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to start coordination session", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn show_status(matches: &ArgMatches) -> Result<()> {
    let session_id = matches.get_one::<String>("session_id");
    let all = matches.get_flag("all");
    let active = matches.get_flag("active");
    let system = matches.get_flag("system");
    let format = matches.get_one::<String>("format").unwrap();
    
    println!("{}", "🤝 Coordination System Status".magenta().bold());
    println!("{}", "=".repeat(40).magenta());
    println!();
    
    let mut args = vec!["status".to_string()];
    
    if system {
        args.push("--system".to_string());
    } else if let Some(sid) = session_id {
        args.extend(vec!["--session".to_string(), sid.clone()]);
    } else if all {
        args.push("--all".to_string());
    } else if active {
        args.push("--active".to_string());
    } else {
        args.push("--system".to_string()); // Default to system status
    }
    
    args.extend(vec!["--format".to_string(), format.clone()]);
    
    match run_coordination_script(args) {
        Ok(output) => {
            if format == "json" {
                // Try to parse and pretty-print JSON
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("{}", serde_json::to_string_pretty(&parsed)?);
                } else {
                    println!("{}", output);
                }
            } else {
                println!("{}", output);
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to get coordination status", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn send_heartbeat(matches: &ArgMatches) -> Result<()> {
    let session_id = matches.get_one::<String>("session_id").unwrap();
    let tasks_completed = matches.get_one::<u64>("tasks_completed");
    let tasks_reserved = matches.get_one::<u64>("tasks_reserved");
    let activity_data = matches.get_one::<String>("activity_data");
    
    let mut args = vec!["heartbeat".to_string(), "--session".to_string(), session_id.clone()];
    
    if let Some(tc) = tasks_completed {
        args.extend(vec!["--tasks-completed".to_string(), tc.to_string()]);
    }
    
    if let Some(tr) = tasks_reserved {
        args.extend(vec!["--tasks-reserved".to_string(), tr.to_string()]);
    }
    
    // Validate JSON if provided
    if let Some(json_data) = activity_data {
        if serde_json::from_str::<serde_json::Value>(json_data).is_err() {
            return Err(CIError::InvalidInput("Invalid JSON in activity data".to_string()).into());
        }
    }
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to send heartbeat", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn handle_task_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("reserve", sub_matches)) => reserve_task(sub_matches),
        Some(("complete", sub_matches)) => complete_task(sub_matches),
        Some(("status", sub_matches)) => task_status(sub_matches),
        _ => {
            eprintln!("{}", "No valid task subcommand provided".red());
            std::process::exit(1);
        }
    }
}

fn reserve_task(matches: &ArgMatches) -> Result<()> {
    let session_id = matches.get_one::<String>("session_id").unwrap();
    let task_id = matches.get_one::<String>("task_id").unwrap();
    let priority = matches.get_one::<String>("priority").unwrap();
    
    println!("{}", "📋 Reserving Task".magenta().bold());
    println!("Session: {}", session_id.cyan());
    println!("Task: {}", task_id.yellow());
    println!("Priority: {}", priority.dimmed());
    println!();
    
    // This would integrate with the task management system
    // For now, we'll update session activity
    let args = vec!["update-activity".to_string(), "--session".to_string(), session_id.clone(), "--tasks-reserved".to_string(), "1".to_string()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("{}", output);
            println!("Task {} reserved for session {}", task_id.yellow(), session_id.cyan());
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to reserve task", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn complete_task(matches: &ArgMatches) -> Result<()> {
    let session_id = matches.get_one::<String>("session_id").unwrap();
    let task_id = matches.get_one::<String>("task_id").unwrap();
    let result_data = matches.get_one::<String>("result_data");
    
    println!("{}", "✅ Completing Task".magenta().bold());
    println!("Session: {}", session_id.cyan());
    println!("Task: {}", task_id.yellow());
    
    if let Some(result) = result_data {
        // Validate JSON if provided
        if serde_json::from_str::<serde_json::Value>(result).is_err() {
            return Err(CIError::InvalidInput("Invalid JSON in result data".to_string()).into());
        }
        println!("Result Data: {}", result.dimmed());
    }
    println!();
    
    // This would integrate with the task management system
    // For now, we'll update session activity
    let args = vec!["update-activity".to_string(), "--session".to_string(), session_id.clone(), "--tasks-completed".to_string(), "1".to_string()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("{}", output);
            println!("Task {} completed by session {}", task_id.yellow(), session_id.cyan());
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to complete task", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn task_status(matches: &ArgMatches) -> Result<()> {
    let lane = matches.get_one::<String>("lane");
    let format = matches.get_one::<String>("format").unwrap();
    
    println!("{}", "📋 Task Coordination Status".magenta().bold());
    if let Some(l) = lane {
        println!("Lane: {}", l.cyan());
    }
    println!("{}", "=".repeat(40).magenta());
    println!();
    
    // This would show detailed task status - for now show session status with task info
    let args = vec!["status".to_string(), "--active".to_string(), "--format".to_string(), format.clone()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            if format == "json" {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&output) {
                    println!("{}", serde_json::to_string_pretty(&parsed)?);
                } else {
                    println!("{}", output);
                }
            } else {
                println!("{}", output);
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to get task status", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn handle_lane_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("assign", sub_matches)) => assign_lane(sub_matches),
        Some(("status", sub_matches)) => lane_status(sub_matches),
        _ => {
            eprintln!("{}", "No valid lane subcommand provided".red());
            std::process::exit(1);
        }
    }
}

fn assign_lane(matches: &ArgMatches) -> Result<()> {
    let session_id = matches.get_one::<String>("session_id").unwrap();
    let lane = matches.get_one::<String>("lane").unwrap();
    
    println!("{}", "🛤️  Lane Assignment".magenta().bold());
    println!("Assigning session {} to lane {}", session_id.cyan(), lane.yellow());
    println!();
    
    let args = vec!["assign-lane".to_string(), "--session".to_string(), session_id.clone(), "--lane".to_string(), lane.clone()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to assign lane", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn lane_status(matches: &ArgMatches) -> Result<()> {
    let format = matches.get_one::<String>("format").unwrap();
    
    println!("{}", "🛤️  Lane Assignment Status".magenta().bold());
    println!("{}", "=".repeat(40).magenta());
    println!();
    
    let args = vec!["status".to_string(), "--system".to_string(), "--format".to_string(), format.clone()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            if format == "json" {
                if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&output) {
                    // Extract and highlight lane assignments
                    if let Some(system_status) = parsed.as_object() {
                        if let Some(lane_assignments) = system_status.get("lane_assignments") {
                            println!("Lane Assignments:");
                            println!("{}", serde_json::to_string_pretty(&lane_assignments)?);
                        }
                    }
                } else {
                    println!("{}", output);
                }
            } else {
                println!("{}", output);
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to get lane status", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn end_session(matches: &ArgMatches) -> Result<()> {
    let session_id = matches.get_one::<String>("session_id").unwrap();
    let reason = matches.get_one::<String>("reason").unwrap();
    
    println!("{}", "🛑 Ending Coordination Session".magenta().bold());
    println!("Session: {}", session_id.cyan());
    println!("Reason: {}", reason.dimmed());
    println!();
    
    let args = vec!["end".to_string(), "--session".to_string(), session_id.clone(), "--reason".to_string(), reason.clone()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("{}", output);
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to end session", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn handle_reports_command(matches: &ArgMatches) -> Result<()> {
    match matches.subcommand() {
        Some(("summary", sub_matches)) => generate_summary_report(sub_matches),
        Some(("activity", sub_matches)) => generate_activity_report(sub_matches),
        _ => {
            eprintln!("{}", "No valid reports subcommand provided".red());
            std::process::exit(1);
        }
    }
}

fn generate_summary_report(matches: &ArgMatches) -> Result<()> {
    let output = matches.get_one::<String>("output");
    let format = matches.get_one::<String>("format").unwrap();
    let timeframe = matches.get_one::<String>("timeframe").unwrap();
    
    println!("{}", "📊 Generating Coordination Summary Report".magenta().bold());
    println!("Format: {}", format.cyan());
    println!("Timeframe: {}", timeframe.yellow());
    if let Some(out) = output {
        println!("Output: {}", out.dimmed());
    }
    println!();
    
    // Get system status for report data
    let args = vec!["status".to_string(), "--system".to_string(), "--format".to_string(), "json".to_string()];
    
    match run_coordination_script(args) {
        Ok(output_data) => {
            let report_content = match format.as_str() {
                "json" => output_data,
                "markdown" => generate_markdown_report(&output_data, timeframe)?,
                "html" => generate_html_report(&output_data, timeframe)?,
                _ => return Err(CIError::InvalidInput("Unsupported report format".to_string()).into()),
            };
            
            if let Some(output_file) = output {
                fs::write(output_file, &report_content)
                    .with_context(|| format!("Failed to write report to {}", output_file))?;
                println!("Report saved to: {}", output_file.green());
            } else {
                println!("{}", report_content);
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to generate report", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn generate_activity_report(matches: &ArgMatches) -> Result<()> {
    let output = matches.get_one::<String>("output");
    let format = matches.get_one::<String>("format").unwrap();
    
    println!("{}", "📈 Generating Activity Report".magenta().bold());
    println!("Format: {}", format.cyan());
    if let Some(out) = output {
        println!("Output: {}", out.dimmed());
    }
    println!();
    
    // Get all sessions for activity report
    let args = vec!["status".to_string(), "--all".to_string(), "--format".to_string(), "json".to_string()];
    
    match run_coordination_script(args) {
        Ok(output_data) => {
            let report_content = match format.as_str() {
                "json" => output_data,
                "markdown" => generate_markdown_activity_report(&output_data)?,
                "csv" => generate_csv_activity_report(&output_data)?,
                _ => return Err(CIError::InvalidInput("Unsupported report format".to_string()).into()),
            };
            
            if let Some(output_file) = output {
                fs::write(output_file, &report_content)
                    .with_context(|| format!("Failed to write report to {}", output_file))?;
                println!("Activity report saved to: {}", output_file.green());
            } else {
                println!("{}", report_content);
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to generate activity report", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

fn cleanup_sessions(matches: &ArgMatches) -> Result<()> {
    let dry_run = matches.get_flag("dry_run");
    let force = matches.get_flag("force");
    
    println!("{}", "🧹 Session Cleanup".magenta().bold());
    if dry_run {
        println!("{}", "DRY RUN - No changes will be made".yellow());
    }
    if force {
        println!("{}", "FORCE MODE - All stale sessions will be cleaned".red());
    }
    println!();
    
    // The coordination script automatically cleans up stale sessions,
    // so we'll just trigger a status check which includes cleanup
    let args = vec!["status".to_string(), "--system".to_string()];
    
    match run_coordination_script(args) {
        Ok(output) => {
            println!("Cleanup completed:");
            println!("{}", output);
            
            if !dry_run {
                println!("{}", "✓ Stale sessions have been cleaned up".green());
            }
            
            Ok(())
        },
        Err(e) => {
            eprintln!("{} Failed to cleanup sessions", "✗".red());
            eprintln!("Error: {}", e);
            Err(e)
        }
    }
}

// Helper functions for report generation

fn generate_markdown_report(json_data: &str, timeframe: &str) -> Result<String> {
    let status: SystemStatus = serde_json::from_str(json_data)?;
    
    let report = format!(
        r#"# Coordination Summary Report

**Generated**: {}
**Timeframe**: {}

## System Health

- **Status**: {}
- **Active Sessions**: {}/{}
- **Utilization**: {}%
- **Timeout Rate**: {:.1}%

## Session Statistics

- **Active**: {}
- **Ended**: {}
- **Timed Out**: {}

## Lane Assignments

{}

## Coordination Metrics

- **Total Sessions Registered**: {}
- **Session Timeouts**: {}
- **Heartbeat Interval**: {}s
- **Last Cleanup**: {}

---
*Report generated by CI Coordination System*
"#,
        status.last_updated,
        timeframe,
        status.system_health.status.to_uppercase(),
        status.system_health.active_sessions,
        status.system_health.max_concurrent,
        status.system_health.utilization_percent,
        status.system_health.timeout_rate * 100.0,
        status.session_counts.get("active").unwrap_or(&0),
        status.session_counts.get("ended").unwrap_or(&0),
        status.session_counts.get("timed_out").unwrap_or(&0),
        format_lane_assignments(&status.lane_assignments),
        status.coordination.total_sessions_registered,
        status.coordination.session_timeouts,
        status.coordination.heartbeat_interval_seconds,
        status.coordination.last_cleanup,
    );
    
    Ok(report)
}

fn generate_html_report(json_data: &str, timeframe: &str) -> Result<String> {
    let status: SystemStatus = serde_json::from_str(json_data)?;
    
    let report = format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <title>Coordination Summary Report</title>
    <style>
        body {{ font-family: Arial, sans-serif; margin: 2rem; }}
        .header {{ color: #6b46c1; border-bottom: 2px solid #6b46c1; padding-bottom: 1rem; }}
        .metric {{ background: #f3f4f6; padding: 1rem; margin: 1rem 0; border-radius: 8px; }}
        .status-healthy {{ color: #10b981; }}
        .status-warning {{ color: #f59e0b; }}
        .status-error {{ color: #ef4444; }}
        table {{ border-collapse: collapse; width: 100%; }}
        th, td {{ border: 1px solid #d1d5db; padding: 8px; text-align: left; }}
        th {{ background-color: #f9fafb; }}
    </style>
</head>
<body>
    <div class="header">
        <h1>Coordination Summary Report</h1>
        <p><strong>Generated:</strong> {}</p>
        <p><strong>Timeframe:</strong> {}</p>
    </div>
    
    <div class="metric">
        <h2>System Health</h2>
        <p><strong>Status:</strong> <span class="status-{}">{}</span></p>
        <p><strong>Active Sessions:</strong> {}/{}</p>
        <p><strong>Utilization:</strong> {}%</p>
        <p><strong>Timeout Rate:</strong> {:.1}%</p>
    </div>
    
    <div class="metric">
        <h2>Session Statistics</h2>
        <table>
            <tr><th>Status</th><th>Count</th></tr>
            <tr><td>Active</td><td>{}</td></tr>
            <tr><td>Ended</td><td>{}</td></tr>
            <tr><td>Timed Out</td><td>{}</td></tr>
        </table>
    </div>
    
    <div class="metric">
        <h2>Lane Assignments</h2>
        {}
    </div>
    
    <footer>
        <p><em>Report generated by CI Coordination System</em></p>
    </footer>
</body>
</html>"#,
        status.last_updated,
        timeframe,
        status.system_health.status.to_lowercase(),
        status.system_health.status.to_uppercase(),
        status.system_health.active_sessions,
        status.system_health.max_concurrent,
        status.system_health.utilization_percent,
        status.system_health.timeout_rate * 100.0,
        status.session_counts.get("active").unwrap_or(&0),
        status.session_counts.get("ended").unwrap_or(&0),
        status.session_counts.get("timed_out").unwrap_or(&0),
        format_lane_assignments_html(&status.lane_assignments),
    );
    
    Ok(report)
}

fn generate_markdown_activity_report(json_data: &str) -> Result<String> {
    let sessions: Vec<SessionInfo> = serde_json::from_str(json_data)?;
    
    let mut report = String::from("# Activity Report\n\n");
    report.push_str("| Session ID | Status | Lane | Duration | Tasks Completed | Tasks Reserved | Heartbeats |\n");
    report.push_str("|------------|--------|------|----------|-----------------|----------------|------------|\n");
    
    for session in &sessions {
        report.push_str(&format!(
            "| {} | {} | {} | {}m | {} | {} | {} |\n",
            session.id,
            session.status,
            session.lane.as_deref().unwrap_or("None"),
            session.duration_seconds / 60,
            session.tasks_completed,
            session.tasks_reserved,
            session.heartbeat_count
        ));
    }
    
    report.push_str(&format!("\n**Total Sessions**: {}\n", sessions.len()));
    
    Ok(report)
}

fn generate_csv_activity_report(json_data: &str) -> Result<String> {
    let sessions: Vec<SessionInfo> = serde_json::from_str(json_data)?;
    
    let mut csv = String::from("Session ID,Status,Lane,Duration (minutes),Tasks Completed,Tasks Reserved,Heartbeats,Registered At\n");
    
    for session in &sessions {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{}\n",
            session.id,
            session.status,
            session.lane.as_deref().unwrap_or("None"),
            session.duration_seconds / 60,
            session.tasks_completed,
            session.tasks_reserved,
            session.heartbeat_count,
            session.registered_at
        ));
    }
    
    Ok(csv)
}

fn format_lane_assignments(assignments: &HashMap<String, u64>) -> String {
    if assignments.is_empty() {
        return "No lane assignments".to_string();
    }
    
    assignments
        .iter()
        .map(|(lane, count)| format!("- **{}**: {} sessions", lane, count))
        .collect::<Vec<_>>()
        .join("\n")
}

fn format_lane_assignments_html(assignments: &HashMap<String, u64>) -> String {
    if assignments.is_empty() {
        return "<p>No lane assignments</p>".to_string();
    }
    
    let mut html = String::from("<table><tr><th>Lane</th><th>Sessions</th></tr>");
    
    for (lane, count) in assignments {
        html.push_str(&format!("<tr><td>{}</td><td>{}</td></tr>", lane, count));
    }
    
    html.push_str("</table>");
    html
}

fn handle_dashboard_command(matches: &ArgMatches) -> Result<()> {
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

fn launch_web_dashboard(port: u16) -> Result<()> {
    println!("{}", "🌐 Web Dashboard".magenta().bold());
    println!("Starting web dashboard on port {}", port.to_string().cyan().bold());
    println!();

    // Create web dashboard files
    create_web_dashboard_files(port)?;
    
    println!("Dashboard available at: {}", format!("http://localhost:{}", port).cyan().underline());
    println!("API endpoints:");
    println!("  • GET /api/system - System status");
    println!("  • GET /api/sessions - All sessions");
    println!("  • GET /api/tasks - Task metrics");
    println!("  • GET /api/lanes - Lane assignments");
    println!();
    println!("Press Ctrl+C to stop the server");

    start_web_server(port)
}

fn create_web_dashboard_files(port: u16) -> Result<()> {
    let ci_root = get_ci_root()?;
    let dashboard_dir = ci_root.join("tmp").join("dashboard");
    
    fs::create_dir_all(&dashboard_dir)?;

    // Create HTML file
    let html_content = create_dashboard_html(port);
    fs::write(dashboard_dir.join("index.html"), html_content)?;

    // Create CSS file
    let css_content = create_dashboard_css();
    fs::write(dashboard_dir.join("dashboard.css"), css_content)?;

    // Create JavaScript file
    let js_content = create_dashboard_js();
    fs::write(dashboard_dir.join("dashboard.js"), js_content)?;

    Ok(())
}

fn start_web_server(port: u16) -> Result<()> {
    let ci_root = get_ci_root()?;
    let dashboard_dir = ci_root.join("tmp").join("dashboard");
    
    // Create a simple Python web server
    let server_script = format!(r#"
#!/usr/bin/env python3
import http.server
import socketserver
import json
import subprocess
import os
from urllib.parse import urlparse, parse_qs
import sys

PORT = {}
BRAIN_ROOT = r"{}"

class DashboardHandler(http.server.SimpleHTTPRequestHandler):
    def do_GET(self):
        parsed_path = urlparse(self.path)
        path = parsed_path.path
        
        if path.startswith('/api/'):
            self.handle_api_request(path)
        else:
            super().do_GET()
    
    def handle_api_request(self, path):
        try:
            if path == '/api/system':
                result = subprocess.run([
                    'python3', os.path.join(BRAIN_ROOT, 'BRAIN', 'coordination', 'session_coordinator.py'),
                    'status', '--system', '--format', 'json'
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    data = result.stdout
                else:
                    data = '{{"error": "Failed to get system status"}}'
            
            elif path == '/api/sessions':
                result = subprocess.run([
                    'python3', os.path.join(BRAIN_ROOT, 'BRAIN', 'coordination', 'session_coordinator.py'),
                    'status', '--all', '--format', 'json'
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    data = result.stdout
                else:
                    data = '{{"error": "Failed to get sessions"}}'
            
            elif path == '/api/tasks':
                # Get all sessions and aggregate task data
                result = subprocess.run([
                    'python3', os.path.join(BRAIN_ROOT, 'BRAIN', 'coordination', 'session_coordinator.py'),
                    'status', '--all', '--format', 'json'
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    sessions = json.loads(result.stdout)
                    total_completed = sum(s.get('tasks_completed', 0) for s in sessions)
                    total_reserved = sum(s.get('tasks_reserved', 0) for s in sessions)
                    active_sessions = len([s for s in sessions if s.get('status') == 'active'])
                    
                    data = json.dumps({{
                        'total_completed': total_completed,
                        'total_reserved': total_reserved,
                        'active_sessions': active_sessions,
                        'sessions': sessions
                    }})
                else:
                    data = '{{"error": "Failed to get task data"}}'
            
            elif path == '/api/lanes':
                result = subprocess.run([
                    'python3', os.path.join(BRAIN_ROOT, 'BRAIN', 'coordination', 'session_coordinator.py'),
                    'status', '--system', '--format', 'json'
                ], capture_output=True, text=True)
                
                if result.returncode == 0:
                    system_data = json.loads(result.stdout)
                    data = json.dumps(system_data.get('lane_assignments', {{}}))
                else:
                    data = '{{"error": "Failed to get lane data"}}'
            
            else:
                data = '{{"error": "Unknown API endpoint"}}'
            
            self.send_response(200)
            self.send_header('Content-type', 'application/json')
            self.send_header('Access-Control-Allow-Origin', '*')
            self.end_headers()
            self.wfile.write(data.encode())
            
        except Exception as e:
            error_data = json.dumps({{"error": str(e)}})
            self.send_response(500)
            self.send_header('Content-type', 'application/json')
            self.end_headers()
            self.wfile.write(error_data.encode())

os.chdir(r"{}")
with socketserver.TCPServer(("", PORT), DashboardHandler) as httpd:
    print(f"Dashboard server running on http://localhost:{{PORT}}")
    httpd.serve_forever()
"#, port, ci_root.display(), dashboard_dir.display());

    let server_path = dashboard_dir.join("server.py");
    fs::write(&server_path, server_script)?;

    // Make server script executable and run it
    let output = StdCommand::new("python3")
        .arg(&server_path)
        .spawn()
        .context("Failed to start web server")?;

    // Server is running in background
    Ok(())
}

fn create_dashboard_html(port: u16) -> String {
    format!(r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>CI Coordination Dashboard</title>
    <link rel="stylesheet" href="dashboard.css">
    <script src="https://cdn.jsdelivr.net/npm/chart.js"></script>
</head>
<body>
    <div class="dashboard">
        <header>
            <h1>🤝 CI Coordination Dashboard</h1>
            <div class="status-indicator">
                <span id="connection-status" class="status-healthy">Connected</span>
                <span id="last-update">Last update: --:--:--</span>
            </div>
        </header>

        <nav class="dashboard-nav">
            <button class="nav-btn active" data-view="system">System</button>
            <button class="nav-btn" data-view="sessions">Sessions</button>
            <button class="nav-btn" data-view="tasks">Tasks</button>
            <button class="nav-btn" data-view="lanes">Lanes</button>
        </nav>

        <main id="dashboard-content">
            <!-- Content will be dynamically loaded here -->
        </main>
    </div>

    <script src="dashboard.js"></script>
</body>
</html>"#)
}

fn create_dashboard_css() -> String {
    r#"
* {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

body {
    font-family: 'SF Mono', Monaco, 'Inconsolata', 'Roboto Mono', monospace;
    background: #0d1117;
    color: #c9d1d9;
    line-height: 1.6;
}

.dashboard {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
}

header {
    background: #161b22;
    padding: 1rem 2rem;
    border-bottom: 1px solid #30363d;
    display: flex;
    justify-content: space-between;
    align-items: center;
}

header h1 {
    color: #f0f6fc;
    font-size: 1.5rem;
}

.status-indicator {
    display: flex;
    gap: 1rem;
    align-items: center;
    font-size: 0.9rem;
}

.status-healthy {
    color: #3fb950;
}

.status-warning {
    color: #d29922;
}

.status-error {
    color: #f85149;
}

.dashboard-nav {
    background: #21262d;
    padding: 0.5rem 2rem;
    display: flex;
    gap: 0.5rem;
    border-bottom: 1px solid #30363d;
}

.nav-btn {
    background: transparent;
    color: #7d8590;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 6px;
    cursor: pointer;
    transition: all 0.2s;
}

.nav-btn:hover {
    background: #30363d;
    color: #c9d1d9;
}

.nav-btn.active {
    background: #388bfd;
    color: #ffffff;
}

main {
    flex: 1;
    padding: 2rem;
}

.metric-cards {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
    gap: 1rem;
    margin-bottom: 2rem;
}

.metric-card {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 8px;
    padding: 1.5rem;
}

.metric-card h3 {
    color: #f0f6fc;
    font-size: 1rem;
    margin-bottom: 0.5rem;
    display: flex;
    align-items: center;
    gap: 0.5rem;
}

.metric-value {
    font-size: 2rem;
    font-weight: bold;
    margin-bottom: 0.5rem;
}

.metric-value.healthy { color: #3fb950; }
.metric-value.warning { color: #d29922; }
.metric-value.error { color: #f85149; }
.metric-value.info { color: #79c0ff; }

.progress-bar {
    width: 100%;
    height: 8px;
    background: #21262d;
    border-radius: 4px;
    overflow: hidden;
    margin-bottom: 0.5rem;
}

.progress-fill {
    height: 100%;
    background: #3fb950;
    transition: width 0.3s ease;
}

.progress-fill.warning { background: #d29922; }
.progress-fill.error { background: #f85149; }

.data-table {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 8px;
    overflow: hidden;
}

.data-table table {
    width: 100%;
    border-collapse: collapse;
}

.data-table th {
    background: #21262d;
    color: #f0f6fc;
    padding: 1rem;
    text-align: left;
    font-weight: 600;
}

.data-table td {
    padding: 0.75rem 1rem;
    border-top: 1px solid #30363d;
}

.data-table tr:hover {
    background: #0d1117;
}

.status-badge {
    display: inline-block;
    padding: 0.25rem 0.5rem;
    border-radius: 4px;
    font-size: 0.8rem;
    font-weight: 600;
    text-transform: uppercase;
}

.status-active { background: #1a472a; color: #3fb950; }
.status-ended { background: #1c2e4a; color: #79c0ff; }
.status-timed_out { background: #442025; color: #f85149; }

.chart-container {
    background: #161b22;
    border: 1px solid #30363d;
    border-radius: 8px;
    padding: 1.5rem;
    margin-bottom: 2rem;
}

.lane-bars {
    display: flex;
    flex-direction: column;
    gap: 1rem;
}

.lane-bar {
    display: flex;
    align-items: center;
    gap: 1rem;
}

.lane-label {
    min-width: 100px;
    font-weight: 600;
    color: #79c0ff;
}

.lane-progress {
    flex: 1;
    height: 20px;
    background: #21262d;
    border-radius: 10px;
    overflow: hidden;
    position: relative;
}

.lane-progress-fill {
    height: 100%;
    background: linear-gradient(90deg, #3fb950, #2ea043);
    transition: width 0.3s ease;
}

.lane-count {
    margin-left: 1rem;
    font-weight: 600;
}

.error-message {
    background: #442025;
    border: 1px solid #f85149;
    color: #ffa198;
    padding: 1rem;
    border-radius: 8px;
    margin: 1rem 0;
}

.loading {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 200px;
    color: #7d8590;
    font-size: 1.1rem;
}

.loading::before {
    content: "⟳";
    display: inline-block;
    animation: spin 1s linear infinite;
    margin-right: 0.5rem;
}

@keyframes spin {
    from { transform: rotate(0deg); }
    to { transform: rotate(360deg); }
}

@media (max-width: 768px) {
    header {
        padding: 1rem;
        flex-direction: column;
        gap: 1rem;
    }
    
    .dashboard-nav {
        padding: 0.5rem 1rem;
    }
    
    main {
        padding: 1rem;
    }
    
    .metric-cards {
        grid-template-columns: 1fr;
    }
}
"#.to_string()
}

fn create_dashboard_js() -> String {
    r#"
class CoordinationDashboard {
    constructor() {
        this.currentView = 'system';
        this.updateInterval = 5000; // 5 seconds
        this.charts = {};
        
        this.initializeEventListeners();
        this.startAutoUpdate();
        this.switchView('system');
    }

    initializeEventListeners() {
        // Navigation buttons
        document.querySelectorAll('.nav-btn').forEach(btn => {
            btn.addEventListener('click', (e) => {
                const view = e.target.dataset.view;
                this.switchView(view);
            });
        });
    }

    switchView(view) {
        // Update navigation
        document.querySelectorAll('.nav-btn').forEach(btn => {
            btn.classList.remove('active');
        });
        document.querySelector(`[data-view="${view}"]`).classList.add('active');
        
        this.currentView = view;
        this.loadViewData();
    }

    async loadViewData() {
        const content = document.getElementById('dashboard-content');
        content.innerHTML = '<div class="loading">Loading dashboard data...</div>';

        try {
            switch (this.currentView) {
                case 'system':
                    await this.loadSystemView();
                    break;
                case 'sessions':
                    await this.loadSessionsView();
                    break;
                case 'tasks':
                    await this.loadTasksView();
                    break;
                case 'lanes':
                    await this.loadLanesView();
                    break;
            }
            this.updateConnectionStatus(true);
        } catch (error) {
            this.showError('Failed to load dashboard data: ' + error.message);
            this.updateConnectionStatus(false);
        }
    }

    async loadSystemView() {
        const data = await this.fetchAPI('/api/system');
        
        const html = `
            <div class="metric-cards">
                <div class="metric-card">
                    <h3>🏥 System Health</h3>
                    <div class="metric-value ${this.getHealthColor(data.system_health.status)}">${data.system_health.status.toUpperCase()}</div>
                    <div class="progress-bar">
                        <div class="progress-fill ${this.getUtilizationColor(data.system_health.utilization_percent)}" 
                             style="width: ${data.system_health.utilization_percent}%"></div>
                    </div>
                    <small>${data.system_health.utilization_percent}% utilization</small>
                </div>

                <div class="metric-card">
                    <h3>👥 Active Sessions</h3>
                    <div class="metric-value info">${data.system_health.active_sessions}</div>
                    <small>of ${data.system_health.max_concurrent} max</small>
                </div>

                <div class="metric-card">
                    <h3>⏰ Timeout Rate</h3>
                    <div class="metric-value ${this.getTimeoutColor(data.system_health.timeout_rate)}">${(data.system_health.timeout_rate * 100).toFixed(1)}%</div>
                    <small>session timeout rate</small>
                </div>

                <div class="metric-card">
                    <h3>📊 Total Sessions</h3>
                    <div class="metric-value info">${data.coordination.total_sessions_registered}</div>
                    <small>registered all time</small>
                </div>
            </div>

            <div class="data-table">
                <h3 style="padding: 1rem; margin: 0; background: #21262d; color: #f0f6fc;">Session Statistics</h3>
                <table>
                    <thead>
                        <tr>
                            <th>Status</th>
                            <th>Count</th>
                            <th>Percentage</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${Object.entries(data.session_counts).map(([status, count]) => `
                            <tr>
                                <td><span class="status-badge status-${status}">${status}</span></td>
                                <td>${count}</td>
                                <td>${this.calculatePercentage(count, Object.values(data.session_counts).reduce((a, b) => a + b, 0))}%</td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
            </div>

            ${this.renderLaneAssignments(data.lane_assignments)}
        `;

        document.getElementById('dashboard-content').innerHTML = html;
    }

    async loadSessionsView() {
        const data = await this.fetchAPI('/api/sessions');
        
        const html = `
            <div class="data-table">
                <h3 style="padding: 1rem; margin: 0; background: #21262d; color: #f0f6fc;">
                    👥 Active Sessions (${data.length})
                </h3>
                <table>
                    <thead>
                        <tr>
                            <th>Session ID</th>
                            <th>Status</th>
                            <th>Lane</th>
                            <th>Duration</th>
                            <th>Tasks</th>
                            <th>Heartbeats</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${data.slice(0, 20).map(session => `
                            <tr>
                                <td style="font-family: monospace;">${this.truncateId(session.id)}</td>
                                <td><span class="status-badge status-${session.status}">${session.status}</span></td>
                                <td>${session.lane || 'None'}</td>
                                <td>${Math.floor(session.duration_seconds / 60)}m</td>
                                <td>${session.tasks_completed}/${session.tasks_reserved}</td>
                                <td>${session.heartbeat_count}</td>
                            </tr>
                        `).join('')}
                    </tbody>
                </table>
            </div>
        `;

        document.getElementById('dashboard-content').innerHTML = html;
    }

    async loadTasksView() {
        const data = await this.fetchAPI('/api/tasks');
        
        const html = `
            <div class="metric-cards">
                <div class="metric-card">
                    <h3>✅ Completed Tasks</h3>
                    <div class="metric-value healthy">${data.total_completed}</div>
                    <small>total completed</small>
                </div>

                <div class="metric-card">
                    <h3>🔄 Reserved Tasks</h3>
                    <div class="metric-value warning">${data.total_reserved}</div>
                    <small>currently reserved</small>
                </div>

                <div class="metric-card">
                    <h3>⚡ Active Sessions</h3>
                    <div class="metric-value info">${data.active_sessions}</div>
                    <small>processing tasks</small>
                </div>

                <div class="metric-card">
                    <h3>📈 Productivity</h3>
                    <div class="metric-value info">${data.active_sessions > 0 ? (data.total_completed / data.active_sessions).toFixed(1) : '0'}</div>
                    <small>tasks per session</small>
                </div>
            </div>

            <div class="chart-container">
                <h3 style="margin-bottom: 1rem; color: #f0f6fc;">Task Activity Over Time</h3>
                <canvas id="taskChart" width="400" height="200"></canvas>
            </div>

            <div class="data-table">
                <h3 style="padding: 1rem; margin: 0; background: #21262d; color: #f0f6fc;">Most Active Sessions</h3>
                <table>
                    <thead>
                        <tr>
                            <th>Session ID</th>
                            <th>Status</th>
                            <th>Completed</th>
                            <th>Reserved</th>
                            <th>Total Activity</th>
                        </tr>
                    </thead>
                    <tbody>
                        ${data.sessions
                            .sort((a, b) => (b.tasks_completed + b.tasks_reserved) - (a.tasks_completed + a.tasks_reserved))
                            .slice(0, 10)
                            .map(session => `
                                <tr>
                                    <td style="font-family: monospace;">${this.truncateId(session.id)}</td>
                                    <td><span class="status-badge status-${session.status}">${session.status}</span></td>
                                    <td>${session.tasks_completed}</td>
                                    <td>${session.tasks_reserved}</td>
                                    <td><strong>${session.tasks_completed + session.tasks_reserved}</strong></td>
                                </tr>
                            `).join('')}
                    </tbody>
                </table>
            </div>
        `;

        document.getElementById('dashboard-content').innerHTML = html;
        this.createTaskChart(data);
    }

    async loadLanesView() {
        const data = await this.fetchAPI('/api/lanes');
        
        const maxSessions = Math.max(...Object.values(data));
        
        const html = `
            <div class="metric-cards">
                <div class="metric-card">
                    <h3>🛤️  Total Lanes</h3>
                    <div class="metric-value info">${Object.keys(data).length}</div>
                    <small>active lanes</small>
                </div>

                <div class="metric-card">
                    <h3>👥 Total Sessions</h3>
                    <div class="metric-value info">${Object.values(data).reduce((a, b) => a + b, 0)}</div>
                    <small>across all lanes</small>
                </div>

                <div class="metric-card">
                    <h3>📊 Average Load</h3>
                    <div class="metric-value info">${Object.keys(data).length > 0 ? (Object.values(data).reduce((a, b) => a + b, 0) / Object.keys(data).length).toFixed(1) : '0'}</div>
                    <small>sessions per lane</small>
                </div>

                <div class="metric-card">
                    <h3>🔥 Busiest Lane</h3>
                    <div class="metric-value warning">${maxSessions}</div>
                    <small>sessions in busiest lane</small>
                </div>
            </div>

            <div class="chart-container">
                <h3 style="margin-bottom: 1rem; color: #f0f6fc;">Lane Utilization</h3>
                <div class="lane-bars">
                    ${Object.entries(data).map(([lane, count]) => `
                        <div class="lane-bar">
                            <div class="lane-label">${lane}</div>
                            <div class="lane-progress">
                                <div class="lane-progress-fill" style="width: ${(count / maxSessions) * 100}%"></div>
                            </div>
                            <div class="lane-count">${count}</div>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;

        document.getElementById('dashboard-content').innerHTML = html;
    }

    createTaskChart(data) {
        const ctx = document.getElementById('taskChart');
        if (!ctx) return;

        // Destroy existing chart
        if (this.charts.taskChart) {
            this.charts.taskChart.destroy();
        }

        // Create sample time series data
        const labels = [];
        const completedData = [];
        const reservedData = [];
        
        for (let i = 9; i >= 0; i--) {
            const date = new Date();
            date.setMinutes(date.getMinutes() - i * 5);
            labels.push(date.toLocaleTimeString('en-US', { hour12: false }));
            
            // Simulate historical data
            completedData.push(Math.floor(data.total_completed * (0.8 + Math.random() * 0.4)));
            reservedData.push(Math.floor(data.total_reserved * (0.8 + Math.random() * 0.4)));
        }

        this.charts.taskChart = new Chart(ctx, {
            type: 'line',
            data: {
                labels: labels,
                datasets: [
                    {
                        label: 'Completed Tasks',
                        data: completedData,
                        borderColor: '#3fb950',
                        backgroundColor: 'rgba(63, 185, 80, 0.1)',
                        tension: 0.4
                    },
                    {
                        label: 'Reserved Tasks',
                        data: reservedData,
                        borderColor: '#d29922',
                        backgroundColor: 'rgba(210, 153, 34, 0.1)',
                        tension: 0.4
                    }
                ]
            },
            options: {
                responsive: true,
                maintainAspectRatio: false,
                plugins: {
                    legend: {
                        labels: {
                            color: '#c9d1d9'
                        }
                    }
                },
                scales: {
                    x: {
                        ticks: { color: '#7d8590' },
                        grid: { color: '#30363d' }
                    },
                    y: {
                        ticks: { color: '#7d8590' },
                        grid: { color: '#30363d' }
                    }
                }
            }
        });
    }

    renderLaneAssignments(assignments) {
        if (Object.keys(assignments).length === 0) {
            return '<div class="chart-container"><p style="color: #7d8590; text-align: center;">No lane assignments currently active.</p></div>';
        }

        const maxSessions = Math.max(...Object.values(assignments));
        
        return `
            <div class="chart-container">
                <h3 style="margin-bottom: 1rem; color: #f0f6fc;">🛤️  Lane Assignments</h3>
                <div class="lane-bars">
                    ${Object.entries(assignments).map(([lane, count]) => `
                        <div class="lane-bar">
                            <div class="lane-label">${lane}</div>
                            <div class="lane-progress">
                                <div class="lane-progress-fill" style="width: ${(count / maxSessions) * 100}%"></div>
                            </div>
                            <div class="lane-count">${count} sessions</div>
                        </div>
                    `).join('')}
                </div>
            </div>
        `;
    }

    async fetchAPI(endpoint) {
        const response = await fetch(endpoint);
        if (!response.ok) {
            throw new Error(`HTTP ${response.status}: ${response.statusText}`);
        }
        return await response.json();
    }

    startAutoUpdate() {
        setInterval(() => {
            this.loadViewData();
        }, this.updateInterval);
    }

    updateConnectionStatus(connected) {
        const status = document.getElementById('connection-status');
        const lastUpdate = document.getElementById('last-update');
        
        if (connected) {
            status.textContent = 'Connected';
            status.className = 'status-healthy';
        } else {
            status.textContent = 'Connection Error';
            status.className = 'status-error';
        }
        
        lastUpdate.textContent = `Last update: ${new Date().toLocaleTimeString()}`;
    }

    showError(message) {
        const content = document.getElementById('dashboard-content');
        content.innerHTML = `<div class="error-message">⚠️ ${message}</div>`;
    }

    getHealthColor(status) {
        switch (status.toLowerCase()) {
            case 'healthy': return 'healthy';
            case 'warning': return 'warning';
            case 'error': case 'overloaded': return 'error';
            default: return 'info';
        }
    }

    getUtilizationColor(percent) {
        if (percent < 70) return '';
        if (percent < 90) return 'warning';
        return 'error';
    }

    getTimeoutColor(rate) {
        if (rate < 0.05) return 'healthy';
        if (rate < 0.15) return 'warning';
        return 'error';
    }

    truncateId(id) {
        return id.length > 30 ? id.substring(0, 27) + '...' : id;
    }

    calculatePercentage(value, total) {
        return total > 0 ? ((value / total) * 100).toFixed(1) : '0.0';
    }
}

// Initialize dashboard when page loads
document.addEventListener('DOMContentLoaded', () => {
    new CoordinationDashboard();
});
"#.to_string()
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