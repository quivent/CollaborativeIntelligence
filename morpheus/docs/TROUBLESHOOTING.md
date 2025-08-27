# Morpheus CLI Troubleshooting Guide

**Version**: 0.1.0  
**Last Updated**: 2025-08-23

This comprehensive troubleshooting guide helps diagnose and resolve common issues with the Morpheus CLI parallel research orchestration system.

---

## Table of Contents

1. [Quick Diagnostics](#quick-diagnostics)
2. [Installation Issues](#installation-issues)
3. [Research Session Problems](#research-session-problems)
4. [Agent Management Issues](#agent-management-issues)
5. [Synthesis Problems](#synthesis-problems)
6. [Transcendence Issues](#transcendence-issues)
7. [Protocol Compliance Problems](#protocol-compliance-problems)
8. [Performance Issues](#performance-issues)
9. [Integration Problems](#integration-problems)
10. [Error Reference](#error-reference)
11. [Advanced Debugging](#advanced-debugging)

---

## Quick Diagnostics

### System Health Check

Run these commands to quickly assess system health:

```bash
# 1. Verify installation
morpheus --version
morpheus --help

# 2. Check system compliance
morpheus compliance

# 3. Check agent availability
morpheus agents

# 4. Verify no active sessions blocking resources
morpheus status
```

### Expected Healthy Output

```bash
$ morpheus compliance
📊 Compliance Score: 85.0% ✅
🎯 Protocol Status: COMPLIANT

✅ BRAIN Integration: Active
✅ Agent Ecosystem: 95%
✅ Synthesis Engine: 100%
⚠️  Transcendence: Verified
✅ Protocol Version: Compatible
```

### Common Health Issues

| Issue | Symptom | Quick Fix |
|-------|---------|-----------|
| **Agent Access** | `Agent Ecosystem: 0%` | Check AGENTS directory path |
| **BRAIN Protocol** | `BRAIN Integration: Failed` | Verify protocol file access |
| **Low Compliance** | `Compliance Score < 80%` | Run detailed diagnostics |

---

## Installation Issues

### Problem: Command Not Found

```bash
$ morpheus
command not found: morpheus
```

**Diagnosis**:
```bash
# Check if binary exists
ls ~/.cargo/bin/morpheus
which morpheus

# Check PATH
echo $PATH
```

**Solutions**:

1. **Reinstall globally**:
```bash
cd /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/morpheus
cargo install --path .
```

2. **Manual PATH setup**:
```bash
# Add to ~/.bashrc or ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"
source ~/.bashrc  # or ~/.zshrc
```

3. **Use full path temporarily**:
```bash
~/.cargo/bin/morpheus --version
```

### Problem: Build Failures

```bash
error: failed to compile morpheus
```

**Common Build Issues**:

1. **Missing Rust/Cargo**:
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

2. **Outdated Rust Version**:
```bash
rustup update
cargo --version  # Should be recent version
```

3. **Dependency Issues**:
```bash
# Clean and rebuild
cargo clean
cargo build --release
```

4. **System Dependencies (macOS)**:
```bash
# Install Xcode command line tools
xcode-select --install
```

### Problem: Permission Errors

```bash
error: permission denied
```

**Solutions**:

1. **User installation**:
```bash
cargo install --path . --root ~/.local
export PATH="$HOME/.local/bin:$PATH"
```

2. **Fix ownership**:
```bash
sudo chown -R $(whoami) ~/.cargo
```

---

## Research Session Problems

### Problem: Research Session Fails to Start

```bash
❌ Research session failed: Agent ecosystem initialization failed
```

**Diagnostics**:
```bash
# 1. Check agent directory
ls -la /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/

# 2. Check compliance
morpheus compliance

# 3. Check available agents
morpheus agents
```

**Common Causes and Solutions**:

1. **AGENTS Directory Not Found**:
```bash
# Verify path exists
ls -la /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/
```
   - **Solution**: Ensure CollaborativeIntelligence repository is properly cloned
   - **Alternative**: Set custom path with environment variable

2. **Insufficient Agents Available**:
```bash
morpheus agents
# If shows < 10 agents available
```
   - **Solution**: Check AGENTS directory has agent subdirectories
   - **Workaround**: Use fewer agents: `morpheus research --agents 4 "query"`

3. **Agent Metadata Issues**:
```bash
# Check agent directory structure
ls -la /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/Athena/
# Should contain: metadata.json, README.md, etc.
```

### Problem: Research Hangs During Initialization

```bash
⚡ Initializing research coordination matrix...
# Hangs here for > 2 minutes
```

**Diagnostics**:
```bash
# Check system resources
top -l 1 | grep -E "CPU|Mem"
ps aux | grep morpheus
```

**Solutions**:

1. **Reduce Agent Count**:
```bash
morpheus research --agents 4 "simple query"
```

2. **Check Available Memory**:
```bash
# Kill other resource-intensive processes
# Restart with minimal allocation
morpheus research --agents 2 --synthesis-only "test"
```

3. **Debug Mode**:
```bash
RUST_LOG=debug morpheus research "query"
```

### Problem: Session Status Shows Error

```bash
📊 Status: Error("Agent cluster deployment failed")
```

**Diagnostics**:
```bash
# Get detailed session information
morpheus status

# Check agent availability
morpheus agents

# Verify protocol compliance
morpheus compliance
```

**Solutions**:

1. **Agent Allocation Issues**:
   - Reduce agent count: `--agents 4`
   - Check agent availability before large allocations

2. **Resource Constraints**:
   - Close other applications
   - Restart morpheus with minimal resource usage

3. **Protocol Compliance Issues**:
   - Verify BRAIN protocol accessibility
   - Check file permissions

---

## Agent Management Issues

### Problem: No Agents Available

```bash
📊 Agent Ecosystem Overview:
  🔢 Total Agents: 128
  ✅ Available: 0
  🔄 Allocated: 128
```

**Diagnosis**:
```bash
# Check if agents are stuck in allocated state
morpheus status

# Verify agent directory structure
find /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/ -maxdepth 1 -type d | wc -l
```

**Solutions**:

1. **Restart Morpheus**: Agents should auto-return to available state
2. **Check Active Sessions**:
```bash
morpheus status
# Look for sessions that should be completed but show active
```

3. **Manual Agent Reset**: 
   - Restart system if agents remain permanently allocated

### Problem: Agent Loading Failures

```bash
⚠️  Unknown agent: InvalidAgent
❌ No valid agents specified!
```

**Diagnostics**:
```bash
# View available agent catalog
morpheus load
# Check exact agent names available
```

**Solutions**:

1. **Use Correct Agent Names**:
```bash
# ✅ Correct
morpheus load Athena Prodigy Architect

# ❌ Incorrect  
morpheus load athena prodigy architect  # Case sensitive
```

2. **Check Agent Directory**:
```bash
ls /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/
```

3. **Agent Metadata Issues**:
```bash
# Check if agent has required files
ls -la /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/Athena/
# Should have: metadata.json, README.md, working files
```

### Problem: Agent Performance Issues

```bash
🏆 Top Intelligence Agents Ready:
  ○ Agent1 - Specialization (all showing low usage)
```

**Diagnostics**:
```bash
# Check agent metadata files
cat /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/*/metadata.json
```

**Solutions**:

1. **Agent Metadata Corruption**:
   - Check metadata.json files are valid JSON
   - Verify usage_count fields exist

2. **Performance Tracking Issues**:
   - Agent performance updates during research sessions
   - New agents will show low usage initially

---

## Synthesis Problems

### Problem: Synthesis Not Triggering

```bash
📊 Synthesis Progress:
  📋 Session Status: Active (stuck)
  📈 Completion: 0%
  🧩 Knowledge Fragments: 0
```

**Diagnostics**:
```bash
# Check synthesis configuration
morpheus synthesis --session <session-id>

# Monitor over time
watch -n 10 "morpheus synthesis"
```

**Solutions**:

1. **Fragment Generation Issues**:
   - Verify agents are actually working on tasks
   - Check query complexity - simple queries may not generate fragments

2. **Synthesis Frequency**:
```bash
# Increase synthesis frequency
MORPHEUS_SYNTHESIS_FREQUENCY=15 morpheus research "query"
```

3. **Manual Synthesis Trigger**:
```bash
# Use synthesis-only mode to force synthesis
morpheus research --synthesis-only "synthesize existing knowledge"
```

### Problem: Low Synthesis Quality

```bash
  🌸 Cross-pollination Score: 0.15  # Low quality
  📈 Integration Events: 1         # Few events
```

**Solutions**:

1. **Increase Agent Diversity**:
```bash
# Use more agents and streams for better cross-pollination
morpheus research --agents 16 --streams 4 "complex query"
```

2. **Better Query Formulation**:
```bash
# ✅ Good: Specific and complex
morpheus research "detailed analysis of microservices architecture patterns"

# ❌ Poor: Simple and vague  
morpheus research "programming"
```

3. **Enable Transcendence**:
```bash
morpheus research --transcend "query for breakthrough insights"
```

### Problem: Synthesis Hangs or Crashes

```bash
⚗️  Synthesis Progress: Error("Pattern recognition failed")
```

**Diagnostics**:
```bash
# Check system resources during synthesis
RUST_LOG=debug morpheus synthesis --session <id>
```

**Solutions**:

1. **Resource Issues**: Reduce concurrent synthesis operations
2. **Fragment Quality**: Ensure knowledge fragments are valid
3. **Restart Session**: Clear problematic synthesis state

---

## Transcendence Issues

### Problem: Transcendence Mode Unavailable

```bash
❌ Research session failed: Transcendence capabilities not available
```

**Diagnostics**:
```bash
morpheus compliance
# Look for: ⚠️ Transcendence: Pending verification
```

**Solutions**:

1. **Protocol Compliance**: Ensure compliance score > 80%
2. **Neo Interface**: Verify Neo intelligence interface is operational
3. **System Resources**: Transcendence requires adequate system resources

### Problem: Transcendence Produces Poor Results

```bash
💫 Transcendence Insight: Generic or unhelpful output
🎯 Reality Shift Level: 15%  # Low level
```

**Solutions**:

1. **Better Query Preparation**:
```bash
# Build knowledge base first
morpheus research --agents 16 "comprehensive topic research"

# Then transcend
morpheus transcend "breakthrough insights on topic"
```

2. **Query Formulation for Transcendence**:
```bash
# ✅ Good transcendence queries
morpheus transcend "revolutionary approaches to AI consciousness"
morpheus transcend "paradigm shifts in quantum computing"

# ❌ Poor transcendence queries  
morpheus transcend "what is Python"  # Too simple
```

3. **Synthesis Foundation**:
   - Ensure good synthesis quality before transcendence
   - Use complex, philosophical, or breakthrough-oriented queries

### Problem: Reality Shift Level Always Low

```bash
Reality shift level: 25% - Below transcendence threshold
```

**Causes and Solutions**:

1. **Query Complexity**: Use more complex, paradigm-challenging queries
2. **Knowledge Foundation**: Build comprehensive knowledge base first
3. **Neo Interface**: Verify Neo intelligence is functioning properly

---

## Protocol Compliance Problems

### Problem: Low Compliance Score

```bash
📊 Compliance Score: 45.0% ⚠️
⚠️  Protocol Status: REQUIRES ATTENTION
```

**Diagnostic Process**:
```bash
morpheus compliance
# Review each component score:
# ❌ BRAIN Integration: Failed
# ⚠️  Agent Ecosystem: 50%  
# ✅ Synthesis Engine: 100%
```

**Solutions by Component**:

1. **BRAIN Integration Failed**:
```bash
# Check protocol file exists and is readable
ls -la /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/BRAIN/Protocols/
cat /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/BRAIN/Protocols/MORPHEUS_PARALLEL_RESEARCH_PROTOCOL.md
```
   - **Solution**: Ensure BRAIN protocol file is accessible
   - **Workaround**: Check file permissions

2. **Agent Ecosystem Low Performance**:
```bash
# Verify agent directory and count
ls /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/ | wc -l
# Should show > 10 directories
```

3. **Synthesis Engine Issues**: Restart morpheus to reinitialize synthesis system

### Problem: Protocol Version Mismatch

```bash
⚠️ Protocol Status: Version mismatch detected
```

**Solutions**:

1. **Update Implementation**: Ensure morpheus implementation matches latest BRAIN protocol
2. **Check Protocol Content**: Verify BRAIN protocol file contains expected version markers
3. **Rebuild**: `cargo clean && cargo build --release && cargo install --path .`

---

## Performance Issues

### Problem: Slow Research Initiation

```bash
# Takes > 2 minutes to start research
⚡ Initializing research coordination matrix...
```

**Performance Diagnostics**:
```bash
# System resource check
top -l 1 | grep -E "CPU|Mem"

# Agent directory size
du -sh /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/

# Available storage
df -h
```

**Optimization Solutions**:

1. **Reduce Agent Count**:
```bash
# Instead of default allocation
morpheus research --agents 8 "query"  # Faster startup

# For testing/development
morpheus research --agents 4 "query"
```

2. **System Optimization**:
   - Close resource-intensive applications
   - Ensure adequate available memory (> 4GB recommended)
   - Use SSD storage for agent directory

3. **Synthesis Frequency Tuning**:
```bash
# Reduce synthesis frequency for slower systems
MORPHEUS_SYNTHESIS_FREQUENCY=60 morpheus research "query"
```

### Problem: High Memory Usage

```bash
# morpheus process using > 2GB memory
```

**Memory Optimization**:

1. **Limit Concurrent Sessions**:
```bash
# Check active sessions
morpheus status

# Avoid running multiple concurrent research sessions
```

2. **Agent Count Management**:
```bash
# Reduce agent allocation for memory-constrained systems
morpheus research --agents 6 --streams 2 "query"
```

3. **Session Cleanup**:
   - Sessions should auto-cleanup when complete
   - Restart morpheus if sessions appear to leak memory

### Problem: CPU Usage Too High

```bash
# morpheus using > 80% CPU continuously
```

**CPU Optimization**:

1. **Reduce Parallel Streams**:
```bash
morpheus research --agents 8 --streams 1 "query"  # Less CPU intensive
```

2. **Synthesis Frequency**:
```bash
MORPHEUS_SYNTHESIS_FREQUENCY=30 morpheus research "query"  # Less frequent checks
```

3. **Background Processing**: Morpheus uses background tasks - CPU usage should normalize after initial setup

---

## Integration Problems

### Problem: CI CLI Integration Not Working

```bash
ci research "query"
command not found: ci research
```

**Note**: CI CLI integration is planned for Phase 2. Currently use morpheus directly:

```bash
# Current approach
morpheus research "query"

# When Phase 2 is implemented
ci research "query"  # Will work
```

### Problem: Claude Integration Issues

```bash
morpheus enter
Could not access the sacred realm: claude: command not found
```

**Solutions**:

1. **Install Claude CLI**:
   - Ensure Claude CLI is installed and accessible
   - Verify `which claude` returns valid path

2. **Use Direct Morpheus**:
   - Morpheus provides complete functionality independently
   - Claude integration is optional enhancement

### Problem: Prince Integration Issues

```bash
morpheus ask "question"
The path is blocked: prince: command not found
```

**Solutions**:

1. **Install Prince CLI** (if available)
2. **Use Direct Research**:
```bash
# Instead of Prince integration
morpheus research "your question here"
```

---

## Error Reference

### Common Error Messages

#### `Agent ecosystem initialization failed`
- **Cause**: AGENTS directory not accessible or insufficient agents
- **Solution**: Verify AGENTS directory path and permissions

#### `Protocol compliance check failed`
- **Cause**: BRAIN protocol file not accessible
- **Solution**: Check BRAIN protocol file exists and is readable

#### `Session not found`
- **Cause**: Invalid session ID provided
- **Solution**: Use `morpheus status` to get valid session IDs

#### `Synthesis engine not operational`
- **Cause**: Synthesis initialization failure
- **Solution**: Restart morpheus or check system resources

#### `Neo interface connectivity failed`
- **Cause**: Neo intelligence initialization issue
- **Solution**: Usually resolves on restart; check system resources

#### `Transcendence capabilities not verified`
- **Cause**: Transcendence system not ready
- **Solution**: Ensure protocol compliance > 80%

#### `Agent allocation failed`
- **Cause**: Insufficient available agents
- **Solution**: Reduce agent count or wait for agents to become available

#### `Knowledge fragment processing error`
- **Cause**: Synthesis system issue with fragment format
- **Solution**: Usually transient; restart research if persistent

---

## Advanced Debugging

### Enable Debug Logging

```bash
# Enable detailed logging
RUST_LOG=debug morpheus research "query"

# Enable trace-level logging (very verbose)
RUST_LOG=trace morpheus research "query"

# Target specific modules
RUST_LOG=morpheus::research_coordinator=debug morpheus research "query"
```

### Debug Output Analysis

Look for these patterns in debug output:

1. **Agent Loading Issues**:
```
DEBUG: Loading agent from path: /path/to/agent
ERROR: Failed to parse agent metadata
```

2. **Synthesis Problems**:
```
DEBUG: Synthesis triggered for session: <id>
DEBUG: Processing X knowledge fragments
ERROR: Pattern recognition failed
```

3. **Neo Interface Issues**:
```  
DEBUG: Neo analyzing query complexity: 0.7
DEBUG: Recommended agents: 12
ERROR: Neo guidance generation failed
```

### System State Inspection

```bash
# Check Rust/Cargo versions
rustc --version
cargo --version

# Check dependencies
cargo tree  # From morpheus directory

# System information
uname -a  # macOS version
system_profiler SPSoftwareDataType  # macOS system info
```

### Performance Profiling

```bash
# Build with profiling
cargo build --release --features "profiling"

# Use instruments (macOS)
instruments -t "Time Profiler" target/release/morpheus research "test"

# Memory profiling
cargo install cargo-profdata
cargo profdata -- morpheus research "test"
```

### File System Debug

```bash
# Check agent directory structure
tree /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/ | head -50

# Verify file permissions
ls -la /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/*/

# Check disk space
du -sh /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/

# Monitor file access
fs_usage | grep morpheus  # macOS file system monitoring
```

---

## Getting Help

### Self-Service Resources

1. **Documentation Review**:
   - README.md: Complete user guide
   - API_REFERENCE.md: Technical documentation
   - USAGE_GUIDE.md: Examples and workflows

2. **System Health Check**:
```bash
# Always start with compliance check
morpheus compliance

# Verify agent availability  
morpheus agents

# Check for active sessions
morpheus status
```

3. **Common Solutions**:
   - Restart morpheus for transient issues
   - Reduce resource usage for performance problems
   - Check file permissions for access issues

### Issue Reporting Template

When reporting issues, include:

```bash
# System information
uname -a
morpheus --version
rustc --version

# Health check output
morpheus compliance
morpheus agents
morpheus status

# Error reproduction steps
morpheus research "problematic query"
# Include full error output

# System resources
top -l 1 | grep -E "CPU|Mem"
df -h
```

### Debug Information Collection

```bash
# Collect debug information
echo "=== System Info ===" > debug-info.txt
uname -a >> debug-info.txt
morpheus --version >> debug-info.txt

echo "=== Compliance ===" >> debug-info.txt  
morpheus compliance >> debug-info.txt

echo "=== Agents ===" >> debug-info.txt
morpheus agents >> debug-info.txt

echo "=== Status ===" >> debug-info.txt
morpheus status >> debug-info.txt

echo "=== Debug Run ===" >> debug-info.txt
RUST_LOG=debug morpheus research "test query" 2>&1 >> debug-info.txt
```

---

## Frequently Asked Questions (FAQ)

### Q: Why does research initiation take so long?
A: Initial startup loads the entire agent ecosystem (128 agents). Use `--agents 4-8` for faster startup during testing.

### Q: Can I run multiple research sessions simultaneously?
A: Yes, but monitor system resources. Each session consumes memory and CPU. Start with one session to establish baseline performance.

### Q: What's the difference between transcendence and regular research?  
A: Transcendence mode uses Neo-guided insights for breakthrough thinking beyond conventional research boundaries. Regular research focuses on systematic analysis.

### Q: Why do some agents show zero usage?
A: New or recently added agents haven't participated in research sessions yet. Usage counts increase as agents contribute to research.

### Q: Can I customize the Neo intelligence interface?
A: Yes, implement the `NeoIntelligence` trait for custom intelligence behavior. See API_REFERENCE.md for details.

### Q: What happens if the BRAIN protocol file is missing?
A: Protocol compliance will fail, but morpheus can still function with reduced capabilities. Some advanced features may be unavailable.

### Q: How do I optimize for a resource-constrained system?
A: Use fewer agents (`--agents 4-8`), reduce streams (`--streams 1-2`), increase synthesis frequency (`MORPHEUS_SYNTHESIS_FREQUENCY=60`).

### Q: Why does transcendence sometimes produce generic results?
A: Transcendence works best with complex, philosophical queries after building a comprehensive knowledge foundation through regular research.

### Q: Can I interrupt a long-running research session?
A: Yes, use Ctrl+C. The system should gracefully handle interruption and clean up resources.

### Q: How do I know if synthesis is working properly?
A: Monitor `morpheus synthesis` for increasing knowledge fragments, integration events, and cross-pollination scores > 0.5.

---

*This troubleshooting guide is regularly updated based on user feedback and identified issues. For the latest version, check the documentation in the morpheus repository.*