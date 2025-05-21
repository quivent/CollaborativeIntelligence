// ci-topologist: Portable Repository Topology Management CLI
// Zero-dependency git repository analysis and phasal commit execution

use clap::{Parser, Subcommand};
use topologist::{Topologist, TopologyAnalysis, FileCategory};
use std::collections::HashMap;

#[derive(Parser)]
#[command(name = "ci-topologist")]
#[command(about = "Intelligent repository organization and commit management")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Analyze repository state without creating files
    Analyze {
        #[arg(long, help = "Check if repository has unorganized files")]
        has_unorganized: bool,
    },
    /// Generate detailed commit plan
    Plan {
        #[arg(long, help = "Save plan to local metadata")]
        save: bool,
        #[arg(long, help = "Check if current plan is executable")]
        check: bool,
    },
    /// Execute commit phases
    Execute {
        #[arg(help = "Phase number to execute, or 'all' for complete execution")]
        phase: String,
    },
    /// Show current topology session status
    Status,
    /// Display size change tracking
    Track,
    /// Initialize topological management
    Init,
    /// Remove all topology metadata
    Clean,
    /// Export commit history summary
    Export,
}

fn main() {
    let cli = Cli::parse();
    let mut topologist = Topologist::new();

    let result = match cli.command {
        Commands::Analyze { has_unorganized } => handle_analyze(&topologist, has_unorganized),
        Commands::Plan { save, check } => handle_plan(&mut topologist, save, check),
        Commands::Execute { phase } => handle_execute(&mut topologist, phase),
        Commands::Status => handle_status(&topologist),
        Commands::Track => handle_track(&topologist),
        Commands::Init => handle_init(&mut topologist),
        Commands::Clean => handle_clean(&mut topologist),
        Commands::Export => handle_export(&topologist),
    };

    if let Err(e) = result {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn handle_analyze(topologist: &Topologist, has_unorganized: bool) -> Result<(), Box<dyn std::error::Error>> {
    let analysis = topologist.analyze_repository()?;
    
    if has_unorganized {
        // Silent check for CI/CD integration
        if analysis.repository_stats.total_files > 0 {
            std::process::exit(1); // Has unorganized files
        } else {
            std::process::exit(0); // Repository is organized
        }
    }

    print_analysis_summary(&analysis);
    Ok(())
}

fn handle_plan(topologist: &mut Topologist, save: bool, check: bool) -> Result<(), Box<dyn std::error::Error>> {
    let analysis = topologist.analyze_repository()?;
    
    if check {
        // Check if plan is executable
        if analysis.commit_phases.is_empty() {
            println!("✅ Repository is organized, no plan needed");
            return Ok(());
        } else {
            println!("📋 Plan available: {} phases ready for execution", analysis.commit_phases.len());
            return Ok(());
        }
    }

    if analysis.commit_phases.is_empty() {
        println!("✅ Repository is already organized!");
        return Ok(());
    }

    print_detailed_plan(&analysis);

    if save {
        topologist.initialize()?;
        println!("\n💾 Plan saved to .topologist/ metadata");
        println!("   Run 'ci-topologist execute all' to proceed");
    } else {
        println!("\n💡 Run with --save to create executable plan");
    }

    Ok(())
}

fn handle_execute(topologist: &mut Topologist, phase: String) -> Result<(), Box<dyn std::error::Error>> {
    let analysis = topologist.analyze_repository()?;
    
    if analysis.commit_phases.is_empty() {
        println!("✅ No phases to execute - repository is organized");
        return Ok(());
    }

    if phase == "all" {
        println!("🚀 Executing all {} phases...\n", analysis.commit_phases.len());
        
        for (i, commit_phase) in analysis.commit_phases.iter().enumerate() {
            let phase_num = i + 1;
            println!("📦 Phase {}: {}", phase_num, commit_phase.commit_message);
            
            let commit_hash = topologist.execute_phase(phase_num, &analysis.commit_phases)?;
            
            println!("   ✅ Commit: {} (+{} estimated lines)", 
                     &commit_hash[..8], commit_phase.estimated_size);
            println!("   📁 Files: {}", commit_phase.files.len());
            println!();
        }
        
        println!("🎉 All phases complete! Repository synchronized.");
        
    } else {
        let phase_num: usize = phase.parse()
            .map_err(|_| "Invalid phase number")?;
            
        if phase_num == 0 || phase_num > analysis.commit_phases.len() {
            return Err(format!("Phase {} not found. Available: 1-{}", 
                             phase_num, analysis.commit_phases.len()).into());
        }
        
        let commit_phase = &analysis.commit_phases[phase_num - 1];
        println!("📦 Executing Phase {}: {}", phase_num, commit_phase.commit_message);
        
        let commit_hash = topologist.execute_phase(phase_num, &analysis.commit_phases)?;
        
        println!("✅ Phase {} Complete: {} (+{} estimated lines)", 
                 phase_num, &commit_hash[..8], commit_phase.estimated_size);
    }

    Ok(())
}

fn handle_status(topologist: &Topologist) -> Result<(), Box<dyn std::error::Error>> {
    match topologist.get_status()? {
        Some(session) => {
            println!("📊 Topology Session Active");
            println!("   Created: {}", session.started);
            println!("   Phases completed: {}/{}", 
                     session.phases.len(), 
                     session.total_planned_phases.unwrap_or(0));
            
            if let Some(last_phase) = session.phases.last() {
                println!("   Last commit: {}", last_phase.commit_hash);
            }
        },
        None => {
            println!("📊 No active topology session");
            println!("   Run 'ci-topologist plan --save' to start");
        }
    }
    Ok(())
}

fn handle_track(topologist: &Topologist) -> Result<(), Box<dyn std::error::Error>> {
    match topologist.get_status()? {
        Some(session) => {
            println!("📈 Size Change Tracking");
            println!("   Session: {}", session.session_id);
            
            let mut total_insertions = 0;
            let mut total_files = 0;
            
            for (i, phase) in session.phases.iter().enumerate() {
                println!("   Phase {}: +{} lines, {} files", 
                         i + 1, phase.size_change, phase.files_count);
                total_insertions += phase.size_change;
                total_files += phase.files_count;
            }
            
            println!("   Total Impact: +{} lines, {} files", total_insertions, total_files);
        },
        None => {
            println!("📈 No tracking data available");
            println!("   Start a session with 'ci-topologist plan --save'");
        }
    }
    Ok(())
}

fn handle_init(topologist: &mut Topologist) -> Result<(), Box<dyn std::error::Error>> {
    topologist.initialize()?;
    println!("🎯 Topology management initialized");
    println!("   Metadata directory: .topologist/");
    println!("   Next: 'ci-topologist analyze' to get started");
    Ok(())
}

fn handle_clean(topologist: &mut Topologist) -> Result<(), Box<dyn std::error::Error>> {
    topologist.clean()?;
    println!("🧹 All topology metadata removed");
    println!("   Repository is clean of all traces");
    Ok(())
}

fn handle_export(topologist: &Topologist) -> Result<(), Box<dyn std::error::Error>> {
    match topologist.get_status()? {
        Some(session) => {
            // Export in JSON format for programmatic use
            let json = serde_json::to_string_pretty(&session)?;
            println!("{}", json);
        },
        None => {
            println!("{}");  // Empty JSON object
        }
    }
    Ok(())
}

fn print_analysis_summary(analysis: &TopologyAnalysis) {
    println!("🔍 Repository Analysis Complete");
    println!("├── {} total files detected", analysis.repository_stats.total_files);
    println!("├── {} untracked files", analysis.repository_stats.untracked_files);  
    println!("├── {} modified files", analysis.repository_stats.modified_files);
    println!("├── Estimated growth: +{} lines", analysis.repository_stats.estimated_total_size);
    println!("└── Suggested: {}-phase commit strategy", analysis.repository_stats.suggested_phases);
    
    if !analysis.commit_phases.is_empty() {
        println!("\n📂 File Categories:");
        let mut category_summary: HashMap<String, usize> = HashMap::new();
        
        for file in &analysis.category_analysis.files {
            let category_name = format!("{:?}", file.category);
            *category_summary.entry(category_name).or_insert(0) += 1;
        }
        
        for (category, count) in category_summary {
            let icon = match category.as_str() {
                "Configuration" => "⚙️",
                "Documentation" => "📚",
                "SourceCode" => "💻",
                "DevelopmentTools" => "🔧",
                "MediaAssets" => "🎨",
                _ => "📄",
            };
            println!("   {} {}: {} files", icon, category, count);
        }
        
        println!("\n💡 Run 'ci-topologist plan --save' to proceed");
    }
}

fn print_detailed_plan(analysis: &TopologyAnalysis) {
    println!("📋 Detailed Commit Plan ({} phases)", analysis.commit_phases.len());
    println!();
    
    for (i, phase) in analysis.commit_phases.iter().enumerate() {
        let phase_num = i + 1;
        let category_icon = match phase.category {
            FileCategory::Configuration => "⚙️",
            FileCategory::Documentation => "📚", 
            FileCategory::SourceCode => "💻",
            FileCategory::DevelopmentTools => "🔧",
            FileCategory::MediaAssets => "🎨",
            _ => "📄",
        };
        
        println!("Phase {}: {} {}", phase_num, category_icon, phase.commit_message);
        println!("├── Files: {} (+{} estimated lines)", phase.files.len(), phase.estimated_size);
        println!("├── Category: {:?}", phase.category);
        
        // Show first few files as examples
        let max_show = 3;
        for (j, file) in phase.files.iter().take(max_show).enumerate() {
            let prefix = if j == phase.files.len().min(max_show) - 1 { "└──" } else { "├──" };
            println!("{}   {}", prefix, file.path);
        }
        
        if phase.files.len() > max_show {
            println!("└──   ... and {} more files", phase.files.len() - max_show);
        }
        println!();
    }
    
    let total_size: usize = analysis.commit_phases.iter().map(|p| p.estimated_size).sum();
    println!("📊 Total Impact: +{} estimated lines across {} commits", total_size, analysis.commit_phases.len());
}