use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use sqlx::FromRow;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Project {
    pub id: Option<i64>,
    pub name: String,
    pub path: String,
    #[sqlx(rename = "type")]
    pub project_type: Option<String>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub last_accessed: Option<DateTime<Utc>>,
    pub ci_version: Option<String>,
    pub metadata: Option<JsonValue>,
    pub config: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Operation {
    pub id: Option<i64>,
    pub project_id: i64,
    pub timestamp: DateTime<Utc>,
    pub command: String,
    pub arguments: Option<JsonValue>,
    pub agent_used: Option<String>,
    pub status: String,
    pub duration_ms: Option<i64>,
    pub error_message: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Integration {
    pub id: Option<i64>,
    pub project_id: i64,
    pub integration_type: String,
    pub config: Option<JsonValue>,
    pub status: String,
    pub created_at: DateTime<Utc>,
    pub last_sync: Option<DateTime<Utc>>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Metric {
    pub id: Option<i64>,
    pub project_id: i64,
    pub metric_type: String,
    pub value: f64,
    pub timestamp: DateTime<Utc>,
    pub context: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct FeatureUsage {
    pub id: Option<i64>,
    pub project_id: i64,
    pub feature_name: String,
    pub first_used: DateTime<Utc>,
    pub last_used: DateTime<Utc>,
    pub usage_count: i32,
    pub preferences: Option<JsonValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ProjectAgent {
    pub id: Option<i64>,
    pub project_id: i64,
    pub agent_name: String,
    pub enabled: bool,
    pub custom_config: Option<JsonValue>,
    pub last_used: Option<DateTime<Utc>>,
}

// Request/Response types
#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterProjectRequest {
    pub name: String,
    pub path: String,
    pub project_type: Option<String>,
    pub ci_version: Option<String>,
    pub metadata: Option<JsonValue>,
    pub config: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogOperationRequest {
    pub project_name: String,
    pub command: String,
    pub arguments: Option<JsonValue>,
    pub agent_used: Option<String>,
    pub metadata: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecordMetricRequest {
    pub project_name: String,
    pub metric_type: String,
    pub value: f64,
    pub context: Option<JsonValue>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectAgentConfigRequest {
    pub project_name: String,
    pub agent_name: String,
    pub enabled: bool,
    pub custom_config: Option<JsonValue>,
}