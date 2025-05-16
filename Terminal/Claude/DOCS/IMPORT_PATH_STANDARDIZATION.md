# Terminal/Claude Import Path Standardization

This document provides guidelines for updating import paths when migrating to the new standardized directory structure for the Terminal/Claude codebase.

## Directory Structure

The new standardized directory structure follows this pattern:

```
Terminal/Claude/
├── src/
│   ├── components/
│   │   ├── base/
│   │   │   └── BaseTerminal.tsx
│   │   ├── common/
│   │   ├── layout/
│   │   │   ├── TerminalContainer.tsx
│   │   │   ├── TerminalFooter.tsx
│   │   │   └── TerminalHeader.tsx
│   │   ├── terminal/
│   │   │   ├── core/
│   │   │   └── variants/
│   │   └── project/
│   ├── contexts/
│   │   ├── ApplicationContext.tsx
│   │   ├── NotificationContext.tsx
│   │   ├── ProjectContext.tsx
│   │   └── TerminalContext.tsx
│   ├── hooks/
│   ├── lib/
│   │   ├── api/
│   │   ├── config/
│   │   └── utils/
│   ├── services/
│   │   └── process-manager/
│   ├── styles/
│   │   ├── BaseTerminal.css
│   │   ├── TerminalContainer.css
│   │   ├── TerminalFooter.css
│   │   └── TerminalHeader.css
│   ├── types/
│   │   └── terminal.ts
│   ├── App.tsx
│   └── main.tsx
```

## Path Aliases

Configure TypeScript path aliases in `tsconfig.json` to simplify imports:

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
      "@styles/*": ["src/styles/*"]
    }
  }
}
```

## Import Path Updates

When migrating files, update the import paths based on the following guidelines:

### Component Imports

**Before:**
```typescript
import Terminal from './Terminal';
import Header from './Header';
import Footer from './Footer';
```

**After:**
```typescript
import { BaseTerminal } from '@components/base/BaseTerminal';
import { TerminalHeader } from '@components/layout/TerminalHeader';
import { TerminalFooter } from '@components/layout/TerminalFooter';
```

### Context Imports

**Before:**
```typescript
import { TerminalContext } from './contexts/TerminalContext';
import { AppContext } from './contexts/AppContext';
```

**After:**
```typescript
import { TerminalContext } from '@contexts/TerminalContext';
import { ApplicationContext } from '@contexts/ApplicationContext';
```

### Utility Imports

**Before:**
```typescript
import { parseCommand } from './utils';
import { fetchData } from './api';
```

**After:**
```typescript
import { parseCommand } from '@lib/utils/commandParser';
import { fetchData } from '@lib/api/dataFetcher';
```

### Style Imports

**Before:**
```typescript
import './Terminal.css';
import './Header.css';
```

**After:**
```typescript
import '@styles/BaseTerminal.css';
import '@styles/TerminalHeader.css';
```

### Type Imports

**Before:**
```typescript
import { TerminalProps } from './types';
```

**After:**
```typescript
import { TerminalProps } from '@types/terminal';
```

## Automated Update Script

Use the following command to update import paths in a file:

```bash
# Update imports in a specific file
npx ts-migrate-imports --aliasFile=tsconfig.json --file=src/components/base/BaseTerminal.tsx
```

For bulk updates:

```bash
# Update all TSX files
npx ts-migrate-imports --aliasFile=tsconfig.json --pattern='src/**/*.tsx'
```

## Verification

After updating import paths:

1. Run TypeScript compiler to check for type errors:
   ```bash
   npx tsc --noEmit
   ```

2. Run linting to check for code style issues:
   ```bash
   npm run lint
   ```

3. Test the application to ensure everything works correctly:
   ```bash
   npm run dev
   ```

## Example Migration

Here's an example of how to migrate a component:

**Original file (Terminal.tsx):**
```typescript
import React from 'react';
import './Terminal.css';
import { TerminalHeader } from './Header';
import { TerminalFooter } from './Footer';
import { useTerminal } from './hooks/useTerminal';
import { TerminalProps } from './types';

export const Terminal: React.FC<TerminalProps> = ({ initialCommand }) => {
  const { command, setCommand, executeCommand } = useTerminal();
  
  return (
    <div className="terminal">
      <TerminalHeader />
      <div className="terminal-content">
        {/* Terminal content */}
      </div>
      <TerminalFooter command={command} setCommand={setCommand} onExecute={executeCommand} />
    </div>
  );
};
```

**Migrated file (src/components/base/BaseTerminal.tsx):**
```typescript
import React from 'react';
import '@styles/BaseTerminal.css';
import { TerminalHeader } from '@components/layout/TerminalHeader';
import { TerminalFooter } from '@components/layout/TerminalFooter';
import { useTerminal } from '@hooks/useTerminal';
import { TerminalProps } from '@types/terminal';

export const BaseTerminal: React.FC<TerminalProps> = ({ initialCommand }) => {
  const { command, setCommand, executeCommand } = useTerminal();
  
  return (
    <div className="terminal">
      <TerminalHeader />
      <div className="terminal-content">
        {/* Terminal content */}
      </div>
      <TerminalFooter command={command} setCommand={setCommand} onExecute={executeCommand} />
    </div>
  );
};
```

## Troubleshooting

Common issues when updating import paths:

1. **Circular Dependencies**: If you encounter circular dependency warnings, consider restructuring your components or creating intermediary modules.

2. **Missing Files**: If TypeScript complains about missing files, check if your path aliases are correctly configured in tsconfig.json.

3. **Case Sensitivity**: Ensure the casing of file names and imports match exactly, especially when migrating between different operating systems.

4. **Path Alias Recognition**: Some tools may not recognize path aliases. Ensure your build system, linter, and IDE are properly configured.