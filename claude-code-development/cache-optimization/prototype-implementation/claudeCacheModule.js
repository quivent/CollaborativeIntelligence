/**
 * ClaudeCodeWithCaching Module
 * 
 * This module implements file caching for Claude Code CLI.
 * Phase 1: Simple cache management
 * Phase 2: Tool interception for caching
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const os = require('os');

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
      debug: false,
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
    try {
      if (!fs.existsSync(CACHE_DIR)) {
        fs.mkdirSync(CACHE_DIR, { recursive: true });
      }
    } catch (err) {
      this.log('Error creating cache directory:', err);
    }
    
    // Load persisted cache if enabled
    if (this.options.validateOnStartup) {
      this.loadCache().catch(err => this.log('Error loading cache:', err));
    }
    
    // Register exit handler
    process.on('exit', () => {
      if (this.options.persistOnExit) {
        this.persistCacheSync();
      }
    });
    
    this.log('Cache initialized with options:', this.options);
  }
  
  /**
   * Get a file from cache
   */
  get(filePath) {
    if (!this.options.enabled) return null;
    
    const entry = this.cache.get(filePath);
    if (!entry) {
      this.stats.misses++;
      this.log(`Cache miss: ${filePath}`);
      return null;
    }
    
    const meta = this.metadata.get(filePath);
    
    // Check TTL
    if (Date.now() - meta.timestamp > this.options.ttl) {
      this.invalidate(filePath);
      this.stats.invalidations++;
      this.log(`Cache expired: ${filePath}`);
      return null;
    }
    
    this.stats.hits++;
    this.log(`Cache hit: ${filePath}`);
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
    
    this.log(`Cache set: ${filePath} (${contentSize} bytes)`);
    
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
      this.log(`Cache invalidated: ${filePath}`);
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
    
    this.log('Cache cleared');
    
    // Remove persisted cache
    try {
      if (fs.existsSync(CACHE_DIR)) {
        const files = fs.readdirSync(CACHE_DIR);
        for (const file of files) {
          fs.unlinkSync(path.join(CACHE_DIR, file));
        }
      }
    } catch (error) {
      this.log('Error clearing persisted cache:', error);
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
    
    this.log(`Cache size (${this.stats.bytesStored} bytes) exceeds max size (${this.options.maxSize} bytes), pruning...`);
    
    // Get entries sorted by timestamp (oldest first)
    const entries = [...this.metadata.entries()]
      .sort((a, b) => a[1].timestamp - b[1].timestamp);
    
    // Remove oldest entries until under max size
    while (this.stats.bytesStored > this.options.maxSize && entries.length) {
      const [filePath, meta] = entries.shift();
      this.log(`Pruning: ${filePath} (${meta.size} bytes)`);
      this.invalidate(filePath);
    }
    
    this.log(`Pruning complete, new cache size: ${this.stats.bytesStored} bytes`);
  }
  
  /**
   * Save cache to disk
   */
  async persistCache() {
    if (!this.options.persistOnExit) return;
    
    try {
      // Ensure cache directory exists
      if (!fs.existsSync(CACHE_DIR)) {
        fs.mkdirSync(CACHE_DIR, { recursive: true });
      }
      
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
      
      this.log(`Persisted ${this.cache.size} entries`);
    } catch (error) {
      this.log('Failed to persist cache:', error);
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
      
      this.log(`Persisted ${this.cache.size} entries (sync)`);
    } catch (error) {
      this.log('Failed to persist cache (sync):', error);
    }
  }
  
  /**
   * Load cache from disk
   */
  async loadCache() {
    try {
      // Check if metadata exists
      const metadataPath = path.join(CACHE_DIR, 'metadata.json');
      if (!fs.existsSync(metadataPath)) {
        this.log('No cache metadata found, skipping load');
        return;
      }
      
      // Load metadata
      const metadataStr = await fs.promises.readFile(metadataPath, 'utf8');
      const metadata = JSON.parse(metadataStr);
      
      // Validate version
      if (metadata.version !== '1.0') {
        this.log('Incompatible cache version, clearing');
        this.clear();
        return;
      }
      
      // Load cache entries
      let loadedCount = 0;
      let skippedCount = 0;
      
      for (const [filePath, meta] of Object.entries(metadata.entries)) {
        const hash = crypto.createHash('md5').update(filePath).digest('hex');
        const cachePath = path.join(CACHE_DIR, hash);
        
        if (fs.existsSync(cachePath)) {
          try {
            // Check if original file still exists and hasn't changed
            if (fs.existsSync(filePath)) {
              const stats = fs.statSync(filePath);
              
              // Skip if file was modified after caching
              if (stats.mtimeMs > meta.timestamp) {
                this.log(`Skipping ${filePath}: modified after caching`);
                skippedCount++;
                continue;
              }
              
              // Load cached content
              const content = await fs.promises.readFile(cachePath, 'utf8');
              
              // Verify content hash
              if (this.hashContent(content) !== meta.hash) {
                this.log(`Skipping ${filePath}: hash mismatch`);
                skippedCount++;
                continue;
              }
              
              // Add to cache
              this.cache.set(filePath, content);
              this.metadata.set(filePath, meta);
              this.stats.bytesStored += meta.size;
              loadedCount++;
            } else {
              this.log(`Skipping ${filePath}: original file not found`);
              skippedCount++;
            }
          } catch (err) {
            // Skip if any error occurred
            this.log(`Error loading ${filePath}:`, err);
            skippedCount++;
            continue;
          }
        } else {
          this.log(`Skipping ${filePath}: cache entry not found`);
          skippedCount++;
        }
      }
      
      this.log(`Loaded ${loadedCount} entries, skipped ${skippedCount} entries`);
    } catch (error) {
      this.log('Failed to load cache:', error);
    }
  }
  
  /**
   * Log debug messages
   */
  log(...args) {
    if (this.options.debug) {
      console.log('[Cache]', ...args);
    }
  }
}

/**
 * Load configuration
 */
async function loadConfig() {
  try {
    if (!fs.existsSync(CONFIG_PATH)) {
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
 * Simple binary file detection
 */
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
 * Format bytes for display
 */
function formatBytes(bytes) {
  if (bytes === 0) return '0 Bytes';
  const k = 1024;
  const sizes = ['Bytes', 'KB', 'MB', 'GB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

// Export functions for use in wrapper script
module.exports = {
  FileCache,
  loadConfig,
  saveConfig,
  fileExists,
  isBinaryFilePath,
  formatBytes
};

// Phase 2 tool interception code
module.exports.extractFilesFromQuery = function(query) {
  // Basic implementation - would need refinement
  const filePatterns = [
    /(?:read|view|open|check|show) ([^ ]+(\.[-a-zA-Z0-9]+)?)/gi,
    /(?:what does|content of) ([^ ]+(\.[-a-zA-Z0-9]+)?)/gi,
    /(?:in|from) (?:file|the file) ([^ ]+(\.[-a-zA-Z0-9]+)?)/gi
  ];
  
  const files = new Set();
  
  for (const pattern of filePatterns) {
    let match;
    while ((match = pattern.exec(query)) !== null) {
      files.add(match[1]);
    }
  }
  
  return Array.from(files);
};

// Check if file has been modified since caching
module.exports.isFileFresh = function(filePath, metadata) {
  try {
    const stats = fs.statSync(filePath);
    const meta = metadata.get(filePath);
    
    if (!meta) return false;
    
    return stats.mtimeMs <= meta.timestamp;
  } catch (err) {
    return false;
  }
};

// Process a query in cached mode
module.exports.processCachedQuery = function(query, cache) {
  // Extract files mentioned in the query
  const mentionedFiles = this.extractFilesFromQuery(query);
  
  // Process files with both absolute and relative paths
  const resolvedFiles = mentionedFiles.map(file => {
    // If it's not an absolute path, try to resolve it relative to cwd
    return path.isAbsolute(file) ? file : path.resolve(process.cwd(), file);
  });
  
  // Check which files are in cache
  const availableFiles = [];
  const missingFiles = [];
  
  for (const file of resolvedFiles) {
    if (cache.has(file)) {
      availableFiles.push(file);
    } else {
      missingFiles.push(file);
    }
  }
  
  // Generate response
  let response = `[CACHED RESPONSE]\n\n`;
  
  if (mentionedFiles.length === 0) {
    response += `No file references detected in your query.\n\n`;
    response += `To make an online query, use --query "${query}"\n`;
    return response;
  }
  
  if (missingFiles.length > 0) {
    response += `⚠️ The following files were not found in cache:\n`;
    for (const file of missingFiles) {
      response += `- ${file}\n`;
    }
    response += `\n`;
  }
  
  if (availableFiles.length > 0) {
    response += `📁 Available cached files:\n`;
    for (const file of availableFiles) {
      const content = cache.get(file);
      // Format a short preview of the file content
      const lines = content.split('\n');
      const totalLines = lines.length;
      const preview = lines.slice(0, 3).join('\n') + 
                    (totalLines > 3 ? `\n... (${totalLines} lines total)` : '');
      
      response += `\n--- ${file} ---\n${preview}\n`;
    }
  }
  
  response += `\nTo make an online query, use --query "${query}"\n`;
  return response;
};

// ---
// 
// Implementation changes (Phase 2, 2024-04-24):
// 
// 1. Added file reference extraction functionality:
//    - Implemented regex-based detection system to find file references in natural language queries
//    - Created pattern matching for common query formats ("view file", "read file", etc.)
//    - Added support for both extensions and extension-less files
// 
// 2. Implemented file freshness checking:
//    - Added timestamp comparison to detect modified files
//    - Created validation functions for cache integrity
// 
// 3. Enhanced cached mode processing:
//    - Developed comprehensive response formatting for cached files
//    - Added file preview functionality with line count information
//    - Created intelligent path resolution for both absolute and relative paths
//    - Implemented clear error reporting for missing files