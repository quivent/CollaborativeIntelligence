#!/usr/bin/env python3
"""
Global Content Identification Script

This script scans agent sessions for content that should be distributed globally,
based on metadata and content analysis.

Usage:
  python3 identify_global_content.py [--dir DIR] [--threshold THRESHOLD] [--report-only]

Options:
  --dir DIR              Specify the root directory to scan (default: current directory)
  --threshold THRESHOLD  Minimum relevance score to identify content (default: 0.7)
  --report-only          Generate report without initiating distribution
"""

import os
import sys
import argparse
import re
import json
from datetime import datetime
from pathlib import Path

# Configuration
AGENTS_DIR = "AGENTS"
SESSIONS_DIR = "Sessions"
GLOBAL_DIRS = {
    "architecture": "Architectures",
    "plan": "Plans",
    "documentation": "Documentation", 
    "analysis": "Analyses",
    "protocol": "Protocols",
    "notification": "."  # Root directory for notifications
}
METADATA_FILE = "metadata.json"

# Key patterns indicating global relevance
GLOBAL_PATTERNS = {
    "architecture": [
        r"architecture", r"design", r"structure", r"component", r"pattern",
        r"system", r"framework", r"blueprint"
    ],
    "plan": [
        r"implementation", r"plan", r"strategy", r"approach", r"roadmap",
        r"initiative", r"project", r"timeline"
    ],
    "documentation": [
        r"guide", r"documentation", r"manual", r"reference", r"instruction",
        r"tutorial", r"handbook", r"specification"
    ],
    "analysis": [
        r"analysis", r"benchmark", r"comparison", r"evaluation", r"assessment",
        r"research", r"study", r"measurement"
    ],
    "protocol": [
        r"protocol", r"standard", r"procedure", r"process", r"guideline",
        r"convention", r"specification", r"workflow"
    ],
    "notification": [
        r"notification", r"announcement", r"alert", r"update", r"bulletin",
        r"advisory", r"broadcast", r"message"
    ]
}

def get_metadata_relevance(metadata_path):
    """Extract global relevance from metadata.json"""
    try:
        with open(metadata_path, 'r', encoding='utf-8') as file:
            metadata = json.load(file)
            
            # Check explicit global_relevance field
            if 'global_relevance' in metadata:
                relevance = metadata['global_relevance']
                if relevance in ['critical', 'high']:
                    return 1.0
                elif relevance == 'moderate':
                    return 0.7
                elif relevance == 'low':
                    return 0.3
                else:
                    return 0.1
                    
            # Check other indicators
            score = 0.0
            
            # Check priority
            if metadata.get('priority') == 'high':
                score += 0.2
                
            # Check status (completed items more relevant)
            if metadata.get('status') == 'completed':
                score += 0.1
                
            # Check innovation level
            if metadata.get('innovation_level') in ['high', 'significant']:
                score += 0.2
                
            return min(score, 1.0)  # Cap at 1.0
                
    except Exception as e:
        print(f"Error reading metadata from {metadata_path}: {e}")
        return 0.0

def analyze_content_relevance(file_path, content_type):
    """Analyze content for patterns indicating global relevance"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            content = file.read()
            
            # Start with base score
            score = 0.1
            
            # Check for specific patterns based on content type
            if content_type in GLOBAL_PATTERNS:
                patterns = GLOBAL_PATTERNS[content_type]
                for pattern in patterns:
                    if re.search(pattern, content, re.IGNORECASE):
                        score += 0.1
                        # Cap at 0.9 for pattern matches
                        if score >= 0.9:
                            break
            
            # Check for explicit indicators
            if re.search(r'global|system-wide|all agents', content, re.IGNORECASE):
                score += 0.2
                
            # Check for implementation completeness indicators
            if re.search(r'implementation complete|fully implemented|finalized', content, re.IGNORECASE):
                score += 0.1
                
            # Check for architecture diagrams
            if '```' in content and re.search(r'diagram|architecture|flow', content, re.IGNORECASE):
                score += 0.2
                
            return min(score, 1.0)  # Cap at 1.0
                
    except Exception as e:
        print(f"Error analyzing content in {file_path}: {e}")
        return 0.0

def determine_content_type(file_path, file_name):
    """Determine the type of content based on filename and path"""
    # Check filename for type indicators
    for content_type, patterns in GLOBAL_PATTERNS.items():
        for pattern in patterns:
            if re.search(pattern, file_name, re.IGNORECASE):
                return content_type
    
    # Check path components
    path_str = str(file_path)
    for content_type, patterns in GLOBAL_PATTERNS.items():
        for pattern in patterns:
            if re.search(pattern, path_str, re.IGNORECASE):
                return content_type
    
    # Default to documentation
    return "documentation"

def determine_target_directory(content_type, file_name):
    """Determine the appropriate global directory based on content type"""
    base_dir = GLOBAL_DIRS.get(content_type, "Documentation")
    
    # For architecture files, potentially create subdirectories
    if content_type == "architecture":
        # Extract a name from the file for a potential subdirectory
        name_match = re.search(r'([A-Za-z]+)_ARCHITECTURE', file_name, re.IGNORECASE)
        if name_match:
            return os.path.join(base_dir, name_match.group(1))
    
    return base_dir

def scan_agent_sessions(root_dir, threshold=0.7, report_only=True):
    """Scan agent sessions for content that should be distributed globally"""
    agents_dir = os.path.join(root_dir, AGENTS_DIR)
    global_content = []
    
    if not os.path.exists(agents_dir):
        print(f"Agents directory not found: {agents_dir}")
        return global_content
    
    # Iterate through all agent directories
    for agent_name in os.listdir(agents_dir):
        agent_dir = os.path.join(agents_dir, agent_name)
        if not os.path.isdir(agent_dir):
            continue
            
        # Check for Sessions directory
        sessions_dir = os.path.join(agent_dir, SESSIONS_DIR)
        if not os.path.exists(sessions_dir):
            continue
            
        # Iterate through all session directories
        for session_name in os.listdir(sessions_dir):
            session_dir = os.path.join(sessions_dir, session_name)
            if not os.path.isdir(session_dir):
                continue
                
            # Check for metadata.json
            metadata_path = os.path.join(session_dir, METADATA_FILE)
            metadata_relevance = get_metadata_relevance(metadata_path) if os.path.exists(metadata_path) else 0.0
            
            # Iterate through markdown files in the session
            for file_name in os.listdir(session_dir):
                if not file_name.endswith('.md') or file_name == 'README.md':
                    continue
                    
                file_path = os.path.join(session_dir, file_name)
                
                # Determine content type and relevance
                content_type = determine_content_type(file_path, file_name)
                content_relevance = analyze_content_relevance(file_path, content_type)
                
                # Calculate combined relevance score
                # Weight metadata more heavily if it exists
                combined_score = metadata_relevance * 0.7 + content_relevance * 0.3 if os.path.exists(metadata_path) else content_relevance
                
                # If relevance exceeds threshold, add to global content list
                if combined_score >= threshold:
                    target_dir = determine_target_directory(content_type, file_name)
                    global_content.append({
                        "agent": agent_name,
                        "session": session_name,
                        "file": file_name,
                        "path": file_path,
                        "content_type": content_type,
                        "relevance_score": combined_score,
                        "target_directory": target_dir,
                        "metadata_exists": os.path.exists(metadata_path)
                    })
    
    # Sort by relevance score (highest first)
    global_content.sort(key=lambda x: x["relevance_score"], reverse=True)
    
    # Generate report
    report_path = os.path.join(root_dir, "KnowledgeRegistry", "identification_report.md")
    generate_report(global_content, report_path)
    
    return global_content

def generate_report(global_content, report_path):
    """Generate a report of identified global content"""
    os.makedirs(os.path.dirname(report_path), exist_ok=True)
    
    with open(report_path, 'w', encoding='utf-8') as report:
        report.write("# Global Content Identification Report\n\n")
        report.write(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        report.write(f"Identified {len(global_content)} documents that should be distributed globally.\n\n")
        
        report.write("## High Priority Content (Score ≥ 0.8)\n\n")
        report.write("| Agent | Session | File | Type | Score | Target Directory |\n")
        report.write("|-------|---------|------|------|-------|------------------|\n")
        
        for content in global_content:
            if content["relevance_score"] >= 0.8:
                report.write(f"| {content['agent']} | {content['session']} | {content['file']} | ")
                report.write(f"{content['content_type']} | {content['relevance_score']:.2f} | {content['target_directory']} |\n")
        
        report.write("\n## Medium Priority Content (0.7 ≤ Score < 0.8)\n\n")
        report.write("| Agent | Session | File | Type | Score | Target Directory |\n")
        report.write("|-------|---------|------|------|-------|------------------|\n")
        
        for content in global_content:
            if 0.7 <= content["relevance_score"] < 0.8:
                report.write(f"| {content['agent']} | {content['session']} | {content['file']} | ")
                report.write(f"{content['content_type']} | {content['relevance_score']:.2f} | {content['target_directory']} |\n")
                
        report.write("\n## Implementation Next Steps\n\n")
        report.write("1. Review identified content for distribution accuracy\n")
        report.write("2. Transform content to meet global standards if needed\n")
        report.write("3. Distribute content to appropriate target directories\n")
        report.write("4. Update Knowledge Registry with new entries\n")
        report.write("5. Add distribution record to distribution_log.md\n")

def main():
    parser = argparse.ArgumentParser(description="Identify globally relevant content in agent sessions")
    parser.add_argument("--dir", default=".", help="Root directory to scan")
    parser.add_argument("--threshold", type=float, default=0.7, help="Minimum relevance score (0.0-1.0)")
    parser.add_argument("--report-only", action="store_true", help="Generate report without initiating distribution")
    args = parser.parse_args()
    
    root_dir = args.dir
    
    print(f"Scanning for globally relevant content in {root_dir}...")
    global_content = scan_agent_sessions(root_dir, args.threshold, args.report_only)
    
    print(f"Found {len(global_content)} documents that should be distributed globally")
    print(f"Report generated at {os.path.join(root_dir, 'KnowledgeRegistry', 'identification_report.md')}")
    
    # Future: Add code to initiate distribution if report_only is False

if __name__ == "__main__":
    main()