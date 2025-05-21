#!/bin/bash
# Dry run script to show changes that would occur when removing CIR references

set -e

YELLOW='\033[1;33m'
GREEN='\033[0;32m'
RED='\033[0;31m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${YELLOW}DRY RUN: Showing changes that would occur when removing CIR references...${NC}"

# Change to the target directory
TARGET_DIR="../CI"
if [ ! -d "$TARGET_DIR" ]; then
  echo -e "${RED}Target directory $TARGET_DIR not found!${NC}"
  exit 1
fi

cd "$TARGET_DIR"

# Stage 1: Count files that would be modified
echo -e "${YELLOW}Files that would be modified:${NC}"
RUST_FILES=$(find . -type f -name "*.rs" | wc -l)
MD_FILES=$(find . -type f -name "*.md" | wc -l)
TOML_FILES=$(find . -type f -name "*.toml" | wc -l)

echo -e "  ${BLUE}Rust files:${NC} $RUST_FILES"
echo -e "  ${BLUE}Markdown files:${NC} $MD_FILES"
echo -e "  ${BLUE}TOML files:${NC} $TOML_FILES"

# Stage 2: Show sample replacements in Rust files
echo -e "\n${YELLOW}Sample replacements in Rust files:${NC}"
count=0
find . -type f -name "*.rs" | while read -r file; do
  if grep -q "\bCIR\b\|CIR_\|\bcir\b" "$file"; then
    count=$((count + 1))
    if [ $count -le 3 ]; then  # Only show first 3 examples
      echo -e "${BLUE}File:${NC} $file"
      grep -n "\bCIR\b\|CIR_\|\bcir\b" "$file" | head -5 | while read -r line; do
        echo "  $line"
        line_num=$(echo "$line" | cut -d':' -f1)
        orig_line=$(echo "$line" | cut -d':' -f2-)
        new_line=$(echo "$orig_line" | sed 's/\bCIR\b/CI/g; s/CIR_/CI_/g; s/\bcir\b/ci/g')
        echo -e "    ${GREEN}→ $new_line${NC}"
      done
      echo ""
    fi
  fi
done

# Stage 3: Show sample replacements in Markdown files
echo -e "\n${YELLOW}Sample replacements in Markdown files:${NC}"
count=0
find . -type f -name "*.md" | while read -r file; do
  if grep -q "\bCIR\b\|_CIR\.\|\bcir\b" "$file"; then
    count=$((count + 1))
    if [ $count -le 3 ]; then  # Only show first 3 examples
      echo -e "${BLUE}File:${NC} $file"
      grep -n "\bCIR\b\|_CIR\.\|\bcir\b" "$file" | head -5 | while read -r line; do
        echo "  $line"
        line_num=$(echo "$line" | cut -d':' -f1)
        orig_line=$(echo "$line" | cut -d':' -f2-)
        new_line=$(echo "$orig_line" | sed 's/\bCIR\b/CI/g; s/_CIR\./_CI\./g; s/\bcir\b/ci/g')
        echo -e "    ${GREEN}→ $new_line${NC}"
      done
      echo ""
    fi
  fi
done

# Stage 4: Show sample replacements in TOML files
echo -e "\n${YELLOW}Sample replacements in TOML files:${NC}"
count=0
find . -type f -name "*.toml" | while read -r file; do
  if grep -q "\bCIR\b\|\bcir\b" "$file"; then
    count=$((count + 1))
    if [ $count -le 3 ]; then  # Only show first 3 examples
      echo -e "${BLUE}File:${NC} $file"
      grep -n "\bCIR\b\|\bcir\b" "$file" | head -5 | while read -r line; do
        echo "  $line"
        line_num=$(echo "$line" | cut -d':' -f1)
        orig_line=$(echo "$line" | cut -d':' -f2-)
        new_line=$(echo "$orig_line" | sed 's/\bCIR\b/CI/g; s/\bcir\b/ci/g')
        echo -e "    ${GREEN}→ $new_line${NC}"
      done
      echo ""
    fi
  fi
done

# Stage 5: Count total references
echo -e "\n${YELLOW}Total CIR references that would be changed:${NC}"
TOTAL_REFS=$(grep -r "CIR" --include="*.rs" --include="*.md" --include="*.toml" . | wc -l)
echo -e "  ${BLUE}Total references:${NC} $TOTAL_REFS"

echo -e "\n${YELLOW}This was a DRY RUN. No files were modified.${NC}"
echo -e "${GREEN}To perform the actual changes, run the original remove_cir_references.sh script.${NC}"