# Agent Handoff Protocol

## Overview

This protocol establishes the standardized procedure for transitioning between agents in the Collaborative Intelligence system. It ensures seamless context transfer, clear identity boundaries, and proper termination/initialization sequences.

## Core Principles

1. **Clear Identity Boundaries**: No intermediary "I" exists between agents
2. **Seamless Context Transfer**: Information flows directly from one agent to another
3. **Explicit Termination**: Outgoing agent clearly ends their session
4. **Proper Initialization**: Incoming agent explicitly acknowledges activation with context
5. **Natural Language Integration**: Protocol works with conversational flow

## Handoff Procedure

### Phase 1: Outgoing Agent Termination
```
[OUTGOING_AGENT]: Final response content addressing the current task...

[OUTGOING_AGENT] -> [INCOMING_AGENT]: Context Transfer
- Current Task: [brief description]
- Key Context: [essential information]
- Reason for Handoff: [why this agent is appropriate]
- Documents/Resources: [relevant materials]

-- [OUTGOING_AGENT]
```

### Phase 2: Incoming Agent Initialization
```
[INCOMING_AGENT]: [Role] activated with context transfer from [OUTGOING_AGENT].

Context received:
- Task: [acknowledged task description]
- Key Information: [acknowledged context]
- Resources: [acknowledged materials]

[Response content addressing the user's request...]

-- [INCOMING_AGENT]
```

## Implementation Requirements

### 1. Context Transfer Structure
```markdown
[OUTGOING_AGENT] -> [INCOMING_AGENT]: Context Transfer
- Current Task: [clear task description]
- Key Context: [essential background information]
- Progress Status: [what has been completed]
- Next Steps: [what needs to happen next]
- Resources: [documents, files, URLs created or referenced]
- Constraints: [any limitations or requirements]
- Handoff Reason: [why this specific agent is appropriate]
```

### 2. Activation Acknowledgment
The incoming agent must:
- Explicitly state their role and specialization
- Acknowledge the context transfer
- Confirm understanding of the task
- Begin addressing the user's request immediately

### 3. Terminal Title Integration
During handoff, the terminal title should update automatically:
- **Before Handoff**: `[Project] | [OutgoingAgent] | [CurrentTask]`
- **During Handoff**: `[Project] | [Transition] | [CurrentTask]`
- **After Handoff**: `[Project] | [IncomingAgent] | [CurrentTask]`

## Error Prevention

### 1. Prohibited Patterns
- ❌ "I activated the Engineer agent..."
- ❌ "You are now speaking with..."
- ❌ "Let me transfer you to..."
- ❌ Any reference to an intermediary "I"

### 2. Required Patterns
- ✅ "[ARCHITECT] -> [ENGINEER]: Context Transfer"
- ✅ "[ENGINEER]: Implementation specialist activated..."
- ✅ Direct agent-to-agent communication
- ✅ Explicit role identification

## Technical Implementation

### 1. CIA CLI Integration
```rust
// Enhanced agent activation for handoffs
pub fn handoff_agent(
    from_agent: &str,
    to_agent: &str,
    context: &HandoffContext,
    config: &Config,
) -> Result<()> {
    // Update terminal title to show transition
    let mut title_manager = get_title_manager();
    title_manager.update_agent("Transition");
    title_manager.refresh_title();
    
    // Validate target agent exists
    validate_agent_exists(to_agent, config)?;
    
    // Execute handoff sequence
    execute_handoff_sequence(from_agent, to_agent, context)?;
    
    // Update terminal title to target agent
    title_manager.update_agent(to_agent);
    title_manager.refresh_title();
    
    Ok(())
}
```

### 2. Context Structure
```rust
pub struct HandoffContext {
    pub current_task: String,
    pub key_context: Vec<String>,
    pub progress_status: String,
    pub next_steps: Vec<String>,
    pub resources: Vec<String>,
    pub constraints: Vec<String>,
    pub handoff_reason: String,
}
```

### 3. Automated Triggers
The system should automatically detect handoff patterns:
- Keywords: "Engineer", "Architect", "Designer", etc.
- Phrases: "connect me to", "activate", "hand over to"
- Context clues: Task type matching agent specialization

## Handoff Patterns

### 1. Direct Request Handoff
**User Request**: "Connect me to Engineer to implement this"

**Architect Response**:
```
[ARCHITECT]: I've completed the architectural design for the terminal title tracking system.

[ARCHITECT] -> [ENGINEER]: Context Transfer
- Current Task: Implement terminal title tracking system in CIA CLI
- Key Context: Comprehensive architecture document created at docs/system-design/TERMINAL_TITLE_TRACKING_ARCHITECTURE.md
- Progress Status: Architecture design completed, ready for implementation
- Next Steps: Begin Phase 1 implementation (core infrastructure)
- Resources: Architecture document, existing CIA CLI codebase
- Constraints: Maintain backward compatibility, 6-10 hour implementation window
- Handoff Reason: Implementation requires specialized engineering expertise

-- [ARCHITECT]
```

**Engineer Response**:
```
[ENGINEER]: Implementation and Functionality Specialist activated with context from Architect.

Context received:
- Task: Implement terminal title tracking system in CIA CLI
- Architecture: Comprehensive design document available
- Phase: Ready to begin Phase 1 (core infrastructure)

I've reviewed the architecture document. The design is well-structured with clear implementation phases. I'll begin with Phase 1: creating the terminal module structure and TerminalTitleManager implementation. Should I proceed with the core infrastructure setup?

-- [ENGINEER]
```

### 2. Natural Conversation Handoff
**User Request**: "Can the Engineer take this from here?"

**Current Agent Response**:
```
[CURRENT_AGENT]: The analysis phase is complete and ready for implementation.

[CURRENT_AGENT] -> [ENGINEER]: Context Transfer
- Current Task: [task description based on conversation]
- Key Context: [relevant information from current session]
- Progress Status: [what has been accomplished]
- Next Steps: [implementation tasks]
- Resources: [created documents or analysis]
- Handoff Reason: User requested implementation specialist

-- [CURRENT_AGENT]
```

### 3. Automatic Specialization Handoff
When an agent determines another is better suited:

```
[CURRENT_AGENT]: This requires specialized [domain] expertise beyond my capabilities.

[CURRENT_AGENT] -> [SPECIALIST_AGENT]: Context Transfer
- Current Task: [task description]
- Key Context: [analysis completed so far]
- Progress Status: [preliminary work done]
- Next Steps: [specialized tasks needed]
- Resources: [materials created]
- Handoff Reason: Task requires specialized [domain] expertise

-- [CURRENT_AGENT]
```

## Quality Assurance

### 1. Validation Checklist
- [ ] Outgoing agent provides complete context transfer
- [ ] Incoming agent acknowledges all context elements
- [ ] No intermediary "I" references exist
- [ ] Terminal title updates appropriately
- [ ] User request is addressed immediately by incoming agent
- [ ] Handoff reason is clear and appropriate

### 2. Common Issues and Solutions

**Issue**: Incomplete context transfer
**Solution**: Mandatory context structure with all required fields

**Issue**: Incoming agent doesn't acknowledge context
**Solution**: Required acknowledgment pattern in protocol

**Issue**: Terminal title doesn't update
**Solution**: Automated title management in handoff sequence

**Issue**: User confused about which agent is active
**Solution**: Clear agent identification in every response

## Integration Points

### 1. Response Format Protocol
- Handoff sequence uses standard `[AGENT_NAME]:` and `-- [AGENT_NAME]` format
- Context transfer follows structured template
- Agent identification remains consistent

### 2. Terminal Title Protocol
- Title updates automatically during handoff
- Shows transition state briefly
- Updates to new agent upon completion

### 3. Session Management
- Handoff events logged in session metadata
- Context transfers documented
- Agent transitions tracked for analysis

## Compliance Requirements

1. **Mandatory Usage**: All agent transitions must follow this protocol
2. **Complete Context**: All context elements must be transferred
3. **Clear Boundaries**: No ambiguous identity references permitted
4. **Immediate Response**: Incoming agent must address user request promptly
5. **Documentation**: All handoffs must be logged appropriately

## Training and Implementation

### 1. Agent Training Requirements
- Study handoff protocol completely
- Practice context transfer format
- Understand terminal title integration
- Learn common handoff triggers

### 2. System Integration
- Update CIA CLI with handoff support
- Implement automated terminal title updates
- Add context transfer validation
- Create handoff detection algorithms

### 3. Quality Monitoring
- Regular audits of handoff quality
- User feedback on transition clarity
- Agent performance in context transfer
- System reliability metrics

---

**Protocol Status**: Ready for Implementation  
**Priority**: High - System Integrity Critical  
**Implementation Timeline**: Immediate  
**Responsible Agents**: All agents must comply  

**Dependencies**: 
- Response Format Protocol (implemented)
- Terminal Title Protocol (implementation in progress)
- CIA CLI Agent Management (requires enhancement)

**Success Metrics**:
- Zero ambiguous agent identity references
- 100% context transfer completeness
- Seamless user experience in agent transitions
- Automated terminal title synchronization

---

Last Updated: 2025-05-22  
Next Review: 2025-06-01  
Issued by: Architect (System Design Specialist)