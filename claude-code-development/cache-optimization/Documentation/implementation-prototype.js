/**
 * Claude Code File Caching System - Prototype Implementation
 * 
 * This prototype demonstrates the core functionality of the proposed
 * caching system for Claude Code. It includes an in-memory cache with
 * persistence capabilities and interceptors for file operations.
 */

import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import os from 'os';

// Configuration
const CACHE_DIR = path.join(os.homedir(), '.claude', 'cache');
const CONFIG_PATH = path.join(os.homedir(), '.claude', 'config.json');

/**
 * Core file cache implementation
 */
class FileCache {
  constructor(options = {}) {
    // Cache storage
    this.cache = new Map();
    this.metadata = new Map();
    
    // Default options
    this.options = {
      enabled: true,
      ttl: 3600000, // 1 hour in milliseconds
      maxSize: 100 * 1024 * 1024, // 100MB
      persistOnExit: true,
      validateOnStartup: true,
      ...options
    };
    
    // Statistics
    this.stats = {
      hits: 0,
      misses: 0,
      invalidations: 0,
      bytesStored: 0
    };
    
    // Initialize
    this.init();
  }
  
  async init() {
    // Create cache directory if it doesn't exist
    if (this.options.persistOnExit) {
      fs.promises.mkdir(CACHE_DIR, { recursive: true })
        .catch(err => console.error('Error creating cache directory:', err));
    }
    
    // Load persisted cache if enabled
    if (this.options.validateOnStartup) {
      this.loadCache().catch(err => console.error('Error loading cache:', err));
    }
    
    // Register exit handler
    process.on('exit', () => {
      if (this.options.persistOnExit) {
        this.persistCacheSync();
      }
    });
  }
  
  /**
   * Get a file from cache
   */
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
  
  /**
   * Store a file in cache
   */
  set(filePath, content) {
    if (!this.options.enabled) return;
    
    // Update or add to cache
    const contentSize = Buffer.byteLength(content, 'utf8');
    
    // If entry already exists, subtract its size first
    if (this.cache.has(filePath)) {
      const oldMeta = this.metadata.get(filePath);
      this.stats.bytesStored -= oldMeta.size;
    }
    
    // Update cache and metadata
    this.cache.set(filePath, content);
    this.metadata.set(filePath, {
      timestamp: Date.now(),
      size: contentSize,
      hash: this.hashContent(content)
    });
    
    // Update stats
    this.stats.bytesStored += contentSize;
    
    // Check if max size exceeded and prune if necessary
    this.pruneIfNeeded();
  }
  
  /**
   * Check if a file exists in cache
   */
  has(filePath) {
    if (!this.options.enabled) return false;
    return this.cache.has(filePath);
  }
  
  /**
   * Remove a file from cache
   */
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
  
  /**
   * Clear the entire cache
   */
  clear() {
    this.cache.clear();
    this.metadata.clear();
    this.stats = {
      hits: 0,
      misses: 0,
      invalidations: 0,
      bytesStored: 0
    };
    
    // Remove persisted cache
    try {
      fs.rmSync(CACHE_DIR, { recursive: true, force: true });
      fs.mkdirSync(CACHE_DIR, { recursive: true });
    } catch (error) {
      console.error('Error clearing persisted cache:', error);
    }
  }
  
  /**
   * Get cache statistics
   */
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
  
  /**
   * Create a hash of file content
   */
  hashContent(content) {
    return crypto.createHash('md5').update(content).digest('hex');
  }
  
  /**
   * Remove oldest entries if cache exceeds max size
   */
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
  
  /**
   * Save cache to disk
   */
  async persistCache() {
    if (!this.options.persistOnExit) return;
    
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
          stats: this.getStats(),
          entries: Object.fromEntries([...this.metadata.entries()])
        }, null, 2)
      );
      
      // Save cache entries
      for (const [filePath, content] of this.cache.entries()) {
        const hash = crypto.createHash('md5').update(filePath).digest('hex');
        const cachePath = path.join(CACHE_DIR, hash);
        await fs.promises.writeFile(cachePath, content);
      }
      
      console.log(`[Cache] Persisted ${this.cache.size} entries`);
    } catch (error) {
      console.error('[Cache] Failed to persist cache:', error);
    }
  }
  
  /**
   * Sync version for exit handler
   */
  persistCacheSync() {
    if (!this.options.persistOnExit) return;
    
    try {
      // Ensure cache directory exists
      if (!fs.existsSync(CACHE_DIR)) {
        fs.mkdirSync(CACHE_DIR, { recursive: true });
      }
      
      // Save metadata
      const metadataPath = path.join(CACHE_DIR, 'metadata.json');
      fs.writeFileSync(
        metadataPath,
        JSON.stringify({
          version: '1.0',
          timestamp: Date.now(),
          stats: this.getStats(),
          entries: Object.fromEntries([...this.metadata.entries()])
        }, null, 2)
      );
      
      // Save cache entries
      for (const [filePath, content] of this.cache.entries()) {
        const hash = crypto.createHash('md5').update(filePath).digest('hex');
        const cachePath = path.join(CACHE_DIR, hash);
        fs.writeFileSync(cachePath, content);
      }
    } catch (error) {
      console.error('[Cache] Failed to persist cache:', error);
    }
  }
  
  /**
   * Load cache from disk
   */
  async loadCache() {
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
            if (this.hashContent(content) !== meta.hash) continue;
            
            // Add to cache
            this.cache.set(filePath, content);
            this.metadata.set(filePath, meta);
            this.stats.bytesStored += meta.size;
          } catch (err) {
            // Skip if any error occurred
            continue;
          }
        }
      }
      
      console.log(`[Cache] Loaded ${this.cache.size} entries`);
    } catch (error) {
      console.error('[Cache] Failed to load cache:', error);
    }
  }
}

/**
 * Check if a file exists
 */
async function fileExists(filePath) {
  try {
    await fs.promises.access(filePath, fs.constants.F_OK);
    return true;
  } catch {
    return false;
  }
}

/**
 * Check if a cached entry is still valid
 */
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

/**
 * Load configuration
 */
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

/**
 * Save configuration
 */
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

/**
 * Deep merge utility
 */
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

/**
 * Format bytes for display
 */
function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/**
 * CLI command handlers for cache management
 */
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

/**
 * File operation interceptors
 */

// Original View tool reference (to be replaced with actual implementation)
async function originalViewFile(filePath, offset, limit) {
  // This would be the existing View tool implementation
  return fs.promises.readFile(filePath, 'utf8');
}

// Cached View tool
async function viewFileWithCache(filePath, offset, limit) {
  // Skip cache for binary files or when offset/limit are specified
  if (isBinaryFilePath(filePath) || offset || limit) {
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

// Original Write tool reference (to be replaced with actual implementation)
async function originalWriteFile(filePath, content) {
  // This would be the existing Write tool implementation
  await fs.promises.writeFile(filePath, content);
  return { success: true, filePath };
}

// Cached Write tool
async function writeFileWithCache(filePath, content) {
  // Write file
  const result = await originalWriteFile(filePath, content);
  
  // Update cache
  if (result.success && !isBinaryFilePath(filePath)) {
    fileCache.set(filePath, content);
  }
  
  return result;
}

// Simple binary file detection
function isBinaryFilePath(filePath) {
  const ext = path.extname(filePath).toLowerCase();
  const binaryExtensions = [
    '.jpg', '.jpeg', '.png', '.gif', '.bmp', '.tiff', 
    '.ico', '.pdf', '.zip', '.tar', '.gz', '.rar',
    '.exe', '.dll', '.so', '.dylib', '.bin'
  ];
  return binaryExtensions.includes(ext);
}

/**
 * Initialize cache on startup
 */
async function initializeCache() {
  try {
    // Load config
    const config = await loadConfig();
    
    // Initialize cache with config
    global.fileCache = new FileCache(config.cache || {});
    
    // Register exit handler for persistence
    process.on('SIGINT', () => {
      fileCache.persistCache()
        .then(() => process.exit(0))
        .catch(() => process.exit(1));
    });
    
    return fileCache;
  } catch (error) {
    console.error('Error initializing cache:', error);
    // Fall back to default cache
    global.fileCache = new FileCache();
    return fileCache;
  }
}

// Example usage
export async function main() {
  // Initialize cache
  await initializeCache();
  
  // Simulate some file operations
  const filePath = path.join(process.cwd(), 'example.txt');
  
  // Write to file
  await writeFileWithCache(filePath, 'Hello, World!');
  
  // Read file (should be cached)
  const content1 = await viewFileWithCache(filePath);
  console.log('First read:', content1);
  
  // Read again (should be a cache hit)
  const content2 = await viewFileWithCache(filePath);
  console.log('Second read:', content2);
  
  // Show cache stats
  await cacheCommands.stats();
}

// Run example if this file is executed directly
if (process.argv[1] === import.meta.url) {
  main().catch(console.error);
}