use anyhow::Result;
use ci_database::{CIDatabaseAPI, internal::AddKnowledgeRequest};
use prettytable::{Row, Cell};
use crate::output::{OutputFormat, OutputFormatter, TableDisplay};
use crate::commands::KnowledgeCommands;

impl TableDisplay for ci_database::KnowledgeEntry {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.id.unwrap_or(0).to_string()),
            Cell::new(&self.knowledge_type),
            Cell::new(&self.confidence.to_string()),
            Cell::new(&self.category.as_ref().unwrap_or(&"".to_string())),
            Cell::new(&self.content[..60.min(self.content.len())]),
        ])
    }
}

pub async fn handle_command(
    cmd: KnowledgeCommands,
    api: &CIDatabaseAPI,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        KnowledgeCommands::Search { query, agent, knowledge_type } => {
            let results = api.search_knowledge(&query).await?;
            
            // Apply filters
            let filtered: Vec<_> = results.into_iter()
                .filter(|k| knowledge_type.as_ref().map_or(true, |t| &k.knowledge_type == t))
                .collect();
            
            match format {
                OutputFormat::Table => {
                    let mut table = OutputFormatter::table(vec![
                        "ID", "Type", "Confidence", "Category", "Content"
                    ]);
                    
                    for knowledge in &filtered {
                        table.add_row(knowledge.to_table_row());
                    }
                    
                    table.printstd();
                    println!("\nFound {} results", filtered.len());
                }
                _ => OutputFormatter::format(&filtered, format)?,
            }
        }
        
        KnowledgeCommands::Add { agent, knowledge_type, content, category, confidence } => {
            let req = AddKnowledgeRequest {
                agent_name: agent.clone(),
                knowledge_type,
                content: content.clone(),
                category,
                confidence,
                metadata: None,
            };
            
            let knowledge = api.add_knowledge(req).await?;
            OutputFormatter::success(&format!("Knowledge added for agent '{}' with ID {}", 
                agent, knowledge.id.unwrap()));
            
            if matches!(format, OutputFormat::Json | OutputFormat::Yaml) {
                OutputFormatter::format(&knowledge, format)?;
            }
        }
    }
    
    Ok(())
}