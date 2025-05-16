# Terminal/Claude Migration Plan

This document outlines the detailed plan for migrating the existing Terminal/Claude codebase to the new standardized structure. The plan follows the principles established in the REPOSITORY_STANDARDIZATION.md document and uses the Reference implementation as a guide for the new structure.

## Phase 1: File Analysis and Mapping

### 1.1 Current Structure Analysis

First, we need to analyze the existing file structure to understand what files need to be moved and how they map to the new structure.

```bash
# Create a file listing of the current codebase structure
find /path/to/Terminal/Claude -type f -name "*.ts*" | sort > current_files.txt

# Analyze component types
grep -l "React.Component\|React.FC\|extends Component" $(find /path/to/Terminal/Claude -type f -name "*.tsx") > component_files.txt

# Analyze context providers
grep -l "createContext\|Context.Provider" $(find /path/to/Terminal/Claude -type f -name "*.ts*") > context_files.txt

# Analyze hook definitions
grep -l "function use[A-Z]" $(find /path/to/Terminal/Claude -type f -name "*.ts*") > hook_files.txt

# Analyze utility functions
grep -l "export function\|export const" $(find /path/to/Terminal/Claude -type f -name "*.ts") | grep -v "use[A-Z]" > utility_files.txt
```

### 1.2 File Categorization

Categorize each file into one of the following groups:

1. **Terminal Components**: Files that contain terminal-related React components
   - Core terminal functionality → `src/components/terminal/core/`
   - Terminal variants → `src/components/terminal/variants/`

2. **UI Components**: Files that contain general UI React components
   - Common UI components → `src/components/common/`
   - Layout components → `src/components/layout/`
   - Project-related components → `src/components/project/`

3. **Context Providers**: Files that define React contexts and providers
   - All context providers → `src/contexts/`

4. **Hooks**: Files that define custom React hooks
   - All hooks → `src/hooks/`

5. **Utilities**: Files that contain utility functions or helpers
   - API utilities → `src/lib/api/`
   - Configuration utilities → `src/lib/config/`
   - General utilities → `src/lib/utils/`

6. **Types**: Files that contain TypeScript type definitions
   - All type definitions → `src/types/`

7. **Backend Code**: Files that contain Tauri backend code
   - Command handlers → `src-tauri/src/commands/`
   - Services → `src-tauri/src/services/`
   - Utility functions → `src-tauri/src/utils/`

8. **Entry Points**: Files that serve as application entry points
   - Main application component → `src/App.tsx`
   - Application entry point → `src/main.tsx`

### 1.3 File Mapping Table

Create a detailed mapping table that specifies where each file should be moved:

| Current Path | New Path | Component Type | Notes |
|--------------|----------|----------------|-------|
| `/path/to/Terminal/Claude/src/App.tsx` | `src/App.tsx` | Entry Point | Main application component |
| `/path/to/Terminal/Claude/src/Terminal.tsx` | `src/components/terminal/core/BaseTerminal.tsx` | Terminal Component | Rename to follow convention |
| ... | ... | ... | ... |

## Phase 2: Directory Structure Creation

Create the new directory structure according to the target structure:

```bash
# Create the new directory structure
mkdir -p src/assets/{icons,styles}
mkdir -p src/components/{common,layout,project,terminal/core,terminal/variants}
mkdir -p src/contexts
mkdir -p src/features/{project-management,terminal-manager,window-manager,notifications}/{components,hooks,utils}
mkdir -p src/hooks
mkdir -p src/lib/{api,config,utils}
mkdir -p src/services/process-manager
mkdir -p src/types
mkdir -p src-tauri/src/{commands,services/process,utils}
mkdir -p docs
mkdir -p tests
mkdir -p config
```

## Phase 3: Incremental File Migration

### 3.1 Prioritization

To minimize disruption, we'll migrate files in the following order:

1. Type definitions (they have the fewest dependencies)
2. Utility functions
3. Context providers
4. Hooks
5. UI components
6. Terminal components
7. Entry points

### 3.2 Migration Process

For each file, follow this process:

1. **Backup**: Create a backup of the file
2. **Move**: Move the file to its new location
3. **Rename**: Rename the file if necessary to follow the naming conventions
4. **Update imports within the file**: Update all import paths within the file
5. **Update imports in other files**: Update all import paths in other files that import this file
6. **Test**: Test the application to ensure it still works correctly

### 3.3 Import Path Updates

To facilitate updating import paths, we'll use the following approach:

1. Configure TypeScript path aliases in `tsconfig.json`:

```json
{
  "compilerOptions": {
    "baseUrl": ".",
    "paths": {
      "@components/*": ["src/components/*"],
      "@contexts/*": ["src/contexts/*"],
      "@hooks/*": ["src/hooks/*"],
      "@lib/*": ["src/lib/*"],
      "@types/*": ["src/types/*"],
      "@services/*": ["src/services/*"],
      "@features/*": ["src/features/*"],
      "@assets/*": ["src/assets/*"]
    }
  }
}
```

2. Use a find-and-replace script to update import paths:

```bash
# Example: Update imports for a file that was moved from src/Terminal.tsx to src/components/terminal/core/BaseTerminal.tsx
find /path/to/Terminal/Claude -type f -name "*.ts*" -exec sed -i 's/import {.*} from "\.\/Terminal"/import {.*} from "@components\/terminal\/core\/BaseTerminal"/g' {} \;
```

### 3.4 Testing Strategy

After each file migration:

1. Run TypeScript compiler to check for type errors:
   ```bash
   npx tsc --noEmit
   ```

2. Run linting to check for code style issues:
   ```bash
   npm run lint
   ```

3. Run tests to ensure functionality is preserved:
   ```bash
   npm test
   ```

4. Run the application to perform manual testing:
   ```bash
   npm run dev
   ```

## Phase 4: Component Refactoring

After moving all files, we'll refactor components to follow the new component hierarchy:

### 4.1 BaseTerminal Component

Refactor terminal components to use the BaseTerminal component as the foundation:

1. Extract common terminal functionality into the BaseTerminal component
2. Implement Higher-Order Components (HOCs) for terminal features
3. Refactor existing terminal variants to use composition

### 4.2 Terminal Factory

Create a TerminalFactory utility for configuring terminal variants:

1. Implement factory functions for creating different terminal instances
2. Configure with specific features based on use case
3. Update all terminal usage to use the factory

## Phase 5: Context Consolidation

Reorganize context providers for better separation of concerns:

1. Refactor existing contexts to follow the domain-specific approach
2. Implement the context provider hierarchy
3. Create custom hooks for accessing context state
4. Update components to use the new hooks

## Phase 6: Application Structure Consolidation

Consolidate multiple application entry points:

1. Create layout components for different application modes
2. Implement the LayoutManager component
3. Update the main App component to use the LayoutManager
4. Configure the application based on startup parameters

## Phase 7: Documentation

Update documentation to reflect the new structure:

1. Update README.md with information about the new structure
2. Create architecture documentation
3. Update component documentation
4. Create migration notes for developers

## Migration Checklist

Use this checklist to track progress through the migration process:

- [ ] Analyze existing file structure
- [ ] Create file categorization
- [ ] Build file mapping table
- [ ] Create new directory structure
- [ ] Migrate type definitions
- [ ] Migrate utility functions
- [ ] Migrate context providers
- [ ] Migrate hooks
- [ ] Migrate UI components
- [ ] Migrate terminal components
- [ ] Migrate entry points
- [ ] Refactor BaseTerminal component
- [ ] Implement terminal feature HOCs
- [ ] Create TerminalFactory utility
- [ ] Refactor contexts
- [ ] Implement context provider hierarchy
- [ ] Consolidate application entry points
- [ ] Update documentation
- [ ] Perform final testing

## Rollback Plan

In case of issues, have a rollback plan ready:

1. Maintain the original file structure alongside the new structure during migration
2. Use feature flags to switch between old and new implementations
3. Have a script ready to revert to the original structure if needed

## Timeline

| Phase | Estimated Duration | Dependencies |
|-------|-------------------|--------------|
| Phase 1: File Analysis and Mapping | 2-3 days | None |
| Phase 2: Directory Structure Creation | 1 day | Phase 1 |
| Phase 3: Incremental File Migration | 5-7 days | Phase 2 |
| Phase 4: Component Refactoring | 3-4 days | Phase 3 |
| Phase 5: Context Consolidation | 2-3 days | Phase 3 |
| Phase 6: Application Structure Consolidation | 2-3 days | Phase 4, Phase 5 |
| Phase 7: Documentation | 1-2 days | All previous phases |

Total estimated time: 2-3 weeks

## Conclusion

This migration plan provides a structured approach to reorganizing the Terminal/Claude codebase according to the new standardized structure. By following this plan, we can minimize disruption while achieving the goals of improved maintainability, scalability, and developer experience.

---

Last Updated: May 15, 2025