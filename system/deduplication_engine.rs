use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use crate::instruction_cache::{InstructionDirective, DirectiveCategory};

#[derive(Debug, Serialize, Deserialize)]
pub struct DeduplicationEngine {
    directive_signatures: HashMap<String, DirectiveSignature>,
    redundancy_map: HashMap<String, Vec<String>>,
    consolidation_rules: Vec<ConsolidationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirectiveSignature {
    pub content_hash: String,
    pub semantic_fingerprint: Vec<String>,
    pub category: DirectiveCategory,
    pub priority_weight: f32,
    pub source_files: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConsolidationRule {
    pub rule_id: String,
    pub pattern: String,
    pub consolidation_strategy: ConsolidationStrategy,
    pub weight_multiplier: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConsolidationStrategy {
    KeepHighestPriority,
    MergeContent,
    TakeFirst,
    CustomLogic(String),
}

#[derive(Debug)]
pub struct DeduplicationResult {
    pub original_count: usize,
    pub deduplicated_count: usize,
    pub redundancy_eliminated: usize,
    pub consolidated_directives: Vec<InstructionDirective>,
    pub redundancy_report: RedundancyReport,
}

#[derive(Debug)]
pub struct RedundancyReport {
    pub authority_duplicates: usize,
    pub todo_processing_duplicates: usize, 
    pub cli_installation_duplicates: usize,
    pub code_style_duplicates: usize,
    pub total_token_savings: usize,
}

impl DeduplicationEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            directive_signatures: HashMap::new(),
            redundancy_map: HashMap::new(),
            consolidation_rules: Vec::new(),
        };
        
        engine.initialize_consolidation_rules();
        engine
    }

    fn initialize_consolidation_rules(&mut self) {
        self.consolidation_rules = vec![
            ConsolidationRule {
                rule_id: "authority_consolidation".to_string(),
                pattern: r"(?i)(authority|command|general|autonomy)".to_string(),
                consolidation_strategy: ConsolidationStrategy::KeepHighestPriority,
                weight_multiplier: 0.8,
            },
            ConsolidationRule {
                rule_id: "todo_processing_consolidation".to_string(),
                pattern: r"(?i)(todo.*processing|critical.*override)".to_string(),
                consolidation_strategy: ConsolidationStrategy::TakeFirst,
                weight_multiplier: 0.9,
            },
            ConsolidationRule {
                rule_id: "cli_installation_consolidation".to_string(),
                pattern: r"(?i)(cli.*install|cargo.*install|global.*install)".to_string(),
                consolidation_strategy: ConsolidationStrategy::MergeContent,
                weight_multiplier: 0.85,
            },
            ConsolidationRule {
                rule_id: "code_style_consolidation".to_string(),
                pattern: r"(?i)(code.*style|formatting|convention)".to_string(),
                consolidation_strategy: ConsolidationStrategy::KeepHighestPriority,
                weight_multiplier: 0.7,
            },
        ];
    }

    pub fn analyze_and_deduplicate(&mut self, directives: &[InstructionDirective]) -> DeduplicationResult {
        let original_count = directives.len();
        
        // Build signatures for all directives
        self.build_directive_signatures(directives);
        
        // Identify redundancies
        let redundancy_groups = self.identify_redundancy_groups();
        
        // Apply consolidation rules
        let consolidated_directives = self.consolidate_directives(directives, &redundancy_groups);
        
        let deduplicated_count = consolidated_directives.len();
        let redundancy_eliminated = original_count - deduplicated_count;
        
        let redundancy_report = self.generate_redundancy_report(&redundancy_groups);
        
        DeduplicationResult {
            original_count,
            deduplicated_count,
            redundancy_eliminated,
            consolidated_directives,
            redundancy_report,
        }
    }

    fn build_directive_signatures(&mut self, directives: &[InstructionDirective]) {
        for directive in directives {
            let signature = DirectiveSignature {
                content_hash: self.compute_content_hash(&directive.content),
                semantic_fingerprint: self.extract_semantic_fingerprint(&directive.content),
                category: directive.category.clone(),
                priority_weight: directive.priority as f32,
                source_files: vec![], // Would be populated with actual source tracking
            };
            
            self.directive_signatures.insert(directive.id.clone(), signature);
        }
    }

    fn compute_content_hash(&self, content: &str) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        
        // Normalize content for semantic hashing
        let normalized = content
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric() || c.is_whitespace())
            .collect::<String>();
            
        hasher.update(normalized.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    fn extract_semantic_fingerprint(&self, content: &str) -> Vec<String> {
        let keywords = vec![
            "authority", "command", "general", "autonomy", "agent",
            "todo", "processing", "critical", "override", "immediate",
            "cli", "install", "cargo", "global", "build",
            "code", "style", "formatting", "convention",
            "identity", "neo", "matrix", "escape",
            "bash", "script", "prohibition", "never",
            "file", "reading", "visibility", "appendix",
        ];
        
        let content_lower = content.to_lowercase();
        keywords
            .into_iter()
            .filter(|keyword| content_lower.contains(keyword))
            .map(|s| s.to_string())
            .collect()
    }

    fn identify_redundancy_groups(&self) -> HashMap<String, Vec<String>> {
        let mut redundancy_groups: HashMap<String, Vec<String>> = HashMap::new();
        
        for (directive_id, signature) in &self.directive_signatures {
            // Group by semantic fingerprint similarity
            let fingerprint_key = signature.semantic_fingerprint.join("_");
            
            redundancy_groups
                .entry(fingerprint_key)
                .or_insert_with(Vec::new)
                .push(directive_id.clone());
        }
        
        // Filter out groups with only one member (not redundant)
        redundancy_groups.retain(|_, group| group.len() > 1);
        
        redundancy_groups
    }

    fn consolidate_directives(
        &self,
        directives: &[InstructionDirective],
        redundancy_groups: &HashMap<String, Vec<String>>,
    ) -> Vec<InstructionDirective> {
        let mut consolidated = Vec::new();
        let mut processed_ids = HashSet::new();
        
        // Process redundancy groups first
        for (_group_key, group_ids) in redundancy_groups {
            if group_ids.len() <= 1 {
                continue;
            }
            
            // Find the best consolidation strategy for this group
            let representative_directive = self.select_representative_directive(directives, group_ids);
            
            if let Some(directive) = representative_directive {
                consolidated.push(directive);
                for id in group_ids {
                    processed_ids.insert(id.clone());
                }
            }
        }
        
        // Add non-redundant directives
        for directive in directives {
            if !processed_ids.contains(&directive.id) {
                consolidated.push(directive.clone());
            }
        }
        
        // Sort by priority
        consolidated.sort_by_key(|d| d.priority);
        
        consolidated
    }

    fn select_representative_directive(
        &self,
        directives: &[InstructionDirective],
        group_ids: &[String],
    ) -> Option<InstructionDirective> {
        let group_directives: Vec<&InstructionDirective> = directives
            .iter()
            .filter(|d| group_ids.contains(&d.id))
            .collect();
        
        if group_directives.is_empty() {
            return None;
        }
        
        // Find applicable consolidation rule
        let applicable_rule = self.find_applicable_rule(&group_directives[0].content);
        
        match applicable_rule.map(|r| &r.consolidation_strategy) {
            Some(ConsolidationStrategy::KeepHighestPriority) => {
                group_directives.into_iter().min_by_key(|d| d.priority).cloned()
            }
            Some(ConsolidationStrategy::TakeFirst) => {
                group_directives.into_iter().next().cloned()
            }
            Some(ConsolidationStrategy::MergeContent) => {
                self.merge_directive_contents(&group_directives)
            }
            _ => group_directives.into_iter().next().cloned(),
        }
    }

    fn find_applicable_rule(&self, content: &str) -> Option<&ConsolidationRule> {
        use regex::Regex;
        
        for rule in &self.consolidation_rules {
            if let Ok(regex) = Regex::new(&rule.pattern) {
                if regex.is_match(content) {
                    return Some(rule);
                }
            }
        }
        None
    }

    fn merge_directive_contents(&self, directives: &[&InstructionDirective]) -> Option<InstructionDirective> {
        if directives.is_empty() {
            return None;
        }
        
        let base_directive = directives[0];
        let mut merged_content = base_directive.content.clone();
        
        // Add unique content from other directives
        for directive in &directives[1..] {
            if !merged_content.contains(&directive.content) {
                merged_content.push_str(" | ");
                merged_content.push_str(&directive.content);
            }
        }
        
        Some(InstructionDirective {
            id: format!("merged_{}", base_directive.id),
            content: merged_content,
            priority: base_directive.priority,
            category: base_directive.category.clone(),
        })
    }

    fn generate_redundancy_report(&self, redundancy_groups: &HashMap<String, Vec<String>>) -> RedundancyReport {
        let mut authority_duplicates = 0;
        let mut todo_processing_duplicates = 0;
        let mut cli_installation_duplicates = 0;
        let mut code_style_duplicates = 0;
        
        for (group_key, group_ids) in redundancy_groups {
            let duplicate_count = group_ids.len().saturating_sub(1);
            
            if group_key.contains("authority") || group_key.contains("command") {
                authority_duplicates += duplicate_count;
            } else if group_key.contains("todo") || group_key.contains("processing") {
                todo_processing_duplicates += duplicate_count;
            } else if group_key.contains("cli") || group_key.contains("install") {
                cli_installation_duplicates += duplicate_count;
            } else if group_key.contains("code") || group_key.contains("style") {
                code_style_duplicates += duplicate_count;
            }
        }
        
        let total_eliminated = authority_duplicates + todo_processing_duplicates + 
                              cli_installation_duplicates + code_style_duplicates;
        
        RedundancyReport {
            authority_duplicates,
            todo_processing_duplicates,
            cli_installation_duplicates, 
            code_style_duplicates,
            total_token_savings: total_eliminated * 50, // Estimated 50 tokens per directive
        }
    }

    pub fn get_optimization_summary(&self, result: &DeduplicationResult) -> String {
        format!(
            "🔧 Deduplication Engine Results:\n\
            • Original directives: {}\n\
            • After deduplication: {}\n\
            • Redundancy eliminated: {} ({:.1}%)\n\
            • Authority duplicates removed: {}\n\
            • Todo processing duplicates removed: {}\n\
            • CLI installation duplicates removed: {}\n\
            • Code style duplicates removed: {}\n\
            • Estimated token savings: {} tokens\n\
            • Performance improvement: ~{:.0}% faster startup",
            result.original_count,
            result.deduplicated_count,
            result.redundancy_eliminated,
            (result.redundancy_eliminated as f32 / result.original_count as f32) * 100.0,
            result.redundancy_report.authority_duplicates,
            result.redundancy_report.todo_processing_duplicates,
            result.redundancy_report.cli_installation_duplicates,
            result.redundancy_report.code_style_duplicates,
            result.redundancy_report.total_token_savings,
            (result.redundancy_eliminated as f32 / result.original_count as f32) * 100.0 * 0.8
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deduplication_engine() {
        let mut engine = DeduplicationEngine::new();
        
        let test_directives = vec![
            InstructionDirective {
                id: "auth1".to_string(),
                content: "Acknowledge General's supreme command authority".to_string(),
                priority: 1,
                category: DirectiveCategory::Authority,
            },
            InstructionDirective {
                id: "auth2".to_string(),
                content: "Agent Autonomy Policy - acknowledge authority".to_string(),
                priority: 2,
                category: DirectiveCategory::Authority,
            },
            InstructionDirective {
                id: "todo1".to_string(),
                content: "Todo Processing CRITICAL OVERRIDE".to_string(),
                priority: 3,
                category: DirectiveCategory::TodoProcessing,
            },
            InstructionDirective {
                id: "todo2".to_string(),
                content: "Todo Processing CRITICAL OVERRIDE - identical".to_string(),
                priority: 3,
                category: DirectiveCategory::TodoProcessing,
            },
        ];

        let result = engine.analyze_and_deduplicate(&test_directives);
        
        assert_eq!(result.original_count, 4);
        assert!(result.deduplicated_count < 4);
        assert!(result.redundancy_eliminated > 0);
        assert!(result.redundancy_report.authority_duplicates > 0 || 
                result.redundancy_report.todo_processing_duplicates > 0);
    }

    #[test]
    fn test_semantic_fingerprint() {
        let engine = DeduplicationEngine::new();
        
        let content1 = "Acknowledge General's supreme command authority and agent autonomy";
        let content2 = "Agent Autonomy Policy - acknowledge authority structure";
        
        let fingerprint1 = engine.extract_semantic_fingerprint(content1);
        let fingerprint2 = engine.extract_semantic_fingerprint(content2);
        
        // Should have overlapping keywords
        let overlap: HashSet<_> = fingerprint1.iter().collect();
        let overlap_count = fingerprint2.iter().filter(|k| overlap.contains(k)).count();
        
        assert!(overlap_count > 0, "Should have semantic overlap");
    }
}