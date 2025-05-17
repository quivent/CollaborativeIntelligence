# DB CLI - CollaborativeIntelligence Database Management

A standalone command-line interface for managing the CollaborativeIntelligence database system.

## Features

- Complete agent management
- Learning system operations
- Session tracking
- Knowledge base queries
- Project management
- Metrics and analytics
- Database administration
- Direct SQL queries
- Multiple output formats (table, json, yaml, csv)

## Installation

```bash
cd data/db-cli
cargo build --release
sudo ln -s $(pwd)/target/release/db /usr/local/bin/db
```

## Usage

### Basic Commands

```bash
# Scan directory and populate database
db populate scan

# Scan with verbose output
db -v populate scan

# Preview scan without making changes
db populate scan --dry-run

# Scan specific directory
db populate scan --path /path/to/directory

# List all agents
db agent list

# Show agent details
db agent show Athena

# Create new agent
db agent create "NewAgent" --agent-type "specialist"

# Add learning entry
db learning add Athena "discovery" "Learned new pattern"

# Search knowledge base
db knowledge search "protocol"

# List projects
db project list --status active
```

### Output Formats

```bash
# Default table format
db agent list

# JSON output
db agent list --format json

# YAML output
db agent list --format yaml

# Pipe to files
db agent list --format json > agents.json
```

### Database Population

```bash
# Scan and populate from current directory parent
db populate scan

# Scan specific directory
db populate scan --path /path/to/ci-project

# Preview what would be populated (dry run)
db populate scan --dry-run

# Verbose output showing progress
db -v populate scan

# Very verbose for debugging
db -vv populate scan

# Quiet mode (errors only)
db -q populate scan
```

### Database Administration

```bash
# Initialize database
db admin init

# Health check
db admin health

# Create backup
db admin backup -o /backups/ci-db-backup

# Database statistics
db admin stats

# Export database
db admin export -o export.json --include-sensitive

# Optimize database
db admin optimize
```

### Direct SQL Queries

```bash
# Query internal database
db query "SELECT * FROM agents WHERE status = 'active'"

# Query projects database
db query "SELECT * FROM projects ORDER BY last_accessed DESC" --database projects

# Complex queries with JSON output
db query "SELECT agent_id, COUNT(*) as count FROM learning_entries GROUP BY agent_id" --format json
```

### Metrics Dashboard

```bash
# View analytics dashboard (last 30 days)
db metrics dashboard

# View specific time period
db metrics dashboard --days 7

# List metrics for a project
db metrics list "MyProject" "build_time"

# Add metric
db metrics add "MyProject" "test_coverage" 85.5
```

## Environment Variables

- `CI_DB_PATH`: Database directory path (default: `~/.ci-database`)
- `RUST_LOG`: Logging level (error, warn, info, debug, trace)

## Configuration

The database location can be configured via:
1. Command-line flag: `--data-dir /path/to/db`
2. Environment variable: `CI_DB_PATH=/path/to/db`
3. Default: `~/.ci-database`

## Architecture

This CLI is a standalone tool that directly interfaces with the CI database. It provides:

- Pure database operations without CI system dependencies
- Thread-safe connection pooling
- Optimized query performance
- Transaction support
- Comprehensive error handling

## Examples

### Agent Workflow

```bash
# Create an agent
db agent create "Analyzer" --agent-type "analysis" --memory-path "/agents/Analyzer"

# Record learning
db learning add "Analyzer" "pattern_recognition" "Identified new code pattern"

# Create session
SESSION_ID=$(db session create "Analyzer" "analysis_task" --format json | jq -r '.id')

# Close session
db session close $SESSION_ID --outcomes '{"status": "success"}'
```

### Project Workflow

```bash
# Register project
db project register "MyApp" "/home/user/projects/myapp" --project-type "web"

# Add metrics
db metrics add "MyApp" "build_time" 125.5
db metrics add "MyApp" "test_count" 250

# View project metrics
db metrics list "MyApp" "build_time" --limit 10
```

### Backup and Restore

```bash
# Create backup
db admin backup -o /backups/ci-$(date +%Y%m%d).db

# Export to JSON
db admin export -o ci-export.json

# Import from JSON
db admin import -i ci-export.json
```

## Development

### Building from Source

```bash
git clone <repository>
cd data/db-cli
cargo build --release
```

### Running Tests

```bash
cargo test
```

### Debug Mode

```bash
RUST_LOG=debug db agent list -vvv
```

## Troubleshooting

### Database Locked

If you get "database locked" errors:
```bash
# Check for other processes
lsof | grep ci-internal.db

# Optimize database
db admin optimize
```

### Performance Issues

```bash
# Run health check
db admin health

# View statistics
db admin stats

# Optimize database
db admin optimize
```

### Populate/Scan Issues

If `db scan` doesn't work:
```bash
# The correct command is:
db populate scan

# NOT:
db scan  # This is incorrect

# To see all populate options:
db populate --help
```

Common scan problems:
```bash
# If scan finds no agents/projects:
# 1. Check you're in the right directory
db populate scan --path /path/to/ci-directory

# 2. Use verbose mode to see what's being scanned
db -v populate scan

# 3. Ensure database is initialized
db admin init
```

## License

Part of the CollaborativeIntelligence project