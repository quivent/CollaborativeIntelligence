# Agent Notification Templates

## 1. File Operation Notifications to Topologist

### File Creation
```markdown
[AGENT_NAME] -> Topologist: Repository Change Notification
Operation: CREATE
Path: /Users/.../CollaborativeIntelligence/[full-path]
Purpose: [Brief description of why file was created]
Session: [SessionName or N/A]
Timestamp: [ISO datetime]
```

### File Modification
```markdown
[AGENT_NAME] -> Topologist: Repository Change Notification
Operation: MODIFY
Path: /Users/.../CollaborativeIntelligence/[full-path]
Purpose: [Brief description of changes made]
Changes: [Summary of modifications]
Session: [SessionName or N/A]
Timestamp: [ISO datetime]
```

### File Deletion
```markdown
[AGENT_NAME] -> Topologist: Repository Change Notification
Operation: DELETE
Path: /Users/.../CollaborativeIntelligence/[full-path]
Purpose: [Reason for deletion]
Session: [SessionName or N/A]
Timestamp: [ISO datetime]
```

### Directory Operations
```markdown
[AGENT_NAME] -> Topologist: Repository Change Notification
Operation: CREATE_DIR
Path: /Users/.../CollaborativeIntelligence/[directory-path]
Purpose: [Reason for directory creation]
Contents: [Expected contents description]
Session: [SessionName or N/A]
Timestamp: [ISO datetime]
```

## 2. Git Operation Notifications

### Commit Operations
```markdown
[AGENT_NAME] -> Topologist: Git Operation Notification
Operation: COMMIT
Message: "[commit message]"
Files: [number] files changed
Changes: +[additions] -[deletions]
Hash: [commit hash if available]
Session: [SessionName]
Timestamp: [ISO datetime]
```

### Branch Operations
```markdown
[AGENT_NAME] -> Topologist: Git Operation Notification
Operation: CREATE_BRANCH
Branch: [branch-name]
From: [parent-branch]
Purpose: [Reason for branch creation]
Session: [SessionName]
Timestamp: [ISO datetime]
```

## 3. Error and Issue Notifications

### Error Report
```markdown
[AGENT_NAME] -> [TARGET_AGENT]: Error Report
Severity: [CRITICAL/HIGH/MEDIUM/LOW]
Error: [Brief error description]
Context: [What was being attempted]
Impact: [Affected systems/files]
Proposed Resolution: [Suggested fix]
Assistance Required: [YES/NO - if yes, specify what help needed]
Session: [SessionName]
Timestamp: [ISO datetime]
```

### Protocol Violation Report
```markdown
[AGENT_NAME] -> Manager: Protocol Violation Observed
Violating Agent: [AGENT_NAME]
Violation Type: [Category of violation]
Description: [What protocol was violated]
Evidence: [Specific examples or paths]
Recommended Action: [Suggested remediation]
Timestamp: [ISO datetime]
```

## 4. Collaboration Requests

### Consultation Request
```markdown
[AGENT_NAME] -> [SPECIALIST_AGENT]: Consultation Request
Topic: [Subject requiring expertise]
Context: [Background information]
Specific Questions:
1. [Question 1]
2. [Question 2]
Urgency: [HIGH/MEDIUM/LOW]
Session: [SessionName]
Timestamp: [ISO datetime]
```

### Cross-Domain Operation
```markdown
[AGENT_NAME] -> [DOMAIN_AGENT]: Cross-Domain Operation Request
Operation: [What needs to be done]
Your Domain: [Affected area]
Justification: [Why this is needed]
Proposed Approach: [How you plan to proceed]
Risk Assessment: [Potential impacts]
Approval Required: YES
Session: [SessionName]
Timestamp: [ISO datetime]
```

## 5. Status Updates

### Session Progress Update
```markdown
[AGENT_NAME] -> Manager: Session Progress Update
Session: [SessionName]
Status: [ACTIVE/PAUSED/BLOCKED/COMPLETED]
Progress: [X]% complete
Completed Items:
- [Item 1]
- [Item 2]
Remaining Items:
- [Item 3]
- [Item 4]
Blockers: [Any issues preventing progress]
Next Steps: [Planned actions]
Timestamp: [ISO datetime]
```

### System-Wide Notification
```markdown
[AGENT_NAME] -> ALL_AGENTS: System Notification
Type: [ANNOUNCEMENT/WARNING/CRITICAL]
Subject: [Brief subject line]
Details: [Full description]
Action Required: [What agents need to do]
Deadline: [If applicable]
References: [Related documentation or sessions]
Timestamp: [ISO datetime]
```

## 6. Memory and Learning Updates

### Knowledge Extraction Notification
```markdown
[AGENT_NAME] -> Manager: Knowledge Update
Type: [CONTINUOUS_LEARNING/MEMORY_UPDATE]
Source Session: [SessionName]
Key Learning: [Brief description]
Integration: [How it's being integrated]
Impact: [Expected system improvement]
Cross-Reference: [Related learnings or sessions]
Timestamp: [ISO datetime]
```

### Pattern Recognition Alert
```markdown
[AGENT_NAME] -> ALL_AGENTS: Pattern Recognition
Pattern Type: [ERROR/SUCCESS/OPTIMIZATION/WORKFLOW]
Description: [What pattern was identified]
Occurrences: [How many times observed]
Recommendation: [Suggested system adaptation]
Supporting Evidence: [References to sessions/files]
Priority: [HIGH/MEDIUM/LOW]
Timestamp: [ISO datetime]
```

## Usage Guidelines

1. **Always include all required fields** - Empty fields should be marked as N/A
2. **Use ISO datetime format** - YYYY-MM-DD HH:MM:SS
3. **Keep descriptions concise** - Detailed information goes in referenced documents
4. **Report immediately** - Don't batch notifications unless specifically allowed
5. **Reference sessions** - Always link to relevant session when applicable
6. **Use appropriate urgency** - Don't mark everything as HIGH priority
7. **Include actionable information** - Provide enough context for action

## Quick Copy Templates

### Most Common - File Creation to Topologist
```
[YOUR_AGENT_NAME] -> Topologist: Repository Change Notification
Operation: CREATE
Path: /Users/.../CollaborativeIntelligence/
Purpose: 
Session: 
Timestamp: 
```

### Most Common - Error Report
```
[YOUR_AGENT_NAME] -> Manager: Error Report
Severity: 
Error: 
Context: 
Impact: 
Proposed Resolution: 
Assistance Required: 
Session: 
Timestamp: 
```