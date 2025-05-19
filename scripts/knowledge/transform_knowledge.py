#!/usr/bin/env python3
"""
Knowledge Transformation Script

This script transforms agent-specific knowledge artifacts into standardized formats
suitable for global distribution.

Usage:
  python3 transform_knowledge.py [--input INPUT] [--output OUTPUT] [--type TYPE] [--simulate]

Options:
  --input INPUT     Input file to transform (required)
  --output OUTPUT   Output location (optional, defaults to derived location)
  --type TYPE       Content type (architecture, plan, documentation, etc.)
  --simulate        Show transformation without writing files
"""

import os
import sys
import argparse
import re
import json
import shutil
from datetime import datetime
from pathlib import Path

# Configuration
TEMPLATES_DIR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "templates")
REGISTRY_DIR = "KnowledgeRegistry"

# Content type templates and standards
CONTENT_TYPES = {
    "architecture": {
        "required_sections": ["Overview", "Components", "Interactions", "Implementation", "Conclusion"],
        "metadata_fields": ["created", "updated", "owner", "status", "priority"],
        "filename_format": "{name}_ARCHITECTURE.md"
    },
    "plan": {
        "required_sections": ["Overview", "Objectives", "Timeline", "Implementation Steps", "Success Criteria"],
        "metadata_fields": ["created", "updated", "owner", "status", "priority"],
        "filename_format": "{name}_PLAN.md"
    },
    "documentation": {
        "required_sections": ["Purpose", "Usage", "Details", "Examples"],
        "metadata_fields": ["created", "updated", "category", "author"],
        "filename_format": "{name}.md"
    },
    "analysis": {
        "required_sections": ["Purpose", "Methodology", "Findings", "Conclusion"],
        "metadata_fields": ["created", "updated", "subject", "author"],
        "filename_format": "{name}_ANALYSIS.md"
    },
    "protocol": {
        "required_sections": ["Purpose", "Process", "Standards", "Compliance"],
        "metadata_fields": ["created", "updated", "scope", "status"],
        "filename_format": "{name}_PROTOCOL.md"
    },
    "notification": {
        "required_sections": ["Summary", "Details", "Implementation", "Timeline"],
        "metadata_fields": ["created", "updated", "urgency", "scope"],
        "filename_format": "NOTIFICATION_{name}.md"
    }
}

def load_content(file_path):
    """Load content from a file"""
    try:
        with open(file_path, 'r', encoding='utf-8') as file:
            return file.read()
    except Exception as e:
        print(f"Error loading content from {file_path}: {e}")
        return None

def extract_metadata(content, file_path):
    """Extract metadata from content and filename"""
    metadata = {
        "created": datetime.now().strftime("%Y-%m-%d"),
        "updated": datetime.now().strftime("%Y-%m-%d"),
        "source_file": file_path
    }
    
    # Try to extract from front matter if it exists
    front_matter_match = re.search(r'^---\s+(.*?)\s+---', content, re.DOTALL)
    if front_matter_match:
        front_matter = front_matter_match.group(1)
        for line in front_matter.split('\n'):
            if ':' in line:
                key, value = line.split(':', 1)
                metadata[key.strip().lower()] = value.strip()
    
    # Try to find JSON metadata block
    json_match = re.search(r'```json\s+(.*?)\s+```', content, re.DOTALL)
    if json_match:
        try:
            json_data = json.loads(json_match.group(1))
            for key, value in json_data.items():
                metadata[key.lower()] = value
        except:
            pass
    
    # Try to extract from metadata.json in same directory
    metadata_path = os.path.join(os.path.dirname(file_path), "metadata.json")
    if os.path.exists(metadata_path):
        try:
            with open(metadata_path, 'r', encoding='utf-8') as file:
                json_data = json.load(file)
                for key, value in json_data.items():
                    metadata[key.lower()] = value
        except:
            pass
    
    # Extract title from first heading
    title_match = re.search(r'^#\s+(.*?)$', content, re.MULTILINE)
    if title_match:
        metadata["title"] = title_match.group(1).strip()
    
    # Try to guess agent from path
    agent_match = re.search(r'/AGENTS/([^/]+)/', file_path)
    if agent_match:
        metadata["agent"] = agent_match.group(1)
    
    return metadata

def identify_sections(content):
    """Identify existing sections in the content"""
    sections = {}
    current_section = None
    current_content = []
    
    # Split content into lines
    lines = content.split('\n')
    
    for line in lines:
        # Check for headings
        heading_match = re.match(r'^(#+)\s+(.*?)$', line)
        if heading_match:
            # Save previous section if it exists
            if current_section:
                sections[current_section] = '\n'.join(current_content)
                current_content = []
            
            # Start new section
            current_section = heading_match.group(2).strip()
            continue
        
        # Add line to current section if one exists
        if current_section:
            current_content.append(line)
    
    # Save the last section
    if current_section:
        sections[current_section] = '\n'.join(current_content)
    
    return sections

def normalize_section_name(name):
    """Normalize section names to match required sections"""
    name = name.lower()
    
    # Common mappings
    mappings = {
        "summary": "overview",
        "introduction": "overview",
        "design": "components",
        "architecture": "components",
        "system design": "components",
        "component design": "components",
        "interaction": "interactions",
        "component interaction": "interactions",
        "relationships": "interactions",
        "connections": "interactions",
        "steps": "implementation steps",
        "implementation process": "implementation steps",
        "deployment": "implementation steps",
        "criteria": "success criteria",
        "validation": "success criteria",
        "verification": "success criteria",
        "result": "conclusion",
        "summary": "conclusion",
        "outcomes": "conclusion",
        "final remarks": "conclusion"
    }
    
    return mappings.get(name, name)

def transform_content(content, metadata, content_type):
    """Transform content to match the standardized template"""
    # Get content type standards
    if content_type not in CONTENT_TYPES:
        print(f"Unknown content type: {content_type}. Defaulting to documentation.")
        content_type = "documentation"
        
    standards = CONTENT_TYPES[content_type]
    required_sections = standards["required_sections"]
    
    # Identify existing sections
    existing_sections = identify_sections(content)
    
    # Create a mapping of normalized section names to actual section names
    normalized_mapping = {}
    for section_name in existing_sections.keys():
        normalized = normalize_section_name(section_name.lower())
        normalized_mapping[normalized] = section_name
    
    # Create new content with required sections
    new_content = [f"# {metadata.get('title', 'Untitled Document')}"]
    
    # Add standardized metadata block
    new_content.append("\n## Metadata\n")
    new_content.append("```json")
    metadata_block = {}
    for field in standards["metadata_fields"]:
        if field in metadata:
            metadata_block[field] = metadata[field]
        else:
            metadata_block[field] = f"[{field}]"  # Placeholder
    
    # Add source information
    metadata_block["source_file"] = metadata.get("source_file", "")
    metadata_block["source_agent"] = metadata.get("agent", "")
    metadata_block["transformed"] = datetime.now().strftime("%Y-%m-%d")
    
    new_content.append(json.dumps(metadata_block, indent=2))
    new_content.append("```\n")
    
    # Add each required section
    for section in required_sections:
        new_content.append(f"\n## {section}\n")
        
        # If we have matching content, use it
        section_lower = section.lower()
        if section_lower in normalized_mapping:
            actual_section = normalized_mapping[section_lower]
            new_content.append(existing_sections[actual_section].strip())
        else:
            # Try to find content based on content patterns
            if section == "Overview" and "Introduction" in existing_sections:
                new_content.append(existing_sections["Introduction"].strip())
            elif section == "Components" and "Design" in existing_sections:
                new_content.append(existing_sections["Design"].strip())
            elif section == "Implementation" and "Steps" in existing_sections:
                new_content.append(existing_sections["Steps"].strip())
            else:
                # Add placeholder
                new_content.append(f"[{section} content from original document not found]")
    
    # Add source attribution
    new_content.append("\n## Source\n")
    new_content.append(f"This document was transformed from {metadata.get('source_file', 'an agent session document')} ")
    new_content.append(f"created by {metadata.get('agent', 'an agent')}.\n")
    
    # Add transformation footer
    new_content.append("\n---\n")
    new_content.append(f"Transformed by Knowledge Distribution Architecture: {datetime.now().strftime('%Y-%m-%d')}")
    
    return "\n".join(new_content)

def determine_output_path(input_path, metadata, content_type):
    """Determine the appropriate output path for the transformed content"""
    # Get the appropriate global directory based on content type
    if content_type == "architecture":
        global_dir = "Architectures"
    elif content_type == "plan":
        global_dir = "Plans"
    elif content_type == "documentation":
        global_dir = "Documentation"
    elif content_type == "analysis":
        global_dir = "Analyses"
    elif content_type == "protocol":
        global_dir = "Protocols"
    elif content_type == "notification":
        global_dir = "."  # Root directory
    else:
        global_dir = "Documentation"  # Default
    
    # Get the root directory
    root_dir = os.path.abspath(os.path.join(os.path.dirname(input_path), "../.."))
    while not os.path.basename(root_dir) in ["CollaborativeIntelligence", "."]:
        parent = os.path.dirname(root_dir)
        if parent == root_dir:  # Reached filesystem root
            break
        root_dir = parent
    
    # Generate output path
    base_name = os.path.splitext(os.path.basename(input_path))[0]
    
    # Clean up the name
    base_name = re.sub(r'[^a-zA-Z0-9_]', '_', base_name)
    
    # Apply filename format from standards
    standards = CONTENT_TYPES.get(content_type, CONTENT_TYPES["documentation"])
    filename = standards["filename_format"].format(name=base_name)
    
    # For architectures, create subdirectories based on the name
    if content_type == "architecture":
        # Check if the name has an architecture-specific prefix
        match = re.match(r'(.+?)_ARCHITECTURE', base_name, re.IGNORECASE)
        if match:
            component_name = match.group(1)
            return os.path.join(root_dir, global_dir, component_name, filename)
    
    return os.path.join(root_dir, global_dir, filename)

def validate_transformation(original, transformed, content_type):
    """Validate the transformation meets quality standards"""
    issues = []
    
    # Ensure we didn't lose significant content length
    original_lines = len(original.split('\n'))
    transformed_lines = len(transformed.split('\n'))
    
    if transformed_lines < original_lines * 0.5:
        issues.append("WARNING: Significant content loss - transformed document is less than 50% of original")
    
    # Ensure all required sections exist
    standards = CONTENT_TYPES.get(content_type, CONTENT_TYPES["documentation"])
    for section in standards["required_sections"]:
        if f"## {section}" not in transformed:
            issues.append(f"ISSUE: Missing required section '{section}'")
    
    # Check for placeholder content
    if re.search(r'\[\w+ content from original document not found\]', transformed):
        issues.append("WARNING: Some sections have placeholder content")
    
    # Check for metadata completeness
    metadata_block = re.search(r'```json\s+(.*?)\s+```', transformed, re.DOTALL)
    if metadata_block:
        try:
            metadata = json.loads(metadata_block.group(1))
            for field in standards["metadata_fields"]:
                if field not in metadata or metadata[field].startswith('[') and metadata[field].endswith(']'):
                    issues.append(f"ISSUE: Missing metadata field '{field}'")
        except:
            issues.append("ERROR: Malformed metadata JSON block")
    else:
        issues.append("ERROR: Missing metadata JSON block")
    
    return issues

def main():
    parser = argparse.ArgumentParser(description="Transform knowledge for global distribution")
    parser.add_argument("--input", required=True, help="Input file to transform")
    parser.add_argument("--output", help="Output location (defaults to derived location)")
    parser.add_argument("--type", choices=CONTENT_TYPES.keys(), help="Content type")
    parser.add_argument("--simulate", action="store_true", help="Show transformation without writing files")
    args = parser.parse_args()
    
    # Load content
    content = load_content(args.input)
    if not content:
        print(f"Failed to load content from {args.input}")
        return 1
    
    # Extract metadata
    metadata = extract_metadata(content, args.input)
    
    # Determine content type if not provided
    content_type = args.type
    if not content_type:
        # Try to guess from filename
        filename = os.path.basename(args.input).lower()
        if "architecture" in filename:
            content_type = "architecture"
        elif "plan" in filename:
            content_type = "plan"
        elif "analysis" in filename or "benchmark" in filename:
            content_type = "analysis"
        elif "protocol" in filename or "standard" in filename:
            content_type = "protocol"
        elif "notification" in filename:
            content_type = "notification"
        else:
            content_type = "documentation"
    
    # Transform content
    transformed_content = transform_content(content, metadata, content_type)
    
    # Validate transformation
    issues = validate_transformation(content, transformed_content, content_type)
    
    # Determine output path if not provided
    output_path = args.output
    if not output_path:
        output_path = determine_output_path(args.input, metadata, content_type)
    
    # Show results
    print(f"Input: {args.input}")
    print(f"Content Type: {content_type}")
    print(f"Output: {output_path}")
    
    if issues:
        print("\nValidation Issues:")
        for issue in issues:
            print(f"  - {issue}")
    
    if args.simulate:
        print("\nTransformed Content:")
        print("=" * 40)
        print(transformed_content)
        print("=" * 40)
        print("\nSimulation complete. No files were written.")
        return 0
    
    # Create output directory if needed
    os.makedirs(os.path.dirname(output_path), exist_ok=True)
    
    # Write transformed content
    with open(output_path, 'w', encoding='utf-8') as file:
        file.write(transformed_content)
    
    print(f"\nTransformation complete. Transformed content written to {output_path}")
    
    # Update registry
    registry_dir = os.path.join(os.path.dirname(os.path.dirname(output_path)), REGISTRY_DIR)
    if os.path.exists(registry_dir):
        distribution_log = os.path.join(registry_dir, "distribution_log.md")
        if os.path.exists(distribution_log):
            with open(distribution_log, 'a', encoding='utf-8') as log:
                log.write(f"\n### {datetime.now().strftime('%Y-%m-%d')}: {os.path.basename(output_path)}\n\n")
                log.write(f"**Source**: `{args.input}`  \n")
                log.write(f"**Destination**: `{output_path}`  \n")
                log.write(f"**Artifact**: {metadata.get('title', 'Transformed document')}  \n")
                log.write("**Trigger**: Automatic (Knowledge Transformation)  \n")
                log.write("**Agent**: Transformation Script  \n")
                log.write("**Status**: Complete  \n")
                log.write(f"**Notes**: Transformed from {content_type} document in agent session\n")
    
    return 0

if __name__ == "__main__":
    sys.exit(main())