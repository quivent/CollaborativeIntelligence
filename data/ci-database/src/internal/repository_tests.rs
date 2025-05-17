#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::SqlitePool;
    use tempfile::NamedTempFile;
    
    async fn create_test_db() -> Pool<Sqlite> {
        let temp_file = NamedTempFile::new().unwrap();
        let db_url = format!("sqlite:{}", temp_file.path().display());
        let pool = SqlitePool::connect(&db_url).await.unwrap();
        
        // Create test schema
        sqlx::query(
            r#"
            CREATE TABLE agents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                type TEXT NOT NULL,
                status TEXT DEFAULT 'active',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                last_active TIMESTAMP,
                activation_count INTEGER DEFAULT 0,
                memory_version INTEGER DEFAULT 1,
                capabilities JSON,
                memory_path TEXT,
                config JSON
            )
            "#
        )
        .execute(&pool)
        .await
        .unwrap();
        
        pool
    }
    
    #[tokio::test]
    async fn test_create_and_get_agent() {
        let pool = create_test_db().await;
        let repo = InternalRepository::new(pool);
        
        let req = CreateAgentRequest {
            name: "TestAgent".to_string(),
            agent_type: "test".to_string(),
            capabilities: Some(serde_json::json!({"test": true})),
            memory_path: None,
            config: None,
        };
        
        let created = repo.create_agent(req).await.unwrap();
        assert_eq!(created.name, "TestAgent");
        
        let retrieved = repo.get_agent("TestAgent").await.unwrap();
        assert_eq!(retrieved.name, created.name);
        assert_eq!(retrieved.agent_type, created.agent_type);
    }
    
    #[tokio::test]
    async fn test_agent_not_found() {
        let pool = create_test_db().await;
        let repo = InternalRepository::new(pool);
        
        let result = repo.get_agent("NonExistent").await;
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(e.to_string().contains("not found"));
        }
    }
}