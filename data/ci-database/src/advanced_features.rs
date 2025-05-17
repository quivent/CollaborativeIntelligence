use crate::{Database, Result, Error};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

/// Advanced database features
pub struct AdvancedFeatures {
    db: Database,
}

impl AdvancedFeatures {
    pub fn new(db: Database) -> Self {
        Self { db }
    }
    
    /// Export database to JSON format
    pub async fn export_to_json(&self, include_sensitive: bool) -> Result<DatabaseExport> {
        // Export agents
        let agents = sqlx::query!(
            "SELECT * FROM agents ORDER BY name"
        )
        .fetch_all(self.db.internal())
        .await?;
        
        // Export projects
        let projects = sqlx::query!(
            "SELECT * FROM projects ORDER BY name"
        )
        .fetch_all(self.db.projects())
        .await?;
        
        // Export learnings (limit to recent)
        let learnings = sqlx::query!(
            "SELECT * FROM learning_entries 
             WHERE timestamp > datetime('now', '-30 days')
             ORDER BY timestamp DESC"
        )
        .fetch_all(self.db.internal())
        .await?;
        
        Ok(DatabaseExport {
            exported_at: Utc::now(),
            version: "1.0.0".to_string(),
            agents: agents.into_iter().map(|a| serde_json::json!({
                "name": a.name,
                "type": a.type_,
                "status": a.status,
                "activation_count": a.activation_count,
            })).collect(),
            projects: projects.into_iter().map(|p| serde_json::json!({
                "name": p.name,
                "path": p.path,
                "status": p.status,
                "ci_version": p.ci_version,
            })).collect(),
            learnings: if include_sensitive {
                learnings.into_iter().map(|l| serde_json::json!({
                    "agent_id": l.agent_id,
                    "trigger_type": l.trigger_type,
                    "content": l.content,
                    "importance": l.importance,
                })).collect()
            } else {
                vec![]
            },
        })
    }
    
    /// Import data from JSON
    pub async fn import_from_json(&self, data: &str) -> Result<ImportSummary> {
        let import_data: DatabaseExport = serde_json::from_str(data)?;
        let mut summary = ImportSummary::default();
        
        // Import agents
        for agent_json in &import_data.agents {
            if let Ok(agent) = serde_json::from_value::<ImportAgent>(agent_json.clone()) {
                let existing = sqlx::query!(
                    "SELECT id FROM agents WHERE name = ?",
                    agent.name
                )
                .fetch_optional(self.db.internal())
                .await?;
                
                if existing.is_none() {
                    sqlx::query!(
                        "INSERT INTO agents (name, type, status) VALUES (?, ?, ?)",
                        agent.name,
                        agent.agent_type,
                        agent.status
                    )
                    .execute(self.db.internal())
                    .await?;
                    
                    summary.agents_imported += 1;
                } else {
                    summary.agents_skipped += 1;
                }
            }
        }
        
        Ok(summary)
    }
    
    /// Generate analytics report
    pub async fn generate_analytics(&self, days: i64) -> Result<AnalyticsReport> {
        let since = Utc::now() - Duration::days(days);
        
        // Agent activity
        let agent_activity = sqlx::query!(
            r#"
            SELECT 
                a.name as agent_name,
                COUNT(s.id) as session_count,
                AVG(JULIANDAY(s.ended_at) - JULIANDAY(s.started_at)) * 24 * 60 as avg_duration_min
            FROM agents a
            LEFT JOIN sessions s ON a.id = s.agent_id
            WHERE s.started_at > ?
            GROUP BY a.id
            ORDER BY session_count DESC
            "#,
            since
        )
        .fetch_all(self.db.internal())
        .await?;
        
        // Project activity
        let project_activity = sqlx::query!(
            r#"
            SELECT 
                p.name as project_name,
                COUNT(o.id) as operation_count,
                AVG(o.duration_ms) as avg_duration_ms
            FROM projects p
            LEFT JOIN operations o ON p.id = o.project_id
            WHERE o.timestamp > ?
            GROUP BY p.id
            ORDER BY operation_count DESC
            "#,
            since
        )
        .fetch_all(self.db.projects())
        .await?;
        
        // Learning trends
        let learning_trends = sqlx::query!(
            r#"
            SELECT 
                DATE(timestamp) as date,
                COUNT(*) as learning_count,
                AVG(importance) as avg_importance
            FROM learning_entries
            WHERE timestamp > ?
            GROUP BY DATE(timestamp)
            ORDER BY date
            "#,
            since
        )
        .fetch_all(self.db.internal())
        .await?;
        
        Ok(AnalyticsReport {
            period_start: since,
            period_end: Utc::now(),
            agent_activity: agent_activity.into_iter().map(|a| AgentActivity {
                agent_name: a.agent_name.unwrap_or_default(),
                session_count: a.session_count.unwrap_or(0) as u32,
                avg_duration_minutes: a.avg_duration_min.unwrap_or(0.0),
            }).collect(),
            project_activity: project_activity.into_iter().map(|p| ProjectActivity {
                project_name: p.project_name,
                operation_count: p.operation_count.unwrap_or(0) as u32,
                avg_duration_ms: p.avg_duration_ms.unwrap_or(0.0),
            }).collect(),
            learning_trends: learning_trends.into_iter().map(|l| LearningTrend {
                date: l.date.unwrap_or_default(),
                count: l.learning_count as u32,
                avg_importance: l.avg_importance.unwrap_or(0.0),
            }).collect(),
        })
    }
    
    /// Automated backup with retention
    pub async fn automated_backup(&self, backup_dir: &str, retention_days: i64) -> Result<BackupSummary> {
        use std::path::Path;
        use std::fs;
        
        let backup_path = Path::new(backup_dir);
        fs::create_dir_all(backup_path)?;
        
        // Perform backup
        self.db.backup(backup_path).await?;
        
        // Clean old backups
        let cutoff = Utc::now() - Duration::days(retention_days);
        let mut deleted_count = 0;
        
        for entry in fs::read_dir(backup_path)? {
            let entry = entry?;
            let metadata = entry.metadata()?;
            
            if let Ok(modified) = metadata.modified() {
                let modified_time: DateTime<Utc> = modified.into();
                if modified_time < cutoff {
                    fs::remove_file(entry.path())?;
                    deleted_count += 1;
                }
            }
        }
        
        Ok(BackupSummary {
            backup_time: Utc::now(),
            old_backups_deleted: deleted_count,
        })
    }
    
    /// Health check with detailed diagnostics
    pub async fn health_check(&self) -> Result<HealthReport> {
        // Check database connections
        let internal_ok = sqlx::query("SELECT 1")
            .fetch_optional(self.db.internal())
            .await
            .is_ok();
            
        let projects_ok = sqlx::query("SELECT 1")
            .fetch_optional(self.db.projects())
            .await
            .is_ok();
        
        // Get database sizes
        let internal_size = sqlx::query_scalar::<_, i64>(
            "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()"
        )
        .fetch_one(self.db.internal())
        .await?;
        
        let projects_size = sqlx::query_scalar::<_, i64>(
            "SELECT page_count * page_size FROM pragma_page_count(), pragma_page_size()"
        )
        .fetch_one(self.db.projects())
        .await?;
        
        // Count records
        let agent_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM agents")
            .fetch_one(self.db.internal())
            .await?;
            
        let project_count = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM projects")
            .fetch_one(self.db.projects())
            .await?;
        
        Ok(HealthReport {
            status: if internal_ok && projects_ok { "healthy" } else { "degraded" },
            internal_db_ok,
            projects_db_ok,
            internal_size_bytes: internal_size,
            projects_size_bytes: projects_size,
            total_agents: agent_count,
            total_projects: project_count,
            last_check: Utc::now(),
        })
    }
    
    /// Knowledge graph visualization data
    pub async fn knowledge_graph_data(&self) -> Result<KnowledgeGraph> {
        // Get agents as nodes
        let agents = sqlx::query!(
            "SELECT id, name, type FROM agents WHERE status = 'active'"
        )
        .fetch_all(self.db.internal())
        .await?;
        
        // Get collaborations as edges
        let collaborations = sqlx::query!(
            r#"
            SELECT 
                c.initiator_agent_id,
                c.participant_agent_id,
                c.collaboration_type,
                COUNT(*) as weight
            FROM collaborations c
            WHERE c.status = 'completed'
            GROUP BY c.initiator_agent_id, c.participant_agent_id, c.collaboration_type
            "#
        )
        .fetch_all(self.db.internal())
        .await?;
        
        let nodes = agents.into_iter().map(|a| GraphNode {
            id: a.id.unwrap() as u64,
            label: a.name,
            node_type: a.type_,
        }).collect();
        
        let edges = collaborations.into_iter().map(|c| GraphEdge {
            source: c.initiator_agent_id as u64,
            target: c.participant_agent_id as u64,
            edge_type: c.collaboration_type,
            weight: c.weight.unwrap_or(1) as u32,
        }).collect();
        
        Ok(KnowledgeGraph { nodes, edges })
    }
}

// Data structures for advanced features

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseExport {
    pub exported_at: DateTime<Utc>,
    pub version: String,
    pub agents: Vec<serde_json::Value>,
    pub projects: Vec<serde_json::Value>,
    pub learnings: Vec<serde_json::Value>,
}

#[derive(Debug, Default)]
pub struct ImportSummary {
    pub agents_imported: u32,
    pub agents_skipped: u32,
    pub projects_imported: u32,
    pub projects_skipped: u32,
}

#[derive(Debug, Deserialize)]
struct ImportAgent {
    name: String,
    #[serde(rename = "type")]
    agent_type: String,
    status: String,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsReport {
    pub period_start: DateTime<Utc>,
    pub period_end: DateTime<Utc>,
    pub agent_activity: Vec<AgentActivity>,
    pub project_activity: Vec<ProjectActivity>,
    pub learning_trends: Vec<LearningTrend>,
}

#[derive(Debug, Serialize)]
pub struct AgentActivity {
    pub agent_name: String,
    pub session_count: u32,
    pub avg_duration_minutes: f64,
}

#[derive(Debug, Serialize)]
pub struct ProjectActivity {
    pub project_name: String,
    pub operation_count: u32,
    pub avg_duration_ms: f64,
}

#[derive(Debug, Serialize)]
pub struct LearningTrend {
    pub date: String,
    pub count: u32,
    pub avg_importance: f64,
}

#[derive(Debug, Serialize)]
pub struct BackupSummary {
    pub backup_time: DateTime<Utc>,
    pub old_backups_deleted: u32,
}

#[derive(Debug, Serialize)]
pub struct HealthReport {
    pub status: &'static str,
    pub internal_db_ok: bool,
    pub projects_db_ok: bool,
    pub internal_size_bytes: i64,
    pub projects_size_bytes: i64,
    pub total_agents: i64,
    pub total_projects: i64,
    pub last_check: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct KnowledgeGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

#[derive(Debug, Serialize)]
pub struct GraphNode {
    pub id: u64,
    pub label: String,
    pub node_type: String,
}

#[derive(Debug, Serialize)]
pub struct GraphEdge {
    pub source: u64,
    pub target: u64,
    pub edge_type: String,
    pub weight: u32,
}