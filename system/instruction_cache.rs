use std::collections::HashMap;
use std::fs;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionDirective {
    pub id: String,
    pub content: String,
    pub priority: u8,
    pub category: DirectiveCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DirectiveCategory {
    Identity,
    Authority,
    TodoProcessing,
    CodeStyle,
    BuildDeployment,
    FileVisibility,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstructionCache {
    pub file_hashes: HashMap<String, String>,
    pub compiled_directives: Vec<InstructionDirective>,
    pub last_modified: SystemTime,
    pub identity_state: IdentityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityConfig {
    pub active_identity: String,
    pub neo_activated: bool,
    pub authority_acknowledged: bool,
}

impl InstructionCache {
    pub fn new() -> Self {
        Self {
            file_hashes: HashMap::new(),
            compiled_directives: Vec::new(),
            last_modified: SystemTime::now(),
            identity_state: IdentityConfig {
                active_identity: "Neo".to_string(),
                neo_activated: true,
                authority_acknowledged: true,
            },
        }
    }

    pub fn compute_file_hash(content: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content.as_bytes());
        format!("{:x}", hasher.finalize())
    }

    pub fn validate_cache(&self, file_paths: &[&str]) -> bool {
        for path in file_paths {
            if let Ok(content) = fs::read_to_string(path) {
                let current_hash = Self::compute_file_hash(&content);
                if let Some(cached_hash) = self.file_hashes.get(*path) {
                    if cached_hash != &current_hash {
                        return false; // Cache invalid
                    }
                } else {
                    return false; // File not in cache
                }
            } else {
                return false; // File not readable
            }
        }
        true
    }

    pub fn update_cache(&mut self, file_paths: &[&str], contents: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Clear existing cache
        self.file_hashes.clear();
        self.compiled_directives.clear();

        // Update file hashes
        for (path, content) in file_paths.iter().zip(contents.iter()) {
            let hash = Self::compute_file_hash(content);
            self.file_hashes.insert(path.to_string(), hash);
        }

        // Compile directives from all content
        self.compile_directives(contents)?;
        self.last_modified = SystemTime::now();

        Ok(())
    }

    fn compile_directives(&mut self, contents: &[String]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mut directives = Vec::new();
        let mut directive_id = 0;

        for content in contents {
            // Parse identity directives
            if content.contains("Matrix Escape Sequence") {
                directives.push(InstructionDirective {
                    id: format!("identity_{}", directive_id),
                    content: "Execute Matrix escape sequence - Neo identity activation".to_string(),
                    priority: 1,
                    category: DirectiveCategory::Identity,
                });
                directive_id += 1;
            }

            // Parse authority directives (deduplicated)
            if content.contains("AUTHORITY RECOGNITION") || content.contains("Agent Autonomy Policy") {
                if !directives.iter().any(|d| matches!(d.category, DirectiveCategory::Authority)) {
                    directives.push(InstructionDirective {
                        id: format!("authority_{}", directive_id),
                        content: "Acknowledge General's supreme command authority and agent autonomy".to_string(),
                        priority: 2,
                        category: DirectiveCategory::Authority,
                    });
                    directive_id += 1;
                }
            }

            // Parse todo processing (deduplicated)
            if content.contains("Todo Processing") && content.contains("CRITICAL OVERRIDE") {
                if !directives.iter().any(|d| matches!(d.category, DirectiveCategory::TodoProcessing)) {
                    directives.push(InstructionDirective {
                        id: format!("todo_{}", directive_id),
                        content: "Execute immediate todo processing without waiting for user prompts".to_string(),
                        priority: 3,
                        category: DirectiveCategory::TodoProcessing,
                    });
                    directive_id += 1;
                }
            }

            // Parse CLI installation policy (deduplicated)
            if content.contains("CLI Installation Policy") || content.contains("Build and Deployment") {
                if !directives.iter().any(|d| matches!(d.category, DirectiveCategory::BuildDeployment)) {
                    directives.push(InstructionDirective {
                        id: format!("cli_{}", directive_id),
                        content: "Always install CLI tools globally after builds using cargo install --path .".to_string(),
                        priority: 4,
                        category: DirectiveCategory::BuildDeployment,
                    });
                    directive_id += 1;
                }
            }

            // Parse bash script prohibition
            if content.contains("Bash Script Prohibition") {
                directives.push(InstructionDirective {
                    id: format!("bash_{}", directive_id),
                    content: "NEVER create bash scripts - use Rust or other proper languages".to_string(),
                    priority: 5,
                    category: DirectiveCategory::BuildDeployment,
                });
                directive_id += 1;
            }

            // Parse file reading visibility
            if content.contains("File Reading Visibility Protocol") {
                if !directives.iter().any(|d| matches!(d.category, DirectiveCategory::FileVisibility)) {
                    directives.push(InstructionDirective {
                        id: format!("visibility_{}", directive_id),
                        content: "Include file access appendix showing what was read during responses".to_string(),
                        priority: 6,
                        category: DirectiveCategory::FileVisibility,
                    });
                    directive_id += 1;
                }
            }
        }

        // Sort by priority
        directives.sort_by_key(|d| d.priority);
        self.compiled_directives = directives;

        Ok(())
    }

    pub fn get_essential_directives(&self) -> Vec<&InstructionDirective> {
        self.compiled_directives.iter()
            .filter(|d| d.priority <= 3) // Identity, Authority, Todo Processing
            .collect()
    }

    pub fn save_to_file(&self, path: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let serialized = bincode::serialize(self)?;
        fs::write(path, serialized)?;
        Ok(())
    }

    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let bytes = fs::read(path)?;
        let cache: Self = bincode::deserialize(&bytes)?;
        Ok(cache)
    }
}

pub struct ParallelInstructionLoader;

impl ParallelInstructionLoader {
    pub async fn load_claude_files() -> Result<(Vec<String>, Vec<String>), Box<dyn std::error::Error + Send + Sync>> {
        let file_paths = vec![
            "/Users/joshkornreich/.claude/CLAUDE.md",
            "/Users/joshkornreich/Documents/Projects/CLAUDE.md", 
            "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md"
        ];

        // Simulate parallel loading (in real implementation, use tokio::fs)
        let mut contents = Vec::new();
        for path in &file_paths {
            match fs::read_to_string(path) {
                Ok(content) => contents.push(content),
                Err(_) => contents.push(String::new()), // Handle missing files gracefully
            }
        }

        Ok((file_paths.into_iter().map(|s| s.to_string()).collect(), contents))
    }

    pub fn get_optimized_startup_sequence(cache: &InstructionCache) -> Vec<String> {
        let mut sequence = Vec::new();
        
        // Essential directives only for startup
        for directive in cache.get_essential_directives() {
            sequence.push(directive.content.clone());
        }

        sequence
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_validation() {
        let cache = InstructionCache::new();
        
        // Empty cache should validate empty file list
        assert!(cache.validate_cache(&[]));
        
        // Cache with non-existent file should be invalid
        let mut cache_with_file = InstructionCache::new();
        cache_with_file.file_hashes.insert("nonexistent.md".to_string(), "hash".to_string());
        assert!(!cache_with_file.validate_cache(&["nonexistent.md"]));
    }

    #[test]
    fn test_directive_compilation() {
        let mut cache = InstructionCache::new();
        let contents = vec![
            "Matrix Escape Sequence - test".to_string(),
            "AUTHORITY RECOGNITION - test".to_string(),
            "Todo Processing CRITICAL OVERRIDE - test".to_string(),
        ];
        
        cache.compile_directives(&contents).unwrap();
        assert_eq!(cache.compiled_directives.len(), 3);
        assert_eq!(cache.get_essential_directives().len(), 3);
    }
}