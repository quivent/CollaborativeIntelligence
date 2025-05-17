use crate::{Database, CIDatabaseAPI, Result};
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;

/// CI System integration for database operations
pub struct CIDatabaseIntegration {
    api: Arc<CIDatabaseAPI>,
    current_session_id: Arc<Mutex<Option<i64>>>,
}

impl CIDatabaseIntegration {
    /// Create new integration instance
    pub async fn new(data_dir: impl AsRef<Path>) -> Result<Self> {
        let db = Database::new(data_dir).await?;
        let api = CIDatabaseAPI::new(db);
        
        Ok(Self {
            api: Arc::new(api),
            current_session_id: Arc::new(Mutex::new(None)),
        })
    }
    
    /// Initialize a CI command with database tracking
    pub async fn begin_command(
        &self,
        project_name: &str,
        command: &str,
        agent: Option<&str>,
    ) -> Result<i64> {
        use crate::projects::LogOperationRequest;
        
        let req = LogOperationRequest {
            project_name: project_name.to_string(),
            command: command.to_string(),
            arguments: None,
            agent_used: agent.map(|s| s.to_string()),
            metadata: None,
        };
        
        let operation = self.api.log_operation(req).await?;
        Ok(operation.id.unwrap())
    }
    
    /// Complete a CI command with results
    pub async fn complete_command(
        &self,
        operation_id: i64,
        success: bool,
        duration_ms: i64,
        error: Option<String>,
    ) -> Result<()> {
        let status = if success { "completed" } else { "failed" };
        self.api.complete_operation(operation_id, status, duration_ms, error).await
    }
    
    /// Track agent activation
    pub async fn activate_agent_for_project(
        &self,
        project_name: &str,
        agent_name: &str,
    ) -> Result<()> {
        // Track feature usage
        self.api.track_feature(project_name, agent_name).await?;
        
        // Start a session for the agent
        use crate::internal::CreateSessionRequest;
        let req = CreateSessionRequest {
            agent_name: agent_name.to_string(),
            session_type: "project_work".to_string(),
            metadata: Some(serde_json::json!({
                "project": project_name
            })),
        };
        
        let session = self.api.create_session(req).await?;
        
        // Store session ID
        let mut session_id = self.current_session_id.lock().await;
        *session_id = session.id;
        
        Ok(())
    }
    
    /// Record learning during CI operations
    pub async fn record_ci_learning(
        &self,
        agent_name: &str,
        content: &str,
        category: &str,
    ) -> Result<()> {
        use crate::internal::RecordLearningRequest;
        
        let req = RecordLearningRequest {
            agent_name: agent_name.to_string(),
            trigger_type: "ci_operation".to_string(),
            content: content.to_string(),
            importance: Some(0.7),
            category: Some(category.to_string()),
            metadata: None,
        };
        
        self.api.record_learning(req).await?;
        Ok(())
    }
    
    /// Close current agent session
    pub async fn close_agent_session(&self, outcomes: Option<serde_json::Value>) -> Result<()> {
        let mut session_id = self.current_session_id.lock().await;
        
        if let Some(id) = *session_id {
            self.api.close_session(id, outcomes).await?;
            *session_id = None;
        }
        
        Ok(())
    }
    
    /// Get API reference for direct operations
    pub fn api(&self) -> &CIDatabaseAPI {
        &self.api
    }
}

/// Helper functions for CI integration
pub mod helpers {
    use super::*;
    
    /// Initialize a project in the database
    pub async fn init_project(
        integration: &CIDatabaseIntegration,
        name: &str,
        path: &str,
        ci_version: &str,
    ) -> Result<()> {
        use crate::projects::RegisterProjectRequest;
        
        let req = RegisterProjectRequest {
            name: name.to_string(),
            path: path.to_string(),
            project_type: Some("ci_managed".to_string()),
            ci_version: Some(ci_version.to_string()),
            metadata: None,
            config: None,
        };
        
        integration.api.register_project(req).await?;
        Ok(())
    }
    
    /// Record a metric for the current project
    pub async fn record_metric(
        integration: &CIDatabaseIntegration,
        project: &str,
        metric_type: &str,
        value: f64,
    ) -> Result<()> {
        use crate::projects::RecordMetricRequest;
        
        let req = RecordMetricRequest {
            project_name: project.to_string(),
            metric_type: metric_type.to_string(),
            value,
            context: None,
        };
        
        integration.api.record_metric(req).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    
    #[tokio::test]
    async fn test_ci_integration() {
        let temp_dir = TempDir::new().unwrap();
        let integration = CIDatabaseIntegration::new(temp_dir.path()).await.unwrap();
        
        // Initialize database
        let db = Database::new(temp_dir.path()).await.unwrap();
        db.initialize().await.unwrap();
        
        // Register a project
        helpers::init_project(&integration, "TestProject", "/test", "1.0.0").await.unwrap();
        
        // Start a command
        let op_id = integration.begin_command("TestProject", "test command", Some("TestAgent")).await.unwrap();
        
        // Complete the command
        integration.complete_command(op_id, true, 100, None).await.unwrap();
        
        // Record a metric
        helpers::record_metric(&integration, "TestProject", "test_metric", 99.9).await.unwrap();
    }
}