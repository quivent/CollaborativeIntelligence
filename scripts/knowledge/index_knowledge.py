#!/usr/bin/env python3
"""
Knowledge Indexing Script

This script scans the repository for global knowledge artifacts and updates
the Knowledge Registry with their metadata.

Usage:
  python3 index_knowledge.py [--dir DIR] [--update]

Options:
  --dir DIR    Specify the root directory to scan (default: current directory)
  --update     Update existing index entries instead of just adding new ones
"""

import os
import sys
import argparse
import re
import json
from datetime import datetime
from pathlib import Path

# Configuration
REGISTRY_DIR = "KnowledgeRegistry"
GLOBAL_DIRS = [
    "Plans",
    "Documentation",
    "Protocols",
    "Analyses",
    "Architectures",
    "OPTIMIZATIONS"
]
EXCLUDED_DIRS = [
    ".git",
    "node_modules",
    "__pycache__"
]
MARKDOWN_EXTENSIONS = [".md", ".markdown"]

# Knowledge categories and their patterns
CATEGORIES = {
    "Architectural Documents": [
        r"architecture", r"design", r"structure", r"component", r"pattern"
    ],
    "Implementation Plans": [
        r"implementation", r"plan", r"strategy", r"approach", r"roadmap"
    ],
    "Documentation": [
        r"guide", r"documentation", r"manual", r"reference", r"instruction"
    ],
    "Analyses": [
        r"analysis", r"benchmark", r"comparison", r"evaluation", r"assessment"
    ],
    "Protocols": [
        r"protocol", r"standard", r"procedure", r"process", r"guideline"
    ],
    "Notifications": [
        r"notification", r"announcement", r"alert", r"update", r"bulletin"
    ]
}

def get_file_modification_date(file_path):
    """Get the last modification date of a file in YYYY-MM-DD format"""
    timestamp = os.path.getmtime(file_path)
    return datetime.fromtimestamp(timestamp).strftime("%Y-%m-%d")

def extract_title(file_path):
    """Extract the title from a markdown file (first # heading)"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            content = file.read()
            match = re.search(r'^#\s+(.+)$', content, re.MULTILINE)
            if match:
                return match.group(1)
            # If no heading found, use the filename
            return os.path.basename(file_path)
    except Exception as e:
        print(f"Error extracting title from {file_path}: {e}")
        return os.path.basename(file_path)

def extract_description(file_path):
    """Extract a brief description from a markdown file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            content = file.read()
            
            # Try to find the first paragraph after the title
            match = re.search(r'^#\s+.+\n\n(.+?)(\n\n|$)', content, re.MULTILINE)
            if match:
                # Limit to 100 characters and add ellipsis if longer
                desc = match.group(1).strip()
                return desc[:100] + ('...' if len(desc) > 100 else '')
            
            # If no clear paragraph, take the first 100 non-empty chars
            lines = [line.strip() for line in content.split('\n') if line.strip()]
            for line in lines:
                if not line.startswith('#') and line:
                    return line[:100] + ('...' if len(line) > 100 else '')
            
            # Default to empty description
            return "No description available"
    except Exception as e:
        print(f"Error extracting description from {file_path}: {e}")
        return "Error extracting description"

def determine_category(file_path, content=None):
    """Determine the most likely category for a file based on its name and content"""
    if content is None:
        try:
            with open(file_path, 'r', encoding='utf-8') as file:
                content = file.read()
        except Exception:
            content = ""
    
    filename = os.path.basename(file_path)
    scores = {category: 0 for category in CATEGORIES}
    
    # Check filename for category patterns
    for category, patterns in CATEGORIES.items():
        for pattern in patterns:
            if re.search(pattern, filename, re.IGNORECASE):
                scores[category] += 2
    
    # Check content for category patterns (with lower weight)
    for category, patterns in CATEGORIES.items():
        for pattern in patterns:
            matches = re.findall(pattern, content, re.IGNORECASE)
            scores[category] += len(matches) * 0.1
    
    # Check directory name for additional context
    dir_name = os.path.basename(os.path.dirname(file_path))
    for category, patterns in CATEGORIES.items():
        for pattern in patterns:
            if re.search(pattern, dir_name, re.IGNORECASE):
                scores[category] += 1
    
    # Find the category with the highest score
    best_category = max(scores.items(), key=lambda x: x[1])
    
    # If the score is too low, default to Documentation
    if best_category[1] == 0:
        return "Documentation"
    
    return best_category[0]

def find_knowledge_files(root_dir):
    """Find all potential knowledge files in the repository"""
    knowledge_files = []
    
    for global_dir in GLOBAL_DIRS:
        dir_path = os.path.join(root_dir, global_dir)
        if not os.path.exists(dir_path):
            continue
        
        for root, dirs, files in os.walk(dir_path):
            # Skip excluded directories
            dirs[:] = [d for d in dirs if d not in EXCLUDED_DIRS]
            
            for file in files:
                if any(file.endswith(ext) for ext in MARKDOWN_EXTENSIONS):
                    file_path = os.path.join(root, file)
                    rel_path = os.path.relpath(file_path, root_dir)
                    
                    knowledge_files.append({
                        "path": file_path,
                        "relative_path": rel_path,
                        "title": extract_title(file_path),
                        "last_updated": get_file_modification_date(file_path),
                        "description": extract_description(file_path),
                        "category": determine_category(file_path)
                    })
    
    return knowledge_files

def update_master_index(knowledge_files, registry_dir, update_existing=False):
    """Update the master_index.md file with the found knowledge files"""
    master_index_path = os.path.join(registry_dir, "master_index.md")
    
    try:
        with open(master_index_path, 'r', encoding='utf-8') as file:
            current_content = file.read()
    except FileNotFoundError:
        # Create a new master index if it doesn't exist
        current_content = """# Knowledge Registry Master Index

## Overview

This document serves as the central catalog of all global knowledge within the Collaborative Intelligence system. It provides a structured reference to help agents discover and access relevant knowledge artifacts.

## How to Use This Index

1. **Search by Category**: Browse the category sections below
2. **Search by Keywords**: Use search tools to find entries containing specific terms
3. **Check Cross-References**: See related documents in the cross_references.md file
4. **View Distribution History**: See the history of knowledge distribution in distribution_log.md

## Categories

"""

    # Group knowledge files by category
    by_category = {}
    for knowledge in knowledge_files:
        category = knowledge["category"]
        if category not in by_category:
            by_category[category] = []
        by_category[category].append(knowledge)
    
    # Create or update each category section
    for category, files in by_category.items():
        # Check if the category section exists
        category_pattern = f"### {category}\n\n"
        category_end_pattern = r"### [A-Za-z]+"
        
        if category_pattern in current_content:
            # Category exists, update it if requested
            if update_existing:
                # Find the category section
                start_idx = current_content.find(category_pattern) + len(category_pattern)
                rest_content = current_content[start_idx:]
                match = re.search(category_end_pattern, rest_content)
                
                if match:
                    end_idx = start_idx + match.start()
                else:
                    end_idx = len(current_content)
                
                # Replace the category section
                table_header = "| Document | Location | Description | Last Updated |\n|----------|----------|-------------|------------|\n"
                table_rows = ""
                
                for knowledge in files:
                    table_rows += f"| {knowledge['title']} | `/{knowledge['relative_path']}` | {knowledge['description']} | {knowledge['last_updated']} |\n"
                
                new_section = table_header + table_rows + "\n"
                current_content = current_content[:start_idx] + new_section + current_content[end_idx:]
        else:
            # Category doesn't exist, add it
            category_section = f"### {category}\n\n"
            category_section += "| Document | Location | Description | Last Updated |\n"
            category_section += "|----------|----------|-------------|------------|\n"
            
            for knowledge in files:
                category_section += f"| {knowledge['title']} | `/{knowledge['relative_path']}` | {knowledge['description']} | {knowledge['last_updated']} |\n"
            
            category_section += "\n"
            
            # Find the right position to insert (after "## Categories" or at the end)
            if "## Categories" in current_content:
                insert_idx = current_content.find("## Categories") + len("## Categories\n\n")
                current_content = current_content[:insert_idx] + category_section + current_content[insert_idx:]
            else:
                current_content += category_section
    
    # Add or update the "Recent Additions" section
    recent_section = """
## Recent Additions

| Document | Category | Added | Description |
|----------|----------|-------|-------------|
"""
    
    # Sort by last_updated date, newest first
    recent_files = sorted(knowledge_files, key=lambda x: x["last_updated"], reverse=True)[:5]
    
    for knowledge in recent_files:
        recent_section += f"| {knowledge['title']} | {knowledge['category']} | {knowledge['last_updated']} | {knowledge['description']} |\n"
    
    # Add implementation status
    current_date = datetime.now().strftime("%Y-%m-%d")
    recent_section += f"""
## Implementation Status

The Knowledge Registry is currently in its initial phase. The following components are in development:

1. **Automated Indexing**: Scripts for automatically cataloging global knowledge
2. **Search Functionality**: Tools for efficiently finding relevant knowledge
3. **Distribution System**: Automation for knowledge artifact distribution
4. **Cross-Reference Generation**: Tools for identifying related knowledge

This index will be expanded as more knowledge artifacts are identified and cataloged.

---

Last Updated: {current_date}"""
    
    # Replace any existing Recent Additions section
    if "## Recent Additions" in current_content:
        pattern = r"## Recent Additions.*?(?=^#|$)"
        current_content = re.sub(pattern, recent_section, current_content, flags=re.DOTALL | re.MULTILINE)
    else:
        current_content += recent_section
    
    # Write the updated content
    with open(master_index_path, 'w', encoding='utf-8') as file:
        file.write(current_content)
    
    return len(knowledge_files)

def main():
    parser = argparse.ArgumentParser(description="Index knowledge artifacts in the repository")
    parser.add_argument("--dir", default=".", help="Root directory to scan")
    parser.add_argument("--update", action="store_true", help="Update existing index entries")
    args = parser.parse_args()
    
    root_dir = args.dir
    registry_dir = os.path.join(root_dir, REGISTRY_DIR)
    
    # Ensure the registry directory exists
    os.makedirs(registry_dir, exist_ok=True)
    
    print(f"Scanning for knowledge artifacts in {root_dir}...")
    knowledge_files = find_knowledge_files(root_dir)
    print(f"Found {len(knowledge_files)} knowledge artifacts")
    
    count = update_master_index(knowledge_files, registry_dir, args.update)
    print(f"Updated master index with {count} entries")
    
    # Optionally create category indices for major categories
    # (This would be implemented in a future version)
    
    print("Knowledge indexing complete")

if __name__ == "__main__":
    main()