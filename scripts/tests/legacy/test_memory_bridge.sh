#!/usr/bin/env bash
# test_memory_bridge.sh - Test suite for the memory optimization Bash integration components
# 
# This script tests the integration between the Bash scripts and the Rust memory optimization
# components, verifying that:
# 1. The bridge correctly detects whether the memory optimization system is available
# 2. The bridge functions correctly call the underlying Rust components
# 3. Fallback to legacy implementations works as expected
# 4. The integration components handle errors appropriately

set -e

# Setup test environment
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$TEST_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$CI_TOOLS_DIR/.." && pwd)"

# Source the required components
source "$CI_TOOLS_DIR/lib/common.sh"
source "$CI_TOOLS_DIR/lib/rust-memory-bridge.sh"

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

# Create a mock memory CLI for testing
create_mock_cli() {
    local cli_path="$TMP_DIR/memory-cli-mock"
    cat > "$cli_path" << 'EOF'
#!/usr/bin/env bash
# Mock memory CLI for testing

# Parse arguments
format="text"
cache_dir=""
command=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        --format)
            format="$2"
            shift 2
            ;;
        --cache-dir)
            cache_dir="$2"
            shift 2
            ;;
        cache|load|validate|status|migrate)
            command="$1"
            shift
            ;;
        --agent)
            agent="$2"
            shift 2
            ;;
        --context)
            context="$2"
            shift 2
            ;;
        --*)
            # Skip other options
            shift 2
            ;;
        *)
            shift
            ;;
    esac
done

# Mock responses
case "$command" in
    cache)
        echo "{"
        echo "  \"success\": true,"
        echo "  \"message\": \"Agent cache created\","
        echo "  \"data\": {"
        echo "    \"agent\": \"$agent\","
        echo "    \"cached\": true"
        echo "  }"
        echo "}"
        exit 0
        ;;
    load)
        if [[ "$agent" == "ERROR" ]]; then
            echo "{"
            echo "  \"success\": false,"
            echo "  \"message\": \"Failed to load agent memory\","
            echo "  \"error\": \"Test error\""
            echo "}"
            exit 1
        fi
        echo "{"
        echo "  \"success\": true,"
        echo "  \"message\": \"Agent memory loaded\","
        echo "  \"data\": {"
        echo "    \"agent\": \"$agent\","
        echo "    \"context\": \"$context\","
        echo "    \"memory\": \"# Test memory for $agent\\n\\nThis is test memory content.\""
        echo "  }"
        echo "}"
        exit 0
        ;;
    validate)
        echo "{"
        echo "  \"success\": true,"
        echo "  \"message\": \"Agent memory validated\","
        echo "  \"data\": {"
        echo "    \"agent\": \"$agent\","
        echo "    \"valid\": true"
        echo "  }"
        echo "}"
        exit 0
        ;;
    status)
        echo "{"
        echo "  \"success\": true,"
        echo "  \"message\": \"Memory system status\","
        echo "  \"data\": {"
        echo "    \"available\": true,"
        echo "    \"version\": \"1.0.0\","
        echo "    \"agents\": 5,"
        echo "    \"segments\": 20"
        echo "  }"
        echo "}"
        exit 0
        ;;
    migrate)
        echo "{"
        echo "  \"success\": true,"
        echo "  \"message\": \"Migration complete\","
        echo "  \"data\": {"
        echo "    \"migrated\": 5,"
        echo "    \"skipped\": 0,"
        echo "    \"errors\": 0"
        echo "  }"
        echo "}"
        exit 0
        ;;
    *)
        echo "{"
        echo "  \"success\": false,"
        echo "  \"message\": \"Unknown command\","
        echo "  \"error\": \"Invalid command: $command\""
        echo "}"
        exit 1
        ;;
esac
EOF
    chmod +x "$cli_path"
    echo "$cli_path"
}

# Replace the memory CLI path with our mock
setup_memory_cli_mock() {
    export MEMORY_CLI_PATH=$(create_mock_cli)
    export MEMORY_OPTIMIZATION_AVAILABLE="true"
}

# Make the memory CLI unavailable for testing fallback
disable_memory_cli() {
    export MEMORY_CLI_PATH="/invalid/path/memory-cli"
    export MEMORY_OPTIMIZATION_AVAILABLE="false"
}

########################
# Begin Tests
########################

print_header "Memory Optimization Bridge Tests"

# Test 1: Verify memory CLI detection
print_header "Testing Memory CLI Detection"

# When CLI is available
setup_memory_cli_mock
check_memory_cli
assert_success "Should detect mock memory CLI as available"

# When CLI is unavailable
disable_memory_cli
check_memory_cli
assert_failure "Should detect memory CLI as unavailable"

# Test 2: Test create_agent_cache function
print_header "Testing create_agent_cache"

# With CLI available
setup_memory_cli_mock
result=$(create_agent_cache "TestAgent" "$TMP_DIR" "TestInit" "Test intent")
assert_success "Should successfully create agent cache entry"
assert_output_contains "$result" "Agent cache created" "Should return success message"

# With CLI unavailable
disable_memory_cli
create_agent_cache_legacy() {
    echo "Legacy cache created for $1"
    return 0
}
result=$(create_agent_cache "TestAgent" "$TMP_DIR" "TestInit" "Test intent")
assert_success "Should fall back to legacy cache creation"
assert_output_contains "$result" "Legacy cache created" "Should use legacy function"

# Test 3: Test load_agent_memory function
print_header "Testing load_agent_memory"

# With CLI available
setup_memory_cli_mock
result=$(load_agent_memory "TestAgent" "general")
assert_success "Should successfully load agent memory"
assert_output_contains "$result" "Agent memory loaded" "Should return success message"

# With CLI available but agent error
result=$(load_agent_memory "ERROR" "general" 2>&1)
assert_failure "Should fail when agent causes an error"

# With CLI unavailable
disable_memory_cli
load_agent_memory_legacy() {
    echo "Legacy memory loaded for $1"
    return 0
}
result=$(load_agent_memory "TestAgent" "general")
assert_success "Should fall back to legacy memory loading"
assert_output_contains "$result" "Legacy memory loaded" "Should use legacy function"

# Test 4: Test validate_agent_memory function
print_header "Testing validate_agent_memory"

# With CLI available
setup_memory_cli_mock
result=$(validate_agent_memory "TestAgent")
assert_success "Should successfully validate agent memory"
assert_output_contains "$result" "Agent memory validated" "Should return validation message"

# With CLI unavailable
disable_memory_cli
validate_agent_memory_legacy() {
    echo "Legacy validation for $1"
    return 0
}
result=$(validate_agent_memory "TestAgent")
assert_success "Should fall back to legacy validation"
assert_output_contains "$result" "Legacy validation" "Should use legacy function"

# Test 5: Test get_memory_system_status function
print_header "Testing get_memory_system_status"

# With CLI available
setup_memory_cli_mock
result=$(get_memory_system_status)
assert_success "Should successfully get memory system status"
assert_output_contains "$result" "Memory system status" "Should return status information"

# With CLI unavailable
disable_memory_cli
result=$(get_memory_system_status)
assert_success "Should provide status even when CLI is unavailable"
assert_output_contains "$result" "unavailable" "Should indicate memory system is unavailable"

# Summary
print_header "Test Summary"
echo -e "Total tests: ${YELLOW}$TESTS_TOTAL${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some tests failed.${NC}"
    exit 1
fi