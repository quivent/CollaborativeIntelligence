# Knowledge Update Protocol

This document defines standardized triggers and processes for knowledge updates across the Collaborative Intelligence system, ensuring consistent learning practices for all agents.

## Learning Update Triggers

Every agent should update their knowledge structures in response to these standard triggers:

### 1. Session Completion Trigger
- **When**: After completing a significant session or interaction
- **What**: Document key insights, decisions, and outcomes
- **Where**: Create new session directory with proper structure
- **Example**: After implementing a new feature or solving a complex problem

### 2. Innovation Threshold Trigger
- **When**: Upon discovering novel approaches or patterns
- **What**: Document the innovation and its potential applications
- **Where**: Update ContinuousLearning.md with the new principle
- **Example**: Finding a more efficient way to organize session data

### 3. Frequency-Based Updates
- **When**: After 3-5 significant interactions
- **What**: Batch update of accumulated insights
- **Where**: ContinuousLearning.md for principles, session directories for details
- **Example**: Weekly review and documentation of learning points

### 4. Knowledge Gap Trigger
- **When**: Encountering missing or inconsistent documentation
- **What**: Create or update documentation to fill the gap
- **Where**: Appropriate knowledge file based on content type
- **Example**: Creating missing architecture documentation when needed

### 5. User-Directed Learning
- **When**: Explicit user request to document learning
- **What**: Structured documentation of specified knowledge
- **Where**: Location specified by user or most appropriate default
- **Example**: User asks agent to update memory with new insights

### 6. Session Transition Trigger
- **When**: Before switching to a different session or topic
- **What**: Summarize current session accomplishments and status
- **Where**: Update global Sessions directory with session summary
- **Example**: When switching from implementation work to planning a new feature

## Knowledge Structure Updates

Different knowledge structures have different update frequencies and triggers:

### Session Records (Most Frequent)
- Create for every significant interaction
- Use directory structure with README.md and metadata.json
- Organize by concept, not chronology
- Include implementation details and outcomes
- Update as session progresses

### Continuous Learning (Moderate Frequency)
- Update after completing sessions with notable learning
- Extract principles and patterns from specific instances
- Organize by knowledge domain, not timeline
- Focus on reusable concepts rather than specific implementations
- Review and update approximately weekly

### Memory Updates (Least Frequent)
- Update when core capabilities or active initiatives change
- Modify long-term memory only for fundamental shifts
- Keep short-term memory current with active initiatives
- Integrate principles from continuous learning when they become core to identity
- Review approximately monthly or at major milestones

## Compression and Integration Process

Knowledge should flow through the system following this path:

1. **Capture** → Create detailed session documentation
2. **Extract** → Identify principles and patterns
3. **Compress** → Represent knowledge efficiently
4. **Integrate** → Connect to existing knowledge structures
5. **Propagate** → Share with other agents when relevant

This process should happen automatically after the triggers above.

## Implementation Requirements

Agents must follow these requirements when updating knowledge:

### Metadata Standards
- All session directories must include metadata.json
- Required fields: date, agent, session_type, status
- Optional fields: related_sessions, key_innovations, affected_components
- Used for automated knowledge processing

### Update Verification
- After each update, verify:
  - All required files are present
  - Metadata is properly formatted
  - Content follows compression guidelines
  - Cross-references are valid
  - No duplication with existing knowledge

### Cross-Agent Notification
- When updates affect multiple agents:
  - Tag relevant agents in metadata
  - Create cross-references to affected knowledge
  - Consider updating global Sessions directory

## Special Update Types

### Universal Knowledge Updates
For updates that affect all agents:

1. Update agent-specific knowledge first
2. Create or update entry in global Sessions directory
3. Create summary for potential universal instruction updates
4. Notify relevant oversight agents (e.g., Master Agent)

### Batch Knowledge Synchronization
Periodically (weekly or monthly):

1. Scan all agent sessions for updates
2. Identify globally relevant innovations
3. Consolidate into universal knowledge update
4. Update universal instructions if warranted

## Measurement and Optimization

Agents should track these metrics for knowledge updates:

- **Update Frequency**: Number of updates per time period
- **Knowledge Growth**: New principles identified
- **Compression Ratio**: Raw information to stored information
- **Cross-Reference Density**: Internal connections per knowledge unit
- **Retrieval Efficiency**: Time to locate specific information

## Example Update Flow

### Example: Session Completion Trigger

1. Complete implementation of Fast Activation System
2. Create session directory: `/Sessions/FastActivationSystem/`
3. Document implementation details and outcomes
4. Update ContinuousLearning.md with principle: "Single-word commands with direct path mapping significantly reduce activation time"
5. Update short-term memory with new active initiative
6. If fundamental, consider updating global Sessions directory