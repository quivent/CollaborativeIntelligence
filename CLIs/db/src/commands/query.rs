use anyhow::Result;
use ci_database::Database;
use sqlx::Row;
use crate::output::{OutputFormat, OutputFormatter};

pub async fn execute_query(
    db: &Database,
    query: &str,
    database: &str,
    format: OutputFormat,
) -> Result<()> {
    let pool = match database {
        "internal" => db.internal(),
        "projects" => db.projects(),
        _ => return Err(anyhow::anyhow!("Invalid database. Use 'internal' or 'projects'")),
    };
    
    // Execute query
    let rows = sqlx::query(query).fetch_all(pool).await?;
    
    if rows.is_empty() {
        OutputFormatter::info("No results");
        return Ok(());
    }
    
    // Get column names from the first row
    let columns: Vec<String> = if let Some(first_row) = rows.first() {
        first_row.columns()
            .iter()
            .map(|c| c.name().to_string())
            .collect()
    } else {
        vec![]
    };
    
    match format {
        OutputFormat::Table => {
            let mut table = prettytable::Table::new();
            
            // Add header
            let header_cells: Vec<_> = columns.iter()
                .map(|col| prettytable::Cell::new(col).style_spec("b"))
                .collect();
            table.add_row(prettytable::Row::new(header_cells));
            
            // Add data rows
            for row in &rows {
                let cells: Vec<_> = (0..columns.len())
                    .map(|i| {
                        let value: String = row.try_get(i)
                            .unwrap_or_else(|_| "NULL".to_string());
                        prettytable::Cell::new(&value)
                    })
                    .collect();
                table.add_row(prettytable::Row::new(cells));
            }
            
            table.printstd();
            println!("\nRows returned: {}", rows.len());
        }
        _ => {
            // Convert to JSON for other formats
            let mut json_rows = Vec::new();
            for row in &rows {
                let mut json_row = serde_json::Map::new();
                for (i, col) in columns.iter().enumerate() {
                    let value: serde_json::Value = row.try_get(i)
                        .unwrap_or(serde_json::Value::Null);
                    json_row.insert(col.clone(), value);
                }
                json_rows.push(serde_json::Value::Object(json_row));
            }
            
            OutputFormatter::format(&json_rows, format)?;
        }
    }
    
    Ok(())
}