# MCP Tools for Claude Code

This document provides a comprehensive list of MCP (Model Context Protocol) tools available for Claude Code, including both built-in tools and popular external MCP servers.

## Built-in Tools

Claude Code includes several built-in tools that don't require additional MCP server configuration:

### Core Tools

1. **Task** - Launch a new agent with access to a full suite of tools for complex, delegated tasks
2. **Bash** - Execute shell commands with detailed controls and safety mechanisms
3. **Glob** - Fast file pattern matching using glob syntax (e.g., "**/*.js")
4. **Grep** - Content-based file searching using regular expressions
5. **LS** - List files and directories in a given path
6. **Read** - Read file contents with support for images and line-based retrieval
7. **Edit** - Make targeted edits to files with precise context
8. **MultiEdit** - Perform multiple edits to a single file in one operation
9. **Write** - Create or completely overwrite files
10. **NotebookRead** - Read Jupyter notebook files with cell structure preservation
11. **NotebookEdit** - Edit specific cells in Jupyter notebooks
12. **WebFetch** - Retrieve and process content from web URLs
13. **Batch** - Run multiple tools in parallel for efficiency
14. **TodoRead** - Retrieve task list information
15. **TodoWrite** - Create and update task lists
16. **WebSearch** - Search the internet for up-to-date information

### Git Integration

Claude Code has deep Git integration and can:
- View and understand commit history
- Create commits with meaningful messages
- Generate pull requests
- Analyze and resolve merge conflicts
- Work with branches and tags

## Popular External MCP Servers

### Official MCP Servers

Anthropic provides pre-built MCP servers for popular enterprise systems:

1. **Google Drive** - Access and manipulate documents in Google Drive
2. **Slack** - Interact with Slack channels and messages
3. **GitHub** - Enhanced GitHub integrations beyond the built-in capabilities
4. **Git** - Extended Git functionality
5. **Postgres** - Query and interact with PostgreSQL databases
6. **Puppeteer** - Web automation and testing

### Community-Developed MCP Servers

The community has developed numerous MCP servers, including:

1. **Ableton Live** - Control Ableton Live for music production
2. **Airbnb** - Search Airbnb and get listing details
3. **AI Agent Marketplace Index** - Search more than 5000+ AI agents and tools
4. **Algorand** - Tools for blockchain interactions
5. **Airflow** - Connect to Apache Airflow using official python client
6. **Arduino** - Enable AI-powered robotics using Claude and Arduino
7. **Atlassian** - Interact with Confluence and Jira
8. **AWS** - Perform operations on AWS resources
9. **Azure DevOps** - Bridge to Azure DevOps services
10. **Baidu AI Search** - Web search with Baidu Cloud's AI Search
11. **BigQuery** - Inspect schemas and execute queries on Google BigQuery
12. **Bing Web Search** - Implementation for Microsoft Bing Web Search API
13. **Desktop Commander** - Access file system and terminal across the entire OS

## Configuring MCP Tools

There are two primary methods to configure MCP tools:

### 1. CLI Wizard

```bash
claude mcp add my-server /path/to/server
```

### 2. Direct Configuration

Edit `.claude.json` directly:

```json
{
  "mcpServers": {
    "my-custom-tool": {
      "type": "stdio",
      "command": "node",
      "args": ["/path/to/tools/my-custom-tool/build/index.js"]
    }
  }
}
```

You can set up MCP servers at three different scopes:

1. **Local Scope** (default): Available only to you in the current project
   ```bash
   claude mcp add my-private-server /path/to/server
   ```

2. **Project Scope**: Available to all team members (stored in `.mcp.json`)
   ```bash
   claude mcp add shared-server -s project /path/to/server
   ```

3. **User Scope**: Available to you across all projects
   ```bash
   claude mcp add my-user-server -s user /path/to/server
   ```

## Debugging MCP

To troubleshoot MCP tool issues, use:

```bash
claude --mcp-debug
```

Within Claude Code, check server status with:

```
/mcp
```

## Custom Commands for MCP Workflows

For specialized workflows, you can create custom slash commands in `.claude/commands/`:

```markdown
# /fix-issue

Please analyze and fix the GitHub issue: $ARGUMENTS.
Follow these steps:
1. Use `gh issue view` to get the issue details
2. Understand the problem described in the issue
3. Search the codebase for relevant files
4. Implement the necessary changes to fix the issue
```

These custom commands can be shared with your team via version control.

## Advanced MCP Features

- **Non-Interactive Mode**: Run Claude Code in headless mode for CI/CD pipelines
- **Permission Bypassing**: Use `--dangerously-skip-permissions` for uninterrupted operations
- **Custom MCP Servers**: Create your own tools by implementing the MCP protocol
- **Chaining MCP Servers**: Connect multiple tools together for complex workflows

## Resources

- [Claude MCP Community](https://www.claudemcp.com/)
- [Model Context Protocol GitHub Repository](https://github.com/modelcontextprotocol/)
- [Anthropic MCP Documentation](https://docs.anthropic.com/en/docs/agents-and-tools/mcp)
- [Claude Code Best Practices](https://www.anthropic.com/engineering/claude-code-best-practices)