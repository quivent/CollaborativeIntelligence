#!/bin/bash
# Knowledge Registry Search CLI
# A user-friendly wrapper around the search_knowledge.py script

REGISTRY_DIR="$(dirname "$(readlink -f "$0")")"
ROOT_DIR="$(dirname "$REGISTRY_DIR")"
SEARCH_SCRIPT="$ROOT_DIR/scripts/knowledge/search_knowledge.py"

# Show banner
echo "==================================="
echo "  Knowledge Registry Search Tool"
echo "==================================="
echo ""

# Check if search script exists
if [ ! -f "$SEARCH_SCRIPT" ]; then
    echo "Error: Search script not found at $SEARCH_SCRIPT"
    exit 1
fi

# Function to show help
show_help() {
    echo "Usage: search_cli.sh [options] [query]"
    echo ""
    echo "Options:"
    echo "  -c, --category CATEGORY  Search by category"
    echo "  -t, --tag TAG           Search by tag/keyword"
    echo "  -r, --recent [N]        Show N most recent additions (default: 5)"
    echo "  -f, --full              Show full document details"
    echo "  -j, --json              Output in JSON format"
    echo "  -h, --help              Show this help message"
    echo ""
    echo "Examples:"
    echo "  search_cli.sh architecture           Search for 'architecture'"
    echo "  search_cli.sh -c 'Protocols'         List all protocols"
    echo "  search_cli.sh -t optimization        Find documents about optimization"
    echo "  search_cli.sh -r                     Show recent additions"
    echo "  search_cli.sh -f 'Memory System'     Detailed info about Memory System"
    echo ""
}

# Parse arguments
QUERY=""
CATEGORY=""
TAG=""
RECENT=""
FULL=""
JSON=""

while [[ $# -gt 0 ]]; do
    key="$1"
    case $key in
        -c|--category)
            CATEGORY="$2"
            shift 2
            ;;
        -t|--tag)
            TAG="$2"
            shift 2
            ;;
        -r|--recent)
            if [[ $2 =~ ^[0-9]+$ ]]; then
                RECENT="$2"
                shift 2
            else
                RECENT="5"
                shift
            fi
            ;;
        -f|--full)
            FULL="--full"
            shift
            ;;
        -j|--json)
            JSON="--json"
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            QUERY="$1"
            shift
            ;;
    esac
done

# Build command
CMD="$SEARCH_SCRIPT"
if [ ! -z "$QUERY" ]; then
    CMD="$CMD \"$QUERY\""
fi
if [ ! -z "$CATEGORY" ]; then
    CMD="$CMD --category \"$CATEGORY\""
fi
if [ ! -z "$TAG" ]; then
    CMD="$CMD --tag \"$TAG\""
fi
if [ ! -z "$RECENT" ]; then
    CMD="$CMD --recent $RECENT"
fi
if [ ! -z "$FULL" ]; then
    CMD="$CMD $FULL"
fi
if [ ! -z "$JSON" ]; then
    CMD="$CMD $JSON"
fi
CMD="$CMD --dir \"$ROOT_DIR\""

# If no arguments provided, show help
if [ -z "$QUERY" ] && [ -z "$CATEGORY" ] && [ -z "$TAG" ] && [ -z "$RECENT" ]; then
    show_help
    exit 0
fi

# Execute command
echo "Searching knowledge registry..."
echo ""
eval $CMD

# Show footer
echo ""
echo "Search complete. For more options, run: search_cli.sh --help"
echo "==================================="