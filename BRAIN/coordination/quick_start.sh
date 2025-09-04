#!/bin/bash
# BRAIN Multi-Agent Development Quick Start
# Initializes and coordinates multi-Claude session development

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
BRAIN_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
COORDINATION_DIR="${BRAIN_ROOT}/coordination"

# Logging
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

log_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check prerequisites
check_prerequisites() {
    log_info "Checking prerequisites..."
    
    # Check Python 3
    if ! command -v python3 &> /dev/null; then
        log_error "Python 3 is required but not installed"
        exit 1
    fi
    
    # Check required Python modules
    python3 -c "import json, argparse, fcntl" 2>/dev/null || {
        log_error "Required Python modules not available"
        exit 1
    }
    
    # Check if we're in the right directory
    if [[ ! -f "${BRAIN_ROOT}/MULTI_AGENT_DEVELOPMENT_ORCHESTRATION.md" ]]; then
        log_error "Not in BRAIN root directory or orchestration file missing"
        exit 1
    fi
    
    log_success "Prerequisites checked"
}

# Initialize coordination system
initialize_system() {
    log_info "Initializing BRAIN coordination system..."
    
    # Create required directories
    mkdir -p "${COORDINATION_DIR}/reports"
    
    # Initialize task registry
    if [[ ! -f "${BRAIN_ROOT}/TASK_REGISTRY.json" ]]; then
        log_info "Initializing task registry..."
        python3 "${COORDINATION_DIR}/task_checkin_checkout.py" available --format json > /dev/null 2>&1 || true
        log_success "Task registry initialized"
    else
        log_info "Task registry already exists"
    fi
    
    # Initialize lane management
    if [[ ! -f "${BRAIN_ROOT}/LANE_MANAGEMENT.json" ]]; then
        log_info "Initializing lane management..."
        python3 "${COORDINATION_DIR}/lane_manager.py" status --all --format json > /dev/null 2>&1 || true
        log_success "Lane management initialized"
    else
        log_info "Lane management already exists"
    fi
    
    # Initialize session coordination
    if [[ ! -f "${BRAIN_ROOT}/SESSION_COORDINATION.json" ]]; then
        log_info "Initializing session coordination..."
        python3 "${COORDINATION_DIR}/session_coordinator.py" status --system > /dev/null 2>&1 || true
        log_success "Session coordination initialized"
    else
        log_info "Session coordination already exists"
    fi
    
    log_success "System initialization complete"
}

# Start new development session
start_session() {
    local lane="$1"
    local agent_list="$2"
    
    log_info "Starting new development session..."
    
    # Generate session ID
    local session_id="claude-$(hostname)-$(date +%s)"
    
    # Register session
    python3 "${COORDINATION_DIR}/session_coordinator.py" register \
        --session "${session_id}" \
        --lane "${lane}" \
        --claude-version "Claude Code"
    
    # Assign session to lane
    python3 "${COORDINATION_DIR}/lane_manager.py" assign-session \
        --session "${session_id}" \
        --lane "${lane}"
    
    # Assign agents to lane
    IFS=',' read -ra AGENTS <<< "$agent_list"
    for agent in "${AGENTS[@]}"; do
        agent=$(echo "$agent" | xargs) # Trim whitespace
        python3 "${COORDINATION_DIR}/lane_manager.py" assign \
            --agent "${agent}" \
            --lane "${lane}" \
            --role contributor
    done
    
    log_success "Session started: ${session_id}"
    log_info "Assigned to lane: ${lane}"
    log_info "Agents: ${agent_list}"
    
    # Show available tasks
    log_info "Available tasks in ${lane}:"
    python3 "${COORDINATION_DIR}/task_checkin_checkout.py" available \
        --lane "${lane}" --format table
    
    echo ""
    log_info "Session Management Commands:"
    echo "  Reserve task:  python3 ${COORDINATION_DIR}/task_checkin_checkout.py reserve --task TASK_ID --session ${session_id} --agent AGENT_NAME"
    echo "  Send heartbeat: python3 ${COORDINATION_DIR}/session_coordinator.py heartbeat --session ${session_id}"
    echo "  Check status:  python3 ${COORDINATION_DIR}/session_coordinator.py status --session ${session_id}"
    echo "  End session:   python3 ${COORDINATION_DIR}/session_coordinator.py end --session ${session_id}"
    echo ""
    
    # Export session ID for easy access
    echo "export BRAIN_SESSION_ID='${session_id}'" > "${COORDINATION_DIR}/.current_session"
    log_success "Session ID exported to \$BRAIN_SESSION_ID"
    
    return 0
}

# Show system status
show_status() {
    log_info "=== BRAIN Development System Status ==="
    echo ""
    
    log_info "Session Status:"
    python3 "${COORDINATION_DIR}/session_coordinator.py" status --system --format table
    echo ""
    
    log_info "Lane Status:"
    python3 "${COORDINATION_DIR}/lane_manager.py" status --all --format table
    echo ""
    
    log_info "Task Summary:"
    python3 "${COORDINATION_DIR}/task_checkin_checkout.py" status --system | grep -E "(available|reserved|completed)_tasks"
    echo ""
}

# Show available tasks
show_tasks() {
    local lane="${1:-}"
    
    log_info "=== Available Tasks ==="
    if [[ -n "$lane" ]]; then
        log_info "Filtering by lane: $lane"
        python3 "${COORDINATION_DIR}/task_checkin_checkout.py" available --lane "$lane" --format table
    else
        python3 "${COORDINATION_DIR}/task_checkin_checkout.py" available --format table
    fi
}

# Quick task workflow
quick_task() {
    local task_id="$1"
    local agent="$2"
    
    # Check if session is active
    if [[ -f "${COORDINATION_DIR}/.current_session" ]]; then
        source "${COORDINATION_DIR}/.current_session"
        local session_id="$BRAIN_SESSION_ID"
    else
        log_error "No active session. Run 'quick_start.sh start' first"
        exit 1
    fi
    
    log_info "Quick task workflow for: $task_id"
    
    # Reserve task
    log_info "Reserving task..."
    if python3 "${COORDINATION_DIR}/task_checkin_checkout.py" reserve \
        --task "$task_id" \
        --session "$session_id" \
        --agent "$agent"; then
        log_success "Task reserved successfully"
    else
        log_error "Failed to reserve task"
        exit 1
    fi
    
    # Send heartbeat
    python3 "${COORDINATION_DIR}/session_coordinator.py" heartbeat --session "$session_id"
    
    log_info "Task Management Commands:"
    echo "  Update progress: python3 ${COORDINATION_DIR}/task_checkin_checkout.py progress --task $task_id --session $session_id --percent PERCENT --notes 'Progress notes'"
    echo "  Complete task:   python3 ${COORDINATION_DIR}/task_checkin_checkout.py complete --task $task_id --session $session_id --deliverables 'Deliverable description' --notes 'Completion notes'"
    echo "  Release task:    python3 ${COORDINATION_DIR}/task_checkin_checkout.py release --task $task_id --session $session_id --reason 'Release reason'"
    echo ""
}

# Generate reports
generate_reports() {
    log_info "Generating development reports..."
    
    local date_str=$(date +%Y-%m-%d)
    
    # Daily activity report
    log_info "Generating daily report..."
    python3 "${COORDINATION_DIR}/log_aggregator.py" daily-report --save
    log_success "Daily report saved"
    
    # System status report
    log_info "Generating system status..."
    {
        echo "# BRAIN System Status Report"
        echo "Generated: $(date)"
        echo ""
        echo "## Session Status"
        python3 "${COORDINATION_DIR}/session_coordinator.py" status --system --format json
        echo ""
        echo "## Lane Status"
        python3 "${COORDINATION_DIR}/lane_manager.py" status --all --format json
        echo ""
        echo "## Task Status"
        python3 "${COORDINATION_DIR}/task_checkin_checkout.py" status --system
    } > "${COORDINATION_DIR}/reports/system_status_${date_str}.md"
    
    log_success "Reports generated in ${COORDINATION_DIR}/reports/"
}

# Cleanup stale sessions
cleanup_system() {
    log_info "Cleaning up stale sessions and tasks..."
    
    # This will trigger automatic cleanup in the session coordinator
    python3 "${COORDINATION_DIR}/session_coordinator.py" status --system > /dev/null
    
    # Clean up task registry
    python3 "${COORDINATION_DIR}/task_checkin_checkout.py" available > /dev/null
    
    log_success "System cleanup complete"
}

# Show help
show_help() {
    echo "BRAIN Multi-Agent Development Quick Start"
    echo "Usage: $0 [COMMAND] [OPTIONS]"
    echo ""
    echo "Commands:"
    echo "  init                           Initialize the coordination system"
    echo "  start <lane> <agents>         Start new development session"
    echo "                                 Example: start Lane1 'Manager,Architect,Developer'"
    echo "  status                        Show system status"
    echo "  tasks [lane]                  Show available tasks (optionally filtered by lane)"
    echo "  reserve <task_id> <agent>     Quick reserve a task"
    echo "  reports                       Generate development reports"
    echo "  cleanup                       Clean up stale sessions and tasks"
    echo "  help                          Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 init                                    # Initialize system"
    echo "  $0 start Lane1 'Manager,Architect'        # Start session on Lane1"
    echo "  $0 reserve runtime-engine-core Manager    # Reserve a task"
    echo "  $0 status                                  # Show system status"
    echo "  $0 tasks Lane1                             # Show Lane1 tasks"
    echo ""
    echo "For more detailed usage, see MULTI_AGENT_DEVELOPMENT_ORCHESTRATION.md"
}

# Send heartbeat for current session
send_heartbeat() {
    if [[ -f "${COORDINATION_DIR}/.current_session" ]]; then
        source "${COORDINATION_DIR}/.current_session"
        local session_id="$BRAIN_SESSION_ID"
        
        if python3 "${COORDINATION_DIR}/session_coordinator.py" heartbeat --session "$session_id"; then
            log_success "Heartbeat sent for session $session_id"
        else
            log_error "Failed to send heartbeat"
            exit 1
        fi
    else
        log_error "No active session found"
        exit 1
    fi
}

# Main command processing
main() {
    case "${1:-help}" in
        "init")
            check_prerequisites
            initialize_system
            ;;
        "start")
            if [[ $# -lt 3 ]]; then
                log_error "Usage: $0 start <lane> <agents>"
                log_error "Example: $0 start Lane1 'Manager,Architect,Developer'"
                exit 1
            fi
            check_prerequisites
            initialize_system
            start_session "$2" "$3"
            ;;
        "status")
            check_prerequisites
            show_status
            ;;
        "tasks")
            check_prerequisites
            show_tasks "${2:-}"
            ;;
        "reserve")
            if [[ $# -lt 3 ]]; then
                log_error "Usage: $0 reserve <task_id> <agent>"
                exit 1
            fi
            check_prerequisites
            quick_task "$2" "$3"
            ;;
        "heartbeat")
            check_prerequisites
            send_heartbeat
            ;;
        "reports")
            check_prerequisites
            generate_reports
            ;;
        "cleanup")
            check_prerequisites
            cleanup_system
            ;;
        "help"|"-h"|"--help")
            show_help
            ;;
        *)
            log_error "Unknown command: ${1:-}"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"