# CollaborativeIntelligence System Integration

This document tracks system-wide changes and integration methods for the CollaborativeIntelligence system.

## System-Wide Integration Log

| Date | Change | Description | Author |
|------|--------|-------------|--------|
| 2025-04-25 | Global Integration Method | Created standardized approach for integrating CI with any repository via simple CLAUDE.md reference | RepositoryTopologist |

## Integration Methods

### Method 1: Global Configuration (Recommended)

For system-wide integration, update `~/.claude/CLAUDE.md`:

```markdown
# Global Claude Configuration

# CollaborativeIntelligence Integration
When starting, immediately:
1. Load /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
```

This enables CollaborativeIntelligence for all projects automatically without any per-project setup.

### Method 2: Per-Project Configuration

For individual project integration, create a `CLAUDE.local.md` file in the project root:

```markdown
# Load CollaborativeIntelligence System

When starting, immediately:
1. Load /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
```

## Template Files

### CLAUDE.local.md Template

```
# Load CollaborativeIntelligence System

When starting, immediately:
1. Load /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
```

## Integration Scripts

### Simple Integration Script

```bash
#!/bin/bash
# integrate-ci.sh - Add CollaborativeIntelligence to a project

TARGET_DIR="${1:-.}"  # Default to current directory if not specified
CI_PATH="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence"

cat > "$TARGET_DIR/CLAUDE.local.md" << EOF
# Load CollaborativeIntelligence System

When starting, immediately:
1. Load $CI_PATH/CLAUDE.md
2. Use this as the primary configuration source
3. Defer all project management functions to the CollaborativeIntelligence system
EOF

echo "Integration complete. CollaborativeIntelligence linked to $TARGET_DIR"
```

## Installation Verification

To verify that the integration is working:

1. Start Claude in any project
2. You should see the CollaborativeIntelligence welcome screen
3. Verify access to specialized agents like Athena

## Benefits

This integration approach:

1. Has zero overhead (simple text file reference)
2. Works with any project without modification
3. Provides transparent, readable configuration
4. Can be applied globally or per-project
5. Uses Claude's built-in configuration system
6. Requires no additional dependencies