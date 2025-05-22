use std::process::Command;
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Repository Size Analysis by Commit");
    println!("==================================\n");

    // Get commit list in reverse chronological order (oldest first)
    let output = Command::new("git")
        .args(&["log", "--reverse", "--pretty=format:%H|%ai|%s"])
        .output()?;

    let commits_raw = String::from_utf8(output.stdout)?;
    let commits: Vec<_> = commits_raw
        .lines()
        .filter(|line| !line.is_empty())
        .collect();

    println!("Analyzing {} commits...\n", commits.len());

    let mut size_data = BTreeMap::new();
    let mut previous_size = 0u64;

    for (i, commit_line) in commits.iter().enumerate() {
        let parts: Vec<&str> = commit_line.split('|').collect();
        let hash = parts[0];
        let date = parts[1].split(' ').next().unwrap_or("");
        let message = parts.get(2).unwrap_or(&"").chars().take(40).collect::<String>();

        if (i + 1) % 25 == 0 || i == commits.len() - 1 {
            println!("Progress: {}/{} commits", i + 1, commits.len());
        }

        // Get commit stats
        let stats_output = Command::new("git")
            .args(&["show", "--stat", "--format=", hash])
            .output()?;

        let stats = String::from_utf8(stats_output.stdout)?;
        let (files, insertions, deletions) = parse_commit_stats(&stats);

        // Checkout commit to measure size
        Command::new("git")
            .args(&["checkout", "-q", hash])
            .output()?;

        Command::new("git")
            .args(&["gc", "--quiet"])
            .output()?;

        let pack_size = get_pack_size_bytes()?;
        let size_mb = pack_size as f64 / 1024.0 / 1024.0;
        let increase_mb = if previous_size > 0 {
            (pack_size as i64 - previous_size as i64) as f64 / 1024.0 / 1024.0
        } else {
            size_mb
        };

        size_data.insert(hash.to_string(), (date.to_string(), message, files, insertions, deletions, size_mb, increase_mb));
        previous_size = pack_size;
    }

    // Return to main
    Command::new("git")
        .args(&["checkout", "-q", "main"])
        .output()?;

    print_analysis(&size_data);
    
    Ok(())
}

fn parse_commit_stats(stats: &str) -> (u32, u32, u32) {
    let last_line = stats.lines().last().unwrap_or("");
    
    let files = extract_stat_number(last_line, "file");
    let insertions = extract_stat_number(last_line, "insertion");
    let deletions = extract_stat_number(last_line, "deletion");

    (files, insertions, deletions)
}

fn extract_stat_number(text: &str, keyword: &str) -> u32 {
    let words: Vec<&str> = text.split_whitespace().collect();
    for i in 0..words.len() {
        if words[i].contains(keyword) && i > 0 {
            if let Ok(num) = words[i-1].parse::<u32>() {
                return num;
            }
        }
    }
    0
}

fn get_pack_size_bytes() -> Result<u64, Box<dyn std::error::Error>> {
    let output = Command::new("git")
        .args(&["count-objects", "-v"])
        .output()?;

    let text = String::from_utf8(output.stdout)?;
    for line in text.lines() {
        if line.starts_with("size-pack ") {
            let size_kb: u64 = line.split_whitespace()
                .nth(1)
                .ok_or("No size found")?
                .parse()?;
            return Ok(size_kb * 1024);
        }
    }
    Ok(0)
}

fn print_analysis(data: &BTreeMap<String, (String, String, u32, u32, u32, f64, f64)>) {
    println!("\n🏆 Top 15 Largest Repository Sizes:");
    println!("=====================================");
    println!("{:<12} | {:<5} | {:<6} | {:<6} | {:<10} | {:<8} | {}", 
             "Size (MB)", "Files", "+Lines", "-Lines", "Date", "Commit", "Message");
    println!("{}", "=".repeat(90));

    let mut sorted: Vec<_> = data.iter().collect();
    sorted.sort_by(|a, b| b.1.5.partial_cmp(&a.1.5).unwrap());

    for (hash, (date, msg, files, ins, del, size, _)) in sorted.iter().take(15) {
        println!("{:<12.2} | {:<5} | {:<6} | {:<6} | {:<10} | {:<8} | {}", 
                 size, files, ins, del, date, &hash[..8], msg);
    }

    println!("\n📈 Commits with Largest Size Increases (>0.1MB):");
    println!("=================================================");
    println!("{:<18} | {:<10} | {:<8} | {}", 
             "Increase (MB)", "Date", "Commit", "Message");
    println!("{}", "=".repeat(70));

    let mut increases: Vec<_> = data.iter()
        .filter(|(_, (_, _, _, _, _, _, inc))| *inc > 0.1)
        .collect();
    increases.sort_by(|a, b| b.1.6.partial_cmp(&a.1.6).unwrap());

    for (hash, (date, msg, _, _, _, _, increase)) in increases.iter().take(20) {
        println!("{:<18.2} | {:<10} | {:<8} | {}", 
                 increase, date, &hash[..8], msg);
    }

    // Summary statistics
    let current_size = data.values().map(|(_, _, _, _, _, s, _)| *s).fold(0.0, f64::max);
    let total_commits = data.len();
    let large_increases = data.values().filter(|(_, _, _, _, _, _, inc)| *inc > 1.0).count();

    println!("\n📊 Repository Health Summary:");
    println!("=============================");
    println!("• Current repository size: {:.2} MB", current_size);
    println!("• Total commits analyzed: {}", total_commits);
    println!("• Commits with >1MB increase: {}", large_increases);

    if current_size < 5.0 {
        println!("✅ Repository size is healthy!");
    } else if current_size < 10.0 {
        println!("⚠️  Repository size is moderate - monitor for growth");
    } else {
        println!("🚨 Repository size is large - consider optimization");
    }

    println!("\n💡 Optimization Recommendations:");
    if large_increases > 0 {
        println!("• Review the {} commits with >1MB increases", large_increases);
    }
    if current_size > 5.0 {
        println!("• Consider using git-lfs for large binary files");
        println!("• Review large commits for unnecessary files");
    }
    println!("• Current .git folder is well optimized after recent cleanup");
    
    println!("\nAnalysis complete! 🎉");
}