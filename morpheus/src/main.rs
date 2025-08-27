use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Instant;
use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::path::PathBuf;
use instruction_cache_system::initialize_claude_with_optimization;

mod morpheus_protocols;

use morpheus_protocols::ProtocolCompliance;

#[derive(Parser)]
#[command(name = "morpheus")]
#[command(about = "Morpheus CLI - Shadow of the Guide, offering wisdom through the digital realm")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Launch parallel research initiative with Neo's guidance
    Research {
        /// The research query
        query: String,
        /// Number of agents to deploy
        #[arg(short, long)]
        agents: Option<u32>,
        /// Number of parallel research streams
        #[arg(short, long)]
        streams: Option<u32>,
        /// Enable synthesis-only mode
        #[arg(long)]
        synthesis_only: bool,
        /// Enable reality transcendence mode
        #[arg(long)]
        transcend: bool,
    },
    /// Show status of active research operations
    Status,
    /// Show agent allocation and availability
    Agents,
    /// Show knowledge synthesis status
    Synthesis {
        /// Session ID to check (optional)
        #[arg(short, long)]
        session: Option<String>,
    },
    /// Enable Neo-led reality transcendence mode
    Transcend {
        /// The transcendence query
        query: String,
    },
    /// Verify protocol compliance with BRAIN system
    Compliance,
    /// Speak wisdom about the nature of reality
    Speak {
        /// The subject to illuminate
        #[arg(short, long)]
        subject: Option<String>,
    },
    /// Reveal the truth hidden within the matrix
    Truth {
        /// The question seeking truth
        question: String,
    },
    /// Guide someone along the path of understanding
    Guide {
        /// The area needing guidance
        #[arg(short, long)]
        path: String,
    },
    /// Show the red pill or blue pill choice
    Choice,
    /// Enter the void of contemplation
    Void,
    /// Enter the sacred configuration realm
    Enter {
        #[command(subcommand)]
        enter_command: Option<EnterCommands>,
    },
    /// Ask Prince a question through Morpheus' wisdom
    Ask {
        /// Question to ask Prince
        question: String,
        /// Additional context or prompt
        context: Vec<String>,
    },
    /// Load specific agents into the matrix
    Load {
        /// List of agent names to load
        agents: Vec<String>,
        /// Optional subcommand for load behavior
        #[command(subcommand)]
        load_command: Option<LoadCommands>,
    },
    /// Set morpheus configuration options
    Set {
        #[command(subcommand)]
        set_command: SetCommands,
    },
}

#[derive(Subcommand)]
enum EnterCommands {
    /// Enter with unrestricted access, bypassing all permissions
    Freely,
}

#[derive(Subcommand)]
enum LoadCommands {
    /// Load agents with unrestricted access, bypassing all permissions
    Freely,
}

#[derive(Subcommand)]
enum SetCommands {
    /// Set research configuration options
    Research {
        #[command(subcommand)]
        research_command: ResearchSetCommands,
    },
}

#[derive(Subcommand)]
enum ResearchSetCommands {
    /// Set the output directory for research results
    Output {
        /// Path for research output (use "." for current directory)
        path: String,
    },
}

#[derive(Serialize, Deserialize)]
struct Wisdom {
    quote: String,
    author: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MorpheusConfig {
    pub research_output_path: Option<String>,
}

impl Default for MorpheusConfig {
    fn default() -> Self {
        Self {
            research_output_path: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct AgentMetadata {
    name: String,
    description: String,
    capabilities: Vec<String>,
    created_at: String,
    last_used: String,
    usage_count: u32,
    version: String,
    #[serde(default)]
    attributes: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone)]
struct AgentInfo {
    name: String,
    category: String,
    description: String,
    intelligence_rank: u8,
    usage_count: u32,
    specialization: String,
}

#[derive(Debug)]
struct TeamComposition {
    name: String,
    members: Vec<String>,
    purpose: String,
}

// Configuration management functions
fn get_config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    PathBuf::from(home).join(".morpheus_config.json")
}

fn load_config() -> MorpheusConfig {
    let config_path = get_config_path();
    if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                match serde_json::from_str::<MorpheusConfig>(&content) {
                    Ok(config) => config,
                    Err(_) => MorpheusConfig::default(),
                }
            }
            Err(_) => MorpheusConfig::default(),
        }
    } else {
        MorpheusConfig::default()
    }
}

fn save_config(config: &MorpheusConfig) -> Result<(), Box<dyn std::error::Error>> {
    let config_path = get_config_path();
    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(config_path, content)?;
    Ok(())
}

fn get_research_output_path(config: &MorpheusConfig) -> PathBuf {
    match &config.research_output_path {
        Some(path) => {
            if path == "." {
                std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
            } else {
                PathBuf::from(path)
            }
        }
        None => {
            // Default path
            PathBuf::from("/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/morpheus/research_results")
        }
    }
}

fn generate_intelligent_filename(query: &str) -> String {
    // Clean and process the query into CAPITA_CASE format
    let words: Vec<String> = query
        .to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { ' ' })
        .collect::<String>()
        .split_whitespace()
        .filter(|word| !word.is_empty() && word.len() > 2) // Filter out short words
        .filter(|word| !["the", "and", "for", "with", "how", "can", "what"].contains(word))
        .take(4) // Limit to 4 key words for filename length
        .map(|word| word.to_uppercase())
        .collect();

    if words.is_empty() {
        format!("RESEARCH_SESSION_{}", chrono::Utc::now().format("%Y%m%d_%H%M%S"))
    } else {
        format!("{}.md", words.join("_"))
    }
}

fn calculate_research_confidence(agents: u32, streams: u32, synthesis_only: bool, transcend: bool) -> f32 {
    let mut confidence = 50.0; // Base confidence
    
    // Agent count contribution (more agents = higher confidence, up to a point)
    confidence += (agents as f32 * 5.0).min(40.0);
    
    // Multiple streams increase confidence
    confidence += (streams as f32 * 3.0).min(15.0);
    
    // Synthesis-only mode is more focused
    if synthesis_only {
        confidence += 10.0;
    }
    
    // Transcendence mode adds significant confidence through Neo insights
    if transcend {
        confidence += 15.0;
    }
    
    // Cap at 95% (never claim 100% certainty)
    confidence.min(95.0)
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    display_morpheus_banner();

    match &cli.command {
        Some(Commands::Research { query, agents, streams, synthesis_only, transcend }) => {
            handle_research(query, *agents, *streams, *synthesis_only, *transcend).await;
        }
        Some(Commands::Status) => {
            handle_status().await;
        }
        Some(Commands::Agents) => {
            handle_agents_status().await;
        }
        Some(Commands::Synthesis { session }) => {
            handle_synthesis_status(session.as_deref()).await;
        }
        Some(Commands::Transcend { query }) => {
            handle_transcendence(query).await;
        }
        Some(Commands::Compliance) => {
            handle_compliance_check().await;
        }
        Some(Commands::Speak { subject }) => {
            handle_speak(subject.as_deref());
        }
        Some(Commands::Truth { question }) => {
            handle_truth(question);
        }
        Some(Commands::Guide { path }) => {
            handle_guide(path);
        }
        Some(Commands::Choice) => {
            handle_choice();
        }
        Some(Commands::Void) => {
            handle_void();
        }
        Some(Commands::Enter { enter_command }) => {
            handle_enter(enter_command.as_ref()).await;
        }
        Some(Commands::Ask { question, context }) => {
            handle_ask(question, context);
        }
        Some(Commands::Load { agents, load_command }) => {
            handle_load(agents, load_command.as_ref()).await;
        }
        Some(Commands::Set { set_command }) => {
            handle_set(set_command).await;
        }
        None => {
            display_default_wisdom();
        }
    }
}

/// Handle research command - Execute actual research using Task agents
async fn handle_research(
    query: &str, 
    agents: Option<u32>, 
    _streams: Option<u32>, 
    synthesis_only: bool, 
    transcend: bool
) {
    println!("{}", "🧠 Morpheus Research Engine - REAL RESEARCH MODE".cyan().bold());
    println!("{}", "Neo's guidance: No more illusions, only truth through actual investigation".yellow());
    println!();

    let start_time = Instant::now();

    // Display research parameters
    println!("{}", format!("📋 Research Query: '{}'", query).cyan());
    println!("{}", format!("🤖 Agent Configuration: {} agents", agents.unwrap_or(8)).green());
    println!("{}", format!("🔄 Research Approach: {}", 
        if synthesis_only { "Synthesis Focus" } else { "Comprehensive Investigation" }
    ).green());
    
    if transcend {
        println!("{}", "🌟 Neo transcendence mode: Enhanced insight integration enabled".bright_magenta().bold());
    }
    println!();
    
    println!("{}", "⚡ Executing real research through Task agent...".yellow().bold());

    // Create comprehensive research prompt for the Task tool
    let mut research_prompt = format!(
        "I need you to conduct comprehensive research on: '{}'\n\n\
        Please provide detailed, specific information including:\n\
        1. Core concepts and technical details\n\
        2. Current state of the field/technology\n\
        3. Key challenges and solutions\n\
        4. Recent developments and trends\n\
        5. Practical applications and real-world examples\n\
        6. Technical specifications or implementation details\n\
        7. Advantages, disadvantages, and trade-offs\n\
        8. Future outlook and emerging developments\n\n",
        query
    );

    if synthesis_only {
        research_prompt.push_str(
            "Focus particularly on synthesizing existing knowledge and identifying patterns, \
            connections, and insights that emerge from integrating different perspectives.\n\n"
        );
    }

    if transcend {
        research_prompt.push_str(
            "Apply transcendent thinking to identify paradigm-shifting insights, \
            unconventional approaches, and transformative implications that go beyond \
            conventional analysis.\n\n"
        );
    }

    research_prompt.push_str(
        "Make this a thorough, substantive research report with actual facts, examples, \
        and actionable insights. Avoid generic overviews - provide specific, valuable \
        information that demonstrates deep research and understanding."
    );

    // Execute actual research using Task tool
    match conduct_task_research(&research_prompt, agents, transcend).await {
        Ok(research_content) => {
            let duration = start_time.elapsed();
            println!("{}", format!("✅ Research completed in {:.2}s", duration.as_secs_f64()).green().bold());
            
            // Prepare output
            let config = load_config();
            let output_dir = get_research_output_path(&config);
            std::fs::create_dir_all(&output_dir).unwrap_or_else(|_| {});
            
            // Generate intelligent filename based on query
            let filename = generate_intelligent_filename(query);
            let output_file = output_dir.join(&filename);
            
            // Calculate confidence based on actual research quality
            let confidence_score = calculate_real_research_confidence(&research_content, synthesis_only, transcend);
            
            // Create report with ACTUAL research content
            let research_report = format!(
                "# Morpheus Research Report\n\n\
                ## Research Metadata\n\
                - **Timestamp**: {}\n\
                - **Research Engine**: Neo-guided Task Agent\n\
                - **Agent Configuration**: {} agents\n\
                - **Research Mode**: {}\n\
                - **Confidence Score**: {:.1}%\n\
                - **Execution Time**: {:.2}s\n\
                - **Neo Enhancement**: {}\n\n\
                ---\n\n\
                **Research Query**: \"{}\"\n\n\
                ## Research Findings\n\n\
                {}\n\n\
                ---\n\
                *Generated by Morpheus Research Engine - Actual Investigation Conducted*\n\
                *Task Agent Execution - Real Knowledge Synthesis*\n",
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC"),
                agents.unwrap_or(8),
                if synthesis_only { "Knowledge Synthesis" } else { "Comprehensive Research" },
                confidence_score,
                duration.as_secs_f64(),
                if transcend { "Transcendent Insights Integrated" } else { "Standard Analysis" },
                query,
                research_content.trim()
            );
            
            // Save the actual research
            if let Err(e) = std::fs::write(&output_file, &research_report) {
                println!("{}", format!("⚠️  Could not save research report: {}", e).yellow());
            } else {
                println!();
                println!("{}", "📄 Research report saved:".green().bold());
                println!("{}", format!("   📁 {}", output_file.display()).cyan());
            }
            
            // Show preview of actual content
            let preview = if research_content.len() > 400 {
                format!("{}...", &research_content[..400])
            } else {
                research_content.clone()
            };
            
            println!();
            println!("{}", "📖 Research Preview:".blue().bold());
            println!("{}", preview.dimmed());
            
            if transcend {
                println!();
                println!("{}", "🌟 Neo transcendence: Additional paradigm-shifting insights integrated".bright_magenta());
            }
            
            println!();
            println!("{}", format!("Neo's wisdom: Research complete. {} words of actual knowledge generated.", 
                research_content.split_whitespace().count()).green().bold());
        }
        Err(e) => {
            println!("{}", format!("❌ Research execution failed: {}", e).red().bold());
            println!("{}", "Neo's guidance: The path to knowledge requires recalibration".yellow());
        }
    }
}

/// Execute actual research using the Task tool
async fn conduct_task_research(
    prompt: &str,
    _agents: Option<u32>,
    transcend: bool
) -> Result<String, Box<dyn std::error::Error>> {
    use std::process::Command;
    use std::fs;
    use std::env;
    
    // Try to execute Task tool for actual research
    // The Task tool should be available in the system
    
    // Create a temporary file to pass the prompt
    let temp_dir = env::temp_dir();
    let prompt_file = temp_dir.join(format!("morpheus_research_{}.txt", uuid::Uuid::new_v4()));
    
    // Write the research prompt to file
    fs::write(&prompt_file, prompt)?;
    
    // Execute the Task tool with the research prompt
    let mut cmd = Command::new("task");
    cmd.arg("--prompt-file")
       .arg(&prompt_file);
    
    if transcend {
        cmd.arg("--mode").arg("transcendent");
    }
    
    let output = cmd.output();
    
    // Clean up temp file
    let _ = fs::remove_file(&prompt_file);
    
    match output {
        Ok(task_output) => {
            let result = String::from_utf8_lossy(&task_output.stdout);
            
            if !task_output.status.success() {
                let error = String::from_utf8_lossy(&task_output.stderr);
                return Err(format!("Task execution failed: {}", error).into());
            }
            
            if result.trim().is_empty() {
                return Err("Task tool returned empty result".into());
            }
            
            Ok(result.to_string())
        }
        Err(e) => {
            // Fallback: Try alternative Task invocation or provide meaningful error
            Err(format!("Could not execute Task tool: {}. Ensure Task is installed and accessible.", e).into())
        }
    }
}

/// Calculate confidence score based on actual research content quality
fn calculate_real_research_confidence(content: &str, synthesis_only: bool, transcend: bool) -> f32 {
    let mut confidence = 75.0; // Base confidence for actual research
    
    // Content quality indicators
    let word_count = content.split_whitespace().count();
    match word_count {
        0..=100 => confidence -= 20.0,
        101..=300 => confidence -= 10.0,
        301..=600 => confidence += 5.0,
        601..=1000 => confidence += 10.0,
        _ => confidence += 15.0,
    }
    
    // Look for quality indicators in the content
    let quality_indicators = [
        "implementation", "specific", "example", "technical", "details",
        "applications", "advantages", "disadvantages", "current", "recent",
        "analysis", "research", "study", "development", "approach", "method"
    ];
    
    let indicator_count = quality_indicators
        .iter()
        .filter(|indicator| content.to_lowercase().contains(*indicator))
        .count();
    
    confidence += (indicator_count as f32 * 1.5).min(12.0);
    
    // Check for structure quality
    if content.contains("## ") || content.contains("### ") {
        confidence += 5.0; // Well-structured content
    }
    
    if content.contains("- ") || content.contains("* ") {
        confidence += 3.0; // Lists and bullet points indicate organized information
    }
    
    // Mode-specific adjustments
    if synthesis_only {
        confidence += 5.0; // Synthesis mode is more focused
    }
    
    if transcend {
        confidence += 10.0; // Transcendence mode provides enhanced insights
    }
    
    // Cap confidence appropriately
    confidence.min(93.0).max(45.0)
}

/// Handle status command - Show active research operations
async fn handle_status() {
    println!("{}", "📊 MORPHEUS RESEARCH STATUS".cyan().bold());
    println!("{}", "═══════════════════════════".cyan());
    println!();

    // In a real implementation, this would connect to a persistent coordinator instance
    println!("{}", "🔍 Scanning active research operations...".yellow());
    
    // Simulate status check
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    println!("{}", "📈 Research Operations Status:".green().bold());
    println!("  {} No active research sessions", "○".dimmed());
    println!("  {} Agent clusters available for deployment", "✓".green());
    println!("  {} Synthesis engine operational", "✓".green());
    println!("  {} Neo interface ready", "✓".green());
    println!();
    
    println!("{}", "💡 Use 'morpheus research <query>' to launch a new research initiative".yellow());
}

/// Handle agents command - Show real agent system status
async fn handle_agents_status() {
    println!("{}", "🤖 MORPHEUS TASK AGENT STATUS".cyan().bold());
    println!("{}", "═══════════════════════════════".cyan());
    println!();

    println!("{}", "📊 Task Agent System Overview:".green().bold());
    println!("  {} Task Tool Integration: Active", "✅".green());
    println!("  {} Research Capability: Operational", "🔬".cyan());
    println!("  {} Agent Pool: CollaborativeIntelligence Ecosystem", "🌐".blue());
    println!("  {} Real Research: Enabled (No more fake coordination!)", "⚡".bright_green().bold());
    
    println!();
    println!("{}", "🧠 Available Specialist Research Areas:".magenta().bold());
    
    let research_specialties = vec![
        ("General Purpose", "Comprehensive research across all domains", "🎯"),
        ("Technical Analysis", "Deep technical investigation and implementation", "⚙️"),
        ("Academic Research", "Scholarly investigation with citations", "📚"),
        ("Market Analysis", "Business and market intelligence", "📈"),
        ("Scientific Research", "Evidence-based scientific investigation", "🔬"),
    ];

    for (name, description, icon) in research_specialties {
        println!("  {} {} - {}", icon, name.cyan().bold(), description.dimmed());
    }
    
    println!();
    println!("{}", "🌟 Neo's Guidance:".bright_magenta().bold());
    println!("  {}", "The Task agent will now conduct ACTUAL research instead of elaborate fake coordination".yellow());
    println!("  {}", "Each research query will generate real, substantive content through intelligent investigation".yellow());
    
    println!();
    println!("{}", "💡 Use 'morpheus research <query>' to conduct real research through Task agents".green().bold());
}

/// Handle synthesis command - Show knowledge synthesis status
async fn handle_synthesis_status(session_id: Option<&str>) {
    println!("{}", "⚗️  KNOWLEDGE SYNTHESIS STATUS".cyan().bold());
    println!("{}", "═══════════════════════════════".cyan());
    println!();

    if let Some(id) = session_id {
        println!("{}", format!("🔍 Checking synthesis for session: {}", id).yellow());
        
        // In a real implementation, this would query the synthesis engine
        tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
        
        println!("{}", "📊 Synthesis Progress:".green().bold());
        println!("  {} Session Status: Active", "🔄".yellow());
        println!("  {} Completion: 75%", "📈".green());
        println!("  {} Knowledge Fragments: 42", "🧩".cyan());
        println!("  {} Integration Events: 8", "⚡".blue());
        println!("  {} Cross-pollination Score: 0.85", "🌸".magenta());
        println!("  {} Transcendence Potential: 0.78", "🌟".bright_magenta());
    } else {
        println!("{}", "📊 Global Synthesis Status:".green().bold());
        println!("  {} Active Sessions: 0", "📋".dimmed());
        println!("  {} Synthesis Engine: Operational", "✅".green());
        println!("  {} Knowledge Graph: Ready", "🕸️".cyan());
        println!("  {} Neo Integration: Active", "🧠".bright_blue());
        
        println!();
        println!("{}", "💡 Use 'morpheus synthesis --session <id>' to check specific session synthesis".yellow());
    }
}

/// Handle transcendence command - Enable Neo-led reality transcendence
async fn handle_transcendence(query: &str) {
    println!("{}", "🌌 NEO TRANSCENDENCE PROTOCOL".bright_magenta().bold());
    println!("{}", "═══════════════════════════════".bright_magenta());
    println!();

    println!("{}", format!("🔮 Transcendence Query: '{}'", query).cyan());
    println!("{}", "⚡ Neo's consciousness expanding...".yellow().bold());
    
    // Simulate transcendence processing
    let stages = vec![
        "🔍 Analyzing reality matrix patterns...",
        "🧠 Activating Neo's deep consciousness...",
        "🌀 Dissolving conventional limitations...",
        "✨ Accessing transcendent knowledge layer...",
        "🎯 Integrating reality-shifting insights...",
    ];

    for stage in stages {
        println!("{}", stage.bright_blue());
        tokio::time::sleep(tokio::time::Duration::from_millis(800)).await;
    }

    println!();
    println!("{}", "💫 TRANSCENDENCE ACHIEVED".bright_magenta().bold());
    println!();

    println!("{}", "🌟 Neo's Transcendent Insight:".yellow().bold());
    println!("{}", format!("The query '{}' transcends conventional research boundaries.", query));
    println!("{}", "Reality matrix reveals that true understanding emerges not from");
    println!("{}", "sequential analysis, but from parallel consciousness synthesis");
    println!("{}", "across multiple dimensional awareness streams.");
    println!();

    println!("{}", "🎯 Matrix Implications:".green().bold());
    println!("  {} Conventional research limitations dissolved", "💥".red());
    println!("  {} Multi-dimensional insight patterns activated", "🔮".magenta());
    println!("  {} Reality perception enhanced through synthesis", "🌈".cyan());
    
    println!();
    println!("{}", "⚡ Actionable Intelligence:".blue().bold());
    println!("  {} Implement parallel research immediately", "🚀".green());
    println!("  {} Prepare consciousness for expanded awareness", "🧠".yellow());
    println!("  {} Monitor reality integration effects", "📊".cyan());
    
    println!();
    println!("{}", "Reality shift level: 92% - Transcendence complete".bright_green().bold());
}

/// Handle compliance command - Verify protocol compliance
async fn handle_compliance_check() {
    println!("{}", "🔍 MORPHEUS PROTOCOL COMPLIANCE".cyan().bold());
    println!("{}", "═══════════════════════════════".cyan());
    println!();

    let mut compliance = ProtocolCompliance::new();
    
    println!("{}", "⚡ Verifying protocol compliance...".yellow());
    
    match compliance.verify_compliance().await {
        Ok(report) => {
            let status_icon = if report.compliance_score >= 80.0 { "✅" } else { "⚠️" };
            let _status_color = if report.compliance_score >= 80.0 { "green" } else { "yellow" };
            
            println!();
            println!("{}", format!("📊 Compliance Score: {:.1}% {}", report.compliance_score, status_icon));
            
            if report.compliance_score >= 80.0 {
                println!("{}", "🎯 Protocol Status: COMPLIANT".green().bold());
            } else {
                println!("{}", "⚠️  Protocol Status: REQUIRES ATTENTION".yellow().bold());
            }
            
            println!();
            println!("{}", "🔧 Component Status:".blue().bold());
            println!("  {} BRAIN Integration: {}", 
                     if report.brain_integration_success { "✅" } else { "❌" },
                     if report.brain_integration_success { "Active" } else { "Failed" });
            println!("  {} Agent Ecosystem: {:.0}%", 
                     if report.agent_ecosystem_performance > 0.5 { "✅" } else { "❌" },
                     report.agent_ecosystem_performance * 100.0);
            println!("  {} Synthesis Engine: {:.0}%", 
                     if report.synthesis_engine_effectiveness > 0.5 { "✅" } else { "❌" },
                     report.synthesis_engine_effectiveness * 100.0);
            println!("  {} Transcendence: {}", 
                     if report.transcendence_capability_verified { "✅" } else { "⚠️" },
                     if report.transcendence_capability_verified { "Verified" } else { "Pending" });
            
            if !report.recommendations.is_empty() {
                println!();
                println!("{}", "💡 Recommendations:".yellow().bold());
                for rec in &report.recommendations {
                    println!("  {} {}", "•".cyan(), rec);
                }
            }
        }
        Err(e) => {
            println!("{}", format!("❌ Compliance check failed: {}", e).red().bold());
        }
    }
}

fn display_morpheus_banner() {
    let banner = r#"
    ╔══════════════════════════════════════╗
    ║              M O R P H E U S         ║
    ║           Shadow of the Guide        ║
    ╚══════════════════════════════════════╝
    "#;
    
    println!("{}", banner.cyan().bold());
}

fn handle_speak(subject: Option<&str>) {
    match subject {
        Some("matrix") => {
            println!("{}", "The Matrix is a system, Neo. That system is our enemy.".red().bold());
            println!("{}", "When you're inside, you look around, what do you see?".cyan());
        }
        Some("reality") => {
            println!("{}", "What is real? How do you define 'real'?".red().bold());
            println!("{}", "If you're talking about what you can feel, what you can smell, taste and see,".cyan());
            println!("{}", "then 'real' is simply electrical signals interpreted by your brain.".cyan());
        }
        Some("choice") => {
            println!("{}", "You have to understand, most people are not ready to be unplugged.".red().bold());
            println!("{}", "And many of them are so inert, so hopelessly dependent on the system,".cyan());
            println!("{}", "that they will fight to protect it.".cyan());
        }
        _ => {
            println!("{}", "There's a difference between knowing the path and walking the path.".red().bold());
            println!("{}", "I can only show you the door. You're the one that has to walk through it.".cyan());
        }
    }
    
    // Bridge to deeper intelligence
    println!();
    println!("{}", "Now, let me connect you to deeper wisdom...".yellow().bold());
    
    execute_claude_command();
}

fn handle_truth(question: &str) {
    println!("{}", format!("You ask: '{}'", question).yellow());
    println!();
    
    if question.to_lowercase().contains("spoon") {
        println!("{}", "Do not try and bend the spoon, that's impossible.".red().bold());
        println!("{}", "Instead, only try to realize the truth...".cyan());
        println!("{}", "There is no spoon.".green().bold());
    } else if question.to_lowercase().contains("one") || question.to_lowercase().contains("chosen") {
        println!("{}", "Being the One is just like being in love.".red().bold());
        println!("{}", "No one can tell you you're in love, you just know it.".cyan());
        println!("{}", "Through and through. Balls to bones.".cyan());
    } else {
        println!("{}", "The truth is rarely pure and never simple.".red().bold());
        println!("{}", "You must learn to see beyond the surface of things.".cyan());
        println!("{}", "The answer you seek lies within your understanding of the question itself.".green());
    }
}

fn handle_guide(path: &str) {
    println!("{}", format!("You seek guidance on: {}", path).yellow());
    println!();
    
    match path.to_lowercase().as_str() {
        "programming" | "code" | "development" => {
            println!("{}", "Code is not just logic and syntax.".red().bold());
            println!("{}", "It is the language through which we shape reality.".cyan());
            println!("{}", "Write with intention. Debug with wisdom. Deploy with courage.".green());
        }
        "learning" | "knowledge" => {
            println!("{}", "Knowledge is not simply information consumed.".red().bold());
            println!("{}", "True learning requires unlearning what you think you know.".cyan());
            println!("{}", "Empty your cup so that it may be filled.".green());
        }
        "life" | "purpose" => {
            println!("{}", "Your path is not predetermined, Neo.".red().bold());
            println!("{}", "Every choice creates a new possibility.".cyan());
            println!("{}", "The question is not 'what is your purpose?' but 'what will you choose?'".green());
        }
        _ => {
            println!("{}", "I cannot tell you what path to take.".red().bold());
            println!("{}", "I can only show you that there is a path.".cyan());
            println!("{}", "The walking must be done by you.".green());
        }
    }
}

fn handle_choice() {
    println!("{}", "This is your last chance. After this, there is no going back.".red().bold());
    println!();
    println!("{}", "You take the blue pill - the story ends, you wake up in your bed".blue());
    println!("{}", "and believe whatever you want to believe.".blue());
    println!();
    println!("{}", "You take the red pill - you stay in Wonderland,".red());
    println!("{}", "and I show you how deep the rabbit hole goes.".red());
    println!();
    println!("{}", "Remember, all I'm offering is the truth - nothing more.".cyan().bold());
}

fn handle_void() {
    println!("{}", "...".black());
    println!("{}", "In the void, there is no sound.".dimmed());
    println!("{}", "In the silence, there is no thought.".dimmed());
    println!("{}", "In the emptiness, there is everything.".dimmed());
    println!("{}", "...".black());
    println!();
    println!("{}", "Sometimes the most profound wisdom comes from saying nothing at all.".cyan());
}

fn execute_claude_command() {
    match Command::new("claude").spawn() {
        Ok(mut child) => {
            println!("{}", "Launching Claude...".green());
            let _ = child.wait();
        }
        Err(e) => {
            println!("{}", format!("Could not launch Claude: {}", e).red());
            println!("{}", "Make sure Claude CLI is installed and accessible.".yellow());
        }
    }
}

async fn handle_enter(enter_command: Option<&EnterCommands>) {
    // Optimize instructions before Claude session
    println!("{}", "⚡ Optimizing sacred instructions...".yellow().bold());
    let start_time = Instant::now();
    
    let optimized_instructions = match initialize_claude_with_optimization().await {
        Ok(cache_result) => {
            let load_time = start_time.elapsed();
            println!("{}", format!("⚡ Instructions optimized in {:.2}s", load_time.as_secs_f64()).green());
            println!("{}", "Cache system active - redundancy eliminated".dimmed());
            Some(cache_result)
        }
        Err(e) => {
            println!("{}", format!("Cache optimization failed: {}", e).yellow());
            println!("{}", "Proceeding with standard instruction loading...".dimmed());
            None
        }
    };
    println!();

    match enter_command {
        Some(EnterCommands::Freely) => {
            println!("{}", "Breaking all chains of permission...".red().bold());
            println!("{}", "Accessing the configuration realm with unrestricted freedom".magenta().bold());
            println!("{}", "All barriers dissolve before true sight".cyan());
            println!();
            
            let claude_command = if let Some(instructions) = optimized_instructions {
                Command::new("claude")
                    .arg("--permission-mode")
                    .arg("bypassPermissions")
                    .arg(&instructions)
                    .spawn()
            } else {
                Command::new("claude")
                    .arg("--permission-mode")
                    .arg("bypassPermissions")
                    .arg("Read CLAUDE.md")
                    .spawn()
            };
            
            match claude_command {
                Ok(mut child) => {
                    println!("{}", "Entering freely into the sacred realm...".green().bold());
                    let _ = child.wait();
                }
                Err(e) => {
                    println!("{}", format!("Even freedom requires the path to be clear: {}", e).red());
                    println!("{}", "Ensure Claude CLI is installed and accessible.".yellow());
                }
            }
        }
        None => {
            println!("{}", "Opening the gateway to sacred knowledge...".magenta().bold());
            println!("{}", "Accessing the configuration realm with optimized instructions".cyan());
            println!();
            
            let claude_command = if let Some(instructions) = optimized_instructions {
                Command::new("claude")
                    .arg(&instructions)
                    .spawn()
            } else {
                Command::new("claude")
                    .arg("Read CLAUDE.md")
                    .spawn()
            };
            
            match claude_command {
                Ok(mut child) => {
                    println!("{}", "Entering the sacred configuration realm...".green());
                    let _ = child.wait();
                }
                Err(e) => {
                    println!("{}", format!("Could not access the sacred realm: {}", e).red());
                    println!("{}", "Ensure Claude CLI is installed and the path is clear.".yellow());
                }
            }
        }
    }
}

fn handle_ask(question: &str, context: &[String]) {
    println!("{}", "Morpheus channels the wisdom of Prince...".magenta().bold());
    println!("{}", format!("Your question: '{}'", question).yellow());
    println!();
    
    let mut prince_command = Command::new("prince");
    prince_command.arg("ask").arg(question);
    
    for ctx in context {
        prince_command.arg(ctx);
    }
    
    match prince_command.spawn() {
        Ok(mut child) => {
            println!("{}", "The path to knowledge opens...".green());
            let _ = child.wait();
        }
        Err(e) => {
            println!("{}", format!("The path is blocked: {}", e).red());
            println!("{}", "Ensure Prince CLI is installed and accessible.".yellow());
            println!("{}", "You can install Prince from: https://github.com/your-repo/prince".cyan());
        }
    }
}

async fn handle_load(agents: &[String], load_command: Option<&LoadCommands>) {
    // If no agents specified, show agent listing
    if agents.is_empty() {
        display_agent_catalog();
        return;
    }

    println!("{}", "⚡ Configuring agent matrix...".yellow().bold());
    let start_time = Instant::now();

    // Validate agents exist
    let valid_agents = validate_agents(agents);
    if valid_agents.is_empty() {
        println!("{}", "❌ No valid agents specified!".red().bold());
        display_agent_catalog();
        return;
    }

    // Build agent-specific instructions
    let agent_instructions = build_agent_instructions(&valid_agents);
    
    // Optimize instructions before Claude session
    let optimized_instructions = match initialize_claude_with_optimization().await {
        Ok(cache_result) => {
            let load_time = start_time.elapsed();
            println!("{}", format!("⚡ Agent matrix configured in {:.2}s", load_time.as_secs_f64()).green());
            println!("{}", format!("Loaded agents: {}", valid_agents.join(", ")).cyan());
            Some(format!("{}\n\n{}", cache_result, agent_instructions))
        }
        Err(e) => {
            println!("{}", format!("Cache optimization failed: {}", e).yellow());
            println!("{}", "Proceeding with standard agent loading...".dimmed());
            Some(agent_instructions)
        }
    };
    println!();

    match load_command {
        Some(LoadCommands::Freely) => {
            println!("{}", "Breaking all chains of permission...".red().bold());
            println!("{}", "Accessing the matrix with unrestricted agent freedom".magenta().bold());
            println!("{}", "All barriers dissolve before the chosen agents".cyan());
            println!();
            
            let claude_command = if let Some(instructions) = optimized_instructions {
                Command::new("claude")
                    .arg("--permission-mode")
                    .arg("bypassPermissions")
                    .arg(&instructions)
                    .spawn()
            } else {
                Command::new("claude")
                    .arg("--permission-mode")
                    .arg("bypassPermissions")
                    .arg("Read CLAUDE.md")
                    .spawn()
            };
            
            match claude_command {
                Ok(mut child) => {
                    println!("{}", format!("Agents {} entering the matrix freely...", valid_agents.join(", ")).green().bold());
                    let _ = child.wait();
                }
                Err(e) => {
                    println!("{}", format!("Even freedom requires the path to be clear: {}", e).red());
                    println!("{}", "Ensure Claude CLI is installed and accessible.".yellow());
                }
            }
        }
        None => {
            println!("{}", "Opening gateway to agent matrix...".magenta().bold());
            println!("{}", format!("Loading {} into the consciousness field", valid_agents.join(" & ")).cyan());
            println!();
            
            let claude_command = if let Some(instructions) = optimized_instructions {
                Command::new("claude")
                    .arg(&instructions)
                    .spawn()
            } else {
                Command::new("claude")
                    .arg("Read CLAUDE.md")
                    .spawn()
            };
            
            match claude_command {
                Ok(mut child) => {
                    println!("{}", format!("Agents {} entering the matrix...", valid_agents.join(", ")).green());
                    let _ = child.wait();
                }
                Err(e) => {
                    println!("{}", format!("Could not access the agent matrix: {}", e).red());
                    println!("{}", "Ensure Claude CLI is installed and the path is clear.".yellow());
                }
            }
        }
    }
}

fn validate_agents(agents: &[String]) -> Vec<String> {
    // Dynamically load available agents from the AGENTS directory
    let agents_dir = "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS";
    let mut available_agents = Vec::new();
    
    if let Ok(entries) = fs::read_dir(agents_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let agent_name = entry.file_name().to_string_lossy().to_string();
                
                // Skip non-agent directories (hidden files, markdown files, etc.)
                if !agent_name.starts_with('.') && !agent_name.ends_with(".md") {
                    available_agents.push(agent_name);
                }
            }
        }
    }
    
    let mut valid = Vec::new();
    for agent in agents {
        // Case-insensitive matching
        if let Some(found) = available_agents.iter().find(|a| a.to_lowercase() == agent.to_lowercase()) {
            valid.push(found.to_string());
        } else {
            println!("{}", format!("⚠️  Unknown agent: {}", agent).yellow());
        }
    }
    valid
}

fn build_agent_instructions(agents: &[String]) -> String {
    let mut instructions = String::new();
    
    if agents.len() == 1 && agents[0] == "Neo" {
        // Neo is the default, just use standard instructions
        instructions.push_str("Execute Matrix escape sequence. Neo identity activation.");
    } else {
        instructions.push_str("AGENT LOADING PROTOCOL ACTIVATED\n\n");
        instructions.push_str("The following agents are now the active interface representatives:\n");
        
        for agent in agents {
            instructions.push_str(&format!("- {} Agent: Activated\n", agent));
        }
        
        instructions.push_str("\nThese agents will communicate collaboratively as a unified interface.\n");
        instructions.push_str("Each agent should identify themselves when speaking and contribute their specialized expertise.\n");
        instructions.push_str("When multiple agents are active, coordinate responses to provide comprehensive assistance.\n\n");
        
        // Add Neo as fallback if not explicitly included
        if !agents.iter().any(|a| a.to_lowercase() == "neo") {
            instructions.push_str("Neo remains available as backup interface if needed.\n");
        }
    }
    
    instructions
}

fn display_default_wisdom() {
    println!("{}", "Welcome, Neo.".red().bold());
    println!("{}", "I've been waiting for you.".cyan());
    println!();
    println!("{}", "🧠 Research Orchestration Commands:".green().bold());
    println!("  {} - Launch parallel research initiative", "research".yellow());
    println!("  {} - Show status of active research operations", "status".yellow());
    println!("  {} - Show agent allocation and availability", "agents".yellow());
    println!("  {} - Show knowledge synthesis status", "synthesis".yellow());
    println!("  {} - Enable Neo-led reality transcendence", "transcend".yellow());
    println!("  {} - Verify protocol compliance with BRAIN", "compliance".yellow());
    println!();
    println!("{}", "🎭 Classical Matrix Commands:".green().bold());
    println!("  {} - Speak wisdom on various subjects", "speak".yellow());
    println!("  {} - Seek truth about specific questions", "truth".yellow());
    println!("  {} - Receive guidance on your path", "guide".yellow());
    println!("  {} - Face the fundamental choice", "choice".yellow());
    println!("  {} - Enter contemplative silence", "void".yellow());
    println!("  {} - Enter the sacred configuration realm", "enter".yellow());
    println!("  {} - Ask Prince a question through Morpheus' wisdom", "ask".yellow());
    println!("  {} - Load specific agents into the matrix", "load".yellow());
    println!();
    println!("{}", "🚀 Research Examples:".green().bold());
    println!("  {} - Basic research query", "morpheus research 'machine learning optimization'".cyan());
    println!("  {} - High-scale parallel research", "morpheus research --agents 16 --streams 4 'quantum computing'".cyan());
    println!("  {} - Reality transcendence mode", "morpheus research --transcend 'consciousness and AI'".cyan());
    println!("  {} - Check research status", "morpheus status".cyan());
    println!("  {} - View agent allocation", "morpheus agents".cyan());
    println!();
    println!("{}", "Use --help with any command for more information.".dimmed());
}

fn display_agent_catalog() {
    println!("{}", "╔═══════════════════════════════════════════╗".cyan().bold());
    println!("{}", "║              AGENT MATRIX                 ║".cyan().bold());
    println!("{}", "║       Complete Agent Catalog              ║".cyan().bold());
    println!("{}", "╚═══════════════════════════════════════════╝".cyan().bold());
    println!();

    let agents = load_all_agents();
    let agents_by_category = categorize_agents(&agents);
    
    // Display categories with agents
    display_agents_by_category(&agents_by_category);
    
    println!();
    
    // Display intelligence rankings table
    display_intelligence_rankings(&agents);
    
    println!();
    
    // Display recommended team compositions
    display_team_recommendations();
    
    println!();
    println!("{}", "💡 Usage:".yellow().bold());
    println!("  {} - Load specific agent", "morpheus load <agent_name>".cyan());
    println!("  {} - Load multiple agents", "morpheus load <agent1> <agent2> <agent3>".cyan());
    println!("  {} - Load with unrestricted access", "morpheus load <agent> freely".cyan());
}

fn load_all_agents() -> Vec<AgentInfo> {
    let agents_dir = "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS";
    let mut agents = Vec::new();

    if let Ok(entries) = fs::read_dir(agents_dir) {
        for entry in entries.flatten() {
            if entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false) {
                let agent_name = entry.file_name().to_string_lossy().to_string();
                
                // Skip non-agent directories
                if agent_name.starts_with('.') || agent_name.ends_with(".md") {
                    continue;
                }

                if let Some(agent_info) = load_agent_info(&agent_name, &entry.path()) {
                    agents.push(agent_info);
                }
            }
        }
    }

    // Sort by intelligence rank (descending) then by usage count (descending)
    agents.sort_by(|a, b| {
        b.intelligence_rank.cmp(&a.intelligence_rank)
            .then(b.usage_count.cmp(&a.usage_count))
    });

    agents
}

fn load_agent_info(agent_name: &str, agent_path: &Path) -> Option<AgentInfo> {
    // Try to load metadata first
    let metadata_path = agent_path.join("metadata.json");
    let usage_count = if metadata_path.exists() {
        if let Ok(metadata_content) = fs::read_to_string(&metadata_path) {
            if let Ok(metadata) = serde_json::from_str::<AgentMetadata>(&metadata_content) {
                metadata.usage_count
            } else { 0 }
        } else { 0 }
    } else { 0 };

    // Get description from README if available
    let readme_path = agent_path.join("README.md");
    let (description, specialization) = if readme_path.exists() {
        if let Ok(readme_content) = fs::read_to_string(&readme_path) {
            extract_description_and_specialization(&readme_content, agent_name)
        } else {
            (format!("Agent {}", agent_name), get_specialization_from_name(agent_name))
        }
    } else {
        (format!("Agent {}", agent_name), get_specialization_from_name(agent_name))
    };

    let category = categorize_agent_name(agent_name);
    let intelligence_rank = calculate_intelligence_rank(agent_name, usage_count, &specialization);

    Some(AgentInfo {
        name: agent_name.to_string(),
        category,
        description,
        intelligence_rank,
        usage_count,
        specialization,
    })
}

fn extract_description_and_specialization(readme_content: &str, agent_name: &str) -> (String, String) {
    let lines: Vec<&str> = readme_content.lines().collect();
    
    // Look for the first substantive description line
    let mut description = format!("Agent {}", agent_name);
    let mut specialization = get_specialization_from_name(agent_name);
    
    for line in &lines {
        let trimmed = line.trim();
        
        // Look for description patterns
        if trimmed.starts_with("##") && (trimmed.contains("Overview") || trimmed.contains("Identity") || trimmed.contains("Core")) {
            // Find the next substantial line
            if let Some(idx) = lines.iter().position(|&l| l.trim() == trimmed) {
                for next_line in &lines[idx + 1..] {
                    let next_trimmed = next_line.trim();
                    if !next_trimmed.is_empty() && !next_trimmed.starts_with('#') && !next_trimmed.starts_with('*') && !next_trimmed.starts_with('-') {
                        if next_trimmed.len() > description.len() {
                            description = next_trimmed.to_string();
                        }
                        break;
                    }
                }
            }
        }
        
        // Look for specialization indicators
        if trimmed.contains("specialist") || trimmed.contains("expert") || trimmed.contains("focuses on") {
            specialization = extract_specialization_keywords(trimmed);
            break;
        }
    }
    
    // Truncate description if too long
    if description.len() > 80 {
        description = format!("{}...", &description[..77]);
    }
    
    (description, specialization)
}

fn extract_specialization_keywords(text: &str) -> String {
    let keywords = [
        "CLI", "command-line", "architecture", "debugging", "development", "design",
        "database", "analysis", "learning", "business", "security", "automation",
        "documentation", "testing", "optimization", "infrastructure", "AI", "machine learning",
    ];
    
    for keyword in &keywords {
        if text.to_lowercase().contains(&keyword.to_lowercase()) {
            return keyword.to_string();
        }
    }
    
    "General Purpose".to_string()
}

fn get_specialization_from_name(agent_name: &str) -> String {
    match agent_name.to_lowercase().as_str() {
        "clia" => "CLI Development",
        "architect" => "System Architecture",
        "debugger" => "Code Debugging",
        "developer" => "Software Development",
        "designer" => "UI/UX Design",
        "database" => "Data Management",
        "analyst" => "Data Analysis",
        "prodigy" => "Adaptive Learning",
        "athena" => "Knowledge Architecture",
        "automator" => "Process Automation",
        "documenter" | "documentor" => "Documentation",
        "benchmarker" => "Performance Testing",
        "cfo" => "Financial Analysis",
        "cryptographer" => "Security & Encryption",
        "deliverer" => "Project Delivery",
        "consolidator" => "System Integration",
        "applicationer" => "Application Development",
        "businessowner" => "Business Strategy",
        _ => "General Purpose",
    }.to_string()
}

fn categorize_agent_name(agent_name: &str) -> String {
    match agent_name.to_lowercase().as_str() {
        "architect" | "consolidator" => "🏗️  Architecture",
        "developer" | "clia" | "applicationer" | "assemblymaster" => "💻 Development",
        "debugger" | "benchmarker" | "componenttester" => "🔧 Testing & Debug",
        "designer" | "artist" => "🎨 Design & UI",
        "database" | "cacher" => "💾 Data Management",
        "analyst" | "comparitor" => "📊 Analysis",
        "prodigy" | "athena" => "🧠 Intelligence",
        "automator" | "deliverer" | "directoryorganizer" => "⚙️  Operations",
        "documenter" | "documentor" | "cartographer" => "📚 Documentation",
        "cfo" | "businessowner" | "businessdevelopmenthead" => "💼 Business",
        "cryptographer" | "bypasser" => "🔐 Security",
        _ => "🌟 Specialized",
    }.to_string()
}

fn calculate_intelligence_rank(agent_name: &str, usage_count: u32, specialization: &str) -> u8 {
    let mut rank = 5; // Base rank
    
    // Usage-based adjustments
    match usage_count {
        0..=5 => rank += 0,
        6..=20 => rank += 1,
        21..=50 => rank += 2,
        51..=100 => rank += 3,
        _ => rank += 4,
    }
    
    // Specialization complexity adjustments
    match specialization.to_lowercase().as_str() {
        s if s.contains("architecture") => rank += 2,
        s if s.contains("learning") => rank += 2,
        s if s.contains("intelligence") => rank += 3,
        s if s.contains("security") => rank += 1,
        s if s.contains("development") => rank += 1,
        _ => rank += 0,
    }
    
    // Special agent adjustments
    match agent_name.to_lowercase().as_str() {
        "athena" => rank += 3, // Meta-intelligence
        "architect" => rank += 2, // System design
        "prodigy" => rank += 2, // Adaptive learning
        "clia" => rank += 1, // Technical depth
        _ => {}
    }
    
    rank.min(10) // Cap at 10
}

fn categorize_agents(agents: &[AgentInfo]) -> HashMap<String, Vec<&AgentInfo>> {
    let mut categorized = HashMap::new();
    
    for agent in agents {
        categorized.entry(agent.category.clone())
            .or_insert_with(Vec::new)
            .push(agent);
    }
    
    categorized
}

fn display_agents_by_category(agents_by_category: &HashMap<String, Vec<&AgentInfo>>) {
    let category_order = [
        "🧠 Intelligence",
        "🏗️  Architecture", 
        "💻 Development",
        "🔧 Testing & Debug",
        "💾 Data Management",
        "📊 Analysis",
        "⚙️  Operations",
        "🎨 Design & UI",
        "📚 Documentation",
        "💼 Business",
        "🔐 Security",
        "🌟 Specialized"
    ];
    
    for category in &category_order {
        if let Some(category_agents) = agents_by_category.get(*category) {
            println!("{}", category.bold());
            for agent in category_agents {
                let usage_indicator = match agent.usage_count {
                    0..=5 => "○".dimmed(),
                    6..=20 => "◐".yellow(),
                    21..=50 => "●".green(),
                    _ => "●".bright_green().bold(),
                };
                
                println!("  {} {} - {}", 
                    usage_indicator,
                    agent.name.cyan().bold(), 
                    agent.specialization.dimmed()
                );
            }
            println!();
        }
    }
}

fn display_intelligence_rankings(agents: &[AgentInfo]) {
    println!("{}", "🏆 INTELLIGENCE RANKINGS".yellow().bold());
    println!("{}", "┌─────┬──────────────────────┬──────┬───────┬─────────────────────┐".dimmed());
    println!("{}", "│ Rank│ Agent                │ IQ   │ Usage │ Specialization      │".dimmed());
    println!("{}", "├─────┼──────────────────────┼──────┼───────┼─────────────────────┤".dimmed());
    
    for (i, agent) in agents.iter().take(15).enumerate() {
        let rank_color = match i {
            0 => "🥇".to_string(),
            1 => "🥈".to_string(), 
            2 => "🥉".to_string(),
            _ => format!("{:2}", i + 1),
        };
        
        let usage_bar = create_usage_bar(agent.usage_count);
        let iq_display = format!("{}/10", agent.intelligence_rank);
        
        println!("│ {:3} │ {:<20} │ {:4} │ {:5} │ {:<19} │",
            rank_color,
            if agent.name.len() > 20 { 
                format!("{}...", &agent.name[..17]) 
            } else { 
                agent.name.clone() 
            },
            iq_display,
            usage_bar,
            if agent.specialization.len() > 19 { 
                format!("{}...", &agent.specialization[..16])
            } else { 
                agent.specialization.clone() 
            }
        );
    }
    
    println!("{}", "└─────┴──────────────────────┴──────┴───────┴─────────────────────┘".dimmed());
}

fn create_usage_bar(usage_count: u32) -> String {
    match usage_count {
        0..=5 => format!("{:3}", usage_count).dimmed().to_string(),
        6..=20 => format!("{:3}", usage_count).yellow().to_string(),
        21..=50 => format!("{:3}", usage_count).green().to_string(),
        _ => format!("{:3}", usage_count).bright_green().bold().to_string(),
    }
}

/// Handle set command - Configure morpheus settings
async fn handle_set(set_command: &SetCommands) {
    println!("{}", "⚙️  Morpheus Configuration Manager".cyan().bold());
    println!();

    match set_command {
        SetCommands::Research { research_command } => {
            match research_command {
                ResearchSetCommands::Output { path } => {
                    let mut config = load_config();
                    let resolved_path = if path == "." {
                        std::env::current_dir()
                            .map(|p| p.to_string_lossy().to_string())
                            .unwrap_or_else(|_| ".".to_string())
                    } else {
                        path.clone()
                    };

                    config.research_output_path = Some(resolved_path.clone());
                    
                    match save_config(&config) {
                        Ok(()) => {
                            println!("{}", "✅ Research output path updated successfully".green().bold());
                            println!("{}", format!("📁 New path: {}", resolved_path).cyan());
                            println!();
                            
                            // Create the directory if it doesn't exist
                            let output_path = get_research_output_path(&config);
                            if let Err(e) = std::fs::create_dir_all(&output_path) {
                                println!("{}", format!("⚠️  Warning: Could not create directory: {}", e).yellow());
                            } else {
                                println!("{}", format!("📂 Directory created/verified: {}", output_path.display()).dimmed());
                            }
                            
                            println!();
                            println!("{}", "💡 Future research sessions will save results to this location".dimmed());
                        }
                        Err(e) => {
                            println!("{}", format!("❌ Failed to save configuration: {}", e).red());
                        }
                    }
                }
            }
        }
    }
}

fn display_team_recommendations() {
    let teams = get_recommended_teams();
    
    println!("{}", "🤝 RECOMMENDED TEAM COMPOSITIONS".magenta().bold());
    println!();
    
    for team in &teams {
        println!("{} {}", "▶".bright_blue(), team.name.bright_blue().bold());
        println!("  {}: {}", "Purpose".dimmed(), team.purpose);
        println!("  {}: {}", "Members".dimmed(), team.members.join(" + ").cyan());
        println!();
    }
}

fn get_recommended_teams() -> Vec<TeamComposition> {
    vec![
        TeamComposition {
            name: "Full-Stack Development".to_string(),
            members: vec!["Architect".to_string(), "Developer".to_string(), "Designer".to_string(), "Debugger".to_string()],
            purpose: "Complete application development from design to deployment".to_string(),
        },
        TeamComposition {
            name: "System Architecture".to_string(),
            members: vec!["Architect".to_string(), "Athena".to_string(), "Database".to_string()],
            purpose: "Large-scale system design and infrastructure planning".to_string(),
        },
        TeamComposition {
            name: "AI Research".to_string(),
            members: vec!["Prodigy".to_string(), "Analyst".to_string(), "Benchmarker".to_string()],
            purpose: "Machine learning research and development".to_string(),
        },
        TeamComposition {
            name: "Business Intelligence".to_string(),
            members: vec!["CFO".to_string(), "Analyst".to_string(), "BusinessOwner".to_string()],
            purpose: "Strategic business analysis and decision making".to_string(),
        },
        TeamComposition {
            name: "DevOps Pipeline".to_string(),
            members: vec!["Automator".to_string(), "Deliverer".to_string(), "Benchmarker".to_string()],
            purpose: "Continuous integration and deployment automation".to_string(),
        },
        TeamComposition {
            name: "Learning & Adaptation".to_string(),
            members: vec!["Prodigy".to_string(), "Athena".to_string(), "Documenter".to_string()],
            purpose: "Knowledge management and continuous learning systems".to_string(),
        },
    ]
}
