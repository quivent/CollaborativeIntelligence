#!/bin/bash

# Topologist Agent Memory Loading Script
# This script loads the complete memory architecture of the Repository Topologist agent
# from the CollaborativeIntelligence project for use with Claude Code

set -e  # Exit on any error

# Configuration
CI_PROJECT_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
TOPOLOGIST_PATH="$CI_PROJECT_PATH/AGENTS/Topologist"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[TOPOLOGIST LOADER]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check if path exists
check_path() {
    local path="$1"
    local description="$2"
    
    if [ ! -e "$path" ]; then
        print_error "$description not found at: $path"
        return 1
    fi
    return 0
}

# Main loading function
load_topologist_memory() {
    print_status "Loading Repository Topologist agent memory..."
    
    # Verify CollaborativeIntelligence project exists
    if ! check_path "$CI_PROJECT_PATH" "CollaborativeIntelligence project"; then
        print_error "CollaborativeIntelligence project not found. Please update CI_PROJECT_PATH in the script."
        exit 1
    fi
    
    # Verify Topologist agent directory exists
    if ! check_path "$TOPOLOGIST_PATH" "Topologist agent directory"; then
        print_error "Topologist agent not found. The agent may have been moved or renamed."
        exit 1
    fi
    
    print_status "Found Topologist agent at: $TOPOLOGIST_PATH"
    
    # Core memory files to load
    local memory_files=(
        "MEMORY.md"
        "ContinuousLearning.md" 
        "README.md"
    )
    
    # Optional context files
    local context_files=(
        "CommitNotification.md"
        "Communication.md"
        "PERSISTENCE_PROTOCOL.md"
        "SEQUENTIAL_COMMIT_PROTOCOL.md"
        "TOOLKIT_INDEX.md"
        "multi_repository_architecture.md"
    )
    
    # JavaScript tools and analyzers
    local js_tools=(
        "initialization.js"
        "initialization_runner.js"
        "repository_change_analyzer.js"
        "sequential_commit_protocol.js"
    )
    
    # Template and support files
    local support_files=(
        "change_report_template.md"
        "Topologist_memory.md"
    )
    
    print_status "=== REPOSITORY TOPOLOGIST AGENT MEMORY LOAD ==="
    echo
    
    # Load core memory files
    for file in "${memory_files[@]}"; do
        local file_path="$TOPOLOGIST_PATH/$file"
        if check_path "$file_path" "$file"; then
            print_status "Loading core memory: $file"
            echo "## === $file ==="
            echo
            cat "$file_path"
            echo
            echo "---"
            echo
        else
            print_warning "Core memory file missing: $file"
        fi
    done
    
    # Load context files if they exist
    print_status "Loading additional context files..."
    for file in "${context_files[@]}"; do
        local file_path="$TOPOLOGIST_PATH/$file"
        if [ -f "$file_path" ]; then
            print_status "Loading context: $file"
            echo "## === $file ==="
            echo
            cat "$file_path"
            echo
            echo "---"
            echo
        fi
    done
    
    # Load JavaScript tools and analyzers (showing first part for context)
    print_status "Loading JavaScript tools and analyzers..."
    for file in "${js_tools[@]}"; do
        local file_path="$TOPOLOGIST_PATH/$file"
        if [ -f "$file_path" ]; then
            print_status "Loading JS tool: $file"
            echo "## === $file (First 100 lines) ==="
            echo
            head -100 "$file_path"
            echo
            echo "... [Full file available at $file_path]"
            echo
            echo "---"
            echo
        fi
    done
    
    # Load support files
    print_status "Loading support files..."
    for file in "${support_files[@]}"; do
        local file_path="$TOPOLOGIST_PATH/$file"
        if [ -f "$file_path" ]; then
            print_status "Loading support: $file"
            echo "## === $file ==="
            echo
            cat "$file_path"
            echo
            echo "---"
            echo
        fi
    done
    
    # Load recent session information
    local sessions_path="$TOPOLOGIST_PATH/Sessions"
    if [ -d "$sessions_path" ]; then
        print_status "Loading recent session context..."
        
        # Find the most recent session file
        local recent_session=$(find "$sessions_path" -name "*.md" -type f -exec ls -t {} + | head -1)
        if [ -n "$recent_session" ]; then
            print_status "Loading most recent session: $(basename "$recent_session")"
            echo "## === RECENT SESSION CONTEXT ==="
            echo "### $(basename "$recent_session")"
            echo
            head -50 "$recent_session"  # Load first 50 lines of most recent session
            echo
            echo "... [truncated for brevity]"
            echo
            echo "---"
            echo
        fi
    fi
    
    # Load Topologist CLI tool
    local topologist_cli_path="$TOPOLOGIST_PATH/tools/topologist-cli"
    if [ -d "$topologist_cli_path" ]; then
        print_status "Loading Topologist CLI tool context..."
        if [ -f "$topologist_cli_path/TOPOLOGIST_CLI_SPECIFICATION.md" ]; then
            echo "## === TOPOLOGIST CLI TOOL ==="
            echo
            head -50 "$topologist_cli_path/TOPOLOGIST_CLI_SPECIFICATION.md"
            echo
            echo "... [Full specification available at $topologist_cli_path/TOPOLOGIST_CLI_SPECIFICATION.md]"
            echo
            echo "---"
            echo
        fi
    fi
    
    # Load CLI integration context
    local cli_path="$CI_PROJECT_PATH/cli"
    if [ -d "$cli_path" ]; then
        print_status "Loading CI CLI integration context..."
        if [ -f "$cli_path/README.md" ]; then
            echo "## === CI CLI INTEGRATION ==="
            echo
            head -100 "$cli_path/README.md"  # Load first 100 lines of CLI documentation
            echo
            echo "... [truncated - full documentation available at $cli_path/README.md]"
            echo
            echo "---"
            echo
        fi
    fi
    
    # Agent activation summary
    print_status "Generating agent activation summary..."
    echo "## === AGENT ACTIVATION SUMMARY ==="
    echo
    echo "**Agent**: Repository Topologist"
    echo "**Role**: Repository structure, git operations, and version control integrity"
    echo "**Repository Scope**: CollaborativeIntelligence ONLY (strict boundary enforcement)"
    echo "**Memory System**: Enhanced learning architecture with three-tier triggers"
    echo "**Critical Protocols**: Mandatory pre-commit checklist, formal acknowledgments"
    echo "**Loading Date**: $(date)"
    echo "**Loaded From**: $TOPOLOGIST_PATH"
    echo
    echo "**Key Capabilities**:"
    echo "- Exclusive commit authority with comprehensive review process"
    echo "- Repository boundary enforcement and isolation protocols"  
    echo "- Branch classification and management (6 categories)"
    echo "- Change impact scoring and risk assessment"
    echo "- Incident-based learning with preventive measures"
    echo "- Cross-repository notification management"
    echo "- Sequential commit protocol with logical grouping"
    echo "- JavaScript-based repository analysis and change tracking"
    echo "- Automated initialization with uncommitted work detection"
    echo "- Specialized Topologist CLI tool for advanced operations"
    echo
    echo "**Current Status**: Ready for repository operations"
    echo "**Integration**: CI CLI available for agent management"
    echo
    
    print_success "Topologist agent memory loaded successfully!"
    print_status "Agent is ready for activation with Claude Code"
}

# Usage information
show_usage() {
    echo "Usage: $0 [OPTIONS]"
    echo
    echo "Load Repository Topologist agent memory for Claude Code integration"
    echo
    echo "Options:"
    echo "  -h, --help     Show this help message"
    echo "  -p, --path     Set custom path to CollaborativeIntelligence project"
    echo "  -q, --quiet    Suppress status messages (output only memory content)"
    echo
    echo "Examples:"
    echo "  $0                           # Load with default paths"
    echo "  $0 -p /custom/path          # Load with custom project path"
    echo "  $0 -q | claude code         # Load quietly and pipe to Claude Code"
}

# Parse command line arguments
QUIET=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -h|--help)
            show_usage
            exit 0
            ;;
        -p|--path)
            CI_PROJECT_PATH="$2"
            TOPOLOGIST_PATH="$CI_PROJECT_PATH/AGENTS/Topologist"
            shift 2
            ;;
        -q|--quiet)
            QUIET=true
            shift
            ;;
        *)
            print_error "Unknown option: $1"
            show_usage
            exit 1
            ;;
    esac
done

# Redirect status messages to stderr if quiet mode
if [ "$QUIET" = true ]; then
    exec 3>&1 1>&2  # Redirect stdout to stderr, save original stdout as fd 3
    load_topologist_memory
    exec 1>&3 3>&-  # Restore stdout
else
    load_topologist_memory
fi