#!/bin/bash
# Enhanced Markdown-to-HTML Converter Script
# Designer Agent - Advanced Markdown System
#
# This script converts markdown files to HTML using the enhanced template system.
# Supports advanced features like frontmatter, admonitions, tabs, and more.
#
# IMPORTANT: DESIGN PRECEDENCE PRINCIPLE
# The Designer's creative decisions always take precedence over this script's defaults.
# This script is meant as a foundation that the Designer can freely override, modify,
# or bypass entirely when better design approaches are available. The Designer has
# complete creative autonomy to diverge from this script when aesthetically appropriate.

# Default values
OUTPUT_DIR="../../../MARKDOWN_DESIGNS"
TEMPLATE_PATH="$(dirname "$0")/../template-enhanced.html"
STYLES_PATH="$(dirname "$0")/../markdown-styles-enhanced.css"
THEME="light"
LAYOUT="centered"
SHOW_TOC=true
SITE_NAME="Markdown Documentation"
HOME_LINK="#"
AUTHOR="Designer Agent"
INCLUDE_JAVASCRIPT=true
TOC_SIDEBAR=false
SIDEBAR=false
DESIGN_LANGUAGE=""
SOCIAL_PREVIEW=true
SITE_URL=""

# Help message
show_help() {
    echo "Enhanced Markdown to HTML Converter"
    echo ""
    echo "Usage: $0 [options] <markdown_file>"
    echo ""
    echo "Options:"
    echo "  -o, --output <path>        Output file name (default: <markdown_name>.html)"
    echo "  -d, --dir <path>           Output directory (default: $OUTPUT_DIR)"
    echo "  -t, --theme <theme>        Specify theme (light, dark, sepia, minimal, ocean, tao)"
    echo "  -l, --layout <layout>      Specify layout (centered, sidebar, toc)"
    echo "  --no-toc                   Disable table of contents"
    echo "  --no-js                    Disable JavaScript functionality"
    echo "  --no-social                Disable social media preview metadata"
    echo "  -s, --site-name <name>        Site name to display in header"
    echo "  --site-url <url>           Base URL for site (used in social media previews)"
    echo "  -n, --next <url:title>     Next page URL and title (separated by colon)"
    echo "  -p, --prev <url:title>     Previous page URL and title (separated by colon)"
    echo "  --template <path>          Custom template path"
    echo "  --styles <path>            Custom styles path"
    echo "  --design <description>     Natural language design description"
    echo "  -h, --help                 Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 document.md"
    echo "  $0 --theme dark --layout toc document.md" 
    echo "  $0 --output custom.html --no-toc document.md"
    echo "  $0 --design \"earth tones with modern typography and rounded elements\" document.md"
    echo ""
}

# Check dependencies
check_dependencies() {
    if ! command -v pandoc &> /dev/null; then
        echo "Error: pandoc is not installed."
        echo "Please install it using your package manager or from https://pandoc.org/installing.html"
        exit 1
    fi
}

# Create temporary file
create_temp_file() {
    TEMP_DIR=$(mktemp -d)
    TEMP_HTML="$TEMP_DIR/output.html"
    TEMP_STYLES="$TEMP_DIR/styles.css"
    TEMP_TEMPLATE="$TEMP_DIR/template.html"
}

# Parse arguments
parse_args() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            -o|--output)
                OUTPUT_FILE="$2"
                shift 2
                ;;
            -d|--dir)
                OUTPUT_DIR="$2"
                shift 2
                ;;
            -t|--theme)
                THEME="$2"
                shift 2
                ;;
            -l|--layout)
                LAYOUT="$2"
                shift 2
                ;;
            --no-toc)
                SHOW_TOC=false
                shift
                ;;
            --no-js)
                INCLUDE_JAVASCRIPT=false
                shift
                ;;
            --no-social)
                SOCIAL_PREVIEW=false
                shift
                ;;
            -s|--site-name)
                SITE_NAME="$2"
                shift 2
                ;;
            --site-url)
                SITE_URL="$2"
                shift 2
                ;;
            -n|--next)
                NEXT_PAGE="$2"
                shift 2
                ;;
            -p|--prev)
                PREV_PAGE="$2"
                shift 2
                ;;
            --template)
                TEMPLATE_PATH="$2"
                shift 2
                ;;
            --styles)
                STYLES_PATH="$2"
                shift 2
                ;;
            --design)
                DESIGN_LANGUAGE="$2"
                shift 2
                ;;
            -h|--help)
                show_help
                exit 0
                ;;
            -*|--*)
                echo "Unknown option $1"
                show_help
                exit 1
                ;;
            *)
                MARKDOWN_FILE="$1"
                shift
                ;;
        esac
    done

    # Validate input file
    if [ -z "$MARKDOWN_FILE" ]; then
        echo "Error: No input file specified."
        show_help
        exit 1
    fi

    if [ ! -f "$MARKDOWN_FILE" ]; then
        echo "Error: Input file '$MARKDOWN_FILE' not found."
        exit 1
    fi

    # Set default output file name if not specified
    if [ -z "$OUTPUT_FILE" ]; then
        FILENAME=$(basename "$MARKDOWN_FILE")
        FILENAME_NO_EXT="${FILENAME%.*}"
        OUTPUT_FILE="$OUTPUT_DIR/$FILENAME_NO_EXT.html"
    fi

    # Create output directory if it doesn't exist
    mkdir -p "$OUTPUT_DIR"
    
    # Set TOC and sidebar flags based on layout
    if [[ "$LAYOUT" == "toc" ]]; then
        TOC_SIDEBAR=true
    elif [[ "$LAYOUT" == "sidebar" ]]; then
        SIDEBAR=true
    fi
}

# Extract metadata from markdown file
extract_metadata() {
    # Title: Use first heading or filename
    TITLE=$(grep "^# " "$MARKDOWN_FILE" | head -n 1 | sed 's/^# //')
    if [ -z "$TITLE" ]; then
        FILENAME=$(basename "$MARKDOWN_FILE")
        TITLE="${FILENAME%.*}"
    fi
    
    DESCRIPTION=""
    
    # Check if file has YAML frontmatter
    if grep -q "^---" "$MARKDOWN_FILE"; then
        # Extract frontmatter
        FRONT_START=$(grep -n "^---" "$MARKDOWN_FILE" | head -n 1 | cut -d ":" -f 1)
        FRONT_END=$(grep -n "^---" "$MARKDOWN_FILE" | head -n 2 | tail -n 1 | cut -d ":" -f 1)
        
        if [ -n "$FRONT_START" ] && [ -n "$FRONT_END" ] && [ "$FRONT_END" -gt "$FRONT_START" ]; then
            FRONTMATTER=$(sed -n "$FRONT_START,$FRONT_END p" "$MARKDOWN_FILE")
            
            # Extract specific fields if they exist
            if echo "$FRONTMATTER" | grep -q "^title:"; then
                TITLE=$(echo "$FRONTMATTER" | grep "^title:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
            fi
            
            if echo "$FRONTMATTER" | grep -q "^description:"; then
                DESCRIPTION=$(echo "$FRONTMATTER" | grep "^description:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
            fi
            
            if echo "$FRONTMATTER" | grep -q "^theme:"; then
                THEME=$(echo "$FRONTMATTER" | grep "^theme:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
            fi
            
            if echo "$FRONTMATTER" | grep -q "^layout:"; then
                LAYOUT=$(echo "$FRONTMATTER" | grep "^layout:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
                
                # Update TOC and sidebar flags based on layout
                if [[ "$LAYOUT" == "toc" ]]; then
                    TOC_SIDEBAR=true
                elif [[ "$LAYOUT" == "sidebar" ]]; then
                    SIDEBAR=true
                fi
            fi
            
            if echo "$FRONTMATTER" | grep -q "^toc:"; then
                TOC_VALUE=$(echo "$FRONTMATTER" | grep "^toc:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
                if [ "$TOC_VALUE" = "false" ]; then
                    SHOW_TOC=false
                fi
            fi
            
            # Check for design language in frontmatter
            if echo "$FRONTMATTER" | grep -q "^design:"; then
                # Use frontmatter design if available, otherwise use command line parameter
                if [ -z "$DESIGN_LANGUAGE" ]; then
                    DESIGN_LANGUAGE=$(echo "$FRONTMATTER" | grep "^design:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
                    echo "Using design language from frontmatter: $DESIGN_LANGUAGE"
                fi
            fi
            
            # Check for social preview settings
            if echo "$FRONTMATTER" | grep -q "^social_preview:"; then
                SOCIAL_PREVIEW_VALUE=$(echo "$FRONTMATTER" | grep "^social_preview:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
                if [ "$SOCIAL_PREVIEW_VALUE" = "false" ]; then
                    SOCIAL_PREVIEW=false
                fi
            fi
            
            # Check for site URL
            if echo "$FRONTMATTER" | grep -q "^site_url:"; then
                # Use frontmatter site_url if available, otherwise use command line parameter
                if [ -z "$SITE_URL" ]; then
                    SITE_URL=$(echo "$FRONTMATTER" | grep "^site_url:" | cut -d ":" -f 2- | sed 's/^[ ]*//')
                fi
            fi
        fi
    fi
    
    # If design language was provided via command line, report it
    if [ -n "$DESIGN_LANGUAGE" ] && ! echo "$FRONTMATTER" | grep -q "^design:"; then
        echo "Using design language from command line: $DESIGN_LANGUAGE"
    fi
}

# Process next/prev page parameters
process_page_links() {
    # Process next page
    if [ -n "$NEXT_PAGE" ]; then
        NEXT_PAGE_URL=$(echo "$NEXT_PAGE" | cut -d ":" -f 1)
        NEXT_PAGE_TITLE=$(echo "$NEXT_PAGE" | cut -d ":" -f 2-)
        NEXT_PAGE_ENABLED=true
    else
        NEXT_PAGE_URL=""
        NEXT_PAGE_TITLE=""
        NEXT_PAGE_ENABLED=false
    fi
    
    # Process prev page
    if [ -n "$PREV_PAGE" ]; then
        PREV_PAGE_URL=$(echo "$PREV_PAGE" | cut -d ":" -f 1)
        PREV_PAGE_TITLE=$(echo "$PREV_PAGE" | cut -d ":" -f 2-)
        PREV_PAGE_ENABLED=true
    else
        PREV_PAGE_URL=""
        PREV_PAGE_TITLE=""
        PREV_PAGE_ENABLED=false
    fi
}

# Prepare styles and template
prepare_files() {
    # Copy the actual styles and template to temp directory
    cp "$STYLES_PATH" "$TEMP_STYLES"
    cp "$TEMPLATE_PATH" "$TEMP_TEMPLATE"
    
    # Read template file
    TEMPLATE_CONTENT=$(cat "$TEMP_TEMPLATE")
    
    # Replace variables in template
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{THEME\}\}/$THEME}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{TITLE\}\}/$TITLE}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{DESCRIPTION\}\}/$DESCRIPTION}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{LAYOUT\}\}/$LAYOUT}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{SITE_TITLE\}\}/$SITE_NAME}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{HOME_LINK\}\}/$HOME_LINK}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{AUTHOR\}\}/$AUTHOR}
    TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{GENERATION_DATE\}\}/$(date +"%Y-%m-%d")}
    
    # Generate social preview metadata placeholder if enabled
    if [ "$SOCIAL_PREVIEW" = true ]; then
        # Create basic social metadata
        SOCIAL_METADATA="<!-- Generated Social Media Preview Metadata -->\n"
        SOCIAL_METADATA+="<!-- Open Graph / Facebook -->\n"
        SOCIAL_METADATA+="<meta property=\"og:type\" content=\"website\">\n"
        
        # Add site URL if provided
        if [ -n "$SITE_URL" ]; then
            SOCIAL_METADATA+="<meta property=\"og:url\" content=\"$SITE_URL\">\n"
        fi
        
        # Add title and description
        SOCIAL_METADATA+="<meta property=\"og:title\" content=\"$TITLE\">\n"
        if [ -n "$DESCRIPTION" ]; then
            SOCIAL_METADATA+="<meta property=\"og:description\" content=\"$DESCRIPTION\">\n"
        fi
        
        # Add site name
        SOCIAL_METADATA+="<meta property=\"og:site_name\" content=\"$SITE_NAME\">\n"
        
        # Add Twitter card metadata
        SOCIAL_METADATA+="\n<!-- Twitter Card -->\n"
        SOCIAL_METADATA+="<meta name=\"twitter:card\" content=\"summary_large_image\">\n"
        SOCIAL_METADATA+="<meta name=\"twitter:title\" content=\"$TITLE\">\n"
        if [ -n "$DESCRIPTION" ]; then
            SOCIAL_METADATA+="<meta name=\"twitter:description\" content=\"$DESCRIPTION\">\n"
        fi
        
        # Replace the placeholder in the template
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{SOCIAL_PREVIEW_METADATA\}\}/$SOCIAL_METADATA}
    else
        # If social preview is disabled, remove the placeholder
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{SOCIAL_PREVIEW_METADATA\}\}/}
    fi
    
    # Handle conditional sections for TOC sidebar
    if [ "$TOC_SIDEBAR" = true ]; then
        # Keep TOC sections
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{#TOC_SIDEBAR\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{\/TOC_SIDEBAR\}\}/}
    else
        # Remove TOC sidebar sections
        TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed '/{{#TOC_SIDEBAR}}/,/{{\/TOC_SIDEBAR}}/d')
    fi
    
    # Handle conditional sections for SIDEBAR
    if [ "$SIDEBAR" = true ]; then
        # Keep sidebar sections
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{#SIDEBAR\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{\/SIDEBAR\}\}/}
    else
        # Remove sidebar sections
        TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed '/{{#SIDEBAR}}/,/{{\/SIDEBAR}}/d')
    fi
    
    # Handle conditional sections for SHOW_TOC
    if [ "$SHOW_TOC" = true ]; then
        # Keep TOC sections
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{#SHOW_TOC\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{\/SHOW_TOC\}\}/}
    else
        # Remove TOC sections
        TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed '/{{#SHOW_TOC}}/,/{{\/SHOW_TOC}}/d')
    fi
    
    # Handle conditional sections for NEXT_PAGE
    if [ "$NEXT_PAGE_ENABLED" = true ]; then
        # Keep next page sections
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{#NEXT_PAGE\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{\/NEXT_PAGE\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{NEXT_PAGE_URL\}\}/$NEXT_PAGE_URL}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{NEXT_PAGE_TITLE\}\}/$NEXT_PAGE_TITLE}
    else
        # Remove next page sections
        TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed '/{{#NEXT_PAGE}}/,/{{\/NEXT_PAGE}}/d')
    fi
    
    # Handle conditional sections for PREV_PAGE
    if [ "$PREV_PAGE_ENABLED" = true ]; then
        # Keep prev page sections
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{#PREV_PAGE\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{\/PREV_PAGE\}\}/}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{PREV_PAGE_URL\}\}/$PREV_PAGE_URL}
        TEMPLATE_CONTENT=${TEMPLATE_CONTENT//\{\{PREV_PAGE_TITLE\}\}/$PREV_PAGE_TITLE}
    else
        # Remove prev page sections
        TEMPLATE_CONTENT=$(echo "$TEMPLATE_CONTENT" | sed '/{{#PREV_PAGE}}/,/{{\/PREV_PAGE}}/d')
    fi
    
    # Write modified template back to file
    echo "$TEMPLATE_CONTENT" > "$TEMP_TEMPLATE"
}

# Convert markdown to HTML using pandoc
convert_markdown() {
    echo "Converting $MARKDOWN_FILE to HTML..."
    
    # Determine TOC option for pandoc
    if [ "$SHOW_TOC" = true ]; then
        TOC_OPTION="--toc"
    else
        TOC_OPTION=""
    fi
    
    # Process admonitions before passing to pandoc (optional, advanced)
    TEMP_MD="$TEMP_DIR/preprocessed.md"
    process_admonitions "$MARKDOWN_FILE" "$TEMP_MD"
    
    # Set standalone mode (or not) based on need
    STANDALONE="--standalone"
    
    # Run pandoc conversion
    pandoc -f markdown+smart+footnotes -t html "$TEMP_MD" \
        -o "$TEMP_HTML" \
        $STANDALONE \
        --template="$TEMP_TEMPLATE" \
        --css="markdown-styles-enhanced.css" \
        $TOC_OPTION \
        --highlight-style=pygments \
        --metadata title="$TITLE" \
        --metadata description="$DESCRIPTION" \
        --metadata date="$(date +"%Y-%m-%d")" \
        --metadata author="$AUTHOR"
    
    # Post-process the HTML if needed
    post_process_html "$TEMP_HTML" "$OUTPUT_FILE"
    
    # This is now handled in post_process_html
    
    echo "Conversion complete! Output saved to $OUTPUT_FILE"
}

# Pre-process markdown for admonitions, tabs, etc.
process_admonitions() {
    local input_file="$1"
    local output_file="$2"
    
    # Skip frontmatter if it exists
    local in_frontmatter=false
    local frontmatter=""
    local content=""
    
    # Read file line by line
    while IFS= read -r line; do
        # Handle frontmatter
        if [[ "$line" == "---" ]]; then
            if [ "$in_frontmatter" = false ]; then
                in_frontmatter=true
                frontmatter+="$line\n"
            else
                in_frontmatter=false
                
                # Add design language to frontmatter if it's not already there and was specified
                if [ -n "$DESIGN_LANGUAGE" ] && ! echo -e "$frontmatter" | grep -q "^design:"; then
                    frontmatter+="design: $DESIGN_LANGUAGE\n"
                fi
                
                frontmatter+="$line\n"
                continue
            fi
        elif [ "$in_frontmatter" = true ]; then
            frontmatter+="$line\n"
            continue
        fi
        
        # Process content (not in frontmatter)
        content+="$line\n"
    done < "$input_file"
    
    # If no frontmatter existed but design language is specified, add it
    if [ -n "$DESIGN_LANGUAGE" ] && [ "$in_frontmatter" = false ]; then
        echo "Adding design language to file without frontmatter"
        content="---\ndesign: $DESIGN_LANGUAGE\n---\n$content"
    else
        # Save processed content
        content="$frontmatter$content"
    fi
    
    # Save processed content
    echo -e "$content" > "$output_file"
}

# Post-process HTML for any final adjustments
post_process_html() {
    local input_file="$1"
    local output_file="$2"
    
    # Copy the file directly for now
    cp "$input_file" "$output_file"
    
    # Copy stylesheet to output directory if it doesn't exist
    local output_dir=$(dirname "$output_file")
    cp "$STYLES_PATH" "$output_dir/markdown-styles-enhanced.css"
}

# Clean up temporary files
cleanup() {
    rm -rf "$TEMP_DIR"
}

# Main function
main() {
    check_dependencies
    create_temp_file
    parse_args "$@"
    extract_metadata
    process_page_links
    prepare_files
    convert_markdown
    cleanup
}

# Run main function with all arguments
main "$@"