# Optimized Kanji Processing Scripts

## Overview

This directory contains scripts for processing Japanese Kanji characters, fetching their definitions from KanjiAPI.dev, and generating comprehensive documentation in markdown files.

## Scripts

1. **fetch_kanji_definitions.js** - Original script for fetching kanji data
   - Reads Joyo Kanji from the provided list file
   - Fetches detailed information from KanjiAPI.dev
   - Formats the data into a readable markdown document
   - Handles API rate limiting with configurable batch sizes and delays

2. **fetch_kanji_split_files.js** - Modified script that creates separate basic and metadata files
   - Separates content into two specialized files
   - Same core functionality as the original script

3. **fetch_kanji_optimized.js** - Fully optimized version with multiple performance enhancements
   - Implements advanced optimizations for better performance and memory efficiency

## Optimized Script Features

The `fetch_kanji_optimized.js` script implements several advanced optimizations:

- **Streaming File Operations**: Uses Node.js streams to reduce memory usage when writing large files
- **HTTP/2 Connection Pooling**: Maintains persistent connections to the API for faster requests
- **Multi-threaded Processing**: Utilizes Node.js worker threads for parallel processing
- **SQLite Database Storage**: Stores fetched data in an SQLite database for efficient querying and storage
- **In-memory LRU Cache**: Implements an LRU cache to minimize disk I/O for frequently accessed data
- **Smart Batching with Exponential Backoff**: Adaptively adjusts batch sizes and implements exponential backoff on failures
- **Checkpoint-based Error Recovery**: Creates recovery checkpoints to resume from the last stable point
- **Kanji API Request Chunking**: Processes multiple kanji in a single batch where possible
- **Memory Usage Optimization**: Uses buffer management and garbage collection hints

## Dependencies

Dependencies are managed through npm. Before running the optimized script, install required packages:

```bash
cd /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts
npm install
```

The required packages are:
- `sqlite3`: SQLite database integration
- `lru-cache`: Efficient memory caching
- `http2-wrapper`: Enhanced HTTP/2 support

## Usage

### Basic Usage

```bash
node fetch_kanji_optimized.js
```

### Advanced Options

```bash
# Start from a specific index
node fetch_kanji_optimized.js --startIndex 100

# Adjust batch size
node fetch_kanji_optimized.js --batchSize 15

# Adjust concurrency
node fetch_kanji_optimized.js --concurrency 6

# Adjust batch delay (milliseconds)
node fetch_kanji_optimized.js --batchDelay 2000

# Disable features
node fetch_kanji_optimized.js --noCache --noDatabase --noWorkers

# Run with garbage collection hints
node --expose-gc fetch_kanji_optimized.js

# Run with increased memory allocation
node --max-old-space-size=4096 fetch_kanji_optimized.js
```

## Output Files

The optimized script generates two markdown files:

1. **JAPANESE_KANJI_BASIC.md**: Contains the kanji character, meanings, and descriptions
2. **JAPANESE_KANJI_METADATA.md**: Contains technical details like Unicode values, readings, stroke counts, and JLPT levels

Additionally, the script maintains these auxiliary files:

- **kanji_progress.json**: Tracks processing progress
- **kanji_checkpoints.json**: Stores recovery checkpoints
- **kanji.db**: SQLite database with all processed kanji (when database mode is enabled)
- **kanji_cache/**: Directory containing cached API responses

## Performance Considerations

- The script automatically adapts its behavior based on API response patterns
- For large datasets, the worker thread implementation offers significant performance benefits
- Memory usage is carefully managed to prevent out-of-memory errors
- Database mode provides the best performance for subsequent operations and exports

## File Structure

- **Input**: `/joyo_kanji_list.txt` - List of Joyo Kanji characters
- **Output**: 
  - `/CACHE/JAPANESE_KANJI_BASIC.md` - Basic kanji information
  - `/CACHE/JAPANESE_KANJI_METADATA.md` - Technical kanji metadata
  - `/CACHE/kanji.db` - SQLite database (when database mode is enabled)

## Resuming Interrupted Processing

All scripts include automatic resumption capabilities. If interrupted, they will automatically resume from the last saved point when restarted. The optimized version includes more robust checkpoint-based recovery.

## Credits

- Kanji data is provided by [KanjiAPI.dev](https://kanjiapi.dev/)
- Joyo Kanji list is sourced from the Japanese Ministry of Education standards
- This script was developed as part of the CollaborativeIntelligence project and optimized by the Developer and Optimizer agents