# CLI Coordination System - Complete Guide

## Overview

The CI CLI coordination system provides comprehensive multi-agent session management, real-time monitoring, and task coordination capabilities. This system integrates seamlessly with existing CI workflows and provides both command-line and dashboard interfaces for managing collaborative intelligence sessions.

## Installation and Setup

### Prerequisites
- Rust (for CLI compilation)
- Python 3.7+ (for coordination backend)
- Access to CollaborativeIntelligence repository

### Installation
```bash
# Install CLI globally
cd /path/to/CollaborativeIntelligence/cli
cargo install --path .

# Verify installation
ci --version
```

## Command Reference

### Core Coordination Commands

#### `ci coordination`
Main coordination system for managing multi-agent sessions.

```bash
# Start a new coordination session
ci coordination start [OPTIONS]

# Options:
--session, -s <SESSION_ID>     # Custom session ID (optional)
--lane, -l <LANE>             # Assign session to specific lane
--claude-version, -v <VERSION> # Claude version identifier
--context, -c <CONTEXT>       # Additional user context

# Examples:
ci coordination start --lane development --claude-version "claude-3.5-sonnet"
ci coordination start --session my-project-session --context "Feature development"
```

#### Session Management

```bash
# Show coordination system status
ci coordination status [OPTIONS]

# Options:
--session, -s <SESSION_ID>    # Show status for specific session
--all, -a                     # Show all sessions
--active                      # Show only active sessions
--system                      # Show system status overview
--format, -f <FORMAT>         # Output format (table, json)

# Examples:
ci coordination status --system
ci coordination status --all --format json
ci coordination status --session claude-dev-12345
```

```bash
# Send heartbeat for active session
ci coordination heartbeat --session <SESSION_ID> [OPTIONS]

# Options:
--tasks-completed, -c <COUNT>  # Number of tasks completed
--tasks-reserved, -r <COUNT>   # Number of tasks reserved
--activity, -a <JSON>          # Additional activity data as JSON

# Example:
ci coordination heartbeat --session my-session --tasks-completed 5 --tasks-reserved 2
```

```bash
# End coordination session
ci coordination end --session <SESSION_ID> [OPTIONS]

# Options:
--reason, -r <REASON>         # Termination reason

# Example:
ci coordination end --session my-session --reason "Work completed"
```

#### Task Coordination

```bash
# Reserve a task for a session
ci coordination task reserve --session <SESSION_ID> --task <TASK_ID> [OPTIONS]

# Options:
--priority, -p <PRIORITY>     # Task priority (high, medium, low)

# Example:
ci coordination task reserve --session dev-session --task feature-123 --priority high
```

```bash
# Mark task as completed
ci coordination task complete --session <SESSION_ID> --task <TASK_ID> [OPTIONS]

# Options:
--result, -r <JSON>           # Task completion result data as JSON

# Example:
ci coordination task complete --session dev-session --task feature-123 --result '{"status": "success", "files_changed": 3}'
```

```bash
# Show task coordination status
ci coordination task status [OPTIONS]

# Options:
--lane, -l <LANE>            # Filter by lane
--format, -f <FORMAT>        # Output format (table, json)

# Example:
ci coordination task status --lane development
```

#### Lane Management

```bash
# Assign session to lane
ci coordination lane assign --session <SESSION_ID> --lane <LANE>

# Show lane assignment status
ci coordination lane status [OPTIONS]

# Options:
--format, -f <FORMAT>        # Output format (table, json)

# Examples:
ci coordination lane assign --session dev-session --lane frontend
ci coordination lane status
```

#### Reports and Analytics

```bash
# Generate coordination summary report
ci coordination reports summary [OPTIONS]

# Options:
--output, -o <FILE>          # Output file path
--format, -f <FORMAT>        # Report format (markdown, json, html)
--timeframe, -t <TIMEFRAME>  # Report timeframe (1h, 24h, 7d, 30d)

# Example:
ci coordination reports summary --output coordination-summary.html --format html --timeframe 24h
```

```bash
# Generate activity report
ci coordination reports activity [OPTIONS]

# Options:
--output, -o <FILE>          # Output file path
--format, -f <FORMAT>        # Report format (markdown, json, csv)

# Example:
ci coordination reports activity --format csv --output activity-report.csv
```

#### System Maintenance

```bash
# Clean up stale sessions
ci coordination cleanup [OPTIONS]

# Options:
--dry-run                    # Show what would be cleaned without making changes
--force                      # Force cleanup even if sessions appear active

# Examples:
ci coordination cleanup --dry-run
ci coordination cleanup --force
```

### Dashboard Commands

#### `ci dashboard`
Launch coordination dashboard for real-time monitoring.

```bash
# Launch terminal dashboard (default)
ci dashboard [OPTIONS]

# Options:
--refresh, -r <SECONDS>      # Refresh interval (default: 5)
--view, -v <VIEW>           # Dashboard view (system, sessions, tasks, lanes)
--export, -e <FILE>         # Export dashboard data to file

# Examples:
ci dashboard --view sessions --refresh 10
ci dashboard --export system-status.json
```

```bash
# Launch web dashboard
ci dashboard --web [OPTIONS]

# Options:
--port, -p <PORT>           # Port for web dashboard (default: 8080)

# Example:
ci dashboard --web --port 3000
```

## Configuration

### Environment Variables

```bash
# Optional configuration
export CI_COORDINATION_HEARTBEAT_INTERVAL=300  # 5 minutes
export CI_COORDINATION_SESSION_TIMEOUT=900     # 15 minutes
export CI_COORDINATION_MAX_SESSIONS=10         # Maximum concurrent sessions
```

### Configuration File

The coordination system uses `/BRAIN/SESSION_COORDINATION.json` for persistent state management. This file is automatically created and managed by the system.

## Integration with Existing Workflows

### Basic Session Lifecycle

```bash
# 1. Start coordination session
SESSION_ID=$(ci coordination start --lane development | grep "session:" | cut -d' ' -f2)

# 2. Work on tasks (send periodic heartbeats)
ci coordination heartbeat --session $SESSION_ID --tasks-reserved 3

# 3. Complete work and update status
ci coordination heartbeat --session $SESSION_ID --tasks-completed 3 --tasks-reserved 0

# 4. End session
ci coordination end --session $SESSION_ID --reason "Development complete"
```

### Multi-Agent Workflow

```bash
# Terminal 1: Frontend development
ci coordination start --lane frontend --claude-version "claude-3.5-sonnet"

# Terminal 2: Backend development  
ci coordination start --lane backend --claude-version "claude-3.5-sonnet"

# Terminal 3: Monitor system
ci dashboard --view lanes
```

### CI/CD Integration

```bash
#!/bin/bash
# Example CI/CD integration script

# Start coordination session
SESSION_ID=$(ci coordination start --lane cicd --context "Automated build" | grep -oP 'session: \K\S+')

# Reserve build task
ci coordination task reserve --session $SESSION_ID --task build-$BUILD_NUMBER --priority high

# Run build process
if ./run-build.sh; then
    ci coordination task complete --session $SESSION_ID --task build-$BUILD_NUMBER --result '{"status": "success"}'
else
    ci coordination task complete --session $SESSION_ID --task build-$BUILD_NUMBER --result '{"status": "failed"}'
fi

# End session
ci coordination end --session $SESSION_ID --reason "Build process completed"
```

## Output Formats

### Table Format (Default)
```
Session ID                                        Status     Lane         Duration   Tasks
claude-hostname-1693234567-abc12345              active     development  25m        3/1
claude-hostname-1693234123-def67890              ended      frontend     45m        5/0
```

### JSON Format
```json
{
  "system_health": {
    "status": "healthy",
    "active_sessions": 2,
    "max_concurrent": 10,
    "utilization_percent": 20,
    "timeout_rate": 0.0
  },
  "session_counts": {
    "active": 2,
    "ended": 15,
    "timed_out": 1
  },
  "lane_assignments": {
    "development": 2,
    "testing": 1
  }
}
```

## Common Use Cases

### Development Session Management

```bash
# Start focused development session
ci coordination start --lane feature-auth --context "Implementing user authentication"

# Monitor progress
ci dashboard --view tasks

# Reserve specific tasks
ci coordination task reserve --session $SESSION_ID --task auth-login --priority high
ci coordination task reserve --session $SESSION_ID --task auth-register --priority medium

# Regular heartbeats with progress updates
ci coordination heartbeat --session $SESSION_ID --tasks-completed 1 --tasks-reserved 1
```

### Team Coordination

```bash
# Check team activity
ci coordination status --all

# View lane distribution
ci coordination lane status

# Generate team report
ci coordination reports activity --format markdown --output team-activity.md
```

### System Monitoring

```bash
# Launch real-time dashboard
ci dashboard --view system --refresh 5

# Export system metrics
ci dashboard --export metrics.json

# Check for stale sessions
ci coordination cleanup --dry-run
```

## Troubleshooting

### Common Issues

1. **Session Not Found**
   ```bash
   # Check if session exists
   ci coordination status --all | grep your-session-id
   
   # Register new session if needed
   ci coordination start --session your-session-id
   ```

2. **Coordination Script Not Found**
   ```bash
   # Verify BRAIN directory structure
   ls -la /path/to/CollaborativeIntelligence/BRAIN/coordination/
   
   # Check Python path
   which python3
   ```

3. **Lock File Issues**
   ```bash
   # Remove stale lock file
   rm /path/to/CollaborativeIntelligence/BRAIN/coordination/.session_lock
   ```

4. **Permission Issues**
   ```bash
   # Check file permissions
   ls -la /path/to/CollaborativeIntelligence/BRAIN/SESSION_COORDINATION.json
   
   # Fix permissions if needed
   chmod 664 /path/to/CollaborativeIntelligence/BRAIN/SESSION_COORDINATION.json
   ```

### Debug Mode

Enable verbose logging:
```bash
export RUST_LOG=debug
ci coordination status --system
```

### Health Checks

```bash
# Verify system health
ci coordination status --system --format json | jq '.system_health'

# Check for timeouts
ci coordination cleanup --dry-run

# Validate configuration
ci coordination status --system | grep -E "(active_sessions|max_concurrent)"
```

## Best Practices

1. **Session Management**
   - Use descriptive session IDs for easier identification
   - Always end sessions gracefully when work is complete
   - Send regular heartbeats during active work

2. **Lane Organization**
   - Organize work by feature, component, or team
   - Use consistent lane naming conventions
   - Monitor lane distribution for load balancing

3. **Task Coordination**
   - Reserve tasks before starting work to avoid conflicts
   - Update task progress regularly
   - Include meaningful result data when completing tasks

4. **Monitoring**
   - Use dashboard for real-time monitoring
   - Generate regular reports for team visibility
   - Set up cleanup routines for stale sessions

5. **Integration**
   - Integrate coordination into existing scripts and workflows
   - Use JSON format for programmatic processing
   - Implement proper error handling in automation scripts

---

**Last Updated**: 2025-09-01  
**Version**: 1.0  
**Compatibility**: CI CLI v1.0+