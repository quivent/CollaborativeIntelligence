# Minimal CI Rust CLI

A lightweight Rust implementation of the Collaborative Intelligence CLI that directly maps the functionality of the original bash scripts.

## Features

- Minimal, focused implementation with direct 1:1 command parity
- Lightweight architecture with few dependencies
- All core commands implemented: init, integrate, status, fix, verify, load
- Designed for simplicity and ease of maintenance

## Commands

- `ci init PROJECT_NAME [OPTIONS]` - Initialize a new project with CI
- `ci integrate [PATH] [OPTIONS]` - Integrate CI into an existing project
- `ci status [PATH]` - Check CI integration status
- `ci fix [PATH] [TYPE]` - Fix CI integration issues
- `ci verify [PATH]` - Verify CI integration is working
- `ci load AGENT [--context=CONTEXT] [--path=PATH]` - Load agent memory

## Installation

### Quick Start

```bash
# Build and install
./build.sh
./install.sh
```

### Manual Installation

```bash
# Build
cargo build --release

# Install to ~/.local/bin
mkdir -p ~/.local/bin
cp target/release/ci ~/.local/bin/
chmod +x ~/.local/bin/ci

# Add to PATH if needed
echo 'export PATH="$PATH:$HOME/.local/bin"' >> ~/.bashrc
source ~/.bashrc
```

## Usage Examples

```bash
# Initialize a new project
ci init my-project

# Integrate CI into an existing project
ci integrate ~/projects/existing-project

# Check CI integration status
ci status

# Fix integration issues
ci fix
ci fix verify  # Fix verify command timeout issue

# Verify integration is working
ci verify

# Load agent memory
ci load Athena
ci load Athena --context="code implementation"
ci load Athena --path=/custom/path/to/memory.md
```

## Design Philosophy

This CLI was built with a focus on:

1. **Simplicity** - Direct mapping to bash commands
2. **Efficiency** - Minimal dependencies, fast execution
3. **Compatibility** - Works just like the original CLI
4. **Maintenance** - Easy to understand and update

## Implementation Notes

Each command directly mirrors the functionality from the bash scripts:

- `init` - Creates a new project with CI integration
- `integrate` - Adds CI to an existing project
- `status` - Checks integration status
- `fix` - Repairs integration issues
- `verify` - Tests if integration is working
- `load` - Loads agent memory (added command)

## Contributing

1. Fork the repository
2. Create a feature branch
3. Submit a pull request

## License

MIT