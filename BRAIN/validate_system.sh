#!/bin/bash

# BRAIN System Validation Script
# CollaborativeIntelligence Multi-Agent System Validator
# Version: 1.0
# Last Updated: 2025-08-30

set -euo pipefail

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BRAIN_DIR="${PROJECT_ROOT}/BRAIN"
AGENTS_DIR="${PROJECT_ROOT}/AGENTS"
INTERFACES_DIR="${PROJECT_ROOT}/interfaces"
CLI_DIR="${PROJECT_ROOT}/cli"

# Test results tracking
TOTAL_TESTS=0
PASSED_TESTS=0
FAILED_TESTS=0
WARNINGS=0

# Logging
LOG_FILE="${SCRIPT_DIR}/validation_$(date +%Y%m%d_%H%M%S).log"
exec 1> >(tee -a "$LOG_FILE")
exec 2> >(tee -a "$LOG_FILE" >&2)

# Helper functions
log_info() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $1"
    ((PASSED_TESTS++))
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $1"
    ((FAILED_TESTS++))
}

log_warning() {
    echo -e "${YELLOW}[WARN]${NC} $1"
    ((WARNINGS++))
}

run_test() {
    local test_name="$1"
    local test_command="$2"
    
    ((TOTAL_TESTS++))
    log_info "Running test: $test_name"
    
    if eval "$test_command" >/dev/null 2>&1; then
        log_success "$test_name"
        return 0
    else
        log_error "$test_name"
        return 1
    fi
}

# Validation functions

validate_environment() {
    log_info "=== Environment Validation ==="
    
    # Check required environment variables
    run_test "CI_DB_PATH environment variable" '[ ! -z "${CI_DB_PATH:-}" ]'
    run_test "CI_REPO_PATH environment variable" '[ ! -z "${CI_REPO_PATH:-}" ]'
    
    # Check required tools
    run_test "Rust toolchain available" 'command -v rustc'
    run_test "Cargo available" 'command -v cargo'
    run_test "SQLite3 available" 'command -v sqlite3'
    run_test "Git available" 'command -v git'
    run_test "jq available" 'command -v jq'
    
    # Check system requirements
    run_test "Minimum disk space (1GB)" '[ $(df "${PROJECT_ROOT}" | tail -1 | awk "{print \$4}") -gt 1048576 ]'
    run_test "Sufficient memory (4GB)" '[ $(free -m | awk "NR==2{printf \"%.0f\", \$7}") -gt 4096 ]'
}

validate_directory_structure() {
    log_info "=== Directory Structure Validation ==="
    
    # Core directories
    run_test "BRAIN directory exists" '[ -d "$BRAIN_DIR" ]'
    run_test "AGENTS directory exists" '[ -d "$AGENTS_DIR" ]'
    run_test "interfaces directory exists" '[ -d "$INTERFACES_DIR" ]'
    run_test "cli directory exists" '[ -d "$CLI_DIR" ]'
    
    # BRAIN subdirectories
    run_test "BRAIN/Core directory exists" '[ -d "$BRAIN_DIR/Core" ]'
    run_test "BRAIN/Architecture directory exists" '[ -d "$BRAIN_DIR/Architecture" ]'
    run_test "BRAIN/Protocols directory exists" '[ -d "$BRAIN_DIR/Protocols" ]'
    run_test "BRAIN/Intake directory exists" '[ -d "$BRAIN_DIR/Intake" ]'
    run_test "BRAIN/Patterns directory exists" '[ -d "$BRAIN_DIR/Patterns" ]'
    run_test "BRAIN/Procedures directory exists" '[ -d "$BRAIN_DIR/Procedures" ]'
    
    # Interface subdirectories
    run_test "interfaces/agent-cache exists" '[ -d "$INTERFACES_DIR/agent-cache" ]'
    run_test "interfaces/db exists" '[ -d "$INTERFACES_DIR/db" ]'
    run_test "interfaces/memory exists" '[ -d "$INTERFACES_DIR/memory" ]'
}

validate_configuration_files() {
    log_info "=== Configuration Files Validation ==="
    
    # Rust configuration files
    run_test "CLI Cargo.toml exists" '[ -f "$CLI_DIR/Cargo.toml" ]'
    run_test "interfaces Cargo.toml exists" '[ -f "$INTERFACES_DIR/Cargo.toml" ]'
    
    # Project configuration
    run_test "Project root Cargo.toml exists" '[ -f "$PROJECT_ROOT/Cargo.toml" ]'
    run_test "Git repository initialized" '[ -d "$PROJECT_ROOT/.git" ]'
    
    # Validate Cargo.toml syntax
    if [ -f "$CLI_DIR/Cargo.toml" ]; then
        run_test "CLI Cargo.toml syntax valid" 'cd "$CLI_DIR" && cargo metadata --no-deps --format-version 1'
    fi
    
    if [ -f "$INTERFACES_DIR/Cargo.toml" ]; then
        run_test "interfaces Cargo.toml syntax valid" 'cd "$INTERFACES_DIR" && cargo metadata --no-deps --format-version 1'
    fi
}

validate_agent_structure() {
    log_info "=== Agent Structure Validation ==="
    
    local agent_count=0
    local valid_agents=0
    
    for agent_dir in "$AGENTS_DIR"/*; do
        if [ -d "$agent_dir" ]; then
            ((agent_count++))
            local agent_name=$(basename "$agent_dir")
            
            # Check required files
            local has_metadata=false
            local has_memory=false
            local has_learning=false
            
            if [ -f "$agent_dir/metadata.json" ]; then
                has_metadata=true
                # Validate JSON syntax
                if jq empty "$agent_dir/metadata.json" 2>/dev/null; then
                    log_success "Agent $agent_name metadata.json syntax valid"
                else
                    log_error "Agent $agent_name metadata.json syntax invalid"
                fi
            else
                log_error "Agent $agent_name missing metadata.json"
            fi
            
            if [ -f "$agent_dir/${agent_name}_memory.md" ] || [ -f "$agent_dir/MEMORY.md" ]; then
                has_memory=true
                log_success "Agent $agent_name has memory file"
            else
                log_warning "Agent $agent_name missing memory file"
            fi
            
            if [ -f "$agent_dir/ContinuousLearning.md" ]; then
                has_learning=true
                log_success "Agent $agent_name has learning file"
            else
                log_warning "Agent $agent_name missing learning file"
            fi
            
            if $has_metadata && $has_memory; then
                ((valid_agents++))
            fi
        fi
    done
    
    log_info "Found $agent_count agents, $valid_agents valid"
    
    if [ $agent_count -gt 50 ]; then
        log_success "Agent count meets system requirements (>50)"
    else
        log_warning "Agent count below recommended threshold ($agent_count/50)"
    fi
}

validate_database_schema() {
    log_info "=== Database Schema Validation ==="
    
    local db_path="${CI_DB_PATH:-${PROJECT_ROOT}/brain.db}"
    
    if [ -f "$db_path" ]; then
        # Check required tables
        run_test "agents table exists" "sqlite3 '$db_path' '.tables' | grep -q agents"
        run_test "sessions table exists" "sqlite3 '$db_path' '.tables' | grep -q sessions"
        run_test "learning_entries table exists" "sqlite3 '$db_path' '.tables' | grep -q learning_entries"
        
        # Check table schemas
        if sqlite3 "$db_path" '.tables' | grep -q agents; then
            run_test "agents table has required columns" "sqlite3 '$db_path' 'PRAGMA table_info(agents)' | grep -E '(name|usage_count|last_used)'"
        fi
        
        # Check database integrity
        run_test "database integrity check" "sqlite3 '$db_path' 'PRAGMA integrity_check' | grep -q OK"
        
        # Check database size
        local db_size=$(stat -f%z "$db_path" 2>/dev/null || stat -c%s "$db_path" 2>/dev/null || echo 0)
        if [ $db_size -gt 1024 ]; then
            log_success "Database has content (${db_size} bytes)"
        else
            log_warning "Database appears empty or very small"
        fi
    else
        log_error "Database file not found at $db_path"
    fi
}

validate_cli_functionality() {
    log_info "=== CLI Functionality Validation ==="
    
    # Build CLI if not already built
    if [ -f "$CLI_DIR/Cargo.toml" ]; then
        log_info "Building CLI tools..."
        if cd "$CLI_DIR" && cargo build --release >/dev/null 2>&1; then
            log_success "CLI build successful"
            
            local ci_binary="$CLI_DIR/target/release/ci"
            local ci_db_binary="$CLI_DIR/target/release/ci-db"
            
            # Test CLI binary
            if [ -f "$ci_binary" ]; then
                run_test "ci binary executable" "[ -x '$ci_binary' ]"
                run_test "ci --help works" "'$ci_binary' --help"
                run_test "ci version command" "'$ci_binary' --version"
            else
                log_error "ci binary not found after build"
            fi
            
            # Test database CLI binary
            if [ -f "$ci_db_binary" ]; then
                run_test "ci-db binary executable" "[ -x '$ci_db_binary' ]"
                run_test "ci-db --help works" "'$ci_db_binary' --help"
            else
                log_error "ci-db binary not found after build"
            fi
        else
            log_error "CLI build failed"
        fi
    else
        log_warning "CLI Cargo.toml not found, skipping build test"
    fi
}

validate_interfaces() {
    log_info "=== Interface System Validation ==="
    
    # Build interfaces if Cargo.toml exists
    if [ -f "$INTERFACES_DIR/Cargo.toml" ]; then
        log_info "Building interface system..."
        if cd "$INTERFACES_DIR" && cargo build --release >/dev/null 2>&1; then
            log_success "Interface system build successful"
        else
            log_error "Interface system build failed"
        fi
    else
        log_warning "Interface Cargo.toml not found"
    fi
    
    # Check interface directories
    for interface in agent-cache db memory; do
        if [ -d "$INTERFACES_DIR/$interface" ]; then
            log_success "Interface $interface directory exists"
            
            # Check for Rust source files
            if find "$INTERFACES_DIR/$interface" -name "*.rs" | head -1 | read; then
                log_success "Interface $interface has Rust source files"
            else
                log_warning "Interface $interface missing Rust source files"
            fi
        else
            log_error "Interface $interface directory missing"
        fi
    done
}

validate_brain_content() {
    log_info "=== BRAIN Content Validation ==="
    
    # Check for core BRAIN files
    run_test "BRAIN README exists" '[ -f "$BRAIN_DIR/README.md" ]'
    run_test "SPECIFICATION.md exists" '[ -f "$BRAIN_DIR/SPECIFICATION.md" ]'
    
    # Check BRAIN content structure
    local core_files=0
    local pattern_files=0
    local procedure_files=0
    
    if [ -d "$BRAIN_DIR/Core" ]; then
        core_files=$(find "$BRAIN_DIR/Core" -name "*.md" | wc -l)
    fi
    
    if [ -d "$BRAIN_DIR/Patterns" ]; then
        pattern_files=$(find "$BRAIN_DIR/Patterns" -name "*.md" | wc -l)
    fi
    
    if [ -d "$BRAIN_DIR/Procedures" ]; then
        procedure_files=$(find "$BRAIN_DIR/Procedures" -name "*.md" | wc -l)
    fi
    
    log_info "BRAIN content: $core_files core files, $pattern_files patterns, $procedure_files procedures"
    
    if [ $core_files -gt 0 ]; then
        log_success "BRAIN has core content"
    else
        log_warning "BRAIN core directory appears empty"
    fi
    
    # Check intake system
    if [ -d "$BRAIN_DIR/Intake" ]; then
        run_test "Intake/Submissions directory exists" '[ -d "$BRAIN_DIR/Intake/Submissions" ]'
        run_test "Intake/Processing directory exists" '[ -d "$BRAIN_DIR/Intake/Processing" ]'
        run_test "Intake/Archive directory exists" '[ -d "$BRAIN_DIR/Intake/Archive" ]'
    fi
}

validate_security() {
    log_info "=== Security Validation ==="
    
    # Check for sensitive files
    run_test "No .env files in git" '! git ls-files | grep -E "\.env$"'
    run_test "No private keys in git" '! git ls-files | grep -E "\.(key|pem)$"'
    run_test "No password files in git" '! git ls-files | grep -i password'
    
    # Check file permissions
    if [ -f "${CI_DB_PATH:-${PROJECT_ROOT}/brain.db}" ]; then
        local db_perms=$(stat -c %a "${CI_DB_PATH:-${PROJECT_ROOT}/brain.db}" 2>/dev/null || stat -f %A "${CI_DB_PATH:-${PROJECT_ROOT}/brain.db}" 2>/dev/null)
        if [[ "$db_perms" =~ ^[6-7][0-4][0-4]$ ]]; then
            log_success "Database file has appropriate permissions"
        else
            log_warning "Database file permissions may be too open ($db_perms)"
        fi
    fi
    
    # Check for hardcoded secrets
    if command -v grep >/dev/null; then
        local secret_patterns=("password" "secret" "key" "token" "api_key")
        for pattern in "${secret_patterns[@]}"; do
            if ! grep -r -i "$pattern" "$PROJECT_ROOT" --include="*.rs" --include="*.toml" --include="*.json" | grep -v "example\|test\|README" >/dev/null; then
                log_success "No hardcoded $pattern found"
            else
                log_warning "Potential hardcoded $pattern found (review manually)"
            fi
        done
    fi
}

validate_performance() {
    log_info "=== Performance Validation ==="
    
    # Check system resources
    local available_memory=$(free -m 2>/dev/null | awk 'NR==2{printf "%.0f", $7}' || echo "unknown")
    local available_disk=$(df "${PROJECT_ROOT}" | tail -1 | awk '{print $4}' || echo "unknown")
    
    log_info "Available memory: ${available_memory}MB"
    log_info "Available disk space: ${available_disk}KB"
    
    # Check project size
    local project_size=$(du -sh "$PROJECT_ROOT" 2>/dev/null | cut -f1 || echo "unknown")
    log_info "Project size: $project_size"
    
    # Check build times (approximate)
    if [ -f "$CLI_DIR/Cargo.toml" ]; then
        log_info "Testing CLI build performance..."
        local start_time=$(date +%s)
        if cd "$CLI_DIR" && cargo check >/dev/null 2>&1; then
            local end_time=$(date +%s)
            local build_time=$((end_time - start_time))
            log_info "CLI check completed in ${build_time}s"
            
            if [ $build_time -lt 30 ]; then
                log_success "CLI build time acceptable (<30s)"
            else
                log_warning "CLI build time may be slow (${build_time}s)"
            fi
        fi
    fi
}

run_integration_tests() {
    log_info "=== Integration Tests ==="
    
    # Test agent metadata parsing
    for agent_dir in "$AGENTS_DIR"/*; do
        if [ -d "$agent_dir" ] && [ -f "$agent_dir/metadata.json" ]; then
            local agent_name=$(basename "$agent_dir")
            if jq -e '.name' "$agent_dir/metadata.json" >/dev/null 2>&1; then
                local json_name=$(jq -r '.name' "$agent_dir/metadata.json")
                if [ "$json_name" = "$agent_name" ]; then
                    log_success "Agent $agent_name metadata name matches directory"
                else
                    log_warning "Agent $agent_name metadata name mismatch: $json_name != $agent_name"
                fi
            else
                log_error "Agent $agent_name metadata missing name field"
            fi
            break # Test only first agent to avoid spam
        fi
    done
    
    # Test database operations (if database exists)
    if [ -f "${CI_DB_PATH:-${PROJECT_ROOT}/brain.db}" ]; then
        local db_path="${CI_DB_PATH:-${PROJECT_ROOT}/brain.db}"
        
        # Test basic queries
        run_test "Can query agents table" "sqlite3 '$db_path' 'SELECT COUNT(*) FROM agents;'"
        run_test "Can query sessions table" "sqlite3 '$db_path' 'SELECT COUNT(*) FROM sessions;'"
        run_test "Can query learning_entries table" "sqlite3 '$db_path' 'SELECT COUNT(*) FROM learning_entries;'"
    fi
    
    # Test file system operations
    local temp_dir=$(mktemp -d)
    run_test "Can create temporary files" "touch '$temp_dir/test_file'"
    run_test "Can write to temporary files" "echo 'test content' > '$temp_dir/test_file'"
    run_test "Can read from temporary files" "[ \"\$(cat '$temp_dir/test_file')\" = 'test content' ]"
    rm -rf "$temp_dir"
}

generate_report() {
    log_info "=== Validation Summary ==="
    
    echo "============================================"
    echo "BRAIN System Validation Report"
    echo "============================================"
    echo "Date: $(date)"
    echo "Project Root: $PROJECT_ROOT"
    echo "Log File: $LOG_FILE"
    echo "============================================"
    echo
    echo "Test Results:"
    echo "  Total Tests: $TOTAL_TESTS"
    echo "  Passed: $PASSED_TESTS"
    echo "  Failed: $FAILED_TESTS"
    echo "  Warnings: $WARNINGS"
    echo
    
    local success_rate=0
    if [ $TOTAL_TESTS -gt 0 ]; then
        success_rate=$(( (PASSED_TESTS * 100) / TOTAL_TESTS ))
    fi
    
    echo "Success Rate: ${success_rate}%"
    echo
    
    if [ $FAILED_TESTS -eq 0 ]; then
        echo -e "${GREEN}✓ VALIDATION PASSED${NC}"
        echo "The BRAIN system appears to be properly configured and functional."
    elif [ $FAILED_TESTS -le 5 ]; then
        echo -e "${YELLOW}⚠ VALIDATION PASSED WITH WARNINGS${NC}"
        echo "The BRAIN system is mostly functional but has some issues that should be addressed."
    else
        echo -e "${RED}✗ VALIDATION FAILED${NC}"
        echo "The BRAIN system has significant issues that prevent proper operation."
    fi
    
    if [ $WARNINGS -gt 0 ]; then
        echo
        echo "Please review warnings in the log file for potential improvements."
    fi
    
    echo
    echo "Detailed results have been logged to: $LOG_FILE"
}

# Main execution
main() {
    echo "BRAIN System Validation Script"
    echo "=============================="
    echo
    
    validate_environment
    validate_directory_structure
    validate_configuration_files
    validate_agent_structure
    validate_database_schema
    validate_cli_functionality
    validate_interfaces
    validate_brain_content
    validate_security
    validate_performance
    run_integration_tests
    
    generate_report
    
    # Exit with appropriate code
    if [ $FAILED_TESTS -eq 0 ]; then
        exit 0
    elif [ $FAILED_TESTS -le 5 ]; then
        exit 1
    else
        exit 2
    fi
}

# Handle command line arguments
case "${1:-}" in
    --help|-h)
        echo "Usage: $0 [OPTIONS]"
        echo
        echo "Options:"
        echo "  --help, -h     Show this help message"
        echo "  --quick        Run only critical tests"
        echo "  --full         Run all tests (default)"
        echo "  --report-only  Generate report from existing log"
        echo
        echo "Environment Variables:"
        echo "  CI_DB_PATH     Path to BRAIN database file"
        echo "  CI_REPO_PATH   Path to CollaborativeIntelligence repository"
        exit 0
        ;;
    --quick)
        echo "Quick validation mode - running critical tests only"
        validate_environment
        validate_directory_structure
        validate_configuration_files
        generate_report
        ;;
    --report-only)
        generate_report
        ;;
    --full|"")
        main
        ;;
    *)
        echo "Unknown option: $1"
        echo "Use --help for usage information"
        exit 1
        ;;
esac