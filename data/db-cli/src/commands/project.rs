use anyhow::Result;
use crate::database::{CIDatabaseAPI, projects::Project};
use prettytable::{Row, Cell};
use crate::output::{OutputFormat, OutputFormatter, TableDisplay};
use crate::commands::ProjectCommands;

impl TableDisplay for Project {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.name),
            Cell::new(&self.project_type.as_ref().unwrap_or(&"general".to_string())),
            Cell::new(&self.status),
            Cell::new(&self.path),
            Cell::new(&self.last_accessed
                .map(|d| d.format("%Y-%m-%d").to_string())
                .unwrap_or_default()),
        ])
    }
}

pub async fn handle_command(
    cmd: ProjectCommands,
    api: &CIDatabaseAPI,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        ProjectCommands::List { status, sort } => {
            let mut projects = api.list_projects().await?;
            
            // Apply filter
            if let Some(status_filter) = status {
                projects.retain(|p| p.status == status_filter);
            }
            
            // Apply sort
            match sort.as_str() {
                "name" => projects.sort_by(|a, b| a.name.cmp(&b.name)),
                "created" => projects.sort_by(|a, b| a.created_at.cmp(&b.created_at)),
                "last_accessed" => projects.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed)),
                _ => {}
            }
            
            match format {
                OutputFormat::Table => {
                    let mut table = OutputFormatter::table(vec![
                        "Name", "Type", "Status", "Path", "Last Accessed"
                    ]);
                    
                    for project in &projects {
                        table.add_row(project.to_table_row());
                    }
                    
                    table.printstd();
                    println!("\nTotal projects: {}", projects.len());
                }
                _ => OutputFormatter::format(&projects, format)?,
            }
        }
        
        ProjectCommands::Show { name } => {
            let project = api.get_project(&name).await?;
            
            match format {
                OutputFormat::Table => {
                    println!("Project: {}", project.name);
                    println!("Path: {}", project.path);
                    println!("Type: {}", project.project_type.unwrap_or_default());
                    println!("Status: {}", project.status);
                    println!("Created: {}", project.created_at);
                    println!("Updated: {}", project.updated_at);
                    if let Some(last_accessed) = project.last_accessed {
                        println!("Last Accessed: {}", last_accessed);
                    }
                    if let Some(ci_version) = &project.ci_version {
                        println!("CI Version: {}", ci_version);
                    }
                }
                _ => OutputFormatter::format(&project, format)?,
            }
        }
        
        ProjectCommands::Register { name, path, project_type } => {
            let req = RegisterProjectRequest {
                name: name.clone(),
                path,
                project_type,
                ci_version: None,
                metadata: None,
                config: None,
            };
            
            let project = api.register_project(req).await?;
            OutputFormatter::success(&format!("Project '{}' registered with ID {}", 
                name, project.id.unwrap()));
            
            if matches!(format, OutputFormat::Json | OutputFormat::Yaml) {
                OutputFormatter::format(&project, format)?;
            }
        }
        
        ProjectCommands::Update { name, status, ci_version } => {
            // Update would need to be implemented
            OutputFormatter::info(&format!("Project update for '{}' not yet implemented", name));
            
            if let Some(new_status) = status {
                OutputFormatter::info(&format!("Would update status to '{}'", new_status));
            }
            
            if let Some(new_version) = ci_version {
                OutputFormatter::info(&format!("Would update CI version to '{}'", new_version));
            }
        }
    }
    
    Ok(())
}