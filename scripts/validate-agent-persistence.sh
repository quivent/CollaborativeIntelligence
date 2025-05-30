#!/bin/bash
# Agent Persistence Validation Script

CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"
AGENTS_PATH="$CI_PATH/AGENTS"

echo "🔍 Validating Agent Persistence System..."

# Check CLAUDE.md has persistence protocol
if grep -q "Agent Persistence Requirements" "$CI_PATH/CLAUDE.md"; then
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
