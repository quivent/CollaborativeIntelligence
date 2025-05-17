# Architecture Revision - Separation of Concerns

## Current Problem

The `ci_integration.rs` module creates tight coupling between the database and CI system, which violates separation of concerns:

1. **Database knows about CI**: The database layer shouldn't know about CI-specific operations
2. **Business logic in data layer**: CI workflow logic is mixed with data persistence
3. **Limited reusability**: Other systems can't use the database without CI baggage
4. **Testing complexity**: Need to mock CI behavior to test database

## Better Architecture

### Layer Separation

```
┌─────────────────────┐
│   CI CLI (Rust)     │  <- Business Logic Layer
├─────────────────────┤
│   CI Service Layer  │  <- Orchestration/Integration
├─────────────────────┤
│  Database Library   │  <- Pure Data Layer (ci_database)
└─────────────────────┘
```

### Revised Approach

1. **Database Library (ci_database crate)**
   - Pure data operations
   - No knowledge of CI concepts
   - Generic agent/project/metrics management
   - Reusable by any system

2. **CI Service Layer (in CI project)**
   - Imports ci_database as dependency
   - Implements CI-specific workflows
   - Handles command tracking logic
   - Manages CI session lifecycle

3. **CI CLI**
   - Uses service layer
   - Handles user interaction
   - Routes commands appropriately

## Implementation Changes

### Remove from ci_database:
- `ci_integration.rs` module
- CI-specific helper functions
- Command tracking logic
- CI session management

### Keep in ci_database:
- Pure database operations
- Generic API (agents, projects, metrics)
- Schema and models
- Database optimization
- Advanced features (export, analytics, etc.)

### Move to CI project:
```rust
// In CI project, not database
pub struct CIDatabaseService {
    db: ci_database::CIDatabaseAPI,
}

impl CIDatabaseService {
    pub async fn track_command(&self, project: &str, cmd: &str) -> Result<CommandTracker> {
        // CI-specific logic here
        let operation = self.db.log_operation(...).await?;
        Ok(CommandTracker { operation_id: operation.id })
    }
}
```

## Benefits

1. **Clean separation**: Database remains a pure data layer
2. **Reusability**: Other projects can use the database
3. **Testability**: Can test database without CI context
4. **Flexibility**: CI can change without affecting database
5. **Maintainability**: Clear boundaries between concerns

## Recommended Actions

1. Remove `ci_integration.rs` from database crate
2. Keep database API generic and CI-agnostic
3. Implement CI-specific logic in the CI project
4. Use database as a library dependency in CI

This maintains proper architectural boundaries while still providing all needed functionality.