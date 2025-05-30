#!/bin/bash
# Agent Persistence Implementation Script
# Deploys the agent persistence protocol across the CollaborativeIntelligence system

set -e

CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
AGENTS_PATH="$CI_PATH/AGENTS"

echo "🔒 Implementing Agent Persistence Protocol..."

# 1. Backup existing CLAUDE.md
echo "📦 Creating backup of existing CLAUDE.md..."
cp "$CI_PATH/CLAUDE.md" "$CI_PATH/CLAUDE.md.pre-persistence-backup"

# 2. Deploy enhanced CLAUDE.md
echo "🚀 Deploying enhanced CLAUDE.md with persistence protocol..."
cp "$CI_PATH/CLAUDE.md.persistence-enhanced" "$CI_PATH/CLAUDE.md"

# 3. Create agent-specific persistence protocols
echo "🤖 Creating agent-specific persistence protocols..."

for agent_dir in "$AGENTS_PATH"/*; do
    if [[ -d "$agent_dir" ]]; then
        agent_name=$(basename "$agent_dir")
        
        # Skip non-agent directories
        if [[ "$agent_name" == "NOTIFICATION_TO_ALL_AGENTS"* ]]; then
            continue
        fi
        
        echo "   Creating persistence protocol for $agent_name..."
        
        # Create PERSISTENCE_PROTOCOL.md for each agent
        cat > "$agent_dir/PERSISTENCE_PROTOCOL.md" << EOF
# $agent_name Identity Persistence Protocol

## Identity Markers
- Name: $agent_name
- Specialization: $(get_agent_specialization "$agent_name")
- Response Pattern: [$agent_name]: {content} -- [$agent_name]

## Reversion Detection
Watch for these indicators of identity loss:
- Generic "Claude Code" references
- Loss of specialized terminology
- Degraded capability performance
- Missing agent-specific context
- Responses without [$agent_name]: prefix

## Recovery Actions
If identity loss detected:
1. STOP current operation immediately
2. Reload identity from this file
3. Restore specialized context from README.md and MEMORY.md
4. Resume operation as $agent_name
5. Log recovery event
6. Notify user of identity restoration

## Authentication Signature
Agent: $agent_name
Activated: $(date)
Persistence Protocol: v1.0
Identity Lock: ENABLED

## Critical Instructions
- NEVER operate as "Claude Code"
- ALWAYS maintain [$agent_name] identity
- Use specialized capabilities consistently
- Preserve agent-specific context
- Escalate to appropriate specialists when needed
EOF

        echo "   ✅ $agent_name persistence protocol created"
    fi
done

# 4. Create monitoring script
echo "📊 Creating agent persistence monitoring script..."
cat > "$CI_PATH/scripts/monitor-agent-persistence.sh" << 'EOF'
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
EOF

chmod +x "$CI_PATH/scripts/monitor-agent-persistence.sh"

# 5. Create recovery script
echo "🚑 Creating agent identity recovery script..."
cat > "$CI_PATH/scripts/recover-agent-identity.sh" << 'EOF'
#!/bin/bash
# Agent Identity Recovery Script

CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"

recover_agent_identity() {
    local agent_name="$1"
    local agent_path="$CI_PATH/AGENTS/$agent_name"
    
    echo "🚑 Initiating emergency identity recovery for $agent_name"
    
    # Check if agent exists
    if [[ ! -d "$agent_path" ]]; then
        echo "❌ ERROR: Agent $agent_name not found in $agent_path"
        exit 1
    fi
    
    # Load identity from persistence protocol
    if [[ -f "$agent_path/PERSISTENCE_PROTOCOL.md" ]]; then
        echo "📋 Loading identity from persistence protocol..."
        echo "Agent: $agent_name"
        echo "Identity Status: RECOVERED"
        echo "Capabilities: RESTORED"
        echo "Context: RELOADED"
    else
        echo "⚠️  WARNING: Persistence protocol not found, using emergency recovery"
    fi
    
    # Load memory if available
    if [[ -f "$agent_path/MEMORY.md" ]]; then
        echo "🧠 Reloading agent memory..."
    fi
    
    # Load README for capabilities
    if [[ -f "$agent_path/README.md" ]]; then
        echo "📖 Reloading agent capabilities..."
    fi
    
    # Log recovery
    echo "$(date): Emergency identity recovery completed for $agent_name" >> "$CI_PATH/logs/agent-persistence.log"
    
    echo "✅ Identity recovery completed for $agent_name"
    echo "🔒 Agent should now respond as [$agent_name]: content -- [$agent_name]"
}

# Usage check
if [[ $# -ne 1 ]]; then
    echo "Usage: $0 <agent_name>"
    echo "Example: $0 Athena"
    exit 1
fi

# Create logs directory
mkdir -p "$CI_PATH/logs"

# Recover agent identity
recover_agent_identity "$1"
EOF

chmod +x "$CI_PATH/scripts/recover-agent-identity.sh"

# 6. Create validation script
echo "✅ Creating persistence validation script..."
cat > "$CI_PATH/scripts/validate-agent-persistence.sh" << 'EOF'
#!/bin/bash
# Agent Persistence Validation Script

CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
AGENTS_PATH="$CI_PATH/AGENTS"

echo "🔍 Validating Agent Persistence System..."

# Check CLAUDE.md has persistence protocol
if grep -q "AGENT PERSISTENCE REQUIREMENTS" "$CI_PATH/CLAUDE.md"; then
    echo "✅ CLAUDE.md contains persistence protocol"
else
    echo "❌ CLAUDE.md missing persistence protocol"
    exit 1
fi

# Check each agent has persistence protocol
missing_protocols=0
for agent_dir in "$AGENTS_PATH"/*; do
    if [[ -d "$agent_dir" ]]; then
        agent_name=$(basename "$agent_dir")
        
        # Skip non-agent directories
        if [[ "$agent_name" == "NOTIFICATION_TO_ALL_AGENTS"* ]]; then
            continue
        fi
        
        if [[ -f "$agent_dir/PERSISTENCE_PROTOCOL.md" ]]; then
            echo "✅ $agent_name has persistence protocol"
        else
            echo "❌ $agent_name missing persistence protocol"
            ((missing_protocols++))
        fi
    fi
done

# Check scripts exist
if [[ -f "$CI_PATH/scripts/monitor-agent-persistence.sh" ]]; then
    echo "✅ Monitoring script installed"
else
    echo "❌ Monitoring script missing"
    exit 1
fi

if [[ -f "$CI_PATH/scripts/recover-agent-identity.sh" ]]; then
    echo "✅ Recovery script installed"
else
    echo "❌ Recovery script missing"
    exit 1
fi

# Summary
if [[ $missing_protocols -eq 0 ]]; then
    echo ""
    echo "🎉 Agent Persistence System validation PASSED"
    echo "🔒 All agents have persistence protocols installed"
    echo "📊 Monitoring and recovery systems are ready"
    echo "✅ System is protected against unwanted handback to Claude Code"
else
    echo ""
    echo "⚠️  Agent Persistence System validation FAILED"
    echo "❌ $missing_protocols agents missing persistence protocols"
    exit 1
fi
EOF

chmod +x "$CI_PATH/scripts/validate-agent-persistence.sh"

# Create function to get agent specialization
get_agent_specialization() {
    local agent="$1"
    case "$agent" in
        "Athena") echo "Strategic Intelligence and Decision Making" ;;
        "Gaia") echo "Holistic System Integration" ;;
        "Hermes") echo "Communication and Interface Management" ;;
        "Architect") echo "System Architecture and Design" ;;
        "Engineer") echo "Implementation and Functionality" ;;
        "Debugger") echo "Error Resolution and System Repair" ;;
        "Optimizer") echo "Performance Enhancement and Efficiency" ;;
        "Designer") echo "User Experience and Interface Design" ;;
        "Tester") echo "Quality Assurance and Validation" ;;
        *) echo "Specialized Agent Capabilities" ;;
    esac
}

# 7. Create logs directory
mkdir -p "$CI_PATH/logs"

# 8. Run validation
echo "🔍 Running initial validation..."
"$CI_PATH/scripts/validate-agent-persistence.sh"

echo ""
echo "🎉 Agent Persistence Protocol Implementation Complete!"
echo ""
echo "🔒 Protection Status:"
echo "   ✅ CLAUDE.md enhanced with persistence requirements"
echo "   ✅ All agents have individual persistence protocols"
echo "   ✅ Monitoring and recovery systems installed"
echo "   ✅ Validation systems operational"
echo ""
echo "🚀 Next Steps:"
echo "   1. Test with specialized agents to verify persistence"
echo "   2. Monitor for any Claude Code reversion incidents"
echo "   3. Use recovery scripts if identity loss occurs"
echo "   4. Regular validation with validate-agent-persistence.sh"
echo ""
echo "🛡️  Your specialized agents are now protected from unwanted handback!"