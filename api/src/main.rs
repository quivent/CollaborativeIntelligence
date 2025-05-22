use axum::{
    extract::{Path, State},
    response::Json,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use chrono::{DateTime, Utc};

mod content_service;
mod document_registry;
mod error;

use content_service::ContentService;
use document_registry::DocumentType;
use error::{ApiError, ApiResult};

#[derive(Clone)]
pub struct AppState {
    content_service: Arc<ContentService>,
}

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    timestamp: DateTime<Utc>,
    version: String,
}

#[derive(Serialize)]
struct AgentsResponse {
    content: String,
    format: String,
    agent_count: usize,
    active_count: usize,
    inactive_count: usize,
    agents: Vec<AgentInfo>,
}

#[derive(Serialize, Deserialize)]
pub struct AgentInfo {
    pub name: String,
    pub path: String,
    pub status: String,
    pub has_readme: bool,
    pub has_metadata: bool,
    pub description: Option<String>,
}

#[derive(Serialize)]
struct DocumentResponse {
    content: String,
    format: String,
    metadata: DocumentMetadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DocumentMetadata {
    pub path: String,
    pub relative_path: String,
    pub last_modified: DateTime<Utc>,
    pub size: u64,
}

async fn health_check() -> ApiResult<Json<HealthResponse>> {
    Ok(Json(HealthResponse {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        version: "1.0.0".to_string(),
    }))
}

async fn get_agents(State(state): State<AppState>) -> ApiResult<Json<AgentsResponse>> {
    let agents = state.content_service.get_agent_list().await?;
    
    let active_agents: Vec<_> = agents.iter().filter(|a| a.status == "active").collect();
    let inactive_agents: Vec<_> = agents.iter().filter(|a| a.status == "inactive").collect();
    
    let mut formatted_output = Vec::new();
    formatted_output.push("# 🤖 Available Agents\n".to_string());
    
    if agents.is_empty() {
        formatted_output.push("No agents found.\n".to_string());
        return Ok(Json(AgentsResponse {
            content: formatted_output.join(""),
            format: "markdown".to_string(),
            agent_count: 0,
            active_count: 0,
            inactive_count: 0,
            agents,
        }));
    }
    
    if !active_agents.is_empty() {
        formatted_output.push("## ✅ Active Agents\n".to_string());
        for agent in &active_agents {
            let status_icon = if agent.has_readme { "🟢" } else { "🟡" };
            let mut line = format!("{} **{}**", status_icon, agent.name);
            if let Some(ref desc) = agent.description {
                line.push_str(&format!(" - {}", desc));
            }
            line.push('\n');
            formatted_output.push(line);
        }
    }
    
    if !inactive_agents.is_empty() {
        formatted_output.push("\n## ⚠️ Inactive Agents\n".to_string());
        for agent in &inactive_agents {
            formatted_output.push(format!("🔴 **{}** - No README found\n", agent.name));
        }
    }
    
    formatted_output.push(format!(
        "\n📊 **Total**: {} agents ({} active, {} inactive)\n",
        agents.len(),
        active_agents.len(),
        inactive_agents.len()
    ));
    
    Ok(Json(AgentsResponse {
        content: formatted_output.join(""),
        format: "markdown".to_string(),
        agent_count: agents.len(),
        active_count: active_agents.len(),
        inactive_count: inactive_agents.len(),
        agents,
    }))
}

async fn get_agent_info(
    Path(agent_name): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<serde_json::Value>> {
    let agents = state.content_service.get_agent_list().await?;
    let agent = agents
        .iter()
        .find(|a| a.name == agent_name)
        .ok_or_else(|| ApiError::NotFound(format!("Agent '{}' not found", agent_name)))?;
    
    let readme_content = if agent.has_readme {
        state.content_service.get_agent_readme(&agent_name).await.unwrap_or_else(|_| "Error reading README.md".to_string())
    } else {
        String::new()
    };
    
    let response = serde_json::json!({
        "agent": agent,
        "readme_content": readme_content,
        "format": "markdown"
    });
    
    Ok(Json(response))
}

async fn get_document(
    Path(document_type): Path<String>,
    State(state): State<AppState>,
) -> ApiResult<Json<DocumentResponse>> {
    let doc_type = match document_type.as_str() {
        "protocols" => DocumentType::Protocols,
        "architecture" => DocumentType::Architecture,
        "agents_overview" => DocumentType::AgentsOverview,
        _ => return Err(ApiError::BadRequest(format!(
            "Invalid document type '{}'. Valid types: protocols, architecture, agents_overview",
            document_type
        ))),
    };
    
    let (content, metadata) = state.content_service.get_document_content(doc_type).await?;
    
    Ok(Json(DocumentResponse {
        content,
        format: "markdown".to_string(),
        metadata,
    }))
}

async fn get_registry(State(state): State<AppState>) -> ApiResult<Json<serde_json::Value>> {
    let registry = state.content_service.get_document_registry().await?;
    Ok(Json(serde_json::to_value(registry)?))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    
    let base_path = std::env::current_dir()?;
    let content_service = Arc::new(ContentService::new(base_path)?);
    
    let state = AppState {
        content_service,
    };
    
    let app = Router::new()
        .route("/api/health", get(health_check))
        .route("/api/agents", get(get_agents))
        .route("/api/agents/:agent_name", get(get_agent_info))
        .route("/api/documents/:document_type", get(get_document))
        .route("/api/registry", get(get_registry))
        .layer(CorsLayer::permissive())
        .with_state(state);
    
    let port = std::env::var("CI_API_PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .unwrap_or(8080);
    
    let listener = tokio::net::TcpListener::bind(format!("127.0.0.1:{}", port)).await?;
    
    tracing::info!("CollaborativeIntelligence API server starting on port {}", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}