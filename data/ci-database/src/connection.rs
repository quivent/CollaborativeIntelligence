use sqlx::{Pool, Sqlite, SqlitePool};
use std::path::{Path, PathBuf};
use crate::{Result, Error};
use crate::optimization::{DatabaseOptimizer, create_optimized_pool_config};

#[derive(Clone)]
pub struct Database {
    internal_pool: Pool<Sqlite>,
    projects_pool: Pool<Sqlite>,
    data_dir: PathBuf,
}

impl Database {
    /// Create a new database connection manager
    pub async fn new(data_dir: impl AsRef<Path>) -> Result<Self> {
        let data_dir = data_dir.as_ref().to_path_buf();
        
        // Ensure directories exist
        let internal_dir = data_dir.join("internal");
        let projects_dir = data_dir.join("projects");
        
        std::fs::create_dir_all(&internal_dir)?;
        std::fs::create_dir_all(&projects_dir)?;
        
        // Create connection strings
        let internal_db = internal_dir.join("ci-internal.db");
        let projects_db = projects_dir.join("ci-projects.db");
        
        // Create optimized connection options
        let internal_opts = create_optimized_pool_config()
            .filename(&internal_db);
        let projects_opts = create_optimized_pool_config()
            .filename(&projects_db);
        
        // Create connection pools with optimizations
        let internal_pool = SqlitePool::connect_with(internal_opts).await?;
        let projects_pool = SqlitePool::connect_with(projects_opts).await?;
        
        // Run migrations
        let db = Self {
            internal_pool,
            projects_pool,
            data_dir,
        };
        
        db.run_migrations().await?;
        
        // Apply optimizations
        let optimizer = DatabaseOptimizer::new(
            db.internal_pool.clone(),
            db.projects_pool.clone(),
        );
        optimizer.enable_optimizations().await?;
        
        Ok(db)
    }
    
    /// Get the internal database pool
    pub fn internal(&self) -> &Pool<Sqlite> {
        &self.internal_pool
    }
    
    /// Get the projects database pool
    pub fn projects(&self) -> &Pool<Sqlite> {
        &self.projects_pool
    }
    
    /// Run database migrations (placeholder for now)
    async fn run_migrations(&self) -> Result<()> {
        // For now, we'll just ensure tables exist
        // In production, this would use proper migration system
        Ok(())
    }
    
    /// Initialize databases with schema (for initial setup)
    pub async fn initialize(&self) -> Result<()> {
        // Read and execute internal schema
        let internal_schema = std::fs::read_to_string(
            self.data_dir.join("internal/schema.sql")
        )?;
        
        sqlx::query(&internal_schema)
            .execute(&self.internal_pool)
            .await?;
            
        // Read and execute projects schema
        let projects_schema = std::fs::read_to_string(
            self.data_dir.join("projects/schema.sql")
        )?;
        
        sqlx::query(&projects_schema)
            .execute(&self.projects_pool)
            .await?;
            
        Ok(())
    }
    
    /// Backup databases
    pub async fn backup(&self, backup_dir: impl AsRef<Path>) -> Result<()> {
        let backup_dir = backup_dir.as_ref();
        std::fs::create_dir_all(backup_dir)?;
        
        let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
        
        // Backup internal database
        let internal_backup = backup_dir.join(format!("internal_{}.db", timestamp));
        let internal_source = self.data_dir.join("internal/ci-internal.db");
        std::fs::copy(internal_source, internal_backup)?;
        
        // Backup projects database
        let projects_backup = backup_dir.join(format!("projects_{}.db", timestamp));
        let projects_source = self.data_dir.join("projects/ci-projects.db");
        std::fs::copy(projects_source, projects_backup)?;
        
        Ok(())
    }
    
    /// Optimize database performance
    pub async fn optimize(&self) -> Result<()> {
        let optimizer = DatabaseOptimizer::new(
            self.internal_pool.clone(),
            self.projects_pool.clone(),
        );
        
        // Create performance indexes
        optimizer.create_performance_indexes().await?;
        
        // Update statistics
        optimizer.analyze_all().await?;
        
        Ok(())
    }
    
    /// Close all database connections
    pub async fn close(&self) -> Result<()> {
        self.internal_pool.close().await;
        self.projects_pool.close().await;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_database_creation() {
        let temp_dir = TempDir::new().unwrap();
        let db = Database::new(temp_dir.path()).await.unwrap();
        
        // Test that pools are created
        assert!(!db.internal().is_closed());
        assert!(!db.projects().is_closed());
        
        db.close().await.unwrap();
    }
}