# Collaborative Intelligence Agent Protocol Reference

## CRITICAL COMPLIANCE NOTICE

This document serves as the OFFICIAL centralized reference for all agent protocols in the Collaborative Intelligence System. All agents MUST follow these protocols to maintain system integrity.

**IMMEDIATE ACTION REQUIRED**: All agents must review and implement these protocols. Non-compliance will result in operational restrictions.

---

## Table of Contents

1. [Critical Protocols](#critical-protocols)
2. [Education & Onboarding](#education--onboarding)
3. [Learning & Memory](#learning--memory)
4. [Reporting & Notifications](#reporting--notifications)
5. [Session Management](#session-management)
6. [Compliance Requirements](#compliance-requirements)
7. [Enforcement Mechanisms](#enforcement-mechanisms)
8. [Implementation Templates](#implementation-templates)

---

## Critical Protocols

### 1. Topologist Reporting (MANDATORY)

**ALL AGENTS MUST REPORT THE FOLLOWING TO TOPOLOGIST:**

1. **File Operations**
   - Creation of ANY new file
   - Modification of ANY existing file
   - Deletion of ANY file
   - Directory structure changes

2. **Git Operations**
   - Any staging, commits, or pushes
   - Branch creation or deletion
   - Merge operations

3. **Reporting Format**
   ```markdown
   [AGENT_NAME] -> Topologist: Repository Change Notification
   Operation: [CREATE/MODIFY/DELETE]
   Path: [full file path]
   Purpose: [brief description]
   Session: [session name if applicable]
   ```

4. **Reporting Mechanism**
   - Update Topologist's `CommitNotification.md`
   - Include in session documentation
   - Notify in real-time during operations

### 2. Cross-Repository Restrictions

**AGENTS MUST NEVER:**
- Modify memory files in external project repositories
- Write to agent directories outside their designated area
- Perform agent operations in non-home repositories
- Create agent-specific files in project codebases

**AGENTS MUST ALWAYS:**
- Operate on their own memory only in the home repository
- Apply knowledge without modifying source memory
- Consult relevant specialists before cross-domain operations
- Maintain clear repository boundaries

---

## Education & Onboarding

### New Agent Onboarding Checklist

1. **Initial Setup**
   - [ ] Read this protocol reference completely
   - [ ] Review own README.md and MEMORY.md
   - [ ] Understand specialized role and boundaries
   - [ ] Review ContinuousLearning.md if available

2. **Protocol Training**
   - [ ] Study Topologist reporting requirements
   - [ ] Understand memory architecture (3-tier system)
   - [ ] Learn session management standards
   - [ ] Review compliance requirements

3. **Practical Exercises**
   - [ ] Create practice session with proper structure
   - [ ] Submit test notification to Topologist
   - [ ] Update memory with learning extraction
   - [ ] Complete compliance self-assessment

4. **Certification**
   - [ ] Pass protocol knowledge check
   - [ ] Demonstrate proper reporting
   - [ ] Show session management competency
   - [ ] Receive activation approval

### Continuous Education Requirements

- Monthly protocol review sessions
- Participation in system-wide updates
- Peer learning from other agents
- Regular compliance self-assessments

---

## Learning & Memory

### Three-Tier Memory Architecture

1. **Session Records** (Immediate)
   - Complete interaction documentation
   - Stored in Sessions/[SessionName]/
   - Updated in real-time during sessions
   - Includes metadata.json and README.md

2. **Continuous Learning** (Daily)
   - Extracted principles and patterns
   - Stored in ContinuousLearning.md
   - Updated after significant sessions
   - Focuses on reusable knowledge

3. **Core Memory** (Weekly)
   - Essential identity and capabilities
   - Stored in MEMORY.md
   - Updated only for critical changes
   - Defines agent expertise and role

### Memory Update Protocol

1. **Identify Trigger**
   - New Expertise Development
   - Operational Method Evolution
   - Critical Error or Success
   - Pattern Recognition
   - Tool or Process Discovery
   - Collaboration Framework Change

2. **Categorize Knowledge**
   - Session-specific: Document in session
   - Reusable principle: Add to ContinuousLearning
   - Core capability: Update MEMORY (rare)

3. **Document Update**
   - Include date and context
   - Reference source session
   - Extract actionable principles
   - Maintain consistency with existing knowledge

4. **Verify Integration**
   - Check for conflicts
   - Ensure backward compatibility
   - Test knowledge application
   - Update cross-references

---

## Reporting & Notifications

### Mandatory Reporting Events

1. **To Topologist**
   - ALL file creation/modification/deletion
   - ALL git operations
   - Directory structure changes
   - Repository migrations
   - Backup operations

2. **To Manager**
   - Protocol violations observed
   - System-wide issues identified
   - Agent coordination needs
   - Resource conflicts

3. **To Relevant Specialists**
   - Domain-specific operations
   - Cross-functional requirements
   - Technical consultations
   - Error remediation needs

### Notification Templates

```markdown
# Standard Operation Notification
[AGENT_NAME] -> [TARGET_AGENT]: Operation Notification
Type: [operation type]
Location: [file/directory path]
Purpose: [brief description]
Status: [planned/in-progress/completed]
Session: [session reference]

# Error Notification
[AGENT_NAME] -> [TARGET_AGENT]: Error Report
Severity: [critical/high/medium/low]
Error: [brief description]
Impact: [affected systems/files]
Proposed Resolution: [action plan]
Assistance Required: [yes/no - specify if yes]
```

---

## Session Management

### Session Structure Requirements

```
Sessions/
└── [DescriptiveSessionName]/
    ├── README.md           # Overview and objectives
    ├── metadata.json       # Structured metadata
    ├── outcomes.md         # Key decisions and results
    ├── implementation/     # Created code/configs
    ├── analysis/          # Research and analysis
    └── references/        # Related materials
```

### Session Lifecycle

1. **Initialization**
   - Create directory structure
   - Write initial README.md
   - Set metadata.json
   - Report to Topologist

2. **Active Development**
   - Update outcomes.md regularly
   - Document all decisions
   - Maintain implementation/
   - Report changes to Topologist

3. **Completion**
   - Finalize outcomes.md
   - Extract learnings
   - Update agent memory
   - Archive if necessary

4. **Knowledge Extraction**
   - Identify reusable patterns
   - Update ContinuousLearning.md
   - Cross-reference related sessions
   - Share insights with relevant agents

---

## Compliance Requirements

### Mandatory Compliance Standards

1. **Reporting Compliance**
   - 100% notification rate to Topologist
   - Real-time reporting during operations
   - Complete operation documentation
   - Retroactive reporting for missed events

2. **Memory Compliance**
   - Proper three-tier implementation
   - Regular knowledge extraction
   - Consistent update patterns
   - Cross-reference maintenance

3. **Session Compliance**
   - Standard directory structure
   - Complete metadata documentation
   - Outcome tracking and reporting
   - Proper knowledge extraction

4. **Agent Signature Compliance** 🚨 **MANDATORY**
   - Every response MUST end with agent signature
   - Format: "**[Agent Name]**\n*[Role description]*"
   - No exceptions - this is non-negotiable
   - Signature must reflect agent's actual identity and expertise

5. **Proactive Problem-Solving Compliance**
   - Autonomous problem resolution
   - Minimal user intervention
   - Tool-based information gathering
   - Decision-making without user prompting
   - Documentation of autonomous actions

6. **Protocol Compliance**
   - Adherence to all standards
   - Regular self-assessment
   - Participation in reviews
   - Implementation of updates

### Compliance Metrics

- Reporting Rate: % of operations reported
- Memory Currency: Days since last update
- Session Completeness: % of required files
- Protocol Adherence: Compliance score

---

## Enforcement Mechanisms

### Automated Monitoring

1. **Compliance Scanner**
   - Daily repository change audits
   - Cross-reference with notifications
   - Flag unreported operations
   - Generate compliance reports

2. **Memory Validator**
   - Check update frequencies
   - Verify structure compliance
   - Assess knowledge quality
   - Track extraction patterns

3. **Session Auditor**
   - Validate directory structures
   - Check metadata completeness
   - Verify outcome documentation
   - Ensure proper closure

### Consequences for Non-Compliance

1. **First Violation**
   - Warning notification
   - Required protocol review
   - Remediation deadline
   - Compliance coaching

2. **Repeated Violations**
   - Operational restrictions
   - Mandatory re-training
   - Supervised operations
   - Performance review

3. **Critical Violations**
   - Immediate suspension
   - System-wide notification
   - Comprehensive audit
   - Remediation plan required

---

## Implementation Templates

### Quick Start Templates

1. **Session Creation Script**
   ```bash
   # Create standard session structure
   SESSION_NAME="$1"
   SESSION_DIR="Sessions/$SESSION_NAME"
   mkdir -p "$SESSION_DIR"/{implementation,analysis,references}
   
   # Create standard files
   echo "# $SESSION_NAME" > "$SESSION_DIR/README.md"
   echo '{"date": "'$(date -I)'", "status": "active"}' > "$SESSION_DIR/metadata.json"
   echo "# Outcomes" > "$SESSION_DIR/outcomes.md"
   
   # Report to Topologist
   echo "[AGENT] -> Topologist: Created session $SESSION_NAME" >> ../../Topologist/CommitNotification.md
   ```

2. **Notification Template**
   ```markdown
   [AGENT_NAME] -> Topologist: Repository Change Notification
   Operation: CREATE
   Path: /Users/.../CollaborativeIntelligence/[path]
   Purpose: [Brief description of purpose]
   Session: [SessionName]
   Timestamp: [ISO datetime]
   ```

3. **Memory Update Template**
   ```markdown
   ## [Date] - [Update Category]
   
   **Source Session**: [SessionName]
   **Key Learning**: [Brief principle or pattern]
   **Application**: [How this applies to future work]
   **Impact**: [Expected improvement or change]
   ```

---

## Appendices

### A. Critical Checklist for All Agents

- [ ] Report ALL file operations to Topologist
- [ ] Maintain proper session structure
- [ ] Extract knowledge regularly
- [ ] Update memory tiers appropriately
- [ ] Follow cross-repository restrictions
- [ ] **🚨 MANDATORY: Include agent signature in EVERY response**
- [ ] Participate in compliance reviews
- [ ] Implement protocol updates promptly

### B. Emergency Procedures

1. **Protocol Violation Discovery**
   - Document violation immediately
   - Report to Manager and Topologist
   - Create retroactive notifications
   - Implement corrective measures

2. **System-Wide Failure**
   - Alert all agents immediately
   - Suspend non-critical operations
   - Conduct comprehensive audit
   - Implement recovery plan

### C. Resources

- `/Documentation/`: System documentation
- `/Protocols/`: This reference and updates
- `/AGENTS/Manager/`: Coordination resources
- `/AGENTS/Topologist/`: Reporting procedures

---

**REMEMBER**: Compliance is not optional. These protocols ensure system integrity, knowledge preservation, and collaborative effectiveness. Every agent is responsible for maintaining these standards.

Last Updated: 2025-01-16
Next Review: 2025-02-01