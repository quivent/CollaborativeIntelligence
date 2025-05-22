# BASH SCRIPTS INVENTORY

A comprehensive catalog of all bash scripts in the Collaborative Intelligence system.

## Core CLI Tools
- **CI** - Main wrapper for the Collaborative Intelligence tool (external system)
- **claude-init** - Script to initialize Claude with CI system
- **terminal-launcher.sh** - Script to launch the terminal with CI integration
- **integrate-ci.sh** - Script to integrate CI into existing projects

## Agent Management Scripts
- **AGENTS/Manager/scripts/create-agent.sh** - Creates a new agent with proper structure and files
- **AGENTS/Manager/scripts/batch-create-agents.sh** - Creates multiple agents from a JSON configuration file
- **AGENTS/Manager/scripts/validate-agent.sh** - Validates agent name and checks for similarity before creation
- **AGENTS/Manager/scripts/quick-agent.sh** - Simplified agent creation script
- **AGENTS/Manager/scripts/intuitive-create.sh** - Natural language agent creation interface
- **AGENTS/Manager/generate-index.sh** - Generates a comprehensive agent index for analysis
- **AGENTS/Streamliner/scripts/create-agent.sh** - Optimized agent creation script (60% faster)

## CI Tools Core Scripts
- **ci-tools/CI** - Main CI tool implementation
- **ci-tools/install.sh** - Installation script for CI tools
- **ci-tools/lib/common.sh** - Common functions for CI tools
- **ci-tools/lib/config.sh** - Configuration handling for CI system
- **ci-tools/lib/output.sh** - Output formatting utilities
- **ci-tools/lib/claude-integrity.sh** - Ensures Claude memory integrity
- **ci-tools/lib/memory-loader.sh** - Handles agent memory loading
- **ci-tools/lib/agent-activator.sh** - Activates agents in the CI system
- **ci-tools/lib/wellness-integrator.sh** - Integrates wellness monitoring
- **ci-tools/cmd/agent.sh** - Agent management commands
- **ci-tools/cmd/fix.sh** - System fix commands
- **ci-tools/cmd/init.sh** - Initialization commands
- **ci-tools/cmd/session.sh** - Session management commands
- **ci-tools/cmd/verify.sh** - Verification commands
- **ci-tools/activate-agent.sh** - Script to activate specific agents
- **ci-tools/enforce-agent-format.sh** - Enforces agent format standards
- **ci-tools/update-terminal-title.sh** - Updates terminal title based on context
- **ci-tools/validate-config.sh** - Validates CI configuration

## Repository Restructuring Scripts
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/execute-restructuring.sh** - Main restructuring execution script
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/phase1-directory-standardization.sh** - Phase 1: Directory standardization
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/phase2-resource-consolidation.sh** - Phase 2: Resource consolidation
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/phase3-project-organization.sh** - Phase 3: Project organization
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/phase4-special-resources.sh** - Phase 4: Special resources handling
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/phase5-validation-finalization.sh** - Phase 5: Validation and finalization
- **AGENTS/Cartographer/Sessions/RepositoryRestructuring/phase5-validation.sh** - Phase 5: Validation

## Database Scripts
- **data/ci-database/init.sh** - Initializes the CI database
- **data/ci-database/init-rust.sh** - Initializes the Rust-based CI database
- **data/ci-database/migrate.sh** - Database migration script
- **data/ci-database/test.sh** - Tests database functionality
- **data/db-cli/build-and-test.sh** - Builds and tests the database CLI
- **data/db-cli/install.sh** - Installs the database CLI

## Plugin Scripts
- **claude-cli-plugin/install.sh** - Installs the Claude CLI plugin
- **claude-cli-plugin/claude-init-impl.sh** - Implementation for Claude initialization

## Building and Testing
- **scripts/build.sh** - Main build script
- **scripts/build_cli.sh** - Builds the CLI tools
- **scripts/agent_switch.sh** - Switches between agents
- **tauri-test-script/run-tauri.sh** - Runs Tauri tests

## Wellness Monitoring
- **AGENTS/Wellness/Toolkit/wellness_service.sh** - Wellness monitoring service
- **AGENTS/Wellness/Toolkit/wellness_trigger.sh** - Triggers wellness checks
- **AGENTS/Wellness/Toolkit/activity_monitor.sh** - Monitors user activity

## Knowledge Management
- **scripts/knowledge/scheduled_workflow.sh** - Handles scheduled knowledge workflows
- **KnowledgeRegistry/search_cli.sh** - CLI for searching the knowledge registry
- **scripts/package_kanji_results.sh** - Packages Kanji processing results
- **Kanji/scripts/package_kanji_results.sh** - Packages Kanji processing results

## Protocol Management
- **Protocols/Templates/compliance-checker.sh** - Checks protocol compliance
- **Protocols/Templates/session-creation.sh** - Creates protocol sessions
- **Protocols/Compliance/protocol_feedback.sh** - Handles protocol feedback
- **AGENTS/Scholar/Sessions/TerminalTitleStandardization/protocol_reminder.sh** - Reminds about terminal title protocol

## Implementation Solutions
- **implemented-solutions/append-manager-import.sh** - Appends Manager import statements
- **implemented-solutions/direct-file-edit.sh** - Performs direct file edits
- **implemented-solutions/fix-unused-imports.sh** - Fixes unused imports
- **implemented-solutions/port-fix-runner.sh** - Runs port fixes
- **implemented-solutions/replace-path-imports.sh** - Replaces path imports
- **implemented-solutions/run-terminal.sh** - Runs terminal with proper configuration
- **implemented-solutions/rust-fix-runner.sh** - Runs Rust fixes
- **implemented-solutions/update-files.sh** - Updates files with required changes
- **implemented-solutions/verify-build.sh** - Verifies build integrity

## Markdown Design Tools
- **SYSTEMS/MARKDOWN_DESIGN/scripts/convert.sh** - Converts markdown to HTML
- **SYSTEMS/MARKDOWN_DESIGN/scripts/convert-enhanced.sh** - Enhanced markdown conversion

---

*Last Updated: 2025-05-19*  
*Author: Basher*  
*Documentation Category: System Inventory*