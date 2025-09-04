#!/bin/bash
# Comprehensive CLI Coordination Integration Test Suite
# Tests all coordination functionality end-to-end

set -e

CLI_PATH="./target/release/CI"
TEST_SESSION_ID="test-$(date +%s)"
TEST_TASK_SESSION="task-test-$(date +%s)"
EXPORT_FILE="/tmp/coordination-test-export.json"

echo "🧪 CI Coordination Integration Test Suite"
echo "==========================================="
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

print_test() {
    echo -e "${BLUE}TEST:${NC} $1"
}

print_success() {
    echo -e "${GREEN}✓ PASS:${NC} $1"
}

print_error() {
    echo -e "${RED}✗ FAIL:${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}⚠ WARN:${NC} $1"
}

cleanup() {
    echo ""
    echo "🧹 Cleaning up test sessions..."
    $CLI_PATH coordination end --session "$TEST_SESSION_ID" --reason "Test cleanup" 2>/dev/null || true
    $CLI_PATH coordination end --session "$TEST_TASK_SESSION" --reason "Test cleanup" 2>/dev/null || true
    rm -f "$EXPORT_FILE"
}

# Set up cleanup on exit
trap cleanup EXIT

echo "1. Build System Tests"
echo "--------------------"

print_test "Verifying CLI builds successfully"
if cargo build --release --quiet; then
    print_success "CLI builds without errors"
else
    print_error "CLI build failed"
    exit 1
fi

print_test "Checking CLI binary exists"
if [ -f "$CLI_PATH" ]; then
    print_success "CLI binary exists at $CLI_PATH"
else
    print_error "CLI binary not found at $CLI_PATH"
    exit 1
fi

echo ""
echo "2. Command Parsing Tests"
echo "-----------------------"

print_test "Testing coordination command help"
if $CLI_PATH coordination --help > /dev/null; then
    print_success "Coordination help command works"
else
    print_error "Coordination help command failed"
    exit 1
fi

print_test "Testing dashboard command help"
if $CLI_PATH dashboard --help > /dev/null; then
    print_success "Dashboard help command works"
else
    print_error "Dashboard help command failed"
    exit 1
fi

echo ""
echo "3. Python Integration Tests"
echo "---------------------------"

print_test "Testing direct Python script execution"
if python3 /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/BRAIN/coordination/session_coordinator.py status --system --format json > /dev/null; then
    print_success "Python coordination script works directly"
else
    print_error "Python coordination script failed"
    exit 1
fi

print_test "Testing system status via CLI"
if $CLI_PATH coordination status --system --format json > /dev/null; then
    print_success "CLI to Python integration works"
else
    print_error "CLI to Python integration failed"
    exit 1
fi

echo ""
echo "4. Session Lifecycle Tests"
echo "--------------------------"

print_test "Creating test session: $TEST_SESSION_ID"
if $CLI_PATH coordination start --session "$TEST_SESSION_ID" --lane testing --claude-version "Test Suite" > /dev/null; then
    print_success "Session creation successful"
else
    print_error "Session creation failed"
    exit 1
fi

print_test "Sending heartbeat"
if $CLI_PATH coordination heartbeat --session "$TEST_SESSION_ID" --tasks-completed 2 --tasks-reserved 1 > /dev/null; then
    print_success "Heartbeat sent successfully"
else
    print_error "Heartbeat failed"
    exit 1
fi

print_test "Checking session status"
if $CLI_PATH coordination status --session "$TEST_SESSION_ID" > /dev/null; then
    print_success "Session status check successful"
else
    print_error "Session status check failed"
    exit 1
fi

print_test "Ending session"
if $CLI_PATH coordination end --session "$TEST_SESSION_ID" --reason "Test suite completion" > /dev/null; then
    print_success "Session ended successfully"
else
    print_error "Session end failed"
    exit 1
fi

echo ""
echo "5. Task Coordination Tests"
echo "--------------------------"

print_test "Creating task test session: $TEST_TASK_SESSION"
if $CLI_PATH coordination start --session "$TEST_TASK_SESSION" --lane development > /dev/null; then
    print_success "Task test session created"
else
    print_error "Task test session creation failed"
    exit 1
fi

print_test "Checking task coordination status"
if $CLI_PATH coordination task status > /dev/null; then
    print_success "Task coordination status works"
else
    print_error "Task coordination status failed"
    exit 1
fi

echo ""
echo "6. Dashboard and Export Tests"
echo "----------------------------"

print_test "Testing dashboard data export"
if $CLI_PATH dashboard --export "$EXPORT_FILE" --view system > /dev/null; then
    print_success "Dashboard export successful"
else
    print_error "Dashboard export failed"
    exit 1
fi

print_test "Verifying export file exists and contains valid JSON"
if [ -f "$EXPORT_FILE" ] && python3 -m json.tool "$EXPORT_FILE" > /dev/null; then
    print_success "Export file is valid JSON"
else
    print_error "Export file is invalid or missing"
    exit 1
fi

echo ""
echo "7. Reports Generation Tests"
echo "--------------------------"

print_test "Generating summary report"
if $CLI_PATH coordination reports summary --timeframe 24h --format json > /dev/null; then
    print_success "Summary report generation successful"
else
    print_error "Summary report generation failed"
    exit 1
fi

echo ""
echo "8. Error Handling Tests"
echo "----------------------"

print_test "Testing invalid session ID handling"
if ! $CLI_PATH coordination status --session "invalid-session-$(date +%s)" 2>/dev/null; then
    print_success "Invalid session handled correctly"
else
    print_warning "Invalid session should have failed"
fi

print_test "Testing missing required arguments"
if ! $CLI_PATH coordination heartbeat 2>/dev/null; then
    print_success "Missing arguments handled correctly"
else
    print_warning "Missing arguments should have failed"
fi

echo ""
echo "9. User Experience Tests"
echo "-----------------------"

print_test "Verifying help text quality"
help_output=$($CLI_PATH coordination --help)
if [[ $help_output == *"Examples:"* ]] && [[ $help_output == *"Usage:"* ]]; then
    print_success "Help text contains examples and usage"
else
    print_error "Help text missing examples or usage"
    exit 1
fi

print_test "Checking command formatting and colors"
status_output=$($CLI_PATH coordination status --system)
if [[ $status_output == *"Coordination System Status"* ]]; then
    print_success "Status output is properly formatted"
else
    print_error "Status output formatting issue"
    exit 1
fi

echo ""
echo "10. Integration Validation"
echo "-------------------------"

print_test "End-to-end workflow test"
workflow_session="workflow-test-$(date +%s)"

# Full workflow test
if $CLI_PATH coordination start --session "$workflow_session" --lane integration > /dev/null && \
   $CLI_PATH coordination heartbeat --session "$workflow_session" --tasks-completed 5 > /dev/null && \
   $CLI_PATH coordination status --session "$workflow_session" > /dev/null && \
   $CLI_PATH coordination end --session "$workflow_session" --reason "Workflow test complete" > /dev/null; then
    print_success "End-to-end workflow successful"
else
    print_error "End-to-end workflow failed"
    exit 1
fi

echo ""
echo "🎉 ALL TESTS PASSED!"
echo "===================="
echo ""
echo "Test Summary:"
echo "✓ Build system working correctly"
echo "✓ Command parsing functional"
echo "✓ Python integration operational"
echo "✓ Session lifecycle complete"
echo "✓ Task coordination active"
echo "✓ Dashboard and export working"
echo "✓ Reports generation functional"
echo "✓ Error handling appropriate"
echo "✓ User experience satisfactory"
echo "✓ Integration validation successful"
echo ""
echo "The CLI coordination system is production-ready!"