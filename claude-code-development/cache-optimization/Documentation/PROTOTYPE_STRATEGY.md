# Simplified Prototype Strategy: Claude Code File Caching

This document provides a streamlined approach to quickly prototype the file caching system for Claude Code. It focuses on demonstrating core functionality with minimal modifications to the existing codebase.

## Prototype Goals

1. Demonstrate caching functionality with minimal changes
2. Prove token reduction potential
3. Validate the architecture
4. Create a foundation for full implementation

## Simplified Approach

Instead of modifying the bundled CLI directly, we'll create a wrapper script that intercepts file operations. This allows us to:
- Avoid complex code analysis and modification
- Test the concept quickly
- Measure performance benefits immediately
- Iterate rapidly

## Implementation Steps

### Step 1: Create Basic Cache Module (1 day)

```javascript
// cache.js - Core cache implementation
import fs from 'fs';
import path from 'path';
import os from 'os';

// Simplified FileCache class
class FileCache {
  constructor() {
    this.cache = new Map();
    this.cacheDir = path.join(os.homedir(), '.claude', 'cache');
    this.ensureCacheDir();
  }
  
  ensureCacheDir() {
    if (!fs.existsSync(this.cacheDir)) {
      fs.mkdirSync(this.cacheDir, { recursive: true });
    }
  }
  
  get(filePath) {
    return this.cache.get(filePath);
  }
  
  set(filePath, content) {
    this.cache.set(filePath, content);
    return content;
  }
  
  has(filePath) {
    return this.cache.has(filePath);
  }
  
  stats() {
    return {
      entries: this.cache.size,
      files: [...this.cache.keys()]
    };
  }
}

export const fileCache = new FileCache();
```

### Step 2: Create Tool Interceptors (1 day)

```javascript
// interceptors.js - Intercept file operations
import { fileCache } from './cache.js';
import fs from 'fs';

// Original tool references will be determined during implementation
let originalViewFile = null;
let originalWriteFile = null;

// Cached view file implementation
export function viewFileWithCache(filePath, offset, limit) {
  console.log(`[File request] ${filePath}`);
  
  // Skip cache if offset/limit used
  if (offset || limit) {
    return originalViewFile(filePath, offset, limit);
  }
  
  // Try to get from cache
  if (fileCache.has(filePath)) {
    console.log(`[Cache hit] ${filePath}`);
    return fileCache.get(filePath);
  }
  
  // Cache miss, read file
  console.log(`[Cache miss] ${filePath}`);
  const content = originalViewFile(filePath, offset, limit);
  
  // Store in cache
  fileCache.set(filePath, content);
  
  return content;
}

// Cached write file implementation
export function writeFileWithCache(filePath, content) {
  // Write file
  const result = originalWriteFile(filePath, content);
  
  // Update cache if successful
  if (result.success) {
    fileCache.set(filePath, content);
  }
  
  return result;
}

// Function to set original tool references
export function setOriginalTools(viewTool, writeTool) {
  originalViewFile = viewTool;
  originalWriteFile = writeTool;
}
```

### Step 3: Integrate Caching with CLI Arguments (3-5 hours)

Since we're working with a forked repository, we can modify the CLI directly to add caching functionality through flags:

```javascript
// Add to cli.js or create a new cache module to import

// Import command-line argument parsing (likely already exists in cli.js)
import { program } from 'commander';

// Import the cache and interceptors
import { fileCache } from './cache.js';
import { viewFileWithCache, writeFileWithCache } from './interceptors.js';

// Track metrics
const metrics = {
  requests: 0,
  cacheHits: 0,
  cacheMisses: 0,
  startTime: Date.now()
};

// Add cache flag to enable caching
program.option('--cache', 'Enable file caching to reduce token usage');

// Store original tool implementations
let originalViewFile = null;
let originalWriteFile = null;

// Hook into CLI initialization
function initializeCache(options) {
  // If cache flag is provided, set up interceptors
  if (options.cache) {
    console.log('[Claude Cache] File caching enabled');
    
    // Store original implementations
    originalViewFile = /* reference to original view implementation */;
    originalWriteFile = /* reference to original write implementation */;
    
    // Replace with cached versions
    /* replace tool implementations with cached versions */
    
    // Initialize cache
    fileCache.ensureCacheDir();
    
    // Register exit handler for stats
    process.on('exit', () => {
      if (options.cache) {
        const stats = fileCache.stats();
        const duration = (Date.now() - metrics.startTime) / 1000;
        
        console.log('\n[Claude Cache] Session statistics:');
        console.log(`- Duration: ${duration.toFixed(2)}s`);
        console.log(`- Cached entries: ${stats.entries}`);
        console.log(`- Cache hit rate: ${metrics.requests > 0 ? 
          (metrics.cacheHits / metrics.requests * 100).toFixed(2) : 0}%`);
      }
    });
  }
}

// Call during CLI initialization
// initializeCache(program.opts());
```

### Step 4: Add Cache Management Commands (2-3 hours)

Add cache-specific subcommands to the CLI:

```javascript
// Add cache command to program
program
  .command('cache')
  .description('Manage Claude file cache')
  .argument('<action>', 'Action to perform: stats, clear, enable, disable')
  .action(async (action) => {
    switch(action) {
      case 'stats':
        const stats = fileCache.stats();
        console.log('Cache Statistics:');
        console.log(`- Entries: ${stats.entries}`);
        console.log(`- Size: ${formatBytes(stats.bytesStored)}`);
        console.log(`- Hit Rate: ${(stats.hitRate * 100).toFixed(2)}%`);
        console.log(`- Hits: ${stats.hits}`);
        console.log(`- Misses: ${stats.misses}`);
        break;
        
      case 'clear':
        await fileCache.clear();
        console.log('Cache cleared successfully');
        break;
        
      case 'enable':
        // Update config to enable cache by default
        await saveConfig({ cache: { enabled: true } });
        console.log('Cache enabled by default');
        break;
        
      case 'disable':
        // Update config to disable cache by default
        await saveConfig({ cache: { enabled: false } });
        console.log('Cache disabled by default');
        break;
        
      default:
        console.error(`Unknown cache action: ${action}`);
        console.log('Available actions: stats, clear, enable, disable');
    }
  });

// Helper function to format bytes
function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// Add to loadConfig function or create if it doesn't exist
async function loadConfig() {
  // ...existing implementation...
  
  // Add default cache configuration
  config.cache = config.cache || {
    enabled: false,
    ttl: 3600000,
    maxSize: 104857600
  };
  
  return config;
}
```

## Testing the Prototype

### Basic Test Suite

1. **Read Test**
   ```bash
   # First read (cache miss)
   claude --cache "Read file.txt"
   
   # Second read (cache hit)
   claude --cache "Read file.txt again"
   ```

2. **Write Test**
   ```bash
   # Write file
   claude --cache "Write 'Hello World' to file.txt"
   
   # Read file (should be cache hit)
   claude --cache "Read file.txt"
   ```

3. **Cache Commands**
   ```bash
   # View statistics
   claude cache stats
   
   # Clear cache
   claude cache clear
   
   # Enable cache by default
   claude cache enable
   ```

### Performance Test

Create a simple script to compare token usage:

```javascript
// benchmark.js
// Simple benchmark to compare token usage with and without cache

const fs = require('fs');
const { spawn } = require('child_process');
const path = require('path');

async function runTest(useCache, filePath, repeats) {
  const args = useCache ? ['--cache', `Read ${filePath}`] : [`Read ${filePath}`];
  
  console.log(`Running ${repeats} reads ${useCache ? 'with' : 'without'} cache...`);
  
  for (let i = 0; i < repeats; i++) {
    // Run command to read file
    await new Promise((resolve) => {
      const proc = spawn('claude', args, {
        stdio: 'inherit'
      });
      proc.on('close', resolve);
    });
  }
}

// Create test file
const testFilePath = path.join(process.cwd(), 'test-file.txt');
fs.writeFileSync(testFilePath, 'This is a test file for benchmarking Claude Cache.');

// Run tests
(async () => {
  // Test without cache
  await runTest(false, testFilePath, 5);
  
  // Test with cache
  await runTest(true, testFilePath, 5);
  
  // Get cache stats
  await new Promise((resolve) => {
    const proc = spawn('claude', ['cache', 'stats'], {
      stdio: 'inherit'
    });
    proc.on('close', resolve);
  });
  
  // Clean up
  fs.unlinkSync(testFilePath);
})();
```

## Expected Timeline

With the advantage of having already developed comprehensive documentation, design plans, and implementation specifications, our prototype development timeline is significantly accelerated:

| Step | Task | Time Estimate |
|------|------|---------------|
| 1 | Create Basic Cache Module | 2-4 hours |
| 2 | Create Tool Interceptors | 2-4 hours |
| 3 | Create Wrapper Script | 4-6 hours |
| 4 | Create CLI Interface | 2-3 hours |
| 5 | Testing and Refinement | 4-8 hours |
| **Total** | | **14-25 hours (2-3 days)** |

This accelerated timeline is possible because:
1. We've already designed the core architecture
2. The cache implementation code is largely written in our technical specification
3. We've identified the key interception points
4. We have prepared test cases and benchmarking methodology

## Limitations of the Prototype

This simplified approach has some limitations:

1. **Tool Interception**: Limited to basic file operations, may miss some edge cases
2. **Persistence**: Basic implementation without robust error handling
3. **Configuration**: Minimal configuration options
4. **Edge Cases**: Not handling all possible file types and scenarios
5. **Integration**: Wrapper approach vs. direct integration

## Transition to Full Implementation

After validating the prototype:

1. Analyze Claude Code internals more thoroughly
2. Extract interception points from prototype
3. Implement full caching solution according to technical specification
4. Add comprehensive testing
5. Develop complete CLI integration

The prototype will provide valuable insights and proven patterns to accelerate the full implementation process.

## Prototype Success Criteria

The prototype will be considered successful if it demonstrates:

1. At least 40% reduction in token usage for repeated file reads
2. Proper caching of file write operations
3. Basic cache stats functionality
4. Stable operation without crashes
5. Measurable performance improvement