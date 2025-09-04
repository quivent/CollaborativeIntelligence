# BRAIN System Integration Protocol
**CollaborativeIntelligence Multi-Agent Integration Framework**

## 📋 Overview

This document defines the integration protocols, interfaces, and standards for connecting external systems, tools, and services with the BRAIN CollaborativeIntelligence system. It provides comprehensive guidance for developers, system integrators, and platform architects.

## 🏗️ Integration Architecture

### Integration Layers

```
┌─────────────────────────────────────────┐
│             External Systems            │
├─────────────────────────────────────────┤
│          Integration Layer              │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │   API   │ │   CLI   │ │  Events │   │
│  │ Gateway │ │Interface│ │  Bus    │   │
│  └─────────┘ └─────────┘ └─────────┘   │
├─────────────────────────────────────────┤
│            BRAIN Core System            │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐   │
│  │ Agents  │ │Knowledge│ │Database │   │
│  │ System  │ │  Base   │ │ Layer   │   │
│  └─────────┘ └─────────┘ └─────────┘   │
└─────────────────────────────────────────┘
```

### Integration Types

1. **API Integration**: RESTful API access to system functionality
2. **CLI Integration**: Command-line interface for automation
3. **Event-Driven Integration**: Asynchronous event handling
4. **Database Integration**: Direct database access patterns
5. **Agent Integration**: Custom agent development and deployment
6. **Knowledge Integration**: Content submission and retrieval

## 🔌 API Integration

### REST API Endpoints

#### Agent Management
```http
# List available agents
GET /api/v1/agents
GET /api/v1/agents/{agent_name}
GET /api/v1/agents/{agent_name}/capabilities

# Activate agents
POST /api/v1/agents/{agent_name}/activate
POST /api/v1/agents/batch/activate

# Agent status and metrics
GET /api/v1/agents/{agent_name}/status
GET /api/v1/agents/{agent_name}/metrics
GET /api/v1/agents/{agent_name}/sessions
```

#### Knowledge Management
```http
# Query knowledge base
GET /api/v1/knowledge/query?q={query}&domain={domain}
GET /api/v1/knowledge/browse/{category}
GET /api/v1/knowledge/recent

# Submit knowledge
POST /api/v1/knowledge/submit
PUT /api/v1/knowledge/{id}
DELETE /api/v1/knowledge/{id}

# Knowledge processing
GET /api/v1/knowledge/intake/status
GET /api/v1/knowledge/intake/queue
POST /api/v1/knowledge/intake/process
```

#### System Operations
```http
# System health and status
GET /api/v1/system/health
GET /api/v1/system/status
GET /api/v1/system/metrics

# Project management
GET /api/v1/projects
POST /api/v1/projects
GET /api/v1/projects/{project_id}
DELETE /api/v1/projects/{project_id}
```

### API Authentication

#### JWT Token Authentication
```http
POST /api/v1/auth/login
Content-Type: application/json

{
  "username": "user@example.com",
  "password": "secure_password",
  "scope": "read write admin"
}

Response:
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "expires_in": 3600,
  "token_type": "Bearer"
}
```

#### API Key Authentication
```http
GET /api/v1/agents
Authorization: Bearer {api_key}
X-Client-ID: your_client_id
```

### Request/Response Formats

#### Standard Request Format
```json
{
  "request_id": "req_123456789",
  "timestamp": "2025-08-30T10:30:00Z",
  "client_id": "external_system_1",
  "version": "1.0",
  "data": {
    // Request-specific data
  }
}
```

#### Standard Response Format
```json
{
  "request_id": "req_123456789",
  "timestamp": "2025-08-30T10:30:01Z",
  "status": "success|error|pending",
  "message": "Operation completed successfully",
  "data": {
    // Response data
  },
  "metadata": {
    "processing_time_ms": 150,
    "agent_used": "Analyst",
    "cache_hit": true
  }
}
```

### Error Handling
```json
{
  "error": {
    "code": "AGENT_NOT_FOUND",
    "message": "Agent 'NonExistentAgent' not found",
    "details": {
      "available_agents": ["Architect", "Developer", "Analyst"],
      "suggestion": "Check agent name spelling"
    },
    "timestamp": "2025-08-30T10:30:01Z"
  }
}
```

## 🖥️ CLI Integration

### Command Structure
```bash
# Standard command format
ci <command> [subcommand] [options] [arguments]

# Integration-friendly commands
ci --json <command>      # JSON output
ci --batch <commands>    # Batch execution
ci --script <script>     # Script execution
ci --webhook <url>       # Result webhooks
```

### Automation Scripts
```bash
#!/bin/bash
# Example integration script

# Set API endpoint
export CI_API_ENDPOINT="http://localhost:8080"
export CI_API_KEY="your_api_key"

# Load agent and execute task
AGENT_STATUS=$(ci load Architect --json)
if [ $(echo $AGENT_STATUS | jq -r '.status') = "success" ]; then
    ci execute --agent Architect --task "analyze_architecture" --output results.json
    ci webhook --url "https://external-system.com/callback" --data results.json
fi
```

### Integration Hooks
```bash
# Pre-execution hooks
ci hooks add pre-execute "/path/to/validation_script.sh"

# Post-execution hooks  
ci hooks add post-execute "/path/to/cleanup_script.sh"

# Event hooks
ci hooks add agent-loaded "/path/to/notification_script.sh"
```

## 📡 Event-Driven Integration

### Event Types
```typescript
interface SystemEvent {
  event_id: string;
  event_type: EventType;
  timestamp: string;
  source: string;
  data: EventData;
  metadata?: EventMetadata;
}

enum EventType {
  AGENT_ACTIVATED = "agent.activated",
  AGENT_DEACTIVATED = "agent.deactivated",
  KNOWLEDGE_SUBMITTED = "knowledge.submitted",
  KNOWLEDGE_PROCESSED = "knowledge.processed",
  SESSION_STARTED = "session.started",
  SESSION_COMPLETED = "session.completed",
  SYSTEM_ERROR = "system.error",
  PERFORMANCE_ALERT = "performance.alert"
}
```

### Event Subscription
```bash
# Subscribe to events via webhook
ci events subscribe --type "agent.*" --webhook "https://your-system.com/events"

# Subscribe to events via WebSocket
ci events subscribe --type "knowledge.*" --websocket "wss://your-system.com/ws"

# File-based event subscription
ci events subscribe --type "*" --file "/path/to/events.log"
```

### Event Processing Examples
```python
# Python webhook handler
from flask import Flask, request
import json

app = Flask(__name__)

@app.route('/events', methods=['POST'])
def handle_event():
    event = request.get_json()
    
    if event['event_type'] == 'agent.activated':
        # Handle agent activation
        agent_name = event['data']['agent_name']
        print(f"Agent {agent_name} activated")
        
    elif event['event_type'] == 'knowledge.processed':
        # Handle knowledge processing
        knowledge_id = event['data']['knowledge_id']
        status = event['data']['status']
        print(f"Knowledge {knowledge_id} processed: {status}")
    
    return {'status': 'received'}, 200
```

## 💾 Database Integration

### Direct Database Access
```sql
-- Agent information queries
SELECT * FROM agents WHERE status = 'active';
SELECT name, usage_count, last_used FROM agents ORDER BY usage_count DESC;

-- Session data queries
SELECT * FROM sessions WHERE agent_name = 'Architect' AND date > '2025-08-01';
SELECT agent_name, COUNT(*) as session_count FROM sessions GROUP BY agent_name;

-- Learning data queries
SELECT * FROM learning_entries WHERE created_date > DATE('now', '-7 days');
SELECT agent_name, insight, impact_score FROM learning_entries ORDER BY impact_score DESC;
```

### Database Connection Configuration
```rust
// Rust database configuration
use sqlx::SqlitePool;

#[derive(Clone)]
pub struct DatabaseConfig {
    pub database_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: Duration,
}

impl DatabaseConfig {
    pub fn from_env() -> Result<Self, ConfigError> {
        Ok(DatabaseConfig {
            database_url: std::env::var("CI_DATABASE_URL")?,
            max_connections: std::env::var("CI_DB_MAX_CONN")?.parse()?,
            min_connections: std::env::var("CI_DB_MIN_CONN")?.parse()?,
            connect_timeout: Duration::from_secs(30),
        })
    }
}
```

### Read-Only Access Pattern
```python
# Python read-only database access
import sqlite3
import os

class BrainDBReader:
    def __init__(self):
        db_path = os.environ.get('CI_DB_PATH', './brain.db')
        self.conn = sqlite3.connect(f"file:{db_path}?mode=ro", uri=True)
        self.conn.row_factory = sqlite3.Row
    
    def get_active_agents(self):
        cursor = self.conn.cursor()
        cursor.execute("SELECT * FROM agents WHERE status = 'active'")
        return [dict(row) for row in cursor.fetchall()]
    
    def get_recent_sessions(self, limit=10):
        cursor = self.conn.cursor()
        cursor.execute("""
            SELECT * FROM sessions 
            ORDER BY created_at DESC 
            LIMIT ?
        """, (limit,))
        return [dict(row) for row in cursor.fetchall()]
```

## 🤖 Agent Integration

### Custom Agent Development

#### Agent Interface Contract
```rust
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

#[async_trait]
pub trait Agent: Send + Sync {
    async fn activate(&mut self, context: AgentContext) -> Result<(), AgentError>;
    async fn execute(&mut self, task: Task) -> Result<TaskResult, AgentError>;
    async fn deactivate(&mut self) -> Result<(), AgentError>;
    fn get_capabilities(&self) -> Vec<Capability>;
    fn get_metadata(&self) -> AgentMetadata;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub name: String,
    pub version: String,
    pub description: String,
    pub capabilities: Vec<Capability>,
    pub dependencies: Vec<String>,
}
```

#### Agent Registration
```bash
# Register new agent
ci agents register --path "/path/to/custom_agent" --metadata agent_metadata.json

# Validate agent implementation
ci agents validate CustomAgent

# Deploy agent to system
ci agents deploy CustomAgent --environment production
```

#### Agent Template
```toml
# Cargo.toml for custom agent
[package]
name = "custom-agent"
version = "1.0.0"
edition = "2021"

[dependencies]
brain-agent-sdk = "1.0"
tokio = "1.0"
serde = { version = "1.0", features = ["derive"] }
async-trait = "0.1"
```

```rust
// src/lib.rs - Custom agent implementation
use brain_agent_sdk::{Agent, AgentContext, Task, TaskResult, AgentError};
use async_trait::async_trait;

pub struct CustomAgent {
    name: String,
    config: CustomAgentConfig,
}

#[async_trait]
impl Agent for CustomAgent {
    async fn activate(&mut self, context: AgentContext) -> Result<(), AgentError> {
        // Agent activation logic
        log::info!("Custom agent {} activated", self.name);
        Ok(())
    }
    
    async fn execute(&mut self, task: Task) -> Result<TaskResult, AgentError> {
        // Task execution logic
        let result = match task.task_type.as_str() {
            "analyze" => self.perform_analysis(task.data).await?,
            "generate" => self.generate_content(task.data).await?,
            _ => return Err(AgentError::UnsupportedTask(task.task_type)),
        };
        
        Ok(TaskResult {
            status: "completed".to_string(),
            data: result,
            metadata: self.create_metadata(),
        })
    }
    
    async fn deactivate(&mut self) -> Result<(), AgentError> {
        log::info!("Custom agent {} deactivated", self.name);
        Ok(())
    }
}
```

### Agent Deployment Pipeline
```yaml
# .github/workflows/agent-deployment.yml
name: Deploy Custom Agent

on:
  push:
    paths:
      - 'agents/custom-agent/**'

jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Build Agent
        run: |
          cd agents/custom-agent
          cargo build --release
          
      - name: Test Agent
        run: |
          ci agents test custom-agent
          
      - name: Deploy Agent
        run: |
          ci agents deploy custom-agent --environment production
          
      - name: Verify Deployment
        run: |
          ci agents verify custom-agent
```

## 📊 Knowledge Integration

### Knowledge Submission API
```http
POST /api/v1/knowledge/submit
Content-Type: application/json
Authorization: Bearer {token}

{
  "title": "Advanced Memory Architecture Patterns",
  "content": "# Content here...",
  "category": "architecture",
  "tags": ["memory", "performance", "optimization"],
  "metadata": {
    "source": "external_system",
    "author": "System Integration",
    "confidence": 0.95
  }
}
```

### Bulk Knowledge Import
```bash
# Import from file
ci knowledge import --file knowledge_export.json --format json

# Import from directory
ci knowledge import --directory "/path/to/knowledge" --recursive

# Import with validation
ci knowledge import --file data.csv --format csv --validate
```

### Knowledge Retrieval
```python
# Python knowledge retrieval example
import requests

class BrainKnowledgeClient:
    def __init__(self, api_key):
        self.api_key = api_key
        self.base_url = "http://localhost:8080/api/v1"
        
    def query(self, query, domain=None):
        params = {"q": query}
        if domain:
            params["domain"] = domain
            
        response = requests.get(
            f"{self.base_url}/knowledge/query",
            params=params,
            headers={"Authorization": f"Bearer {self.api_key}"}
        )
        return response.json()
    
    def submit(self, title, content, category):
        data = {
            "title": title,
            "content": content,
            "category": category
        }
        
        response = requests.post(
            f"{self.base_url}/knowledge/submit",
            json=data,
            headers={"Authorization": f"Bearer {self.api_key}"}
        )
        return response.json()
```

## 🔧 Integration Tools and SDKs

### Official SDKs

#### Python SDK
```python
# Installation
pip install brain-client-sdk

# Usage
from brain_client import BrainClient

client = BrainClient(api_key="your_key", endpoint="http://localhost:8080")

# Load agent
agent = client.load_agent("Architect")

# Execute task
result = agent.execute({
    "task_type": "analyze",
    "data": {"project_path": "/path/to/project"}
})

# Submit knowledge
client.knowledge.submit("Title", content, "category")
```

#### JavaScript SDK
```javascript
// Installation
npm install @brain/client-sdk

// Usage
import { BrainClient } from '@brain/client-sdk';

const client = new BrainClient({
  apiKey: 'your_key',
  endpoint: 'http://localhost:8080'
});

// Load agent
const agent = await client.loadAgent('Developer');

// Execute task
const result = await agent.execute({
  taskType: 'code_review',
  data: { code: sourceCode }
});
```

#### Rust SDK
```rust
// Cargo.toml
[dependencies]
brain-client = "1.0"

// Usage
use brain_client::{BrainClient, AgentTask};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = BrainClient::new("your_key", "http://localhost:8080");
    
    let agent = client.load_agent("Tester").await?;
    
    let task = AgentTask::new("run_tests")
        .with_data(serde_json::json!({"test_suite": "integration"}));
    
    let result = agent.execute(task).await?;
    
    Ok(())
}
```

### Integration Testing Framework
```bash
# Integration test suite
ci test integration --suite external_apis
ci test integration --suite database_access
ci test integration --suite event_handling
ci test integration --suite agent_deployment

# Load testing
ci test load --agents 10 --duration 300s --rps 100

# Compatibility testing
ci test compatibility --version 1.0 --agents "Architect,Developer"
```

## 🚀 Deployment Patterns

### Docker Integration
```dockerfile
# Dockerfile for BRAIN system
FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y sqlite3 ca-certificates
COPY --from=builder /app/target/release/ci /usr/local/bin/
COPY --from=builder /app/target/release/ci-db /usr/local/bin/

ENV CI_DB_PATH=/data/brain.db
ENV CI_REPO_PATH=/data/brain

VOLUME ["/data"]
EXPOSE 8080

CMD ["ci", "serve", "--port", "8080"]
```

### Kubernetes Deployment
```yaml
# kubernetes/brain-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: brain-system
spec:
  replicas: 3
  selector:
    matchLabels:
      app: brain-system
  template:
    metadata:
      labels:
        app: brain-system
    spec:
      containers:
      - name: brain-api
        image: brain-system:latest
        ports:
        - containerPort: 8080
        env:
        - name: CI_DB_PATH
          value: "/data/brain.db"
        volumeMounts:
        - name: brain-data
          mountPath: /data
      volumes:
      - name: brain-data
        persistentVolumeClaim:
          claimName: brain-storage
```

### Cloud Integration
```yaml
# AWS CloudFormation template
AWSTemplateFormatVersion: '2010-09-09'
Resources:
  BrainSystem:
    Type: AWS::ECS::Service
    Properties:
      Cluster: !Ref ECSCluster
      TaskDefinition: !Ref BrainTaskDefinition
      DesiredCount: 2
      
  BrainDatabase:
    Type: AWS::RDS::DBInstance
    Properties:
      DBInstanceClass: db.t3.micro
      Engine: sqlite
      AllocatedStorage: 20
```

## 📈 Monitoring and Observability

### Metrics Integration
```prometheus
# Prometheus metrics
brain_agent_activation_total{agent="Architect"} 150
brain_agent_execution_duration_seconds{agent="Developer"} 2.5
brain_knowledge_submissions_total{category="architecture"} 45
brain_system_health{component="database"} 1
```

### Logging Integration
```json
{
  "timestamp": "2025-08-30T10:30:00Z",
  "level": "INFO",
  "service": "brain-system",
  "component": "agent-manager",
  "message": "Agent Architect activated successfully",
  "metadata": {
    "agent_name": "Architect",
    "session_id": "sess_123456",
    "activation_time_ms": 150
  }
}
```

### Health Check Endpoints
```http
GET /health/live      # Liveness probe
GET /health/ready     # Readiness probe  
GET /health/metrics   # Detailed metrics
GET /health/agents    # Agent health status
```

## 🔐 Security Integration

### Authentication Integration
```yaml
# OAuth 2.0 integration
oauth2:
  providers:
    - name: "google"
      client_id: "${GOOGLE_CLIENT_ID}"
      client_secret: "${GOOGLE_CLIENT_SECRET}"
      redirect_uri: "http://localhost:8080/auth/callback"
      
  jwt:
    secret: "${JWT_SECRET}"
    expiry: 3600
    refresh_expiry: 86400
```

### Access Control
```json
{
  "roles": {
    "admin": ["*"],
    "developer": ["agents:read", "agents:execute", "knowledge:read", "knowledge:write"],
    "readonly": ["agents:read", "knowledge:read", "system:status"]
  },
  "policies": {
    "agent_access": {
      "effect": "allow",
      "principals": ["role:developer"],
      "actions": ["agents:*"],
      "resources": ["agent:*"]
    }
  }
}
```

## 🎯 Integration Best Practices

### Performance Optimization
1. **Use connection pooling** for database access
2. **Implement caching** for frequently accessed data
3. **Batch requests** when possible
4. **Use async operations** for non-blocking integration

### Error Handling
1. **Implement retry logic** with exponential backoff
2. **Use circuit breakers** for external service calls
3. **Log all integration errors** with context
4. **Provide meaningful error messages** to clients

### Security Best Practices
1. **Use HTTPS** for all API communications
2. **Validate all input data** before processing
3. **Implement rate limiting** to prevent abuse
4. **Regular security audits** of integration points

### Monitoring and Alerting
1. **Monitor integration performance** continuously
2. **Set up alerts** for critical failures
3. **Track usage metrics** for capacity planning
4. **Regular health checks** on all integration points

---

*Last Updated: 2025-08-30*  
*Version: 1.0*  
*Next Review: 2025-09-30*