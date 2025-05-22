# DB CLI Examples

This document provides practical examples of using the CollaborativeIntelligence database CLI.

## Quick Start

### Initial Setup

```bash
# Initialize the database
db admin init

# Scan and populate the database
db populate scan

# Check database health
db admin health
```

## Common Workflows

### Agent Management

```bash
# List all agents
db agent list

# View specific agent details
db agent show Athena

# Create a new agent
db agent create "DataAnalyst" --agent-type "analysis" \
  --memory-path "/AGENTS/DataAnalyst" \
  --status "active"

# Update agent status
db agent update DataAnalyst --status "inactive"

# Delete an agent (careful!)
db agent delete DataAnalyst --force
```

### Learning System

```bash
# Add learning entry
db learning add Athena "pattern_recognition" \
  "Identified new optimization pattern in codebase"

# List recent learning for an agent
db learning list Athena --limit 10

# Search learning entries
db learning search "optimization"

# List learning by type
db learning list --discovery-type "insight"
```

### Session Management

```bash
# Create a new session
SESSION_ID=$(db session create "Optimizer" "code_review" \
  --format json | jq -r '.id')

# Update session status
db session update $SESSION_ID --status "in_progress"

# List active sessions
db session list --status "active"

# Close session with outcomes
db session close $SESSION_ID \
  --outcomes '{"files_reviewed": 15, "issues_found": 3}'
```

### Knowledge Base

```bash
# Add knowledge entry
db knowledge add "git_workflow" "development" \
  "Always create feature branches from main"

# Search knowledge base
db knowledge search "git" --category "development"

# List all categories
db knowledge categories

# Get knowledge by ID
db knowledge get 12345
```

### Project Management

```bash
# Register a new project
db project register "MyApp" "/home/user/projects/myapp" \
  --project-type "web" \
  --language "typescript"

# Update project status
db project update "MyApp" --status "active"

# List all projects
db project list

# List by status
db project list --status "active"

# List by type
db project list --project-type "web"
```

### Metrics and Analytics

```bash
# Add metrics
db metrics add "MyApp" "build_time" 125.5
db metrics add "MyApp" "test_coverage" 85.2
db metrics add "MyApp" "bundle_size" 2.1

# View recent metrics
db metrics list "MyApp" "build_time" --limit 10

# View metrics dashboard
db metrics dashboard

# View dashboard for specific time period
db metrics dashboard --days 7

# Export metrics as JSON
db metrics list "MyApp" "all" --format json > metrics.json
```

### Database Population

```bash
# Basic scan
db populate scan

# Scan specific directory
db populate scan --path /path/to/project

# Dry run (preview changes)
db populate scan --dry-run

# Verbose scan
db -v populate scan

# Very verbose (debug)
db -vv populate scan

# Quiet mode
db -q populate scan
```

### Direct SQL Queries

```bash
# Query internal database
db query "SELECT * FROM agents WHERE status = 'active'"

# Query projects database
db query "SELECT * FROM projects ORDER BY last_accessed DESC" \
  --database projects

# Complex query with joins
db query "
  SELECT a.agent_id, COUNT(l.id) as learning_count 
  FROM agents a 
  LEFT JOIN learning_entries l ON a.agent_id = l.agent_id 
  GROUP BY a.agent_id 
  ORDER BY learning_count DESC
" --format json

# Export query results
db query "SELECT * FROM knowledge_base" --format csv > knowledge.csv
```

### Backup and Maintenance

```bash
# Create backup
db admin backup -o /backups/ci-db-$(date +%Y%m%d-%H%M%S).db

# Export to JSON
db admin export -o ci-export-$(date +%Y%m%d).json

# Import from JSON
db admin import -i ci-export.json

# Database statistics
db admin stats

# Optimize database
db admin optimize

# Health check
db admin health
```

## Advanced Examples

### Batch Operations

```bash
# Create multiple agents from a file
cat agents.txt | while read name type; do
  db agent create "$name" --agent-type "$type"
done

# Add multiple metrics
for i in {1..10}; do
  db metrics add "MyApp" "response_time" $(echo "scale=2; $RANDOM/1000" | bc)
  sleep 1
done
```

### JSON Processing

```bash
# Export agents and process with jq
db agent list --format json | jq '.[] | select(.status == "active")'

# Get agent IDs
db agent list --format json | jq -r '.[].agent_id'

# Complex JSON query
db session list --format json | jq '
  group_by(.agent_id) | 
  map({
    agent: .[0].agent_id,
    session_count: length,
    last_session: (sort_by(.created_at) | last)
  })
'
```

### Shell Integration

```bash
# Function to quickly add learning
add_learning() {
  local agent=$1
  local type=$2
  local content=$3
  db learning add "$agent" "$type" "$content"
}

# Usage
add_learning "Optimizer" "insight" "Found performance bottleneck"

# Alias for common operations
alias dbl='db agent list'
alias dbs='db session list --status active'
alias dbq='db query'
```

### Error Handling

```bash
# Check if command succeeded
if db populate scan; then
  echo "Scan completed successfully"
else
  echo "Scan failed with exit code $?"
fi

# Capture output and errors
output=$(db agent show NonExistent 2>&1)
if [ $? -ne 0 ]; then
  echo "Error: $output"
fi
```

## Tips and Tricks

1. **Use JSON output for scripting**
   ```bash
   db agent list --format json | jq -r '.[].agent_id' | xargs -I {} db agent show {}
   ```

2. **Set default format**
   ```bash
   export CI_DB_FORMAT=json
   ```

3. **Use verbose mode for debugging**
   ```bash
   db -vv populate scan
   ```

4. **Combine with other tools**
   ```bash
   db metrics list "MyApp" "build_time" --format csv | gnuplot -p plot.gnu
   ```

5. **Regular backups**
   ```bash
   crontab -e
   # Add: 0 3 * * * db admin backup -o /backups/ci-db-$(date +\%Y\%m\%d).db
   ```

## Common Issues

### "Command not found: scan"
The correct command is `db populate scan`, not `db scan`.

### Database Locked
```bash
# Find processes using the database
lsof | grep ci-internal.db

# Kill stuck processes if needed
kill -9 <PID>

# Optimize database
db admin optimize
```

### No Results Found
```bash
# Check database is populated
db admin stats

# Re-scan if needed
db populate scan

# Verify with direct query
db query "SELECT COUNT(*) FROM agents"
```