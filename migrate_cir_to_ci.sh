#!/bin/bash
# Script to migrate CIR references to CI within the CollaborativeIntelligence repository
# Phase 2 of the CLI modernization plan

set -e

YELLOW='\033[1;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}Starting CIR to CI migration for CollaborativeIntelligence repository...${NC}"

# Step 1: Create a backup of critical files
echo -e "\n${BLUE}Step 1: Creating backup of critical files...${NC}"
BACKUP_DIR="./backup_cir_migration_$(date +%Y%m%d%H%M%S)"
mkdir -p "$BACKUP_DIR"

# Backup .cir directory
if [ -d "./.cir" ]; then
  echo -e "  Backing up .cir directory..."
  cp -r ./.cir "$BACKUP_DIR/"
  echo -e "  ${GREEN}✓ Backup created at $BACKUP_DIR/.cir${NC}"
fi

echo -e "  ${GREEN}✓ Backups completed${NC}"

# Step 2: Rename .cir directory to .ci
echo -e "\n${BLUE}Step 2: Renaming .cir directory to .ci...${NC}"
if [ -d "./.cir" ]; then
  if [ -d "./.ci" ]; then
    echo -e "  ${RED}Warning: .ci directory already exists. Cannot rename .cir to .ci.${NC}"
    echo -e "  Please manually merge the contents of .cir into .ci."
  else
    mv ./.cir ./.ci
    echo -e "  ${GREEN}✓ Directory renamed from .cir to .ci${NC}"
  fi
else
  echo -e "  ${YELLOW}Note: .cir directory not found, nothing to rename.${NC}"
fi

# Step 3: Update metadata.json in the .ci directory
echo -e "\n${BLUE}Step 3: Updating metadata.json in the .ci directory...${NC}"
if [ -f "./.ci/metadata.json" ]; then
  echo -e "  Updating metadata.json..."
  sed -i.bak 's/CIR/CI/g; s/cir/ci/g' ./.ci/metadata.json
  rm -f ./.ci/metadata.json.bak
  echo -e "  ${GREEN}✓ metadata.json updated${NC}"
else
  echo -e "  ${YELLOW}Note: metadata.json not found in .ci directory, nothing to update.${NC}"
fi

# Step 4: Find and update any remaining CIR references in code
echo -e "\n${BLUE}Step 4: Finding and updating remaining CIR references...${NC}"

# Function to update CIR references in a file
update_file() {
  local file=$1
  local backup="${file}.bak"
  
  # Create a backup
  cp "$file" "$backup"
  
  # Perform replacements
  sed -i '' 's/\bCIR\b/CI/g; s/CIR_/CI_/g; s/\bcir\b/ci/g; s/_CIR\./_CI\./g' "$file"
  
  # Check if the file changed
  if cmp -s "$file" "$backup"; then
    # No changes, restore from backup
    rm "$backup"
    return 1
  else
    # Changes made, remove backup
    rm "$backup"
    return 0
  fi
}

# Update files with CIR references
files_updated=0
files_checked=0

# Process Rust files
echo -e "  Checking Rust files..."
while IFS= read -r file; do
  files_checked=$((files_checked + 1))
  if update_file "$file"; then
    files_updated=$((files_updated + 1))
    echo -e "  ${GREEN}✓ Updated: $file${NC}"
  fi
done < <(find . -type f -name "*.rs" | grep -v "target/")

# Process Markdown files
echo -e "  Checking Markdown files..."
while IFS= read -r file; do
  # Skip the migration report
  if [[ "$file" != *"CIR_TO_CI_MIGRATION_REPORT.md"* ]]; then
    files_checked=$((files_checked + 1))
    if update_file "$file"; then
      files_updated=$((files_updated + 1))
      echo -e "  ${GREEN}✓ Updated: $file${NC}"
    fi
  fi
done < <(find . -type f -name "*.md")

# Process TOML files
echo -e "  Checking TOML files..."
while IFS= read -r file; do
  files_checked=$((files_checked + 1))
  if update_file "$file"; then
    files_updated=$((files_updated + 1))
    echo -e "  ${GREEN}✓ Updated: $file${NC}"
  fi
done < <(find . -type f -name "*.toml")

# Process Shell scripts
echo -e "  Checking Shell scripts..."
while IFS= read -r file; do
  # Skip this migration script
  if [[ "$file" != *"migrate_cir_to_ci.sh"* ]] && [[ "$file" != *"remove_cir_references_dry_run.sh"* ]]; then
    files_checked=$((files_checked + 1))
    if update_file "$file"; then
      files_updated=$((files_updated + 1))
      echo -e "  ${GREEN}✓ Updated: $file${NC}"
    fi
  fi
done < <(find . -type f -name "*.sh")

# Process JavaScript files
echo -e "  Checking JavaScript files..."
while IFS= read -r file; do
  files_checked=$((files_checked + 1))
  if update_file "$file"; then
    files_updated=$((files_updated + 1))
    echo -e "  ${GREEN}✓ Updated: $file${NC}"
  fi
done < <(find . -type f -name "*.js")

echo -e "  ${GREEN}✓ Files checked: $files_checked${NC}"
echo -e "  ${GREEN}✓ Files updated: $files_updated${NC}"

echo -e "\n${GREEN}CIR to CI migration completed successfully!${NC}"
echo -e "${YELLOW}Please review the changes before committing.${NC}"

# Remove backup directory if everything went well
if [ $files_updated -eq 0 ]; then
  echo -e "\n${BLUE}No files were updated. Removing backup directory...${NC}"
  rm -rf "$BACKUP_DIR"
  echo -e "${GREEN}✓ Backup directory removed${NC}"
else
  echo -e "\n${BLUE}Backup directory preserved at: $BACKUP_DIR${NC}"
  echo -e "${YELLOW}You can remove it manually after verifying the changes.${NC}"
fi