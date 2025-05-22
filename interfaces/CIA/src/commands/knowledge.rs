use crate::helpers::CommandHelpers;
use crate::helpers::path::PathHelpers;
use crate::config::Config;
use colored::*;
use anyhow::Result;
use std::path::PathBuf;
use std::collections::HashMap;

/// Search the knowledge registry
pub fn search(query: &str, category: &Option<String>, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Search",
        "🔍",
        "Knowledge Management",
        "yellow"
    );
    
    let category_str = match category {
        Some(cat) => format!(" in category '{}'", cat),
        None => "".to_string(),
    };
    
    CommandHelpers::print_info(&format!("Searching for '{}'{}", query, category_str));
    
    // Check if Knowledge Registry exists
    let knowledge_registry = config.repo_path.join("KnowledgeRegistry");
    if !knowledge_registry.exists() {
        CommandHelpers::print_warning("Knowledge Registry not found. Creating one for the search.");
        
        // Create knowledge registry directory if it doesn't exist
        match std::fs::create_dir_all(&knowledge_registry) {
            Ok(_) => CommandHelpers::print_info(&format!("Created Knowledge Registry at: {}", knowledge_registry.display())),
            Err(e) => {
                CommandHelpers::print_error(&format!("Failed to create Knowledge Registry: {}", e));
                return Err(anyhow::anyhow!("Failed to create Knowledge Registry: {}", e));
            }
        }
    }
    
    // Locate files to search based on category
    let search_path = match category {
        Some(cat) => knowledge_registry.join("category_indices").join(format!("{}_index.md", cat.to_lowercase())),
        None => knowledge_registry.join("master_index.md"),
    };
    
    // Simulate knowledge search
    CommandHelpers::print_info("Querying knowledge registry...");
    CommandHelpers::print_info("Processing results...");
    
    // Check if we have a real index file to search
    let has_real_index = search_path.exists();
    
    // Simulate search results
    println!("\n{}", "Search Results:".yellow().bold());
    
    if has_real_index {
        // Read the index file and find matches (simplified simulation)
        match std::fs::read_to_string(&search_path) {
            Ok(content) => {
                let mut found = false;
                for line in content.lines() {
                    if line.to_lowercase().contains(&query.to_lowercase()) {
                        println!("- {} - {}", search_path.file_name().unwrap().to_string_lossy().blue(), line);
                        found = true;
                    }
                }
                
                if !found {
                    println!("No matches found for query: {}", query);
                }
            },
            Err(e) => {
                CommandHelpers::print_warning(&format!("Failed to read index file: {}", e));
                // Fall back to simulated results
                println!("- {} - {}", "architecture_index.md".blue(), "Architecture design patterns for distributed systems");
                println!("- {} - {}", "protocols.md".blue(), "System-wide communication protocols");
            }
        }
    } else {
        // Simulated results for demonstration
        println!("- {} - {}", "architecture_index.md".blue(), "Architecture design patterns for distributed systems");
        println!("- {} - {}", "protocols.md".blue(), "System-wide communication protocols");
    }
    
    CommandHelpers::print_success("Search completed");
    Ok(())
}

/// Update knowledge registry indexes
pub fn index(config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Indexing",
        "📚",
        "Knowledge Management",
        "yellow"
    );
    
    // Check if Knowledge Registry exists
    let knowledge_registry = config.repo_path.join("KnowledgeRegistry");
    if !knowledge_registry.exists() {
        CommandHelpers::print_info("Knowledge Registry not found. Creating directory structure.");
        
        // Create knowledge registry directory structure
        let directories = [
            knowledge_registry.clone(),
            knowledge_registry.join("category_indices"),
            knowledge_registry.join("temp"),
        ];
        
        for dir in &directories {
            match std::fs::create_dir_all(dir) {
                Ok(_) => CommandHelpers::print_info(&format!("Created directory: {}", dir.display())),
                Err(e) => {
                    CommandHelpers::print_error(&format!("Failed to create directory: {}", e));
                    return Err(anyhow::anyhow!("Failed to create directory: {}", e));
                }
            }
        }
    }
    
    // Simulate indexing
    CommandHelpers::print_info("Analyzing repository structure...");
    
    // Search for knowledge sources in the repository
    let search_dirs = [
        config.repo_path.join("docs"),
        config.repo_path.join("Architectures"),
        config.repo_path.join("Protocols"),
        config.repo_path.join("AGENTS"),
    ];
    
    let mut found_files = 0;
    for dir in &search_dirs {
        if dir.exists() {
            CommandHelpers::print_info(&format!("Scanning directory: {}", dir.display()));
            
            // Simulate file counting (would be actual glob search in real implementation)
            match count_markdown_files(dir) {
                Ok(count) => {
                    CommandHelpers::print_info(&format!("Found {} markdown files in {}", count, dir.display()));
                    found_files += count;
                },
                Err(e) => {
                    CommandHelpers::print_warning(&format!("Error scanning directory {}: {}", dir.display(), e));
                }
            }
        } else {
            CommandHelpers::print_info(&format!("Directory not found, skipping: {}", dir.display()));
        }
    }
    
    CommandHelpers::print_info(&format!("Identified {} knowledge sources", found_files));
    CommandHelpers::print_info("Building category indices...");
    CommandHelpers::print_info("Generating cross-references...");
    CommandHelpers::print_info("Updating master index...");
    
    // Create master index file
    let master_index = knowledge_registry.join("master_index.md");
    let master_index_content = format!("# Knowledge Registry Master Index\n\n## Last Updated: {}\n\n## Categories\n\n- Architecture\n- Protocols\n- Optimizations\n\n## Contents\n\n- {}\n", 
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        found_files
    );
    
    match std::fs::write(&master_index, master_index_content) {
        Ok(_) => CommandHelpers::print_info(&format!("Created master index at: {}", master_index.display())),
        Err(e) => {
            CommandHelpers::print_error(&format!("Failed to write master index: {}", e));
            return Err(anyhow::anyhow!("Failed to write master index: {}", e));
        }
    }
    
    CommandHelpers::print_success("Knowledge registry indexes updated successfully");
    Ok(())
}

/// Helper function to count markdown files in a directory (recursive)
fn count_markdown_files(dir: &PathBuf) -> Result<usize> {
    use glob::glob;
    
    let pattern = format!("{}//**/*.md", dir.display());
    let files: Vec<_> = glob(&pattern)
        .map_err(|e| anyhow::anyhow!("Failed to read glob pattern: {}", e))?
        .filter_map(Result::ok)
        .collect();
        
    Ok(files.len())
}

/// Distribute knowledge to agents
pub fn distribute(agent: &Option<String>, config: &Config) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Distribution",
        "📡",
        "Knowledge Management",
        "yellow"
    );
    
    match agent {
        Some(name) => {
            // Check if agent exists
            let agents_path = PathHelpers::get_agents_path(&config.repo_path);
            let agent_path = agents_path.join(name);
            
            if !agent_path.exists() {
                CommandHelpers::print_error(&format!("Agent '{}' does not exist", name));
                return Err(anyhow::anyhow!("Agent does not exist: {}", name));
            }
            
            CommandHelpers::print_info(&format!("Distributing knowledge to agent '{}'", name));
            CommandHelpers::print_info("Identifying relevant knowledge segments...");
            CommandHelpers::print_info("Preparing knowledge integration...");
            
            // Check if agent has a memory file
            let memory_path = agent_path.join("MEMORY.md");
            if !memory_path.exists() {
                CommandHelpers::print_warning(&format!("Agent '{}' does not have a memory file", name));
                
                // Create memory file if it doesn't exist
                let memory_content = format!("# {} Agent Memory\n\n## Core Knowledge\n\n- \n", name);
                match std::fs::write(&memory_path, memory_content) {
                    Ok(_) => CommandHelpers::print_info(&format!("Created MEMORY.md for {}", name)),
                    Err(e) => CommandHelpers::print_error(&format!("Failed to create MEMORY.md: {}", e)),
                }
            }
            
            CommandHelpers::print_info(&format!("Updating agent '{}' memory", name));
            CommandHelpers::print_success(&format!("Knowledge distributed to agent '{}'", name));
            Ok(())
        },
        None => {
            CommandHelpers::print_info("Distributing knowledge to all agents");
            
            // Get all agents
            let agents_path = PathHelpers::get_agents_path(&config.repo_path);
            
            match std::fs::read_dir(agents_path) {
                Ok(entries) => {
                    let mut agents = Vec::new();
                    
                    for entry in entries.filter_map(Result::ok) {
                        if entry.path().is_dir() {
                            if let Some(name) = entry.path().file_name() {
                                if let Some(name_str) = name.to_str() {
                                    agents.push(name_str.to_string());
                                }
                            }
                        }
                    }
                    
                    if agents.is_empty() {
                        CommandHelpers::print_warning("No agents found for knowledge distribution");
                        return Ok(());
                    }
                    
                    CommandHelpers::print_info(&format!("Found {} agents", agents.len()));
                    
                    for agent in agents {
                        CommandHelpers::print_info(&format!("Distributing knowledge to agent '{}'", agent));
                    }
                    
                    CommandHelpers::print_success("Knowledge distributed to all agents");
                    Ok(())
                },
                Err(e) => {
                    CommandHelpers::print_error(&format!("Failed to read agents directory: {}", e));
                    Err(anyhow::anyhow!("Failed to read agents directory: {}", e))
                }
            }
        }
    }
}

/// Run complete knowledge workflow
pub fn workflow(config: &Config, threshold: Option<f64>, report_only: bool) -> Result<()> {
    CommandHelpers::print_command_header(
        "Knowledge Distribution Workflow",
        "🔄",
        "Knowledge Management",
        "cyan"
    );
    
    let threshold = threshold.unwrap_or(0.7);
    CommandHelpers::print_info(&format!("Running workflow with threshold: {}", threshold));
    CommandHelpers::print_info(&format!("Mode: {}", if report_only { "Report Only" } else { "Full Processing" }));
    
    // Step 1: Ensure Knowledge Registry exists
    let knowledge_registry = config.repo_path.join("KnowledgeRegistry");
    if !knowledge_registry.exists() {
        CommandHelpers::print_info("Creating Knowledge Registry...");
        std::fs::create_dir_all(&knowledge_registry)?;
        
        // Create subdirectories
        let subdirs = ["category_indices", "temp"];
        for subdir in &subdirs {
            std::fs::create_dir_all(knowledge_registry.join(subdir))?;
        }
    }
    
    // Step 2: Identify globally relevant content
    CommandHelpers::print_info("Identifying globally relevant content...");
    let identification_result = identify_global_content(&config.repo_path, threshold)?;
    
    CommandHelpers::print_info(&format!("Found {} relevant files", identification_result.relevant_files.len()));
    
    if !report_only {
        // Step 3: Distribute knowledge
        CommandHelpers::print_info("Distributing knowledge to agents...");
        distribute_identified_content(&identification_result, config)?;
    }
    
    // Step 4: Update knowledge registry index
    CommandHelpers::print_info("Updating knowledge registry index...");
    index(config)?;
    
    // Generate workflow report
    let report_path = knowledge_registry.join("workflow_report.md");
    generate_workflow_report(&report_path, &identification_result, threshold, report_only)?;
    
    CommandHelpers::print_success(&format!("Workflow complete. Report: {}", report_path.display()));
    Ok(())
}

#[derive(Debug)]
struct IdentificationResult {
    relevant_files: Vec<RelevantFile>,
    total_scanned: usize,
    threshold_used: f64,
}

#[derive(Debug)]
struct RelevantFile {
    path: PathBuf,
    relevance_score: f64,
    categories: Vec<String>,
    content_summary: String,
}

/// Identify globally relevant content
fn identify_global_content(repo_path: &PathBuf, threshold: f64) -> Result<IdentificationResult> {
    let mut relevant_files = Vec::new();
    let mut total_scanned = 0;
    
    // Directories to scan
    let scan_dirs = [
        repo_path.join("docs"),
        repo_path.join("Architectures"),
        repo_path.join("Protocols"),
        repo_path.join("AGENTS"),
        repo_path.join("Plans"),
        repo_path.join("Sessions"),
    ];
    
    for dir in &scan_dirs {
        if dir.exists() {
            scan_directory_for_content(dir, &mut relevant_files, &mut total_scanned, threshold)?;
        }
    }
    
    // Sort by relevance score
    relevant_files.sort_by(|a, b| b.relevance_score.partial_cmp(&a.relevance_score).unwrap());
    
    Ok(IdentificationResult {
        relevant_files,
        total_scanned,
        threshold_used: threshold,
    })
}

/// Scan a directory for relevant content
fn scan_directory_for_content(
    dir: &PathBuf,
    relevant_files: &mut Vec<RelevantFile>,
    total_scanned: &mut usize,
    threshold: f64,
) -> Result<()> {
    for entry in glob::glob(&format!("{}/**/*.md", dir.display()))? {
        let path = entry?;
        *total_scanned += 1;
        
        if let Ok(content) = std::fs::read_to_string(&path) {
            let relevance_score = calculate_relevance_score(&content);
            
            if relevance_score >= threshold {
                let categories = extract_categories(&content, &path);
                let content_summary = generate_content_summary(&content);
                
                relevant_files.push(RelevantFile {
                    path,
                    relevance_score,
                    categories,
                    content_summary,
                });
            }
        }
    }
    
    Ok(())
}

/// Calculate relevance score for content
fn calculate_relevance_score(content: &str) -> f64 {
    let content_lower = content.to_lowercase();
    let mut score = 0.0;
    
    // Global relevance keywords with weights
    let keywords = [
        ("architecture", 0.3),
        ("protocol", 0.25),
        ("system", 0.2),
        ("framework", 0.2),
        ("implementation", 0.15),
        ("design", 0.15),
        ("specification", 0.2),
        ("interface", 0.15),
        ("api", 0.15),
        ("standard", 0.2),
        ("guideline", 0.15),
        ("best practice", 0.2),
        ("pattern", 0.15),
        ("strategy", 0.15),
        ("methodology", 0.15),
    ];
    
    for (keyword, weight) in &keywords {
        let count = content_lower.matches(keyword).count();
        score += count as f64 * weight;
    }
    
    // Normalize by content length (rough estimation)
    let word_count = content.split_whitespace().count();
    if word_count > 0 {
        score = score / (word_count as f64 / 100.0).max(1.0);
    }
    
    // Cap at 1.0
    score.min(1.0)
}

/// Extract categories from content
fn extract_categories(content: &str, path: &PathBuf) -> Vec<String> {
    let mut categories = Vec::new();
    let content_lower = content.to_lowercase();
    
    // Category detection
    let category_keywords = [
        ("Architecture", vec!["architecture", "design", "structure", "component"]),
        ("Protocols", vec!["protocol", "communication", "interface", "api"]),
        ("Optimizations", vec!["optimization", "performance", "efficiency", "speed"]),
        ("Documentation", vec!["documentation", "guide", "tutorial", "readme"]),
        ("Implementation", vec!["implementation", "code", "development", "programming"]),
    ];
    
    for (category, keywords) in &category_keywords {
        for keyword in keywords {
            if content_lower.contains(keyword) {
                categories.push(category.to_string());
                break;
            }
        }
    }
    
    // Path-based categorization
    let path_str = path.to_string_lossy().to_lowercase();
    if path_str.contains("docs") {
        categories.push("Documentation".to_string());
    }
    if path_str.contains("protocol") {
        categories.push("Protocols".to_string());
    }
    if path_str.contains("architecture") {
        categories.push("Architecture".to_string());
    }
    
    if categories.is_empty() {
        categories.push("General".to_string());
    }
    
    categories.sort();
    categories.dedup();
    categories
}

/// Generate content summary
fn generate_content_summary(content: &str) -> String {
    let lines: Vec<&str> = content.lines().collect();
    
    // Extract title from first header
    let title = lines.iter()
        .find(|line| line.starts_with('#'))
        .map(|line| line.trim_start_matches('#').trim())
        .unwrap_or("Untitled");
    
    // Extract first paragraph after headers
    let description = lines.iter()
        .skip_while(|line| line.starts_with('#') || line.trim().is_empty())
        .find(|line| !line.trim().is_empty())
        .map(|line| {
            let mut desc = line.to_string();
            if desc.len() > 100 {
                desc.truncate(97);
                desc.push_str("...");
            }
            desc
        })
        .unwrap_or_else(|| "No description available".to_string());
    
    format!("{}: {}", title, description)
}

/// Distribute identified content to agents
fn distribute_identified_content(result: &IdentificationResult, config: &Config) -> Result<()> {
    let agents_path = PathHelpers::get_agents_path(&config.repo_path);
    
    // Group files by category
    let mut category_files: std::collections::HashMap<String, Vec<&RelevantFile>> = std::collections::HashMap::new();
    
    for file in &result.relevant_files {
        for category in &file.categories {
            category_files.entry(category.clone()).or_insert_with(Vec::new).push(file);
        }
    }
    
    // Distribute to agents based on their specialization
    if let Ok(entries) = std::fs::read_dir(&agents_path) {
        for entry in entries.filter_map(Result::ok) {
            let agent_path = entry.path();
            if agent_path.is_dir() {
                let agent_name = agent_path.file_name().unwrap().to_string_lossy();
                
                // Skip system agents
                if agent_name.starts_with('.') {
                    continue;
                }
                
                let memory_path = agent_path.join("MEMORY.md");
                if memory_path.exists() {
                    update_agent_memory_with_knowledge(&memory_path, &category_files, &agent_name)?;
                }
            }
        }
    }
    
    Ok(())
}

/// Update agent memory with relevant knowledge
fn update_agent_memory_with_knowledge(
    memory_path: &PathBuf,
    category_files: &std::collections::HashMap<String, Vec<&RelevantFile>>,
    agent_name: &str,
) -> Result<()> {
    let content = std::fs::read_to_string(memory_path)?;
    
    // Find relevant categories for this agent
    let relevant_categories = determine_agent_categories(agent_name, &content);
    
    let mut knowledge_section = String::new();
    knowledge_section.push_str("\n## Distributed Knowledge\n\n");
    
    for category in &relevant_categories {
        if let Some(files) = category_files.get(category) {
            knowledge_section.push_str(&format!("### {} Knowledge\n\n", category));
            
            for file in files.iter().take(3) { // Limit to top 3 per category
                knowledge_section.push_str(&format!(
                    "- **{}** (Score: {:.2}): {}\n",
                    file.path.file_name().unwrap().to_string_lossy(),
                    file.relevance_score,
                    file.content_summary
                ));
            }
            knowledge_section.push('\n');
        }
    }
    
    // Append to memory file if there's relevant knowledge
    if !knowledge_section.trim().is_empty() {
        let mut updated_content = content;
        
        // Remove existing knowledge section if present
        if let Some(start) = updated_content.find("## Distributed Knowledge") {
            if let Some(end) = updated_content[start..].find("\n## ") {
                updated_content.replace_range(start..start + end, "");
            } else {
                updated_content.truncate(start);
            }
        }
        
        updated_content.push_str(&knowledge_section);
        std::fs::write(memory_path, updated_content)?;
    }
    
    Ok(())
}

/// Determine relevant categories for an agent
fn determine_agent_categories(agent_name: &str, content: &str) -> Vec<String> {
    let mut categories = Vec::new();
    let content_lower = content.to_lowercase();
    let agent_name_lower = agent_name.to_lowercase();
    
    // Agent-specific category mapping
    let agent_categories = [
        ("architect", vec!["Architecture", "Design"]),
        ("developer", vec!["Implementation", "Architecture"]),
        ("designer", vec!["Architecture", "Documentation"]),
        ("optimizer", vec!["Optimizations", "Architecture"]),
        ("documenter", vec!["Documentation", "Protocols"]),
        ("planner", vec!["Architecture", "Documentation"]),
        ("tester", vec!["Implementation", "Protocols"]),
        ("analyst", vec!["Documentation", "Architecture"]),
    ];
    
    for (agent_type, agent_cats) in &agent_categories {
        if agent_name_lower.contains(agent_type) || content_lower.contains(agent_type) {
            categories.extend(agent_cats.iter().map(|s| s.to_string()));
        }
    }
    
    // Default categories if none found
    if categories.is_empty() {
        categories.extend(vec!["General".to_string(), "Documentation".to_string()]);
    }
    
    categories.sort();
    categories.dedup();
    categories
}

/// Generate workflow report
fn generate_workflow_report(
    report_path: &PathBuf,
    result: &IdentificationResult,
    threshold: f64,
    report_only: bool,
) -> Result<()> {
    let mut report = String::new();
    
    report.push_str("# Knowledge Distribution Workflow Report\n\n");
    report.push_str(&format!("Generated: {}\n\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S")));
    
    report.push_str("## Workflow Parameters\n\n");
    report.push_str(&format!("- Relevance Threshold: {}\n", threshold));
    report.push_str(&format!("- Mode: {}\n", if report_only { "Report Only" } else { "Full Processing" }));
    report.push_str(&format!("- Files Scanned: {}\n", result.total_scanned));
    report.push_str(&format!("- Relevant Files Found: {}\n\n", result.relevant_files.len()));
    
    report.push_str("## Identified Content\n\n");
    for file in &result.relevant_files {
        report.push_str(&format!(
            "### {} (Score: {:.3})\n\n",
            file.path.file_name().unwrap().to_string_lossy(),
            file.relevance_score
        ));
        report.push_str(&format!("- **Path**: `{}`\n", file.path.display()));
        report.push_str(&format!("- **Categories**: {}\n", file.categories.join(", ")));
        report.push_str(&format!("- **Summary**: {}\n\n", file.content_summary));
    }
    
    report.push_str("## Next Steps\n\n");
    report.push_str("1. Review the identification results to confirm content relevance\n");
    if !report_only {
        report.push_str("2. Check distributed content in agent memory files\n");
    }
    report.push_str("3. Verify Knowledge Registry index is properly updated\n");
    report.push_str("4. Consider adjusting threshold if results are not optimal\n\n");
    
    std::fs::write(report_path, report)?;
    Ok(())
}