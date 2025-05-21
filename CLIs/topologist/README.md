# Topologist - Portable Repository Topology Management

![Version](https://img.shields.io/badge/version-0.1.0-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

A zero-dependency, portable toolkit for intelligent repository organization and systematic commit management. Topologist analyzes your repository state, generates optimal commit plans, and executes them with detailed size tracking - all without leaving traces back to any source system.

## ✨ Features

- **🔍 Intelligent Analysis**: Smart file categorization with pattern matching
- **📋 Strategic Planning**: Generates optimal phasal commit strategies
- **📈 Size Tracking**: Real-time repository growth measurement
- **🏃 Portable Execution**: Works in any git repository, any environment
- **🧹 Clean Footprint**: Complete metadata removal with single command
- **⚡ Zero Dependencies**: Pure git + standard tools, no external services

## 🚀 Quick Start

```bash
# Install from source
cd CLIs/topologist
cargo build --release

# Copy binary to PATH
cp target/release/ci-topologist /usr/local/bin/

# Use in any git repository
cd /path/to/any/git/repo
ci-topologist analyze
```

## 📖 Usage

### Basic Workflow

```bash
# 1. Analyze repository (no files created)
ci-topologist analyze
# Output: "Found 15 files, 4-phase plan, +2.1KB estimated"

# 2. Generate executable plan
ci-topologist plan --save
# Creates .topologist/ metadata directory

# 3. Execute phases with size tracking
ci-topologist execute all
# Executes all phases with detailed progress

# 4. Clean all traces (optional)
ci-topologist clean
# Removes all metadata, repository is clean
```

### Advanced Usage

```bash
# Execute specific phases
ci-topologist execute 1
ci-topologist execute 2

# Check current session status
ci-topologist status

# View size change tracking
ci-topologist track

# Export session data
ci-topologist export > session.json

# CI/CD integration
if ci-topologist analyze --has-unorganized; then
  ci-topologist execute all
fi
```

## 🏗️ Architecture

### Core Components

```
topologist/
├── src/
│   ├── file_categorizer.rs    # Smart file pattern matching
│   ├── commit_planner.rs      # Strategic commit sequence generation
│   ├── git_operations.rs      # Safe git command execution
│   ├── metadata_manager.rs    # Local state persistence
│   ├── lib.rs                 # Core library interface
│   └── main.rs                # CLI interface
├── Cargo.toml                 # Dependencies and metadata
└── README.md                  # This file
```

### File Categories

Topologist automatically categorizes files into intelligent groups:

- **⚙️ Configuration**: `*.json`, `*.yaml`, `.env*`, `Makefile*`
- **📚 Documentation**: `*.md`, `docs/`, `README*`, `CHANGELOG*` 
- **💻 Source Code**: `*.rs`, `*.js`, `*.py`, language-specific patterns
- **🔧 Development Tools**: `scripts/`, `*.sh`, CI/CD files
- **🎨 Media Assets**: `*.png`, `*.jpg`, `assets/`, static files
- **📦 Build Artifacts**: `target/`, `build/`, `*.min.*`, binaries

### Commit Strategies

- **Sequential**: Standard category-based progression
- **Size Optimized**: Balanced commit sizes for efficient review
- **Category First**: Logical grouping by file type and purpose
- **Dependency Aware**: Considers file interdependencies

## 📊 Metadata Structure

### Minimal Footprint Design

```
.topologist/                    # Hidden directory (~2KB total)
├── config.json                # Project configuration
├── commit_history.json        # Session tracking
└── analysis_cache.json        # Temporary analysis cache
```

### Configuration Format

```json
{
  "version": "1.0",
  "project_id": "uuid-generated",
  "created": "2025-05-22T05:50:00.000Z",
  "commit_strategy": "phasal",
  "size_tracking": true,
  "auto_gitignore": true,
  "phases_completed": [1, 2, 3]
}
```

## 🔧 Development

### Building from Source

```bash
# Clone and build
git clone <repository-url>
cd CLIs/topologist
cargo build --release

# Run tests
cargo test

# Development build with debug info
cargo build
```

### Testing

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test file_categorizer
cargo test commit_planner

# Run with output
cargo test -- --nocapture
```

## 🌟 Examples

### Example Output

```bash
$ ci-topologist analyze
🔍 Repository Analysis Complete
├── 15 total files detected
├── 12 untracked files
├── 3 modified files
├── Estimated growth: +2,150 lines
└── Suggested: 4-phase commit strategy

📂 File Categories:
   ⚙️ Configuration: 3 files
   📚 Documentation: 5 files
   💻 SourceCode: 4 files
   🔧 DevelopmentTools: 3 files

💡 Run 'ci-topologist plan --save' to proceed
```

```bash
$ ci-topologist execute all
🚀 Executing all 4 phases...

📦 Phase 1: config: Add 3 configuration files
   ✅ Commit: a1b2c3d4 (+150 estimated lines)
   📁 Files: 3

📦 Phase 2: docs: Add 5 documentation and guides
   ✅ Commit: e5f6g7h8 (+800 estimated lines)
   📁 Files: 5

📦 Phase 3: feat: Add 4 source code files
   ✅ Commit: i9j0k1l2 (+900 estimated lines)
   📁 Files: 4

📦 Phase 4: tools: Add 3 development tools and scripts
   ✅ Commit: m3n4o5p6 (+300 estimated lines)
   📁 Files: 3

🎉 All phases complete! Repository synchronized.
```

## 📋 Command Reference

### Core Commands

| Command | Description | Options |
|---------|-------------|---------|
| `analyze` | Scan repository state | `--has-unorganized` |
| `plan` | Generate commit plan | `--save`, `--check` |
| `execute <phase\|all>` | Execute phases | `1`, `2`, `all` |
| `status` | Show session status | |
| `track` | Display size tracking | |
| `init` | Initialize metadata | |
| `clean` | Remove all traces | |
| `export` | Export session data | |

### Integration Examples

#### Git Hooks

```bash
# .git/hooks/pre-commit
#!/bin/bash
if ci-topologist analyze --has-unorganized; then
  echo "Run 'ci-topologist execute all' to organize repository"
  exit 1
fi
```

#### GitHub Actions

```yaml
- name: Repository Organization Check
  run: |
    if ci-topologist analyze --has-unorganized; then
      ci-topologist execute all
      git push
    fi
```

## 🚨 Important Notes

### Security & Privacy

- **Zero Trace-Back**: No references to source repositories
- **Local Only**: No network communication or external data transmission
- **Clean Exit**: Complete metadata removal with `clean` command
- **Path Sanitization**: All paths stored relative to project root

### Limitations

- Requires working git repository
- File size estimation is approximate
- Dependency analysis is currently basic
- Works best with standard project structures

## 🛠️ Troubleshooting

### Common Issues

**"Not a git repository"**
```bash
git init  # Initialize git if needed
```

**"Project not initialized"**
```bash
ci-topologist init  # Initialize topologist metadata
```

**Large files detected**
```bash
# Check .gitignore patterns
ci-topologist analyze  # Shows file categorization
```

### Debug Mode

```bash
# Set environment variable for detailed logging
RUST_LOG=debug ci-topologist analyze
```

## 📜 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## ⭐ Acknowledgments

- Built for universal git repository management
- Inspired by systematic commit organization patterns
- Designed for privacy-first, portable deployment
- Zero-dependency architecture for maximum compatibility