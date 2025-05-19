#!/usr/bin/env bash
# mock_legacy_functions.sh - Mock implementations of legacy memory functions
#
# This script provides mock implementations of the legacy memory functions
# that will be used when testing the fallback mechanisms of the memory optimization bridge.

# Mock legacy agent cache creation
create_agent_cache_legacy() {
    local agent_name="$1"
    local directory="$2"
    local initializer="$3"
    local intent="$4"
    
    echo "MOCK: Created legacy cache for agent '$agent_name' in directory '$directory'"
    echo "MOCK: Initializer: $initializer, Intent: $intent"
    return 0
}

# Mock legacy agent memory loading
load_agent_memory_legacy() {
    local agent_name="$1"
    local context="$2"
    
    echo "MOCK: Loaded legacy memory for agent '$agent_name' with context '$context'"
    echo "# Mock Legacy Memory for $agent_name"
    echo ""
    echo "This is mock legacy memory content for testing."
    echo "Context: $context"
    return 0
}

# Mock legacy agent memory validation
validate_agent_memory_legacy() {
    local agent_name="$1"
    
    echo "MOCK: Validated legacy memory for agent '$agent_name'"
    return 0
}

# Mock legacy memory segment loading
load_memory_segment_legacy() {
    local agent_name="$1"
    local segment_name="$2"
    
    echo "MOCK: Loaded legacy memory segment '$segment_name' for agent '$agent_name'"
    echo "# Mock Legacy Memory Segment: $segment_name"
    echo ""
    echo "This is mock legacy memory segment content for testing."
    return 0
}

# Mock legacy memory segment listing
list_memory_segments_legacy() {
    local agent_name="$1"
    
    echo "MEMORY"
    echo "ContinuousLearning"
    echo "README"
    return 0
}

# Mock legacy memory checksum validation
validate_memory_checksum_legacy() {
    local agent_name="$1"
    local segment_name="$2"
    
    echo "MOCK: Validated legacy memory checksum for segment '$segment_name' of agent '$agent_name'"
    return 0
}

# Mock legacy memory backup creation
create_memory_backup_legacy() {
    local agent_name="$1"
    
    echo "MOCK: Created legacy memory backup for agent '$agent_name'"
    return 0
}

# Mock legacy memory restore from backup
restore_memory_from_backup_legacy() {
    local agent_name="$1"
    local backup_file="$2"
    
    echo "MOCK: Restored legacy memory for agent '$agent_name' from backup '$backup_file'"
    return 0
}