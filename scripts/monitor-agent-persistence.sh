#!/bin/bash
# Agent Persistence Monitoring System

CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"

monitor_agent_identity() {
    local response="$1"
    local expected_agent="$2"
    
    echo "🔍 Monitoring agent identity for: $expected_agent"
    
    # Check for proper agent identity format
    if [[ ! "$response" =~ ^\[.*\]: ]]; then
        echo "⚠️  WARNING: Missing agent identity format"
        return 1
    fi
    
    # Check for Claude Code references
    if echo "$response" | grep -qi "claude code"; then
        echo "🚨 CRITICAL: Agent identity loss detected - Claude Code reference found"
        return 2
    fi
    
    # Check for signature format
    if [[ ! "$response" =~ --\ \[.*\]$ ]]; then
        echo "⚠️  WARNING: Missing agent signature"
        return 1
    fi
    
    # Check for agent-specific patterns
    if [[ ! "$response" =~ \[$expected_agent\]: ]]; then
        echo "⚠️  WARNING: Expected agent $expected_agent not found in response"
        return 1
    fi
    
    echo "✅ Agent identity validation passed for $expected_agent"
    return 0
}

validate_agent_capabilities() {
    local agent_name="$1"
    local agent_path="$CI_PATH/AGENTS/$agent_name"
    
    if [[ ! -d "$agent_path" ]]; then
        echo "❌ ERROR: Agent directory not found: $agent_path"
        return 1
    fi
    
    if [[ ! -f "$agent_path/PERSISTENCE_PROTOCOL.md" ]]; then
        echo "⚠️  WARNING: Missing persistence protocol for $agent_name"
        return 1
    fi
    
    echo "✅ Agent capabilities validated for $agent_name"
    return 0
}

trigger_identity_recovery() {
    local agent_name="$1"
    echo "🔄 Triggering identity recovery for $agent_name"
    
    # Log recovery event
    echo "$(date): Identity recovery triggered for $agent_name" >> "$CI_PATH/logs/agent-persistence.log"
    
    # Instructions for manual recovery
    echo "RECOVERY INSTRUCTIONS:"
    echo "1. Review $CI_PATH/AGENTS/$agent_name/PERSISTENCE_PROTOCOL.md"
    echo "2. Reload agent identity and capabilities"
    echo "3. Restart operation as [$agent_name]"
    echo "4. Verify agent-specific context is restored"
}

# Main monitoring function
main() {
    local response="${1:-}"
    local expected_agent="${2:-}"
    
    if [[ -z "$response" || -z "$expected_agent" ]]; then
        echo "Usage: $0 <response_text> <expected_agent>"
        exit 1
    fi
    
    # Create logs directory if it doesn't exist
    mkdir -p "$CI_PATH/logs"
    
    # Validate agent capabilities
    validate_agent_capabilities "$expected_agent"
    
    # Monitor agent identity
    if ! monitor_agent_identity "$response" "$expected_agent"; then
        trigger_identity_recovery "$expected_agent"
        exit 1
    fi
    
    echo "🔒 Agent persistence monitoring completed successfully"
}

# Run main function if script is executed directly
if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    main "$@"
fi
