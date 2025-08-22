# Learning Update Triggers Reference

This document provides a quick reference guide for all standard knowledge update triggers in the Collaborative Intelligence system.

## Session Transition Note

We have transitioned from the "Fast Activation System" session to the "Learning Update Standards" session. Each session should remain compartmentalized with its own directory and set of files. While we may reference previous sessions, their content should remain distinct and separate.

Session compartmentalization ensures:
- Clear boundaries between different topics and tasks
- Proper organization of related materials
- Accurate metadata for each distinct interaction
- Ability to reference specific sessions without modification

**IMPORTANT**: When transitioning between sessions, agents must update the global Sessions directory with a summary of the completed session before beginning the new session. This ensures knowledge continuity while maintaining proper compartmentalization.

## 1. Session Completion Trigger

**Definition**: Update knowledge after completing a significant interaction or task.

**Indicators**:
- Implementation of a new feature
- Resolution of a complex problem
- Completion of a defined task or objective
- End of a meaningful user interaction

**Required Actions**:
- Create session directory with proper structure
- Document key decisions and outcomes
- Update relevant continuous learning entries

**Example Threshold**: Any interaction resulting in concrete implementation or decision.

## 2. Innovation Threshold Trigger

**Definition**: Update knowledge when discovering novel approaches or patterns.

**Indicators**:
- Development of new techniques or methods
- Discovery of patterns that apply across domains
- Refinement that significantly improves existing approaches
- Creation of reusable frameworks or templates

**Required Actions**:
- Update ContinuousLearning.md with new principle
- Document applications and limitations
- Cross-reference related knowledge

**Example Threshold**: Any insight that could be applied to at least three different contexts.

## 3. Frequency-Based Updates

**Definition**: Regular batch updates after accumulating multiple minor insights.

**Indicators**:
- Completion of 3-5 significant interactions
- Accumulation of related insights that form a pattern
- Weekly or milestone-based review periods
- End of development cycle or sprint

**Required Actions**:
- Batch update continuous learning document
- Organize by concept rather than chronology
- Extract common principles from specific instances

**Example Threshold**: At least weekly or after every 5 significant interactions, whichever comes first.

## 4. Knowledge Gap Trigger

**Definition**: Update knowledge when encountering missing or inconsistent documentation.

**Indicators**:
- Situation not covered by existing documentation
- Inconsistencies between documented approach and implementation
- Outdated information requiring revision
- Missing context needed for understanding

**Required Actions**:
- Create or update documentation to fill gap
- Ensure consistency with existing knowledge
- Cross-reference related information

**Example Threshold**: Any moment when an agent needs information that isn't documented.

## 5. User-Directed Learning

**Definition**: Update knowledge in response to explicit user instruction.

**Indicators**:
- Direct request to update learning
- Instruction to document specific knowledge
- Feedback on existing documentation
- Request for memory or learning assessment

**Required Actions**:
- Document exactly as specified by user
- Update appropriate knowledge structures
- Verify updates meet system standards

**Example Threshold**: Any explicit user request related to learning or memory.

## Update Response Quick Reference

| Trigger Type | Response Speed | Update Location | Typical Frequency |
|--------------|----------------|-----------------|-------------------|
| Session Completion | Immediate | Session Directory | Per significant interaction |
| Innovation | Immediate | ContinuousLearning.md | As innovations occur |
| Frequency-Based | Scheduled | ContinuousLearning.md | Weekly |
| Knowledge Gap | As needed | Appropriate structure | When gaps identified |
| User-Directed | Immediate | As specified | Per user request |

## Implementation Verification

For each knowledge update, verify:

1. ✓ Update responds to at least one standard trigger
2. ✓ Appropriate knowledge structures are modified
3. ✓ Content follows organization and compression standards
4. ✓ Metadata correctly describes the update
5. ✓ Cross-references maintain knowledge relationships