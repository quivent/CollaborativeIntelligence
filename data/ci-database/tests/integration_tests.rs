use ci_database::{Database, CIDatabaseAPI};
use ci_database::internal::{CreateAgentRequest, RecordLearningRequest, CreateSessionRequest, AddKnowledgeRequest};
use ci_database::projects::{RegisterProjectRequest, LogOperationRequest, RecordMetricRequest};
use tempfile::TempDir;
use serial_test::serial;

#[tokio::test]
#[serial]
async fn test_agent_lifecycle() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    // Initialize database
    db.initialize().await.unwrap();
    
    // Test agent creation
    let req = CreateAgentRequest {
        name: "TestAgent".to_string(),
        agent_type: "test-type".to_string(),
        capabilities: Some(serde_json::json!({"test": true})),
        memory_path: Some("/test/path".to_string()),
        config: None,
    };
    
    let agent = api.create_agent(req).await.unwrap();
    assert_eq!(agent.name, "TestAgent");
    assert_eq!(agent.status, "active");
    
    // Test agent retrieval
    let retrieved = api.get_agent("TestAgent").await.unwrap();
    assert_eq!(retrieved.name, agent.name);
    
    // Test agent activation/deactivation
    api.deactivate_agent("TestAgent").await.unwrap();
    let deactivated = api.get_agent("TestAgent").await.unwrap();
    assert_eq!(deactivated.status, "inactive");
    
    api.activate_agent("TestAgent").await.unwrap();
    let activated = api.get_agent("TestAgent").await.unwrap();
    assert_eq!(activated.status, "active");
    
    // Test agent listing
    let agents = api.list_agents().await.unwrap();
    assert_eq!(agents.len(), 1);
}

#[tokio::test]
#[serial]
async fn test_learning_system() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Create an agent first
    let agent_req = CreateAgentRequest {
        name: "LearningAgent".to_string(),
        agent_type: "learner".to_string(),
        capabilities: None,
        memory_path: None,
        config: None,
    };
    api.create_agent(agent_req).await.unwrap();
    
    // Record learning
    let learning_req = RecordLearningRequest {
        agent_name: "LearningAgent".to_string(),
        trigger_type: "test_trigger".to_string(),
        content: "Test learning content".to_string(),
        importance: Some(0.8),
        category: Some("test_category".to_string()),
        metadata: Some(serde_json::json!({"test": true})),
    };
    
    let learning = api.record_learning(learning_req).await.unwrap();
    assert_eq!(learning.content, "Test learning content");
    assert_eq!(learning.importance, 0.8);
    
    // Get learnings
    let learnings = api.get_learnings("LearningAgent").await.unwrap();
    assert_eq!(learnings.len(), 1);
    assert_eq!(learnings[0].trigger_type, "test_trigger");
}

#[tokio::test]
#[serial]
async fn test_session_management() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Create an agent
    let agent_req = CreateAgentRequest {
        name: "SessionAgent".to_string(),
        agent_type: "worker".to_string(),
        capabilities: None,
        memory_path: None,
        config: None,
    };
    api.create_agent(agent_req).await.unwrap();
    
    // Create session
    let session_req = CreateSessionRequest {
        agent_name: "SessionAgent".to_string(),
        session_type: "test_session".to_string(),
        metadata: Some(serde_json::json!({"purpose": "testing"})),
    };
    
    let session = api.create_session(session_req).await.unwrap();
    assert_eq!(session.status, "active");
    
    // Close session
    let outcomes = serde_json::json!({"result": "success"});
    api.close_session(session.id.unwrap(), Some(outcomes)).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_knowledge_base() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Create an agent
    let agent_req = CreateAgentRequest {
        name: "KnowledgeAgent".to_string(),
        agent_type: "expert".to_string(),
        capabilities: None,
        memory_path: None,
        config: None,
    };
    api.create_agent(agent_req).await.unwrap();
    
    // Add knowledge
    let knowledge_req = AddKnowledgeRequest {
        agent_name: "KnowledgeAgent".to_string(),
        knowledge_type: "technical".to_string(),
        content: "Rust is a systems programming language".to_string(),
        category: Some("programming".to_string()),
        confidence: Some(0.95),
        metadata: None,
    };
    
    let knowledge = api.add_knowledge(knowledge_req).await.unwrap();
    assert_eq!(knowledge.confidence, 0.95);
    
    // Search knowledge
    let results = api.search_knowledge("Rust").await.unwrap();
    assert_eq!(results.len(), 1);
    assert!(results[0].content.contains("Rust"));
}

#[tokio::test]
#[serial]
async fn test_project_management() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Register project
    let project_req = RegisterProjectRequest {
        name: "TestProject".to_string(),
        path: "/test/project".to_string(),
        project_type: Some("rust".to_string()),
        ci_version: Some("1.0.0".to_string()),
        metadata: None,
        config: None,
    };
    
    let project = api.register_project(project_req).await.unwrap();
    assert_eq!(project.name, "TestProject");
    assert_eq!(project.status, "active");
    
    // Get project
    let retrieved = api.get_project("TestProject").await.unwrap();
    assert_eq!(retrieved.path, "/test/project");
    
    // List projects
    let projects = api.list_projects().await.unwrap();
    assert_eq!(projects.len(), 1);
}

#[tokio::test]
#[serial]
async fn test_operation_logging() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Register project first
    let project_req = RegisterProjectRequest {
        name: "OpProject".to_string(),
        path: "/ops/project".to_string(),
        project_type: None,
        ci_version: None,
        metadata: None,
        config: None,
    };
    api.register_project(project_req).await.unwrap();
    
    // Log operation
    let op_req = LogOperationRequest {
        project_name: "OpProject".to_string(),
        command: "test command".to_string(),
        arguments: Some(serde_json::json!({"arg": "value"})),
        agent_used: Some("TestAgent".to_string()),
        metadata: None,
    };
    
    let operation = api.log_operation(op_req).await.unwrap();
    assert_eq!(operation.command, "test command");
    assert_eq!(operation.status, "started");
    
    // Complete operation
    api.complete_operation(
        operation.id.unwrap(),
        "completed",
        1500,
        None
    ).await.unwrap();
}

#[tokio::test]
#[serial]
async fn test_metrics() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Register project
    let project_req = RegisterProjectRequest {
        name: "MetricsProject".to_string(),
        path: "/metrics/project".to_string(),
        project_type: None,
        ci_version: None,
        metadata: None,
        config: None,
    };
    api.register_project(project_req).await.unwrap();
    
    // Record metrics
    let metric_req = RecordMetricRequest {
        project_name: "MetricsProject".to_string(),
        metric_type: "performance".to_string(),
        value: 95.5,
        context: Some(serde_json::json!({"unit": "percent"})),
    };
    
    let metric = api.record_metric(metric_req).await.unwrap();
    assert_eq!(metric.value, 95.5);
    
    // Get metrics
    let metrics = api.get_metrics("MetricsProject", "performance").await.unwrap();
    assert_eq!(metrics.len(), 1);
    assert_eq!(metrics[0].metric_type, "performance");
}

#[tokio::test]
#[serial]
async fn test_concurrent_operations() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Create multiple agents concurrently
    let mut handles = vec![];
    
    for i in 0..5 {
        let api_clone = api.clone();
        let handle = tokio::spawn(async move {
            let req = CreateAgentRequest {
                name: format!("ConcurrentAgent{}", i),
                agent_type: "concurrent".to_string(),
                capabilities: None,
                memory_path: None,
                config: None,
            };
            api_clone.create_agent(req).await
        });
        handles.push(handle);
    }
    
    // Wait for all to complete
    for handle in handles {
        handle.await.unwrap().unwrap();
    }
    
    // Verify all agents were created
    let agents = api.list_agents().await.unwrap();
    assert_eq!(agents.len(), 5);
}

#[tokio::test]
#[serial]
async fn test_error_handling() {
    let temp_dir = TempDir::new().unwrap();
    let db = Database::new(temp_dir.path()).await.unwrap();
    let api = CIDatabaseAPI::new(db.clone());
    
    db.initialize().await.unwrap();
    
    // Test non-existent agent
    let result = api.get_agent("NonExistent").await;
    assert!(result.is_err());
    
    // Test duplicate agent creation
    let req = CreateAgentRequest {
        name: "DuplicateAgent".to_string(),
        agent_type: "test".to_string(),
        capabilities: None,
        memory_path: None,
        config: None,
    };
    
    api.create_agent(req.clone()).await.unwrap();
    let duplicate_result = api.create_agent(req).await;
    assert!(duplicate_result.is_err());
}