# Development Setup: Claude Code File Caching

This guide provides step-by-step instructions for setting up a development environment to work on the Claude Code file caching feature.

## Prerequisites

- Node.js 18.0.0 or higher
- Git
- npm or yarn
- Basic understanding of JavaScript/Node.js
- Text editor or IDE (VS Code recommended)

## Environment Setup

### 1. Fork and Clone Repository

```bash
# Fork the repository on GitHub first, then:
git clone https://github.com/yourusername/claude-code.git ClaudeCodeWithCaching
cd ClaudeCodeWithCaching

# Create a feature branch
git checkout -b feature/file-caching
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Set Up Local Claude Code

```bash
# Create a symbolic link to use your development version
npm link

# Verify installation
claude --version
```

## Project Structure Analysis

Before making changes, let's understand the project structure:

```bash
# List top-level files and directories
ls -la

# Find main JavaScript files
find . -name "*.js" | grep -v "node_modules"

# Look for existing tool implementations
grep -r "View" --include="*.js" .
grep -r "Edit" --include="*.js" .
```

## Creating the Cache Module

### 1. Create Cache Directory Structure

```bash
# Create directory for cache module
mkdir -p src/cache
```

### 2. Implement Core Cache Module

Create the file `src/cache/index.js`:

```javascript
import fs from 'fs';
import path from 'path';
import crypto from 'crypto';
import os from 'os';

// Import cache implementation from our documentation
// ... (copy FileCache class implementation)
```

### 3. Implement Tool Interceptors

Create the file `src/cache/interceptors.js`:

```javascript
// Import original tool implementations
// ... (implement tool interception logic)
```

## Integrating with CLI

### 1. Find CLI Entry Point

Locate the main CLI entry point (likely `cli.js` or similar):

```bash
# Look for command registration patterns
grep -r "command(" --include="*.js" .
```

### 2. Add Cache Initialization

Modify the main CLI file to initialize cache during startup.

### 3. Register Cache Commands

Add the cache command handlers to the CLI command registry.

## Testing Your Implementation

### 1. Basic Functionality Tests

```bash
# Test cache statistics
claude cache stats

# Test reading a file and verify it's cached
claude "Show me the content of package.json"
claude "Show me the content of package.json again" # Should use cache

# Check statistics to see hit count increased
claude cache stats
```

### 2. Write a Test Suite

Create tests in appropriate test directory:

```javascript
// Example test for FileCache class
describe('FileCache', () => {
  test('should store and retrieve file content', () => {
    // Test implementation
  });
  
  // More tests...
});
```

## Debugging

### 1. Enable Debug Logging

```bash
# Run with debug logging
NODE_DEBUG=claude:cache claude "Your command here"
```

### 2. Inspect Cache Contents

```bash
# View cache directory
ls -la ~/.claude/cache

# Examine metadata
cat ~/.claude/cache/metadata.json
```

### 3. Use Node Inspector

For deeper debugging:

```bash
# Run with Node Inspector
node --inspect $(which claude) "Your command here"

# Then connect Chrome DevTools to the inspector
```

## Performance Testing

### 1. Set Up Benchmark Scripts

Create a benchmark script that:
- Measures token usage with and without cache
- Times operations with various file sizes
- Tests cache hit rates in different scenarios

### 2. Run Benchmarks

```bash
# Run benchmark script
node benchmark.js
```

### 3. Analyze Results

Compare performance metrics and adjust cache parameters for optimal performance.

## Working with Bundled Code

If the code is bundled and source is not directly accessible:

### 1. Analyze Bundle

```bash
# Look at the bundled file structure
head -n 100 cli.js
```

### 2. Consider Alternative Approaches

1. **Wrapper Approach**: Create a wrapper that intercepts file operations
2. **Module Replacement**: Use Node.js module system to replace file operation modules
3. **Proxy Implementation**: Use JavaScript proxies to intercept operations

## Local Development Workflow

### 1. Make Changes

```bash
# Edit files
code src/cache/index.js

# Test changes
claude cache stats
```

### 2. Run Local Tests

```bash
# Run test suite
npm test
```

### 3. Create Commit

```bash
git add .
git commit -m "Add file caching system"
```

## Preparing for Pull Request

### 1. Update Documentation

Ensure all documentation is up to date:
- README.md updated with cache commands
- Configuration options documented
- Performance benefits quantified

### 2. Run Final Tests

```bash
# Run complete test suite
npm test

# Verify across platforms (if possible)
```

### 3. Create Pull Request

```bash
# Push changes
git push origin feature/file-caching

# Create PR through GitHub UI
```

## Tips and Best Practices

1. **Development Shortcuts**
   - Use `NODE_ENV=development` for additional debugging
   - Create aliases for common commands

2. **Testing Efficiently**
   - Create test fixtures for various file types
   - Test with both small and large files
   - Test edge cases like empty files and binary files

3. **Clean Development**
   - Regularly clear test cache: `rm -rf ~/.claude/cache/*`
   - Keep separate profiles for testing and development