#!/usr/bin/env bash
# run_tests.sh - Main test runner for the memory optimization system
#
# This script runs all the test suites for the memory optimization system:
# 1. Memory CLI tests
# 2. Bash integration tests
# 3. Fallback mechanism tests
# 4. End-to-end integration tests

set -e

# Setup test environment
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$TEST_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$CI_TOOLS_DIR/.." && pwd)"

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test results
TESTS_TOTAL=0
TESTS_PASSED=0
TESTS_FAILED=0

print_header() {
    echo -e "\n${BLUE}====== $1 ======${NC}"
}

run_test_suite() {
    local test_script="$1"
    local description="$2"
    
    print_header "Running $description"
    
    if [ -x "$test_script" ]; then
        if "$test_script"; then
            echo -e "${GREEN}✓ $description passed${NC}"
            TESTS_PASSED=$((TESTS_PASSED + 1))
        else
            echo -e "${RED}✗ $description failed${NC}"
            TESTS_FAILED=$((TESTS_FAILED + 1))
        fi
    else
        echo -e "${YELLOW}! $description skipped (script not executable)${NC}"
        chmod +x "$test_script"
        echo -e "${YELLOW}Made script executable, try again.${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
}

# Make sure test scripts are executable
chmod +x "$TEST_DIR"/*.sh

# Run test suites
run_test_suite "$TEST_DIR/test_memory_cli.sh" "Memory CLI Tests"
run_test_suite "$TEST_DIR/test_memory_bridge.sh" "Memory Bridge Tests"
run_test_suite "$TEST_DIR/test_fallback_mechanisms.sh" "Fallback Mechanism Tests"

# Summary
print_header "Test Summary"
echo -e "Total test suites: ${YELLOW}$TESTS_TOTAL${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All test suites passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some test suites failed.${NC}"
    exit 1
fi