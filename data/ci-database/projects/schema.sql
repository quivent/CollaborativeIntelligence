-- CI Projects Database Schema
-- For tracking external projects using CI CLI

-- Project registry
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    path TEXT NOT NULL,
    type TEXT DEFAULT 'general',
    status TEXT CHECK(status IN ('active', 'inactive', 'archived')) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_accessed TIMESTAMP,
    ci_version TEXT,
    metadata JSON,
    config JSON
);

-- CI CLI operations log
CREATE TABLE operations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    command TEXT NOT NULL,
    arguments JSON,
    agent_used TEXT,
    status TEXT CHECK(status IN ('started', 'completed', 'failed', 'cancelled')) DEFAULT 'started',
    duration_ms INTEGER,
    error_message TEXT,
    metadata JSON,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Project integration points
CREATE TABLE integrations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    integration_type TEXT NOT NULL,
    config JSON,
    status TEXT DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_sync TIMESTAMP,
    metadata JSON,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Performance metrics
CREATE TABLE metrics (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    metric_type TEXT NOT NULL,
    value REAL NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    context JSON,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- CI feature usage tracking
CREATE TABLE feature_usage (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    feature_name TEXT NOT NULL,
    first_used TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_used TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    usage_count INTEGER DEFAULT 1,
    preferences JSON,
    FOREIGN KEY (project_id) REFERENCES projects(id)
);

-- Project-specific agent configurations
CREATE TABLE project_agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    agent_name TEXT NOT NULL,
    enabled BOOLEAN DEFAULT TRUE,
    custom_config JSON,
    last_used TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects(id),
    UNIQUE(project_id, agent_name)
);

-- Indexes for performance
CREATE INDEX idx_operations_project_timestamp ON operations(project_id, timestamp DESC);
CREATE INDEX idx_operations_status ON operations(status);
CREATE INDEX idx_metrics_project_type ON metrics(project_id, metric_type);
CREATE INDEX idx_feature_project ON feature_usage(project_id, feature_name);
CREATE INDEX idx_project_agents ON project_agents(project_id, agent_name);

-- Triggers for automatic timestamps
CREATE TRIGGER update_project_timestamp
AFTER UPDATE ON projects
BEGIN
    UPDATE projects SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_project_access
AFTER INSERT ON operations
BEGIN
    UPDATE projects SET last_accessed = CURRENT_TIMESTAMP WHERE id = NEW.project_id;
END;

CREATE TRIGGER track_feature_usage
AFTER INSERT ON operations
WHEN NEW.agent_used IS NOT NULL
BEGIN
    INSERT OR REPLACE INTO feature_usage (project_id, feature_name, last_used, usage_count)
    VALUES (
        NEW.project_id,
        NEW.agent_used,
        CURRENT_TIMESTAMP,
        COALESCE((SELECT usage_count FROM feature_usage WHERE project_id = NEW.project_id AND feature_name = NEW.agent_used), 0) + 1
    );
END;