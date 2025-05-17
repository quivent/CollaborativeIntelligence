use prettytable::{Table, Row, Cell};
use serde::Serialize;
use colored::*;

#[derive(Debug, Clone, Copy, clap::ValueEnum)]
pub enum OutputFormat {
    Table,
    Json,
    Csv,
    Yaml,
}

pub struct OutputFormatter;

impl OutputFormatter {
    pub fn success(message: &str) {
        println!("{} {}", "✓".green(), message);
    }
    
    pub fn error(message: &str) {
        eprintln!("{} {}", "✗".red(), message);
    }
    
    pub fn info(message: &str) {
        println!("{} {}", "ℹ".blue(), message);
    }
    
    pub fn format<T: Serialize>(data: &T, format: OutputFormat) -> anyhow::Result<()> {
        match format {
            OutputFormat::Json => {
                let json = serde_json::to_string_pretty(data)?;
                println!("{}", json);
            }
            OutputFormat::Yaml => {
                let yaml = serde_yaml::to_string(data)?;
                println!("{}", yaml);
            }
            OutputFormat::Csv => {
                // CSV formatting would need custom implementation per type
                println!("CSV output not yet implemented");
            }
            OutputFormat::Table => {
                // Table formatting is handled per command
                println!("Table formatting should be handled in specific command");
            }
        }
        Ok(())
    }
    
    pub fn table(headers: Vec<&str>) -> Table {
        let mut table = Table::new();
        let header_row = Row::new(headers.into_iter()
            .map(|h| Cell::new(h).style_spec("b"))
            .collect());
        table.add_row(header_row);
        table
    }
}

pub trait TableDisplay {
    fn to_table_row(&self) -> Row;
}