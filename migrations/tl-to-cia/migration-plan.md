# TL to CIA Migration Plan

## Overview
Migrating the TL (Teaching Learning) CLI to CIA (Collaborative Intelligence Adaption) CLI

**Target**: `/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLIs/CIA/`
**Source**: `/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/CLIs/TL/`

## Migration Status
- **Status**: In Progress
- **Started**: 2025-05-22
- **Phase**: Initial Assessment Complete

## TL CLI Structure Assessment
```
TL/
├── Cargo.lock
├── Cargo.toml
├── DESIGN_GUIDE.md
├── README.md
├── VISUAL_DESIGN_SYSTEM.md
├── build.rs
├── src/
│   ├── bin/
│   ├── commands.rs
│   ├── commands/
│   │   ├── agent.rs + agent/
│   │   ├── debug.rs + debug/
│   │   ├── knowledge.rs + knowledge/
│   │   ├── memory.rs + memory/
│   │   └── system.rs + system/
│   ├── config/mod.rs
│   ├── error.rs
│   ├── helpers/
│   │   ├── command.rs
│   │   ├── execution.rs
│   │   ├── mod.rs
│   │   ├── path.rs
│   │   ├── repository.rs
│   │   └── terminal.rs
│   ├── main.rs
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── path.rs
│   │   └── terminal.rs
│   └── target/
```

## Current Issues
1. CIA CLI already exists but has compilation errors
2. Missing dependencies (glob crate)
3. Clap version mismatch (v3 vs v4)
4. Various type mismatches in function calls

## Migration Phases

### Phase 1: Fix Existing CIA CLI ✅ (In Progress)
- [ ] Fix compilation errors
- [ ] Update dependencies
- [ ] Resolve type mismatches

### Phase 2: Content Migration
- [ ] Copy and adapt TL source files to CIA
- [ ] Update all references from TL to CIA
- [ ] Update documentation and guides

### Phase 3: Configuration Updates
- [ ] Update build scripts
- [ ] Update Cargo.toml metadata
- [ ] Update any external references

### Phase 4: Testing & Validation
- [ ] Functional testing
- [ ] Integration testing
- [ ] Documentation validation

### Phase 5: Cleanup
- [ ] Archive TL CLI
- [ ] Update any scripts or references
- [ ] Final verification

## Next Steps
1. Fix compilation errors in existing CIA CLI
2. Analyze TL source files for complete migration mapping
3. Execute systematic migration