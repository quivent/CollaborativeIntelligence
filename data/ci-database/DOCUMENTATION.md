# CI Database Documentation

## Overview

The CI Database is the central data persistence layer for the CollaborativeIntelligence system. It manages all internal operations, agent activities, learning systems, and tracks external projects using the CI CLI.

## Database Architecture

The system uses a dual-database architecture:

1. **Internal Database** (`internal/ci-internal.db`)
   - Manages CollaborativeIntelligence system operations
   - Tracks agents, learning, sessions, and knowledge

2. **Projects Database** (`projects/ci-projects.db`)
   - Tracks external projects using CI CLI
   - Records operations, metrics, and feature usage

## Schema Documentation

### Internal Database Tables

#### 1. `agents`
Stores all registered agents in the CollaborativeIntelligence system.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| name | TEXT | Unique agent name |
| type | TEXT | Agent type (e.g., 'system-architect', 'memory-specialist') |
| status | TEXT | Current status: 'active', 'inactive', 'learning', 'error' |
| created_at | TIMESTAMP | Creation timestamp |
| last_active | TIMESTAMP | Last activity timestamp |
| activation_count | INTEGER | Number of times activated |
| memory_version | INTEGER | Current memory version |
| capabilities | JSON | Agent capabilities configuration |
| memory_path | TEXT | Path to agent's memory files |
| config | JSON | Additional configuration |

#### 2. `learning_entries`
Records all learning events for agents.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| agent_id | INTEGER | Foreign key to agents |
| timestamp | TIMESTAMP | When learning occurred |
| trigger_type | TEXT | Type of trigger (e.g., 'protocol_update', 'error_correction') |
| content | TEXT | Learning content |
| importance | REAL | Importance score (0.0-1.0) |
| category | TEXT | Learning category |
| metadata | JSON | Additional metadata |

#### 3. `sessions`
Tracks agent work sessions.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| agent_id | INTEGER | Foreign key to agents |
| session_type | TEXT | Type of session |
| started_at | TIMESTAMP | Session start time |
| ended_at | TIMESTAMP | Session end time |
| status | TEXT | 'active', 'completed', 'failed', 'timeout' |
| metadata | JSON | Session metadata |
| outcomes | JSON | Session outcomes |

#### 4. `knowledge_entries`
Central knowledge base for the system.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| agent_id | INTEGER | Foreign key to agents |
| created_at | TIMESTAMP | Creation time |
| updated_at | TIMESTAMP | Last update time |
| knowledge_type | TEXT | Type of knowledge |
| category | TEXT | Knowledge category |
| content | TEXT | Knowledge content |
| confidence | REAL | Confidence score (0.0-1.0) |
| usage_count | INTEGER | Times accessed |
| last_accessed | TIMESTAMP | Last access time |
| metadata | JSON | Additional metadata |

#### 5. `collaborations`
Inter-agent collaboration tracking.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| initiator_agent_id | INTEGER | Foreign key to initiating agent |
| participant_agent_id | INTEGER | Foreign key to participating agent |
| started_at | TIMESTAMP | Collaboration start time |
| ended_at | TIMESTAMP | Collaboration end time |
| collaboration_type | TEXT | Type of collaboration |
| status | TEXT | Collaboration status |
| outcomes | JSON | Collaboration outcomes |

#### 6. `memory_snapshots`
Agent memory version control.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| agent_id | INTEGER | Foreign key to agents |
| created_at | TIMESTAMP | Snapshot creation time |
| version | INTEGER | Memory version number |
| memory_type | TEXT | Type of memory (long-term, short-term) |
| content | TEXT | Memory content |
| hash | TEXT | Content hash for deduplication |

### Projects Database Tables

#### 1. `projects`
External projects using CI CLI.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| name | TEXT | Unique project name |
| path | TEXT | Project file path |
| type | TEXT | Project type |
| status | TEXT | 'active', 'inactive', 'archived' |
| created_at | TIMESTAMP | Creation time |
| updated_at | TIMESTAMP | Last update time |
| last_accessed | TIMESTAMP | Last access time |
| ci_version | TEXT | CI version used |
| metadata | JSON | Project metadata |
| config | JSON | Project configuration |

#### 2. `operations`
CI CLI operations log.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| project_id | INTEGER | Foreign key to projects |
| timestamp | TIMESTAMP | Operation time |
| command | TEXT | CI command executed |
| arguments | JSON | Command arguments |
| agent_used | TEXT | Agent that handled operation |
| status | TEXT | 'started', 'completed', 'failed', 'cancelled' |
| duration_ms | INTEGER | Operation duration |
| error_message | TEXT | Error message if failed |
| metadata | JSON | Additional metadata |

#### 3. `integrations`
Project integration points.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| project_id | INTEGER | Foreign key to projects |
| integration_type | TEXT | Type of integration |
| config | JSON | Integration configuration |
| status | TEXT | Integration status |
| created_at | TIMESTAMP | Creation time |
| last_sync | TIMESTAMP | Last synchronization |
| metadata | JSON | Additional metadata |

#### 4. `metrics`
Performance and usage metrics.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| project_id | INTEGER | Foreign key to projects |
| metric_type | TEXT | Type of metric |
| value | REAL | Metric value |
| timestamp | TIMESTAMP | Measurement time |
| context | JSON | Metric context |

#### 5. `feature_usage`
CI feature usage tracking.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| project_id | INTEGER | Foreign key to projects |
| feature_name | TEXT | Feature/agent name |
| first_used | TIMESTAMP | First usage time |
| last_used | TIMESTAMP | Last usage time |
| usage_count | INTEGER | Total usage count |
| preferences | JSON | User preferences |

#### 6. `project_agents`
Project-specific agent configurations.

| Column | Type | Description |
|--------|------|-------------|
| id | INTEGER | Primary key |
| project_id | INTEGER | Foreign key to projects |
| agent_name | TEXT | Agent name |
| enabled | BOOLEAN | Whether agent is enabled |
| custom_config | JSON | Custom configuration |
| last_used | TIMESTAMP | Last usage time |

## CLI Commands

### Agent Management
- `ci-db agent register <name> --agent-type <type>` - Register new agent
- `ci-db agent get <name>` - Get agent details
- `ci-db agent list` - List all agents
- `ci-db agent activate <name>` - Activate agent
- `ci-db agent deactivate <name>` - Deactivate agent

### Learning System
- `ci-db learning record <agent> <trigger> <content>` - Record learning
- `ci-db learning get <agent>` - Get agent learnings

### Session Management
- `ci-db session create <agent> <type>` - Create session
- `ci-db session close <id>` - Close session

### Knowledge Base
- `ci-db knowledge add <agent> <type> <content>` - Add knowledge
- `ci-db knowledge search <query>` - Search knowledge

### Project Management
- `ci-db project register <name> <path>` - Register project
- `ci-db project get <name>` - Get project details
- `ci-db project list` - List projects

### Operations
- `ci-db operation log <project> <command>` - Log operation
- `ci-db operation complete <id> --status <status>` - Complete operation

### Metrics
- `ci-db metric record <project> <type> <value>` - Record metric
- `ci-db metric get <project> <type>` - Get metrics

### Database Management
- `ci-db db init` - Initialize databases
- `ci-db db migrate` - Run migrations
- `ci-db db backup --output <path>` - Backup databases

## Integration Examples

### From Rust Code
```rust
use ci_database::{Database, CIDatabaseAPI};
use ci_database::internal::CreateAgentRequest;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize database
    let db = Database::new("/path/to/data").await?;
    let api = CIDatabaseAPI::new(db);
    
    // Register an agent
    let req = CreateAgentRequest {
        name: "Athena".to_string(),
        agent_type: "system-architect".to_string(),
        capabilities: None,
        memory_path: Some("/agents/Athena".to_string()),
        config: None,
    };
    
    let agent = api.create_agent(req).await?;
    println!("Agent created: {:?}", agent);
    
    Ok(())
}
```

### From Shell Scripts
```bash
#!/bin/bash

# Register a new agent
ci-db agent register "Athena" --agent-type "system-architect"

# Record a learning event
ci-db learning record "Athena" "protocol_update" "Updated protocol handling"

# Create a session
SESSION_ID=$(ci-db session create "Athena" "code-review" | grep "ID:" | cut -d' ' -f2)

# ... do work ...

# Close the session
ci-db session close $SESSION_ID
```

## Best Practices

1. **Agent Naming**: Use consistent snake_case for agent names
2. **Learning Categories**: Use predefined categories for better organization
3. **Session Management**: Always close sessions to maintain accurate metrics
4. **Error Handling**: Check return codes in scripts
5. **Backup**: Regular backups using `ci-db db backup`

## Troubleshooting

### Common Issues

1. **Database Locked**
   - Multiple processes accessing the database
   - Solution: Ensure proper connection pooling

2. **Migration Failures**
   - Schema conflicts
   - Solution: Backup, then reinitialize

3. **Performance Issues**
   - Missing indexes
   - Solution: Run `ANALYZE` command

### Debug Mode
Enable verbose logging:
```bash
ci-db -vvv <command>
```

## Future Enhancements

1. **Full-text search** across all content
2. **Graph visualization** for collaborations
3. **Real-time metrics** dashboard
4. **Automated backups**
5. **Replication support**