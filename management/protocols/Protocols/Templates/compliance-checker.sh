#!/bin/bash
# Agent Compliance Self-Check Script
# Usage: ./compliance-checker.sh

AGENT_NAME=$(basename $(dirname $(pwd)))
TIMESTAMP=$(date -Iseconds)
COMPLIANCE_SCORE=0
TOTAL_CHECKS=0

echo "=== Collaborative Intelligence Compliance Check ==="
echo "Agent: $AGENT_NAME"
echo "Date: $TIMESTAMP"
echo ""

# Function to check and score
check_item() {
    local description="$1"
    local check_command="$2"
    
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
    
    if eval "$check_command"; then
        echo "✓ $description"
        COMPLIANCE_SCORE=$((COMPLIANCE_SCORE + 1))
        return 0
    else
        echo "✗ $description"
        return 1
    fi
}

# Check for required files
echo "1. Core Files Check:"
check_item "README.md exists" "[ -f README.md ]"
check_item "MEMORY.md exists" "[ -f MEMORY.md ]"
check_item "ContinuousLearning.md exists" "[ -f ContinuousLearning.md ]"
check_item "Sessions directory exists" "[ -d Sessions ]"
echo ""

# Check session compliance
echo "2. Session Structure Check:"
if [ -d Sessions ]; then
    for session in Sessions/*/; do
        if [ -d "$session" ]; then
            session_name=$(basename "$session")
            echo "   Checking $session_name:"
            check_item "    - README.md" "[ -f $session/README.md ]"
            check_item "    - metadata.json" "[ -f $session/metadata.json ]"
            check_item "    - outcomes.md" "[ -f $session/outcomes.md ]"
        fi
    done
else
    echo "   No sessions found"
fi
echo ""

# Check recent updates
echo "3. Memory Update Recency:"
if [ -f MEMORY.md ]; then
    last_modified=$(stat -f %Sm -t "%Y-%m-%d" MEMORY.md 2>/dev/null || stat -c %y MEMORY.md 2>/dev/null | cut -d' ' -f1)
    days_old=$(( ($(date +%s) - $(date -d "$last_modified" +%s || date -j -f "%Y-%m-%d" "$last_modified" +%s)) / 86400 ))
    
    if [ $days_old -lt 30 ]; then
        echo "✓ MEMORY.md updated within 30 days ($days_old days old)"
        COMPLIANCE_SCORE=$((COMPLIANCE_SCORE + 1))
    else
        echo "✗ MEMORY.md not updated in $days_old days"
    fi
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
fi

if [ -f ContinuousLearning.md ]; then
    last_modified=$(stat -f %Sm -t "%Y-%m-%d" ContinuousLearning.md 2>/dev/null || stat -c %y ContinuousLearning.md 2>/dev/null | cut -d' ' -f1)
    days_old=$(( ($(date +%s) - $(date -d "$last_modified" +%s || date -j -f "%Y-%m-%d" "$last_modified" +%s)) / 86400 ))
    
    if [ $days_old -lt 7 ]; then
        echo "✓ ContinuousLearning.md updated within 7 days ($days_old days old)"
        COMPLIANCE_SCORE=$((COMPLIANCE_SCORE + 1))
    else
        echo "✗ ContinuousLearning.md not updated in $days_old days"
    fi
    TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
fi
echo ""

# Check for Topologist reporting
echo "4. Topologist Reporting Check:"
TOPOLOGIST_PATH="../../Topologist/CommitNotification.md"
if [ -f "$TOPOLOGIST_PATH" ]; then
    if grep -q "$AGENT_NAME" "$TOPOLOGIST_PATH"; then
        echo "✓ Found agent reports in Topologist notification log"
        COMPLIANCE_SCORE=$((COMPLIANCE_SCORE + 1))
    else
        echo "✗ No reports found in Topologist notification log"
    fi
else
    echo "✗ Could not find Topologist notification file"
fi
TOTAL_CHECKS=$((TOTAL_CHECKS + 1))
echo ""

# Calculate compliance percentage
COMPLIANCE_PERCENTAGE=$(( (COMPLIANCE_SCORE * 100) / TOTAL_CHECKS ))

echo "=== Compliance Summary ==="
echo "Score: $COMPLIANCE_SCORE / $TOTAL_CHECKS"
echo "Compliance: $COMPLIANCE_PERCENTAGE%"
echo ""

if [ $COMPLIANCE_PERCENTAGE -ge 90 ]; then
    echo "Status: COMPLIANT ✓"
elif [ $COMPLIANCE_PERCENTAGE -ge 70 ]; then
    echo "Status: PARTIALLY COMPLIANT ⚠️"
    echo "Action Required: Address failing checks"
else
    echo "Status: NON-COMPLIANT ✗"
    echo "IMMEDIATE ACTION REQUIRED: Review protocol reference"
fi

# Generate report
REPORT_FILE="compliance-report-$(date +%Y%m%d-%H%M%S).md"
cat > "$REPORT_FILE" << EOF
# Compliance Report - $AGENT_NAME

Generated: $TIMESTAMP

## Score: $COMPLIANCE_SCORE / $TOTAL_CHECKS ($COMPLIANCE_PERCENTAGE%)

## Detailed Results

### Core Files
$([ -f README.md ] && echo "✓ README.md" || echo "✗ README.md")
$([ -f MEMORY.md ] && echo "✓ MEMORY.md" || echo "✗ MEMORY.md")
$([ -f ContinuousLearning.md ] && echo "✓ ContinuousLearning.md" || echo "✗ ContinuousLearning.md")
$([ -d Sessions ] && echo "✓ Sessions/" || echo "✗ Sessions/")

### Recommendations
$([ $COMPLIANCE_PERCENTAGE -lt 90 ] && echo "- Review /Protocols/AGENT_PROTOCOL_REFERENCE.md" || echo "- Maintain current compliance")
$([ ! -f ContinuousLearning.md ] && echo "- Create ContinuousLearning.md")
$([ $COMPLIANCE_PERCENTAGE -lt 70 ] && echo "- Schedule immediate protocol training")

## Next Steps
1. Address any failing checks
2. Update outdated documentation
3. Ensure Topologist reporting compliance
4. Schedule regular compliance reviews
EOF

echo ""
echo "Full report saved to: $REPORT_FILE"