use anyhow::Result;
use ci_database::Database;
use ci_database::advanced_features::AdvancedFeatures;
use crate::output::{OutputFormat, OutputFormatter};
use crate::commands::AdminCommands;
use colored::*;

pub async fn handle_command(
    cmd: AdminCommands,
    db: &Database,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        AdminCommands::Init { force } => {
            if force {
                OutputFormatter::info("Force initializing database...");
            }
            
            db.initialize().await?;
            db.optimize().await?;
            
            OutputFormatter::success("Database initialized successfully");
        }
        
        AdminCommands::Optimize => {
            OutputFormatter::info("Optimizing database...");
            db.optimize().await?;
            OutputFormatter::success("Database optimization complete");
        }
        
        AdminCommands::Backup { output } => {
            OutputFormatter::info(&format!("Creating backup at {}...", output.display()));
            db.backup(&output).await?;
            OutputFormatter::success(&format!("Backup created at {}", output.display()));
        }
        
        AdminCommands::Restore { input } => {
            // Restore would need to be implemented
            OutputFormatter::error(&format!("Database restore from {} not yet implemented", 
                input.display()));
        }
        
        AdminCommands::Health => {
            let advanced = AdvancedFeatures::new(db.clone());
            let health = advanced.health_check().await?;
            
            match format {
                OutputFormat::Table => {
                    println!("Health Check Report");
                    println!("==================");
                    println!("Status: {}", 
                        if health.status == "healthy" { 
                            health.status.green() 
                        } else { 
                            health.status.red() 
                        }
                    );
                    println!("Internal DB: {}", 
                        if health.internal_db_ok { "OK".green() } else { "ERROR".red() }
                    );
                    println!("Projects DB: {}", 
                        if health.projects_db_ok { "OK".green() } else { "ERROR".red() }
                    );
                    println!("Internal Size: {:.2} MB", 
                        health.internal_size_bytes as f64 / 1_048_576.0);
                    println!("Projects Size: {:.2} MB", 
                        health.projects_size_bytes as f64 / 1_048_576.0);
                    println!("Total Agents: {}", health.total_agents);
                    println!("Total Projects: {}", health.total_projects);
                    println!("Last Check: {}", health.last_check);
                }
                _ => OutputFormatter::format(&health, format)?,
            }
        }
        
        AdminCommands::Stats => {
            let advanced = AdvancedFeatures::new(db.clone());
            let health = advanced.health_check().await?;
            
            // Basic stats from health check
            let stats = serde_json::json!({
                "databases": {
                    "internal": {
                        "size_mb": health.internal_size_bytes as f64 / 1_048_576.0,
                        "status": health.internal_db_ok,
                        "total_agents": health.total_agents,
                    },
                    "projects": {
                        "size_mb": health.projects_size_bytes as f64 / 1_048_576.0,
                        "status": health.projects_db_ok,
                        "total_projects": health.total_projects,
                    }
                },
                "timestamp": health.last_check,
            });
            
            match format {
                OutputFormat::Table => {
                    println!("Database Statistics");
                    println!("==================");
                    println!();
                    println!("Internal Database:");
                    println!("  Size: {:.2} MB", health.internal_size_bytes as f64 / 1_048_576.0);
                    println!("  Agents: {}", health.total_agents);
                    println!();
                    println!("Projects Database:");
                    println!("  Size: {:.2} MB", health.projects_size_bytes as f64 / 1_048_576.0);
                    println!("  Projects: {}", health.total_projects);
                }
                _ => OutputFormatter::format(&stats, format)?,
            }
        }
        
        AdminCommands::Export { output, include_sensitive } => {
            let advanced = AdvancedFeatures::new(db.clone());
            
            OutputFormatter::info("Exporting database...");
            let export = advanced.export_to_json(include_sensitive).await?;
            
            let json = serde_json::to_string_pretty(&export)?;
            std::fs::write(&output, json)?;
            
            OutputFormatter::success(&format!("Database exported to {}", output.display()));
        }
        
        AdminCommands::Import { input } => {
            let advanced = AdvancedFeatures::new(db.clone());
            
            OutputFormatter::info("Importing data...");
            let data = std::fs::read_to_string(&input)?;
            let summary = advanced.import_from_json(&data).await?;
            
            OutputFormatter::success("Import completed");
            println!("Agents imported: {}", summary.agents_imported);
            println!("Agents skipped: {}", summary.agents_skipped);
            println!("Projects imported: {}", summary.projects_imported);
            println!("Projects skipped: {}", summary.projects_skipped);
        }
    }
    
    Ok(())
}