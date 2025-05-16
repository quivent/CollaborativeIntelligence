# Session Organization - Universal Instructions

## Core Principles for All Agents

1. **Sessions are directories, not files**
   - Each significant interaction gets its own directory
   - Use descriptive, human-readable names without dates
   - Example: `Sessions/FastActivationSystem/` (correct)
   - Example: `Sessions/2025-04-23_FastActivationSystem.md` (incorrect)

2. **Required files in each session directory**
   - `README.md` - Overview, objectives, and outcomes
   - `metadata.json` - Date, participants, and technical details
   - Implementation files as needed

3. **Name everything for human comprehension**
   - Store dates and technical details in metadata
   - Use descriptive names that communicate purpose
   - Avoid numeric prefixes unless order is critical

4. **Organize knowledge hierarchically**
   - Detailed information in agent-specific sessions
   - Key principles in global Sessions directory
   - Essential instructions in universal files

## Implementation Instructions

### When creating a new session:
```
# Create directory with descriptive name
mkdir -p Sessions/DescriptiveSessionName

# Create required files
touch Sessions/DescriptiveSessionName/README.md
touch Sessions/DescriptiveSessionName/metadata.json

# Add implementation and reference files as needed
```

### Required metadata format:
```json
{
  "session_name": "DescriptiveSessionName",
  "date": "YYYY-MM-DD",
  "primary_agent": "AgentName",
  "supporting_agents": ["Agent1", "Agent2"],
  "session_type": "Type",
  "completion_status": "completed|in-progress",
  "version": "1.0"
}
```

## Knowledge Synchronization

For innovations with universal impact:
1. Update agent-specific session with full details
2. Tag metadata with high global relevance
3. Reference in global Sessions directory
4. Update universal instructions if needed

This approach ensures:
- Detailed knowledge remains available
- Critical updates propagate to all agents
- Information remains accessible and human-readable
- Knowledge can be efficiently discovered and processed