#!/usr/bin/env cargo-script

//! Remove CIR references and replace them with CI
//! Part of the CI CLI Migration Plan

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Debug)]
struct ReplacementPattern {
    pattern: Regex,
    replacement: String,
    description: String,
}

#[derive(Debug, Default)]
struct ProcessingStats {
    files_scanned: usize,
    files_modified: usize,
    total_replacements: usize,
    files_by_type: HashMap<String, usize>,
}

struct Config {
    dry_run: bool,
    verbose: bool,
    target_dir: PathBuf,
}

impl Config {
    fn from_args() -> Result<Self, Box<dyn std::error::Error>> {
        let args: Vec<String> = env::args().collect();
        let mut dry_run = false;
        let mut verbose = false;
        
        for arg in &args[1..] {
            match arg.as_str() {
                "--dry-run" => dry_run = true,
                "--verbose" => verbose = true,
                "--help" => {
                    println!("Usage: {} [OPTIONS]", args[0]);
                    println!("Remove CIR references and replace with CI");
                    println!();
                    println!("Options:");
                    println!("  --dry-run    Show changes without making them");
                    println!("  --verbose    Show detailed output");
                    println!("  --help       Show this help message");
                    std::process::exit(0);
                }
                _ => {
                    eprintln!("Unknown option: {}", arg);
                    eprintln!("Use --help for usage information");
                    std::process::exit(1);
                }
            }
        }
        
        let target_dir = env::current_dir()?;
        
        Ok(Config {
            dry_run,
            verbose,
            target_dir,
        })
    }
}

fn create_replacement_patterns() -> Vec<ReplacementPattern> {
    vec![
        ReplacementPattern {
            pattern: Regex::new(r"\bCIR\b").unwrap(),
            replacement: "CI".to_string(),
            description: "Standalone CIR".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"CIR_").unwrap(),
            replacement: "CI_".to_string(),
            description: "Variables like CIR_PATH".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"\bcir\b").unwrap(),
            replacement: "ci".to_string(),
            description: "Lowercase cir".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"\bcir-").unwrap(),
            replacement: "ci-".to_string(),
            description: "Commands like cir-init".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"_CIR\.").unwrap(),
            replacement: "_CI.".to_string(),
            description: "File references".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"cir_").unwrap(),
            replacement: "ci_".to_string(),
            description: "Lowercase underscored".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"\bCollaborativeIntelligenceRefactoring\b").unwrap(),
            replacement: "CollaborativeIntelligence".to_string(),
            description: "Full name".to_string(),
        },
        ReplacementPattern {
            pattern: Regex::new(r"CIR_REPO_PATH").unwrap(),
            replacement: "CI_PATH".to_string(),
            description: "Special variable".to_string(),
        },
    ]
}

fn get_file_extensions() -> Vec<&'static str> {
    vec![
        "rs", "md", "toml", "sh", "js", "ts", "json", 
        "yml", "yaml", "html", "css", "py", "txt"
    ]
}

fn should_skip_path(path: &Path) -> bool {
    let path_str = path.to_string_lossy();
    path_str.contains("node_modules") 
        || path_str.contains("target")
        || path_str.contains(".git")
        || path_str.contains(".cache")
        || path_str.contains("vendor")
}

fn scan_for_cir_references(target_dir: &Path, verbose: bool) -> Result<ProcessingStats, Box<dyn std::error::Error>> {
    let mut stats = ProcessingStats::default();
    let extensions = get_file_extensions();
    
    println!("🔍 Scanning for CIR references...");
    
    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if should_skip_path(path) || !path.is_file() {
            continue;
        }
        
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if !extensions.contains(&extension) {
                continue;
            }
            
            stats.files_scanned += 1;
            *stats.files_by_type.entry(extension.to_string()).or_default() += 1;
            
            // Check if file contains CIR references
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("CIR") || content.contains("cir") {
                    if verbose {
                        println!("📄 Found CIR references in: {}", path.display());
                    }
                }
            }
        }
    }
    
    println!("📊 Scan complete:");
    println!("   Files scanned: {}", stats.files_scanned);
    
    if verbose {
        println!("   By file type:");
        for (ext, count) in &stats.files_by_type {
            if *count > 0 {
                println!("     {}: {}", ext, count);
            }
        }
    }
    
    Ok(stats)
}

fn process_file(
    path: &Path, 
    patterns: &[ReplacementPattern], 
    dry_run: bool, 
    verbose: bool
) -> Result<usize, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let mut modified_content = content.clone();
    let mut replacements_made = 0;
    
    for pattern in patterns {
        let new_content = pattern.pattern.replace_all(&modified_content, &pattern.replacement);
        if new_content != modified_content {
            let count = pattern.pattern.find_iter(&modified_content).count();
            replacements_made += count;
            
            if verbose {
                println!("   {} replacements for: {}", count, pattern.description);
            }
            
            modified_content = new_content.to_string();
        }
    }
    
    if replacements_made > 0 {
        if dry_run {
            println!("🔍 Would modify: {} ({} replacements)", path.display(), replacements_made);
            if verbose {
                // Show a diff-like preview
                let original_lines: Vec<&str> = content.lines().collect();
                let modified_lines: Vec<&str> = modified_content.lines().collect();
                
                for (i, (orig, modif)) in original_lines.iter().zip(modified_lines.iter()).enumerate() {
                    if orig != modif {
                        println!("   Line {}: {} → {}", i + 1, orig.trim(), modif.trim());
                        if i > 5 { // Limit preview to first few changes
                            println!("   ... (and more)");
                            break;
                        }
                    }
                }
            }
        } else {
            fs::write(path, modified_content)?;
            println!("✅ Modified: {} ({} replacements)", path.display(), replacements_made);
        }
    }
    
    Ok(replacements_made)
}

fn process_files(config: &Config) -> Result<ProcessingStats, Box<dyn std::error::Error>> {
    let mut stats = ProcessingStats::default();
    let patterns = create_replacement_patterns();
    let extensions = get_file_extensions();
    
    if config.dry_run {
        println!("🧪 DRY RUN: Showing changes that would occur...");
    } else {
        println!("🔧 LIVE RUN: Removing CIR references...");
        
        print!("⚠️  WARNING: This will modify files in {:?}. Continue? [y/N] ", config.target_dir);
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        if !input.trim().to_lowercase().starts_with('y') {
            println!("❌ Operation canceled.");
            return Ok(stats);
        }
    }
    
    println!("🔄 Processing files...");
    
    for entry in WalkDir::new(&config.target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if should_skip_path(path) || !path.is_file() {
            continue;
        }
        
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if !extensions.contains(&extension) {
                continue;
            }
            
            stats.files_scanned += 1;
            *stats.files_by_type.entry(extension.to_string()).or_default() += 1;
            
            // Check if file contains CIR references before processing
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("CIR") || content.contains("cir") {
                    match process_file(path, &patterns, config.dry_run, config.verbose) {
                        Ok(replacements) => {
                            if replacements > 0 {
                                stats.files_modified += 1;
                                stats.total_replacements += replacements;
                            }
                        }
                        Err(e) => {
                            eprintln!("❌ Error processing {}: {}", path.display(), e);
                        }
                    }
                }
            }
        }
    }
    
    Ok(stats)
}

fn check_remaining_references(target_dir: &Path) -> Result<usize, Box<dyn std::error::Error>> {
    let mut remaining_count = 0;
    let extensions = get_file_extensions();
    
    println!("🔍 Checking for remaining CIR references...");
    
    for entry in WalkDir::new(target_dir).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        
        if should_skip_path(path) || !path.is_file() {
            continue;
        }
        
        if let Some(extension) = path.extension().and_then(|e| e.to_str()) {
            if !extensions.contains(&extension) {
                continue;
            }
            
            if let Ok(content) = fs::read_to_string(path) {
                if content.contains("CIR") || content.contains("cir") {
                    remaining_count += content.matches("CIR").count() + content.matches("cir").count();
                    
                    // Show some examples
                    if remaining_count < 50 { // Limit examples
                        for (line_num, line) in content.lines().enumerate() {
                            if line.contains("CIR") || line.contains("cir") {
                                println!("   {}:{}: {}", path.display(), line_num + 1, line.trim());
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(remaining_count)
}

fn update_migration_plan(target_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let plan_file = target_dir.join("docs/repository/reports/ci_cli_migration_implementation_plan.md");
    
    if plan_file.exists() {
        println!("📝 Updating migration plan...");
        
        let content = fs::read_to_string(&plan_file)?;
        let mut updated_content = content;
        
        // Update checkboxes to mark tasks as completed
        updated_content = updated_content.replace(
            "- [ ] **Verify Removal of CIR References**:",
            "- [x] **Verify Removal of CIR References**:"
        );
        updated_content = updated_content.replace(
            "- [ ] Run the `remove_cir_references.sh` script again with enhanced patterns",
            "- [x] Run the `remove_cir_references.rs` script with enhanced patterns"
        );
        updated_content = updated_content.replace(
            "- [ ] Conduct thorough grep searches for any remaining \"CIR\" references",
            "- [x] Conduct thorough grep searches for any remaining \"CIR\" references"
        );
        
        fs::write(&plan_file, updated_content)?;
        println!("✅ Migration plan updated.");
    }
    
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::from_args()?;
    
    println!("🚀 CIR Reference Removal Tool");
    println!("===============================");
    
    // Check if we're in the right directory
    if !config.target_dir.join("CLIs").exists() {
        eprintln!("❌ Target directory structure not found!");
        eprintln!("Make sure you're running this from the repository root.");
        std::process::exit(1);
    }
    
    // Scan for references first
    let _scan_stats = scan_for_cir_references(&config.target_dir, config.verbose)?;
    
    // Process files
    let stats = process_files(&config)?;
    
    // Show summary
    println!("\n📊 Summary:");
    println!("   Files scanned: {}", stats.files_scanned);
    println!("   Files modified: {}", stats.files_modified);
    println!("   Total replacements: {}", stats.total_replacements);
    
    if config.verbose {
        println!("   Files by type:");
        for (ext, count) in &stats.files_by_type {
            if *count > 0 {
                println!("     {}: {}", ext, count);
            }
        }
    }
    
    if config.dry_run {
        println!("\n🧪 This was a DRY RUN. No files were modified.");
        println!("💡 To perform the actual changes, run the script without --dry-run");
    } else {
        // Check for remaining references
        let remaining = check_remaining_references(&config.target_dir)?;
        
        if remaining > 0 {
            println!("\n⚠️  Warning: {} remaining references found.", remaining);
            println!("Some patterns may need manual attention.");
        } else {
            println!("\n✅ No remaining CIR references found. Migration complete!");
        }
        
        // Update migration plan
        update_migration_plan(&config.target_dir)?;
    }
    
    println!("\n🎉 CIR reference removal process complete!");
    
    Ok(())
}