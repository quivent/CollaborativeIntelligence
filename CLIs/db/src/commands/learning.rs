use anyhow::Result;
use crate::database::{CIDatabaseAPI, internal::LearningEntry};
use prettytable::{Row, Cell};
use crate::output::{OutputFormat, OutputFormatter, TableDisplay};
use crate::commands::LearningCommands;
use chrono::NaiveDate;

impl TableDisplay for LearningEntry {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.timestamp.format("%Y-%m-%d %H:%M").to_string()),
            Cell::new(&self.trigger_type),
            Cell::new(&self.importance.to_string()),
            Cell::new(&self.category.as_ref().unwrap_or(&"".to_string())),
            Cell::new(&self.content[..50.min(self.content.len())]),
        ])
    }
}

pub async fn handle_command(
    cmd: LearningCommands,
    api: &CIDatabaseAPI,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        LearningCommands::List { agent, limit, since } => {
            let learnings = if let Some(agent_name) = agent {
                api.get_learnings(&agent_name).await?
            } else {
                // Would need to implement get_all_learnings
                vec![]
            };
            
            let mut filtered = learnings;
            
            // Apply date filter
            if let Some(date_str) = since {
                let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?;
                let datetime = date.and_hms_opt(0, 0, 0).unwrap();
                filtered.retain(|l| l.timestamp.naive_utc() >= datetime);
            }
            
            // Apply limit
            filtered.truncate(limit as usize);
            
            match format {
                OutputFormat::Table => {
                    let mut table = OutputFormatter::table(vec![
                        "Timestamp", "Trigger", "Importance", "Category", "Content"
                    ]);
                    
                    for learning in &filtered {
                        table.add_row(learning.to_table_row());
                    }
                    
                    table.printstd();
                    println!("\nTotal learnings: {}", filtered.len());
                }
                _ => OutputFormatter::format(&filtered, format)?,
            }
        }
        
        LearningCommands::Add { agent, trigger, content, importance, category } => {
            let req = RecordLearningRequest {
                agent_name: agent.clone(),
                trigger_type: trigger,
                content: content.clone(),
                importance,
                category,
                metadata: None,
            };
            
            let learning = api.record_learning(req).await?;
            OutputFormatter::success(&format!("Learning recorded for agent '{}' with ID {}", 
                agent, learning.id.unwrap()));
            
            if matches!(format, OutputFormat::Json | OutputFormat::Yaml) {
                OutputFormatter::format(&learning, format)?;
            }
        }
        
        LearningCommands::Consolidate { agent, threshold } => {
            // Consolidation would need to be implemented
            OutputFormatter::info(&format!(
                "Learning consolidation for agent '{}' with threshold {} not yet implemented", 
                agent, threshold
            ));
        }
    }
    
    Ok(())
}