#!/usr/bin/env bash
# output.sh - Standardized output functions for the CI system
# Part of the Collaborative Intelligence Refactoring Plan

# Strict mode
set -euo pipefail

# Color codes
if [[ -t 1 ]] && [[ -t 2 ]]; then
  # Use colors only when stdout and stderr are terminals
  readonly RED='\033[0;31m'
  readonly GREEN='\033[0;32m'
  readonly YELLOW='\033[0;33m'
  readonly BLUE='\033[0;34m'
  readonly MAGENTA='\033[0;35m'
  readonly CYAN='\033[0;36m'
  readonly BOLD='\033[1m'
  readonly RESET='\033[0m'
else
  # No colors for non-interactive sessions
  readonly RED=''
  readonly GREEN=''
  readonly YELLOW=''
  readonly BLUE=''
  readonly MAGENTA=''
  readonly CYAN=''
  readonly BOLD=''
  readonly RESET=''
fi

# Output formatted text
# Usage: print_formatted <color> <prefix> <message>
print_formatted() {
  local color="$1"
  local prefix="$2"
  local message="$3"
  
  echo -e "${color}${prefix}${RESET} ${message}"
}

# Print success message
# Usage: print_success <message>
print_success() {
  print_formatted "${GREEN}${BOLD}" "[SUCCESS]" "$1"
}

# Print error message
# Usage: print_error <message>
print_error() {
  print_formatted "${RED}${BOLD}" "[ERROR]" "$1" >&2
}

# Print warning message
# Usage: print_warning <message>
print_warning() {
  print_formatted "${YELLOW}${BOLD}" "[WARNING]" "$1" >&2
}

# Print info message
# Usage: print_info <message>
print_info() {
  print_formatted "${BLUE}" "[INFO]" "$1"
}

# Print debug message
# Usage: print_debug <message>
print_debug() {
  if [[ "${CI_DEBUG:-false}" == "true" ]]; then
    print_formatted "${MAGENTA}" "[DEBUG]" "$1" >&2
  fi
}

# Print step header
# Usage: print_step <step_number> <total_steps> <step_description>
print_step() {
  local step_num="$1"
  local total_steps="$2"
  local description="$3"
  
  echo -e "\n${CYAN}${BOLD}[STEP ${step_num}/${total_steps}]${RESET} ${description}"
  echo -e "${CYAN}$(printf '=%.0s' $(seq 1 60))${RESET}"
}

# Print section header
# Usage: print_section <section_title>
print_section() {
  local title="$1"
  
  echo -e "\n${BOLD}${title}${RESET}"
  echo -e "$(printf '%s' "$(printf -- '-%.0s' $(seq 1 ${#title}))")"
}

# Progress indicator
# Start progress indicator
# Usage: start_progress <message>
start_progress() {
  local message="$1"
  
  # Only show in interactive sessions
  if [[ -t 1 ]]; then
    echo -ne "${BLUE}${message}${RESET} "
    
    # Store the message for later use
    export CI_PROGRESS_MESSAGE="$message"
    
    # Start spinning animation in background
    (
      i=0
      spin='-\|/'
      while true; do
        i=$(( (i+1) % 4 ))
        echo -ne "\b${spin:$i:1}"
        sleep 0.1
      done
    ) &
    
    # Store the PID of the animation process
    export CI_PROGRESS_PID=$!
  fi
}

# Stop progress indicator and show result
# Usage: stop_progress <success|failure|warning>
stop_progress() {
  local result="$1"
  
  # Only proceed if we have an active progress indicator
  if [[ -n "${CI_PROGRESS_PID:-}" ]]; then
    # Kill the animation process
    kill $CI_PROGRESS_PID 2>/dev/null || true
    wait $CI_PROGRESS_PID 2>/dev/null || true
    unset CI_PROGRESS_PID
    
    # Clear the spinning character
    echo -ne "\b"
    
    # Print the appropriate result indicator
    case "$result" in
      success)
        echo -e "${GREEN}✓${RESET}"
        ;;
      failure)
        echo -e "${RED}✗${RESET}"
        ;;
      warning)
        echo -e "${YELLOW}!${RESET}"
        ;;
      *)
        echo -e "${BLUE}•${RESET}"
        ;;
    esac
    
    # Unset the stored message
    unset CI_PROGRESS_MESSAGE
  fi
}

# Print a divider line
# Usage: print_divider
print_divider() {
  echo -e "${BLUE}$(printf '─%.0s' $(seq 1 60))${RESET}"
}

# Print a box around text
# Usage: print_box <message>
print_box() {
  local message="$1"
  local width=$(( ${#message} + 4 ))
  
  echo -e "${BLUE}┌$(printf '─%.0s' $(seq 1 $width))┐${RESET}"
  echo -e "${BLUE}│  ${RESET}${BOLD}${message}${RESET}${BLUE}  │${RESET}"
  echo -e "${BLUE}└$(printf '─%.0s' $(seq 1 $width))┘${RESET}"
}

# Print a list item with optional status
# Usage: print_list_item <message> [status]
print_list_item() {
  local message="$1"
  local status="${2:-}"
  
  echo -ne "${BLUE}• ${RESET}${message}"
  
  if [[ -n "$status" ]]; then
    case "$status" in
      success)
        echo -e " ${GREEN}[OK]${RESET}"
        ;;
      failure)
        echo -e " ${RED}[FAILED]${RESET}"
        ;;
      warning)
        echo -e " ${YELLOW}[WARNING]${RESET}"
        ;;
      skipped)
        echo -e " ${MAGENTA}[SKIPPED]${RESET}"
        ;;
      *)
        echo -e " ${CYAN}[${status}]${RESET}"
        ;;
    esac
  else
    echo
  fi
}