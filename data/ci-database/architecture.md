# Dual Database Architecture

## Overview
The CI system uses two separate SQLite databases for different purposes:

### 1. Internal Database (`ci_internal.db`)
**Purpose**: Track CI system's internal operations
- Agent memory and learning
- Session records
- Internal knowledge base
- Agent collaboration patterns
- System evolution tracking

**Location**: `/data/ci-database/internal/ci_internal.db`

### 2. Projects Database (`ci_projects.db`)
**Purpose**: Manage external projects using CI CLI
- Project registry
- External operation logs
- CI CLI usage tracking
- Integration metadata
- Performance metrics

**Location**: `/data/ci-database/projects/ci_projects.db`

## Benefits

1. **Isolation**: Internal agent operations don't interfere with project management
2. **Security**: External projects can't access internal agent memory
3. **Performance**: Each database optimized for its workload
4. **Scalability**: Can independently scale or migrate databases
5. **Backup**: Different backup strategies for each database

## Access Patterns

### Internal Database
- Only accessible by CI agents
- Higher frequency of reads/writes
- Complex relational queries
- Full transaction support

### Projects Database
- Accessible by CI CLI and agents
- Lower frequency operations
- Simpler query patterns
- Read-heavy workload

## Migration Path
1. Initialize both databases with their schemas
2. Implement connection management for each
3. Create separate APIs for internal vs external operations
4. Establish synchronization points where needed