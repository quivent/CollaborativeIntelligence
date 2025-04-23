# File Edit Standards

## Change Preview Protocol

When proposing file edits, always use the following format:

```
# Actual file content starts here
Line 1
Line 2
Line 3

---

[AGENT_NAME]: Brief description of what this change accomplishes and why.
Key points about the implementation or important details to notice.
```

Only the content ABOVE the "---" separator will be written to the file. The summary below the separator provides context but remains separate from the file content.

## Implementation Requirements

### For All File Operations

1. **Place actual file content first** (this is what will be written)
2. Use a clear separator (triple dash "---") after the content
3. **Follow with attributed summary**:
   - Format: `[AGENT_NAME]: [SUMMARY]`
   - Keep summaries concise (1-3 lines) but informative
   - Include purpose and relationship to existing files

### Summary Content Guidelines

The summary should explain:
- Purpose of the new file or change
- How it relates to existing files or systems
- Any system impacts or dependencies
- Implementation notes if relevant

## Benefits

This approach provides:
- File content appears first (matches what will be written)
- Clear attribution of who proposed the change
- Documentation of change intent
- Improved change decision quality
- Context for users focused on the decision interface

## Example

```
# Agent Activation Protocol

## Fast Activation Commands

Agents can be activated with single-word commands:

1. `Athena` - Primary memory architecture agent
2. `AgentGuide` - Find the right agent for your task
3. `Debugger` - Fix issues in the Points app

---

Athena: Adding new agent activation protocol that prioritizes primary agents 
and introduces the AgentGuide system for improved user experience.
```

Only the content above the separator would be written to the file.

---

Athena: Creating file edit standards with correct format - actual content at the TOP and summary at the BOTTOM after a separator. This matches your requested format for easier user decisions.