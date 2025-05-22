use anyhow::Result;
use crate::database::Database;
use crate::output::{OutputFormat, OutputFormatter};
use crate::commands::EntitiesCommands;
use prettytable::{Row, Cell, Table, row};

pub async fn handle_command(
    cmd: EntitiesCommands,
    db: &Database,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        EntitiesCommands::List => {
            let internal_tables = get_database_tables(db, "internal").await?;
            let projects_tables = get_database_tables(db, "projects").await?;
            
            match format {
                OutputFormat::Table => {
                    println!("Database Entities\n================");
                    
                    println!("\nInternal Database:");
                    println!("-----------------");
                    let mut table = Table::new();
                    table.set_titles(row!["Table Name", "Type", "Row Count"]);
                    
                    for (name, count) in &internal_tables {
                        table.add_row(Row::new(vec![
                            Cell::new(name),
                            Cell::new("table"),
                            Cell::new(&count.to_string()),
                        ]));
                    }
                    table.printstd();
                    
                    println!("\nProjects Database:");
                    println!("-----------------");
                    let mut table = Table::new();
                    table.set_titles(row!["Table Name", "Type", "Row Count"]);
                    
                    for (name, count) in &projects_tables {
                        table.add_row(Row::new(vec![
                            Cell::new(name),
                            Cell::new("table"),
                            Cell::new(&count.to_string()),
                        ]));
                    }
                    table.printstd();
                }
                _ => {
                    let result = serde_json::json!({
                        "internal": internal_tables,
                        "projects": projects_tables
                    });
                    OutputFormatter::format(&result, format)?;
                }
            }
        }
        
        EntitiesCommands::Attributes { entity_name } => {
            let internal_schema = get_table_schema(db, "internal", &entity_name).await.ok();
            let projects_schema = get_table_schema(db, "projects", &entity_name).await.ok();
            
            if internal_schema.is_none() && projects_schema.is_none() {
                return Err(anyhow::anyhow!("Entity '{}' not found in any database", entity_name));
            }
            
            match format {
                OutputFormat::Table => {
                    println!("Entity: {}\n=========", entity_name);
                    
                    if let Some(schema) = internal_schema {
                        println!("\nInternal Database Schema:");
                        println!("------------------------");
                        let mut table = Table::new();
                        table.set_titles(row!["Column", "Type", "Nullable", "Default", "Primary Key"]);
                        
                        for col in &schema {
                            table.add_row(Row::new(vec![
                                Cell::new(&col.name),
                                Cell::new(&col.column_type),
                                Cell::new(if col.nullable { "YES" } else { "NO" }),
                                Cell::new(&col.default_value.as_ref().unwrap_or(&"-".to_string())),
                                Cell::new(if col.is_primary { "YES" } else { "NO" }),
                            ]));
                        }
                        table.printstd();
                    }
                    
                    if let Some(schema) = projects_schema {
                        println!("\nProjects Database Schema:");
                        println!("------------------------");
                        let mut table = Table::new();
                        table.set_titles(row!["Column", "Type", "Nullable", "Default", "Primary Key"]);
                        
                        for col in &schema {
                            table.add_row(Row::new(vec![
                                Cell::new(&col.name),
                                Cell::new(&col.column_type),
                                Cell::new(if col.nullable { "YES" } else { "NO" }),
                                Cell::new(&col.default_value.as_ref().unwrap_or(&"-".to_string())),
                                Cell::new(if col.is_primary { "YES" } else { "NO" }),
                            ]));
                        }
                        table.printstd();
                    }
                }
                _ => {
                    let result = serde_json::json!({
                        "entity": entity_name,
                        "internal": internal_schema,
                        "projects": projects_schema
                    });
                    OutputFormatter::format(&result, format)?;
                }
            }
        }
    }
    
    Ok(())
}

async fn get_database_tables(db: &Database, database: &str) -> Result<Vec<(String, i64)>> {
    use std::sync::Arc;
    
    let conn = match database {
        "internal" => Arc::clone(&db.internal_conn),
        "projects" => Arc::clone(&db.projects_conn),
        _ => return Err(anyhow::anyhow!("Invalid database name")),
    };
    
    let conn = conn.lock().await;
    
    let mut stmt = conn.prepare(
        "SELECT name FROM sqlite_master WHERE type = 'table' AND name NOT LIKE 'sqlite_%'"
    )?;
    
    let tables = stmt.query_map([], |row| {
        row.get::<_, String>(0)
    })?;
    
    let mut result = Vec::new();
    for table_name in tables {
        let table_name = table_name?;
        
        // Get row count
        let count_query = format!("SELECT COUNT(*) FROM {}", table_name);
        let count: i64 = conn.query_row(&count_query, [], |row| row.get(0))?;
        
        result.push((table_name, count));
    }
    
    Ok(result)
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct ColumnSchema {
    name: String,
    column_type: String,
    nullable: bool,
    default_value: Option<String>,
    is_primary: bool,
}

async fn get_table_schema(db: &Database, database: &str, table: &str) -> Result<Vec<ColumnSchema>> {
    use std::sync::Arc;
    
    let conn = match database {
        "internal" => Arc::clone(&db.internal_conn),
        "projects" => Arc::clone(&db.projects_conn),
        _ => return Err(anyhow::anyhow!("Invalid database name")),
    };
    
    let conn = conn.lock().await;
    
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    
    let columns = stmt.query_map([], |row| {
        Ok(ColumnSchema {
            name: row.get(1)?,
            column_type: row.get(2)?,
            nullable: row.get::<_, i32>(3)? == 0,
            default_value: row.get(4)?,
            is_primary: row.get::<_, i32>(5)? == 1,
        })
    })?;
    
    let mut result = Vec::new();
    for col in columns {
        result.push(col?);
    }
    
    Ok(result)
}