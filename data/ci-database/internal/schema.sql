-- CI Internal Database Schema
-- For tracking internal CI system operations

-- Agent registry with extended metadata
CREATE TABLE agents (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE,
    type TEXT NOT NULL,
    status TEXT CHECK(status IN ('active', 'inactive', 'learning', 'error')) DEFAULT 'active',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    last_active TIMESTAMP,
    activation_count INTEGER DEFAULT 0,
    memory_version INTEGER DEFAULT 1,
    capabilities JSON,
    memory_path TEXT,
    config JSON
);

-- Agent learning records
CREATE TABLE learning_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    timestamp TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    trigger_type TEXT NOT NULL,
    content TEXT NOT NULL,
    importance REAL DEFAULT 0.5,
    category TEXT,
    metadata JSON,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

-- Session records for agent activities
CREATE TABLE sessions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    session_type TEXT NOT NULL,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP,
    status TEXT CHECK(status IN ('active', 'completed', 'failed', 'timeout')) DEFAULT 'active',
    metadata JSON,
    outcomes JSON,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

-- Inter-agent collaboration tracking
CREATE TABLE collaborations (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    initiator_agent_id INTEGER NOT NULL,
    participant_agent_id INTEGER NOT NULL,
    started_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    ended_at TIMESTAMP,
    collaboration_type TEXT NOT NULL,
    status TEXT,
    outcomes JSON,
    FOREIGN KEY (initiator_agent_id) REFERENCES agents(id),
    FOREIGN KEY (participant_agent_id) REFERENCES agents(id)
);

-- System knowledge base
CREATE TABLE knowledge_entries (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    knowledge_type TEXT NOT NULL,
    category TEXT,
    content TEXT NOT NULL,
    confidence REAL DEFAULT 0.8,
    usage_count INTEGER DEFAULT 0,
    last_accessed TIMESTAMP,
    metadata JSON,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

-- Agent memory snapshots
CREATE TABLE memory_snapshots (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    agent_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    version INTEGER NOT NULL,
    memory_type TEXT NOT NULL,
    content TEXT NOT NULL,
    hash TEXT UNIQUE,
    FOREIGN KEY (agent_id) REFERENCES agents(id)
);

-- Indexes for performance
CREATE INDEX idx_learning_agent_timestamp ON learning_entries(agent_id, timestamp DESC);
CREATE INDEX idx_sessions_agent_status ON sessions(agent_id, status);
CREATE INDEX idx_knowledge_agent_type ON knowledge_entries(agent_id, knowledge_type);
CREATE INDEX idx_collaborations_agents ON collaborations(initiator_agent_id, participant_agent_id);
CREATE INDEX idx_memory_agent_version ON memory_snapshots(agent_id, version DESC);

-- Triggers for automatic timestamps
CREATE TRIGGER update_knowledge_timestamp
AFTER UPDATE ON knowledge_entries
BEGIN
    UPDATE knowledge_entries SET updated_at = CURRENT_TIMESTAMP WHERE id = NEW.id;
END;

CREATE TRIGGER update_agent_activity
AFTER INSERT ON sessions
BEGIN
    UPDATE agents SET last_active = CURRENT_TIMESTAMP, activation_count = activation_count + 1 
    WHERE id = NEW.agent_id;
END;