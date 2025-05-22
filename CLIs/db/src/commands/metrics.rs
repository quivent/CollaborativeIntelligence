use anyhow::Result;
use ci_database::{CIDatabaseAPI, projects::RecordMetricRequest};
use ci_database::advanced_features::AdvancedFeatures;
use prettytable::{Row, Cell};
use crate::output::{OutputFormat, OutputFormatter, TableDisplay};
use crate::commands::MetricsCommands;

impl TableDisplay for ci_database::Metric {
    fn to_table_row(&self) -> Row {
        Row::new(vec![
            Cell::new(&self.timestamp.format("%Y-%m-%d %H:%M:%S").to_string()),
            Cell::new(&self.metric_type),
            Cell::new(&self.value.to_string()),
        ])
    }
}

pub async fn handle_command(
    cmd: MetricsCommands,
    api: &CIDatabaseAPI,
    format: OutputFormat,
) -> Result<()> {
    match cmd {
        MetricsCommands::Dashboard { days } => {
            let db = ci_database::Database::new("./data").await?;
            let advanced = AdvancedFeatures::new(db);
            let report = advanced.generate_analytics(days).await?;
            
            match format {
                OutputFormat::Table => {
                    println!("Analytics Dashboard ({} days)", days);
                    println!("======================");
                    
                    println!("\nTop Agents by Activity:");
                    let mut agent_table = OutputFormatter::table(vec![
                        "Agent", "Sessions", "Avg Duration (min)"
                    ]);
                    
                    for agent in report.agent_activity.iter().take(5) {
                        agent_table.add_row(Row::new(vec![
                            Cell::new(&agent.agent_name),
                            Cell::new(&agent.session_count.to_string()),
                            Cell::new(&format!("{:.1}", agent.avg_duration_minutes)),
                        ]));
                    }
                    agent_table.printstd();
                    
                    println!("\nTop Projects by Operations:");
                    let mut project_table = OutputFormatter::table(vec![
                        "Project", "Operations", "Avg Duration (ms)"
                    ]);
                    
                    for project in report.project_activity.iter().take(5) {
                        project_table.add_row(Row::new(vec![
                            Cell::new(&project.project_name),
                            Cell::new(&project.operation_count.to_string()),
                            Cell::new(&format!("{:.0}", project.avg_duration_ms)),
                        ]));
                    }
                    project_table.printstd();
                    
                    println!("\nLearning Trends:");
                    let mut learning_table = OutputFormatter::table(vec![
                        "Date", "Learnings", "Avg Importance"
                    ]);
                    
                    for trend in report.learning_trends.iter().take(7) {
                        learning_table.add_row(Row::new(vec![
                            Cell::new(&trend.date),
                            Cell::new(&trend.count.to_string()),
                            Cell::new(&format!("{:.2}", trend.avg_importance)),
                        ]));
                    }
                    learning_table.printstd();
                }
                _ => OutputFormatter::format(&report, format)?,
            }
        }
        
        MetricsCommands::List { project, metric_type, limit } => {
            let mut metrics = api.get_metrics(&project, &metric_type).await?;
            metrics.truncate(limit as usize);
            
            match format {
                OutputFormat::Table => {
                    let mut table = OutputFormatter::table(vec![
                        "Timestamp", "Type", "Value"
                    ]);
                    
                    for metric in &metrics {
                        table.add_row(metric.to_table_row());
                    }
                    
                    table.printstd();
                    println!("\nTotal metrics: {}", metrics.len());
                }
                _ => OutputFormatter::format(&metrics, format)?,
            }
        }
        
        MetricsCommands::Add { project, metric_type, value } => {
            let req = RecordMetricRequest {
                project_name: project.clone(),
                metric_type: metric_type.clone(),
                value,
                context: None,
            };
            
            let metric = api.record_metric(req).await?;
            OutputFormatter::success(&format!("Metric recorded for project '{}' with ID {}", 
                project, metric.id.unwrap()));
            
            if matches!(format, OutputFormat::Json | OutputFormat::Yaml) {
                OutputFormatter::format(&metric, format)?;
            }
        }
        
        MetricsCommands::Report { output, report_type } => {
            OutputFormatter::info(&format!("Report generation of type '{}' not yet implemented", 
                report_type));
            
            if let Some(output_path) = output {
                OutputFormatter::info(&format!("Would save to: {}", output_path.display()));
            }
        }
    }
    
    Ok(())
}