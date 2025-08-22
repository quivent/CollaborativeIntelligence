# Technical Specification: Claude Code File Caching

## Core Components

### 1. Cache Store

```javascript
class FileCache {
  constructor(options = {}) {
    this.cache = new Map();
    this.metadata = new Map();
    this.options = {
      enabled: true,
      ttl: 3600000, // 1 hour in milliseconds
      maxSize: 100 * 1024 * 1024, // 100MB
      persistOnExit: true,
      validateOnStartup: true,
      ...options
    };
    this.stats = {
      hits: 0,
      misses: 0,
      invalidations: 0,
      bytesStored: 0
    };
  }
  
  get(filePath) {
    if (!this.options.enabled) return null;
    
    const entry = this.cache.get(filePath);
    if (!entry) {
      this.stats.misses++;
      return null;
    }
    
    const meta = this.metadata.get(filePath);
    
    // Check TTL
    if (Date.now() - meta.timestamp > this.options.ttl) {
      this.invalidate(filePath);
      this.stats.invalidations++;
      return null;
    }
    
    this.stats.hits++;
    return entry;
  }
  
  set(filePath, content) {
    if (!this.options.enabled) return;
    
    // Update cache and metadata
    this.cache.set(filePath, content);
    this.metadata.set(filePath, {
      timestamp: Date.now(),
      size: content.length,
      hash: this.hashContent(content)
    });
    
    // Update stats
    this.stats.bytesStored += content.length;
    
    // Check if max size exceeded and prune if necessary
    this.pruneIfNeeded();
  }
  
  has(filePath) {
    if (!this.options.enabled) return false;
    return this.cache.has(filePath);
  }
  
  invalidate(filePath) {
    if (!this.options.enabled) return;
    
    if (this.cache.has(filePath)) {
      const meta = this.metadata.get(filePath);
      this.stats.bytesStored -= meta.size;
      this.stats.invalidations++;
    }
    
    this.cache.delete(filePath);
    this.metadata.delete(filePath);
  }
  
  clear() {
    this.cache.clear();
    this.metadata.clear();
    this.stats = {
      hits: 0,
      misses: 0,
      invalidations: 0,
      bytesStored: 0
    };
  }
  
  getStats() {
    return {
      entries: this.cache.size,
      sizeBytes: this.stats.bytesStored,
      hitRate: this.stats.hits / (this.stats.hits + this.stats.misses || 1),
      hits: this.stats.hits,
      misses: this.stats.misses,
      invalidations: this.stats.invalidations
    };
  }
  
  hashContent(content) {
    return crypto.createHash('md5').update(content).digest('hex');
  }
  
  pruneIfNeeded() {
    if (this.stats.bytesStored <= this.options.maxSize) return;
    
    // Get entries sorted by timestamp (oldest first)
    const entries = [...this.metadata.entries()]
      .sort((a, b) => a[1].timestamp - b[1].timestamp);
    
    // Remove oldest entries until under max size
    while (this.stats.bytesStored > this.options.maxSize && entries.length) {
      const [filePath, meta] = entries.shift();
      this.invalidate(filePath);
    }
  }
}
```

### 2. Cache Persistence

```javascript
const CACHE_DIR = path.join(os.homedir(), '.claude', 'cache');

async function persistCache() {
  if (!fileCache.options.persistOnExit) return;
  
  try {
    // Ensure cache directory exists
    await fs.promises.mkdir(CACHE_DIR, { recursive: true });
    
    // Save metadata
    const metadataPath = path.join(CACHE_DIR, 'metadata.json');
    await fs.promises.writeFile(
      metadataPath,
      JSON.stringify({
        version: '1.0',
        timestamp: Date.now(),
        stats: fileCache.getStats(),
        entries: Object.fromEntries(fileCache.metadata)
      })
    );
    
    // Save cache entries
    for (const [filePath, content] of fileCache.cache.entries()) {
      const hash = crypto.createHash('md5').update(filePath).digest('hex');
      const cachePath = path.join(CACHE_DIR, hash);
      await fs.promises.writeFile(cachePath, content);
    }
    
    console.log(`[Cache] Persisted ${fileCache.cache.size} entries`);
  } catch (error) {
    console.error('[Cache] Failed to persist cache:', error);
  }
}

async function loadCache() {
  if (!fileCache.options.validateOnStartup) return;
  
  try {
    // Check if metadata exists
    const metadataPath = path.join(CACHE_DIR, 'metadata.json');
    if (!await fileExists(metadataPath)) return;
    
    // Load metadata
    const metadataStr = await fs.promises.readFile(metadataPath, 'utf8');
    const metadata = JSON.parse(metadataStr);
    
    // Validate version
    if (metadata.version !== '1.0') {
      console.log('[Cache] Incompatible cache version, clearing');
      return;
    }
    
    // Load cache entries
    for (const [filePath, meta] of Object.entries(metadata.entries)) {
      const hash = crypto.createHash('md5').update(filePath).digest('hex');
      const cachePath = path.join(CACHE_DIR, hash);
      
      if (await fileExists(cachePath)) {
        try {
          // Check if original file still exists and hasn't changed
          const stats = await fs.promises.stat(filePath);
          
          // Skip if file was modified after caching
          if (stats.mtimeMs > meta.timestamp) continue;
          
          // Load cached content
          const content = await fs.promises.readFile(cachePath, 'utf8');
          
          // Verify content hash
          if (fileCache.hashContent(content) !== meta.hash) continue;
          
          // Add to cache
          fileCache.cache.set(filePath, content);
          fileCache.metadata.set(filePath, meta);
          fileCache.stats.bytesStored += meta.size;
        } catch (err) {
          // Skip if any error occurred
          continue;
        }
      }
    }
    
    console.log(`[Cache] Loaded ${fileCache.cache.size} entries`);
  } catch (error) {
    console.error('[Cache] Failed to load cache:', error);
  }
}
```

### 3. Tool Interceptors

```javascript
// Intercept View tool
async function viewFileWithCache(filePath, offset, limit) {
  // Skip cache for binary files
  if (isBinaryFile(filePath)) {
    return originalViewFile(filePath, offset, limit);
  }
  
  // Try to get from cache
  if (fileCache.has(filePath)) {
    const cachedContent = fileCache.get(filePath);
    
    // Validate cache entry
    if (await isValidCacheEntry(filePath, cachedContent)) {
      console.log(`[Cache hit] ${filePath}`);
      return cachedContent;
    }
    
    // Invalid cache entry
    fileCache.invalidate(filePath);
  }
  
  // Cache miss, read file
  console.log(`[Cache miss] ${filePath}`);
  const content = await originalViewFile(filePath, offset, limit);
  
  // Store in cache
  fileCache.set(filePath, content);
  
  return content;
}

// Intercept Edit/Replace tool
async function writeFileWithCache(filePath, content) {
  // Write file
  const result = await originalWriteFile(filePath, content);
  
  // Update cache
  if (result.success) {
    fileCache.set(filePath, content);
  }
  
  return result;
}

// Cache validation
async function isValidCacheEntry(filePath, cachedContent) {
  try {
    const stats = await fs.promises.stat(filePath);
    const meta = fileCache.metadata.get(filePath);
    
    // Check modification time
    if (stats.mtimeMs > meta.timestamp) {
      console.log(`[Cache invalidated] ${filePath} (modified)`);
      return false;
    }
    
    return true;
  } catch (error) {
    // File doesn't exist anymore
    console.log(`[Cache invalidated] ${filePath} (not found)`);
    return false;
  }
}
```

### 4. CLI Commands

```javascript
// CLI command handlers
const cacheCommands = {
  async stats() {
    const stats = fileCache.getStats();
    console.log('Cache Statistics:');
    console.log(`- Entries: ${stats.entries}`);
    console.log(`- Size: ${formatBytes(stats.sizeBytes)}`);
    console.log(`- Hit Rate: ${(stats.hitRate * 100).toFixed(2)}%`);
    console.log(`- Hits: ${stats.hits}`);
    console.log(`- Misses: ${stats.misses}`);
    console.log(`- Invalidations: ${stats.invalidations}`);
  },
  
  async clear() {
    fileCache.clear();
    
    // Remove persisted cache
    try {
      await fs.promises.rm(CACHE_DIR, { recursive: true, force: true });
      await fs.promises.mkdir(CACHE_DIR, { recursive: true });
    } catch (error) {
      console.error('Error clearing persisted cache:', error);
    }
    
    console.log('Cache cleared successfully');
  },
  
  async enable() {
    fileCache.options.enabled = true;
    await saveConfig({ cache: { enabled: true } });
    console.log('Cache enabled');
  },
  
  async disable() {
    fileCache.options.enabled = false;
    await saveConfig({ cache: { enabled: false } });
    console.log('Cache disabled');
  }
};

// Helper to format bytes
function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}
```

### 5. Configuration Integration

```javascript
const CONFIG_PATH = path.join(os.homedir(), '.claude', 'config.json');

async function loadConfig() {
  try {
    if (!await fileExists(CONFIG_PATH)) {
      return {};
    }
    
    const configStr = await fs.promises.readFile(CONFIG_PATH, 'utf8');
    return JSON.parse(configStr);
  } catch (error) {
    console.error('Error loading config:', error);
    return {};
  }
}

async function saveConfig(updatedConfig) {
  try {
    const existingConfig = await loadConfig();
    const newConfig = deepMerge(existingConfig, updatedConfig);
    
    await fs.promises.writeFile(
      CONFIG_PATH,
      JSON.stringify(newConfig, null, 2)
    );
    
    return true;
  } catch (error) {
    console.error('Error saving config:', error);
    return false;
  }
}

// Deep merge utility
function deepMerge(target, source) {
  const result = { ...target };
  
  for (const key in source) {
    if (Object.prototype.hasOwnProperty.call(source, key)) {
      if (isObject(source[key]) && isObject(target[key])) {
        result[key] = deepMerge(target[key], source[key]);
      } else {
        result[key] = source[key];
      }
    }
  }
  
  return result;
}

function isObject(item) {
  return item && typeof item === 'object' && !Array.isArray(item);
}
```

## Integration Points

### Main Application Entry Point

```javascript
// Add to the beginning of CLI application
async function initializeCache() {
  // Load config
  const config = await loadConfig();
  
  // Initialize cache with config
  global.fileCache = new FileCache(config.cache || {});
  
  // Load persisted cache
  await loadCache();
  
  // Register exit handler for persistence
  process.on('exit', () => {
    persistCache();
  });
}

// Call during startup
await initializeCache();
```

### Command Registration

```javascript
// Add cache commands to CLI
program
  .command('cache')
  .description('Cache management commands')
  .argument('<action>', 'Action to perform: stats, clear, enable, disable')
  .action(async (action) => {
    if (cacheCommands[action]) {
      await cacheCommands[action]();
    } else {
      console.error(`Unknown cache action: ${action}`);
      console.log('Available actions: stats, clear, enable, disable');
    }
  });

// Add cache configuration to config command
program
  .command('config')
  .option('set <key> <value>', 'Set a configuration value')
  .option('get <key>', 'Get a configuration value')
  .action(async (options) => {
    // Handle cache configuration specifically
    if (options.set && options.set.startsWith('cache.')) {
      const subKey = options.set.split('.')[1];
      const value = parseConfigValue(options.value);
      
      await saveConfig({
        cache: {
          [subKey]: value
        }
      });
      
      // Update runtime config
      fileCache.options[subKey] = value;
      
      console.log(`Set cache.${subKey} to ${value}`);
    }
    
    // ... other config handling
  });
```

## Performance Considerations

### Memory Management

- Keep watch on cache size to prevent excessive memory usage
- Use TTL to automatically expire old entries
- Implement LRU eviction when approaching max size

### File Change Detection

- Watch for file changes in project directories
- Use fs.stat calls to verify modification times
- Consider file size as a quick change indicator

### Edge Cases

- Handle large binary files differently (don't cache or use separate limits)
- Special handling for image files and other non-text content
- Consider workspace-specific cache settings for multi-project work

## Test Plan

1. **Unit Tests**
   - Test cache operations (get, set, invalidate)
   - Test persistence operations
   - Test file change detection

2. **Integration Tests**
   - Test file read/write with cache
   - Test CLI commands
   - Test configuration changes

3. **Performance Tests**
   - Measure token reduction in typical workflows
   - Benchmark cache hit rates
   - Measure memory consumption

4. **Edge Case Tests**
   - Test with very large files
   - Test with binary files
   - Test with rapidly changing files