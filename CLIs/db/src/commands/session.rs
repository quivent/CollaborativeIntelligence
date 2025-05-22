use anyhow::Result;
use ci_database::{CIDatabaseAPI, internal::CreateSessionRequest};
use prettytable::{Row, Cell};
use crate::output::{OutputFormat, OutputFormatter, TableDisplay};
use crate::commands::SessionCommands;

impl TableDisplay for ci_database::Session {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.id.unwrap_or(0).to_string()),
            Cell::new(&self.session_type),
            Cell::new(&self.status),
            Cell::new(&self.started_at.format("%Y-%m-%d %H:%M").to_string()),
            Cell::new(&self.ended_at
                .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_default()),
        ])
    }
}

pub async fn handle_command(
    cmd: SessionCommands,
    api: &CIDatabaseAPI,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        SessionCommands::List { agent, status, active } => {
            // Would need to implement get_sessions in API
            let sessions: Vec<ci_database::Session> = vec![];
            
            match format {
                OutputFormat::Table => {
                    let mut table = OutputFormatter::table(vec![
                        "ID", "Type", "Status", "Started", "Ended"
                    ]);
                    
                    for session in &sessions {
                        table.add_row(session.to_table_row());
                    }
                    
                    table.printstd();
                    println!("\nTotal sessions: {}", sessions.len());
                }
                _ => OutputFormatter::format(&sessions, format)?,
            }
        }
        
        SessionCommands::Create { agent, session_type } => {
            let req = CreateSessionRequest {
                agent_name: agent.clone(),
                session_type: session_type.clone(),
                metadata: None,
            };
            
            let session = api.create_session(req).await?;
            OutputFormatter::success(&format!("Session created for agent '{}' with ID {}", 
                agent, session.id.unwrap()));
            
            if matches!(format, OutputFormat::Json | OutputFormat::Yaml) {
                OutputFormatter::format(&session, format)?;
            }
        }
        
        SessionCommands::Close { id, outcomes } => {
            let outcomes_json = if let Some(outcomes_str) = outcomes {
                Some(serde_json::from_str(&outcomes_str)?)
            } else {
                None
            };
            
            api.close_session(id, outcomes_json).await?;
            OutputFormatter::success(&format!("Session {} closed", id));
        }
    }
    
    Ok(())
}