# BRAIN Coordination System
**Multi-Agent Development Coordination Tools**

This directory contains the coordination infrastructure for managing multi-Claude development sessions on the BRAIN CollaborativeIntelligence project.

## 📁 System Components

### Core Scripts

- **`task_checkin_checkout.py`** - Task reservation and completion management
- **`lane_manager.py`** - Development lane coordination and progress tracking
- **`session_coordinator.py`** - Multi-Claude session management and heartbeats
- **`log_aggregator.py`** - Activity logging and report generation
- **`quick_start.sh`** - One-command setup and session initiation

### Data Files (Auto-generated)

- **`../TASK_REGISTRY.json`** - Central task database with status tracking
- **`../LANE_MANAGEMENT.json`** - Lane assignments and progress data
- **`../SESSION_COORDINATION.json`** - Active session tracking
- **`../DEVELOPMENT_LOG.jsonl`** - Comprehensive activity logs

## 🚀 Quick Start

### 1. Initialize System
```bash
# One-time setup
./quick_start.sh init
```

### 2. Start Development Session
```bash
# Start session on Lane1 with specific agents
./quick_start.sh start Lane1 "Manager,Architect,Developer"

# This will:
# - Register your Claude session
# - Assign you to Lane1
# - Show available tasks
# - Export session ID to environment
```

### 3. Work with Tasks
```bash
# Reserve a task
./quick_start.sh reserve runtime-engine-core Manager

# Update progress (use the individual script for more control)
python3 task_checkin_checkout.py progress --task runtime-engine-core --session $BRAIN_SESSION_ID --percent 50 --notes "Core architecture implemented"

# Complete task
python3 task_checkin_checkout.py complete --task runtime-engine-core --session $BRAIN_SESSION_ID --deliverables "Runtime engine with agent lifecycle management" --notes "All tests passing"
```

### 4. Monitor and Report
```bash
# Check system status
./quick_start.sh status

# Generate daily report
./quick_start.sh reports

# Send session heartbeat
./quick_start.sh heartbeat
```

## 🔧 Individual Tool Usage

### Task Management (`task_checkin_checkout.py`)

#### List Available Tasks
```bash
# All available tasks
python3 task_checkin_checkout.py available

# Filter by lane
python3 task_checkin_checkout.py available --lane Lane1

# Filter by agent assignment
python3 task_checkin_checkout.py available --agent Manager
```

#### Reserve Task
```bash
python3 task_checkin_checkout.py reserve \
  --task runtime-engine-core \
  --session claude-hostname-1234567890 \
  --agent Manager \
  --timeout 8  # Optional: timeout in hours (default: 4)
```

#### Update Progress
```bash
python3 task_checkin_checkout.py progress \
  --task runtime-engine-core \
  --session claude-hostname-1234567890 \
  --percent 75 \
  --notes "Agent activation system complete, working on task scheduling"
```

#### Complete Task
```bash
python3 task_checkin_checkout.py complete \
  --task runtime-engine-core \
  --session claude-hostname-1234567890 \
  --deliverables "Fully functional runtime engine with tests" \
  --notes "Exceeded performance targets - <1.5s activation time"
```

#### Task Status and Management
```bash
# Get task status
python3 task_checkin_checkout.py status --task runtime-engine-core

# Get all tasks for a session
python3 task_checkin_checkout.py status --session claude-hostname-1234567890

# System overview
python3 task_checkin_checkout.py status --system

# Release a task if blocked
python3 task_checkin_checkout.py release \
  --task runtime-engine-core \
  --session claude-hostname-1234567890 \
  --reason "Waiting for architecture decision"
```

### Lane Management (`lane_manager.py`)

#### Lane Status
```bash
# All lanes overview
python3 lane_manager.py status --all

# Specific lane details
python3 lane_manager.py status --lane Lane1

# Lanes for specific session
python3 lane_manager.py status --session claude-hostname-1234567890
```

#### Lane Assignments
```bash
# Assign agent to lane
python3 lane_manager.py assign --agent Manager --lane Lane1 --role lead
python3 lane_manager.py assign --agent Developer --lane Lane1 --role contributor

# Assign session to lane
python3 lane_manager.py assign-session --session claude-hostname-1234567890 --lane Lane1

# Remove session from lane
python3 lane_manager.py remove-session --session claude-hostname-1234567890 --lane Lane1
```

#### Blocker Management
```bash
# Add blocker
python3 lane_manager.py add-blocker \
  --lane Lane1 \
  --description "Waiting for database schema approval" \
  --severity high

# Resolve blocker
python3 lane_manager.py resolve-blocker \
  --lane Lane1 \
  --blocker blocker_1693234567 \
  --resolution "Schema approved, proceeding with implementation"
```

#### Dependency Checking
```bash
# Check if lane dependencies are met
python3 lane_manager.py check-deps --lane Lane2
```

#### Generate Reports
```bash
# Lane report in text format
python3 lane_manager.py report --lane Lane1 --format text

# JSON format for integration
python3 lane_manager.py report --lane Lane1 --format json
```

### Session Coordination (`session_coordinator.py`)

#### Session Registration
```bash
# Register new session
python3 session_coordinator.py register \
  --lane Lane1 \
  --claude-version "Claude Code 1.5"

# Register with specific session ID
python3 session_coordinator.py register \
  --session custom-session-id \
  --lane Lane2
```

#### Heartbeat Management
```bash
# Send basic heartbeat
python3 session_coordinator.py heartbeat --session claude-hostname-1234567890

# Heartbeat with activity data
python3 session_coordinator.py heartbeat \
  --session claude-hostname-1234567890 \
  --tasks-reserved 2 \
  --tasks-completed 1
```

#### Session Status
```bash
# Specific session status
python3 session_coordinator.py status --session claude-hostname-1234567890

# All active sessions
python3 session_coordinator.py status --active

# All sessions (including ended)
python3 session_coordinator.py status --all

# System health overview
python3 session_coordinator.py status --system
```

#### Session Management
```bash
# End session gracefully
python3 session_coordinator.py end \
  --session claude-hostname-1234567890 \
  --reason "Development work completed"

# Update session activity
python3 session_coordinator.py update-activity \
  --session claude-hostname-1234567890 \
  --tasks-completed 3

# Assign session to different lane
python3 session_coordinator.py assign-lane \
  --session claude-hostname-1234567890 \
  --lane Lane2
```

### Log Analysis (`log_aggregator.py`)

#### Activity Analysis
```bash
# Activity summary for last 24 hours
python3 log_aggregator.py activity --hours 24

# Activity for specific time period
python3 log_aggregator.py activity --hours 168  # Last week
```

#### Error Analysis
```bash
# Error analysis
python3 log_aggregator.py errors --hours 24 --format text

# Detailed error JSON
python3 log_aggregator.py errors --hours 48 --format json
```

#### Performance Metrics
```bash
# Performance analysis
python3 log_aggregator.py performance --hours 24

# Extended performance review
python3 log_aggregator.py performance --hours 168
```

#### Agent Analysis
```bash
# Agent activity analysis
python3 log_aggregator.py agents --hours 24

# Agent performance over time
python3 log_aggregator.py agents --hours 168 --format json
```

#### Reports
```bash
# Generate daily report
python3 log_aggregator.py daily-report

# Generate and save report
python3 log_aggregator.py daily-report --save

# Historical report
python3 log_aggregator.py daily-report --date 2025-08-29 --save
```

#### Log Search
```bash
# Search for specific patterns
python3 log_aggregator.py search "error" --hours 24

# Search with result limit
python3 log_aggregator.py search "Manager" --hours 48 --max-results 20
```

## 🔄 Multi-Session Workflows

### Scenario 1: Parallel Development
```bash
# Session A: Focus on Lane1 infrastructure
./quick_start.sh start Lane1 "Manager,Architect,Developer"
./quick_start.sh reserve runtime-engine-core Manager

# Session B: Focus on Lane2 intelligence (if dependencies met)
# In different terminal/Claude instance:
./quick_start.sh start Lane2 "Analyst,Planner,Developer"  
./quick_start.sh reserve trinity-curation-system Analyst

# Session C: Quality assurance across lanes
# In third terminal/Claude instance:
./quick_start.sh start Lane1 "Enforcer,Topologist"
# Review and validate work from Sessions A & B
```

### Scenario 2: Handoff Between Sessions
```bash
# Session A: Complete initial work
python3 task_checkin_checkout.py complete \
  --task runtime-engine-core \
  --session $BRAIN_SESSION_ID \
  --deliverables "Core engine implemented" \
  --notes "Ready for integration testing"

# Session A ends
python3 session_coordinator.py end --session $BRAIN_SESSION_ID

# Session B: Pick up follow-up work  
./quick_start.sh start Lane1 "Developer,Tester"
./quick_start.sh reserve agent-communication-infrastructure Developer
```

### Scenario 3: Collaborative Review
```bash
# Multiple sessions working on related tasks
# Session A reserves and implements
# Session B reviews and provides feedback
# Session C integrates and tests

# Coordination through lane management and task dependencies
python3 lane_manager.py check-deps --lane Lane2  # Verify Lane1 completion
python3 task_checkin_checkout.py available --lane Lane2  # Show ready tasks
```

## 📊 Monitoring and Reporting

### Real-time Monitoring
```bash
# System health dashboard (run periodically)
watch -n 60 './quick_start.sh status'

# Activity monitoring
watch -n 300 'python3 log_aggregator.py activity --hours 1'
```

### Daily Operations
```bash
# Morning standup information
./quick_start.sh status
python3 log_aggregator.py performance --hours 24
python3 log_aggregator.py errors --hours 24

# End of day reporting
./quick_start.sh reports
python3 log_aggregator.py daily-report --save
```

### Weekly Reviews
```bash
# Weekly performance review
python3 log_aggregator.py performance --hours 168 > weekly_performance.json
python3 log_aggregator.py agents --hours 168 > weekly_agents.json

# System cleanup
./quick_start.sh cleanup
```

## 🔧 Configuration and Customization

### Environment Variables
```bash
# Optional: Customize paths
export BRAIN_ROOT="/path/to/CollaborativeIntelligence/BRAIN"
export BRAIN_SESSION_ID="your-session-id"  # Auto-set by quick_start.sh

# Optional: Customize timeouts
export BRAIN_TASK_TIMEOUT_HOURS="6"
export BRAIN_SESSION_TIMEOUT_MINUTES="20"
```

### Adding New Tasks
Edit the task registry initialization in `task_checkin_checkout.py` or manually edit `TASK_REGISTRY.json` to add new tasks with proper structure:

```json
{
  "new-task-id": {
    "id": "new-task-id",
    "title": "New Task Implementation", 
    "description": "Description of what needs to be done",
    "lane": "Lane1",
    "phase": "Foundation",
    "priority": "P1",
    "estimated_hours": 40,
    "status": "available",
    "dependencies": ["prerequisite-task-id"],
    "assigned_agents": ["Agent1", "Agent2"],
    "subtasks": ["subtask-1", "subtask-2"],
    "created_date": "2025-08-30T12:00:00",
    "completion_criteria": [
      "Criterion 1 met",
      "Criterion 2 verified"
    ]
  }
}
```

### Adding New Lanes
Edit the lane initialization in `lane_manager.py` or manually edit `LANE_MANAGEMENT.json`.

## 🚨 Troubleshooting

### Common Issues

#### Task Conflicts
```bash
# If task reservation fails due to conflicts:
python3 task_checkin_checkout.py status --task task-id
# Check if task is already reserved or has unmet dependencies

# Force release if needed (be careful!)
python3 task_checkin_checkout.py release --task task-id --session session-id --reason "Force release"
```

#### Session Timeouts
```bash
# If session times out:
python3 session_coordinator.py status --session your-session-id
# Check last heartbeat time

# Re-register if needed:
python3 session_coordinator.py register --session your-session-id --lane Lane1
```

#### File Locking Issues
```bash
# If you get file locking errors:
# Wait a moment and retry - another session may be writing
# Or check for stale processes:
ps aux | grep python3 | grep coordination
```

### Recovery Procedures

#### System Reset
```bash
# Clean reset (careful - will lose active session data!)
rm -f ../TASK_REGISTRY.json ../LANE_MANAGEMENT.json ../SESSION_COORDINATION.json
./quick_start.sh init
```

#### Backup and Restore
```bash
# Backup coordination data
tar czf brain_coordination_backup_$(date +%Y%m%d_%H%M).tar.gz \
  ../TASK_REGISTRY.json ../LANE_MANAGEMENT.json ../SESSION_COORDINATION.json ../DEVELOPMENT_LOG.jsonl

# Restore from backup
tar xzf brain_coordination_backup_20250830_1200.tar.gz
```

## 🔗 Integration with Task Tool

When using Claude's Task tool to activate agents, include session coordination:

```markdown
Use the Task tool with the following agents for runtime engine implementation:

**Session Setup:**
- Session ID: claude-hostname-1234567890 (from $BRAIN_SESSION_ID)
- Lane: Lane1
- Task: runtime-engine-core (reserved)

**Agents to Activate:** Manager, Architect, Developer

**Task Context:** 
We're implementing the core runtime engine for the BRAIN system. This includes:
- Agent lifecycle management
- Task scheduling engine
- Session management system
- Integration testing

**Progress Reporting:**
Please update progress using:
```bash
python3 /path/to/coordination/task_checkin_checkout.py progress \
  --task runtime-engine-core --session $BRAIN_SESSION_ID --percent XX --notes "Progress description"
```

**Completion:**
When finished, mark task complete with deliverables and update the session.
```

## 📚 Additional Resources

- **Main Orchestration Document:** `../MULTI_AGENT_DEVELOPMENT_ORCHESTRATION.md`
- **System Specification:** `../SPECIFICATION.md`  
- **Development Plan:** `../DEVELOPMENT.md`
- **Usage Guide:** `../USAGE.md`

---

**System Status:** Ready for multi-agent development coordination  
**Last Updated:** 2025-08-30  
**Version:** 1.0