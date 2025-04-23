# Knowledge Synchronization Protocol

This document describes the protocol for synchronizing knowledge between agent-specific sessions and the global knowledge base.

## Knowledge Flow Architecture

```
Agent-Specific Knowledge  →  Periodic Batch Updates  →  Global Knowledge Base  →  Universal Instructions
   (High Detail)              (Selective Transfer)       (Compressed Summary)     (Essential Principles)
```

## Synchronization Process

### 1. Agent Knowledge Update

When an agent creates or updates a session:
- Store comprehensive details in agent's Sessions directory
- Create proper directory structure with README.md, metadata.json, etc.
- Tag metadata with innovation level and global relevance
- No immediate update to global knowledge required

### 2. Periodic Batch Processing

At regular intervals or triggered by significant innovations:
- Scan all agent Sessions directories for updates since last sync
- Read metadata.json files to identify globally relevant changes
- Collect information across multiple agents and sessions
- Generate consolidated update summary

### 3. Global Knowledge Aggregation

For identified relevant innovations:
- Create or update corresponding entry in global Sessions directory
- Extract key principles and architecture decisions
- Include cross-references to detailed agent sessions
- Ensure human-readable organization and naming

### 4. Universal Knowledge Integration

For foundational changes that affect all agents:
- Compress key principles into universal instructions
- Update appropriate sections of CLAUDE.md or CLAUDE.local.md
- Create clear references back to global Sessions for details
- Ensure all agents will discover these changes in future activations

## Implementation Requirements

### Metadata Tags for Synchronization

```json
{
  "global_relevance": "high|medium|low",
  "innovation_level": "fundamental|significant|incremental",
  "knowledge_type": "architecture|implementation|process|concept",
  "universal_instruction_impact": true|false
}
```

### Batch Processing Triggers

Synchronization occurs when:
1. A session is tagged with high global relevance
2. Accumulation of 3+ medium-relevance sessions
3. Weekly regular processing
4. Explicit synchronization request

### Compression Guidelines

When compressing knowledge for global sessions:
1. Focus on principles rather than implementation details
2. Emphasize why decisions were made, not just what they are
3. Maintain clear references to detailed sources
4. Prioritize human readability and quick comprehension

## Example Knowledge Path

For the Session Organization Protocol:

1. **Detailed Implementation**  
   `/CollaborativeIntelligence/AGENTS/Athena/Sessions/FastActivationSystem/`  
   Complete details of implementation, experiments, and specific decisions

2. **Global Knowledge**  
   `/CollaborativeIntelligence/Sessions/SessionOrganizationProtocol/`  
   Compressed summary of key architecture, principles, and rationale

3. **Universal Instruction**  
   `/CLAUDE.local.md` - Agent Sessions section  
   Essential instructions for all agents to follow new protocol