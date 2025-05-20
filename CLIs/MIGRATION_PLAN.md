# CLI Migration and Consolidation Plan

## Current Status

The Collaborative Intelligence system currently has multiple CLI implementations with naming inconsistencies and unclear separation of concerns:

1. **Internal CLI Tools (CollaborativeIntelligence/ci-tools)**: Bash-based implementation for internal project use
2. **Minimal Rust CLI (CollaborativeIntelligence/ci-rust-minimal)**: Partial Rust implementation
3. **Main CI Implementation (../CI)**: Advanced Rust CLI for integration with other projects
4. **Deprecated CI_D (../CI_D)**: Legacy implementation still referenced
5. **CIR References**: Inconsistent renaming from CI to CIR when porting to Rust
6. **Legacy Implementations**: Various deprecated scripts in legacy/ directories
   - `/agent_cache/legacy/agent-activator.sh`
   - `/scripts/legacy/memory-loader.sh`
   - `/scripts/legacy/rust-memory-bridge.sh`
   - `/ci-rust-minimal/legacy/CI` and `/ci-rust-minimal/legacy/CI.refactored`
7. **Various specialized CLIs**: Memory management, agent cache, knowledge registry

## Migration Goals

1. **Separate External and Internal Concerns**:
   - Maintain "../CI" as the primary external Rust CLI for project integration with other repositories
   - Establish a new "TL" (Teaching/Learning) CLI for internal repository maintenance
   - Ensure clear naming separation between the two

2. **Fix Naming Inconsistencies**:
   - Revert all "CIR" references back to "CI" for the external CLI
   - Remove deprecated implementations (../CI_D, ../deprecated)
   - Create a consistent naming scheme for all CLI components

3. **Consolidate Functionality**:
   - Merge duplicated functionality
   - Standardize interfaces and command patterns
   - Remove deprecated implementations
   - Preserve styling and formatting across both CLIs

4. **Complete Rust Migration**:
   - Convert remaining Bash scripts to Rust
   - Implement a consistent error handling and logging strategy
   - Use modern Rust practices and crates

## Components to Cherry-Pick from ../CI

Based on analysis of the codebases, these components should be incorporated into the ../CI implementation:

### 1. Command Structure and Organization

The main CI implementation uses a superior command categorization system:
- **Intelligence Commands**: Agent and memory management
- **Source Control Commands**: Repository and version management
- **Lifecycle Commands**: Project creation and management
- **System Commands**: Configuration and maintenance

This structure should be preserved and enhanced with specialized functionality from CollaborativeIntelligence.

### 2. Terminal Styling and Output Formatting

The CollaborativeIntelligence repository has superior terminal styling elements that should be preserved:

- **ANSI Color Codes**: RED, GREEN, YELLOW, BLUE, MAGENTA, CYAN definitions with proper terminal detection
- **Formatted Output Functions**: Utility functions for consistent success/error/warning messages
- **Progress Indicators**: Spinners and step tracking for long-running operations
- **Agent Response Formatting**: Standardized agent response formatting with bracketed headers
- **Multiple Output Formats**: JSON, plaintext, and Markdown output options

### 3. Memory Management System

The CollaborativeIntelligence project has advanced memory management that should be integrated:
- **Memory Segmentation**: From `memory_segment_parser.rs` 
- **Checksum Caching**: From `memory_checksum_cache.rs`
- **Unified CLI Interface**: From `unified_memory_cli.rs`

These should be cherry-picked as a complete module to replace any simplified memory management in ../CI.

### 4. Agent Cache System

The agent_cache implementation provides critical functionality:
- **Activation Tracking**: Maintain an audit trail of agent activations
- **Stateful Management**: Track active agents and their contexts
- **Performance Optimizations**: In-memory caching with disk persistence

This system should become a core component of the ../CI implementation.

### 5. Knowledge Registry Integration

The knowledge registry CLI provides powerful search and knowledge management:
- **Category-based Search**: Find knowledge by category
- **Tag-based Search**: Find knowledge by topic
- **Text Search**: Full text search capabilities

These features should be incorporated into the ../CI implementation's Intelligence command group.

## Implementation Strategy

1. **Phase 1: Clear Separation and Naming Correction**
   - Fix all "CIR" references back to "CI" in the external CLI
   - Create a distinct separation between external CLI (CI) and internal CLI (TL)
   - Define command boundaries between the two tools:
     - **CI**: Project integration, external repository management, lifecycle commands
     - **TL**: Internal maintenance, agent management, memory operations
   - Establish folder structures for each CLI

2. **Phase 2: Core Structure Alignment**
   - Create consistent command structure across both CLIs
   - Standardize error handling and logging
   - Implement common configuration management
   - Consolidate styling and display elements
   - Create shared utility libraries

3. **Phase 3: Integration and Cherry-Picking**
   - Integrate memory management functionality into ../CI
   - Incorporate agent cache system
   - Add knowledge registry integration
   - Create proper modular organization

4. **Phase 4: Rust Migration**
   - Convert remaining Bash scripts to Rust
   - Implement complete test coverage
   - Ensure backward compatibility
   - Enforce Rust-only rule for all CLI components

5. **Phase 5: Cleanup and Deprecation**
   - Remove deprecated implementations (../CI_D, ../deprecated)
   - Update documentation
   - Create migration guides for users

## Future Considerations

1. **Plugin Architecture**:
   - Implement a formal plugin system for extensions
   - Support third-party plugins

2. **Configuration Unification**:
   - Single configuration format across all tools
   - Hierarchical configuration with overrides

3. **Enhanced User Experience**:
   - Command completion
   - Interactive mode
   - Guided setup

4. **Cloud Integration**:
   - Support for remote configurations
   - Team synchronization

## Recommendation

Begin with Phase 1 to establish a consistent structure, then proceed to Phase 2 to cherry-pick and integrate the valuable components from CollaborativeIntelligence into the main ../CI implementation. This will unify the functionality while maintaining the separation between external tooling (../CI) and internal utilities (CollaborativeIntelligence/CLIs).