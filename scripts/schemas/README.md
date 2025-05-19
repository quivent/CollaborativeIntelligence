# Memory Optimization Schema Documentation

This directory contains JSON schema definitions for the memory optimization system. These schemas define the structure and format of data exchanged between components.

## Core Schemas

### AgentCacheEntry

Represents an agent activation record:

```json
{
  "agent_name": "string",   // Name of the agent
  "directory": "string",    // Working directory
  "initializer": "string",  // Entity that activated the agent
  "timestamp": 1621506789,  // Unix timestamp of activation
  "intent": "string",       // Purpose of activation
  "status": "Active"        // Current agent status (Active/Inactive)
}
```

### MemorySegment

Represents a piece of agent memory:

```json
{
  "id": "string",                     // Unique identifier
  "memory_type": "Semantic",          // Type of memory
  "tier": "Tier1",                    // Memory priority tier
  "domains": ["domain1", "domain2"],  // Knowledge domains
  "content": "string",                // Actual memory content
  "source_file": "string",            // Source file
  "line_start": 10,                   // Starting line number
  "line_end": 15,                     // Ending line number
  "last_accessed": 1621506789,        // Last access timestamp
  "access_count": 5,                  // Number of accesses
  "metadata": {}                      // Additional properties
}
```

### ChecksumEntry

Represents a checksum record for a memory file:

```json
{
  "file_path": "string",                   // Path to the memory file
  "checksum": "string",                    // MD5 checksum of the entire file
  "last_checked": 1621506789,              // Last check timestamp
  "segments_checksums": {                  // Map of segment ID to checksum
    "segment_id_1": "checksum_1",
    "segment_id_2": "checksum_2"
  },
  "metadata": {}                           // Additional properties
}
```

## Command Response Schemas

All CLI commands return a standardized response format:

### Success Response

```json
{
  "success": true,
  "message": "Human-readable message",
  "data": [operation-specific data],
  "error": null,
  "timestamp": 1621506789
}
```

### Error Response

```json
{
  "success": false,
  "message": "Operation failed",
  "data": null,
  "error": "Detailed error message",
  "timestamp": 1621506789
}
```

## Command-Specific Schemas

### Cache Command

Creates or updates an agent cache entry.

**Response:**
```json
{
  "success": true,
  "message": "Cache entry created for agent Athena",
  "data": "Athena_test_dir_Human_1621506789",
  "error": null,
  "timestamp": 1621506789
}
```

### Load Command

Loads memory segments for an agent.

**Response:**
```json
{
  "success": true,
  "message": "Loaded memory for agent Athena",
  "data": [
    {
      "id": "memory_segment_1",
      "memory_type": "Semantic",
      "tier": "Tier1",
      "domains": ["system", "architecture"],
      "content": "Memory architecture is a hierarchical structure.",
      "source_file": "MEMORY.md",
      "line_start": 10,
      "line_end": 12,
      "last_accessed": 1621506789,
      "access_count": 5,
      "metadata": {}
    }
  ],
  "error": null,
  "timestamp": 1621506789
}
```

### Validate Command

Validates memory files for changes.

**Response:**
```json
{
  "success": true,
  "message": "Validated memory files for agent Athena",
  "data": [
    "/path/to/MEMORY.md",
    "/path/to/ContinuousLearning.md"
  ],
  "error": null,
  "timestamp": 1621506789
}
```

### Status Command

Shows the status of all active agents.

**Response:**
```json
{
  "success": true,
  "message": "Found 3 active agents",
  "data": [
    "Athena",
    "Developer",
    "Optimizer"
  ],
  "error": null,
  "timestamp": 1621506789
}
```

### Migrate Command

Migrates data from old format to new format.

**Response:**
```json
{
  "success": true,
  "message": "Migration completed successfully",
  "data": 42,  // Number of entries migrated
  "error": null,
  "timestamp": 1621506789
}
```

## Schema Validation

These schemas can be used to validate data during development and testing. Use a JSON schema validator to ensure that your data conforms to these schemas.

```bash
# Example validation using ajv
npx ajv validate -s memory_cli_schemas.json -d data.json
```

## Versioning

Schema versioning is managed through a version field in the schema definitions. When making changes to schemas, increment the version number and maintain backward compatibility whenever possible.

For more detailed information, see the main `memory_cli_schemas.json` file.