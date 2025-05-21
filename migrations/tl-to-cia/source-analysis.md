# TL to CIA Source Analysis

## Key Differences Identified

### Cargo.toml
**TL version:**
- name: "tl-cli"
- binary name: "tl" 
- description: "Topologist CLI for repository management"
- Missing: `glob` dependency
- Missing: `clap` derive features

**CIA version:**
- name: "cia-cli"
- binary name: "cia"
- description: "Collaborative Intelligence Adaption CLI for repository intelligence adaption"
- Has: `glob = "0.3.0"` ✅
- Has: `clap = { version = "3.2.22", features = ["derive"] }` ✅

### Main.rs Structure
Both have identical command structure:
- Agent commands
- Memory commands  
- Knowledge commands
- Debug commands
- System commands

**Key text replacements needed:**
- "TL CLI" → "CIA CLI"
- "Teaching/Learning" → "Collaborative Intelligence Adaption"
- "repository maintenance" → "repository intelligence adaption"
- "TL" → "CIA" (in clap name)

## Migration Strategy

### Phase 1: Core Files ✅ (Dependencies fixed)
- Cargo.toml dependencies aligned

### Phase 2: Source Code Migration
1. **main.rs**: Update CLI name and descriptions
2. **commands/**: Copy all command modules from TL
3. **helpers/**: Copy helper modules
4. **utils/**: Copy utility modules
5. **config/**: Copy configuration handling
6. **error.rs**: Copy error handling (already exists in CIA)

### Phase 3: Documentation
1. **README.md**: Update from TL context to CIA
2. **DESIGN_GUIDE.md**: Adapt design principles
3. **VISUAL_DESIGN_SYSTEM.md**: Update branding

### Phase 4: Build System
1. **build.rs**: Copy and adapt build script

## Files to Migrate
- ✅ src/main.rs (update text)
- ⏳ src/commands/ (copy all from TL)
- ⏳ src/helpers/ (copy all from TL)  
- ⏳ src/utils/ (copy all from TL)
- ⏳ src/config/ (copy from TL)
- ✅ src/error.rs (already exists)
- ⏳ README.md (adapt)
- ⏳ DESIGN_GUIDE.md (adapt)
- ⏳ build.rs (copy)

## Critical Replacements Needed
1. All "TL" → "CIA"
2. All "tl" → "cia" (in paths/commands)
3. "Teaching/Learning" → "Collaborative Intelligence Adaption"
4. "Topologist" → "Collaborative Intelligence Adaption"
5. Update any CLI references in help text