use sqlx::{Pool, Sqlite};
use crate::{Result, Error};
use super::models::*;
use chrono::Utc;

pub struct InternalRepository {
    pool: Pool<Sqlite>,
}

impl InternalRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
    
    // Agent operations
    pub async fn create_agent(&self, req: CreateAgentRequest) -> Result<Agent> {
        let agent = sqlx::query_as!(
            Agent,
            r#"
            INSERT INTO agents (name, type, status, created_at, memory_version, capabilities, memory_path, config)
            VALUES (?1, ?2, 'active', ?3, 1, ?4, ?5, ?6)
            RETURNING *
            "#,
            req.name,
            req.agent_type,
            Utc::now(),
            req.capabilities,
            req.memory_path,
            req.config
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(agent)
    }
    
    pub async fn get_agent(&self, name: &str) -> Result<Agent> {
        let agent = sqlx::query_as!(
            Agent,
            "SELECT * FROM agents WHERE name = ?",
            name
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or_else(|| Error::NotFound(format!("Agent '{}' not found", name)))?;
        
        Ok(agent)
    }
    
    pub async fn update_agent_status(&self, name: &str, status: &str) -> Result<()> {
        let result = sqlx::query!(
            "UPDATE agents SET status = ?, last_active = ? WHERE name = ?",
            status,
            Utc::now(),
            name
        )
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            return Err(Error::NotFound(format!("Agent '{}' not found", name)));
        }
        
        Ok(())
    }
    
    pub async fn list_agents(&self) -> Result<Vec<Agent>> {
        let agents = sqlx::query_as!(
            Agent,
            "SELECT * FROM agents ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(agents)
    }
    
    // Learning operations
    pub async fn record_learning(&self, req: RecordLearningRequest) -> Result<LearningEntry> {
        // First get the agent ID
        let agent = self.get_agent(&req.agent_name).await?;
        
        let learning = sqlx::query_as!(
            LearningEntry,
            r#"
            INSERT INTO learning_entries (
                agent_id, timestamp, trigger_type, content, 
                importance, category, metadata
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)
            RETURNING *
            "#,
            agent.id,
            Utc::now(),
            req.trigger_type,
            req.content,
            req.importance.unwrap_or(0.5),
            req.category,
            req.metadata
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(learning)
    }
    
    pub async fn get_learnings(&self, agent_name: &str) -> Result<Vec<LearningEntry>> {
        let agent = self.get_agent(agent_name).await?;
        
        let learnings = sqlx::query_as!(
            LearningEntry,
            "SELECT * FROM learning_entries WHERE agent_id = ? ORDER BY timestamp DESC",
            agent.id
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(learnings)
    }
    
    // Session operations
    pub async fn create_session(&self, req: CreateSessionRequest) -> Result<Session> {
        let agent = self.get_agent(&req.agent_name).await?;
        
        let session = sqlx::query_as!(
            Session,
            r#"
            INSERT INTO sessions (
                agent_id, session_type, started_at, status, metadata
            )
            VALUES (?1, ?2, ?3, 'active', ?4)
            RETURNING *
            "#,
            agent.id,
            req.session_type,
            Utc::now(),
            req.metadata
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(session)
    }
    
    pub async fn close_session(&self, session_id: i64, outcomes: Option<serde_json::Value>) -> Result<()> {
        sqlx::query!(
            r#"
            UPDATE sessions 
            SET ended_at = ?, status = 'completed', outcomes = ?
            WHERE id = ?
            "#,
            Utc::now(),
            outcomes,
            session_id
        )
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }
    
    // Knowledge operations
    pub async fn add_knowledge(&self, req: AddKnowledgeRequest) -> Result<KnowledgeEntry> {
        let agent = self.get_agent(&req.agent_name).await?;
        let now = Utc::now();
        
        let knowledge = sqlx::query_as!(
            KnowledgeEntry,
            r#"
            INSERT INTO knowledge_entries (
                agent_id, created_at, updated_at, knowledge_type,
                category, content, confidence, usage_count, metadata
            )
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, 0, ?8)
            RETURNING *
            "#,
            agent.id,
            now,
            now,
            req.knowledge_type,
            req.category,
            req.content,
            req.confidence.unwrap_or(0.8),
            req.metadata
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(knowledge)
    }
    
    pub async fn search_knowledge(&self, query: &str) -> Result<Vec<KnowledgeEntry>> {
        let knowledge = sqlx::query_as!(
            KnowledgeEntry,
            r#"
            SELECT * FROM knowledge_entries 
            WHERE content LIKE '%' || ? || '%'
            ORDER BY confidence DESC, usage_count DESC
            "#,
            query
        )
        .fetch_all(&self.pool)
        .await?;
        
        Ok(knowledge)
    }
    
    // Collaboration operations
    pub async fn record_collaboration(
        &self,
        initiator: &str,
        participant: &str,
        collaboration_type: &str,
    ) -> Result<Collaboration> {
        let initiator_agent = self.get_agent(initiator).await?;
        let participant_agent = self.get_agent(participant).await?;
        
        let collaboration = sqlx::query_as!(
            Collaboration,
            r#"
            INSERT INTO collaborations (
                initiator_agent_id, participant_agent_id, 
                started_at, collaboration_type, status
            )
            VALUES (?1, ?2, ?3, ?4, 'active')
            RETURNING *
            "#,
            initiator_agent.id,
            participant_agent.id,
            Utc::now(),
            collaboration_type
        )
        .fetch_one(&self.pool)
        .await?;
        
        Ok(collaboration)
    }
}