# Knowledge Distribution System

This directory contains the scripts that implement the Knowledge Distribution Architecture, automating the identification, transformation, and distribution of globally relevant knowledge across the Collaborative Intelligence system.

## Components

### 1. Knowledge Indexing

**Script**: `index_knowledge.py`

**Purpose**: Scans the repository for global knowledge artifacts and updates the Knowledge Registry with their metadata.

**Usage**:
```bash
./index_knowledge.py [--dir DIR] [--update]
```

**Options**:
- `--dir DIR`: Specify the root directory to scan (default: current directory)
- `--update`: Update existing index entries instead of just adding new ones

### 2. Global Content Identification

**Script**: `identify_global_content.py`

**Purpose**: Scans agent sessions for content that should be distributed globally, based on metadata and content analysis.

**Usage**:
```bash
./identify_global_content.py [--dir DIR] [--threshold THRESHOLD] [--report-only]
```

**Options**:
- `--dir DIR`: Specify the root directory to scan (default: current directory)
- `--threshold THRESHOLD`: Minimum relevance score (0.0-1.0) to identify content (default: 0.7)
- `--report-only`: Generate report without initiating distribution

### 3. Knowledge Transformation

**Script**: `transform_knowledge.py`

**Purpose**: Transforms agent-specific knowledge artifacts into standardized formats suitable for global distribution.

**Usage**:
```bash
./transform_knowledge.py --input INPUT [--output OUTPUT] [--type TYPE] [--simulate]
```

**Options**:
- `--input INPUT`: Input file to transform (required)
- `--output OUTPUT`: Output location (defaults to derived location)
- `--type TYPE`: Content type (architecture, plan, documentation, etc.)
- `--simulate`: Show transformation without writing files

### 4. Knowledge Distribution (Coming Soon)

**Script**: `distribute_knowledge.py`

**Purpose**: Distributes transformed knowledge artifacts to appropriate global directories and updates the Knowledge Registry.

## Complete Workflow

The Knowledge Distribution System operates through this workflow:

1. **Identification**: Scan agent sessions to identify globally relevant content
   ```bash
   ./identify_global_content.py
   ```
   
2. **Transformation**: For each identified file, transform to standardized format
   ```bash
   ./transform_knowledge.py --input AGENT_FILE --type CONTENT_TYPE
   ```
   
3. **Distribution**: Move transformed content to appropriate global location
   ```bash
   # Coming soon
   ```
   
4. **Indexing**: Update the Knowledge Registry with new global content
   ```bash
   ./index_knowledge.py --update
   ```

## Automation

For automated execution of the complete workflow, a combined script will be implemented that:

1. Identifies globally relevant content
2. Transforms each identified file
3. Distributes the transformed content
4. Updates the Knowledge Registry

This script will be scheduled to run regularly, ensuring that knowledge flows smoothly from agent sessions to global repositories.

## Implementation Status

- ✅ Knowledge Registry Foundation
- ✅ Knowledge Indexing Script
- ✅ Global Content Identification Script
- ✅ Knowledge Transformation Script
- ⬜ Knowledge Distribution Script
- ⬜ Automated Workflow Script

---

Last Updated: 2025-05-18