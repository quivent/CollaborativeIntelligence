use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Agent {
    pub id: Option<i64>,
    pub name: String,
    #[sqlx(rename = "type")]
    pub agent_type: String,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_active: Option<DateTime<Utc>>,
    pub activation_count: i32,
    pub memory_version: i32,
    pub capabilities: Option<JsonValue>,
    pub memory_path: Option<String>,
    pub config: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct LearningEntry {
    pub id: Option<i64>,
    pub agent_id: i64,
    pub timestamp: DateTime<Utc>,
    pub trigger_type: String,
    pub content: String,
    pub importance: f64,
    pub category: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Session {
    pub id: Option<i64>,
    pub agent_id: i64,
    pub session_type: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub status: String,
    pub metadata: Option<JsonValue>,
    pub outcomes: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KnowledgeEntry {
    pub id: Option<i64>,
    pub agent_id: i64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub knowledge_type: String,
    pub category: Option<String>,
    pub content: String,
    pub confidence: f64,
    pub usage_count: i32,
    pub last_accessed: Option<DateTime<Utc>>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Collaboration {
    pub id: Option<i64>,
    pub initiator_agent_id: i64,
    pub participant_agent_id: i64,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub collaboration_type: String,
    pub status: Option<String>,
    pub outcomes: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MemorySnapshot {
    pub id: Option<i64>,
    pub agent_id: i64,
    pub created_at: DateTime<Utc>,
    pub version: i32,
    pub memory_type: String,
    pub content: String,
    pub hash: Option<String>,
}

// Request/Response types for API
#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAgentRequest {
    pub name: String,
    pub agent_type: String,
    pub capabilities: Option<JsonValue>,
    pub memory_path: Option<String>,
    pub config: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordLearningRequest {
    pub agent_name: String,
    pub trigger_type: String,
    pub content: String,
    pub importance: Option<f64>,
    pub category: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionRequest {
    pub agent_name: String,
    pub session_type: String,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddKnowledgeRequest {
    pub agent_name: String,
    pub knowledge_type: String,
    pub content: String,
    pub category: Option<String>,
    pub confidence: Option<f64>,
    pub metadata: Option<JsonValue>,
}