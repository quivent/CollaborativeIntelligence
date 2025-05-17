use ci_database::ci_integration::{CIDatabaseIntegration, helpers};
use ci_database::internal::CreateAgentRequest;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize database integration
    let integration = CIDatabaseIntegration::new("./data").await?;
    
    // Initialize the database
    let db = ci_database::Database::new("./data").await?;
    db.initialize().await?;
    
    // Register an agent
    let agent_req = CreateAgentRequest {
        name: "ExampleAgent".to_string(),
        agent_type: "example".to_string(),
        capabilities: Some(serde_json::json!({
            "language": "rust",
            "features": ["database", "cli"]
        })),
        memory_path: None,
        config: None,
    };
    
    integration.api().create_agent(agent_req).await?;
    
    // Initialize a project
    helpers::init_project(&integration, "ExampleProject", "/example/path", "1.0.0").await?;
    
    // Simulate a CI command execution
    println!("Starting CI command...");
    let op_id = integration.begin_command(
        "ExampleProject",
        "ci build",
        Some("ExampleAgent")
    ).await?;
    
    // Simulate some work
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    // Record a learning event
    integration.record_ci_learning(
        "ExampleAgent",
        "Build completed successfully with optimizations",
        "build_optimization"
    ).await?;
    
    // Complete the command
    integration.complete_command(op_id, true, 100, None).await?;
    
    // Record a metric
    helpers::record_metric(&integration, "ExampleProject", "build_time", 100.0).await?;
    
    println!("CI command completed successfully!");
    
    // Query some data
    let agents = integration.api().list_agents().await?;
    println!("Registered agents: {}", agents.len());
    
    let projects = integration.api().list_projects().await?;
    println!("Registered projects: {}", projects.len());
    
    let metrics = integration.api().get_metrics("ExampleProject", "build_time").await?;
    println!("Build time metrics: {}", metrics.len());
    
    Ok(())
}