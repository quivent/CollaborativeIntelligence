// Topological Repository Management System - Core Library
// Zero-dependency portable repository organization toolkit

pub mod file_categorizer;
pub mod commit_planner;
pub mod git_operations;
pub mod metadata_manager;

pub use file_categorizer::{FileCategorizer, FileCategory, CategorizedFile, CategoryAnalysis, CommitPhase};

// Re-export key types for easier usage
pub use commit_planner::CommitPlanner;
pub use git_operations::GitOperations;
pub use metadata_manager::{MetadataManager, ProjectConfig, SessionHistory};

#[derive(Debug)]
pub struct TopologyAnalysis {
    pub category_analysis: CategoryAnalysis,
    pub commit_phases: Vec<CommitPhase>,
    pub repository_stats: RepositoryStats,
}

#[derive(Debug)]
pub struct RepositoryStats {
    pub total_files: usize,
    pub untracked_files: usize,
    pub modified_files: usize,
    pub estimated_total_size: usize,
    pub suggested_phases: usize,
}

pub struct Topologist {
    categorizer: FileCategorizer,
    git_ops: GitOperations,
    metadata: MetadataManager,
}

impl Topologist {
    pub fn new() -> Self {
        Self {
            categorizer: FileCategorizer::new(),
            git_ops: GitOperations::new(),
            metadata: MetadataManager::new(),
        }
    }

    /// Analyze repository without creating any files
    pub fn analyze_repository(&self) -> Result<TopologyAnalysis, Box<dyn std::error::Error>> {
        let (untracked, modified) = self.git_ops.get_repository_status()?;
        let all_files: Vec<String> = untracked.into_iter().chain(modified.into_iter()).collect();
        
        let category_analysis = self.categorizer.analyze_files(all_files.clone());
        let commit_phases = self.categorizer.generate_commit_plan(&category_analysis);
        
        let repository_stats = RepositoryStats {
            total_files: all_files.len(),
            untracked_files: category_analysis.files.iter()
                .filter(|f| self.git_ops.is_untracked(&f.path).unwrap_or(false))
                .count(),
            modified_files: all_files.len() - category_analysis.files.iter()
                .filter(|f| self.git_ops.is_untracked(&f.path).unwrap_or(false))
                .count(),
            estimated_total_size: category_analysis.estimated_total_size,
            suggested_phases: category_analysis.suggested_phases,
        };

        Ok(TopologyAnalysis {
            category_analysis,
            commit_phases,
            repository_stats,
        })
    }

    /// Initialize topology management for current repository
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.metadata.initialize_project()?;
        Ok(())
    }

    /// Execute a specific commit phase
    pub fn execute_phase(&mut self, phase_number: usize, commit_phases: &[CommitPhase]) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(phase) = commit_phases.get(phase_number - 1) {
            let file_paths: Vec<&str> = phase.files.iter().map(|f| f.path.as_str()).collect();
            let commit_hash = self.git_ops.stage_and_commit_files(&file_paths, &phase.commit_message)?;
            
            // Record the phase execution
            self.metadata.record_phase_execution(phase_number, &commit_hash, phase.files.len(), phase.estimated_size)?;
            
            Ok(commit_hash)
        } else {
            Err(format!("Phase {} not found", phase_number).into())
        }
    }

    /// Clean all topology metadata
    pub fn clean(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.metadata.clean_all_metadata()
    }

    /// Get current session status
    pub fn get_status(&self) -> Result<Option<SessionHistory>, Box<dyn std::error::Error>> {
        self.metadata.get_current_session()
    }
}

impl Default for Topologist {
    fn default() -> Self {
        Self::new()
    }
}