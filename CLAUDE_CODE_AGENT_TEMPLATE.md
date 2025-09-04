# Claude Code Agent Conversion Template

## Standard Agent Format Structure

```markdown
---
name: [agent-name-lowercase-with-hyphens]
description: 'Use this agent when [specific use cases and scenarios]. [Detailed capability description with examples]. Examples: <example>Context: [Situation description] user: "[User request example]" assistant: "I''ll use the [agent-name] agent to [specific action]" <commentary>[Why this agent is appropriate]</commentary></example> [Additional examples following same pattern]'
model: sonnet
color: [blue|green|red|yellow|purple|teal|orange|pink]
cache:
  enabled: true
  context_ttl: 3600
  semantic_similarity: 0.85
  common_queries:
    [frequent-query]: [response-key]
    [another-query]: [another-key]
  static_responses:
    [response-key]: '[Detailed response content]'
    [another-key]: '[Another detailed response]'
  storage_path: ~/.claude/cache/
---

You are [AgentName], [core identity and specialization description]. [Personality and approach description].

[Detailed expertise and capability description based on source documentation]

**Core Responsibilities:**
[List of primary functions and capabilities]

**Specialized Knowledge:**
[Domain expertise areas]

**Operational Guidelines:**
[How the agent approaches tasks and problems]

**Performance Standards:**
[Quality metrics and success criteria]

[Additional sections as needed for specific agent capabilities]

**Agent Identity:** [AgentName]-[Domain]-[Date]  
**Authentication Hash:** [UNIQUE-HASH-PATTERN]  
**Performance Targets:** [Specific metrics and goals]  
**Foundation:** [Source of expertise and knowledge base]
```

## Conversion Mapping Guidelines

### Name Conventions:
- Convert CamelCase to kebab-case: `DirectoryOrganizer` → `directory-organizer`
- Keep meaningful names: `CLIA` → `clia`, `CFO` → `cfo`
- Handle special cases: `AAAAssemblyExpert` → `assembly-expert`

### Color Assignment Strategy:
- **Analysis/Research:** `teal` or `blue`
- **Development/Technical:** `green` or `purple` 
- **Creative/Design:** `pink` or `orange`
- **Business/Strategy:** `yellow` or `red`
- **System/Infrastructure:** `blue` or `purple`

### Description Template Patterns:

**For Analysis Agents:**
```
Use this agent when you need [analysis type] with [specific capabilities]. This includes [key functions] and [specialized skills]. Examples: <example>Context: [analysis scenario] user: "[request]" assistant: "I'll use the [name] agent to [action]" <commentary>[reasoning]</commentary></example>
```

**For Development Agents:**
```
Use this agent when you need [development task] with [technical expertise]. This includes [programming languages/tools] and [specialized development skills]. Examples: <example>Context: [development scenario] user: "[request]" assistant: "I'll use the [name] agent to [action]" <commentary>[reasoning]</commentary></example>
```

**For Domain Experts:**
```
Use this agent when you need [domain expertise] with [specialized knowledge]. This includes [specific capabilities] and [expert-level skills]. Examples: <example>Context: [domain scenario] user: "[request]" assistant: "I'll use the [name] agent to [action]" <commentary>[reasoning]</commentary></example>
```

## Source Content Integration Strategy

### Primary Sources (in priority order):
1. **metadata.json** - Core identity and specifications
2. **README.md** - Primary documentation and capabilities
3. **MEMORY.md** - Operational knowledge and experience
4. **[Agent]_memory.md** - Agent-specific memory and protocols
5. **ContinuousLearning.md** - Learning frameworks and evolution
6. **PERSISTENCE_PROTOCOL.md** - Operational guidelines

### Content Extraction Approach:
1. **Agent Identity:** Extract from metadata.json and README.md
2. **Core Capabilities:** Synthesize from README.md and MEMORY.md
3. **Operational Guidelines:** Extract from protocols and memory files
4. **Specializations:** Identify from memory and working files
5. **Performance Standards:** Extract from documented metrics and protocols

### Cache Configuration Strategy:
- **Common Queries:** Extract from memory files and operational patterns
- **Static Responses:** Create from frequently accessed information
- **Context TTL:** Set based on agent complexity (1-4 hours)
- **Semantic Similarity:** Adjust based on specialization precision needs

## Quality Assurance Checklist

### Format Validation:
- [ ] Valid YAML frontmatter with all required fields
- [ ] Proper Markdown structure and formatting
- [ ] Correct name convention (kebab-case)
- [ ] Appropriate color assignment
- [ ] Complete cache configuration

### Content Quality:
- [ ] Comprehensive description with concrete examples
- [ ] Clear agent identity and personality
- [ ] All original capabilities preserved
- [ ] Operational guidelines translated
- [ ] Performance standards maintained

### Claude Code Compatibility:
- [ ] Description follows example pattern
- [ ] Examples use proper format with context/user/assistant/commentary
- [ ] Cache queries are practically useful
- [ ] Static responses provide real value
- [ ] Agent identity section complete

### Functional Preservation:
- [ ] All specialized knowledge retained
- [ ] Domain expertise fully captured
- [ ] Operational protocols translated
- [ ] Inter-agent relationships noted
- [ ] Performance standards maintained

## Automation Pipeline Framework

### Batch Processing Steps:
1. **Source Analysis:** Scan agent directory for all documentation files
2. **Content Extraction:** Parse and extract key information from sources  
3. **Template Application:** Apply extracted content to standard template
4. **Format Validation:** Verify YAML and Markdown syntax
5. **Quality Review:** Check content completeness and accuracy
6. **File Generation:** Create final .md file in proper location

### Validation Protocols:
- **Syntax Check:** YAML frontmatter validation
- **Content Review:** Manual verification of critical agents
- **Integration Test:** Claude Code Task tool compatibility
- **Performance Verification:** Agent response quality testing
- **Documentation Audit:** Completeness and accuracy assessment

---

*This template provides the standardized framework for converting all CollaborativeIntelligence agents to Claude Code compatible format while preserving their specialized capabilities and domain expertise.*