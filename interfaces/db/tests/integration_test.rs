use std::process::Command;
use tempfile::TempDir;

#[test]
fn test_db_help() {
    let output = Command::new("./target/release/db")
        .arg("--help")
        .output()
        .expect("Failed to execute db");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("A command-line interface for managing the CollaborativeIntelligence database system"));
    assert!(stdout.contains("Commands:"));
}

#[test]
fn test_db_without_args_shows_help() {
    let output = Command::new("./target/release/db")
        .output()
        .expect("Failed to execute db");
    
    // Should show help when no args provided
    let stderr = String::from_utf8(output.stderr).unwrap();
    assert!(stderr.contains("CollaborativeIntelligence Database CLI"));
}

#[test]
fn test_agent_list() {
    let temp_dir = TempDir::new().unwrap();
    
    let output = Command::new("./target/release/db")
        .arg("--data-dir")
        .arg(temp_dir.path())
        .arg("agent")
        .arg("list")
        .output()
        .expect("Failed to execute db");
    
    // Should succeed even with empty database
    assert!(output.status.success());
}

#[test]
fn test_admin_health() {
    let temp_dir = TempDir::new().unwrap();
    
    let output = Command::new("./target/release/db")
        .arg("--data-dir")
        .arg(temp_dir.path())
        .arg("admin")
        .arg("health")
        .output()
        .expect("Failed to execute db");
    
    // Should show health status
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Database health check (mock)"));
    assert!(stdout.contains("Status: healthy"));
}

#[test]
fn test_output_formats() {
    let temp_dir = TempDir::new().unwrap();
    
    // Test JSON format
    let output = Command::new("./target/release/db")
        .arg("--data-dir")
        .arg(temp_dir.path())
        .arg("--format")
        .arg("json")
        .arg("agent")
        .arg("list")
        .output()
        .expect("Failed to execute db");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    // Empty array is valid JSON
    assert!(stdout.contains("[") || stdout.contains("[]"));
}

#[test]
fn test_agent_creation() {
    let temp_dir = TempDir::new().unwrap();
    
    // Create agent
    let output = Command::new("./target/release/db")
        .arg("--data-dir")
        .arg(temp_dir.path())
        .arg("agent")
        .arg("create")
        .arg("TestAgent")
        .arg("--agent-type")
        .arg("test")
        .output()
        .expect("Failed to execute db");
    
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("created"));
}

#[test]
fn test_query_execution() {
    let temp_dir = TempDir::new().unwrap();
    
    // Initialize database first
    Command::new("./target/release/db")
        .arg("--data-dir")
        .arg(temp_dir.path())
        .arg("admin")
        .arg("init")
        .output()
        .expect("Failed to execute db");
    
    // Execute a simple query
    let output = Command::new("./target/release/db")
        .arg("--data-dir")
        .arg(temp_dir.path())
        .arg("query")
        .arg("SELECT 1 as test")
        .output()
        .expect("Failed to execute db");
    
    assert!(output.status.success());
}