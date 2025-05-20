#!/bin/bash
# update-terminal-title.sh - Simple terminal title updater
#
# Usage: ./update-terminal-title.sh "Agent Name" ["Task Description"]

AGENT_NAME="$1"
TASK="${2:-Active Session}"
DIRECTORY="$(basename "$(pwd)")"

# Update terminal title using ANSI escape sequence
echo -ne "\033]0;[$DIRECTORY] | [$AGENT_NAME] | [$TASK]\007"

echo "Terminal title updated to: [$DIRECTORY] | [$AGENT_NAME] | [$TASK]"
