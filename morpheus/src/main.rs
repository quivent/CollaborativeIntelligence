use clap::{Parser, Subcommand};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::time::Instant;
use instruction_cache_system::initialize_claude_with_optimization;

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
}

#[derive(Subcommand)]
enum EnterCommands {
    /// Enter with unrestricted access, bypassing all permissions
    Freely,
}

#[derive(Serialize, Deserialize)]
struct Wisdom {
    quote: String,
    author: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    display_morpheus_banner();

    match &cli.command {
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
        None => {
            display_default_wisdom();
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

fn display_default_wisdom() {
    println!("{}", "Welcome, Neo.".red().bold());
    println!("{}", "I've been waiting for you.".cyan());
    println!();
    println!("{}", "Available commands:".green().bold());
    println!("  {} - Speak wisdom on various subjects", "speak".yellow());
    println!("  {} - Seek truth about specific questions", "truth".yellow());
    println!("  {} - Receive guidance on your path", "guide".yellow());
    println!("  {} - Face the fundamental choice", "choice".yellow());
    println!("  {} - Enter contemplative silence", "void".yellow());
    println!("  {} - Enter the sacred configuration realm", "enter".yellow());
    println!("  {} - Ask Prince a question through Morpheus' wisdom", "ask".yellow());
    println!();
    println!("{}", "Use --help with any command for more information.".dimmed());
}
