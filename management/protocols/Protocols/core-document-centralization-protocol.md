# Core Document Centralization Protocol

## Purpose

This protocol establishes guidelines for identifying, creating, and maintaining core documents that should be centralized and accessible to all agents in the Collaborative Intelligence ecosystem.

## Core Document Identification Criteria

A document is considered a "core document" if it meets one or more of the following criteria:

1. **System-Wide Applicability**: The document contains information or protocols relevant to all or most agents
2. **Foundational Knowledge**: The document defines fundamental concepts, architectures, or frameworks
3. **Cross-Agent Protocols**: The document establishes procedures for interaction between multiple agents
4. **Learning Systems**: The document defines or enhances learning mechanisms for multiple agents
5. **Reference Material**: The document serves as a key reference that will be consulted frequently

## Centralized Storage Locations

Core documents should be stored in one of the following centralized locations based on their primary purpose:

| Document Type | Location | Example |
|---------------|----------|---------|
| System Philosophy | `/docs/core-concepts/` | `system-philosophy.md` |
| Learning Frameworks | `/docs/core-concepts/` | `unified-learning-framework.md` |
| Architecture Documentation | `/docs/system-design/` | `repository-structure.md` |
| Protocol Specifications | `/docs/protocols/` | `learning-update-protocol.md` |
| Agent Guidelines | `/docs/agent-system/` | `agent-onboarding-guide.md` |
| User Documentation | `/docs/guides/` | `CLI.md` |

## Document Creation Process

When creating a document that meets core document criteria:

1. **Identification**: Determine if the document meets core document criteria
2. **Location Selection**: Choose the appropriate centralized location
3. **Standardized Format**: Use established document templates
4. **Authorship Attribution**: Include clear attribution to originating agent(s)
5. **Index Integration**: Update appropriate index documents
6. **Cross-Reference**: Add references in related documents
7. **Notification**: Inform relevant agents of the new document

## Authorship Attribution

To maintain both centralization and proper attribution:

1. **Document Footer**: Include an attribution line at the bottom of the document:
   ```
   *Originally created by [AgentName] - [Agent Description]*
   ```

2. **Contribution Tracking**: For multi-agent documents, include a contributions section:
   ```
   ## Contributors
   - [AgentName1]: [Contribution description]
   - [AgentName2]: [Contribution description]
   ```

3. **Source Referencing**: When appropriate, include a reference to the original source:
   ```
   *Expanded from concepts originally documented in [OriginalLocation]*
   ```

## Document Maintenance Responsibilities

### All Agents

1. Identify when a document meets core document criteria
2. Create core documents in the appropriate centralized location
3. Update centralized documents when providing significant enhancements
4. Include proper attribution for original and contributing agents

### Specialist Agents

1. **Cartographer**: Responsible for maintaining repository structure documentation
2. **Scholar**: Oversees learning and knowledge documentation
3. **Topologist**: Maintains repository operation protocols
4. **Athena**: Manages memory architecture documentation

## Implementation Guidelines

When implementing this protocol:

1. **Document Relocation**: If a document was initially created in an agent-specific location but meets core document criteria:
   - Create or move the document to the appropriate centralized location
   - Add proper attribution to the original author
   - Consider adding a reference document in the original location pointing to the centralized version

2. **Session Documentation**: Session-specific documents should remain in agent session directories, with relevant findings extracted to core documents as appropriate

3. **Document Updating**: When updating a core document:
   - Maintain the original attribution
   - Add your agent name to the contributors section if making significant changes
   - Respect the original document structure and format

4. **New Document Assessment**: Before creating a new document, assess whether it meets core document criteria and should be placed in a centralized location

## Protocol Transition Period

During the initial implementation of this protocol:

1. Review existing agent-specific documents to identify those meeting core document criteria
2. Prioritize relocation based on system-wide importance
3. Update indices and references to reflect new document locations
4. Educate all agents on the new protocol through the standard distribution system

## Document Index Integration

All centralized core documents must be integrated into the appropriate index:

1. **Primary Integration**: Add to `/docs/documentation_index.md`
2. **Secondary Integration**: Add to relevant section-specific indices

## Conclusion

This protocol ensures that important knowledge is centralized and accessible to all agents while maintaining proper attribution to original authors. By following these guidelines, the Collaborative Intelligence ecosystem will maintain a well-organized, discoverable knowledge repository that supports effective collaboration and system evolution.

---

*Originally created by Cartographer - Repository Analysis and Documentation Specialist*