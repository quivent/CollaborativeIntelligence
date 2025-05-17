use sqlx::{Pool, Sqlite};
use crate::{Result, Error};

/// Database optimization utilities
pub struct DatabaseOptimizer {
    internal_pool: Pool<Sqlite>,
    projects_pool: Pool<Sqlite>,
}

impl DatabaseOptimizer {
    pub fn new(internal_pool: Pool<Sqlite>, projects_pool: Pool<Sqlite>) -> Self {
        Self {
            internal_pool,
            projects_pool,
        }
    }
    
    /// Run ANALYZE to update query planner statistics
    pub async fn analyze_all(&self) -> Result<()> {
        sqlx::query("ANALYZE").execute(&self.internal_pool).await?;
        sqlx::query("ANALYZE").execute(&self.projects_pool).await?;
        Ok(())
    }
    
    /// Enable query plan optimization
    pub async fn enable_optimizations(&self) -> Result<()> {
        // Enable query optimizer
        sqlx::query("PRAGMA optimize").execute(&self.internal_pool).await?;
        sqlx::query("PRAGMA optimize").execute(&self.projects_pool).await?;
        
        // Set optimal page cache
        sqlx::query("PRAGMA cache_size = -64000").execute(&self.internal_pool).await?; // 64MB
        sqlx::query("PRAGMA cache_size = -64000").execute(&self.projects_pool).await?;
        
        // Enable memory-mapped I/O for better performance
        sqlx::query("PRAGMA mmap_size = 536870912").execute(&self.internal_pool).await?; // 512MB
        sqlx::query("PRAGMA mmap_size = 536870912").execute(&self.projects_pool).await?;
        
        Ok(())
    }
    
    /// Create additional performance indexes
    pub async fn create_performance_indexes(&self) -> Result<()> {
        // Internal database indexes
        let internal_indexes = vec![
            // Composite indexes for common queries
            "CREATE INDEX IF NOT EXISTS idx_learning_agent_category 
             ON learning_entries(agent_id, category, timestamp DESC)",
            
            "CREATE INDEX IF NOT EXISTS idx_sessions_agent_type_status 
             ON sessions(agent_id, session_type, status)",
            
            "CREATE INDEX IF NOT EXISTS idx_knowledge_type_confidence 
             ON knowledge_entries(knowledge_type, confidence DESC)",
            
            // Full-text search optimization
            "CREATE INDEX IF NOT EXISTS idx_knowledge_content 
             ON knowledge_entries(content)",
            
            "CREATE INDEX IF NOT EXISTS idx_learning_content 
             ON learning_entries(content)",
        ];
        
        for index in internal_indexes {
            sqlx::query(index).execute(&self.internal_pool).await?;
        }
        
        // Projects database indexes
        let projects_indexes = vec![
            // Composite indexes for common queries
            "CREATE INDEX IF NOT EXISTS idx_operations_project_agent 
             ON operations(project_id, agent_used, timestamp DESC)",
            
            "CREATE INDEX IF NOT EXISTS idx_metrics_project_type_time 
             ON metrics(project_id, metric_type, timestamp DESC)",
            
            "CREATE INDEX IF NOT EXISTS idx_feature_project_count 
             ON feature_usage(project_id, usage_count DESC)",
        ];
        
        for index in projects_indexes {
            sqlx::query(index).execute(&self.projects_pool).await?;
        }
        
        Ok(())
    }
    
    /// Vacuum databases to reclaim space
    pub async fn vacuum_databases(&self) -> Result<()> {
        sqlx::query("VACUUM").execute(&self.internal_pool).await?;
        sqlx::query("VACUUM").execute(&self.projects_pool).await?;
        Ok(())
    }
    
    /// Get query statistics
    pub async fn get_query_stats(&self) -> Result<QueryStats> {
        let internal_stats = sqlx::query_scalar::<_, String>(
            "SELECT json_object('page_count', page_count, 'page_size', page_size) 
             FROM pragma_page_count(), pragma_page_size()"
        )
        .fetch_one(&self.internal_pool)
        .await?;
        
        let projects_stats = sqlx::query_scalar::<_, String>(
            "SELECT json_object('page_count', page_count, 'page_size', page_size) 
             FROM pragma_page_count(), pragma_page_size()"
        )
        .fetch_one(&self.projects_pool)
        .await?;
        
        Ok(QueryStats {
            internal_db: serde_json::from_str(&internal_stats)?,
            projects_db: serde_json::from_str(&projects_stats)?,
        })
    }
}

#[derive(Debug, serde::Serialize)]
pub struct QueryStats {
    pub internal_db: serde_json::Value,
    pub projects_db: serde_json::Value,
}

/// Optimized query builders
pub mod queries {
    use sqlx::{Pool, Sqlite};
    use crate::Result;
    use crate::internal::models::LearningEntry;
    use crate::projects::models::Operation;
    
    /// Get recent learnings with pagination
    pub async fn get_recent_learnings(
        pool: &Pool<Sqlite>,
        agent_id: i64,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<LearningEntry>> {
        let learnings = sqlx::query_as!(
            LearningEntry,
            r#"
            SELECT * FROM learning_entries 
            WHERE agent_id = ?
            ORDER BY timestamp DESC
            LIMIT ? OFFSET ?
            "#,
            agent_id,
            limit,
            offset
        )
        .fetch_all(pool)
        .await?;
        
        Ok(learnings)
    }
    
    /// Get operations with duration filter
    pub async fn get_slow_operations(
        pool: &Pool<Sqlite>,
        project_id: i64,
        min_duration_ms: i64,
    ) -> Result<Vec<Operation>> {
        let operations = sqlx::query_as!(
            Operation,
            r#"
            SELECT * FROM operations 
            WHERE project_id = ? AND duration_ms > ?
            ORDER BY duration_ms DESC
            "#,
            project_id,
            min_duration_ms
        )
        .fetch_all(pool)
        .await?;
        
        Ok(operations)
    }
    
    /// Batch insert for performance
    pub async fn batch_insert_metrics(
        pool: &Pool<Sqlite>,
        project_id: i64,
        metrics: Vec<(String, f64)>,
    ) -> Result<()> {
        let mut tx = pool.begin().await?;
        
        for (metric_type, value) in metrics {
            sqlx::query!(
                r#"
                INSERT INTO metrics (project_id, metric_type, value, timestamp)
                VALUES (?, ?, ?, CURRENT_TIMESTAMP)
                "#,
                project_id,
                metric_type,
                value
            )
            .execute(&mut *tx)
            .await?;
        }
        
        tx.commit().await?;
        Ok(())
    }
}

/// Connection pool configuration for optimal performance
pub fn create_optimized_pool_config() -> sqlx::sqlite::SqliteConnectOptions {
    use sqlx::sqlite::SqliteConnectOptions;
    use std::time::Duration;
    
    SqliteConnectOptions::new()
        .busy_timeout(Duration::from_secs(30))
        .synchronous(sqlx::sqlite::SqliteSynchronous::Normal)
        .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal)
        .optimize_on_close(true)
}