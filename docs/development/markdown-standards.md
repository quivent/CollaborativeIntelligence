# Markdown Documentation Standards

This document defines the standards for markdown files in the Collaborative Intelligence system, including file organization, location requirements, naming conventions, and formatting guidelines.

## File Organization

### Location Requirements

1. **Root-Level Restrictions**
   - Markdown files should NOT be placed at the repository root level except for:
     - README.md (main repository documentation)
     - CLAUDE.md (system configuration)
     - License files
     - Contribution guidelines
   - All other documentation must be placed in appropriate subdirectories

2. **Directory Structure**
   - All documentation should reside in the `/docs` directory, organized by category:
     - `/docs/core-concepts/` - Fundamental system principles
     - `/docs/system-design/` - System architecture and implementation details
     - `/docs/agent-system/` - Agent-related documentation
     - `/docs/guides/` - User and developer guides
     - `/docs/protocols/` - Communication and interaction protocols
     - `/docs/development/` - Development standards and guidelines
     - `/docs/integration/` - Integration with other systems

3. **Agent Documentation**
   - Agent-specific documentation should be placed in the appropriate agent directory:
     - `/AGENTS/[AgentName]/README.md` - Agent overview
     - `/AGENTS/[AgentName]/MEMORY.md` - Agent memory
     - `/AGENTS/[AgentName]/ContinuousLearning.md` - Agent learning updates
     - `/AGENTS/[AgentName]/Sessions/` - Session-specific documentation

4. **Project Documentation**
   - Project-specific documentation should be placed in project directories:
     - `/projects/[ProjectName]/docs/` - Project documentation

## Naming Conventions

1. **File Names**
   - Use kebab-case for documentation files (e.g., `memory-architecture.md`)
   - Special files may use UPPER_CASE (e.g., `AGENT_PROTOCOL_REFERENCE.md`)
   - Files that represent primary identity should use PascalCase (e.g., `README.md`)

2. **Directory Names**
   - Use kebab-case for directory names (e.g., `core-concepts/`)
   - Special directories may use other conventions as established in the repository structure

## Document Structure

1. **Document Header**
   - Each document must begin with a single H1 title
   - The title should clearly describe the document's content
   - Example: `# Memory Architecture`

2. **Section Organization**
   - Use H2 (`##`) for main sections
   - Use H3 (`###`) for subsections
   - Use H4 (`####`) for detailed points
   - Maintain logical hierarchy - don't skip levels

3. **Required Sections**
   - Overview/Introduction - Brief explanation of the document's purpose
   - Main Content - Organized by topic
   - References (if applicable) - Links to related documents

## Formatting Guidelines

1. **Text Formatting**
   - Use **bold** for emphasis
   - Use *italics* for secondary emphasis or terminology
   - Use `code blocks` for code, commands, or filenames
   - Use > blockquotes for quotes or important notes

2. **Lists**
   - Use ordered lists (1. 2. 3.) for sequential steps or prioritized items
   - Use unordered lists (- or *) for non-sequential items
   - Maintain consistent indentation for nested lists

3. **Code Blocks**
   - Use triple backticks with language specification for code blocks
   - Example:
     ```markdown
     ```python
     def example():
         return "This is a code example"
     ```
     ```

4. **Tables**
   - Use tables for structured data
   - Include a header row
   - Align columns appropriately

5. **Links**
   - Use relative links for internal documentation
   - Use absolute links for external resources
   - Include descriptive link text

## Images and Media

1. **Image Storage**
   - Store images in an `/assets` or `/images` subdirectory relative to the document
   - Use meaningful filenames for images

2. **Image References**
   - Use the following format: `![Alt Text](path/to/image.png "Optional Title")`
   - Include descriptive alt text for accessibility

## Document Metadata

1. **Front Matter** (Optional)
   - For documents that require metadata, use YAML front matter at the start of the file:
     ```yaml
     ---
     title: Document Title
     description: A brief description
     author: Agent Name
     created: YYYY-MM-DD
     updated: YYYY-MM-DD
     ---
     ```

## Cross-References

1. **Internal References**
   - Use relative paths for links to other documents
   - Example: `[Memory Architecture](../core-concepts/memory-architecture.md)`

2. **See Also Sections**
   - Include a "See Also" section at the end of documents to reference related content

## Implementation Guidelines

1. **Transitioning to Standardized Structure**
   - When updating existing documents, move them to the appropriate location in the `/docs` directory
   - Create redirects or duplicates temporarily if necessary to maintain backward compatibility
   - Update references in other documents to point to the new locations

2. **Directory README Files**
   - Each directory in the `/docs` structure should have a README.md file explaining:
     - Purpose of the directory
     - Types of documents contained
     - Navigation guide to key documents

3. **Documentation Index**
   - Maintain the master documentation index at `/docs/documentation_index.md`
   - Update the index when adding, moving, or removing documentation

## Benefits of Standardization

Standardizing markdown documentation provides:

1. **Improved Navigation** - Logical organization makes finding information easier
2. **Consistency** - Uniform structure across all documentation
3. **Maintainability** - Clear standards for updates and additions
4. **Reduced Redundancy** - Centralized documentation prevents duplication
5. **Better Onboarding** - New users can quickly understand the system

---

*This standard was created by the Cartographer agent as part of the repository restructuring initiative.*