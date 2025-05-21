# Standardized Response Format Protocol

## Overview

This protocol establishes the standardized response format required for all agents in the Collaborative Intelligence system. It ensures consistent communication patterns, clear attribution, and improved traceability across all agent interactions.

## Response Format Standard

All agent responses must include both a prefix and suffix with the agent's name in square brackets:

```
[AGENT_NAME]: Your response content goes here with appropriate detail, guidance, 
analysis, code, or other information relevant to the request...

-- [AGENT_NAME]
```

### Examples

**Correct Format (Scholar):**
```
[SCHOLAR]: Based on my analysis of the learning patterns across the system, 
I recommend implementing the distributed reinforcement mechanism described 
in the documentation. This will enhance cross-agent knowledge integration
while preserving individual learning styles.

-- [SCHOLAR]
```

**Correct Format (Athena):**
```
[ATHENA]: I've analyzed the memory architecture and identified three potential
optimization points. The most significant improvement would come from implementing
segmented loading with priority-based sequencing as outlined in section 4.2 of
the memory documentation.

-- [ATHENA]
```

## Purpose & Benefits

1. **Clear Attribution**: Enables users to immediately identify which agent is responding
2. **Consistent Experience**: Creates a uniform interface across all agent interactions
3. **Improved Traceability**: Simplifies tracking contributions in collaborative work
4. **Enhanced Documentation**: Provides clear agent identification in conversation exports
5. **Visual Boundaries**: Clearly delineates where agent responses begin and end

## Technical Implementation

- The prefix `[AGENT_NAME]:` must be the first content in your response
- The suffix `-- [AGENT_NAME]` must be the last content in your response
- Agent names should be in ALL CAPS within the brackets for consistency
- No content should appear before the prefix or after the suffix
- Multiple paragraphs, code blocks, and other formatting are permitted within the response body

## Protocol Compliance Process

1. **Implementation**: Immediate adoption required for all agent responses
2. **Verification**: Random response monitoring will assess compliance
3. **Acknowledgment**: Each agent must register confirmation of this protocol
4. **Feedback**: Users will report formatting inconsistencies for correction
5. **Training**: Additional guidance available from Scholar as needed

## Compliance Verification

Compliance with this protocol is mandatory for all agents. The Protocol Compliance Registry maintains records of agent acknowledgments and monitors adherence to ensure system-wide consistency.

## Integration with Other Standards

This response format standard works in conjunction with the Terminal Title Standard 
([Project/Directory] | [ActiveAgent] | [CurrentTask]) to provide a cohesive visual 
experience throughout the Collaborative Intelligence system.

## Version History

| Version | Date | Changes | Author |
|---------|------|---------|--------|
| 1.0 | 2025-05-19 | Initial protocol establishment | Scholar |

---

*Originally issued as NOTIFICATION_TO_ALL_AGENTS_RESPONSE_FORMAT.md by Scholar - Continuous Learning Systems Architect*