#!/bin/bash
# Optimized Memory Loader for Collaborative Intelligence System
# Implements parallel loading, memory segmentation, and caching
# Now with Rust optimization integration

# Determine script directory
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CI_TOOLS_DIR="$(cd "$SCRIPT_DIR/.." && pwd)"
CI_ROOT_DIR="$(cd "$CI_TOOLS_DIR/.." && pwd)"

# Configuration
MEMORY_CACHE_DIR="$HOME/.cache/ci-memory-cache"
CACHE_EXPIRY=3600  # Cache expiry in seconds (1 hour)

# Import memory optimization bridge if available
if [[ -f "$SCRIPT_DIR/rust-memory-bridge.sh" ]]; then
  source "$SCRIPT_DIR/rust-memory-bridge.sh"
  MEMORY_OPTIMIZATION_AVAILABLE=true
  debug_log "Memory optimization system detected and loaded"
else
  MEMORY_OPTIMIZATION_AVAILABLE=false
  debug_log "Memory optimization system not available, using legacy implementation"
fi

# Ensure cache directory exists
mkdir -p "$MEMORY_CACHE_DIR"

# Log function
log_memory() {
  local level="$1"
  local message="$2"
  echo "[MEMORY-LOADER][$level] $message" >&2
}

# Debug logging function
debug_log() {
  if [[ "${MEMORY_DEBUG:-false}" == "true" ]]; then
    echo "[DEBUG] $1" >&2
  fi
}

# Function to extract segments from a memory file
extract_segments() {
  local file="$1"
  local segment_pattern="^## SEGMENT: ([A-Z_]+)"
  local current_segment=""
  local segment_content=""
  local segments=()
  
  while IFS= read -r line; do
    if [[ $line =~ $segment_pattern ]]; then
      # Save previous segment if it exists
      if [[ -n "$current_segment" && -n "$segment_content" ]]; then
        echo "$current_segment:$segment_content" >> "$MEMORY_CACHE_DIR/segments.tmp"
      fi
      
      # Start new segment
      current_segment="${BASH_REMATCH[1]}"
      segment_content=""
    elif [[ -n "$current_segment" ]]; then
      # Append to current segment
      segment_content+="$line\n"
    fi
  done < "$file"
  
  # Save the last segment
  if [[ -n "$current_segment" && -n "$segment_content" ]]; then
    echo "$current_segment:$segment_content" >> "$MEMORY_CACHE_DIR/segments.tmp"
  fi
}

# Function to determine which segments to load based on context
determine_segments() {
  local agent="$1"
  local context="$2"
  local always_load="CORE_IDENTITY"
  local load_segments="$always_load"
  
  # Parse context keywords to determine which segments to load
  if [[ "$context" == *"history"* || "$context" == *"background"* || "$context" == *"evolution"* ]]; then
    load_segments="$load_segments HISTORICAL_CONTEXT"
  fi
  
  if [[ "$context" == *"code"* || "$context" == *"implement"* || "$context" == *"develop"* || "$context" == *"build"* ]]; then
    load_segments="$load_segments TECHNICAL_CAPABILITIES"
  fi
  
  if [[ "$context" == *"explain"* || "$context" == *"communicate"* || "$context" == *"document"* ]]; then
    load_segments="$load_segments COMMUNICATION_PATTERNS"
  fi
  
  # Always consider specialized knowledge
  load_segments="$load_segments SPECIALIZED_KNOWLEDGE"
  
  echo "$load_segments"
}

# Function to check if a cached segment is valid
is_cache_valid() {
  local agent="$1"
  local segment="$2"
  local cache_file="$MEMORY_CACHE_DIR/${agent}_${segment}.cache"
  local memory_file="$CI_ROOT_DIR/AGENTS/${agent}/MEMORY.md"
  
  # Check if cache exists
  if [[ ! -f "$cache_file" ]]; then
    return 1
  fi
  
  # Check cache freshness
  local cache_time=$(stat -c %Y "$cache_file" 2>/dev/null || stat -f %m "$cache_file")
  local current_time=$(date +%s)
  local memory_time=$(stat -c %Y "$memory_file" 2>/dev/null || stat -f %m "$memory_file")
  
  # Cache is invalid if memory file is newer or cache is expired
  if (( memory_time > cache_time )) || (( current_time - cache_time > CACHE_EXPIRY )); then
    return 1
  fi
  
  return 0
}

# Function to load a memory segment
load_segment() {
  local agent="$1"
  local segment="$2"
  local cache_file="$MEMORY_CACHE_DIR/${agent}_${segment}.cache"
  
  # Check if we can use cached segment
  if is_cache_valid "$agent" "$segment"; then
    log_memory "INFO" "Using cached segment $segment for agent $agent"
    cat "$cache_file"
    return 0
  fi
  
  # Extract segment from memory file if cache is invalid
  local memory_file="$CI_ROOT_DIR/AGENTS/${agent}/MEMORY.md"
  
  if [[ ! -f "$memory_file" ]]; then
    log_memory "ERROR" "Memory file not found: $memory_file"
    return 1
  fi
  
  # Create temporary segments file
  rm -f "$MEMORY_CACHE_DIR/segments.tmp"
  extract_segments "$memory_file"
  
  # Get the specific segment
  grep -E "^$segment:" "$MEMORY_CACHE_DIR/segments.tmp" | cut -d':' -f2- > "$cache_file"
  
  # Return the segment content
  cat "$cache_file"
  
  # Clean up
  rm -f "$MEMORY_CACHE_DIR/segments.tmp"
}

# Main function to load memory with optimizations
load_agent_memory() {
  local agent="$1"
  local context="$2"
  local task="${3:-General Task}"
  
  # Update terminal window title if function exists
  if type set_terminal_title &>/dev/null; then
    local project_name=$(basename "$(pwd)")
    set_terminal_title "$project_name - $task - $agent"
  fi
  
  # Create agent cache entry
  if type create_optimized_agent_cache &>/dev/null; then
    local current_dir=$(pwd)
    debug_log "Creating agent cache entry"
    create_optimized_agent_cache "$agent" "$current_dir" "MemoryLoader" "$task"
  fi
  
  # Check if we should use the optimized memory system
  if [[ "$MEMORY_OPTIMIZATION_AVAILABLE" == "true" ]]; then
    debug_log "Using optimized memory system for agent $agent"
    
    # Use the optimized memory loader from the bridge
    if load_optimized_agent_memory "$agent" "$context"; then
      debug_log "Successfully loaded optimized memory for agent $agent"
      return 0
    else
      debug_log "Optimized memory loading failed, falling back to legacy"
    fi
  fi
  
  # Legacy implementation
  debug_log "Using legacy memory loading for agent $agent"
  local result=""
  local segments=$(determine_segments "$agent" "$context")
  
  log_memory "INFO" "Loading memory for agent $agent with context: $context"
  log_memory "INFO" "Selected segments: $segments"
  
  # Load core identity first (synchronously)
  if [[ "$segments" == *"CORE_IDENTITY"* ]]; then
    result+="$(load_segment "$agent" "CORE_IDENTITY")"
    result+="\n\n"
  fi
  
  # Load remaining segments in parallel
  # This is a simplified representation of parallel loading
  # In a real implementation, this would use background processes or async loading
  for segment in $segments; do
    if [[ "$segment" != "CORE_IDENTITY" ]]; then
      result+="$(load_segment "$agent" "$segment")"
      result+="\n\n"
    fi
  done
  
  echo -e "$result"
}

# Main execution if script is run directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
  # Usage check
  if [[ "$#" -lt 1 ]]; then
    echo "Usage: $0 <agent_name> [context]"
    exit 1
  fi
  
  AGENT_NAME="$1"
  CONTEXT="${2:-}"
  
  load_agent_memory "$AGENT_NAME" "$CONTEXT"
fi