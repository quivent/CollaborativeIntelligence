# Phase 2 Implementation Notes

This document outlines the plan for Phase 2 of the Claude Code cache system, which will implement actual file caching, offline mode processing, and tool interception.

## Current Status (Phase 1)

Phase 1 (completed) provides:
- Mode switching system (cached/default)
- Command-line interface with flags
- Session persistence
- Cache management commands
- Claude command wrapping

## Technical Approaches

We have two main approaches for implementing Phase 2:

### Approach 1: Module Interception

Using Node.js module interception to hook into Claude Code's file operations:

```javascript
// Add to claude-cache wrapper
const claudeCliPath = require.resolve('@anthropic-ai/claude-code');
const claudeToolsPath = path.dirname(claudeCliPath) + '/tools'; // Likely location

// Intercept View tool
const originalViewTool = require(viewToolPath);
require.cache[require.resolve(viewToolPath)].exports = function(args) {
  const filePath = args.file_path;
  
  // Try cache first
  if (fileCache.has(filePath) && isValidCacheEntry(filePath)) {
    // Return cached content with proper formatting
    return {
      content: fileCache.get(filePath),
      file_path: filePath
    };
  }
  
  // If in cached mode, return error or empty result
  if (inCachedMode()) {
    return {
      error: `File ${filePath} not found in cache and running in cached mode.`,
      file_path: filePath
    };
  }
  
  // Not in cache and in online mode, use original function
  const result = originalViewTool(args);
  
  // Cache the result
  fileCache.set(filePath, result.content);
  
  return result;
};
```

### Approach 2: Offline Processing

Implementing a fully local processing system for cached mode:

```javascript
// In claude-cache-module.js

// Function to process queries in cached mode
function processCachedQuery(query) {
  // Extract files mentioned in the query
  const mentionedFiles = extractFilesFromQuery(query);
  
  // Check which files are in cache
  const availableFiles = [];
  const missingFiles = [];
  
  for (const file of mentionedFiles) {
    if (fileCache.has(file)) {
      availableFiles.push(file);
    } else {
      missingFiles.push(file);
    }
  }
  
  // Generate response
  let response = `[CACHED RESPONSE]\n\n`;
  
  if (missingFiles.length > 0) {
    response += `⚠️ The following files were not found in cache:\n`;
    for (const file of missingFiles) {
      response += `- ${file}\n`;
    }
    response += `\n`;
  }
  
  if (availableFiles.length > 0) {
    response += `📁 Available files:\n`;
    for (const file of availableFiles) {
      const content = fileCache.get(file);
      const preview = content.slice(0, 100) + (content.length > 100 ? '...' : '');
      response += `- ${file}: ${preview}\n`;
    }
  }
  
  response += `\nTo make an online query, use --query "${query}"\n`;
  return response;
}
```

## Phase 2 Implementation Components

### 1. File Operation Interception

In `claude-cache-module.js`, we'll implement these interceptors:

```javascript
// View file interceptor
function interceptViewFile(filePath, offset, limit) {
  // Check if file is in cache
  if (fileCache.has(filePath)) {
    // Return cached content
    return fileCache.get(filePath);
  }
  
  // If in online mode, fetch and cache
  if (inOnlineMode()) {
    const content = originalViewFile(filePath, offset, limit);
    fileCache.set(filePath, content);
    return content;
  }
  
  // In cached mode with no cached content
  return `Error: File ${filePath} not found in cache and running in cached mode.`;
}

// Edit file interceptor
function interceptEditFile(filePath, oldString, newString, expectedReplacements) {
  // Only allow in online mode
  if (inOnlineMode()) {
    const result = originalEditFile(filePath, oldString, newString, expectedReplacements);
    // Update cache if successful
    if (result.success) {
      // Update cache with the new content
      if (fileCache.has(filePath)) {
        const cachedContent = fileCache.get(filePath);
        const updatedContent = cachedContent.replace(oldString, newString);
        fileCache.set(filePath, updatedContent);
      }
    }
    return result;
  }
  
  // In cached mode
  return {
    success: false,
    error: "Error: File edits are only available in online mode. Use !default to switch modes."
  };
}
```

### 2. Query Parsing

For extracting file references from queries:

```javascript
function extractFilesFromQuery(query) {
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
}
```

### 3. Cache Synchronization

To ensure cache validity:

```javascript
// Function to check if file has been modified since caching
function isFileFresh(filePath) {
  try {
    const stats = fs.statSync(filePath);
    const meta = fileCache.metadata.get(filePath);
    
    if (!meta) return false;
    
    return stats.mtimeMs <= meta.timestamp;
  } catch (err) {
    return false;
  }
}

// Function to validate all cached files
function validateCache() {
  const invalidFiles = [];
  
  for (const [filePath, _] of fileCache.cache.entries()) {
    if (!isFileFresh(filePath)) {
      invalidFiles.push(filePath);
    }
  }
  
  // Invalidate modified files
  for (const filePath of invalidFiles) {
    fileCache.invalidate(filePath);
  }
  
  return invalidFiles.length;
}
```

### 4. Enhanced CLI

Additional CLI commands and features:

```bash
# Add more comprehensive mode commands
if [[ "$1" == "mode" ]]; then
  case "$2" in
    cached)
      set_session_mode "cached"
      echo -e "\033[1;34m[MODE]\033[0m Switched to cached mode."
      ;;
    default|online)
      set_session_mode "default"
      echo -e "\033[1;32m[MODE]\033[0m Switched to online mode."
      ;;
    auto)
      # Auto mode: use cache if available, else use online
      set_session_mode "auto"
      echo -e "\033[1;35m[MODE]\033[0m Switched to auto mode."
      ;;
    *)
      echo -e "\033[1;36m[HELP]\033[0m Usage: claude mode [cached|default|auto]"
      ;;
  esac
  exit 0
fi
```

## Implementation Timeline

1. **Module Analysis (1-2 days)**
   - Analyze Claude CLI structure
   - Identify interception points

2. **Proof of Concept (1-2 days)**
   - Implement basic interception
   - Test with simple commands

3. **Query Processing (2-3 days)**
   - Implement file reference extraction
   - Create cached mode query processor
   - Add response formatting

4. **Full Implementation (3-4 days)**
   - Complete cache integration
   - Add all validation logic
   - Implement tool interception

5. **Testing and Refinement (2-3 days)**
   - Comprehensive testing
   - Performance optimization
   - Edge case handling

6. **Documentation & Polish (1-2 days)**
   - Update README with new features
   - Add detailed examples
   - Create troubleshooting guide

## Technical Challenges

1. **Module Discovery**
   - The Claude CLI may be bundled, making it difficult to locate specific modules
   - Solution: Use dynamic module inspection to locate file operation functions

2. **API Communication**
   - We must preserve all API communication pathways
   - Solution: Only intercept content, not requests

3. **Query Understanding**
   - Accurately extracting file references from natural language queries
   - Solution: Use multiple extraction patterns and refine over time

4. **Edge Cases**
   - Handling binary files, large files, and access permission issues
   - Solution: Implement specific handlers for each type

## Success Criteria for Phase 2

1. Users can work completely offline using cached files
2. The system provides helpful responses using only cached data
3. One-time online queries properly update the cache
4. Mode switching works reliably between sessions
5. Performance shows >70% reduction in API calls for repeat operations

## Future Enhancements (Phase 3)

1. **Local semantic processing** - Use a smaller local model to answer simple questions
2. **Intelligent file tracking** - Track file relationships to suggest related files
3. **Command history caching** - Cache command results for similar queries
4. **Cache preloading** - Option to preload commonly accessed files
5. **Advanced query parsing** - Better understand natural language queries