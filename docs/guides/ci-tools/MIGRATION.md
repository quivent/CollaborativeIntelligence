# CI Tools Migration Guide

## Overview

This document explains the migration of files from the `ci-tools` directory to their appropriate locations within the CollaborativeIntelligence repository. This reorganization aims to improve the project structure by placing files in more appropriate locations based on their functionality, aligning with the project's Rust-first approach.

## File Relocations

The following files have been relocated from their original locations in `ci-tools`:

### Memory-Related Shell Scripts
| Original Location | New Location |
|-------------------|-------------|
| `ci-tools/lib/rust-memory-bridge.sh` | `scripts/legacy/rust-memory-bridge.sh` |
| `ci-tools/lib/memory-loader.sh` | `scripts/legacy/memory-loader.sh` |
| `ci-tools/lib/common.sh` | `scripts/legacy/common.sh` |
| `ci-tools/lib/output.sh` | `scripts/legacy/output.sh` |
| `ci-tools/lib/config.sh` | `scripts/legacy/config.sh` |

### Main CI Script
| Original Location | New Location |
|-------------------|-------------|
| `ci-tools/CI` | `ci-rust-minimal/legacy/CI` |

### Agent Activator
| Original Location | New Location |
|-------------------|-------------|
| `ci-tools/lib/agent-activator.sh` | `agent_cache/legacy/agent-activator.sh` |

### Documentation
| Original Location | New Location |
|-------------------|-------------|
| `ci-tools/README.md` | `docs/guides/ci-tools/README.md` |
| `ci-tools/USAGE.md` | `docs/guides/ci-tools/USAGE.md` |

### Testing Scripts
| Original Location | New Location |
|-------------------|-------------|
| `ci-tools/test/test_memory_bridge.sh` | `scripts/tests/legacy/test_memory_bridge.sh` |
| `ci-tools/test/test_memory_cli.sh` | `scripts/tests/legacy/test_memory_cli.sh` |
| `ci-tools/test/run_benchmarks.sh` | `scripts/tests/legacy/run_benchmarks.sh` |
| `ci-tools/test/run_tests.sh` | `scripts/tests/legacy/run_tests.sh` |
| `ci-tools/test/test_fallback_mechanisms.sh` | `scripts/tests/legacy/test_fallback_mechanisms.sh` |
| `ci-tools/test/mock_legacy_functions.sh` | `scripts/tests/legacy/mock_legacy_functions.sh` |

### Templates
| Original Location | New Location |
|-------------------|-------------|
| `ci-tools/templates/CLAUDE.md.template` | `core/templates/CLAUDE.md.template` |

## Migration Strategy

This migration follows the repository's transition from Bash scripts to Rust implementations as defined in the project's CLAUDE.md configuration. All relocated shell scripts are now in "legacy" directories, indicating that they should eventually be replaced with Rust implementations.

### Major Changes

1. **Memory-related shell scripts** - Moved to the `scripts/legacy` directory to live alongside the Rust implementations in the `scripts` directory
2. **Main CI script** - Relocated to `ci-rust-minimal/legacy` to make way for the Rust implementation in `ci-rust-minimal`
3. **Agent activator** - Moved to `agent_cache/legacy` to be closer to the agent cache functionality
4. **Documentation** - Consolidated into `docs/guides/ci-tools` for better organization
5. **Testing scripts** - Relocated to `scripts/tests/legacy` to better align with Rust testing structure
6. **Templates** - Moved to `core/templates` to align with core system components

## Updating References

A utility script has been created to update references to these files across the codebase:

```bash
# Run the reference update script
/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/scripts/legacy/update_references.sh
```

This script will:
1. Search for references to the old file paths in all relevant files
2. Update them to point to the new locations
3. Check for and optionally update symbolic links that reference the moved files

## Symbolic Links

If you previously created symbolic links to any of the relocated files (especially `agent-activator.sh`), you'll need to update them. Common symbolic links that may need updating:

```bash
# Update @ and # agent activator links
ln -sf "$(pwd)/agent_cache/legacy/agent-activator.sh" /usr/local/bin/\@
ln -sf "$(pwd)/agent_cache/legacy/agent-activator.sh" /usr/local/bin/\#
```

## Next Steps

As part of the project's Rust-first approach, the next steps should include:

1. Converting the legacy Bash scripts to Rust implementations
2. Updating any remaining references to the old file paths
3. Implementing tests for the new Rust components
4. Eventually removing the legacy directories once all functionality is properly migrated to Rust

## Timeline

The shell scripts in legacy directories should be considered deprecated, with new development focusing exclusively on their Rust counterparts. The tentative timeline for full migration is:

1. References updated immediately after file relocation
2. Rust implementations to replace shell scripts
3. Legacy shell scripts to be removed once Rust implementations are stable

## Troubleshooting

If you encounter issues after this migration:

1. Run the update_references.sh script to ensure all references are updated
2. Check any custom scripts that might reference the old paths
3. Verify symbolic links are pointing to the correct locations
4. Run the test suite to identify any remaining issues

For any persistent issues, refer to the project's issue tracker or contact the development team.