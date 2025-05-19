#!/bin/bash

# Script to switch between agents and update the cache

# Default paths
CACHE_DIR="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CACHE/SESSIONS"
AGENT_BINARY="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts/agent_cache_system"

# Check if agent binary exists
if [ ! -f "$AGENT_BINARY" ]; then
    echo "Agent cache system binary not found at $AGENT_BINARY"
    echo "Please build it first with: cd scripts && ./build.sh"
    exit 1
fi

# Function to switch to an agent
switch_to_agent() {
    local agent_name="$1"
    local directory="$2"
    local initializer="$3"
    local intent="$4"
    
    # Deactivate current agent if needed
    if [ -f "$CACHE_DIR/current_agent.txt" ]; then
        local current_agent=$(cat "$CACHE_DIR/current_agent.txt")
        if [ -n "$current_agent" ] && [ "$current_agent" != "$agent_name" ]; then
            echo "Deactivating current agent: $current_agent"
            "$AGENT_BINARY" deactivate "$current_agent"
        fi
    fi
    
    # Activate new agent
    echo "Activating agent: $agent_name"
    "$AGENT_BINARY" activate "$agent_name" "$directory" "$initializer" "$intent"
    
    # Update current agent
    echo "$agent_name" > "$CACHE_DIR/current_agent.txt"
    
    # Set terminal title (as a backup to the Rust implementation)
    echo -e "\033]0;Agent: $agent_name\007"
    
    echo "Switched to agent: $agent_name"
}

# Check if we need to build the agent_cache_system
if [ ! -f "$AGENT_BINARY" ]; then
    echo "Building agent cache system..."
    (cd "$(dirname "$AGENT_BINARY")" && ./build.sh)
fi

# Ensure CACHE_DIR exists
mkdir -p "$CACHE_DIR"

# Main logic
if [ "$1" == "switch" ] && [ -n "$2" ]; then
    # Get agent name
    agent_name="$2"
    
    # Get current directory
    directory="$(pwd)"
    
    # Get initializer (default to "User")
    initializer="${3:-User}"
    
    # Get intent (default to "Manual activation")
    intent="${4:-Manual activation}"
    
    # Switch to the agent
    switch_to_agent "$agent_name" "$directory" "$initializer" "$intent"
    
elif [ "$1" == "list" ]; then
    # List active agents
    echo "Active agents:"
    "$AGENT_BINARY" list
    
elif [ "$1" == "current" ]; then
    # Show current agent
    if [ -f "$CACHE_DIR/current_agent.txt" ]; then
        echo "Current agent: $(cat "$CACHE_DIR/current_agent.txt")"
    else
        echo "No agent is currently active"
    fi
    
elif [ "$1" == "cleanup" ]; then
    # Clean up old cache files
    days="${2:-7}"
    echo "Cleaning up inactive agent cache files older than $days days..."
    "$AGENT_BINARY" cleanup "$days"
    
else
    # Show usage
    echo "Usage:"
    echo "  $0 switch AGENT_NAME [INITIALIZER] [INTENT]"
    echo "  $0 list"
    echo "  $0 current"
    echo "  $0 cleanup [DAYS]"
    echo ""
    echo "Examples:"
    echo "  $0 switch Athena"
    echo "  $0 switch Scholar User \"Research task on memory systems\""
    echo "  $0 list"
    echo "  $0 current"
    echo "  $0 cleanup 14"
fi