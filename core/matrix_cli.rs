// Matrix Quadrant CLI - Interactive tool for creating and managing quadrant matrices
// Part of the Collaborative Intelligence core system

use clap::{Args, Parser, Subcommand};
use colored::*;
use collaborative_intelligence_core::*;
use std::io::{self, Write};

// Simple footer component for matrix CLI
struct MatrixFooter {
    docs_visible: bool,
}

impl MatrixFooter {
    fn new() -> Self {
        MatrixFooter { docs_visible: false }
    }
    
    fn render(&self) -> String {
        let doc_status = if self.docs_visible { "📚 DOCS" } else { "📖 docs" };
        format!(
            "\n{}\n{} | {} | {} | {} {}\n{}",
            "─".repeat(80),
            "CollaborativeIntelligence".white().bold(),
            "🔧 matrix".cyan(),
            "🔐 secure".green(),
            doc_status.blue(),
            "| 'matrix help' for usage".dimmed(),
            "─".repeat(80)
        )
    }
    
    fn show_quick_help(&self) {
        println!("{}", "🔧 MATRIX SYSTEM - QUICK REFERENCE".cyan().bold());
        println!("{}", "=".repeat(60));
        println!("\n{}", "📊 Available Commands:".green().bold());
        println!("  {} - Create new matrix", "matrix create".cyan());
        println!("  {} - Add item to matrix", "matrix add".cyan());
        println!("  {} - View matrix", "matrix show".cyan());
        println!("  {} - List templates", "matrix templates".cyan());
        println!("  {} - Interactive mode", "matrix interactive".cyan());
        
        println!("\n{}", "🎯 Matrix Types:".blue().bold());
        println!("  • {} - Agent Performance Analysis", "performance".yellow());
        println!("  • {} - Task Priority (Eisenhower)", "priority".yellow());
        println!("  • {} - Capability Assessment", "capability".yellow());
        println!("  • {} - Risk Analysis", "risk".yellow());
        println!("  • {} - Knowledge Gaps", "knowledge".yellow());
        println!("  • {} - Decision Support", "decision".yellow());
        
        println!("\n{}", "💡 Quick Start:".green().bold());
        println!("  1. {} - See available templates", "matrix templates".cyan());
        println!("  2. {} - Create matrix", "matrix create --matrix-type priority --title \"My Tasks\" --x-label \"Urgency\" --y-label \"Importance\"".cyan());
        println!("  3. {} - Add items", "matrix add --id task1 --name \"Important Task\" --x=0.8 --y=0.9".cyan());
        println!("  4. {} - View results", "matrix show --detailed".cyan());
    }
}

#[derive(Parser)]
#[command(name = "matrix")]
#[command(about = "Matrix Quadrant System for Collaborative Intelligence")]
#[command(version = "1.0.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new matrix
    Create(CreateArgs),
    /// Add item to matrix
    Add(AddArgs),
    /// Show matrix visualization
    Show(ShowArgs),
    /// List available matrix templates
    Templates,
    /// Interactive matrix builder
    Interactive,
    /// Export matrix to JSON
    Export(ExportArgs),
    /// Import matrix from JSON
    Import(ImportArgs),
    /// Show help and usage information
    Help,
}

#[derive(Args)]
struct CreateArgs {
    /// Matrix type (performance, priority, capability, risk, knowledge)
    #[arg(short, long)]
    matrix_type: String,
    /// Matrix title
    #[arg(short, long)]
    title: String,
    /// X-axis label
    #[arg(short, long)]
    x_label: String,
    /// Y-axis label
    #[arg(short, long)]
    y_label: String,
    /// Output file path
    #[arg(short, long, default_value = "matrix.json")]
    output: String,
}

#[derive(Args)]
struct AddArgs {
    /// Matrix file path
    #[arg(short, long, default_value = "matrix.json")]
    file: String,
    /// Item ID
    #[arg(short, long)]
    id: String,
    /// Item name
    #[arg(short, long)]
    name: String,
    /// X position (-1.0 to 1.0)
    #[arg(short, long)]
    x: f64,
    /// Y position (-1.0 to 1.0)
    #[arg(short, long)]
    y: f64,
    /// Optional description
    #[arg(short, long)]
    description: Option<String>,
    /// Optional score
    #[arg(short, long)]
    score: Option<f64>,
}

#[derive(Args)]
struct ShowArgs {
    /// Matrix file path
    #[arg(short, long, default_value = "matrix.json")]
    file: String,
    /// Show detailed view
    #[arg(short, long)]
    detailed: bool,
}

#[derive(Args)]
struct ExportArgs {
    /// Matrix file path
    #[arg(short, long, default_value = "matrix.json")]
    file: String,
    /// Output file
    #[arg(short, long)]
    output: String,
}

#[derive(Args)]
struct ImportArgs {
    /// JSON file path
    #[arg(short, long)]
    file: String,
    /// Output matrix file
    #[arg(short, long, default_value = "matrix.json")]
    output: String,
}

fn main() {
    let cli = Cli::parse();
    let footer = MatrixFooter::new();

    let result = match cli.command {
        Commands::Create(args) => create_matrix(args),
        Commands::Add(args) => add_item(args),
        Commands::Show(args) => show_matrix(args),
        Commands::Templates => show_templates(),
        Commands::Interactive => interactive_mode(),
        Commands::Export(args) => export_matrix(args),
        Commands::Import(args) => import_matrix(args),
        Commands::Help => {
            footer.show_quick_help();
            Ok(())
        }
    };

    // Show footer for all commands
    println!("{}", footer.render());
    
    if let Err(e) = result {
        eprintln!("❌ Error: {}", e);
        std::process::exit(1);
    }
}

fn create_matrix(args: CreateArgs) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🔧 Creating new matrix...".cyan().bold());
    
    let matrix_type = match args.matrix_type.to_lowercase().as_str() {
        "performance" => QuadrantType::Performance,
        "priority" => QuadrantType::Priority,
        "capability" => QuadrantType::Capability,
        "risk" => QuadrantType::Risk,
        "knowledge" => QuadrantType::Knowledge,
        "decision" => QuadrantType::Decision,
        _ => {
            eprintln!("❌ Invalid matrix type. Use: performance, priority, capability, risk, knowledge, decision");
            return Err("Invalid matrix type".into());
        }
    };

    let matrix = QuadrantMatrix::new(
        matrix_type,
        args.title,
        args.x_label,
        args.y_label,
    );

    match save_matrix(&matrix, &args.output) {
        Ok(_) => {
            println!("✅ Matrix created and saved to {}", args.output.green());
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ Failed to save matrix: {}", e);
            Err(e)
        }
    }
}

fn add_item(args: AddArgs) -> Result<(), Box<dyn std::error::Error>> {
    let mut matrix = load_matrix(&args.file)?;

    let mut item = MatrixItem::new(args.id, args.name, args.x, args.y);
    
    if let Some(desc) = args.description {
        item = item.with_description(desc);
    }
    
    if let Some(score) = args.score {
        item = item.with_score(score);
    }

    println!("📍 Adding item '{}' at position ({}, {})", item.name.yellow(), args.x, args.y);
    println!("🎯 Quadrant: {}", item.position.get_quadrant_name(&matrix.matrix_type).green());
    
    matrix.add_item(item);

    save_matrix(&matrix, &args.file)?;
    println!("✅ Item added successfully");
    Ok(())
}

fn show_matrix(args: ShowArgs) -> Result<(), Box<dyn std::error::Error>> {
    let matrix = load_matrix(&args.file)?;

    println!("{}", matrix.render_ascii());

    if args.detailed {
        println!("\n{}", "📋 DETAILED ANALYSIS".cyan().bold());
        println!("{}", "=".repeat(60));
        
        for quadrant in 1..=4 {
            let items = matrix.get_items_in_quadrant(quadrant);
            if !items.is_empty() {
                println!("\n{} Quadrant {}: {}", 
                    "🎯".yellow(), 
                    quadrant, 
                    QuadrantPosition::new(
                        if quadrant == 1 || quadrant == 4 { 0.5 } else { -0.5 },
                        if quadrant == 1 || quadrant == 2 { 0.5 } else { -0.5 }
                    ).get_quadrant_name(&matrix.matrix_type).green()
                );
                
                for item in items {
                    println!("  • {} ({})", item.name.white().bold(), item.id.dimmed());
                    if let Some(desc) = &item.description {
                        println!("    {}", desc.dimmed());
                    }
                    if let Some(score) = item.score {
                        println!("    Score: {}", format!("{:.1}", score).cyan());
                    }
                    println!("    Position: ({:.2}, {:.2})", item.position.x, item.position.y);
                }
            }
        }
    }
    Ok(())
}

fn show_templates() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "📋 AVAILABLE MATRIX TEMPLATES".cyan().bold());
    println!("{}", "=".repeat(60));
    
    let templates = vec![
        ("performance", "Agent Performance Analysis", "Task Complexity", "Performance Level"),
        ("priority", "Task Priority Matrix (Eisenhower)", "Urgency", "Importance"),
        ("capability", "Agent Capability Assessment", "Learning Potential", "Current Skill"),
        ("risk", "Risk Assessment Matrix", "Probability", "Impact"),
        ("knowledge", "Knowledge Gap Analysis", "Criticality", "Knowledge Status"),
        ("decision", "Decision Support Matrix", "Time Pressure", "Complexity"),
    ];

    for (name, title, x_axis, y_axis) in templates {
        println!("\n🎯 {}", name.yellow().bold());
        println!("   Title: {}", title);
        println!("   X-Axis: {} | Y-Axis: {}", x_axis.green(), y_axis.green());
        println!("   Usage: {} create -t {} -T \"{}\" -x \"{}\" -y \"{}\"", 
            "matrix".cyan(), name, title, x_axis, y_axis);
    }
    Ok(())
}

fn interactive_mode() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🎮 INTERACTIVE MATRIX BUILDER".cyan().bold());
    println!("{}", "=".repeat(60));
    
    // Select matrix type
    println!("\n📋 Select matrix type:");
    println!("1. Performance Analysis");
    println!("2. Priority Matrix (Eisenhower)");
    println!("3. Capability Assessment");
    println!("4. Risk Assessment");
    println!("5. Knowledge Gap Analysis");
    println!("6. Decision Support");
    
    print!("Enter choice (1-6): ");
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    
    let mut matrix = match input.trim() {
        "1" => MatrixTemplates::agent_performance_matrix(),
        "2" => MatrixTemplates::priority_matrix(),
        "3" => MatrixTemplates::capability_matrix(),
        "4" => MatrixTemplates::risk_matrix(),
        "5" => MatrixTemplates::knowledge_matrix(),
        "6" => QuadrantMatrix::new(
            QuadrantType::Decision,
            "Decision Support Matrix".to_string(),
            "Time Pressure →".to_string(),
            "Complexity ↑".to_string(),
        ),
        _ => {
            eprintln!("Invalid choice");
            return;
        }
    };

    // Add items interactively
    loop {
        println!("\n{}", matrix.render_ascii());
        
        println!("\n🔧 Options:");
        println!("1. Add item");
        println!("2. Save matrix");
        println!("3. Exit");
        
        print!("Enter choice: ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                // Add item
                print!("Item ID: ");
                io::stdout().flush().unwrap();
                let mut id = String::new();
                io::stdin().read_line(&mut id).unwrap();
                
                print!("Item name: ");
                io::stdout().flush().unwrap();
                let mut name = String::new();
                io::stdin().read_line(&mut name).unwrap();
                
                print!("X position (-1.0 to 1.0): ");
                io::stdout().flush().unwrap();
                let mut x_str = String::new();
                io::stdin().read_line(&mut x_str).unwrap();
                let x: f64 = x_str.trim().parse().unwrap_or(0.0);
                
                print!("Y position (-1.0 to 1.0): ");
                io::stdout().flush().unwrap();
                let mut y_str = String::new();
                io::stdin().read_line(&mut y_str).unwrap();
                let y: f64 = y_str.trim().parse().unwrap_or(0.0);
                
                let item = MatrixItem::new(id.trim().to_string(), name.trim().to_string(), x, y);
                matrix.add_item(item);
                
                println!("✅ Item added");
            },
            "2" => {
                print!("Save as (filename): ");
                io::stdout().flush().unwrap();
                let mut filename = String::new();
                io::stdin().read_line(&mut filename).unwrap();
                
                match save_matrix(&matrix, filename.trim()) {
                    Ok(_) => println!("✅ Matrix saved"),
                    Err(e) => eprintln!("❌ Save failed: {}", e),
                }
            },
            "3" => break,
            _ => println!("Invalid choice"),
        }
    }
}

fn export_matrix(args: ExportArgs) {
    let matrix = match load_matrix(&args.file) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Failed to load matrix: {}", e);
            return;
        }
    };

    match matrix.export_to_json() {
        Ok(json) => {
            match std::fs::write(&args.output, json) {
                Ok(_) => println!("✅ Matrix exported to {}", args.output.green()),
                Err(e) => eprintln!("❌ Failed to write file: {}", e),
            }
        },
        Err(e) => eprintln!("❌ Failed to serialize matrix: {}", e),
    }
}

fn import_matrix(args: ImportArgs) {
    let json = match std::fs::read_to_string(&args.file) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("❌ Failed to read file: {}", e);
            return;
        }
    };

    let matrix = match QuadrantMatrix::import_from_json(&json) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("❌ Failed to parse JSON: {}", e);
            return;
        }
    };

    match save_matrix(&matrix, &args.output) {
        Ok(_) => println!("✅ Matrix imported to {}", args.output.green()),
        Err(e) => eprintln!("❌ Failed to save matrix: {}", e),
    }
}

fn load_matrix(path: &str) -> Result<QuadrantMatrix, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let matrix = serde_json::from_str(&content)?;
    Ok(matrix)
}

fn save_matrix(matrix: &QuadrantMatrix, path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string_pretty(matrix)?;
    std::fs::write(path, json)?;
    Ok(())
}