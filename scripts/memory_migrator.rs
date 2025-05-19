// memory_migrator.rs - Tool for migrating legacy memory to optimized database
//
// This module provides functionality to migrate legacy memory files to
// the optimized database format. It performs the following operations:
// 1. Scans the agents directory for memory files
// 2. Creates agent cache entries in the optimized database
// 3. Validates data integrity during migration
// 4. Provides logging and rollback capabilities

use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::memory_schema::{
    AgentCacheEntry, MemoryDatabase, MemorySegment, Timestamp,
    load_database, save_database
};

/// Results of a migration operation
#[derive(Debug)]
pub struct MigrationResult {
    /// Total number of agents migrated
    pub migrated: usize,
    
    /// Number of agents skipped (already in database)
    pub skipped: usize,
    
    /// Number of migration errors
    pub errors: usize,
    
    /// Map of agents to their migration status
    pub agent_statuses: HashMap<String, MigrationStatus>,
}

/// Status of an individual agent migration
#[derive(Debug, Clone)]
pub enum MigrationStatus {
    /// Successfully migrated
    Success { segments: usize },
    
    /// Skipped (already in database)
    Skipped,
    
    /// Failed to migrate with error
    Error { message: String },
}

/// Options for controlling the migration process
#[derive(Debug, Clone)]
pub struct MigrationOptions {
    /// Whether to overwrite existing entries
    pub force_overwrite: bool,
    
    /// Whether to create a backup before migration
    pub create_backup: bool,
    
    /// Whether to validate data integrity after migration
    pub validate: bool,
    
    /// Whether to perform a dry run (no actual changes)
    pub dry_run: bool,
}

impl Default for MigrationOptions {
    fn default() -> Self {
        Self {
            force_overwrite: false,
            create_backup: true,
            validate: true,
            dry_run: false,
        }
    }
}

/// Main migrator for converting legacy memory to optimized format
pub struct MemoryMigrator {
    /// Path to agents directory
    agents_dir: PathBuf,
    
    /// Path to optimized database file
    db_path: PathBuf,
    
    /// Options controlling migration
    options: MigrationOptions,
    
    /// Database instance
    db: MemoryDatabase,
}

impl MemoryMigrator {
    /// Create a new memory migrator
    pub fn new(agents_dir: PathBuf, db_path: PathBuf, options: MigrationOptions) -> io::Result<Self> {
        // Load existing database or create a new one
        let db = load_database(&db_path)?;
        
        Ok(Self {
            agents_dir,
            db_path,
            options,
            db,
        })
    }
    
    /// Migrate a single agent's memory to the optimized database
    pub fn migrate_agent(&mut self, agent_name: &str) -> Result<MigrationStatus, io::Error> {
        let agent_dir = self.agents_dir.join(agent_name);
        
        // Check if agent directory exists
        if !agent_dir.exists() || !agent_dir.is_dir() {
            return Ok(MigrationStatus::Error {
                message: format!("Agent directory not found: {}", agent_dir.display()),
            });
        }
        
        // Check if agent already exists in database
        if !self.options.force_overwrite && self.db.agents.contains_key(agent_name) {
            return Ok(MigrationStatus::Skipped);
        }
        
        // Create agent cache entry
        let agent_entry = AgentCacheEntry::new(
            agent_name.to_string(),
            agent_dir.clone(),
            "Migrator".to_string(),
            "Migrated from legacy memory files".to_string(),
        );
        
        // Simulate creation if dry run
        if self.options.dry_run {
            return Ok(MigrationStatus::Success { segments: 0 });
        }
        
        // Create modified agent entry with populated segments
        let mut agent_entry = self.populate_agent_segments(agent_entry, &agent_dir)?;
        
        // Add metadata about migration
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as Timestamp;
            
        agent_entry = agent_entry
            .with_metadata("migrated_at", &now.to_string())
            .with_metadata("migrated_by", "memory_migrator");
        
        // Count segments
        let segment_count = agent_entry.segments.len();
        
        // Add agent to database
        self.db.add_agent(agent_entry);
        
        // Save database if not dry run
        if !self.options.dry_run {
            save_database(&self.db, &self.db_path)?;
        }
        
        Ok(MigrationStatus::Success { segments: segment_count })
    }
    
    /// Populate agent segments by reading memory files
    fn populate_agent_segments(
        &self,
        mut agent_entry: AgentCacheEntry,
        agent_dir: &PathBuf,
    ) -> io::Result<AgentCacheEntry> {
        // Find all .md files in the agent directory
        let entries = fs::read_dir(agent_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Only process .md files
            if path.is_file() && path.extension().map_or(false, |ext| ext == "md") {
                // Get segment name from filename (without extension)
                let segment_name = path.file_stem()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Read file content
                let mut file = fs::File::open(&path)?;
                let mut content = String::new();
                file.read_to_string(&mut content)?;
                
                // Create memory segment
                let segment = MemorySegment::new(segment_name.clone(), content)
                    .with_metadata("source_file", &path.to_string_lossy());
                
                // Add segment to agent
                agent_entry.add_segment(segment);
            }
        }
        
        Ok(agent_entry)
    }
    
    /// Migrate all agents in the agents directory
    pub fn migrate_all(&mut self) -> Result<MigrationResult, io::Error> {
        let mut result = MigrationResult {
            migrated: 0,
            skipped: 0,
            errors: 0,
            agent_statuses: HashMap::new(),
        };
        
        // Create backup if requested
        if self.options.create_backup && !self.options.dry_run {
            self.create_backup()?;
        }
        
        // Get list of agent directories
        let entries = fs::read_dir(&self.agents_dir)?;
        
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            
            // Only process directories
            if path.is_dir() {
                let agent_name = path.file_name()
                    .and_then(|name| name.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                
                // Skip hidden directories
                if agent_name.starts_with('.') {
                    continue;
                }
                
                // Migrate agent
                match self.migrate_agent(&agent_name) {
                    Ok(MigrationStatus::Success { segments }) => {
                        result.migrated += 1;
                        result.agent_statuses.insert(
                            agent_name,
                            MigrationStatus::Success { segments },
                        );
                    }
                    Ok(MigrationStatus::Skipped) => {
                        result.skipped += 1;
                        result.agent_statuses.insert(
                            agent_name,
                            MigrationStatus::Skipped,
                        );
                    }
                    Ok(MigrationStatus::Error { message }) | Err(e) => {
                        let error_message = match e {
                            error @ _ => format!("{}", error),
                        };
                        
                        result.errors += 1;
                        result.agent_statuses.insert(
                            agent_name,
                            MigrationStatus::Error { message: error_message },
                        );
                    }
                }
            }
        }
        
        // Validate migration if requested
        if self.options.validate && !self.options.dry_run {
            self.validate_migration()?;
        }
        
        Ok(result)
    }
    
    /// Create a backup of the database before migration
    fn create_backup(&self) -> io::Result<PathBuf> {
        // Skip if database doesn't exist yet
        if !self.db_path.exists() {
            return Ok(PathBuf::new());
        }
        
        // Create backup filename with timestamp
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
            
        let backup_path = self.db_path.with_extension(format!("backup.{}", now));
        
        // Copy database to backup
        fs::copy(&self.db_path, &backup_path)?;
        
        Ok(backup_path)
    }
    
    /// Validate data integrity after migration
    fn validate_migration(&self) -> io::Result<bool> {
        // Load database from disk to verify it was saved correctly
        let loaded_db = load_database(&self.db_path)?;
        
        // Check that all agents were migrated
        for (agent_name, _) in &self.db.agents {
            if !loaded_db.agents.contains_key(agent_name) {
                return Ok(false);
            }
        }
        
        // Check segment counts
        for (agent_name, agent) in &self.db.agents {
            if let Some(loaded_agent) = loaded_db.agents.get(agent_name) {
                if agent.segments.len() != loaded_agent.segments.len() {
                    return Ok(false);
                }
            } else {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Restore from backup after failed migration
    pub fn restore_from_backup(&self, backup_path: &Path) -> io::Result<()> {
        if backup_path.exists() {
            fs::copy(backup_path, &self.db_path)?;
        }
        
        Ok(())
    }
}

/// Utility function to migrate all agents in one call
pub fn migrate_all_agents(
    agents_dir: PathBuf,
    db_path: PathBuf,
    options: MigrationOptions,
) -> Result<MigrationResult, io::Error> {
    let mut migrator = MemoryMigrator::new(agents_dir, db_path, options)?;
    migrator.migrate_all()
}

/// Utility function to migrate a single agent
pub fn migrate_single_agent(
    agent_name: &str,
    agents_dir: PathBuf,
    db_path: PathBuf,
    options: MigrationOptions,
) -> Result<MigrationStatus, io::Error> {
    let mut migrator = MemoryMigrator::new(agents_dir, db_path, options)?;
    migrator.migrate_agent(agent_name)
}