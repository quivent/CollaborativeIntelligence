use crate::{Database, Result};
use crate::internal::{InternalRepository, Agent, LearningEntry, Session, KnowledgeEntry};
use crate::internal::{CreateAgentRequest, RecordLearningRequest, CreateSessionRequest, AddKnowledgeRequest};
use crate::projects::{ProjectsRepository, Project, Operation, Metric};
use crate::projects::{RegisterProjectRequest, LogOperationRequest, RecordMetricRequest, ProjectAgentConfigRequest};

/// High-level API for CI database operations
pub struct CIDatabaseAPI {
    internal: InternalRepository,
    projects: ProjectsRepository,
}

impl CIDatabaseAPI {
    pub fn new(db: Database) -> Self {
        Self {
            internal: InternalRepository::new(db.internal().clone()),
            projects: ProjectsRepository::new(db.projects().clone()),
        }
    }
    
    // Agent management
    pub async fn create_agent(&self, req: CreateAgentRequest) -> Result<Agent> {
        self.internal.create_agent(req).await
    }
    
    pub async fn get_agent(&self, name: &str) -> Result<Agent> {
        self.internal.get_agent(name).await
    }
    
    pub async fn list_agents(&self) -> Result<Vec<Agent>> {
        self.internal.list_agents().await
    }
    
    pub async fn activate_agent(&self, name: &str) -> Result<()> {
        self.internal.update_agent_status(name, "active").await
    }
    
    pub async fn deactivate_agent(&self, name: &str) -> Result<()> {
        self.internal.update_agent_status(name, "inactive").await
    }
    
    // Learning system
    pub async fn record_learning(&self, req: RecordLearningRequest) -> Result<LearningEntry> {
        self.internal.record_learning(req).await
    }
    
    pub async fn get_learnings(&self, agent_name: &str) -> Result<Vec<LearningEntry>> {
        self.internal.get_learnings(agent_name).await
    }
    
    // Session management
    pub async fn create_session(&self, req: CreateSessionRequest) -> Result<Session> {
        self.internal.create_session(req).await
    }
    
    pub async fn close_session(&self, session_id: i64, outcomes: Option<serde_json::Value>) -> Result<()> {
        self.internal.close_session(session_id, outcomes).await
    }
    
    // Knowledge base
    pub async fn add_knowledge(&self, req: AddKnowledgeRequest) -> Result<KnowledgeEntry> {
        self.internal.add_knowledge(req).await
    }
    
    pub async fn search_knowledge(&self, query: &str) -> Result<Vec<KnowledgeEntry>> {
        self.internal.search_knowledge(query).await
    }
    
    // Collaboration
    pub async fn record_collaboration(
        &self,
        initiator: &str,
        participant: &str,
        collaboration_type: &str,
    ) -> Result<()> {
        self.internal.record_collaboration(initiator, participant, collaboration_type).await?;
        Ok(())
    }
    
    // Project management
    pub async fn register_project(&self, req: RegisterProjectRequest) -> Result<Project> {
        self.projects.register_project(req).await
    }
    
    pub async fn get_project(&self, name: &str) -> Result<Project> {
        self.projects.get_project(name).await
    }
    
    pub async fn list_projects(&self) -> Result<Vec<Project>> {
        self.projects.list_projects().await
    }
    
    // Operation logging
    pub async fn log_operation(&self, req: LogOperationRequest) -> Result<Operation> {
        self.projects.log_operation(req).await
    }
    
    pub async fn complete_operation(
        &self,
        operation_id: i64,
        status: &str,
        duration_ms: i64,
        error_message: Option<String>,
    ) -> Result<()> {
        self.projects.complete_operation(operation_id, status, duration_ms, error_message).await
    }
    
    // Metrics
    pub async fn record_metric(&self, req: RecordMetricRequest) -> Result<Metric> {
        self.projects.record_metric(req).await
    }
    
    pub async fn get_metrics(&self, project_name: &str, metric_type: &str) -> Result<Vec<Metric>> {
        self.projects.get_metrics(project_name, metric_type).await
    }
    
    // Feature usage
    pub async fn track_feature(&self, project_name: &str, feature_name: &str) -> Result<()> {
        self.projects.track_feature(project_name, feature_name).await
    }
    
    // Project agents
    pub async fn configure_project_agent(&self, req: ProjectAgentConfigRequest) -> Result<()> {
        self.projects.configure_project_agent(req).await
    }
    
    pub async fn get_project_agents(&self, project_name: &str) -> Result<Vec<crate::projects::ProjectAgent>> {
        self.projects.get_project_agents(project_name).await
    }
}