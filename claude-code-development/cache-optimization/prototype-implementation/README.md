# Claude Code Cache System

A non-invasive caching wrapper for the Claude Code CLI that adds offline/cached mode and reduces token usage.

## Overview

This implementation provides a wrapper-based approach to caching file operations in Claude Code without modifying the core CLI. It maintains full compatibility with the Anthropic API while adding local caching capabilities, including a new offline/cached mode for working without API calls.

## Installation

1. Make sure the original Claude Code CLI is installed:
   ```
   npm install -g @anthropic-ai/claude-code
   ```

2. Run the installer:
   ```
   bash install.sh
   ```

3. The installer will:
   - Create necessary directories
   - Install the wrapper scripts
   - Create a `claude` wrapper command
   - Add the installation directory to your PATH (if needed)

## Usage

The wrapper provides a drop-in replacement for the Claude CLI:

```bash
# Run in cached mode (default, no API calls)
claude "What does this file do?"

# Force online mode for a single query
claude --query "What does this file do?" 
# or
claude -q "What does this file do?"

# Switch to online mode (makes API calls)
claude !default

# Switch back to cached mode (no API calls)
claude !cached

# Check current mode and cache statistics
claude !status
```

### Cache Management

```bash
# Show cache statistics
claude cache stats

# Clear all cached files
claude cache clear

# Disable caching (persists between sessions)
claude cache disable

# Enable caching
claude cache enable

# Toggle debug mode
claude cache debug on
claude cache debug off
```

## How It Works

This wrapper approach:

1. Intercepts all `claude` commands by creating a wrapper
2. Defaults to cached mode (offline, no API calls)
3. Allows toggling between cached and online modes
4. Supports one-time online queries with `--query` or `-q`
5. Saves both cache and session state between terminal sessions
6. Automatically caches files when detected in queries
7. Provides file content previews in cached mode

### Key Features

1. **Cached Mode (Default)**
   - All queries use only cached data
   - No API calls are made
   - Safe for offline use or to avoid token usage

2. **Session Management**
   - `!cached` - Switch to cached mode
   - `!default` - Switch to online mode
   - `!status` - View current mode and cache stats
   - Session state persists between terminal sessions

3. **One-Time Online Queries**
   - `--query` or `-q` for single online queries
   - Returns to cached mode after the query

4. **File Operations**
   - Automatic file detection in queries
   - Intelligent caching of referenced files
   - Preview of cached file contents
   - Support for both absolute and relative paths

5. **Safety Features**
   - Clear instructions for switching modes
   - Emergency escape back to online mode if issues occur
   - Detailed status information

### Implementation Status

**Phase 1: ✅ Completed**
- Fully functional mode switching system
- Cache management commands
- Session persistence
- Claude command wrapping

**Phase 2: ✅ Completed**
- Automatic file detection from queries
- Background caching of files
- Local file previews in cached mode
- Smart path resolution

**Phase 3: 🔄 Planned**
- Local semantic processing
- Command history caching
- Advanced query parsing
- Intelligent file tracking

## Troubleshooting

If you encounter any issues:

1. Switch to online mode:
   ```
   claude !default
   ```

2. Check cache status:
   ```
   claude cache stats
   ```

3. Try clearing the cache:
   ```
   claude cache clear
   ```

4. View your current settings:
   ```
   claude !status
   ```

5. Check your installation:
   ```
   which claude
   ```

## Uninstallation

To remove the wrapper:

```
rm ~/.local/bin/claude
rm ~/.local/bin/claude-cache
rm ~/.local/bin/claude-cache-module.js
```

Your original Claude CLI will still be available after removing the wrapper.

---

## Recent Implementation Changes

### Phase 2 Implementation (2024-04-24)

The cache system has been enhanced with actual file processing capabilities:

1. **File Reference Extraction**
   - Added natural language parsing to detect file references in queries
   - Implemented regex-based detection for various query phrasings
   - Added support for both absolute and relative file paths

2. **Automatic Caching**
   - Added background file caching when using online mode
   - Files referenced in queries are automatically detected and cached
   - Binary file detection to avoid caching non-text files

3. **Cached Mode Processing**
   - Implemented file content preview functionality
   - Added detailed response formatting for cached files
   - Created helpful error messages for missing files

4. **Cache Validation**
   - Added timestamp checking to detect file modifications
   - Implemented file freshness validation

All agents working on this codebase should include a summary of their changes at the end of modified files, separated by a line (---). This summary should include:
1. What changes were made
2. Why they were made
3. Any important implementation details

The summary should be brief but comprehensive enough for other agents to understand the purpose and scope of the changes.