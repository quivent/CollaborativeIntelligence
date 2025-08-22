#!/bin/bash
# Standard Session Creation Script for Collaborative Intelligence Agents
# Usage: ./session-creation.sh "SessionName" "Brief description"

# Validate arguments
if [ $# -lt 2 ]; then
    echo "Usage: $0 <SessionName> <Description>"
    echo "Example: $0 ProtocolUpdate \"Updating system protocols\""
    exit 1
fi

SESSION_NAME="$1"
DESCRIPTION="$2"
AGENT_NAME=$(basename $(dirname $(pwd)))
SESSION_DIR="Sessions/$SESSION_NAME"
TIMESTAMP=$(date -Iseconds)

# Create session directory structure
echo "Creating session: $SESSION_NAME"
mkdir -p "$SESSION_DIR"/{implementation,analysis,references}

# Create README.md
cat > "$SESSION_DIR/README.md" << EOF
# $SESSION_NAME

## Purpose
$DESCRIPTION

## Status
Active - Started $TIMESTAMP

## Objectives
- [Define primary objectives]
- [Add secondary objectives]

## Key Decisions
- [Document decisions as they occur]

## References
- [Link to related sessions]
- [Link to relevant documentation]
EOF

# Create metadata.json
cat > "$SESSION_DIR/metadata.json" << EOF
{
  "session_name": "$SESSION_NAME",
  "description": "$DESCRIPTION",
  "agent": "$AGENT_NAME",
  "created": "$TIMESTAMP",
  "status": "active",
  "participants": ["$AGENT_NAME"],
  "tags": [],
  "references": []
}
EOF

# Create outcomes.md
cat > "$SESSION_DIR/outcomes.md" << EOF
# Outcomes - $SESSION_NAME

## Summary
[To be completed at session end]

## Key Achievements
- [List major accomplishments]

## Lessons Learned
- [Document important insights]

## Action Items
- [ ] [Future tasks identified]

## Impact
- [Describe impact on system/project]
EOF

# Report to Topologist
TOPOLOGIST_NOTIFICATION="[$AGENT_NAME] -> Topologist: Repository Change Notification
Operation: CREATE
Path: $(pwd)/$SESSION_DIR/
Purpose: Starting new session - $DESCRIPTION
Session: $SESSION_NAME
Timestamp: $TIMESTAMP"

# Check if we can find Topologist's CommitNotification.md
TOPOLOGIST_PATH="../../Topologist/CommitNotification.md"
if [ -f "$TOPOLOGIST_PATH" ]; then
    echo -e "\n$TOPOLOGIST_NOTIFICATION" >> "$TOPOLOGIST_PATH"
    echo "✓ Reported to Topologist"
else
    echo "⚠️  Warning: Could not find Topologist notification file"
    echo "Manual notification required:"
    echo "$TOPOLOGIST_NOTIFICATION"
fi

echo "✓ Session created successfully: $SESSION_DIR"
echo "✓ Standard files initialized"
echo "Next steps:"
echo "  1. Update README.md with specific objectives"
echo "  2. Begin work and document in implementation/"
echo "  3. Update outcomes.md as you progress"
echo "  4. Report all file changes to Topologist"