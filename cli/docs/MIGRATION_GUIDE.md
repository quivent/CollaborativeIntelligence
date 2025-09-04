# Migration Guide: Legacy Scripts to CLI Commands

## Overview

This guide helps users migrate from legacy scripts (such as `quick_start.sh`) to the new CI CLI coordination system. The migration provides improved functionality, better error handling, comprehensive monitoring, and standardized workflows.

## Migration Benefits

### Before (Legacy Scripts)
- Manual session management
- Limited coordination between instances
- No centralized monitoring
- Inconsistent error handling
- Bash-specific implementations

### After (CLI Coordination System)
- Automated session lifecycle management
- Multi-agent coordination and task management
- Real-time monitoring dashboards
- Comprehensive error handling and recovery
- Language-agnostic integration (Python, Node.js, etc.)

## Command Migration Matrix

### Basic Operations

| Legacy Script | New CLI Command | Notes |
|---------------|-----------------|-------|
| `./quick_start.sh` | `ci coordination start` | Enhanced with session management |
| Manual process management | `ci coordination heartbeat` | Automated health monitoring |
| No centralized status | `ci coordination status` | Comprehensive status reporting |
| Manual cleanup | `ci coordination cleanup` | Automated stale session cleanup |
| No monitoring | `ci dashboard` | Real-time monitoring dashboard |

### Detailed Command Mappings

#### Starting Sessions
```bash
# Legacy approach
./quick_start.sh --project myproject

# New CLI approach
ci coordination start --lane development --context "myproject development" --claude-version "claude-3.5-sonnet"
```

#### Session Management
```bash
# Legacy: No built-in session management
# Manual process tracking required

# New CLI: Full session lifecycle
SESSION_ID=$(ci coordination start --lane development | grep -oP 'session: \K\S+')
ci coordination heartbeat --session $SESSION_ID --tasks-completed 3
ci coordination end --session $SESSION_ID --reason "Work completed"
```

#### Monitoring and Status
```bash
# Legacy: No centralized monitoring
ps aux | grep claude  # Manual process checking

# New CLI: Comprehensive monitoring
ci coordination status --all
ci dashboard --view system
ci coordination reports summary
```

## Step-by-Step Migration Process

### Phase 1: Assessment and Planning

#### 1. Inventory Existing Scripts
```bash
# Find all legacy scripts in your project
find . -name "*.sh" -type f -exec grep -l "claude\|quick_start" {} \;

# Document current workflow patterns
# - How many concurrent sessions do you typically run?
# - What coordination is needed between sessions?
# - What monitoring/reporting is currently done?
```

#### 2. Identify Dependencies
```bash
# Check for script dependencies
grep -r "source\|\./" your_scripts/

# Document environment variables used
grep -r "export\|\$[A-Z]" your_scripts/
```

#### 3. Plan Migration Strategy
- **Incremental Migration**: Replace scripts one at a time
- **Parallel Running**: Run both systems temporarily for validation
- **Team Coordination**: Ensure all team members understand the changes

### Phase 2: CLI Setup and Validation

#### 1. Install CLI System
```bash
# Install the CLI
cd /path/to/CollaborativeIntelligence/cli
cargo install --path .

# Verify installation
ci --version
ci coordination status --system
```

#### 2. Test Basic Functionality
```bash
# Test session creation
TEST_SESSION=$(ci coordination start --lane test --context "Migration testing" | grep -oP 'session: \K\S+')

# Test heartbeat
ci coordination heartbeat --session $TEST_SESSION --tasks-completed 1

# Test status checking
ci coordination status --session $TEST_SESSION

# Test cleanup
ci coordination end --session $TEST_SESSION --reason "Migration test completed"
```

#### 3. Validate Integration
```bash
# Test dashboard functionality
ci dashboard --export test-export.json --view system

# Verify Python integration works
python3 -c "
import subprocess
result = subprocess.run(['ci', 'coordination', 'status', '--system'], capture_output=True, text=True)
print('CLI integration test:', 'PASSED' if result.returncode == 0 else 'FAILED')
"
```

### Phase 3: Script Conversion

#### Example: Converting quick_start.sh

##### Legacy Script Pattern
```bash
#!/bin/bash
# legacy_quick_start.sh

PROJECT_NAME="${1:-default-project}"
CLAUDE_VERSION="${2:-claude-3.5-sonnet}"

echo "Starting Claude Code session for project: $PROJECT_NAME"

# Manual process management
export PROJECT_CONTEXT="$PROJECT_NAME"
export CLAUDE_VERSION="$CLAUDE_VERSION"

# Start Claude Code (simplified)
claude_code_start() {
    # Legacy implementation
    echo "Claude Code started with PID: $$"
    
    # Manual heartbeat (if any)
    while true; do
        echo "Session active at $(date)"
        sleep 300
    done
}

claude_code_start &
CLAUDE_PID=$!
echo "Claude session started with PID: $CLAUDE_PID"

# Manual cleanup on exit
cleanup() {
    echo "Cleaning up session..."
    kill $CLAUDE_PID 2>/dev/null
    echo "Session ended"
}

trap cleanup EXIT
wait $CLAUDE_PID
```

##### Migrated CLI Version
```bash
#!/bin/bash
# modern_coordination_start.sh

PROJECT_NAME="${1:-default-project}"
CLAUDE_VERSION="${2:-claude-3.5-sonnet}"
LANE="${3:-development}"

echo "Starting coordinated Claude session for project: $PROJECT_NAME"

# Enhanced session management with coordination
start_coordinated_session() {
    local session_id
    
    # Start coordination session with enhanced context
    session_id=$(ci coordination start \
        --lane "$LANE" \
        --claude-version "$CLAUDE_VERSION" \
        --context "Project: $PROJECT_NAME" | \
        grep -oP 'session: \K\S+')
    
    if [ -z "$session_id" ]; then
        echo "Failed to start coordination session"
        exit 1
    fi
    
    echo "Coordination session started: $session_id"
    
    # Export session ID for other processes
    export CI_SESSION_ID="$session_id"
    
    # Start heartbeat monitoring in background
    start_heartbeat_monitor "$session_id" &
    HEARTBEAT_PID=$!
    
    # Enhanced cleanup
    cleanup() {
        echo "Ending coordinated session..."
        
        # Stop heartbeat monitor
        kill $HEARTBEAT_PID 2>/dev/null
        
        # Gracefully end session
        ci coordination end --session "$session_id" --reason "Script completion"
        
        echo "Session ended gracefully"
    }
    
    trap cleanup EXIT
    
    # Start main work process
    main_work_process "$session_id"
}

start_heartbeat_monitor() {
    local session_id="$1"
    local task_count=0
    
    while true; do
        # Enhanced heartbeat with progress tracking
        ci coordination heartbeat \
            --session "$session_id" \
            --tasks-completed "$task_count" \
            --tasks-reserved 1
        
        # Simulate work progress
        task_count=$((task_count + 1))
        
        sleep 300  # 5-minute heartbeat interval
    done
}

main_work_process() {
    local session_id="$1"
    
    echo "Starting main work process for session: $session_id"
    
    # Your actual work logic here
    # This replaces the original Claude Code startup
    
    # Example: Reserve and complete tasks
    ci coordination task reserve \
        --session "$session_id" \
        --task "$PROJECT_NAME-development" \
        --priority high
    
    # Simulate work
    echo "Working on $PROJECT_NAME..."
    sleep 30
    
    # Complete task
    ci coordination task complete \
        --session "$session_id" \
        --task "$PROJECT_NAME-development" \
        --result '{"status": "completed", "project": "'$PROJECT_NAME'"}'
    
    echo "Main work completed for session: $session_id"
}

# Start the coordinated session
start_coordinated_session
```

#### Enhanced Multi-Session Coordination
```bash
#!/bin/bash
# multi_session_coordinator.sh

PROJECT_NAME="${1:-multi-component-project}"

echo "Starting multi-agent coordination for project: $PROJECT_NAME"

# Session configuration
declare -A SESSIONS=(
    ["frontend"]="React UI development"
    ["backend"]="API and database development"
    ["testing"]="Test automation and QA"
    ["documentation"]="Technical documentation"
)

# Start coordinated sessions
start_multi_session_project() {
    local project="$1"
    local session_ids=()
    
    echo "Initializing multi-agent coordination for: $project"
    
    # Start sessions for each component
    for lane in "${!SESSIONS[@]}"; do
        local context="${SESSIONS[$lane]}"
        local session_id
        
        echo "Starting $lane session: $context"
        
        session_id=$(ci coordination start \
            --lane "$lane" \
            --claude-version "claude-3.5-sonnet" \
            --context "Project: $project - $context" | \
            grep -oP 'session: \K\S+')
        
        if [ -n "$session_id" ]; then
            session_ids+=("$session_id")
            echo "✓ $lane session started: $session_id"
            
            # Reserve initial tasks for each session
            ci coordination task reserve \
                --session "$session_id" \
                --task "$project-$lane-setup" \
                --priority high
        else
            echo "✗ Failed to start $lane session"
        fi
    done
    
    # Export session IDs for external scripts
    printf '%s\n' "${session_ids[@]}" > "$project.session_ids"
    
    echo "Multi-agent coordination initialized"
    echo "Session IDs saved to: $project.session_ids"
    echo "Monitor with: ci dashboard --view lanes"
    
    # Setup monitoring
    monitor_multi_session_project "${session_ids[@]}"
}

monitor_multi_session_project() {
    local session_ids=("$@")
    
    echo "Starting coordination monitoring..."
    
    # Monitoring loop
    while true; do
        local active_count=0
        
        for session_id in "${session_ids[@]}"; do
            if ci coordination status --session "$session_id" >/dev/null 2>&1; then
                active_count=$((active_count + 1))
            fi
        done
        
        echo "Active sessions: $active_count/${#session_ids[@]}"
        
        if [ $active_count -eq 0 ]; then
            echo "All sessions completed"
            break
        fi
        
        sleep 60
    done
    
    # Generate final report
    ci coordination reports summary \
        --format markdown \
        --output "$PROJECT_NAME-completion-report.md"
    
    echo "Project coordination completed. Report: $PROJECT_NAME-completion-report.md"
}

# Cleanup function
cleanup_project() {
    echo "Cleaning up project coordination..."
    
    if [ -f "$PROJECT_NAME.session_ids" ]; then
        while read -r session_id; do
            if [ -n "$session_id" ]; then
                echo "Ending session: $session_id"
                ci coordination end --session "$session_id" --reason "Project cleanup"
            fi
        done < "$PROJECT_NAME.session_ids"
        
        rm -f "$PROJECT_NAME.session_ids"
    fi
    
    echo "Project coordination cleanup completed"
}

trap cleanup_project EXIT

# Start the multi-session project
start_multi_session_project "$PROJECT_NAME"
```

### Phase 4: Team Migration

#### 1. Team Communication
```bash
# Create team migration announcement
cat > MIGRATION_ANNOUNCEMENT.md << 'EOF'
# Migration to CI Coordination System

## What's Changing
We're migrating from legacy scripts to the new CI coordination system.

## Benefits
- Better session management and coordination
- Real-time monitoring dashboards
- Improved error handling and recovery
- Standardized workflows across the team

## Timeline
- Week 1: Setup and testing
- Week 2: Parallel running (old + new)
- Week 3: Full migration to new system

## Action Items
1. Install new CLI: `cargo install --path /path/to/cli`
2. Test basic functionality: `ci coordination status --system`
3. Review migration guide: `/docs/MIGRATION_GUIDE.md`
4. Update bookmarks and documentation

## Support
- Migration guide: `/docs/MIGRATION_GUIDE.md`
- Full documentation: `/docs/CLI_COORDINATION_GUIDE.md`
- Questions: Ask in #dev-coordination channel
EOF
```

#### 2. Training Materials
```bash
# Create training script
cat > training_demo.sh << 'EOF'
#!/bin/bash
# CI Coordination System Training Demo

echo "=== CI Coordination System Training ==="
echo

echo "1. Basic session management:"
echo "   ci coordination start --lane training --context 'Training session'"
echo "   ci coordination status --system"
echo "   ci coordination end --session <session_id> --reason 'Training completed'"
echo

echo "2. Multi-agent coordination:"
echo "   ci coordination start --lane frontend"
echo "   ci coordination start --lane backend"
echo "   ci dashboard --view lanes"
echo

echo "3. Task management:"
echo "   ci coordination task reserve --session <id> --task 'feature-auth' --priority high"
echo "   ci coordination task complete --session <id> --task 'feature-auth'"
echo

echo "4. Monitoring and reports:"
echo "   ci dashboard --view system --refresh 10"
echo "   ci coordination reports summary --format markdown"
echo

echo "Start your training with:"
echo "ci coordination start --lane training --context 'Getting started with coordination'"
EOF

chmod +x training_demo.sh
```

#### 3. Rollout Plan
```bash
#!/bin/bash
# rollout_coordinator.sh

TEAM_MEMBERS=("alice" "bob" "charlie" "diana")
ROLLOUT_PHASE="${1:-planning}"

case "$ROLLOUT_PHASE" in
    "planning")
        echo "Phase 1: Planning and Preparation"
        echo "- Document current workflows"
        echo "- Install CLI on all development machines"
        echo "- Create training materials"
        echo "- Set up team coordination channels"
        ;;
    
    "testing")
        echo "Phase 2: Testing and Validation"
        for member in "${TEAM_MEMBERS[@]}"; do
            echo "- $member: Test basic CLI functionality"
            echo "- $member: Migrate one simple script"
            echo "- $member: Validate dashboard access"
        done
        ;;
    
    "parallel")
        echo "Phase 3: Parallel Running"
        echo "- Run both old and new systems"
        echo "- Compare outputs and identify issues"
        echo "- Gather feedback from team members"
        echo "- Refine migration scripts"
        ;;
    
    "migration")
        echo "Phase 4: Full Migration"
        echo "- Disable legacy scripts"
        echo "- Update all documentation"
        echo "- Train team on new workflows"
        echo "- Establish new monitoring procedures"
        ;;
    
    "completion")
        echo "Phase 5: Migration Completion"
        echo "- Remove legacy script references"
        echo "- Update CI/CD pipelines"
        echo "- Document lessons learned"
        echo "- Celebrate successful migration!"
        ;;
    
    *)
        echo "Usage: $0 {planning|testing|parallel|migration|completion}"
        exit 1
        ;;
esac
```

## Troubleshooting Migration Issues

### Common Migration Problems

#### 1. Session ID Management
**Problem**: Scripts expecting simple process IDs get complex session IDs

**Solution**:
```bash
# Legacy: PID-based tracking
OLD_PID=$$

# New: Session ID-based tracking
SESSION_ID=$(ci coordination start --lane migration | grep -oP 'session: \K\S+')

# Store session ID for later use
echo "$SESSION_ID" > .current_session_id

# Retrieve session ID in other scripts
if [ -f .current_session_id ]; then
    SESSION_ID=$(cat .current_session_id)
fi
```

#### 2. Environment Variable Dependencies
**Problem**: Legacy scripts rely on global environment variables

**Solution**:
```bash
# Legacy approach
export GLOBAL_PROJECT_STATE="active"

# New approach: Use session context
ci coordination start --context "project_state=active,environment=development"

# Retrieve context from session status
SESSION_CONTEXT=$(ci coordination status --session "$SESSION_ID" --format json | jq -r '.user_info.context')
```

#### 3. Process Communication
**Problem**: Inter-process communication between legacy scripts

**Solution**:
```bash
# Legacy: File-based communication
echo "ready" > /tmp/process_state

# New: Task-based coordination
ci coordination task reserve --session "$SESSION_ID" --task "process-ready" --priority high
ci coordination task complete --session "$SESSION_ID" --task "process-ready" --result '{"state": "ready"}'

# Check task status across sessions
ci coordination task status --lane communication
```

#### 4. Error Handling Migration
**Problem**: Different error handling patterns

**Solution**:
```bash
# Legacy error handling
some_command || { echo "Command failed"; exit 1; }

# Enhanced error handling with coordination
if ! some_command; then
    echo "Command failed, updating session status"
    ci coordination heartbeat --session "$SESSION_ID" --activity '{"status": "error", "error": "command_failed"}'
    ci coordination end --session "$SESSION_ID" --reason "Command execution failed"
    exit 1
fi
```

### Migration Validation

#### Validation Checklist
```bash
#!/bin/bash
# migration_validation.sh

echo "=== Migration Validation Checklist ==="

# 1. CLI Installation
if command -v ci &> /dev/null; then
    echo "✓ CI CLI installed"
else
    echo "✗ CI CLI not found"
    exit 1
fi

# 2. Coordination System
if ci coordination status --system &> /dev/null; then
    echo "✓ Coordination system accessible"
else
    echo "✗ Coordination system not accessible"
    exit 1
fi

# 3. Session Management
TEST_SESSION=$(ci coordination start --lane validation --context "Migration validation test" | grep -oP 'session: \K\S+')
if [ -n "$TEST_SESSION" ]; then
    echo "✓ Session creation works"
    
    # Test heartbeat
    if ci coordination heartbeat --session "$TEST_SESSION" --tasks-completed 1; then
        echo "✓ Heartbeat functionality works"
    else
        echo "✗ Heartbeat functionality failed"
    fi
    
    # Test session end
    if ci coordination end --session "$TEST_SESSION" --reason "Validation test completed"; then
        echo "✓ Session termination works"
    else
        echo "✗ Session termination failed"
    fi
else
    echo "✗ Session creation failed"
fi

# 4. Dashboard Access
if ci dashboard --export /tmp/validation_export.json --view system; then
    echo "✓ Dashboard export works"
    rm -f /tmp/validation_export.json
else
    echo "✗ Dashboard export failed"
fi

# 5. Report Generation
if ci coordination reports summary --output /tmp/validation_report.md --format markdown; then
    echo "✓ Report generation works"
    rm -f /tmp/validation_report.md
else
    echo "✗ Report generation failed"
fi

echo "=== Validation Complete ==="
```

## Post-Migration Optimization

### Performance Tuning

#### 1. Heartbeat Optimization
```bash
# Optimize heartbeat frequency based on workload
WORKLOAD_TYPE="${1:-normal}"

case "$WORKLOAD_TYPE" in
    "intensive")
        HEARTBEAT_INTERVAL=60   # 1 minute for intensive work
        ;;
    "normal")
        HEARTBEAT_INTERVAL=300  # 5 minutes for normal work
        ;;
    "idle")
        HEARTBEAT_INTERVAL=900  # 15 minutes for idle sessions
        ;;
esac

# Custom heartbeat function
send_optimized_heartbeat() {
    local session_id="$1"
    local task_count="$2"
    
    ci coordination heartbeat \
        --session "$session_id" \
        --tasks-completed "$task_count" \
        --activity "{\"workload_type\": \"$WORKLOAD_TYPE\", \"heartbeat_interval\": $HEARTBEAT_INTERVAL}"
}
```

#### 2. Resource Monitoring
```bash
# Monitor resource usage after migration
monitor_post_migration_performance() {
    local log_file="post_migration_performance.log"
    
    while true; do
        local timestamp=$(date '+%Y-%m-%d %H:%M:%S')
        local active_sessions=$(ci coordination status --system --format json | jq -r '.system_health.active_sessions')
        local cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | awk -F'%' '{print $1}')
        local memory_usage=$(free | grep Mem | awk '{printf("%.1f"), $3/$2 * 100.0}')
        
        echo "$timestamp,$active_sessions,$cpu_usage,$memory_usage" >> "$log_file"
        sleep 300
    done
}
```

### Workflow Optimization

#### 1. Team Coordination Patterns
```bash
# Optimized team workflow
optimize_team_coordination() {
    local project="$1"
    local team_size="$2"
    
    # Calculate optimal lane distribution
    local lanes_per_member=$((10 / team_size))  # Assume 10 available lanes
    
    echo "Optimizing coordination for $team_size team members"
    echo "Recommended lanes per member: $lanes_per_member"
    
    # Set up lane templates
    LANE_TEMPLATES=(
        "frontend:UI development and styling"
        "backend:API and server-side logic"
        "database:Schema and query optimization"
        "testing:Test automation and QA"
        "documentation:Technical and user documentation"
        "devops:Deployment and infrastructure"
        "security:Security review and implementation"
        "performance:Performance optimization"
        "integration:System integration and testing"
        "maintenance:Bug fixes and maintenance"
    )
    
    # Create optimized session startup script
    for i in $(seq 1 $team_size); do
        local member_lanes=()
        local start_idx=$(((i-1) * lanes_per_member))
        
        for j in $(seq 0 $((lanes_per_member-1))); do
            local lane_idx=$((start_idx + j))
            if [ $lane_idx -lt ${#LANE_TEMPLATES[@]} ]; then
                member_lanes+=("${LANE_TEMPLATES[$lane_idx]}")
            fi
        done
        
        echo "Team member $i recommended lanes:"
        printf '  %s\n' "${member_lanes[@]}"
    done
}
```

## Migration Success Metrics

### Key Performance Indicators

#### 1. Migration Completion Tracking
```bash
# Track migration progress
track_migration_progress() {
    local total_scripts="$1"
    local migrated_scripts="$2"
    
    local completion_percent=$(( (migrated_scripts * 100) / total_scripts ))
    
    echo "Migration Progress: $migrated_scripts/$total_scripts ($completion_percent%)"
    
    # Update coordination system with progress
    ci coordination start --lane migration-tracking --context "progress=$completion_percent"
}
```

#### 2. System Health Monitoring
```bash
# Post-migration health monitoring
monitor_migration_health() {
    local health_status=$(ci coordination status --system --format json | jq -r '.system_health.status')
    local utilization=$(ci coordination status --system --format json | jq -r '.system_health.utilization_percent')
    local timeout_rate=$(ci coordination status --system --format json | jq -r '.system_health.timeout_rate')
    
    echo "System Health: $health_status"
    echo "Utilization: $utilization%"
    echo "Timeout Rate: $timeout_rate"
    
    # Alert if metrics indicate problems
    if [ "$utilization" -gt 90 ] || (( $(echo "$timeout_rate > 0.1" | bc -l) )); then
        echo "ALERT: System metrics indicate potential issues"
        # Send notification to team
    fi
}
```

## Best Practices for Migration

### 1. Incremental Migration Strategy
- Migrate one script at a time
- Maintain parallel systems during transition
- Validate each migration step thoroughly
- Get team feedback before proceeding

### 2. Documentation and Training
- Update all documentation to reflect new commands
- Provide comprehensive training materials
- Create quick reference guides
- Establish support channels for questions

### 3. Testing and Validation
- Test all migrated functionality thoroughly
- Validate performance meets or exceeds legacy system
- Ensure error handling is robust
- Verify team workflows are not disrupted

### 4. Rollback Planning
- Maintain legacy scripts during migration period
- Create rollback procedures for each migration phase
- Document known issues and workarounds
- Plan for emergency rollback scenarios

---

**Last Updated**: 2025-09-01  
**Version**: 1.0  
**Compatibility**: CI CLI v1.0+, Legacy script systems