use sqlx::{Pool, Sqlite};
use crate::{Result, Error};
use super::models::*;
use chrono::Utc;

pub struct ProjectsRepository {
    pool: Pool<Sqlite>,
}

impl ProjectsRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    // Project operations
    pub async fn register_project(&self, req: RegisterProjectRequest) -> Result<Project> {
        let now = Utc::now();
        
        let project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO projects (
                name, path, type, status, created_at, updated_at,
                ci_version, metadata, config
            )
            VALUES (?1, ?2, ?3, 'active', ?4, ?5, ?6, ?7, ?8)
            RETURNING *
            "#,
            req.name,
            req.path,
            req.project_type.unwrap_or_else(|| "general".to_string()),
            now,
            now,
            req.ci_version,
            req.metadata,
            req.config
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(project)
    }
    
    pub async fn get_project(&self, name: &str) -> Result<Project> {
        let project = sqlx::query_as!(
            Project,
            "SELECT * FROM projects WHERE name = ?",
            name
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Project '{}' not found", name)))?;
        
        Ok(project)
    }
    
    pub async fn update_project_access(&self, name: &str) -> Result<()> {
        sqlx::query!(
            "UPDATE projects SET last_accessed = ? WHERE name = ?",
            Utc::now(),
            name
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        let projects = sqlx::query_as!(
            Project,
            "SELECT * FROM projects ORDER BY last_accessed DESC"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(projects)
    }
    
    // Operation logging
    pub async fn log_operation(&self, req: LogOperationRequest) -> Result<Operation> {
        let project = self.get_project(&req.project_name).await?;
        
        let operation = sqlx::query_as!(
            Operation,
            r#"
            INSERT INTO operations (
                project_id, timestamp, command, arguments,
                agent_used, status, metadata
            )
            VALUES (?1, ?2, ?3, ?4, ?5, 'started', ?6)
            RETURNING *
            "#,
            project.id,
            Utc::now(),
            req.command,
            req.arguments,
            req.agent_used,
            req.metadata
        )
        .fetch_one(&self.pool)
        .await?;
        
        // Update project access time
        self.update_project_access(&req.project_name).await?;
        
        Ok(operation)
    }
    
    pub async fn complete_operation(
        &self,
        operation_id: i64,
        status: &str,
        duration_ms: i64,
        error_message: Option<String>,
    ) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE operations 
            SET status = ?, duration_ms = ?, error_message = ?
            WHERE id = ?
            "#,
            status,
            duration_ms,
            error_message,
            operation_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // Metrics
    pub async fn record_metric(&self, req: RecordMetricRequest) -> Result<Metric> {
        let project = self.get_project(&req.project_name).await?;
        
        let metric = sqlx::query_as!(
            Metric,
            r#"
            INSERT INTO metrics (
                project_id, metric_type, value, timestamp, context
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            RETURNING *
            "#,
            project.id,
            req.metric_type,
            req.value,
            Utc::now(),
            req.context
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(metric)
    }
    
    pub async fn get_metrics(&self, project_name: &str, metric_type: &str) -> Result<Vec<Metric>> {
        let project = self.get_project(project_name).await?;
        
        let metrics = sqlx::query_as!(
            Metric,
            r#"
            SELECT * FROM metrics 
            WHERE project_id = ? AND metric_type = ?
            ORDER BY timestamp DESC
            "#,
            project.id,
            metric_type
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(metrics)
    }
    
    // Feature usage
    pub async fn track_feature(&self, project_name: &str, feature_name: &str) -> Result<()> {
        let project = self.get_project(project_name).await?;
        let now = Utc::now();
        
        // Try to update existing record
        let result = sqlx::query!(
            r#"
            UPDATE feature_usage 
            SET last_used = ?, usage_count = usage_count + 1
            WHERE project_id = ? AND feature_name = ?
            "#,
            now,
            project.id,
            feature_name
        )
        .execute(&self.pool)
        .await?;
        
        // If no rows updated, insert new record
        if result.rows_affected() == 0 {
            sqlx::query!(
                r#"
                INSERT INTO feature_usage (
                    project_id, feature_name, first_used, last_used, usage_count
                )
                VALUES (?1, ?2, ?3, ?4, 1)
                "#,
                project.id,
                feature_name,
                now,
                now
            )
            .execute(&self.pool)
            .await?;
        }
        
        Ok(())
    }
    
    // Project agents
    pub async fn configure_project_agent(&self, req: ProjectAgentConfigRequest) -> Result<()> {
        let project = self.get_project(&req.project_name).await?;
        
        // Insert or update
        sqlx::query!(
            r#"
            INSERT OR REPLACE INTO project_agents (
                project_id, agent_name, enabled, custom_config, last_used
            )
            VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            project.id,
            req.agent_name,
            req.enabled,
            req.custom_config,
            Utc::now()
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    pub async fn get_project_agents(&self, project_name: &str) -> Result<Vec<ProjectAgent>> {
        let project = self.get_project(project_name).await?;
        
        let agents = sqlx::query_as!(
            ProjectAgent,
            r#"
            SELECT * FROM project_agents 
            WHERE project_id = ? AND enabled = true
            ORDER BY agent_name
            "#,
            project.id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(agents)
    }
}