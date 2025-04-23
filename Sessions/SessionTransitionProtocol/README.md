# Session Transition Protocol

## Overview

This global session defines the standardized protocol for transitioning between different sessions within the Collaborative Intelligence system, ensuring proper knowledge continuity while maintaining session compartmentalization.

## Session Transition Workflow

When an agent needs to transition from one session to another, they must follow this sequence:

1. **Complete Current Session**
   - Finish current tasks or reach a logical stopping point
   - Document outcomes in the session directory
   - Update session README.md with final status

2. **Summarize for Global Knowledge**
   - Create or update entry in global Sessions directory
   - Summarize key accomplishments and outcomes
   - Include metadata for automated processing
   - Document any universal principles discovered

3. **Update Agent Learning**
   - Update agent's ContinuousLearning.md with principles
   - Reflect any active initiative changes in MEMORY.md
   - Document transition in short-term memory

4. **Initialize New Session**
   - Create new session directory with descriptive name
   - Set up proper structure with README.md and metadata.json
   - Reference previous session if relevant, without modification
   - Begin work in the new session context

## Compartmentalization Principles

Each session must remain distinct and separate:
- No retroactive changes to previous session content
- Cross-references allowed, but no content merging
- Each session has its own complete metadata
- Session boundaries clearly defined by directory structure

## Implementation Requirements

### Transition Documentation

All session transitions require:
1. Explicit note in new session README.md acknowledging transition
2. Metadata documentation of related previous session
3. Global Sessions directory update
4. Timestamp of transition in all relevant files

### Knowledge Preservation

To ensure no knowledge is lost during transitions:
1. Extract all relevant principles before closing session
2. Document open questions and future work
3. Create cross-references to maintain relationships
4. Ensure global knowledge reflects completed work

## Universal Application

This protocol applies to all agents and all session transitions without exception. It ensures:
- Clean knowledge organization
- Proper attribution of work
- Reliable knowledge continuity
- Efficient information retrieval
- Maintainable session history