#!/bin/bash
# terminal-launcher.sh - Launcher script for Claude Terminal with port conflict resolution
# Install this script in your project root directory

# Configuration
TERMINAL_DIR="/Users/joshkornreich/Documents/Projects/Terminals/Claude"
DEFAULT_PORT=28973
FALLBACK_PORTS=(29973 30973 31973 32973)
CONFIG_FILE="$TERMINAL_DIR/src-tauri/tauri.conf.json"
PACKAGE_JSON="$TERMINAL_DIR/package.json"

# Function to print colored text
print_colored() {
  local color=$1
  local text=$2
  
  case $color in
    "green") echo -e "\033[0;32m$text\033[0m" ;;
    "yellow") echo -e "\033[0;33m$text\033[0m" ;;
    "red") echo -e "\033[0;31m$text\033[0m" ;;
    "blue") echo -e "\033[0;34m$text\033[0m" ;;
    *) echo "$text" ;;
  esac
}

# Check directory exists
if [ ! -d "$TERMINAL_DIR" ]; then
  print_colored "red" "Error: Terminal directory not found at $TERMINAL_DIR"
  exit 1
fi

print_colored "blue" "Starting Claude Terminal launcher..."

# Function to check if a port is available
is_port_available() {
  local port=$1
  if lsof -i:"$port" > /dev/null; then
    return 1 # Port is in use
  else
    return 0 # Port is available
  fi
}

# Function to update tauri.conf.json with new port
update_tauri_config() {
  local port=$1
  
  # Make backup of original file
  if [ ! -f "${CONFIG_FILE}.bak" ]; then
    cp "$CONFIG_FILE" "${CONFIG_FILE}.bak"
    print_colored "yellow" "Created backup of original config at ${CONFIG_FILE}.bak"
  fi
  
  # Update config file with new port using sed
  # This handles JSON properly unlike basic string replacement
  sed -i.tmp "s|\"devPath\": \"http://localhost:[0-9]*\"|\"devPath\": \"http://localhost:$port\"|g" "$CONFIG_FILE"
  rm -f "${CONFIG_FILE}.tmp"
  
  print_colored "green" "Updated tauri.conf.json with new port: $port"
}

# Function to update package.json with new port
update_package_json() {
  local port=$1
  
  # Make backup of original file
  if [ ! -f "${PACKAGE_JSON}.bak" ]; then
    cp "$PACKAGE_JSON" "${PACKAGE_JSON}.bak"
    print_colored "yellow" "Created backup of original package.json at ${PACKAGE_JSON}.bak"
  fi
  
  # Check if port is already in package.json
  if grep -q "\"dev\": \"vite --port" "$PACKAGE_JSON"; then
    # Update existing port
    sed -i.tmp "s|\"dev\": \"vite --port [0-9]* --strictPort [a-z]*\"|\"dev\": \"vite --port $port --strictPort false\"|g" "$PACKAGE_JSON"
  else
    # Add port configuration
    sed -i.tmp "s|\"dev\": \"vite\"|\"dev\": \"vite --port $port --strictPort false\"|g" "$PACKAGE_JSON"
  fi
  rm -f "${PACKAGE_JSON}.tmp"
  
  print_colored "green" "Updated package.json dev script with port: $port"
}

# Function to kill process on port
kill_process_on_port() {
  local port=$1
  local pid=$(lsof -t -i:"$port")
  
  if [ -n "$pid" ]; then
    print_colored "yellow" "Killing process $pid using port $port..."
    kill "$pid" 2>/dev/null
    sleep 1
    
    # Check if process was killed
    if lsof -i:"$port" > /dev/null; then
      print_colored "red" "Failed to kill process on port $port. Try manual termination."
      return 1
    else
      print_colored "green" "Successfully killed process on port $port"
      return 0
    fi
  fi
  
  return 0 # No process to kill
}

# Main port selection logic
selected_port=$DEFAULT_PORT

# Check if default port is available
if is_port_available "$DEFAULT_PORT"; then
  print_colored "green" "Default port $DEFAULT_PORT is available"
else
  print_colored "yellow" "Default port $DEFAULT_PORT is in use"
  
  # Ask user what to do
  print_colored "blue" "Choose an option:"
  print_colored "blue" "1) Kill process on port $DEFAULT_PORT and use it"
  print_colored "blue" "2) Use an alternative port"
  print_colored "blue" "3) Exit"
  
  read -p "Enter option (1-3): " option
  
  case $option in
    1)
      # Try to kill process
      if kill_process_on_port "$DEFAULT_PORT"; then
        selected_port=$DEFAULT_PORT
      else
        print_colored "red" "Couldn't kill process. Please terminate it manually."
        exit 1
      fi
      ;;
    2)
      # Try alternative ports
      for port in "${FALLBACK_PORTS[@]}"; do
        if is_port_available "$port"; then
          selected_port=$port
          print_colored "green" "Using alternative port: $port"
          break
        fi
      done
      
      if [ "$selected_port" = "$DEFAULT_PORT" ]; then
        print_colored "red" "No available ports found in the fallback list"
        print_colored "blue" "Please specify a custom port:"
        read -p "Enter port number: " custom_port
        
        if is_port_available "$custom_port"; then
          selected_port=$custom_port
        else
          print_colored "red" "Port $custom_port is also in use. Please try again later."
          exit 1
        fi
      fi
      ;;
    3)
      print_colored "blue" "Exiting launcher"
      exit 0
      ;;
    *)
      print_colored "red" "Invalid option"
      exit 1
      ;;
  esac
fi

# Update configuration files if not using default port
if [ "$selected_port" != "$DEFAULT_PORT" ]; then
  update_tauri_config "$selected_port"
  update_package_json "$selected_port"
  print_colored "yellow" "Configuration updated to use port $selected_port"
else
  print_colored "green" "Using default port: $DEFAULT_PORT"
fi

# Change to project directory
print_colored "blue" "Changing to project directory: $TERMINAL_DIR"
cd "$TERMINAL_DIR" || {
  print_colored "red" "Failed to change to project directory: $TERMINAL_DIR"
  exit 1
}

# Start the application
print_colored "blue" "Starting Claude Terminal on port $selected_port..."
print_colored "green" "Running: npm run tauri dev"

# Start the application
npm run tauri dev

# Restore original config files when exiting
trap 'if [ -f "${CONFIG_FILE}.bak" ]; then mv "${CONFIG_FILE}.bak" "$CONFIG_FILE"; fi; if [ -f "${PACKAGE_JSON}.bak" ]; then mv "${PACKAGE_JSON}.bak" "$PACKAGE_JSON"; fi' EXIT