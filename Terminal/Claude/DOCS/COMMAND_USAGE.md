# Collaborative Intelligence Command-Line Tools

This document provides usage instructions for the CI command-line tools now available in the root directory.

## Core Commands

### CI

Main command-line tool for Collaborative Intelligence.

```bash
CI COMMAND [OPTIONS]
```

Subcommands:
- `init PROJECT_NAME` - Initialize a project with CI
- `integrate [PATH]` - Integrate CI into an existing project
- `status [PATH]` - Check CI integration status
- `fix [PATH] [TYPE]` - Repair CI integration issues
- `verify [PATH]` - Verify CI integration is working properly

### ci-init

Simple project initializer (shorthand for `CI init`).

```bash
ci-init PROJECT_NAME
```

### ci-verify

Verify CI integration is working properly (shorthand for `CI verify`).

```bash
ci-verify
```

### ci-fix

Fix common CI integration issues (shorthand for `CI fix`).

```bash
ci-fix
```

## New Management Commands

### ci-agent

Agent management tool for Collaborative Intelligence.

```bash
ci-agent SUBCOMMAND [OPTIONS]
```

Subcommands:
- `list` - List all available agents
- `info <agent_name>` - Show detailed information about an agent
- `create <agent_name>` - Create a new agent
- `enable <agent_name>` - Enable an agent in the current project
- `disable <agent_name>` - Disable an agent in the current project
- `template <template>` - Create a new agent from a predefined template

Examples:
```bash
ci-agent list
ci-agent info Athena
ci-agent create MyAgent
ci-agent enable Gaia
ci-agent disable TheFixer
```

### ci-session

Session management tool for Collaborative Intelligence.

```bash
ci-session SUBCOMMAND [OPTIONS]
```

Subcommands:
- `list` - List all available sessions
- `info <session_id>` - Show detailed information about a session
- `create <session_name>` - Create a new session
- `archive <session_id>` - Archive a completed session
- `export <session_id>` - Export a session to markdown or JSON
- `import <file>` - Import a session from markdown or JSON

Options:
- `-a, --agent AGENT` - Specify agent for the session
- `--format FORMAT` - Specify export format (markdown or json)

Examples:
```bash
ci-session list
ci-session info 20250515-feature-design
ci-session create "Feature Design"
ci-session archive 20250515-feature-design
ci-session export 20250515-feature-design --format markdown
```

## Environment Variables

- `CI_REPO_PATH` - Path to the CI repository (default: auto-detected)

## Best Practices

1. Use `ci-agent list` to see available agents before enabling them
2. Create descriptive session names with `ci-session create`
3. Use the `--agent` flag with `ci-session create` to associate sessions with specific agents
4. Export important sessions with `ci-session export` for documentation