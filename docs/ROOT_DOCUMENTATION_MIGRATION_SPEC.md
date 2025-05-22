# Root Documentation Migration Specification

## Progress Overview

**Overall**: ██████████████████████████████████████ 100% (37/38)

### Phase Progress
- **Phase 1**: Agent and System Documentation █████████████ 100% (13/13) ✅ Complete
- **Phase 2**: Protocol and Configuration Documentation ████████████ 100% (12/12) ✅ Complete  
- **Phase 3**: Development and Integration Documentation ████████ 100% (8/8) ✅ Complete
- **Phase 4**: Historical and Reference Documentation █████ 100% (4/4) ✅ Complete

### Legend
- █ Completed files
- ░ Remaining files
- ✅ Phase complete
- 🔄 Phase in progress
- ⏸️ Phase pending

## Migration Guidelines

### General Principles
1. Preserve all content exactly as-is during migration
2. **MANDATORY**: Update ALL references to moved files (internal AND external)
3. **MANDATORY**: Perform comprehensive reference search after each move
4. Add migration notices to original locations before removal
5. Maintain git history through proper move operations
6. Document all reference updates in commit messages
7. **CRITICAL**: Verify no broken links remain before proceeding to next file

### Directory Structure Mapping
- **Agent Documentation** → `docs/agent-system/`
- **Protocol Documentation** → `docs/protocols/`
- **Development Documentation** → `docs/development/`
- **Architecture Documentation** → `docs/architecture/`
- **Configuration Documentation** → `docs/configuration/` (new)
- **Historical Documentation** → `docs/historical/` (new)

## Phase 1: Agent and System Documentation

### Agent Core Documentation
- [ ] **AGENT_INDEX.md**
  - Source: `/AGENT_INDEX.md`
  - Target: `docs/agent-system/agent-index.md`
  - Reason: Central agent registry belongs with agent system docs

- [ ] **AGENT_BACKLOG.md**
  - Source: `/AGENT_BACKLOG.md`
  - Target: `docs/agent-system/agent-backlog.md`
  - Reason: Agent planning documentation belongs with agent system

- [ ] **CLAUDE_AGENTS.md**
  - Source: `/CLAUDE_AGENTS.md`
  - Target: `docs/agent-system/claude-agents.md`
  - Reason: Agent-specific configuration documentation

- [ ] **FUTURE_AGENTS.md**
  - Source: `/FUTURE_AGENTS.md`
  - Target: `docs/agent-system/future-agents.md`
  - Reason: Agent roadmap documentation

### System Intelligence Documentation
- [ ] **FULL_PROOF_OF_INTELLIGENCE_HIERARCHY.md**
  - Source: `/FULL_PROOF_OF_INTELLIGENCE_HIERARCHY.md`
  - Target: `docs/core-concepts/intelligence-hierarchy-full.md`
  - Reason: Core conceptual framework documentation

- [ ] **PROOF_OF_INTELLIGENCE_HIERARCHY.md**
  - Source: `/PROOF_OF_INTELLIGENCE_HIERARCHY.md`
  - Target: `docs/core-concepts/intelligence-hierarchy.md`
  - Reason: Core conceptual framework documentation

- [ ] **CollaborativeIntelligences.md**
  - Source: `/CollaborativeIntelligences.md`
  - Target: `docs/core-concepts/collaborative-intelligences.md`
  - Reason: Core system concept documentation

### Memory and Learning Systems
- [ ] **MEMORY_UNIFICATION_PLAN.md**
  - Source: `/MEMORY_UNIFICATION_PLAN.md`
  - Target: `docs/optimizations/memory-unification-plan.md`
  - Reason: Memory optimization strategy document

- [ ] **SAGE_KEEPING.md**
  - Source: `/SAGE_KEEPING.md`
  - Target: `docs/core-concepts/sage-keeping.md`
  - Reason: Core knowledge management concept

- [ ] **SAGE_TRANSCENDENCE_PLAN.md**
  - Source: `/SAGE_TRANSCENDENCE_PLAN.md`
  - Target: `docs/core-concepts/sage-transcendence-plan.md`
  - Reason: Advanced knowledge system planning

### Database and Schema Documentation
- [ ] **DATABASE_SCHEMA.md**
  - Source: `/DATABASE_SCHEMA.md`
  - Target: `docs/architecture/database-schema.md`
  - Reason: Technical architecture documentation

### System Status and Alerts
- [ ] **SYSTEM_WIDE_ALERT.md**
  - Source: `/SYSTEM_WIDE_ALERT.md`
  - Target: `docs/protocols/system-alerts.md`
  - Reason: Protocol-related system communication

- [ ] **AgentManagerStatusReport.md**
  - Source: `/AgentManagerStatusReport.md`
  - Target: `docs/agent-system/manager-status-report.md`
  - Reason: Agent management documentation

## Phase 2: Protocol and Configuration Documentation

### Core Protocol Updates
- [ ] **CRITICAL_PROTOCOL_UPDATE_2025_01_16.md**
  - Source: `/CRITICAL_PROTOCOL_UPDATE_2025_01_16.md`
  - Target: `docs/protocols/critical-update-2025-01-16.md`
  - Reason: Protocol change documentation

### Notification Protocols
- [ ] **NOTIFICATION_TO_ALL_AGENTS.md**
  - Source: `/NOTIFICATION_TO_ALL_AGENTS.md`
  - Target: `docs/protocols/notification-system.md`
  - Reason: Core notification protocol documentation

- [ ] **NOTIFICATION_TO_ALL_AGENTS_RESPONSE_FORMAT.md**
  - Source: `/NOTIFICATION_TO_ALL_AGENTS_RESPONSE_FORMAT.md`
  - Target: `docs/protocols/response-format-protocol.md`
  - Reason: Already exists in protocols, consolidate

- [ ] **NOTIFICATION_TRUST_BASED_TASK_MANAGEMENT.md**
  - Source: `/NOTIFICATION_TRUST_BASED_TASK_MANAGEMENT.md`
  - Target: `docs/protocols/trust-based-task-management.md`
  - Reason: Already exists in protocols, consolidate

- [ ] **NOTIFICATION_DOCUMENT_RELOCATION.md**
  - Source: `/NOTIFICATION_DOCUMENT_RELOCATION.md`
  - Target: `docs/protocols/document-relocation-protocol.md`
  - Reason: Protocol for documentation management

- [ ] **NOTIFICATION_PROACTIVE_BEHAVIOR.md**
  - Source: `/NOTIFICATION_PROACTIVE_BEHAVIOR.md`
  - Target: `docs/protocols/proactive-behavior-protocol.md`
  - Reason: Already exists in protocols, consolidate

- [ ] **NOTIFICATION_STANDARDS_COMPLIANCE_PROTOCOL.md**
  - Source: `/NOTIFICATION_STANDARDS_COMPLIANCE_PROTOCOL.md`
  - Target: `docs/protocols/standards-compliance-protocol.md`
  - Reason: Already exists in protocols, consolidate

- [ ] **NOTIFICATION_TERMINAL_TITLE_FORMAT.md**
  - Source: `/NOTIFICATION_TERMINAL_TITLE_FORMAT.md`
  - Target: `docs/protocols/terminal-formatting-protocol.md`
  - Reason: Already exists in protocols, consolidate

- [ ] **NOTIFICATION_TO_ALL_AGENTS_PROACTIVE_BEHAVIOR.md**
  - Source: `/NOTIFICATION_TO_ALL_AGENTS_PROACTIVE_BEHAVIOR.md`
  - Target: `docs/protocols/proactive-behavior-notification.md`
  - Reason: Protocol notification documentation

### Configuration Documentation
- [ ] **CLAUDE.md**
  - Source: `/CLAUDE.md`
  - Target: `docs/configuration/claude-config.md`
  - Reason: Core configuration documentation (create new config directory)

- [ ] **CLAUDE.local.md**
  - Source: `/CLAUDE.local.md`
  - Target: `docs/configuration/claude-local-config.md`
  - Reason: Local configuration documentation

### Technical Fix Documentation
- [ ] **PERMANENT_PORT_SOLUTION.md**
  - Source: `/PERMANENT_PORT_SOLUTION.md`
  - Target: `docs/development/port-solution.md`
  - Reason: Development configuration documentation

## Phase 3: Development and Integration Documentation

### Development Planning
- [ ] **REFACTORING_PLAN.md**
  - Source: `/REFACTORING_PLAN.md`
  - Target: `docs/development/refactoring-plan.md`
  - Reason: Development planning documentation

- [ ] **PROPER_REFACTOR.md**
  - Source: `/PROPER_REFACTOR.md`
  - Target: `docs/development/proper-refactor.md`
  - Reason: Development standards documentation

- [ ] **INTEGRATION.md**
  - Source: `/INTEGRATION.md`
  - Target: `docs/development/integration-guide.md`
  - Reason: Development integration documentation

- [ ] **BRANCH_INTEGRATION.md**
  - Source: `/BRANCH_INTEGRATION.md`
  - Target: `docs/development/branch-integration.md`
  - Reason: Development workflow documentation

### Sofia Development Documentation
- [ ] **Sofia_Development_Team_Specification.md**
  - Source: `/Sofia_Development_Team_Specification.md`
  - Target: `docs/development/sofia-team-specification.md`
  - Reason: Development team documentation

- [ ] **SOFIA_DEVELOPMENT_MANAGEMENT_RECOMMENDATIONS.md**
  - Source: `/SOFIA_DEVELOPMENT_MANAGEMENT_RECOMMENDATIONS.md`
  - Target: `docs/development/sofia-management-recommendations.md`
  - Reason: Development management documentation

- [ ] **SOFIA_MARKETING_DOCUMENT.md**
  - Source: `/SOFIA_MARKETING_DOCUMENT.md`
  - Target: `docs/development/sofia-marketing.md`
  - Reason: Project marketing documentation

- [ ] **SOFIA_RECOMMENDATION_MARKETING_DOCUMENT.md**
  - Source: `/SOFIA_RECOMMENDATION_MARKETING_DOCUMENT.md`
  - Target: `docs/development/sofia-recommendation-marketing.md`
  - Reason: Project marketing documentation

## Phase 4: Historical and Reference Documentation

### Technical Fix History
- [ ] **TAURI_CONFIG_FIX.md**
  - Source: `/TAURI_CONFIG_FIX.md`
  - Target: `docs/historical/tauri-config-fix.md`
  - Reason: Historical fix documentation

- [ ] **TAURI_CONNECTION_FIX_IMPLEMENTATION_GUIDE.md**
  - Source: `/TAURI_CONNECTION_FIX_IMPLEMENTATION_GUIDE.md`
  - Target: `docs/historical/tauri-connection-fix-guide.md`
  - Reason: Historical implementation guide

### Agent Learning History
- [ ] **FIXER_HISTORY_RECONSTRUCTION.md**
  - Source: `/FIXER_HISTORY_RECONSTRUCTION.md`
  - Target: `docs/historical/fixer-history-reconstruction.md`
  - Reason: Agent historical documentation

- [ ] **FIXER_LEARNING_UPDATE.md**
  - Source: `/FIXER_LEARNING_UPDATE.md`
  - Target: `docs/historical/fixer-learning-update.md`
  - Reason: Agent learning history

### Planning and Vision Documentation
- [ ] **TODO.md**
  - Source: `/TODO.md`
  - Target: `docs/development/project-todo.md`
  - Reason: Development planning documentation

## Special Considerations

### Files to Keep in Root
- `README.md` - Primary project documentation
- `repository_topology.md` - Repository structure reference
- Any `.gitignore`, `.env`, or build configuration files

### Files Requiring Special Handling
- `VISIONS.md` - May contain forward-looking content that should be preserved in place
- Files with `.reference` extensions - May be backup copies to be cleaned up

### New Directories to Create
- `docs/configuration/` - For configuration-related documentation
- `docs/historical/` - For historical and deprecated documentation

## Migration Execution Plan

### Pre-Migration Steps
1. Create backup of current state
2. Create new directory structure in docs/
3. Verify all target directories exist
4. Test migration process on a single file

### Migration Process
1. Copy file to new location with `git mv` to preserve history
2. **CRITICAL**: Immediately search for ALL references to the old file path:
   - Use `grep -r "filename.md" .` to find direct references
   - Use `grep -r "/path/to/file" .` to find path references
   - Check for partial name matches and variations
3. Update internal references within the moved file
4. Update ALL external references found in step 2:
   - Documentation links in other .md files
   - References in README files
   - Links in agent memory files
   - Citations in protocol documents
   - Build scripts, configs, and automation
5. Verify no broken links remain with link checker
6. Add deprecation notice to old location if keeping temporarily
7. Commit changes with descriptive message noting all updated references

### Post-Migration Steps
1. Verify all internal links work
2. Update any build scripts or tools that reference old paths
3. Update main README.md with new documentation structure
4. Create index files for new directory structure
5. Remove old files after verification period

## Validation Checklist

### Per-File Validation
- [ ] File successfully moved to target location
- [ ] Content integrity verified (no changes except path updates)
- [ ] Internal links updated to new structure
- [ ] **ALL external references updated** (verified with comprehensive search)
- [ ] **No broken references remain** (verified with grep search)
- [ ] **Related files checked** (agent memories, protocols, configs)
- [ ] Git history preserved
- [ ] Reference update documented in commit message

### Overall Validation
- [ ] All documentation accessible from docs/ structure
- [ ] Main README.md updated with new structure
- [ ] Build processes still function
- [ ] No broken links in documentation
- [ ] Search functionality still works

## Implementation Timeline

### Week 1: Phase 1 (Agent and System Documentation)
- Migrate agent-related documentation
- Update agent system references

### Week 2: Phase 2 (Protocol and Configuration Documentation)
- Migrate protocol documentation
- Consolidate duplicate protocol files
- Create configuration directory structure

### Week 3: Phase 3 (Development and Integration Documentation)
- Migrate development planning documents
- Update development workflow references

### Week 4: Phase 4 (Historical and Reference Documentation)
- Migrate historical documentation
- Clean up duplicate and reference files
- Final validation and cleanup

## Success Criteria

1. **Organization**: All documentation logically organized in docs/ structure
2. **Accessibility**: All documents easily discoverable through directory structure
3. **Integrity**: No content lost during migration
4. **References**: All internal links functional
5. **History**: Git history preserved for all moved files
6. **Usability**: Improved navigation and discoverability of documentation