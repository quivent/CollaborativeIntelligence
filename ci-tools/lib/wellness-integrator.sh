#!/bin/bash

# ============================================================================
# Wellness Agent CI Tools Integration
# 
# This script integrates the Wellness agent's trigger system with the
# Collaborative Intelligence CI tools.
# ============================================================================

# Configuration
CI_DIR="$(dirname "$(dirname "$0")")"
WELLNESS_DIR="/Users/joshkornreich/Documents/Projects/CollaborativeIntelligence/AGENTS/Wellness"
TOOLKIT_DIR="$WELLNESS_DIR/Toolkit"
SERVICE_SCRIPT="$TOOLKIT_DIR/wellness_service.sh"

# Include common functions from CI tools
if [[ -f "$CI_DIR/lib/common.sh" ]]; then
    source "$CI_DIR/lib/common.sh"
else
    echo "ERROR: Failed to load common.sh from CI tools"
    exit 1
fi

# Initialize the Wellness agent trigger system
initialize_wellness_triggers() {
    echo_info "Initializing Wellness agent trigger system"
    
    # Check if the wellness service script exists
    if [[ ! -f "$SERVICE_SCRIPT" ]]; then
        echo_error "Wellness service script not found: $SERVICE_SCRIPT"
        return 1
    fi
    
    # Start the wellness service
    "$SERVICE_SCRIPT" start
    
    if [[ $? -eq 0 ]]; then
        echo_success "Wellness trigger system initialized successfully"
    else
        echo_error "Failed to initialize Wellness trigger system"
        return 1
    fi
    
    return 0
}

# Stop the Wellness agent trigger system
stop_wellness_triggers() {
    echo_info "Stopping Wellness agent trigger system"
    
    # Check if the wellness service script exists
    if [[ ! -f "$SERVICE_SCRIPT" ]]; then
        echo_error "Wellness service script not found: $SERVICE_SCRIPT"
        return 1
    fi
    
    # Stop the wellness service
    "$SERVICE_SCRIPT" stop
    
    if [[ $? -eq 0 ]]; then
        echo_success "Wellness trigger system stopped successfully"
    else
        echo_error "Failed to stop Wellness trigger system"
        return 1
    fi
    
    return 0
}

# Check the status of the Wellness agent trigger system
check_wellness_status() {
    echo_info "Checking Wellness agent trigger system status"
    
    # Check if the wellness service script exists
    if [[ ! -f "$SERVICE_SCRIPT" ]]; then
        echo_error "Wellness service script not found: $SERVICE_SCRIPT"
        return 1
    fi
    
    # Check the status of the wellness service
    "$SERVICE_SCRIPT" status
    
    return $?
}

# Install the Wellness agent trigger system as a LaunchAgent
install_wellness_service() {
    echo_info "Installing Wellness agent trigger system as a LaunchAgent"
    
    # Check if the wellness service script exists
    if [[ ! -f "$SERVICE_SCRIPT" ]]; then
        echo_error "Wellness service script not found: $SERVICE_SCRIPT"
        return 1
    fi
    
    # Set up the LaunchAgent
    "$SERVICE_SCRIPT" setup
    
    if [[ $? -eq 0 ]]; then
        echo_success "Wellness service installed successfully"
        
        # Load the LaunchAgent immediately
        local plist_file="$HOME/Library/LaunchAgents/com.collaborativeintelligence.wellness.plist"
        if [[ -f "$plist_file" ]]; then
            launchctl load "$plist_file"
            if [[ $? -eq 0 ]]; then
                echo_success "Wellness service loaded"
            else
                echo_warning "Failed to load Wellness service. You may need to restart or manually run: launchctl load $plist_file"
            fi
        else
            echo_error "LaunchAgent plist file not found: $plist_file"
            return 1
        fi
    else
        echo_error "Failed to install Wellness service"
        return 1
    fi
    
    return 0
}

# Function to hook into the CI agent startup process
hook_agent_startup() {
    echo_info "Hooking Wellness trigger system into CI agent startup"
    
    # If we're starting a session and the Wellness service isn't running, start it
    check_wellness_status > /dev/null 2>&1
    if [[ $? -ne 0 ]]; then
        echo_info "Wellness service not running, starting it now"
        initialize_wellness_triggers
    fi
    
    return 0
}

# Main function
main() {
    # Process command line arguments
    case "$1" in
        start)
            initialize_wellness_triggers
            ;;
        stop)
            stop_wellness_triggers
            ;;
        status)
            check_wellness_status
            ;;
        install)
            install_wellness_service
            ;;
        hook)
            hook_agent_startup
            ;;
        *)
            echo "Usage: $0 {start|stop|status|install|hook}"
            echo
            echo "Commands:"
            echo "  start   - Start the Wellness trigger system"
            echo "  stop    - Stop the Wellness trigger system"
            echo "  status  - Check the status of the Wellness trigger system"
            echo "  install - Install the Wellness service as a LaunchAgent"
            echo "  hook    - Hook into CI agent startup process"
            exit 1
            ;;
    esac
    
    exit $?
}

# Run the main function with all arguments
main "$@"