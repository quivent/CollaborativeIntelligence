#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! chrono = "0.4"
//! serde = { version = "1.0", features = ["derive"] }
//! serde_json = "1.0"
//! ```

use std::process::{Command, Stdio};
use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct CommitAnalysis {
    hash: String,
    short_hash: String,
    author_date: String,
    commit_date: String,
    message: String,
    files_changed: u32,
    insertions: u32,
    deletions: u32,
    pack_size_bytes: u64,
    pack_size_mb: f64,
    size_increase_mb: f64,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Repository Size Analysis by Commit");
    println!("==================================");
    println!();

    // Get list of commits in chronological order (oldest first)
    let commits = get_commit_list()?;
    println!("Analyzing {} commits...", commits.len());
    println!();

    let mut analyses = Vec::new();
    let mut previous_size = 0u64;

    for (i, commit_hash) in commits.iter().enumerate() {
        if (i + 1) % 20 == 0 || i == commits.len() - 1 {
            println!("Progress: {}/{} commits analyzed", i + 1, commits.len());
        }

        let analysis = analyze_commit(commit_hash, previous_size)?;
        previous_size = analysis.pack_size_bytes;
        analyses.push(analysis);
    }

    // Return to main branch
    Command::new("git")
        .args(&["checkout", "-q", "main"])
        .output()?;

    println!();
    print_size_analysis(&analyses);
    print_bloat_analysis(&analyses);
    print_optimization_suggestions(&analyses);

    Ok(())
}

fn get_commit_list() -> Result<Vec<String>, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["log", "--reverse", "--pretty=format:%H"])
        .output()?;

    let commits = String::from_utf8(output.stdout)?
        .lines()
        .map(|line| line.trim().to_string())
        .filter(|line| !line.is_empty())
        .collect();

    Ok(commits)
}

fn analyze_commit(commit_hash: &str, previous_size: u64) -> Result<CommitAnalysis, Box<dyn std::error::Error>> {
    // Get commit metadata
    let metadata_output = Command::new("git")
        .args(&["show", "--format=%ai|%ci|%s", "--no-patch", commit_hash])
        .output()?;
    
    let metadata = String::from_utf8(metadata_output.stdout)?;
    let parts: Vec<&str> = metadata.split('|').collect();
    
    let author_date = parts.get(0).unwrap_or(&"").trim().to_string();
    let commit_date = parts.get(1).unwrap_or(&"").trim().to_string();
    let message = parts.get(2).unwrap_or(&"").trim().to_string();

    // Get commit stats
    let stats_output = Command::new("git")
        .args(&["show", "--stat", "--format=", commit_hash])
        .output()?;

    let stats = String::from_utf8(stats_output.stdout)?;
    let (files_changed, insertions, deletions) = parse_stats(&stats);

    // Checkout commit and get pack size
    Command::new("git")
        .args(&["checkout", "-q", commit_hash])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    // Force garbage collection to get accurate size
    Command::new("git")
        .args(&["gc", "--quiet"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()?;

    let pack_size_bytes = get_pack_size()?;
    let pack_size_mb = pack_size_bytes as f64 / 1024.0 / 1024.0;
    let size_increase_mb = if previous_size > 0 {
        (pack_size_bytes as i64 - previous_size as i64) as f64 / 1024.0 / 1024.0
    } else {
        pack_size_mb
    };

    Ok(CommitAnalysis {
        hash: commit_hash.to_string(),
        short_hash: commit_hash[..8].to_string(),
        author_date,
        commit_date,
        message: message.chars().take(50).collect(),
        files_changed,
        insertions,
        deletions,
        pack_size_bytes,
        pack_size_mb,
        size_increase_mb,
    })
}

fn parse_stats(stats: &str) -> (u32, u32, u32) {
    let last_line = stats.lines().last().unwrap_or("");
    
    let files_changed = extract_number(last_line, r"(\d+) files? changed").unwrap_or(0);
    let insertions = extract_number(last_line, r"(\d+) insertions?").unwrap_or(0);
    let deletions = extract_number(last_line, r"(\d+) deletions?").unwrap_or(0);

    (files_changed, insertions, deletions)
}

fn extract_number(text: &str, pattern: &str) -> Option<u32> {
    use std::str::FromStr;
    
    // Simple regex-like extraction
    if let Some(start) = text.find(pattern.split('(').nth(1)?.split(')').next()?) {
        let number_part = &text[..start];
        if let Some(num_start) = number_part.rfind(' ') {
            let num_str = &number_part[num_start + 1..];
            return u32::from_str(num_str).ok();
        }
    }
    
    // Fallback: look for numbers before keywords
    if text.contains("files changed") {
        if let Some(pos) = text.find(" files") {
            let before = &text[..pos];
            if let Some(space) = before.rfind(' ') {
                if let Ok(num) = before[space + 1..].parse::<u32>() {
                    return Some(num);
                }
            }
        }
    }
    
    None
}

fn get_pack_size() -> Result<u64, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["count-objects", "-v"])
        .output()?;

    let count_output = String::from_utf8(output.stdout)?;
    
    for line in count_output.lines() {
        if line.starts_with("size-pack ") {
            let size_kb: u64 = line.split_whitespace()
                .nth(1)
                .ok_or("Failed to parse size-pack")?
                .parse()?;
            return Ok(size_kb * 1024); // Convert KB to bytes
        }
    }

    Ok(0)
}

fn print_size_analysis(analyses: &[CommitAnalysis]) {
    println!("Top 15 Commits by Repository Size:");
    println!("==================================");
    println!();
    println!("{:<12} | {:<5} | {:<6} | {:<6} | {:<10} | {:<8} | {}", 
             "Size (MB)", "Files", "+Lines", "-Lines", "Date", "Commit", "Message");
    println!("{}", "-".repeat(80));

    let mut sorted_analyses = analyses.to_vec();
    sorted_analyses.sort_by(|a, b| b.pack_size_bytes.cmp(&a.pack_size_bytes));

    for analysis in sorted_analyses.iter().take(15) {
        let date = analysis.author_date.split(' ').next().unwrap_or("");
        println!("{:<12.2} | {:<5} | {:<6} | {:<6} | {:<10} | {:<8} | {}", 
                 analysis.pack_size_mb,
                 analysis.files_changed,
                 analysis.insertions,
                 analysis.deletions,
                 date,
                 analysis.short_hash,
                 analysis.message);
    }
    println!();
}

fn print_bloat_analysis(analyses: &[CommitAnalysis]) {
    println!("Commits with Largest Size Increases:");
    println!("====================================");
    println!();
    println!("{:<18} | {:<10} | {:<8} | {}", 
             "Size Increase (MB)", "Date", "Commit", "Message");
    println!("{}", "-".repeat(70));

    for analysis in analyses {
        if analysis.size_increase_mb > 0.1 {  // Only show increases > 100KB
            let date = analysis.author_date.split(' ').next().unwrap_or("");
            println!("{:<18.2} | {:<10} | {:<8} | {}", 
                     analysis.size_increase_mb,
                     date,
                     analysis.short_hash,
                     analysis.message);
        }
    }
    println!();
}

fn print_optimization_suggestions(analyses: &[CommitAnalysis]) {
    println!("Repository Optimization Analysis:");
    println!("================================");
    println!();

    let current_size = analyses.last().map(|a| a.pack_size_mb).unwrap_or(0.0);
    let total_commits = analyses.len();
    let large_commits = analyses.iter().filter(|a| a.size_increase_mb > 1.0).count();

    println!("📊 Repository Statistics:");
    println!("  • Current size: {:.2} MB", current_size);
    println!("  • Total commits analyzed: {}", total_commits);
    println!("  • Commits with >1MB increase: {}", large_commits);
    println!();

    println!("🔍 Size Distribution:");
    let size_ranges = [
        (0.0, 0.5, "< 0.5 MB"),
        (0.5, 1.0, "0.5-1.0 MB"), 
        (1.0, 2.0, "1.0-2.0 MB"),
        (2.0, 5.0, "2.0-5.0 MB"),
        (5.0, f64::MAX, "> 5.0 MB"),
    ];

    for (min, max, label) in &size_ranges {
        let count = analyses.iter()
            .filter(|a| a.pack_size_mb >= *min && a.pack_size_mb < *max)
            .count();
        if count > 0 {
            println!("  • {}: {} commits", label, count);
        }
    }
    println!();

    println!("💡 Optimization Recommendations:");
    
    if current_size < 5.0 {
        println!("  ✅ Repository size is healthy (< 5MB)");
    } else if current_size < 10.0 {
        println!("  ⚠️  Repository size is moderate. Consider:");
        println!("     - Review large commits for unnecessary files");
        println!("     - Use git-lfs for large binary files");
    } else {
        println!("  🚨 Repository size is large. Recommended actions:");
        println!("     - Use git filter-repo to remove large files");
        println!("     - Move large assets to git-lfs");
        println!("     - Consider splitting into multiple repositories");
    }

    if large_commits > 0 {
        println!("  • {} commits added >1MB each - review these for optimization", large_commits);
    }

    let avg_commit_size = analyses.iter()
        .map(|a| a.size_increase_mb)
        .filter(|&s| s > 0.0)
        .sum::<f64>() / analyses.len() as f64;
    
    if avg_commit_size > 0.1 {
        println!("  • Average commit size: {:.2}MB - consider smaller, more focused commits", avg_commit_size);
    }

    println!();
    println!("Analysis complete! 🎉");
}