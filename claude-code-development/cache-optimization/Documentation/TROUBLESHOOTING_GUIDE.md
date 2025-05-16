# Troubleshooting Guide: Claude Code File Caching

This guide provides solutions for common issues that may arise with the file caching system.

## Common Issues and Solutions

### Cache Not Working

**Symptoms:**
- No performance improvement
- Every file read results in a cache miss
- No cache statistics shown

**Possible Causes and Solutions:**

1. **Cache is Disabled**
   - Check if cache is enabled: `claude cache stats`
   - Enable cache: `claude config set cache.enabled true`

2. **Cache Directory Permissions**
   - Ensure `~/.claude/cache` directory has proper read/write permissions
   - Fix with: `chmod 755 ~/.claude/cache`

3. **Configuration Issues**
   - Reset cache configuration: `claude cache clear && claude config set cache.enabled true`
   - Verify config file: `cat ~/.claude/config.json`

### Cache Corruption

**Symptoms:**
- Errors when reading files
- Unexpected file contents
- Cache statistics show entries but performance doesn't improve

**Solutions:**

1. **Clear Cache**
   - Clear and reset cache: `claude cache clear`

2. **Rebuild Metadata**
   - Delete only metadata file: `rm ~/.claude/cache/metadata.json`
   - Next startup will rebuild metadata from existing cache files

3. **Verify File Integrity**
   - Run cache validation: `claude cache validate` (if implemented)

### High Memory Usage

**Symptoms:**
- Claude Code process consuming excessive memory
- Performance degradation over time

**Solutions:**

1. **Reduce Cache Size**
   - Set lower max cache size: `claude config set cache.maxSize 52428800` (50MB)

2. **Decrease TTL**
   - Set shorter time-to-live: `claude config set cache.ttl 1800000` (30 minutes)

3. **Disable Caching for Large Files**
   - Configure max file size: `claude config set cache.maxFileSize 1048576` (1MB)

### Cache Inconsistencies

**Symptoms:**
- Cached content doesn't match actual file content
- Changes made outside Claude Code aren't reflected

**Solutions:**

1. **Increase Validation Frequency**
   - Set stricter validation: `claude config set cache.validateOnRead true`

2. **Clear Specific Entry**
   - If implemented: `claude cache clear --file=/path/to/file`

3. **Disable and Re-enable Cache**
   - Reset cache state: `claude cache disable && claude cache enable`

### Performance Issues

**Symptoms:**
- Caching doesn't improve performance
- High disk I/O

**Solutions:**

1. **Optimize Cache Settings**
   - Increase memory cache ratio: `claude config set cache.memoryRatio 0.7`
   - Disable persistence for temporary improvements: `claude config set cache.persistOnExit false`

2. **Check Antivirus Interference**
   - Some antivirus software monitors file operations
   - Add cache directory to exclusions

3. **Analyze Cache Statistics**
   - Review hit/miss patterns: `claude cache stats`
   - Adjust TTL based on patterns: `claude config set cache.ttl <value>`

## Diagnostic Commands

### Check Cache Status

```bash
# View cache statistics
claude cache stats

# Check if cache is enabled
claude config get cache.enabled

# Verify cache directory exists
ls -la ~/.claude/cache
```

### Troubleshoot Cache Issues

```bash
# Clear cache completely
claude cache clear

# View cache configuration
claude config get cache

# Verify file system permissions
ls -la ~/.claude
```

### Debug Logging

If implemented:

```bash
# Enable debug logging
claude --debug cache

# Run a command with cache debug output
claude --debug cache help

# View recent cache logs
cat ~/.claude/logs/cache.log
```

## Environment Verification

### Node.js Version

```bash
# Check Node.js version
node --version

# Ensure version is 18.0.0 or higher
```

### Disk Space

```bash
# Check available disk space
df -h ~

# Ensure sufficient space for cache
```

### File System Issues

```bash
# Check for file system errors
touch ~/.claude/cache/test && rm ~/.claude/cache/test

# Verify no permission issues
```

## Recovery Procedures

### Complete Reset

If cache is completely broken:

1. Stop Claude Code process
2. Remove cache directory: `rm -rf ~/.claude/cache`
3. Remove cache config: `cat ~/.claude/config.json | jq 'del(.cache)' > ~/.claude/config.json.new && mv ~/.claude/config.json.new ~/.claude/config.json`
4. Restart Claude Code

### Recover From Corruption

1. Stop Claude Code process
2. Backup current cache: `cp -r ~/.claude/cache ~/.claude/cache_backup`
3. Clear only metadata: `rm ~/.claude/cache/metadata.json`
4. Restart Claude Code to rebuild metadata

### Emergency Fallback

If caching causes serious issues:

1. Disable cache via config file directly:
   ```bash
   cat > ~/.claude/config.json << EOF
   {
     "cache": {
       "enabled": false
     }
   }
   EOF
   ```
2. Restart Claude Code

## Reporting Issues

When reporting cache-related issues:

1. Include cache statistics: `claude cache stats`
2. Share cache configuration: `claude config get cache`
3. Describe steps to reproduce
4. Note your operating system and Node.js version
5. Include relevant error messages