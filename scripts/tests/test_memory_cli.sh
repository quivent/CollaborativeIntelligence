#!/usr/bin/env bash
# test_memory_cli.sh - Test suite for memory CLI
# Part of the Memory Optimization Integration Plan

set -euo pipefail

# Determine script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SCRIPTS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CI_ROOT_DIR="$(cd "$SCRIPTS_DIR/.." && pwd)"

# Set paths
MEMORY_CLI_PATH="${CI_ROOT_DIR}/target/release/memory_cli"
TEST_CACHE_DIR="${SCRIPT_DIR}/test_cache"

# Create test directories
mkdir -p "${TEST_CACHE_DIR}/AGENTS/TestAgent/Sessions"
mkdir -p "${TEST_CACHE_DIR}/CACHE/SESSIONS"
mkdir -p "${TEST_CACHE_DIR}/CACHE/CHECKSUMS"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

# Test counter
TESTS_TOTAL=0
TESTS_PASSED=0
TESTS_FAILED=0

# Test function
run_test() {
    local test_name="$1"
    local command="$2"
    local expected_status="$3"
    
    echo -e "\n${YELLOW}Running test: ${test_name}${NC}"
    echo "Command: $command"
    
    # Run the command
    set +e
    eval "$command"
    local status=$?
    set -e
    
    # Check status
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    if [[ $status -eq $expected_status ]]; then
        TESTS_PASSED=$((TESTS_PASSED + 1))
        echo -e "${GREEN}✓ Test passed${NC}"
    else
        TESTS_FAILED=$((TESTS_FAILED + 1))
        echo -e "${RED}✗ Test failed - Expected status: $expected_status, Got: $status${NC}"
    fi
}

# Helper function to create test files
create_test_memory_files() {
    # Create test MEMORY.md
    cat > "${TEST_CACHE_DIR}/AGENTS/TestAgent/MEMORY.md" << EOF
# TestAgent Memory

This is a test memory file for TestAgent.

## Core Knowledge

- Agent Name: TestAgent
- Creation Date: $(date)
- Primary Function: Testing the memory CLI

## System Integration

The TestAgent is part of the testing framework.

## Expertise

- Memory testing
- CLI validation
- Integration testing
EOF

    # Create test ContinuousLearning.md
    cat > "${TEST_CACHE_DIR}/AGENTS/TestAgent/ContinuousLearning.md" << EOF
# TestAgent Continuous Learning

This file documents the learning progress of TestAgent.

## Learning Record

### $(date +"%Y-%m-%d")

- CLI test created
- Memory validation tested
- Cache operations validated
EOF
}

# Check if memory CLI is available, build if not
if [[ ! -f "$MEMORY_CLI_PATH" ]]; then
    echo "Memory CLI not found. Building..."
    cd "$SCRIPTS_DIR"
    ./build_memory_cli.sh
    
    if [[ ! -f "$MEMORY_CLI_PATH" ]]; then
        echo -e "${RED}ERROR: Failed to build memory CLI.${NC}"
        exit 1
    fi
fi

# Create test files
echo "Setting up test environment..."
create_test_memory_files

# Run tests

# Test 1: Check command help
run_test "CLI Help" \
    "$MEMORY_CLI_PATH --help" \
    0

# Test 2: Create Agent Cache
run_test "Create Agent Cache" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' cache --agent TestAgent --directory '/test/dir' --initializer 'Test' --intent 'Testing cache creation'" \
    0

# Test 3: Check status
run_test "Check Status" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' status" \
    0

# Test 4: Load agent memory
run_test "Load Agent Memory" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' load --agent TestAgent" \
    0

# Test 5: Load agent memory with context
run_test "Load Agent Memory with Context" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' load --agent TestAgent --context 'test'" \
    0

# Test 6: Validate agent memory
run_test "Validate Agent Memory" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' validate --agent TestAgent" \
    0

# Test 7: Create agent cache with JSON output
run_test "Create Agent Cache with JSON Output" \
    "$MEMORY_CLI_PATH --format json --cache-dir '$TEST_CACHE_DIR' cache --agent TestAgent --directory '/test/dir' --initializer 'Test' --intent 'Testing JSON output'" \
    0

# Test 8: Create agent cache with text output
run_test "Create Agent Cache with Text Output" \
    "$MEMORY_CLI_PATH --format text --cache-dir '$TEST_CACHE_DIR' cache --agent TestAgent --directory '/test/dir' --initializer 'Test' --intent 'Testing text output'" \
    0

# Test 9: Create agent cache with markdown output
run_test "Create Agent Cache with Markdown Output" \
    "$MEMORY_CLI_PATH --format markdown --cache-dir '$TEST_CACHE_DIR' cache --agent TestAgent --directory '/test/dir' --initializer 'Test' --intent 'Testing markdown output'" \
    0

# Test 10: Test missing required argument
run_test "Missing Required Argument" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' cache --agent TestAgent --directory '/test/dir' --initializer 'Test'" \
    1

# Test 11: Test invalid format
run_test "Invalid Format" \
    "$MEMORY_CLI_PATH --format invalid --cache-dir '$TEST_CACHE_DIR' status" \
    1

# Test 12: Test invalid agent
run_test "Invalid Agent" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' load --agent NonExistentAgent" \
    1

# Test 13: Test migration
run_test "Test Migration" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' migrate" \
    0

# Test 14: Test forced migration
run_test "Test Forced Migration" \
    "$MEMORY_CLI_PATH --cache-dir '$TEST_CACHE_DIR' migrate --force" \
    0

# Test with bash bridge
# Source the bridge
source "${CI_ROOT_DIR}/ci-tools/lib/rust-memory-bridge.sh"

# Set cache directory for the bridge
CACHE_DIR="$TEST_CACHE_DIR"

# Test 15: Create agent cache with bridge
run_test "Create Agent Cache with Bridge" \
    "create_agent_cache 'TestAgent' '/test/dir' 'Test' 'Testing bridge'" \
    0

# Test 16: Load agent memory with bridge
run_test "Load Agent Memory with Bridge" \
    "load_agent_memory 'TestAgent'" \
    0

# Test 17: Validate agent memory with bridge
run_test "Validate Agent Memory with Bridge" \
    "validate_agent_memory 'TestAgent'" \
    0

# Test 18: Get agent status with bridge
run_test "Get Agent Status with Bridge" \
    "get_agent_status" \
    0

# Print summary
echo -e "\n${YELLOW}=== TEST SUMMARY ===${NC}"
echo -e "${GREEN}Tests passed: $TESTS_PASSED${NC}"
echo -e "${RED}Tests failed: $TESTS_FAILED${NC}"
echo -e "Total tests: $TESTS_TOTAL"

# Cleanup
echo -e "\n${YELLOW}Cleaning up test environment...${NC}"
rm -rf "$TEST_CACHE_DIR"

# Exit with status
if [[ $TESTS_FAILED -eq 0 ]]; then
    echo -e "${GREEN}All tests passed!${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed.${NC}"
    exit 1
fi