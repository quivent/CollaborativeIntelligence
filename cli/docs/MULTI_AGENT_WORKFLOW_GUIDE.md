# Multi-Agent Workflow Guide

## Overview

This guide provides practical examples and best practices for coordinating multiple Claude Code sessions using the CI coordination system. Learn how to effectively manage complex development workflows, coordinate team activities, and optimize multi-agent collaboration.

## Core Concepts

### Session Management
- **Session**: Individual Claude Code instance with unique coordination identity
- **Heartbeat**: Regular status updates to maintain session health
- **Lane**: Logical grouping mechanism for organizing related work
- **Task Coordination**: System for reserving and tracking work items

### Coordination Architecture
```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Claude #1     │    │   Claude #2     │    │   Claude #3     │
│  (Frontend)     │    │  (Backend)      │    │  (Testing)      │
│                 │    │                 │    │                 │
├─────────────────┤    ├─────────────────┤    ├─────────────────┤
│  Session ID:    │    │  Session ID:    │    │  Session ID:    │
│  fe-dev-001     │    │  be-dev-001     │    │  test-001       │
│                 │    │                 │    │                 │
│  Lane:          │    │  Lane:          │    │  Lane:          │
│  frontend       │    │  backend        │    │  testing        │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
                     ┌─────────────────────┐
                     │ Coordination System │
                     │                     │
                     │ - Session Registry  │
                     │ - Task Management   │
                     │ - Health Monitoring │
                     │ - Lane Organization │
                     └─────────────────────┘
```

## Starting Multi-Agent Sessions

### Basic Multi-Agent Setup

#### Scenario: Full-Stack Development
```bash
# Terminal 1: Frontend Development
SESSION_FE=$(ci coordination start --lane frontend --claude-version "claude-3.5-sonnet" --context "React components and UI" | grep -oP 'session: \K\S+')
echo "Frontend session: $SESSION_FE"

# Terminal 2: Backend Development
SESSION_BE=$(ci coordination start --lane backend --claude-version "claude-3.5-sonnet" --context "API development and database" | grep -oP 'session: \K\S+')
echo "Backend session: $SESSION_BE"

# Terminal 3: Testing and QA
SESSION_QA=$(ci coordination start --lane testing --claude-version "claude-3.5-sonnet" --context "Test automation and validation" | grep -oP 'session: \K\S+')
echo "Testing session: $SESSION_QA"

# Terminal 4: Monitoring Dashboard
ci dashboard --view lanes --refresh 5
```

#### Advanced Setup with Custom Session IDs
```bash
# Use descriptive session IDs for better tracking
ci coordination start --session "auth-feature-frontend" --lane frontend --context "User authentication UI"
ci coordination start --session "auth-feature-backend" --lane backend --context "Auth API and middleware"
ci coordination start --session "auth-feature-testing" --lane testing --context "Authentication tests"
```

## Task Coordination Workflows

### Sequential Task Processing

#### Example: Feature Development Pipeline
```bash
#!/bin/bash
# feature-pipeline.sh

FEATURE_NAME="user-profile"
BASE_SESSION="profile-dev-$(date +%s)"

# Stage 1: Design and Planning
DESIGN_SESSION="${BASE_SESSION}-design"
ci coordination start --session "$DESIGN_SESSION" --lane design --context "Profile feature design"

# Reserve design tasks
ci coordination task reserve --session "$DESIGN_SESSION" --task "profile-wireframes" --priority high
ci coordination task reserve --session "$DESIGN_SESSION" --task "api-specification" --priority high
ci coordination task reserve --session "$DESIGN_SESSION" --task "database-schema" --priority medium

echo "Design phase started. Complete wireframes and specifications."
read -p "Press enter when design tasks are complete..."

# Complete design tasks
ci coordination task complete --session "$DESIGN_SESSION" --task "profile-wireframes" --result '{"status": "completed", "artifacts": ["wireframes.png"]}'
ci coordination task complete --session "$DESIGN_SESSION" --task "api-specification" --result '{"status": "completed", "artifacts": ["api-spec.yaml"]}'
ci coordination task complete --session "$DESIGN_SESSION" --task "database-schema" --result '{"status": "completed", "artifacts": ["schema.sql"]}'

# Stage 2: Implementation
IMPL_SESSION="${BASE_SESSION}-implementation"
ci coordination start --session "$IMPL_SESSION" --lane development --context "Profile implementation"

# Reserve implementation tasks
ci coordination task reserve --session "$IMPL_SESSION" --task "profile-api" --priority high
ci coordination task reserve --session "$IMPL_SESSION" --task "profile-ui" --priority high
ci coordination task reserve --session "$IMPL_SESSION" --task "profile-tests" --priority medium

echo "Implementation phase started."

# End design session
ci coordination end --session "$DESIGN_SESSION" --reason "Design phase completed"
```

### Parallel Task Processing

#### Example: Microservices Development
```bash
#!/bin/bash
# microservices-coordination.sh

# Start services in parallel
SERVICES=("user-service" "order-service" "payment-service" "notification-service")

for service in "${SERVICES[@]}"; do
    SESSION_ID="${service}-$(date +%s)"
    echo "Starting session for $service: $SESSION_ID"
    
    ci coordination start --session "$SESSION_ID" --lane "$service" --context "Microservice development" &
    
    # Reserve service-specific tasks
    sleep 1  # Brief delay to ensure session is registered
    ci coordination task reserve --session "$SESSION_ID" --task "${service}-implementation" --priority high &
    ci coordination task reserve --session "$SESSION_ID" --task "${service}-testing" --priority medium &
    ci coordination task reserve --session "$SESSION_ID" --task "${service}-documentation" --priority low &
done

wait  # Wait for all background processes

echo "All microservice sessions started. Monitor with:"
echo "ci dashboard --view lanes"
```

### Task Dependencies and Coordination

#### Example: Frontend-Backend Coordination
```bash
#!/bin/bash
# coordinated-development.sh

# Backend must complete API before frontend can integrate
BACKEND_SESSION="api-dev-$(date +%s)"
FRONTEND_SESSION="ui-dev-$(date +%s)"

# Start backend session
ci coordination start --session "$BACKEND_SESSION" --lane backend --context "API development"
ci coordination task reserve --session "$BACKEND_SESSION" --task "user-api" --priority high

# Monitor backend progress
echo "Monitoring backend API development..."

while true; do
    # Check if API task is completed
    BACKEND_STATUS=$(ci coordination status --session "$BACKEND_SESSION" --format json | jq -r '.activity.tasks_completed')
    
    if [ "$BACKEND_STATUS" -gt 0 ]; then
        echo "Backend API completed. Starting frontend development."
        break
    fi
    
    echo "Waiting for backend API... (completed: $BACKEND_STATUS tasks)"
    sleep 30
done

# Start frontend session after backend API is ready
ci coordination start --session "$FRONTEND_SESSION" --lane frontend --context "UI integration with API"
ci coordination task reserve --session "$FRONTEND_SESSION" --task "api-integration" --priority high
ci coordination task reserve --session "$FRONTEND_SESSION" --task "ui-components" --priority medium

echo "Frontend session started with API dependency resolved."
```

## Session Lifecycle Management

### Comprehensive Session Management

#### Health Monitoring and Recovery
```bash
#!/bin/bash
# session-health-monitor.sh

SESSION_ID="$1"
if [ -z "$SESSION_ID" ]; then
    echo "Usage: $0 <session_id>"
    exit 1
fi

while true; do
    # Check session health
    SESSION_STATUS=$(ci coordination status --session "$SESSION_ID" --format json 2>/dev/null)
    
    if [ $? -ne 0 ] || [ -z "$SESSION_STATUS" ]; then
        echo "ALERT: Session $SESSION_ID not found or unhealthy"
        
        # Attempt recovery
        echo "Attempting to register new session..."
        ci coordination start --session "$SESSION_ID" --context "Recovered session"
        
        if [ $? -eq 0 ]; then
            echo "Session recovered successfully"
        else
            echo "CRITICAL: Unable to recover session $SESSION_ID"
            # Send alert to team
            echo "Session $SESSION_ID failed recovery at $(date)" >> session-failures.log
        fi
    else
        # Check if session is stale
        LAST_HEARTBEAT=$(echo "$SESSION_STATUS" | jq -r '.time_since_last_heartbeat_seconds // 0')
        
        if [ "$LAST_HEARTBEAT" -gt 600 ]; then  # 10 minutes
            echo "WARNING: Session $SESSION_ID has stale heartbeat ($LAST_HEARTBEAT seconds)"
            
            # Send heartbeat
            ci coordination heartbeat --session "$SESSION_ID"
            echo "Heartbeat sent for session $SESSION_ID"
        fi
    fi
    
    sleep 60  # Check every minute
done
```

#### Graceful Session Shutdown
```bash
#!/bin/bash
# graceful-shutdown.sh

SESSION_ID="$1"
REASON="${2:-Planned shutdown}"

echo "Initiating graceful shutdown for session $SESSION_ID..."

# Get current session status
SESSION_INFO=$(ci coordination status --session "$SESSION_ID" --format json)

if [ $? -ne 0 ]; then
    echo "Session $SESSION_ID not found"
    exit 1
fi

# Check for active tasks
ACTIVE_TASKS=$(echo "$SESSION_INFO" | jq -r '.activity.tasks_reserved')

if [ "$ACTIVE_TASKS" -gt 0 ]; then
    echo "WARNING: Session has $ACTIVE_TASKS reserved tasks"
    echo "Tasks will be released upon session termination"
    read -p "Continue with shutdown? (y/N): " -n 1 -r
    echo
    
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        echo "Shutdown cancelled"
        exit 0
    fi
fi

# Final heartbeat with completion status
ci coordination heartbeat --session "$SESSION_ID" --activity '{"status": "shutting_down"}'

# End session gracefully
ci coordination end --session "$SESSION_ID" --reason "$REASON"

echo "Session $SESSION_ID shut down gracefully"
```

### Automated Session Maintenance

#### Session Cleanup Script
```bash
#!/bin/bash
# session-maintenance.sh

echo "Starting session maintenance..."

# Get system status
SYSTEM_STATUS=$(ci coordination status --system --format json)
ACTIVE_SESSIONS=$(echo "$SYSTEM_STATUS" | jq -r '.system_health.active_sessions')
TIMEOUT_RATE=$(echo "$SYSTEM_STATUS" | jq -r '.system_health.timeout_rate')

echo "Current active sessions: $ACTIVE_SESSIONS"
echo "Timeout rate: $TIMEOUT_RATE"

# Cleanup stale sessions
echo "Cleaning up stale sessions..."
CLEANED=$(ci coordination cleanup --dry-run | grep "would be cleaned" | wc -l)

if [ "$CLEANED" -gt 0 ]; then
    echo "Found $CLEANED stale sessions"
    read -p "Proceed with cleanup? (y/N): " -n 1 -r
    echo
    
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        ci coordination cleanup
        echo "Cleanup completed"
    fi
fi

# Generate maintenance report
ci coordination reports summary --format markdown --output "maintenance-report-$(date +%Y%m%d).md"
echo "Maintenance report generated"
```

## Lane Management Strategies

### Lane Organization Patterns

#### By Development Phase
```bash
# Phase-based lanes
ci coordination start --lane planning --context "Requirements analysis"
ci coordination start --lane design --context "System architecture"
ci coordination start --lane implementation --context "Feature development"
ci coordination start --lane testing --context "Quality assurance"
ci coordination start --lane deployment --context "Release preparation"
```

#### By Technical Domain
```bash
# Domain-based lanes
ci coordination start --lane frontend --context "React/UI development"
ci coordination start --lane backend --context "Node.js/API development"
ci coordination start --lane database --context "Schema and queries"
ci coordination start --lane infrastructure --context "DevOps and deployment"
ci coordination start --lane mobile --context "React Native development"
```

#### By Feature/Epic
```bash
# Feature-based lanes
ci coordination start --lane auth-system --context "User authentication feature"
ci coordination start --lane payment-flow --context "Payment processing feature"
ci coordination start --lane analytics --context "User analytics feature"
ci coordination start --lane admin-panel --context "Administrative interface"
```

### Dynamic Lane Assignment

#### Load Balancing Script
```bash
#!/bin/bash
# lane-load-balancer.sh

AVAILABLE_LANES=("frontend" "backend" "testing" "documentation")
TARGET_SESSIONS_PER_LANE=2

# Get current lane distribution
LANE_STATUS=$(ci coordination lane status --format json)

# Find lane with lowest session count
MIN_SESSIONS=999
SELECTED_LANE=""

for lane in "${AVAILABLE_LANES[@]}"; do
    SESSION_COUNT=$(echo "$LANE_STATUS" | jq -r ".[\"$lane\"] // 0")
    
    if [ "$SESSION_COUNT" -lt "$MIN_SESSIONS" ] && [ "$SESSION_COUNT" -lt "$TARGET_SESSIONS_PER_LANE" ]; then
        MIN_SESSIONS=$SESSION_COUNT
        SELECTED_LANE=$lane
    fi
done

if [ -n "$SELECTED_LANE" ]; then
    echo "Assigning to lane: $SELECTED_LANE (current sessions: $MIN_SESSIONS)"
    ci coordination start --lane "$SELECTED_LANE" --context "Auto-assigned for load balancing"
else
    echo "All lanes at capacity, using default assignment"
    ci coordination start --context "Default assignment - all lanes full"
fi
```

## Error Handling and Recovery

### Robust Session Management

#### Session Recovery Pattern
```bash
#!/bin/bash
# robust-session-wrapper.sh

SESSION_ID="robust-dev-$(date +%s)"
MAX_RETRIES=3
RETRY_COUNT=0

start_session() {
    ci coordination start --session "$SESSION_ID" --lane development --context "Robust development session"
    return $?
}

send_heartbeat() {
    ci coordination heartbeat --session "$SESSION_ID" --tasks-completed "${1:-0}" --tasks-reserved "${2:-0}"
    return $?
}

# Attempt to start session with retries
while [ $RETRY_COUNT -lt $MAX_RETRIES ]; do
    if start_session; then
        echo "Session started successfully: $SESSION_ID"
        break
    else
        RETRY_COUNT=$((RETRY_COUNT + 1))
        echo "Failed to start session (attempt $RETRY_COUNT/$MAX_RETRIES)"
        
        if [ $RETRY_COUNT -lt $MAX_RETRIES ]; then
            echo "Retrying in 5 seconds..."
            sleep 5
        fi
    fi
done

if [ $RETRY_COUNT -eq $MAX_RETRIES ]; then
    echo "CRITICAL: Unable to start session after $MAX_RETRIES attempts"
    exit 1
fi

# Heartbeat monitoring with recovery
while true; do
    if ! send_heartbeat; then
        echo "Heartbeat failed, attempting recovery..."
        
        # Check if session still exists
        if ! ci coordination status --session "$SESSION_ID" >/dev/null 2>&1; then
            echo "Session lost, attempting to re-register..."
            
            if start_session; then
                echo "Session recovered: $SESSION_ID"
            else
                echo "CRITICAL: Session recovery failed"
                exit 1
            fi
        fi
    fi
    
    sleep 300  # 5-minute heartbeat interval
done
```

### Coordination System Monitoring

#### System Health Check
```bash
#!/bin/bash
# system-health-check.sh

check_system_health() {
    local health_status
    health_status=$(ci coordination status --system --format json | jq -r '.system_health.status')
    
    case "$health_status" in
        "healthy")
            echo "✓ System is healthy"
            return 0
            ;;
        "warning")
            echo "⚠ System is in warning state"
            return 1
            ;;
        "overloaded")
            echo "✗ System is overloaded"
            return 2
            ;;
        *)
            echo "? Unknown system status: $health_status"
            return 3
            ;;
    esac
}

# Comprehensive health check
echo "Performing system health check..."

if ! check_system_health; then
    echo "System health check failed, investigating..."
    
    # Get detailed system status
    SYSTEM_INFO=$(ci coordination status --system --format json)
    ACTIVE_SESSIONS=$(echo "$SYSTEM_INFO" | jq -r '.system_health.active_sessions')
    MAX_SESSIONS=$(echo "$SYSTEM_INFO" | jq -r '.system_health.max_concurrent')
    UTILIZATION=$(echo "$SYSTEM_INFO" | jq -r '.system_health.utilization_percent')
    TIMEOUT_RATE=$(echo "$SYSTEM_INFO" | jq -r '.system_health.timeout_rate')
    
    echo "Active sessions: $ACTIVE_SESSIONS/$MAX_SESSIONS ($UTILIZATION%)"
    echo "Timeout rate: $TIMEOUT_RATE"
    
    # Recommend actions based on metrics
    if [ "$UTILIZATION" -gt 90 ]; then
        echo "RECOMMENDATION: Consider increasing max_concurrent_sessions"
    fi
    
    if (( $(echo "$TIMEOUT_RATE > 0.1" | bc -l) )); then
        echo "RECOMMENDATION: High timeout rate detected, check session health"
    fi
    
    # Auto-cleanup if needed
    echo "Running automatic cleanup..."
    ci coordination cleanup
fi

echo "Health check completed"
```

## Performance Optimization

### Efficient Heartbeat Patterns

#### Smart Heartbeat Scheduling
```bash
#!/bin/bash
# smart-heartbeat.sh

SESSION_ID="$1"
WORK_STATE="${2:-idle}"  # idle, active, busy

# Adjust heartbeat frequency based on work state
case "$WORK_STATE" in
    "idle")
        HEARTBEAT_INTERVAL=600  # 10 minutes for idle sessions
        ;;
    "active")
        HEARTBEAT_INTERVAL=300  # 5 minutes for active work
        ;;
    "busy")
        HEARTBEAT_INTERVAL=60   # 1 minute for intensive work
        ;;
esac

echo "Using $HEARTBEAT_INTERVAL second heartbeat interval for $WORK_STATE state"

while true; do
    if ! ci coordination heartbeat --session "$SESSION_ID"; then
        echo "Heartbeat failed for $SESSION_ID"
        exit 1
    fi
    
    sleep $HEARTBEAT_INTERVAL
done
```

### Resource Management

#### Session Resource Monitoring
```bash
#!/bin/bash
# resource-monitor.sh

# Monitor system resources during multi-agent coordination
while true; do
    ACTIVE_SESSIONS=$(ci coordination status --system --format json | jq -r '.system_health.active_sessions')
    
    # System resource usage
    CPU_USAGE=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | awk -F'%' '{print $1}')
    MEMORY_USAGE=$(free | grep Mem | awk '{printf("%.1f"), $3/$2 * 100.0}')
    
    # Log resource usage with session count
    TIMESTAMP=$(date '+%Y-%m-%d %H:%M:%S')
    echo "$TIMESTAMP,$ACTIVE_SESSIONS,$CPU_USAGE,$MEMORY_USAGE" >> resource-usage.csv
    
    # Alert if resources are high
    if (( $(echo "$CPU_USAGE > 80" | bc -l) )) || (( $(echo "$MEMORY_USAGE > 85" | bc -l) )); then
        echo "ALERT: High resource usage - CPU: $CPU_USAGE%, Memory: $MEMORY_USAGE%"
        echo "Active sessions: $ACTIVE_SESSIONS"
    fi
    
    sleep 60
done
```

## Best Practices Summary

### Session Management
1. **Use Descriptive Session IDs**: Include project, feature, or role information
2. **Implement Health Monitoring**: Regular heartbeats and status checks
3. **Graceful Shutdown**: Always end sessions with appropriate reasons
4. **Error Recovery**: Implement retry logic and fallback mechanisms

### Task Coordination
1. **Clear Task Boundaries**: Define tasks with specific, measurable outcomes
2. **Priority Management**: Use priority levels to guide work sequencing
3. **Dependency Tracking**: Coordinate dependent tasks across sessions
4. **Progress Reporting**: Regular updates on task completion status

### Lane Organization
1. **Consistent Naming**: Use standardized lane naming conventions
2. **Load Balancing**: Distribute work evenly across available lanes
3. **Logical Grouping**: Organize lanes by domain, phase, or feature
4. **Dynamic Assignment**: Implement automatic lane selection based on load

### Performance Optimization
1. **Efficient Heartbeats**: Adjust frequency based on activity level
2. **Resource Monitoring**: Track system resource usage during coordination
3. **Cleanup Automation**: Regular maintenance of stale sessions
4. **Monitoring Integration**: Use dashboards for real-time visibility

### Team Coordination
1. **Shared Visibility**: Use web dashboard for team awareness
2. **Communication**: Document session purposes and progress
3. **Standardized Processes**: Consistent workflow patterns across the team
4. **Knowledge Sharing**: Regular reviews of coordination effectiveness

---

**Last Updated**: 2025-09-01  
**Version**: 1.0  
**Compatibility**: CI CLI v1.0+, Bash 4.0+