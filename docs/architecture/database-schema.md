# CI System Database Schema

## Overview
This document defines the database schema for the Collaborative Intelligence system's centralized data management. The database uses SQLite for lightweight, embedded storage.

## Core Tables

### projects
Tracks all projects managed by the CI system.

```sql
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    path TEXT NOT NULL,
    status TEXT CHECK(status IN ('active', 'inactive', 'archived')) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    metadata JSON
);
```

### agents
Maintains registry of all CI agents.

```sql
CREATE TABLE agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL,
    status TEXT CHECK(status IN ('active', 'inactive', 'deprecated')) DEFAULT 'active',
    last_active TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    capabilities JSON,
    memory_path TEXT
);
```

### operations
Logs all significant operations performed by agents.

```sql
CREATE TABLE operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    agent_id INTEGER NOT NULL,
    project_id INTEGER,
    operation_type TEXT NOT NULL,
    description TEXT,
    status TEXT CHECK(status IN ('started', 'completed', 'failed')) DEFAULT 'started',
    metadata JSON,
    FOREIGN KEY (agent_id) REFERENCES agents(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);
```

### sessions
Tracks agent work sessions.

```sql
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    project_id INTEGER,
    session_name TEXT NOT NULL,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP,
    status TEXT CHECK(status IN ('active', 'completed', 'terminated')) DEFAULT 'active',
    summary TEXT,
    metadata JSON,
    FOREIGN KEY (agent_id) REFERENCES agents(id),
    FOREIGN KEY (project_id) REFERENCES projects(id)
);
```

### metrics
Stores various performance and activity metrics.

```sql
CREATE TABLE metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER,
    agent_id INTEGER,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    unit TEXT,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    metadata JSON,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);
```

### knowledge_items
Tracks learned knowledge and insights.

```sql
CREATE TABLE knowledge_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    category TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    confidence REAL CHECK(confidence >= 0 AND confidence <= 1),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    references JSON,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);
```

### integrations
Manages cross-system integrations.

```sql
CREATE TABLE integrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL,
    config JSON NOT NULL,
    status TEXT CHECK(status IN ('active', 'inactive', 'error')) DEFAULT 'active',
    last_sync TIMESTAMP,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
```

## Indexes

```sql
CREATE INDEX idx_operations_timestamp ON operations(timestamp);
CREATE INDEX idx_operations_agent ON operations(agent_id);
CREATE INDEX idx_metrics_timestamp ON metrics(timestamp);
CREATE INDEX idx_sessions_active ON sessions(status) WHERE status = 'active';
CREATE INDEX idx_knowledge_agent ON knowledge_items(agent_id);
```

## Views

```sql
-- Active agent summary
CREATE VIEW active_agents AS
SELECT a.name, a.type, a.last_active, 
       COUNT(DISTINCT s.id) as active_sessions,
       COUNT(DISTINCT o.id) as recent_operations
FROM agents a
LEFT JOIN sessions s ON a.id = s.agent_id AND s.status = 'active'
LEFT JOIN operations o ON a.id = o.agent_id 
    AND o.timestamp > datetime('now', '-1 day')
WHERE a.status = 'active'
GROUP BY a.id;

-- Project activity
CREATE VIEW project_activity AS
SELECT p.name as project_name,
       COUNT(DISTINCT o.agent_id) as active_agents,
       COUNT(o.id) as operations_count,
       MAX(o.timestamp) as last_activity
FROM projects p
LEFT JOIN operations o ON p.id = o.project_id
WHERE p.status = 'active'
GROUP BY p.id
ORDER BY last_activity DESC;
```

## Schema Versioning

Schema versions are tracked in the `schema_version` table:

```sql
CREATE TABLE schema_version (
    version INTEGER PRIMARY KEY,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    description TEXT
);
```

## Migration Strategy

1. All schema changes must be versioned
2. Migrations are stored in `/data/schemas/`
3. Each migration file is named `vX.sql` where X is the version number
4. Migrations must be idempotent and include rollback procedures

## Data Retention

- Operations older than 90 days may be archived
- Metrics older than 30 days may be aggregated
- Knowledge items are retained indefinitely
- Session data is retained for 180 days

## Backup Strategy

- Daily backups of the entire database
- Transaction log for point-in-time recovery
- Regular exports to JSON for transparency