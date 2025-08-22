#!/bin/bash
# Script to directly implement port conflict fixes

# Terminal directory
TERMINAL_DIR="/Users/joshkornreich/Documents/Projects/Terminals/Claude"
PACKAGE_JSON="$TERMINAL_DIR/package.json"

if [ ! -d "$TERMINAL_DIR" ]; then
  echo "Error: Terminal directory not found at $TERMINAL_DIR"
  exit 1
fi

# Copy the launcher script
cp /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/terminal-launcher.sh "$TERMINAL_DIR/"
chmod +x "$TERMINAL_DIR/terminal-launcher.sh"
echo "✅ Installed terminal-launcher.sh in the project directory"

# Copy the auto-detect port script
cp /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/auto-detect-port.js "$TERMINAL_DIR/"
echo "✅ Installed auto-detect-port.js in the project directory"

# Update package.json to use dynamic port
if [ -f "$PACKAGE_JSON" ]; then
  # Create backup
  cp "$PACKAGE_JSON" "${PACKAGE_JSON}.bak"
  
  # Read the current file
  content=$(cat "$PACKAGE_JSON")
  
  # Update the dev script using temporary file to maintain formatting
  echo "$content" | 
    perl -pe 's/"dev": "vite"/"dev": "vite --port 28973 --strictPort false"/' > "${PACKAGE_JSON}.tmp"
  
  # Replace the original
  mv "${PACKAGE_JSON}.tmp" "$PACKAGE_JSON"
  
  echo "✅ Updated package.json to use dynamic port"
else
  echo "❌ Could not find package.json at $PACKAGE_JSON"
fi

# Create a symbolic link to the launcher for easy access
ln -sf "$TERMINAL_DIR/terminal-launcher.sh" /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/implemented-solutions/run-terminal.sh
chmod +x /Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/implemented-solutions/run-terminal.sh

echo "✅ Implementation complete - you can now run the terminal with:"
echo "/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/implemented-solutions/run-terminal.sh"
echo
echo "The permanent solution has been applied to your project files."