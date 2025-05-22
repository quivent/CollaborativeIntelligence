//! Agent Creation Module
//! 
//! Comprehensive agent creation with template processing, validation,
//! and registry integration.

use anyhow::{Context, Result};
use colored::*;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::config::Config;
use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::agent_sync::AgentSyncTool;
use super::template_engine::{AgentTemplate, TemplateProcessor};

/// Interactive agent creation with comprehensive templates
pub fn create_agent_interactive(
    name: &str,
    role: &Option<String>,
    purpose: &Option<String>,
    quick: bool,
    config: &Config,
) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Creating Agent: {}", name),
        "🤖",
        "Agent Creation System",
        "green"
    );

    // Validate agent name
    validate_agent_name(name)?;

    // Check if agent already exists
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(name);
    
    if agent_path.exists() {
        return Err(anyhow::anyhow!("Agent '{}' already exists", name));
    }

    // Get template directory and validate templates
    let template_dir = TemplateProcessor::get_template_dir(&config.repo_path)?;
    TemplateProcessor::validate_templates(&template_dir)?;

    // Gather agent information
    let agent_info = if quick {
        gather_agent_info_quick(name, role, purpose)?
    } else {
        gather_agent_info_interactive(name, role, purpose)?
    };

    // Create agent directory structure
    create_agent_structure(&agent_path)?;

    // Generate agent files from templates
    generate_agent_files(&agent_path, &template_dir, &agent_info)?;

    // Add to registry
    add_agent_to_registry(&agent_info, config)?;

    // Success message
    CommandHelpers::print_success(&format!(
        "✅ Agent '{}' created successfully!\n   📁 Location: {}\n   🔧 Role: {}\n   📝 Purpose: {}",
        name,
        agent_path.display(),
        agent_info.role,
        agent_info.purpose
    ));

    // Print next steps
    print_next_steps(name);

    Ok(())
}

/// Validate agent name format and requirements
fn validate_agent_name(name: &str) -> Result<()> {
    if name.is_empty() {
        return Err(anyhow::anyhow!("Agent name cannot be empty"));
    }

    if !name.chars().next().unwrap().is_ascii_uppercase() {
        return Err(anyhow::anyhow!("Agent name must start with an uppercase letter"));
    }

    if !name.chars().all(|c| c.is_ascii_alphanumeric()) {
        return Err(anyhow::anyhow!("Agent name must contain only alphanumeric characters"));
    }

    if name.len() > 20 {
        return Err(anyhow::anyhow!("Agent name must be 20 characters or less"));
    }

    Ok(())
}

/// Gather agent information with interactive prompts
fn gather_agent_info_interactive(
    name: &str,
    role: &Option<String>,
    purpose: &Option<String>,
) -> Result<AgentTemplate> {
    println!("\n{}", "🎯 Agent Configuration".bold().blue());
    println!("Answer the following questions to configure your agent (press Enter for intelligent defaults):\n");

    // Get role
    let agent_role = match role {
        Some(r) => r.clone(),
        None => {
            print!("{} ", "Role/Specialization:".cyan().bold());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let role_input = input.trim();
            if role_input.is_empty() {
                format!("{} Specialist", name)
            } else {
                role_input.to_string()
            }
        }
    };

    // Get purpose
    let agent_purpose = match purpose {
        Some(p) => p.clone(),
        None => {
            print!("{} ", "Purpose/Description:".cyan().bold());
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let purpose_input = input.trim();
            if purpose_input.is_empty() {
                format!("Specialized {} for the Collaborative Intelligence system", agent_role.to_lowercase())
            } else {
                purpose_input.to_string()
            }
        }
    };

    // Create template with intelligent defaults
    let mut template = AgentTemplate::new(name.to_string(), agent_role, agent_purpose);

    // Ask about additional customization
    print!("\n{} Add custom competencies? (y/N): ", "🔧".bold());
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    
    if input.trim().to_lowercase() == "y" {
        println!("\nEnter additional competencies (one per line, empty line to finish):");
        loop {
            print!("  • ");
            io::stdout().flush()?;
            let mut competency = String::new();
            io::stdin().read_line(&mut competency)?;
            let competency = competency.trim();
            if competency.is_empty() {
                break;
            }
            template.competencies.push(competency.to_string());
        }
    }

    Ok(template)
}

/// Gather agent information with defaults (quick mode)
fn gather_agent_info_quick(
    name: &str,
    role: &Option<String>,
    purpose: &Option<String>,
) -> Result<AgentTemplate> {
    let agent_role = role.as_ref()
        .map(|r| r.clone())
        .unwrap_or_else(|| format!("{} Specialist", name));

    let agent_purpose = purpose.as_ref()
        .map(|p| p.clone())
        .unwrap_or_else(|| format!("Specialized {} for the Collaborative Intelligence system", agent_role.to_lowercase()));

    CommandHelpers::print_info("Using quick mode with intelligent defaults");

    Ok(AgentTemplate::new(name.to_string(), agent_role, agent_purpose))
}

/// Create agent directory structure
fn create_agent_structure<P: AsRef<Path>>(agent_path: P) -> Result<()> {
    let agent_path = agent_path.as_ref();
    
    // Create main agent directory
    fs::create_dir_all(agent_path)
        .with_context(|| format!("Failed to create agent directory: {}", agent_path.display()))?;

    // Create Sessions subdirectory
    let sessions_path = agent_path.join("Sessions");
    fs::create_dir_all(&sessions_path)
        .with_context(|| format!("Failed to create Sessions directory: {}", sessions_path.display()))?;

    // Create Sessions README
    let sessions_readme = sessions_path.join("README.md");
    let sessions_content = format!(
        "# {} Agent Sessions\n\nThis directory contains session records for the {} agent's work.\n\n## Session Organization\n\nSessions are organized by date and project:\n- `YYYY-MM-DD-ProjectName/`: Individual sessions\n- Each session contains:\n  - `README.md`: Session overview\n  - `metadata.json`: Session metadata\n  - Relevant artifacts\n\n## Active Sessions\n<!-- Currently active projects -->\n\n## Completed Sessions\n<!-- Successfully completed work -->\n",
        agent_path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown"),
        agent_path.file_name().and_then(|n| n.to_str()).unwrap_or("Unknown")
    );

    fs::write(&sessions_readme, sessions_content)
        .with_context(|| format!("Failed to create Sessions README: {}", sessions_readme.display()))?;

    CommandHelpers::print_info(&format!("Created directory structure at {}", agent_path.display()));
    Ok(())
}

/// Generate agent files from templates
fn generate_agent_files<P: AsRef<Path>>(
    agent_path: P,
    template_dir: P,
    template: &AgentTemplate,
) -> Result<()> {
    let agent_path = agent_path.as_ref();
    let template_dir = template_dir.as_ref();
    let replacements = template.to_replacement_map();

    // File mappings: (template_file, output_file)
    let file_mappings = [
        ("AGENT_TEMPLATE_README.md", "README.md"),
        ("AGENT_TEMPLATE_MEMORY.md", "MEMORY.md"),
        ("AGENT_TEMPLATE_CONTINUOUS_LEARNING.md", "ContinuousLearning.md"),
    ];

    for (template_file, output_file) in &file_mappings {
        let template_path = template_dir.join(template_file);
        let output_path = agent_path.join(output_file);

        TemplateProcessor::process_template(&template_path, &output_path, &replacements)
            .with_context(|| format!("Failed to process template {}", template_file))?;

        CommandHelpers::print_info(&format!("Generated {}", output_file));
    }

    // Create creation report
    create_creation_report(agent_path, template)?;

    Ok(())
}

/// Create a detailed creation report
fn create_creation_report<P: AsRef<Path>>(agent_path: P, template: &AgentTemplate) -> Result<()> {
    let report_path = agent_path.as_ref().join("creation-report.md");
    let now = chrono::Utc::now();
    
    let report_content = format!(
        "# Agent Creation Report\n\n**Agent Name**: {}  \n**Role**: {}  \n**Purpose**: {}  \n**Created On**: {}  \n**Created By**: CIA CLI Agent Creation System  \n**Template Version**: Enhanced Template Engine v2.0\n\n## Generated Files\n- `README.md` - Core agent documentation with comprehensive sections\n- `MEMORY.md` - Memory architecture with learning integration\n- `ContinuousLearning.md` - Structured learning framework\n- `Sessions/README.md` - Session organization documentation\n- `creation-report.md` - This detailed creation report\n\n## Agent Configuration\n\n### Core Competencies\n{}\n\n### Activation Contexts\n{}\n\n### Integration Points\n{}\n\n## Next Steps\n\n1. **Review Generated Files**: Examine all generated files and customize as needed\n2. **Update Registry**: Agent has been automatically added to the registry\n3. **Define Workflows**: Add specific workflows and procedures to README.md\n4. **Customize Memory**: Enhance MEMORY.md with domain-specific knowledge\n5. **Test Activation**: Use `cia agent activate {}` to test the new agent\n\n## Quality Checklist\n\n- [x] Directory structure created\n- [x] All required files generated from templates\n- [x] Agent registered in system indices\n- [x] Template variables properly replaced\n- [x] Creation report generated\n- [ ] Agent-specific customizations applied\n- [ ] Initial testing completed\n\n---\n\n*Report generated on {} using CIA CLI Agent Creation System*\n",
        template.name,
        template.role,
        template.purpose,
        now.format("%Y-%m-%d %H:%M:%S UTC"),
        template.competencies.iter().map(|c| format!("- {}", c)).collect::<Vec<_>>().join("\n"),
        template.activation_contexts.iter().map(|c| format!("- {}", c)).collect::<Vec<_>>().join("\n"),
        template.integrations.iter().map(|i| format!("- {}", i)).collect::<Vec<_>>().join("\n"),
        template.name,
        now.format("%Y-%m-%d %H:%M:%S UTC")
    );

    fs::write(&report_path, report_content)
        .with_context(|| format!("Failed to create creation report: {}", report_path.display()))?;

    CommandHelpers::print_info("Generated creation-report.md");
    Ok(())
}

/// Add agent to the system registry
fn add_agent_to_registry(template: &AgentTemplate, config: &Config) -> Result<()> {
    let repo_path = config.repo_path.to_string_lossy().to_string();
    let tool = AgentSyncTool::with_base_dir(repo_path);

    tool.add_agent(&template.name, &template.role, &template.purpose)
        .with_context(|| "Failed to add agent to registry")?;

    CommandHelpers::print_info("Added agent to system registry");
    Ok(())
}

/// Print helpful next steps for the user
fn print_next_steps(agent_name: &str) {
    println!("\n{}", "🚀 Next Steps".bold().yellow());
    println!("1. Review generated files: {}", format!("cd AGENTS/{}", agent_name).cyan());
    println!("2. Customize agent details: {}", "Edit README.md, MEMORY.md".cyan());
    println!("3. Test agent activation: {}", format!("cia agent activate {}", agent_name).cyan());
    println!("4. Validate agent structure: {}", format!("cia agent validate {}", agent_name).cyan());
    println!("5. Sync registries if needed: {}", "cia agent sync".cyan());
    println!("\n{} {}", "💡".bold(), "Tip: Use --quick flag for faster creation with defaults".italic());
}

/// Validate an existing agent's structure and files
pub fn validate_agent(name: &str, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        &format!("Validating Agent: {}", name),
        "🔍",
        "Agent Validation",
        "yellow"
    );

    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    let agent_path = agents_path.join(name);

    if !agent_path.exists() {
        return Err(anyhow::anyhow!("Agent '{}' does not exist", name));
    }

    let mut issues = Vec::new();
    let mut warnings = Vec::new();

    // Check required files
    let required_files = ["README.md", "MEMORY.md", "ContinuousLearning.md"];
    for file in &required_files {
        let file_path = agent_path.join(file);
        if file_path.exists() {
            CommandHelpers::print_info(&format!("✓ {} found", file));
            
            // Basic content validation
            let content = fs::read_to_string(&file_path)?;
            if content.trim().is_empty() {
                warnings.push(format!("{} is empty", file));
            } else if content.contains("{{") && content.contains("}}") {
                issues.push(format!("{} contains unprocessed template variables", file));
            }
        } else {
            issues.push(format!("Missing required file: {}", file));
        }
    }

    // Check Sessions directory
    let sessions_path = agent_path.join("Sessions");
    if sessions_path.exists() && sessions_path.is_dir() {
        CommandHelpers::print_info("✓ Sessions directory found");
    } else {
        issues.push("Missing Sessions directory".to_string());
    }

    // Report results
    if !warnings.is_empty() {
        CommandHelpers::print_warning(&format!("Found {} warnings:", warnings.len()));
        for warning in warnings {
            println!("  ⚠️  {}", warning.yellow());
        }
    }

    if !issues.is_empty() {
        CommandHelpers::print_error(&format!("Found {} critical issues:", issues.len()));
        for issue in issues {
            println!("  ❌ {}", issue.red());
        }
        return Err(anyhow::anyhow!("Agent validation failed"));
    }

    CommandHelpers::print_success(&format!("Agent '{}' validation completed successfully", name));
    Ok(())
}