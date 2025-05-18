# CLAUDE.md Protection System

## Overview

This document describes the protection system implemented to safeguard the critical CLAUDE.md file from unauthorized modifications or corruption. The CLAUDE.md file is a core component of the Collaborative Intelligence system, containing essential instructions for agent behavior, command processing, and system operations.

## Protection Mechanisms

The protection system employs a multi-layered approach:

### 1. Backup File (.CLAUDE.md.backup)

A read-only backup of CLAUDE.md is created at:
```
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/.CLAUDE.md.backup
```

This file is set to read-only permissions (444) to prevent accidental modification. It serves as the primary recovery source if the main CLAUDE.md file is corrupted or modified inappropriately.

### 2. Protected Critical Instructions (CLAUDE.md.protected)

A dedicated file containing only the most critical instructions:
```
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md.protected
```

This file:
- Contains only the essential agent activation and response formatting instructions
- Is set to read-only permissions (444)
- Provides a clean source for restoring critical functionality if both CLAUDE.md and the backup become corrupted

### 3. Enhanced CLAUDE.local.md.template

The CLAUDE.local.md.template has been enhanced to:
- Load both CLAUDE.md and CLAUDE.md.protected
- Include redundant critical instructions directly in the template
- Provide fallback functionality if the main files are missing instructions

### 4. Automated Integrity Verification

A system of scripts to verify and maintain CLAUDE.md integrity:

- **claude-integrity.sh**: Standalone script to check and repair CLAUDE.md
- **common.sh enhancements**: Integration with the CI system's common library
- **CI script integration**: Automatic integrity check on CI command execution

## How It Works

### Verification Process

1. When the CI command or integrity script runs, it:
   - Checks if CLAUDE.md exists
   - Verifies that critical sections are present
   - Compares against the backup if available
   - Creates a backup if one doesn't exist

2. If corruption is detected:
   - First attempts to restore from .CLAUDE.md.backup
   - If backup is unavailable, attempts to merge CLAUDE.md.protected into existing file
   - Reports the recovery action taken

### Recovery Hierarchy

The system uses the following recovery sources, in order of preference:

1. .CLAUDE.md.backup (primary backup)
2. CLAUDE.md.protected (critical instructions only)
3. Instructions embedded in CLAUDE.local.md.template (last resort)

## Manual Recovery

If automatic recovery fails, manual restoration can be performed:

```bash
# Option 1: Restore from backup
cp /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/.CLAUDE.md.backup /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md

# Option 2: Use the protected file
cp /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md.protected /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md
```

## Testing and Verification

To verify the protection system is working:

```bash
# Run the integrity check script
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/ci-tools/lib/claude-integrity.sh

# Test recovery by deliberately corrupting CLAUDE.md
# (For testing purposes only)
sed -i.bak '/Agent Activation/d' /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLAUDE.md
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/ci-tools/lib/claude-integrity.sh
```

## Maintenance

Regular maintenance of this protection system:

1. Update backups when legitimate changes are made to CLAUDE.md
2. Periodically verify the integrity of all protection files
3. Ensure the protection scripts are included in system updates

---

*Created by: Fixer*
*Date: May 17, 2025*