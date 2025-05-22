# Terminal Title Tracking System Architecture

## Overview

This document provides the architectural design for implementing comprehensive terminal title tracking in the CIA CLI system. The system will automatically update terminal titles to reflect current project context, active agent, and task progress.

## Current State Analysis

### Existing Implementation
- **Location**: `CLIs/CIA/src/helpers/terminal.rs:84`
- **Functionality**: Static title setting using ANSI escape sequences
- **Current Format**: `CIA: {title}` 
- **Limitations**: No task tracking, no dynamic updates, no context awareness

### Protocol Requirements
- **Standard Format**: `[Project/Directory] | [ActiveAgent] | [CurrentTask]`
- **Update Triggers**: Directory changes, agent activation, task changes
- **Existing Script**: `CLIs/ci/bash/update-terminal-title.sh`

## Architecture Design

### 1. Core Components

#### A. Terminal Title Manager
```rust
// Location: CLIs/CIA/src/terminal/title_manager.rs
pub struct TerminalTitleManager {
    current_project: String,
    current_agent: Option<String>,
    current_task: Option<String>,
    update_enabled: bool,
}

impl TerminalTitleManager {
    pub fn new() -> Self;
    pub fn update_project(&mut self, project: &str);
    pub fn update_agent(&mut self, agent: &str);
    pub fn update_task(&mut self, task: &str);
    pub fn clear_task(&mut self);
    pub fn refresh_title(&self);
    pub fn get_current_title(&self) -> String;
}
```

#### B. Task Context Tracker
```rust
// Location: CLIs/CIA/src/terminal/task_tracker.rs
pub struct TaskTracker {
    active_tasks: Vec<TaskContext>,
    current_focus: Option<String>,
}

pub struct TaskContext {
    id: String,
    description: String,
    agent: String,
    start_time: SystemTime,
    status: TaskStatus,
}

pub enum TaskStatus {
    Active,
    Paused,
    Completed,
}
```

#### C. Context Monitor
```rust
// Location: CLIs/CIA/src/terminal/context_monitor.rs
pub struct ContextMonitor {
    last_directory: PathBuf,
    last_agent: Option<String>,
    title_manager: Arc<Mutex<TerminalTitleManager>>,
}

impl ContextMonitor {
    pub fn start_monitoring(&mut self);
    pub fn check_for_changes(&mut self);
    pub fn on_directory_change(&mut self, new_dir: &Path);
    pub fn on_agent_change(&mut self, agent: &str);
}
```

### 2. Integration Points

#### A. Agent Activation Enhancement
```rust
// Modify: CLIs/CIA/src/commands/agent.rs:activate()
pub fn activate(name: &str, config: &Config) -> Result<()> {
    // ... existing code ...
    
    // Enhanced terminal title update
    let mut title_manager = get_title_manager();
    title_manager.update_agent(name);
    title_manager.update_project(&get_project_name(&config.repo_path));
    title_manager.refresh_title();
    
    // ... rest of function ...
}
```

#### B. Command Execution Hooks
```rust
// Add to: CLIs/CIA/src/main.rs
fn main() -> Result<()> {
    // Initialize title tracking
    let title_manager = TerminalTitleManager::new();
    title_manager.update_project(&get_current_project());
    
    // Start context monitoring
    let mut context_monitor = ContextMonitor::new(title_manager);
    context_monitor.start_monitoring();
    
    // ... existing main logic ...
}
```

#### C. Task Command Integration
```rust
// New file: CLIs/CIA/src/commands/task.rs
pub fn start_task(description: &str, config: &Config) -> Result<()> {
    let mut title_manager = get_title_manager();
    title_manager.update_task(description);
    title_manager.refresh_title();
    
    CommandHelpers::print_success(&format!("Started task: {}", description));
    Ok(())
}

pub fn complete_task(config: &Config) -> Result<()> {
    let mut title_manager = get_title_manager();
    title_manager.clear_task();
    title_manager.refresh_title();
    
    CommandHelpers::print_success("Task completed");
    Ok(())
}
```

### 3. Configuration System

#### A. Title Format Configuration
```rust
// Add to: CLIs/CIA/src/config/mod.rs
#[derive(Debug, Clone)]
pub struct TitleConfig {
    pub format_template: String,
    pub update_enabled: bool,
    pub project_detection: ProjectDetectionMethod,
    pub task_timeout_minutes: Option<u32>,
}

impl Default for TitleConfig {
    fn default() -> Self {
        Self {
            format_template: "[{project}] | [{agent}] | [{task}]".to_string(),
            update_enabled: true,
            project_detection: ProjectDetectionMethod::DirectoryName,
            task_timeout_minutes: Some(60),
        }
    }
}

pub enum ProjectDetectionMethod {
    DirectoryName,
    GitRepository,
    ConfigFile,
}
```

#### B. Global State Management
```rust
// Add to: CLIs/CIA/src/state/mod.rs
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

static TITLE_MANAGER: Lazy<Arc<Mutex<TerminalTitleManager>>> = 
    Lazy::new(|| Arc::new(Mutex::new(TerminalTitleManager::new())));

pub fn get_title_manager() -> Arc<Mutex<TerminalTitleManager>> {
    TITLE_MANAGER.clone()
}
```

## Implementation Plan

### Phase 1: Core Infrastructure (2-3 hours)

#### 1.1 Create Terminal Module Structure
- [ ] Create `CLIs/CIA/src/terminal/mod.rs`
  - [ ] Add module declarations for title_manager, task_tracker, context_monitor
  - [ ] Re-export public interfaces
  - [ ] Add module-level documentation
- [ ] Create `CLIs/CIA/src/terminal/title_manager.rs`
  - [ ] Define TerminalTitleManager struct
  - [ ] Implement basic constructor and state management
  - [ ] Add project/agent/task update methods
  - [ ] Implement title formatting and ANSI escape sequences
- [ ] Create `CLIs/CIA/src/terminal/task_tracker.rs`
  - [ ] Define TaskTracker and TaskContext structs
  - [ ] Implement task lifecycle management
  - [ ] Add task status tracking (Active, Paused, Completed)
  - [ ] Create task timeout handling

#### 1.2 Implement TerminalTitleManager Core
- [ ] **TerminalTitleManager struct implementation**
  - [ ] Add fields: current_project, current_agent, current_task, update_enabled
  - [ ] Implement `new()` constructor with sensible defaults
  - [ ] Add `update_project(&mut self, project: &str)` method
  - [ ] Add `update_agent(&mut self, agent: &str)` method
  - [ ] Add `update_task(&mut self, task: &str)` method
  - [ ] Add `clear_task(&mut self)` method
  - [ ] Implement `get_current_title(&self) -> String` with format `[{project}] | [{agent}] | [{task}]`
  - [ ] Add `refresh_title(&self)` method using ANSI escape sequences
- [ ] **Thread-safe access patterns**
  - [ ] Add Arc<Mutex<T>> wrapper support
  - [ ] Implement global state management
  - [ ] Create helper functions for safe access
- [ ] **Title formatting**
  - [ ] Handle empty/None values gracefully
  - [ ] Implement configurable format templates
  - [ ] Add length limits for terminal compatibility

#### 1.3 Update Existing Terminal Helpers
- [ ] **Modify `src/helpers/terminal.rs`**
  - [ ] Update `set_terminal_title()` to use TerminalTitleManager
  - [ ] Maintain backward compatibility with direct title setting
  - [ ] Add deprecation warnings for direct usage
- [ ] **Update `src/helpers/command.rs`**
  - [ ] Ensure CommandHelpers::set_terminal_title delegates correctly
  - [ ] Maintain existing API surface
- [ ] **Add integration points**
  - [ ] Create helper functions for common title operations
  - [ ] Add error handling for terminal title failures

### Phase 2: Agent Integration (1-2 hours)

#### 2.1 Enhance Agent Activation
- [ ] **Update `commands/agent.rs:activate()` function**
  - [ ] Import TerminalTitleManager and global state helpers
  - [ ] Add automatic project detection using `get_project_name()`
  - [ ] Update title with agent name on activation
  - [ ] Integrate with existing agent activation flow
  - [ ] Add error handling for title update failures
- [ ] **Implement project detection**
  - [ ] Add `get_project_name()` function in `helpers/path.rs`
  - [ ] Support directory name detection
  - [ ] Support git repository name detection
  - [ ] Support config file detection
  - [ ] Add fallback to "CollaborativeIntelligence"

#### 2.2 Add Task Management Commands
- [ ] **Create `commands/task.rs`**
  - [ ] Add `start_task(description: &str, config: &Config) -> Result<()>`
  - [ ] Add `complete_task(config: &Config) -> Result<()>`
  - [ ] Add `status_task(config: &Config) -> Result<()>`
  - [ ] Add `list_tasks(config: &Config) -> Result<()>`
  - [ ] Integrate with TerminalTitleManager for title updates
  - [ ] Add task persistence to configuration
- [ ] **Update CLI argument parsing in `main.rs`**
  - [ ] Add "task" subcommand to CLI
  - [ ] Add task start, complete, status, list sub-subcommands
  - [ ] Add help text and usage examples
  - [ ] Wire up command routing

### Phase 3: Context Monitoring (2-3 hours)

#### 3.1 Implement Context Monitoring
- [ ] **Create `terminal/context_monitor.rs`**
  - [ ] Define ContextMonitor struct with directory tracking
  - [ ] Implement `start_monitoring(&mut self)` with background thread
  - [ ] Add `check_for_changes(&mut self)` for periodic updates
  - [ ] Implement `on_directory_change(&mut self, new_dir: &Path)`
  - [ ] Implement `on_agent_change(&mut self, agent: &str)`
  - [ ] Add graceful shutdown handling
- [ ] **Directory change detection**
  - [ ] Monitor current working directory
  - [ ] Detect project context changes
  - [ ] Update title automatically on directory changes
  - [ ] Handle network drives and special filesystems
- [ ] **Agent switching detection**
  - [ ] Track active agent state
  - [ ] Detect agent activation commands
  - [ ] Update title on agent transitions

#### 3.2 Add Configuration System
- [ ] **Extend `config/mod.rs` with TitleConfig**
  - [ ] Add TitleConfig struct with format template, enable flag, detection method
  - [ ] Implement Default trait with sensible defaults
  - [ ] Add ProjectDetectionMethod enum (DirectoryName, GitRepository, ConfigFile)
  - [ ] Integrate with existing config loading
- [ ] **Configuration file support**
  - [ ] Add terminal title section to config files
  - [ ] Support format customization
  - [ ] Add enable/disable toggle
  - [ ] Support task timeout configuration

### Phase 4: Advanced Features (1-2 hours)

#### 4.1 Task Timeout Handling
- [ ] **Implement automatic task clearing**
  - [ ] Add configurable timeout (default 60 minutes)
  - [ ] Create background timer for task expiration
  - [ ] Clear task from title on timeout
  - [ ] Add user notification for task clearing
- [ ] **Task persistence across sessions**
  - [ ] Save active task to config/state file
  - [ ] Restore task on CLI startup
  - [ ] Handle stale task detection
  - [ ] Add manual task clear command

#### 4.2 Integration with Existing Scripts
- [ ] **Compatibility with `update-terminal-title.sh`**
  - [ ] Test existing script with new system
  - [ ] Add migration helper functions
  - [ ] Document compatibility layer
  - [ ] Provide upgrade path
- [ ] **Migration path for existing users**
  - [ ] Add feature flag for gradual rollout
  - [ ] Create migration documentation
  - [ ] Test with existing workflows
  - [ ] Provide rollback capability

## Testing Checklist

### Unit Testing
- [ ] **TerminalTitleManager tests**
  - [ ] Test title formatting with various inputs
  - [ ] Test state updates and retrieval
  - [ ] Test thread safety with concurrent access
  - [ ] Test configuration integration
- [ ] **TaskTracker tests**
  - [ ] Test task lifecycle management
  - [ ] Test timeout handling
  - [ ] Test persistence functionality
- [ ] **ContextMonitor tests**
  - [ ] Test directory change detection
  - [ ] Test agent change tracking
  - [ ] Test automatic updates

### Integration Testing
- [ ] **CLI command tests**
  - [ ] Test agent activation with title updates
  - [ ] Test task management commands
  - [ ] Test configuration loading
  - [ ] Test error handling
- [ ] **Cross-platform compatibility**
  - [ ] Test on macOS, Linux, Windows
  - [ ] Test various terminal emulators
  - [ ] Test ANSI escape sequence support

### Manual Testing
- [ ] **End-to-end workflow testing**
  - [ ] Test complete agent activation flow
  - [ ] Test task start/complete cycle
  - [ ] Test directory change detection
  - [ ] Test error recovery scenarios

## Technical Specifications

### Dependencies
```toml
# Add to CLIs/CIA/Cargo.toml
[dependencies]
once_cell = "1.19"
tokio = { version = "1.0", features = ["time", "sync"] }
```

### File Structure
```
CLIs/CIA/src/
├── terminal/
│   ├── mod.rs
│   ├── title_manager.rs
│   ├── task_tracker.rs
│   └── context_monitor.rs
├── commands/
│   ├── task.rs              # New
│   └── agent.rs             # Modified
├── config/
│   └── mod.rs               # Modified
└── state/
    └── mod.rs               # New
```

### Error Handling
- Graceful degradation if terminal doesn't support title setting
- Fallback to existing behavior on errors
- Comprehensive logging for debugging

### Testing Strategy
- Unit tests for each component
- Integration tests for CLI commands
- Mock terminal for testing title updates
- Cross-platform compatibility testing

## Backward Compatibility

### Existing Behavior Preservation
- Current `set_terminal_title()` function remains functional
- Existing scripts continue to work
- No breaking changes to public APIs

### Migration Path
1. Default behavior matches current implementation
2. New features are opt-in initially
3. Gradual rollout with feature flags

## Future Enhancements

### Planned Extensions
1. **Session Persistence**: Save/restore title state across sessions
2. **Multi-Project Support**: Handle multiple concurrent projects
3. **Visual Indicators**: Progress bars or status indicators in title
4. **Integration Hooks**: API for external tools to update context

### Performance Considerations
- Minimal overhead for title updates
- Efficient change detection
- Configurable update intervals
- Memory-conscious state management

## Validation Criteria

### Success Metrics
1. **Functionality**
   - Terminal title updates automatically on agent activation
   - Project context detected correctly
   - Task descriptions appear in title
   - Format matches protocol specification

2. **Performance**
   - No noticeable CLI performance impact
   - Title updates complete within 100ms
   - Memory usage remains minimal

3. **Reliability**
   - No crashes or errors during normal operation
   - Graceful handling of edge cases
   - Cross-platform compatibility maintained

4. **User Experience**
   - Clear visual feedback for title changes
   - Intuitive task management commands
   - Helpful error messages and documentation

---

**Document Status**: Ready for Implementation  
**Priority**: High  
**Estimated Effort**: 6-10 hours  
**Target Engineer**: Implementation Specialist  

**Dependencies**: None (self-contained enhancement)  
**Testing Requirements**: Unit tests, integration tests, manual testing across platforms  
**Documentation Requirements**: User guide, CLI help updates, protocol compliance verification