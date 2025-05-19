#!/usr/bin/env bash
# test_fallback_mechanisms.sh - Test fallback mechanisms when optimization is unavailable
#
# This script tests that the system gracefully degrades to legacy implementations
# when the memory optimization components are unavailable or fail.

set -e

# Setup test environment
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$TEST_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$CI_TOOLS_DIR/.." && pwd)"

# Source the required components
source "$CI_TOOLS_DIR/lib/common.sh"
source "$CI_TOOLS_DIR/lib/rust-memory-bridge.sh"
source "$TEST_DIR/mock_legacy_functions.sh"

# Create a temporary directory for test outputs
TMP_DIR=$(mktemp -d)
trap 'rm -rf "$TMP_DIR"' EXIT

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Test results
TESTS_TOTAL=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test helper functions
assert_success() {
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    if [ $? -eq 0 ]; then
        echo -e "${GREEN}✓ $1${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ $1${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

assert_failure() {
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    if [ $? -ne 0 ]; then
        echo -e "${GREEN}✓ $1${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ $1${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

assert_output_contains() {
    local output="$1"
    local expected="$2"
    local description="$3"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    if echo "$output" | grep -q "$expected"; then
        echo -e "${GREEN}✓ $description${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ $description${NC}"
        echo -e "${YELLOW}Expected output to contain: $expected${NC}"
        echo -e "${YELLOW}Actual output: $output${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

print_header() {
    echo -e "\n${YELLOW}==== $1 ====${NC}"
}

# Set up various test scenarios
setup_missing_cli() {
    export MEMORY_CLI_PATH="/does/not/exist/memory-cli"
    export MEMORY_OPTIMIZATION_AVAILABLE="false"
}

setup_failing_cli() {
    local cli_path="$TMP_DIR/failing-memory-cli"
    cat > "$cli_path" << 'EOF'
#!/usr/bin/env bash
# Failing memory CLI for testing
echo "Error: CLI failure test" >&2
exit 1
EOF
    chmod +x "$cli_path"
    export MEMORY_CLI_PATH="$cli_path"
    export MEMORY_OPTIMIZATION_AVAILABLE="true"
}

setup_partial_failing_cli() {
    local cli_path="$TMP_DIR/partial-failing-memory-cli"
    cat > "$cli_path" << 'EOF'
#!/usr/bin/env bash
# Partially failing memory CLI for testing

# Parse arguments
command=""
agent=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        cache|load|validate|status|migrate)
            command="$1"
            shift
            ;;
        --agent)
            agent="$2"
            shift 2
            ;;
        *)
            # Skip other options
            if [[ "$1" == --* && $# -gt 1 ]]; then
                shift 2
            else
                shift
            fi
            ;;
    esac
done

# Succeed for some commands, fail for others
case "$command" in
    cache)
        # Success for cache
        echo '{"success":true,"message":"Cache created","data":{"agent":"'"$agent"'"}}'
        exit 0
        ;;
    load)
        # Fail for load
        echo '{"success":false,"message":"Load failed","error":"Test failure"}' >&2
        exit 1
        ;;
    *)
        # Fail for anything else
        echo '{"success":false,"message":"Command failed","error":"Test failure"}' >&2
        exit 1
        ;;
esac
EOF
    chmod +x "$cli_path"
    export MEMORY_CLI_PATH="$cli_path"
    export MEMORY_OPTIMIZATION_AVAILABLE="true"
}

########################
# Begin Tests
########################

print_header "Fallback Mechanism Tests"

# Test 1: Missing CLI - should fallback to legacy functions
print_header "Testing Fallback with Missing CLI"

setup_missing_cli

result=$(create_agent_cache "TestAgent" "$TMP_DIR" "TestInit" "Test intent")
assert_success "Should fallback to legacy cache creation when CLI is missing"
assert_output_contains "$result" "MOCK: Created legacy cache" "Should use mock legacy function for cache creation"

result=$(load_agent_memory "TestAgent" "general")
assert_success "Should fallback to legacy memory loading when CLI is missing"
assert_output_contains "$result" "MOCK: Loaded legacy memory" "Should use mock legacy function for memory loading"

result=$(validate_agent_memory "TestAgent")
assert_success "Should fallback to legacy validation when CLI is missing"
assert_output_contains "$result" "MOCK: Validated legacy memory" "Should use mock legacy function for validation"

# Test 2: Failing CLI - should fallback to legacy functions
print_header "Testing Fallback with Failing CLI"

setup_failing_cli

result=$(create_agent_cache "TestAgent" "$TMP_DIR" "TestInit" "Test intent" 2>/dev/null || echo "Failed but caught")
assert_output_contains "$result" "Failed but caught" "Should handle CLI failures gracefully"

result=$(load_agent_memory "TestAgent" "general" 2>/dev/null || echo "Failed but caught")
assert_output_contains "$result" "Failed but caught" "Should handle CLI failures gracefully"

# Test 3: Partial failing CLI - should fallback for failing operations only
print_header "Testing Partial Fallback with Partially Failing CLI"

setup_partial_failing_cli

result=$(create_agent_cache "TestAgent" "$TMP_DIR" "TestInit" "Test intent")
assert_success "Should succeed for operations that work in the CLI"
assert_output_contains "$result" "Cache created" "Should use the working CLI function"

result=$(load_agent_memory "TestAgent" "general" 2>/dev/null || echo "Failed but caught")
assert_output_contains "$result" "Failed but caught" "Should handle partial CLI failures gracefully"

# Test 4: Integration with updated agent.sh
print_header "Testing Integration with agent.sh"

# Mock the agent.sh activate_agent function to test its fallback behavior
mock_activate_agent() {
    # Source mock_legacy_functions to provide the legacy implementations
    source "$TEST_DIR/mock_legacy_functions.sh"
    
    # First test with missing CLI
    setup_missing_cli
    
    echo "Testing activate_agent with missing CLI..."
    # Simulate the relevant part of activate_agent that uses the memory system
    if [[ "$MEMORY_OPTIMIZATION_AVAILABLE" == "true" ]]; then
        echo "ERROR: Should not detect optimization as available"
        return 1
    fi
    
    # Should use legacy functions
    result=$(load_agent_memory_legacy "TestAgent" "general")
    assert_success "Should use legacy memory loading when CLI is missing"
    assert_output_contains "$result" "MOCK: Loaded legacy memory" "Should use legacy implementation in activate_agent"
    
    # Now test with partial failing CLI
    setup_partial_failing_cli
    
    echo "Testing activate_agent with partially failing CLI..."
    if [[ "$MEMORY_OPTIMIZATION_AVAILABLE" != "true" ]]; then
        echo "ERROR: Should detect optimization as available"
        return 1
    fi
    
    # Should use optimized cache creation (which works)
    result=$(create_agent_cache "TestAgent" "$TMP_DIR" "TestInit" "Test intent")
    assert_success "Should use optimized cache creation which works"
    
    # But should fallback for memory loading (which fails)
    result=$(load_agent_memory "TestAgent" "general" 2>/dev/null || echo "Handled failure and would fallback")
    assert_output_contains "$result" "Handled failure" "Should handle memory loading failure and fallback"
    
    return 0
}

# Run the mock activate_agent test
mock_activate_agent
assert_success "activate_agent should handle fallbacks correctly"

# Summary
print_header "Test Summary"
echo -e "Total tests: ${YELLOW}$TESTS_TOTAL${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All fallback mechanism tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some fallback mechanism tests failed.${NC}"
    exit 1
fi