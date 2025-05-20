#!/usr/bin/env bash
# rust-memory-bridge.sh - Bridge between bash scripts and Rust memory optimization system
# Part of the Memory Optimization Integration Plan

# Determine script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CI_ROOT_DIR="$(cd "$CI_TOOLS_DIR/.." && pwd)"

# Import common library
source "$SCRIPT_DIR/common.sh"

# Set default paths
MEMORY_CLI_PATH="$CI_ROOT_DIR/target/release/memory_cli"
CACHE_DIR="$CI_ROOT_DIR"
DB_PATH="$CI_ROOT_DIR/memory.db.json"
AGENTS_DIR="$CI_ROOT_DIR/AGENTS"

# Default to dual-mode operation (uses both legacy and optimized storage)
MEMORY_DUAL_MODE=${MEMORY_DUAL_MODE:-true}

# Default segmentation settings
MEMORY_SEGMENTATION_ENABLED=${MEMORY_SEGMENTATION_ENABLED:-true}
MEMORY_SEGMENT_SIZE=${MEMORY_SEGMENT_SIZE:-4096}
MEMORY_SEGMENT_OVERLAP=${MEMORY_SEGMENT_OVERLAP:-256}

# Debug logging
debug_log() {
  if [[ "${MEMORY_BRIDGE_DEBUG:-false}" == "true" ]]; then
    echo "[Memory Bridge Debug] $1" >&2
  fi
}

# Error logging with better error handling
error_log() {
  local error_code=$1
  local error_message=$2
  
  # Log the error
  echo "[Memory Bridge Error] ($error_code) $error_message" >&2
  
  # Return the error code
  return $error_code
}

# Check if memory CLI is available
check_memory_cli() {
  if [[ ! -f "$MEMORY_CLI_PATH" ]]; then
    # Check for debug build if release not found
    if [[ -f "$CI_ROOT_DIR/target/debug/memory_cli" ]]; then
      MEMORY_CLI_PATH="$CI_ROOT_DIR/target/debug/memory_cli"
      print_warning "Using debug build of memory CLI"
    else
      print_error "Memory CLI not found at $MEMORY_CLI_PATH"
      print_info "Run 'cargo build --release' in scripts directory to build it"
      return 1
    fi
  fi
  return 0
}

# Check if database exists and is accessible
check_database() {
  if [[ ! -f "$DB_PATH" ]]; then
    debug_log "Database not found at $DB_PATH"
    return 1
  fi
  
  # Try to read database
  if ! "$MEMORY_CLI_PATH" --db-path "$DB_PATH" --format json status &>/dev/null; then
    debug_log "Database exists but cannot be accessed"
    return 1
  fi
  
  return 0
}

# Get memory system status
get_memory_system_status() {
  local format="${1:-json}"
  
  # Check for memory CLI
  if ! check_memory_cli; then
    debug_log "Memory CLI not available for status check"
    
    # Return formatted status indicating system is unavailable
    case "$format" in
      json)
        echo '{"success":true,"message":"Memory system status","data":{"available":false,"reason":"CLI not found"}}'
        ;;
      markdown)
        echo "# Memory System Status"
        echo ""
        echo "System is **unavailable**."
        echo ""
        echo "Reason: Memory CLI not found"
        ;;
      *)
        echo "Memory system is unavailable (CLI not found)"
        ;;
    esac
    
    return 0
  fi
  
  # Check database in DB mode
  local db_available=false
  if check_database; then
    db_available=true
  fi
  
  # If database is available and we're not in dual mode, use DB operations
  if [[ "$db_available" == "true" && "$MEMORY_DUAL_MODE" != "true" ]]; then
    debug_log "Using database mode for status"
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" status
    return $?
  fi
  
  # Otherwise use legacy CLI status
  debug_log "Using legacy mode for status"
  "$MEMORY_CLI_PATH" --format "$format" --cache-dir "$CACHE_DIR" status
  
  return $?
}

# Create an agent cache entry
create_agent_cache() {
  local agent_name="$1"
  local directory="$2"
  local initializer="$3"
  local intent="$4"
  local format="${5:-json}"
  
  # Validate required arguments
  if [[ -z "$agent_name" || -z "$directory" || -z "$initializer" ]]; then
    print_error "Missing required arguments for create_agent_cache"
    print_info "Usage: create_agent_cache <agent_name> <directory> <initializer> <intent> [format]"
    return 1
  fi
  
  debug_log "Creating agent cache for $agent_name with initializer $initializer"
  
  # Check for memory CLI
  if ! check_memory_cli; then
    # Fall back to legacy implementation if CLI not available
    debug_log "Memory CLI not available, falling back to legacy agent cache implementation"
    create_agent_cache_legacy "$agent_name" "$directory" "$initializer" "$intent"
    return $?
  fi
  
  # Check database in DB mode
  local db_available=false
  if check_database; then
    db_available=true
  fi
  
  # Dual-mode: Create cache in both legacy and DB if available
  if [[ "$MEMORY_DUAL_MODE" == "true" ]]; then
    debug_log "Operating in dual-mode for agent cache creation"
    
    # Always try to create legacy cache
    "$MEMORY_CLI_PATH" --format "$format" --cache-dir "$CACHE_DIR" cache \
      --agent "$agent_name" \
      --directory "$directory" \
      --initializer "$initializer" \
      --intent "$intent"
    
    local legacy_result=$?
    
    # If DB is available, also create DB cache
    if [[ "$db_available" == "true" ]]; then
      debug_log "Creating DB cache for $agent_name"
      "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" init-agent \
        --agent "$agent_name" \
        --directory "$directory" \
        --initializer "$initializer" \
        --intent "$intent" >/dev/null
    fi
    
    return $legacy_result
  fi
  
  # DB-only mode
  if [[ "$db_available" == "true" && "$MEMORY_DUAL_MODE" != "true" ]]; then
    debug_log "Operating in DB-only mode for agent cache creation"
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" init-agent \
      --agent "$agent_name" \
      --directory "$directory" \
      --initializer "$initializer" \
      --intent "$intent"
    
    return $?
  fi
  
  # Legacy-only mode
  debug_log "Operating in legacy-only mode for agent cache creation"
  "$MEMORY_CLI_PATH" --format "$format" --cache-dir "$CACHE_DIR" cache \
    --agent "$agent_name" \
    --directory "$directory" \
    --initializer "$initializer" \
    --intent "$intent"
  
  return $?
}

# Create an agent cache entry using legacy implementation
create_agent_cache_legacy() {
  local agent_name="$1"
  local directory="$2"
  local initializer="$3"
  local intent="$4"
  
  debug_log "Using legacy implementation to create cache for $agent_name"
  
  # Legacy implementation code (fallback)
  local timestamp=$(date +%s)
  local cache_dir="$CACHE_DIR/CACHE/SESSIONS"
  local cache_file="$cache_dir/${agent_name}_${directory//\//_}_${initializer}_$timestamp.md"
  
  # Create cache directory if it doesn't exist
  mkdir -p "$cache_dir"
  
  # Create cache file with markdown format
  cat > "$cache_file" << EOF
# Agent Activation Cache

## Metadata
- **Agent:** $agent_name
- **Working Directory:** $directory
- **Initializer:** $initializer
- **Timestamp:** $(date)

## Intent
$intent

## Status
Active

This file was automatically generated by the Agent Cache System and should be immediately integrated into the agent's short-term memory upon activation.
EOF
  
  # Update terminal title
  echo -ne "\033]0;Agent: $agent_name\007"
  
  return 0
}

# Load agent memory
load_agent_memory() {
  local agent_name="$1"
  local context="$2"
  local format="${3:-markdown}"
  local segment_id="$4"
  
  # Validate required arguments
  if [[ -z "$agent_name" ]]; then
    print_error "Missing required arguments for load_agent_memory"
    print_info "Usage: load_agent_memory <agent_name> [context] [format] [segment_id]"
    return 1
  fi
  
  debug_log "Loading memory for agent $agent_name with context $context, segment $segment_id"
  
  # Check for memory CLI
  if ! check_memory_cli; then
    # Fall back to legacy implementation if CLI not available
    debug_log "Memory CLI not available, falling back to legacy memory loading implementation"
    load_agent_memory_legacy "$agent_name" "$context"
    return $?
  fi
  
  # Check database in DB mode
  local db_available=false
  if check_database; then
    db_available=true
  fi
  
  # Prepare segmentation parameters
  local segmentation_args=""
  if [[ "$MEMORY_SEGMENTATION_ENABLED" == "true" ]]; then
    segmentation_args="--segmentation enabled --segment-size $MEMORY_SEGMENT_SIZE --segment-overlap $MEMORY_SEGMENT_OVERLAP"
    
    # Add segment ID if specified
    if [[ -n "$segment_id" ]]; then
      segmentation_args="$segmentation_args --segment-id $segment_id"
    fi
  fi
  
  # If database is available and not in dual mode, use DB operations
  if [[ "$db_available" == "true" && "$MEMORY_DUAL_MODE" != "true" ]]; then
    debug_log "Using database mode for memory loading with segmentation: $MEMORY_SEGMENTATION_ENABLED"
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" load \
      --agent "$agent_name" \
      --context "${context:-general}" \
      $segmentation_args
    
    return $?
  fi
  
  # In dual mode, try DB first, fall back to legacy if not found
  if [[ "$db_available" == "true" && "$MEMORY_DUAL_MODE" == "true" ]]; then
    debug_log "Dual mode: trying DB first for memory loading with segmentation: $MEMORY_SEGMENTATION_ENABLED"
    
    # Try to load from database
    if "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" load \
         --agent "$agent_name" \
         --context "${context:-general}" \
         $segmentation_args 2>/dev/null; then
      debug_log "Successfully loaded memory from database"
      return 0
    fi
    
    debug_log "Agent not found in database, falling back to legacy"
  fi
  
  # Legacy mode (no segmentation support)
  debug_log "Using legacy mode for memory loading (no segmentation)"
  "$MEMORY_CLI_PATH" --format "$format" --cache-dir "$CACHE_DIR" load \
    --agent "$agent_name" \
    --context "${context:-general}"
  
  return $?
}

# Load agent memory using legacy implementation
load_agent_memory_legacy() {
  local agent_name="$1"
  local context="$2"
  
  debug_log "Using legacy implementation to load memory for $agent_name"
  
  # Legacy implementation code (fallback)
  local agent_dir="$CACHE_DIR/AGENTS/$agent_name"
  local memory_file="$agent_dir/MEMORY.md"
  local learning_file="$agent_dir/ContinuousLearning.md"
  
  # Check if agent directory exists
  if [[ ! -d "$agent_dir" ]]; then
    print_error "Agent not found: $agent_name"
    return 1
  fi
  
  # Output memory contents
  if [[ -f "$memory_file" ]]; then
    cat "$memory_file"
    echo -e "\n\n# Continuous Learning\n"
  fi
  
  if [[ -f "$learning_file" ]]; then
    cat "$learning_file"
  fi
  
  # If context is provided, look for context-specific files
  if [[ -n "$context" ]]; then
    local context_files=($(find "$agent_dir/Sessions" -type f -name "*${context}*.md" 2>/dev/null))
    
    if [[ ${#context_files[@]} -gt 0 ]]; then
      echo -e "\n\n# Context-Specific Memory: $context\n"
      
      for file in "${context_files[@]}"; do
        echo -e "## $(basename "$file" .md)\n"
        cat "$file"
        echo -e "\n"
      done
    fi
  fi
  
  return 0
}

# Validate agent memory
validate_agent_memory() {
  local agent_name="$1"
  local format="${2:-json}"
  
  # Validate required arguments
  if [[ -z "$agent_name" ]]; then
    print_error "Missing required arguments for validate_agent_memory"
    print_info "Usage: validate_agent_memory <agent_name> [format]"
    return 1
  fi
  
  debug_log "Validating memory for agent $agent_name"
  
  # Check for memory CLI
  if ! check_memory_cli; then
    print_error "Memory CLI required for validation"
    print_info "Run 'cargo build --release' in scripts directory to build it"
    return 1
  fi
  
  # Check database in DB mode
  local db_available=false
  if check_database; then
    db_available=true
  fi
  
  # If database is available and not in dual mode, use DB operations
  if [[ "$db_available" == "true" && "$MEMORY_DUAL_MODE" != "true" ]]; then
    debug_log "Using database mode for memory validation"
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" validate \
      --agent "$agent_name"
    
    return $?
  fi
  
  # In dual mode, validate both systems if available
  if [[ "$db_available" == "true" && "$MEMORY_DUAL_MODE" == "true" ]]; then
    debug_log "Dual mode: validating both systems"
    
    # Validate legacy first
    "$MEMORY_CLI_PATH" --format "$format" --cache-dir "$CACHE_DIR" validate \
      --agent "$agent_name"
      
    local legacy_result=$?
    
    # Also validate database
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" validate \
      --agent "$agent_name" >/dev/null
    
    # Return the legacy result
    return $legacy_result
  fi
  
  # Legacy mode
  debug_log "Using legacy mode for memory validation"
  "$MEMORY_CLI_PATH" --format "$format" --cache-dir "$CACHE_DIR" validate \
    --agent "$agent_name"
  
  return $?
}

# Migrate agent memory from legacy to database
migrate_agent_memory() {
  local agent_name="$1"
  local force="${2:-false}"
  local format="${3:-json}"
  
  debug_log "Migrating memory for agent $agent_name (force=$force)"
  
  # Check for memory CLI
  if ! check_memory_cli; then
    print_error "Memory CLI required for migration"
    print_info "Run 'cargo build --release' in scripts directory to build it"
    return 1
  fi
  
  # Execute CLI with migrate command
  if [[ -n "$agent_name" && "$agent_name" != "all" ]]; then
    debug_log "Migrating single agent: $agent_name"
    
    if [[ "$force" == "true" ]]; then
      "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" migrate \
        --agent "$agent_name" --force
    else
      "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" migrate \
        --agent "$agent_name"
    fi
  else
    debug_log "Migrating all agents"
    
    if [[ "$force" == "true" ]]; then
      "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" migrate --force
    else
      "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" migrate
    fi
  fi
  
  return $?
}

# Set memory operation mode (legacy, db, dual)
set_memory_mode() {
  local mode="$1"
  
  case "$mode" in
    legacy)
      debug_log "Setting memory mode to legacy-only"
      export MEMORY_DUAL_MODE=false
      ;;
    db)
      debug_log "Setting memory mode to database-only"
      export MEMORY_DUAL_MODE=false
      ;;
    dual)
      debug_log "Setting memory mode to dual (legacy+db)"
      export MEMORY_DUAL_MODE=true
      ;;
    *)
      print_error "Invalid memory mode: $mode"
      print_info "Valid modes: legacy, db, dual"
      return 1
      ;;
  esac
  
  return 0
}

# Configure memory segmentation settings
set_memory_segmentation() {
  local enabled="$1"
  local segment_size="$2"
  local segment_overlap="$3"
  
  if [[ "$enabled" == "true" || "$enabled" == "false" ]]; then
    debug_log "Setting memory segmentation enabled=$enabled"
    export MEMORY_SEGMENTATION_ENABLED="$enabled"
  else
    error_log 1 "Invalid segmentation enabled value: $enabled (must be 'true' or 'false')"
  fi
  
  if [[ -n "$segment_size" ]]; then
    if [[ "$segment_size" =~ ^[0-9]+$ && "$segment_size" -gt 0 ]]; then
      debug_log "Setting memory segment size=$segment_size"
      export MEMORY_SEGMENT_SIZE="$segment_size"
    else
      error_log 1 "Invalid segment size: $segment_size (must be a positive integer)"
    fi
  fi
  
  if [[ -n "$segment_overlap" ]]; then
    if [[ "$segment_overlap" =~ ^[0-9]+$ && "$segment_overlap" -ge 0 ]]; then
      debug_log "Setting memory segment overlap=$segment_overlap"
      export MEMORY_SEGMENT_OVERLAP="$segment_overlap"
    else
      error_log 1 "Invalid segment overlap: $segment_overlap (must be a non-negative integer)"
    fi
  fi
  
  return 0
}

# Initialize the database
init_database() {
  local force="${1:-false}"
  local format="${2:-json}"
  
  debug_log "Initializing database (force=$force)"
  
  # Check for memory CLI
  if ! check_memory_cli; then
    print_error "Memory CLI required for database initialization"
    print_info "Run 'cargo build --release' in scripts directory to build it"
    return 1
  fi
  
  # Initialize database
  if [[ "$force" == "true" ]]; then
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" init --force
  else
    "$MEMORY_CLI_PATH" --format "$format" --db-path "$DB_PATH" --agents-dir "$AGENTS_DIR" init
  fi
  
  return $?
}

# Export functions
export -f create_agent_cache
export -f load_agent_memory
export -f validate_agent_memory
export -f get_memory_system_status
export -f migrate_agent_memory
export -f set_memory_mode
export -f set_memory_segmentation
export -f init_database

# Set up optimization detection (used by agent scripts)
if check_memory_cli; then
  export MEMORY_OPTIMIZATION_AVAILABLE="true"
  debug_log "Memory optimization system is available"
else
  export MEMORY_OPTIMIZATION_AVAILABLE="false"
  debug_log "Memory optimization system is NOT available"
fi

# Run command if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  # Extract command and arguments
  if [[ $# -eq 0 ]]; then
    print_error "No command specified"
    print_info "Available commands: create_agent_cache, load_agent_memory, validate_agent_memory, get_memory_system_status, migrate_agent_memory, set_memory_mode, set_memory_segmentation, init_database"
    exit 1
  fi
  
  cmd="$1"
  shift
  
  # Run the specified command
  case "$cmd" in
    create_agent_cache)
      create_agent_cache "$@"
      ;;
    load_agent_memory)
      load_agent_memory "$@"
      ;;
    validate_agent_memory)
      validate_agent_memory "$@"
      ;;
    get_memory_system_status)
      get_memory_system_status "$@"
      ;;
    migrate_agent_memory)
      migrate_agent_memory "$@"
      ;;
    set_memory_mode)
      set_memory_mode "$@"
      ;;
    set_memory_segmentation)
      set_memory_segmentation "$@"
      ;;
    init_database)
      init_database "$@"
      ;;
    *)
      print_error "Unknown command: $cmd"
      print_info "Available commands: create_agent_cache, load_agent_memory, validate_agent_memory, get_memory_system_status, migrate_agent_memory, set_memory_mode, set_memory_segmentation, init_database"
      exit 1
      ;;
  esac
  
  exit $?
fi