#!/bin/bash
# Markdown to HTML Conversion Script
# This script converts markdown files to elegant HTML using the Designer agent's system

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
TEMPLATE_FILE="$SCRIPT_DIR/template.html"
CSS_FILE="$SCRIPT_DIR/markdown-styles.css"
OUTPUT_DIR="$SCRIPT_DIR"

# Check for required tools
command -v pandoc >/dev/null 2>&1 || { echo "Error: pandoc is required but not installed. Please install pandoc."; exit 1; }

# Display usage information
function show_usage {
    echo "Usage: $0 [options] <input_file.md>"
    echo ""
    echo "Options:"
    echo "  -o, --output <file>    Specify output file (default: same name with .html extension)"
    echo "  -t, --theme <theme>    Specify theme (light, dark, sepia, minimal, ocean)"
    echo "  -n, --no-toc           Disable table of contents generation"
    echo "  -h, --help             Show this help message"
    echo ""
    exit 1
}

# Process command line arguments
GENERATE_TOC=true
THEME="light"
OUTPUT_FILE=""

while [[ $# -gt 0 ]]; do
    case $1 in
        -o|--output)
            OUTPUT_FILE="$2"
            shift 2
            ;;
        -t|--theme)
            THEME="$2"
            shift 2
            ;;
        -n|--no-toc)
            GENERATE_TOC=false
            shift
            ;;
        -h|--help)
            show_usage
            ;;
        -*)
            echo "Unknown option: $1"
            show_usage
            ;;
        *)
            INPUT_FILE="$1"
            shift
            ;;
    esac
done

# Check if input file is provided
if [ -z "$INPUT_FILE" ]; then
    echo "Error: No input file specified"
    show_usage
fi

# Check if input file exists
if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: Input file '$INPUT_FILE' does not exist"
    exit 1
fi

# Set default output file if not specified
if [ -z "$OUTPUT_FILE" ]; then
    # Replace .md extension with .html
    OUTPUT_FILE="${INPUT_FILE%.md}.html"
    # Handle files without .md extension
    if [ "$OUTPUT_FILE" = "$INPUT_FILE" ]; then
        OUTPUT_FILE="$INPUT_FILE.html"
    fi
fi

echo "Converting $INPUT_FILE to $OUTPUT_FILE..."

# Extract title from the markdown file (first # heading)
TITLE=$(grep -m 1 "^# " "$INPUT_FILE" | sed 's/^# //')
if [ -z "$TITLE" ]; then
    # Use filename as title if no heading found
    TITLE=$(basename "$INPUT_FILE" .md)
fi

# Temporary file for content
TEMP_FILE=$(mktemp)

# Generate table of contents if enabled
TOC_HTML=""
if [ "$GENERATE_TOC" = true ]; then
    TOC_HTML="<div class=\"toc\">\n<div class=\"toc-title\">Table of Contents</div>\n"
    TOC_HTML+="$(pandoc --toc --template="$SCRIPT_DIR/toc-template.html" "$INPUT_FILE")"
    TOC_HTML+="</div>\n"
fi

# Convert markdown to HTML
pandoc -f markdown -t html "$INPUT_FILE" -o "$TEMP_FILE"

# Read converted HTML content
HTML_CONTENT=$(cat "$TEMP_FILE")

# Read template
TEMPLATE=$(cat "$TEMPLATE_FILE")

# Get current timestamp
TIMESTAMP=$(date "+%Y-%m-%d %H:%M:%S")

# Replace placeholders in template
RESULT="${TEMPLATE/\{\{TITLE\}\}/$TITLE}"
RESULT="${RESULT/\{\{CONTENT\}\}/$TOC_HTML$HTML_CONTENT}"
RESULT="${RESULT/\{\{TIMESTAMP\}\}/$TIMESTAMP}"
RESULT="${RESULT/\{\{NAVIGATION\}\}/}"
RESULT="${RESULT/data-theme=\"light\"/data-theme=\"$THEME\"}"

# Write output
echo "$RESULT" > "$OUTPUT_FILE"

# Clean up
rm "$TEMP_FILE"

echo "Conversion complete: $OUTPUT_FILE"

# Copy the file to the same name in the output directory if different
if [ "$(dirname "$OUTPUT_FILE")" != "$OUTPUT_DIR" ]; then
    TARGET_FILE="$OUTPUT_DIR/$(basename "$OUTPUT_FILE")"
    cp "$OUTPUT_FILE" "$TARGET_FILE"
    echo "Copied to: $TARGET_FILE"
fi

echo "Done."