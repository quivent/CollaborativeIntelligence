#!/usr/bin/env bash
# update_references.sh - Update references to moved files across the codebase
# Part of ci-tools reorganization effort

# Define color codes for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Define the project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

# Print section header
print_header() {
  echo -e "${YELLOW}=== $1 ===${NC}"
}

# Print success message
print_success() {
  echo -e "${GREEN}✓ $1${NC}"
}

# Print error message
print_error() {
  echo -e "${RED}✗ $1${NC}"
}

# Print informational message
print_info() {
  echo -e "$1"
}

# Update path mappings
declare -A PATH_MAPPINGS=(
  # Memory-related shell scripts
  ["ci-tools/lib/rust-memory-bridge.sh"]="scripts/legacy/rust-memory-bridge.sh"
  ["ci-tools/lib/memory-loader.sh"]="scripts/legacy/memory-loader.sh"
  ["ci-tools/lib/common.sh"]="scripts/legacy/common.sh"
  ["ci-tools/lib/output.sh"]="scripts/legacy/output.sh"
  ["ci-tools/lib/config.sh"]="scripts/legacy/config.sh"
  
  # Main CI script
  ["ci-tools/CI"]="ci-rust-minimal/legacy/CI"
  
  # Agent activator
  ["ci-tools/lib/agent-activator.sh"]="agent_cache/legacy/agent-activator.sh"
  
  # Documentation
  ["ci-tools/README.md"]="docs/guides/ci-tools/README.md"
  ["ci-tools/USAGE.md"]="docs/guides/ci-tools/USAGE.md"
  
  # Testing scripts
  ["ci-tools/test/test_memory_bridge.sh"]="scripts/tests/legacy/test_memory_bridge.sh"
  ["ci-tools/test/test_memory_cli.sh"]="scripts/tests/legacy/test_memory_cli.sh"
  ["ci-tools/test/run_benchmarks.sh"]="scripts/tests/legacy/run_benchmarks.sh"
  ["ci-tools/test/run_tests.sh"]="scripts/tests/legacy/run_tests.sh"
  ["ci-tools/test/test_fallback_mechanisms.sh"]="scripts/tests/legacy/test_fallback_mechanisms.sh"
  ["ci-tools/test/mock_legacy_functions.sh"]="scripts/tests/legacy/mock_legacy_functions.sh"
  
  # Templates
  ["ci-tools/templates/CLAUDE.md.template"]="core/templates/CLAUDE.md.template"
)

print_header "Updating file references across the codebase"

# First, find all shell scripts and other files that might contain references
print_info "Finding files that may contain references to moved files..."
FILES_TO_CHECK=$(find "$PROJECT_ROOT" -type f \( -name "*.sh" -o -name "*.md" -o -name "*.rs" -o -name "*.js" -o -name "*.json" -o -name "*.toml" \) -not -path "*/target/*" -not -path "*/.git/*")

# Count of updated files
UPDATED_COUNT=0

# For each file that might contain references
for file in $FILES_TO_CHECK; do
  # Skip the current script
  if [[ "$file" == "$0" ]]; then
    continue
  fi
  
  # Flag to track if this file was modified
  FILE_MODIFIED=false
  
  # Check each path mapping
  for old_path in "${!PATH_MAPPINGS[@]}"; do
    new_path="${PATH_MAPPINGS[$old_path]}"
    
    # If the file contains the old path
    if grep -q "$old_path" "$file"; then
      # Replace the old path with the new path
      sed -i.bak "s|$old_path|$new_path|g" "$file"
      
      # Mark file as modified
      FILE_MODIFIED=true
      print_info "  Updated references in: $file"
    fi
  done
  
  # If the file was modified, increment the counter
  if [[ "$FILE_MODIFIED" == "true" ]]; then
    ((UPDATED_COUNT++))
    
    # Remove backup files
    rm -f "${file}.bak"
  fi
done

print_success "Updated references in $UPDATED_COUNT files"

# Update symbolic links if they exist
print_header "Checking for symbolic links to update"

# Common locations for symbolic links
LINK_LOCATIONS=(
  "/usr/local/bin/@"
  "/usr/local/bin/#"
  "$HOME/bin/@"
  "$HOME/bin/#"
)

for link in "${LINK_LOCATIONS[@]}"; do
  if [[ -L "$link" ]]; then
    # Get the current target
    current_target=$(readlink "$link")
    
    # Check if it points to the old location
    if [[ "$current_target" == *"ci-tools/lib/agent-activator.sh"* ]]; then
      print_info "Found symbolic link: $link"
      print_info "  Current target: $current_target"
      
      # Calculate the new target
      new_target="${current_target/ci-tools\/lib\/agent-activator.sh/agent_cache\/legacy\/agent-activator.sh}"
      print_info "  New target would be: $new_target"
      
      # Ask user if they want to update the link
      read -p "Update this symbolic link? (y/n) " -n 1 -r
      echo
      if [[ $REPLY =~ ^[Yy]$ ]]; then
        # Try to update the link
        if ln -sf "$new_target" "$link"; then
          print_success "Updated symbolic link: $link"
        else
          print_error "Failed to update symbolic link: $link (permission denied?)"
          print_info "  Try running: sudo ln -sf \"$new_target\" \"$link\""
        fi
      else
        print_info "Skipping update for $link"
      fi
    fi
  fi
done

print_header "Reference update complete"
print_info "If any scripts fail after this update, check file paths manually."
print_info "For Rust code, you may need to update imports and cargo configurations separately."
print_info "Consider running the test suite to verify everything still works correctly."

exit 0