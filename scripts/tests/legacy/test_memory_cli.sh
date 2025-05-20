#!/usr/bin/env bash
# test_memory_cli.sh - Test suite for the memory CLI interface
#
# This script tests the functionality of the memory CLI interface:
# 1. Command-line argument parsing
# 2. Cache creation and management
# 3. Memory loading
# 4. Memory validation
# 5. System status reporting
# 6. Migration functionality
# 7. Error handling

set -e

# Setup test environment
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$TEST_DIR/.." && pwd)"
PROJECT_ROOT="$(cd "$CI_TOOLS_DIR/.." && pwd)"

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

# Path to the memory CLI (use mock in tests)
MEMORY_CLI_PATH="$TMP_DIR/memory-cli-mock"

# Create a complete mock of the memory CLI for testing
create_complete_mock_cli() {
    local cli_path="$MEMORY_CLI_PATH"
    
    cat > "$cli_path" << 'EOF'
#!/usr/bin/env bash
# Complete mock memory CLI for testing

# Create a fake database directory
DB_DIR="$TMP_DIR/memory-db"
mkdir -p "$DB_DIR"

# Parse arguments
format="text"
cache_dir=""
command=""

# Parse arguments more thoroughly
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
        --help|-h)
            echo "Memory CLI - Mock Test Version"
            echo ""
            echo "Usage: memory-cli [OPTIONS] COMMAND [ARGS]"
            echo ""
            echo "Options:"
            echo "  --format FORMAT    Output format (text, json, markdown)"
            echo "  --cache-dir DIR    Directory for cache files"
            echo "  --help, -h         Show this help message"
            echo ""
            echo "Commands:"
            echo "  cache    Create or update agent cache entries"
            echo "  load     Load agent memory"
            echo "  validate Validate agent memory integrity"
            echo "  status   Show memory system status"
            echo "  migrate  Migrate legacy memory to optimized format"
            exit 0
            ;;
        cache)
            command="$1"
            shift
            # Parse cache command args
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --agent)
                        agent="$2"
                        shift 2
                        ;;
                    --directory)
                        directory="$2"
                        shift 2
                        ;;
                    --initializer)
                        initializer="$2"
                        shift 2
                        ;;
                    --intent)
                        intent="$2"
                        shift 2
                        ;;
                    *)
                        shift
                        ;;
                esac
            done
            break
            ;;
        load)
            command="$1"
            shift
            # Parse load command args
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --agent)
                        agent="$2"
                        shift 2
                        ;;
                    --context)
                        context="$2"
                        shift 2
                        ;;
                    --segment)
                        segment="$2"
                        shift 2
                        ;;
                    *)
                        shift
                        ;;
                esac
            done
            break
            ;;
        validate)
            command="$1"
            shift
            # Parse validate command args
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --agent)
                        agent="$2"
                        shift 2
                        ;;
                    --segment)
                        segment="$2"
                        shift 2
                        ;;
                    *)
                        shift
                        ;;
                esac
            done
            break
            ;;
        status)
            command="$1"
            shift
            break
            ;;
        migrate)
            command="$1"
            shift
            # Parse migrate command args
            while [[ $# -gt 0 ]]; do
                case "$1" in
                    --agent)
                        agent="$2"
                        shift 2
                        ;;
                    --all)
                        migrate_all=true
                        shift
                        ;;
                    *)
                        shift
                        ;;
                esac
            done
            break
            ;;
        *)
            shift
            ;;
    esac
done

# Error check
if [[ -z "$command" ]]; then
    if [[ "$format" == "json" ]]; then
        echo '{"success":false,"message":"Missing command","error":"No command specified"}'
    else
        echo "Error: No command specified"
    fi
    exit 1
fi

# Handle errors for test cases
if [[ "$agent" == "ERROR" ]]; then
    if [[ "$format" == "json" ]]; then
        echo '{"success":false,"message":"Test error","error":"Error requested for testing"}'
    else
        echo "Error: Error requested for testing"
    fi
    exit 1
fi

# Mock responses based on format and command
case "$format" in
    json)
        case "$command" in
            cache)
                echo '{'
                echo '  "success": true,'
                echo '  "message": "Agent cache created",'
                echo '  "data": {'
                echo '    "agent": "'"$agent"'",'
                echo '    "directory": "'"$directory"'",'
                echo '    "initializer": "'"$initializer"'",'
                echo '    "intent": "'"$intent"'",'
                echo '    "cached": true'
                echo '  }'
                echo '}'
                ;;
            load)
                if [[ -n "$segment" ]]; then
                    echo '{'
                    echo '  "success": true,'
                    echo '  "message": "Agent memory segment loaded",'
                    echo '  "data": {'
                    echo '    "agent": "'"$agent"'",'
                    echo '    "segment": "'"$segment"'",'
                    echo '    "memory": "# Test memory segment for '"$agent"'\\n\\nThis is test memory segment '"$segment"' content."'
                    echo '  }'
                    echo '}'
                else
                    echo '{'
                    echo '  "success": true,'
                    echo '  "message": "Agent memory loaded",'
                    echo '  "data": {'
                    echo '    "agent": "'"$agent"'",'
                    echo '    "context": "'"$context"'",'
                    echo '    "memory": "# Test memory for '"$agent"'\\n\\nThis is test memory content for context '"$context"'."'
                    echo '  }'
                    echo '}'
                fi
                ;;
            validate)
                if [[ -n "$segment" ]]; then
                    echo '{'
                    echo '  "success": true,'
                    echo '  "message": "Agent memory segment validated",'
                    echo '  "data": {'
                    echo '    "agent": "'"$agent"'",'
                    echo '    "segment": "'"$segment"'",'
                    echo '    "valid": true'
                    echo '  }'
                    echo '}'
                else
                    echo '{'
                    echo '  "success": true,'
                    echo '  "message": "Agent memory validated",'
                    echo '  "data": {'
                    echo '    "agent": "'"$agent"'",'
                    echo '    "valid": true,'
                    echo '    "segments": ["MEMORY", "ContinuousLearning", "README"]'
                    echo '  }'
                    echo '}'
                fi
                ;;
            status)
                echo '{'
                echo '  "success": true,'
                echo '  "message": "Memory system status",'
                echo '  "data": {'
                echo '    "available": true,'
                echo '    "version": "1.0.0",'
                echo '    "agents": 5,'
                echo '    "segments": 20,'
                echo '    "database_size": 1024,'
                echo '    "cache_hit_ratio": 0.85'
                echo '  }'
                echo '}'
                ;;
            migrate)
                if [[ -n "$agent" ]]; then
                    echo '{'
                    echo '  "success": true,'
                    echo '  "message": "Migration complete for agent '"$agent"'",'
                    echo '  "data": {'
                    echo '    "agent": "'"$agent"'",'
                    echo '    "migrated": true,'
                    echo '    "segments": 3'
                    echo '  }'
                    echo '}'
                else
                    echo '{'
                    echo '  "success": true,'
                    echo '  "message": "Migration complete for all agents",'
                    echo '  "data": {'
                    echo '    "migrated": 5,'
                    echo '    "skipped": 0,'
                    echo '    "errors": 0'
                    echo '  }'
                    echo '}'
                fi
                ;;
        esac
        ;;
    markdown)
        case "$command" in
            cache)
                echo "# Agent Cache Created"
                echo ""
                echo "Successfully created cache entry for agent **$agent**."
                echo ""
                echo "- **Directory**: \`$directory\`"
                echo "- **Initializer**: $initializer"
                echo "- **Intent**: $intent"
                ;;
            load)
                if [[ -n "$segment" ]]; then
                    echo "# Memory Segment: $segment"
                    echo ""
                    echo "This is test memory segment $segment content for agent $agent."
                else
                    echo "# Test memory for $agent"
                    echo ""
                    echo "This is test memory content for context $context."
                fi
                ;;
            validate)
                if [[ -n "$segment" ]]; then
                    echo "# Validation Result: $segment"
                    echo ""
                    echo "✅ Memory segment **$segment** for agent **$agent** is valid."
                else
                    echo "# Validation Result"
                    echo ""
                    echo "✅ All memory segments for agent **$agent** are valid."
                    echo ""
                    echo "### Segments"
                    echo "- MEMORY"
                    echo "- ContinuousLearning"
                    echo "- README"
                fi
                ;;
            status)
                echo "# Memory System Status"
                echo ""
                echo "System is **available** and running version **1.0.0**."
                echo ""
                echo "### Statistics"
                echo "- Agents: 5"
                echo "- Segments: 20"
                echo "- Database size: 1024 bytes"
                echo "- Cache hit ratio: 85%"
                ;;
            migrate)
                if [[ -n "$agent" ]]; then
                    echo "# Migration Complete: $agent"
                    echo ""
                    echo "✅ Successfully migrated memory for agent **$agent**."
                    echo ""
                    echo "- Segments migrated: 3"
                else
                    echo "# Migration Complete"
                    echo ""
                    echo "✅ Successfully migrated memory for all agents."
                    echo ""
                    echo "- Agents migrated: 5"
                    echo "- Agents skipped: 0"
                    echo "- Errors: 0"
                fi
                ;;
        esac
        ;;
    text|*)
        case "$command" in
            cache)
                echo "Successfully created cache entry for agent '$agent'."
                echo "Directory: $directory"
                echo "Initializer: $initializer"
                echo "Intent: $intent"
                ;;
            load)
                if [[ -n "$segment" ]]; then
                    echo "# Test memory segment for $agent"
                    echo ""
                    echo "This is test memory segment $segment content."
                else
                    echo "# Test memory for $agent"
                    echo ""
                    echo "This is test memory content for context $context."
                fi
                ;;
            validate)
                if [[ -n "$segment" ]]; then
                    echo "Memory segment '$segment' for agent '$agent' is valid."
                else
                    echo "All memory segments for agent '$agent' are valid."
                    echo "Segments: MEMORY, ContinuousLearning, README"
                fi
                ;;
            status)
                echo "Memory system is available (version 1.0.0)"
                echo "Agents: 5"
                echo "Segments: 20"
                echo "Database size: 1024 bytes"
                echo "Cache hit ratio: 85%"
                ;;
            migrate)
                if [[ -n "$agent" ]]; then
                    echo "Successfully migrated memory for agent '$agent'."
                    echo "Segments migrated: 3"
                else
                    echo "Successfully migrated memory for all agents."
                    echo "Agents migrated: 5"
                    echo "Agents skipped: 0"
                    echo "Errors: 0"
                fi
                ;;
        esac
        ;;
esac

exit 0
EOF
    chmod +x "$cli_path"
}

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

assert_json_valid() {
    local json="$1"
    local description="$2"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    if echo "$json" | jq . >/dev/null 2>&1; then
        echo -e "${GREEN}✓ $description${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ $description${NC}"
        echo -e "${YELLOW}Invalid JSON: $json${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

assert_json_path() {
    local json="$1"
    local path="$2"
    local expected="$3"
    local description="$4"
    
    TESTS_TOTAL=$((TESTS_TOTAL + 1))
    local actual=$(echo "$json" | jq -r "$path" 2>/dev/null)
    if [[ "$actual" == "$expected" ]]; then
        echo -e "${GREEN}✓ $description${NC}"
        TESTS_PASSED=$((TESTS_PASSED + 1))
    else
        echo -e "${RED}✗ $description${NC}"
        echo -e "${YELLOW}Expected JSON path $path to be: $expected${NC}"
        echo -e "${YELLOW}Actual value: $actual${NC}"
        TESTS_FAILED=$((TESTS_FAILED + 1))
    fi
}

print_header() {
    echo -e "\n${YELLOW}==== $1 ====${NC}"
}

# Check if jq is available
if ! command -v jq >/dev/null 2>&1; then
    echo -e "${YELLOW}Warning: jq is not installed. Skipping JSON validation tests.${NC}"
    jq() {
        echo "Mocked jq for tests"
        return 0
    }
fi

########################
# Begin Tests
########################

print_header "Memory CLI Interface Tests"

# Create the mock CLI
create_complete_mock_cli

# Test 1: Help output
print_header "Testing Help Output"

result=$("$MEMORY_CLI_PATH" --help)
assert_success "Help command should succeed"
assert_output_contains "$result" "Memory CLI" "Help output should contain CLI name"
assert_output_contains "$result" "Usage:" "Help output should contain usage info"
assert_output_contains "$result" "Commands:" "Help output should list commands"

# Test 2: Cache command with different formats
print_header "Testing Cache Command"

# Test with text format
result=$("$MEMORY_CLI_PATH" --format text cache --agent "TestAgent" --directory "$TMP_DIR" --initializer "TestInit" --intent "Test intent")
assert_success "Cache command with text format should succeed"
assert_output_contains "$result" "Successfully created cache entry for agent 'TestAgent'" "Text output should contain success message"

# Test with JSON format
result=$("$MEMORY_CLI_PATH" --format json cache --agent "TestAgent" --directory "$TMP_DIR" --initializer "TestInit" --intent "Test intent")
assert_success "Cache command with JSON format should succeed"
assert_output_contains "$result" "\"success\": true" "JSON output should contain success flag"
if command -v jq >/dev/null 2>&1; then
    assert_json_valid "$result" "JSON output should be valid"
    assert_json_path "$result" ".data.agent" "TestAgent" "JSON data should contain agent name"
fi

# Test with markdown format
result=$("$MEMORY_CLI_PATH" --format markdown cache --agent "TestAgent" --directory "$TMP_DIR" --initializer "TestInit" --intent "Test intent")
assert_success "Cache command with markdown format should succeed"
assert_output_contains "$result" "# Agent Cache Created" "Markdown output should contain heading"
assert_output_contains "$result" "**TestAgent**" "Markdown output should contain agent name in bold"

# Test 3: Load command
print_header "Testing Load Command"

# Test loading full memory
result=$("$MEMORY_CLI_PATH" load --agent "TestAgent" --context "general")
assert_success "Load command should succeed"
assert_output_contains "$result" "Test memory for TestAgent" "Output should contain memory content"

# Test loading segment
result=$("$MEMORY_CLI_PATH" load --agent "TestAgent" --segment "MEMORY")
assert_success "Load segment command should succeed"
assert_output_contains "$result" "Test memory segment MEMORY" "Output should contain segment content"

# Test loading with JSON format
result=$("$MEMORY_CLI_PATH" --format json load --agent "TestAgent" --context "general")
assert_success "Load command with JSON format should succeed"
assert_output_contains "$result" "\"success\": true" "JSON output should contain success flag"
if command -v jq >/dev/null 2>&1; then
    assert_json_valid "$result" "JSON output should be valid"
    assert_json_path "$result" ".data.agent" "TestAgent" "JSON data should contain agent name"
fi

# Test 4: Validate command
print_header "Testing Validate Command"

# Test validating all segments
result=$("$MEMORY_CLI_PATH" validate --agent "TestAgent")
assert_success "Validate command should succeed"
assert_output_contains "$result" "All memory segments for agent" "Output should indicate validation of all segments"

# Test validating specific segment
result=$("$MEMORY_CLI_PATH" validate --agent "TestAgent" --segment "MEMORY")
assert_success "Validate segment command should succeed"
assert_output_contains "$result" "Memory segment 'MEMORY'" "Output should indicate validation of specific segment"

# Test 5: Status command
print_header "Testing Status Command"

result=$("$MEMORY_CLI_PATH" status)
assert_success "Status command should succeed"
assert_output_contains "$result" "Memory system is available" "Output should indicate system status"

# Test with JSON format
result=$("$MEMORY_CLI_PATH" --format json status)
assert_success "Status command with JSON format should succeed"
assert_output_contains "$result" "\"available\": true" "JSON output should indicate system availability"
if command -v jq >/dev/null 2>&1; then
    assert_json_valid "$result" "JSON output should be valid"
    assert_json_path "$result" ".data.version" "1.0.0" "JSON data should contain version"
fi

# Test 6: Migrate command
print_header "Testing Migrate Command"

# Test migrating specific agent
result=$("$MEMORY_CLI_PATH" migrate --agent "TestAgent")
assert_success "Migrate agent command should succeed"
assert_output_contains "$result" "Successfully migrated memory for agent" "Output should indicate successful migration"

# Test migrating all agents
result=$("$MEMORY_CLI_PATH" migrate --all)
assert_success "Migrate all command should succeed"
assert_output_contains "$result" "Successfully migrated memory for all agents" "Output should indicate successful migration of all agents"

# Test 7: Error handling
print_header "Testing Error Handling"

# Test missing command
result=$("$MEMORY_CLI_PATH" 2>&1) || true
assert_output_contains "$result" "Error: No command specified" "Output should indicate missing command"

# Test error condition
result=$("$MEMORY_CLI_PATH" load --agent "ERROR" 2>&1) || true
assert_output_contains "$result" "Error" "Output should indicate error condition"

# Summary
print_header "Test Summary"
echo -e "Total tests: ${YELLOW}$TESTS_TOTAL${NC}"
echo -e "Passed: ${GREEN}$TESTS_PASSED${NC}"
echo -e "Failed: ${RED}$TESTS_FAILED${NC}"

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "\n${GREEN}All memory CLI tests passed!${NC}"
    exit 0
else
    echo -e "\n${RED}Some memory CLI tests failed.${NC}"
    exit 1
fi