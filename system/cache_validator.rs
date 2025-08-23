use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, Duration};
use serde::{Deserialize, Serialize};
use crate::instruction_cache::InstructionCache;
use crate::cache_persistence::PersistentCacheManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct ValidationResult {
    pub is_valid: bool,
    pub validation_timestamp: SystemTime,
    pub issues: Vec<ValidationIssue>,
    pub recommendations: Vec<String>,
    pub cache_health_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidationIssue {
    pub severity: IssueSeverity,
    pub issue_type: IssueType,
    pub description: String,
    pub affected_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueType {
    FileModified,
    FileMissing,
    HashMismatch,
    CacheCorrupted,
    CacheExpired,
    PermissionDenied,
}

pub struct CacheValidator {
    cache_manager: PersistentCacheManager,
    max_cache_age_hours: u64,
    file_check_interval: Duration,
}

impl CacheValidator {
    pub fn new(cache_dir: Option<&str>) -> Self {
        Self {
            cache_manager: PersistentCacheManager::new(cache_dir),
            max_cache_age_hours: 24, // Cache expires after 24 hours
            file_check_interval: Duration::from_secs(300), // Check files every 5 minutes
        }
    }

    pub fn validate_cache_integrity(&self, cache: &InstructionCache) -> ValidationResult {
        let mut issues = Vec::new();
        let mut recommendations = Vec::new();
        let validation_timestamp = SystemTime::now();

        // Check file modifications
        self.check_file_modifications(cache, &mut issues, &mut recommendations);
        
        // Check cache age
        self.check_cache_age(cache, &mut issues, &mut recommendations);
        
        // Check cache completeness
        self.check_cache_completeness(cache, &mut issues, &mut recommendations);
        
        // Check for permission issues
        self.check_file_permissions(&cache.file_hashes, &mut issues, &mut recommendations);

        let cache_health_score = self.calculate_health_score(&issues);
        let is_valid = cache_health_score >= 0.8 && !issues.iter().any(|i| matches!(i.severity, IssueSeverity::Critical));

        ValidationResult {
            is_valid,
            validation_timestamp,
            issues,
            recommendations,
            cache_health_score,
        }
    }

    fn check_file_modifications(&self, cache: &InstructionCache, issues: &mut Vec<ValidationIssue>, recommendations: &mut Vec<String>) {
        for (file_path, cached_hash) in &cache.file_hashes {
            match fs::read_to_string(file_path) {
                Ok(content) => {
                    let current_hash = InstructionCache::compute_file_hash(&content);
                    if current_hash != *cached_hash {
                        issues.push(ValidationIssue {
                            severity: IssueSeverity::High,
                            issue_type: IssueType::FileModified,
                            description: format!("File {} has been modified since caching", file_path),
                            affected_files: vec![file_path.clone()],
                        });
                        recommendations.push(format!("Update cache for modified file: {}", file_path));
                    }
                }
                Err(_) => {
                    issues.push(ValidationIssue {
                        severity: IssueSeverity::Critical,
                        issue_type: IssueType::FileMissing,
                        description: format!("Cached file {} is no longer accessible", file_path),
                        affected_files: vec![file_path.clone()],
                    });
                    recommendations.push(format!("Remove missing file from cache: {}", file_path));
                }
            }
        }
    }

    fn check_cache_age(&self, cache: &InstructionCache, issues: &mut Vec<ValidationIssue>, recommendations: &mut Vec<String>) {
        if let Ok(age) = SystemTime::now().duration_since(cache.last_modified) {
            let age_hours = age.as_secs() / 3600;
            
            if age_hours > self.max_cache_age_hours {
                issues.push(ValidationIssue {
                    severity: IssueSeverity::Medium,
                    issue_type: IssueType::CacheExpired,
                    description: format!("Cache is {} hours old (max: {} hours)", age_hours, self.max_cache_age_hours),
                    affected_files: vec![],
                });
                recommendations.push("Refresh cache due to age".to_string());
            }
        }
    }

    fn check_cache_completeness(&self, cache: &InstructionCache, issues: &mut Vec<ValidationIssue>, recommendations: &mut Vec<String>) {
        let expected_files = vec![
            "/Users/joshkornreich/.claude/CLAUDE.md",
            "/Users/joshkornreich/Documents/Projects/CLAUDE.md",
            "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md",
        ];

        let mut missing_files = Vec::new();
        for expected_file in expected_files {
            if !cache.file_hashes.contains_key(expected_file) {
                if Path::new(expected_file).exists() {
                    missing_files.push(expected_file.to_string());
                }
            }
        }

        if !missing_files.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::Medium,
                issue_type: IssueType::FileModified,
                description: "Cache is missing some expected files".to_string(),
                affected_files: missing_files.clone(),
            });
            recommendations.push("Add missing files to cache".to_string());
        }

        // Check for empty directives
        if cache.compiled_directives.is_empty() {
            issues.push(ValidationIssue {
                severity: IssueSeverity::High,
                issue_type: IssueType::CacheCorrupted,
                description: "Cache has no compiled directives".to_string(),
                affected_files: vec![],
            });
            recommendations.push("Recompile cache directives".to_string());
        }
    }

    fn check_file_permissions(&self, file_hashes: &HashMap<String, String>, issues: &mut Vec<ValidationIssue>, recommendations: &mut Vec<String>) {
        for file_path in file_hashes.keys() {
            match fs::metadata(file_path) {
                Ok(metadata) => {
                    if metadata.permissions().readonly() {
                        // This is actually expected for CLAUDE.md files, but note it
                        continue;
                    }
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::PermissionDenied {
                        issues.push(ValidationIssue {
                            severity: IssueSeverity::High,
                            issue_type: IssueType::PermissionDenied,
                            description: format!("Permission denied accessing file: {}", file_path),
                            affected_files: vec![file_path.clone()],
                        });
                        recommendations.push(format!("Check file permissions for: {}", file_path));
                    }
                }
            }
        }
    }

    fn calculate_health_score(&self, issues: &[ValidationIssue]) -> f32 {
        if issues.is_empty() {
            return 1.0;
        }

        let total_penalty: f32 = issues.iter().map(|issue| {
            match issue.severity {
                IssueSeverity::Low => 0.1,
                IssueSeverity::Medium => 0.2,
                IssueSeverity::High => 0.4,
                IssueSeverity::Critical => 0.8,
            }
        }).sum();

        (1.0 - total_penalty).max(0.0)
    }

    pub async fn incremental_validation(&self, cache: &InstructionCache) -> ValidationResult {
        // Fast validation - only check modified times, not full content hashes
        let mut issues = Vec::new();
        let recommendations = Vec::new();
        let validation_timestamp = SystemTime::now();

        for file_path in cache.file_hashes.keys() {
            if let Ok(metadata) = fs::metadata(file_path) {
                if let Ok(modified_time) = metadata.modified() {
                    if modified_time > cache.last_modified {
                        issues.push(ValidationIssue {
                            severity: IssueSeverity::Medium,
                            issue_type: IssueType::FileModified,
                            description: format!("File {} modified after cache creation", file_path),
                            affected_files: vec![file_path.clone()],
                        });
                    }
                }
            }
        }

        let cache_health_score = self.calculate_health_score(&issues);
        let is_valid = cache_health_score >= 0.8;

        ValidationResult {
            is_valid,
            validation_timestamp,
            issues,
            recommendations,
            cache_health_score,
        }
    }

    pub fn auto_repair_cache(&self, cache: &mut InstructionCache, validation_result: &ValidationResult) -> RepairResult {
        let mut repaired_issues = Vec::new();
        let mut failed_repairs = Vec::new();

        for issue in &validation_result.issues {
            match issue.issue_type {
                IssueType::FileModified => {
                    if let Some(file_path) = issue.affected_files.first() {
                        match self.repair_modified_file(cache, file_path) {
                            Ok(_) => repaired_issues.push(issue.clone()),
                            Err(e) => failed_repairs.push((issue.clone(), e.to_string())),
                        }
                    }
                }
                IssueType::FileMissing => {
                    if let Some(file_path) = issue.affected_files.first() {
                        cache.file_hashes.remove(file_path);
                        repaired_issues.push(issue.clone());
                    }
                }
                IssueType::CacheCorrupted => {
                    // This requires full recompilation
                    failed_repairs.push((issue.clone(), "Requires manual cache rebuild".to_string()));
                }
                _ => {
                    // Other issues require manual intervention
                    failed_repairs.push((issue.clone(), "Manual intervention required".to_string()));
                }
            }
        }

        RepairResult {
            repaired_count: repaired_issues.len(),
            failed_count: failed_repairs.len(),
            repaired_issues,
            failed_repairs,
        }
    }

    fn repair_modified_file(&self, cache: &mut InstructionCache, file_path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(file_path)?;
        let new_hash = InstructionCache::compute_file_hash(&content);
        cache.file_hashes.insert(file_path.to_string(), new_hash);
        cache.last_modified = SystemTime::now();
        Ok(())
    }

    pub fn get_validation_report(&self, validation_result: &ValidationResult) -> String {
        let mut report = format!(
            "🔍 Cache Validation Report\n\
            ========================\n\
            • Status: {}\n\
            • Health Score: {:.1}%\n\
            • Issues Found: {}\n\
            • Validation Time: {:?}\n\n",
            if validation_result.is_valid { "✅ VALID" } else { "❌ INVALID" },
            validation_result.cache_health_score * 100.0,
            validation_result.issues.len(),
            validation_result.validation_timestamp
        );

        if !validation_result.issues.is_empty() {
            report.push_str("Issues Detected:\n");
            for (i, issue) in validation_result.issues.iter().enumerate() {
                let severity_icon = match issue.severity {
                    IssueSeverity::Low => "🟡",
                    IssueSeverity::Medium => "🟠", 
                    IssueSeverity::High => "🔴",
                    IssueSeverity::Critical => "⚠️",
                };
                report.push_str(&format!("{}. {} {} - {}\n", i + 1, severity_icon, 
                                       format!("{:?}", issue.issue_type), issue.description));
            }
        }

        if !validation_result.recommendations.is_empty() {
            report.push_str("\nRecommendations:\n");
            for (i, recommendation) in validation_result.recommendations.iter().enumerate() {
                report.push_str(&format!("{}. {}\n", i + 1, recommendation));
            }
        }

        report
    }
}

#[derive(Debug)]
pub struct RepairResult {
    pub repaired_count: usize,
    pub failed_count: usize,
    pub repaired_issues: Vec<ValidationIssue>,
    pub failed_repairs: Vec<(ValidationIssue, String)>,
}

impl RepairResult {
    pub fn format_summary(&self) -> String {
        format!(
            "🔧 Auto-Repair Results:\n\
            • Successfully repaired: {} issues\n\
            • Failed to repair: {} issues\n\
            • Success rate: {:.1}%",
            self.repaired_count,
            self.failed_count,
            if self.repaired_count + self.failed_count > 0 {
                self.repaired_count as f32 / (self.repaired_count + self.failed_count) as f32 * 100.0
            } else {
                100.0
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_validation() {
        let temp_dir = tempdir().unwrap();
        let validator = CacheValidator::new(Some(temp_dir.path().to_str().unwrap()));
        
        let mut cache = InstructionCache::new();
        cache.file_hashes.insert("nonexistent.md".to_string(), "fake_hash".to_string());
        
        let result = validator.validate_cache_integrity(&cache);
        
        assert!(!result.is_valid);
        assert!(!result.issues.is_empty());
        assert!(result.cache_health_score < 1.0);
        
        // Should have issues for missing files
        assert!(result.issues.iter().any(|i| matches!(i.issue_type, IssueType::FileMissing)));
    }

    #[test]
    fn test_health_score_calculation() {
        let validator = CacheValidator::new(None);
        
        let issues = vec![
            ValidationIssue {
                severity: IssueSeverity::Low,
                issue_type: IssueType::FileModified,
                description: "Test".to_string(),
                affected_files: vec![],
            },
            ValidationIssue {
                severity: IssueSeverity::High,
                issue_type: IssueType::CacheCorrupted,
                description: "Test".to_string(),
                affected_files: vec![],
            },
        ];

        let score = validator.calculate_health_score(&issues);
        assert!(score < 1.0);
        assert!(score >= 0.0);
        
        // Low (0.1) + High (0.4) = 0.5 penalty, so score should be 0.5
        assert!((score - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_auto_repair() {
        let temp_dir = tempdir().unwrap();
        let validator = CacheValidator::new(Some(temp_dir.path().to_str().unwrap()));
        
        let mut cache = InstructionCache::new();
        cache.file_hashes.insert("missing.md".to_string(), "hash".to_string());
        
        let validation_result = ValidationResult {
            is_valid: false,
            validation_timestamp: SystemTime::now(),
            issues: vec![ValidationIssue {
                severity: IssueSeverity::Critical,
                issue_type: IssueType::FileMissing,
                description: "File missing".to_string(),
                affected_files: vec!["missing.md".to_string()],
            }],
            recommendations: vec![],
            cache_health_score: 0.2,
        };

        let repair_result = validator.auto_repair_cache(&mut cache, &validation_result);
        
        assert_eq!(repair_result.repaired_count, 1);
        assert!(!cache.file_hashes.contains_key("missing.md"));
    }
}