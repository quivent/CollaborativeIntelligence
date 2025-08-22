# File Caching Benchmark Methodology

This document outlines the methodology for measuring the performance impact of the file caching system in Claude Code.

## Key Metrics

1. **Token Usage Reduction**
   - Percentage reduction in total tokens used for the same operations
   - Absolute token count savings

2. **Cache Efficiency**
   - Cache hit rate (percentage of file accesses served from cache)
   - Cache invalidation rate (percentage of cache entries that become invalid)

3. **Response Time**
   - Time to complete common operations with and without cache
   - Latency reduction for frequent operations

4. **Resource Utilization**
   - Memory usage of the cache
   - Disk space used by persistent cache

## Benchmark Scenarios

### 1. Iterative Development Workflow

Simulate a typical development flow with frequent modifications to the same files:

1. Read a file to understand its structure
2. Make a small edit to the file
3. Read the file again to verify changes
4. Make another edit to the same file
5. Read related files to understand dependencies
6. Return to the original file for additional edits

**Expected Outcome:** 40-60% reduction in token usage

### 2. Exploration Workflow

Simulate exploring a new codebase:

1. List files in the repository
2. Read several files to understand structure
3. Search for specific patterns across files
4. Read files found in the search
5. Return to previously read files for comparison

**Expected Outcome:** 30-40% reduction in token usage

### 3. Documentation Writing

Simulate writing documentation for a codebase:

1. Read source files to understand functionality
2. Create documentation files
3. Re-read source files to verify accuracy
4. Edit documentation files
5. Review documentation and source side by side

**Expected Outcome:** 60-70% reduction in token usage

### 4. Large File Operations

Work with large files (>100KB):

1. Read a large file
2. Make minor edits to specific sections
3. Re-read the file to verify changes
4. Make additional edits
5. Read the file a final time

**Expected Outcome:** 70-80% reduction in token usage

## Measurement Tools

### Token Usage Tracking

Implement a token counter that:
1. Records tokens used for each file operation
2. Tracks cumulative token usage
3. Compares with and without cache

```javascript
class TokenTracker {
  constructor() {
    this.requestCounts = {
      withCache: 0,
      withoutCache: 0
    };
    
    this.tokenCounts = {
      withCache: 0,
      withoutCache: 0
    };
    
    this.fileCounts = {
      withCache: new Map(),
      withoutCache: new Map()
    };
  }
  
  recordOperation(file, tokens, mode) {
    this.requestCounts[mode]++;
    this.tokenCounts[mode] += tokens;
    
    const currentCount = this.fileCounts[mode].get(file) || 0;
    this.fileCounts[mode].set(file, currentCount + 1);
  }
  
  getReduction() {
    return {
      requests: 1 - (this.requestCounts.withCache / this.requestCounts.withoutCache),
      tokens: 1 - (this.tokenCounts.withCache / this.tokenCounts.withoutCache)
    };
  }
  
  getReport() {
    const reduction = this.getReduction();
    return {
      requests: {
        withCache: this.requestCounts.withCache,
        withoutCache: this.requestCounts.withoutCache,
        reduction: reduction.requests * 100
      },
      tokens: {
        withCache: this.tokenCounts.withCache,
        withoutCache: this.tokenCounts.withoutCache,
        reduction: reduction.tokens * 100
      },
      fileAccesses: {
        withCache: Object.fromEntries(this.fileCounts.withCache),
        withoutCache: Object.fromEntries(this.fileCounts.withoutCache)
      }
    };
  }
}
```

### Performance Measurement

Add timing instrumentation:

```javascript
async function measureOperation(operation, params) {
  const start = process.hrtime.bigint();
  const result = await operation(...params);
  const end = process.hrtime.bigint();
  
  return {
    result,
    durationNs: Number(end - start)
  };
}
```

### Cache Analytics

Monitor cache behavior:

```javascript
function getCacheAnalytics() {
  return {
    entries: fileCache.cache.size,
    hitRate: fileCache.stats.hits / (fileCache.stats.hits + fileCache.stats.misses),
    invalidationRate: fileCache.stats.invalidations / fileCache.cache.size,
    averageEntrySize: fileCache.stats.bytesStored / fileCache.cache.size,
    totalSize: fileCache.stats.bytesStored
  };
}
```

## Benchmark Runner

Create an automated benchmark runner:

```javascript
async function runBenchmark(scenario, options = {}) {
  // Initialize trackers
  const tokenTracker = new TokenTracker();
  const timings = {
    withCache: [],
    withoutCache: []
  };
  
  // Run without cache
  fileCache.options.enabled = false;
  console.log(`Running benchmark "${scenario.name}" without cache...`);
  const baselineStart = process.hrtime.bigint();
  await scenario.run(false, tokenTracker, timings);
  const baselineDuration = process.hrtime.bigint() - baselineStart;
  
  // Clear and reset
  fileCache.clear();
  
  // Run with cache
  fileCache.options.enabled = true;
  console.log(`Running benchmark "${scenario.name}" with cache...`);
  const cacheStart = process.hrtime.bigint();
  await scenario.run(true, tokenTracker, timings);
  const cacheDuration = process.hrtime.bigint() - cacheStart;
  
  // Calculate results
  const report = tokenTracker.getReport();
  const cacheAnalytics = getCacheAnalytics();
  const timeReduction = 1 - (Number(cacheDuration) / Number(baselineDuration));
  
  // Generate report
  return {
    scenario: scenario.name,
    tokenReduction: report.tokens.reduction.toFixed(2) + '%',
    requestReduction: report.requests.reduction.toFixed(2) + '%',
    timeReduction: (timeReduction * 100).toFixed(2) + '%',
    cacheStats: cacheAnalytics,
    details: {
      tokens: report.tokens,
      requests: report.requests,
      timing: {
        withCache: Number(cacheDuration) / 1_000_000, // to ms
        withoutCache: Number(baselineDuration) / 1_000_000, // to ms
      }
    }
  };
}
```

## Result Analysis

For each benchmark scenario, analyze:

1. **Token Usage Graph**
   - Plot tokens used with and without cache
   - Calculate percentage reduction

2. **Request Count Comparison**
   - Compare API requests with and without cache
   - Highlight eliminated redundant requests

3. **Performance Impact**
   - Measure latency improvements
   - Calculate time savings for common operations

4. **Efficiency Analysis**
   - Evaluate cache hit rate across scenarios
   - Identify patterns that benefit most from caching

## Expected Results

| Scenario | Token Reduction | Request Reduction | Time Improvement |
|----------|----------------|-------------------|------------------|
| Iterative Development | 40-60% | 50-70% | 30-40% |
| Exploration | 30-40% | 40-50% | 20-30% |
| Documentation | 60-70% | 70-80% | 50-60% |
| Large Files | 70-80% | 80-90% | 60-70% |

## Reproducibility

All benchmarks should:
1. Be deterministic with fixed seed values
2. Run in isolated environments
3. Include multiple iterations for statistical significance
4. Remove outliers for consistent results
5. Be executable by other developers to verify findings