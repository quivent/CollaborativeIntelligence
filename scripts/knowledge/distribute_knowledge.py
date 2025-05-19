#!/usr/bin/env python3
"""
Knowledge Distribution Script

This script manages the final stage of the Knowledge Distribution Architecture,
distributing transformed content to the appropriate global directories.

Usage:
  python3 distribute_knowledge.py [--input INPUT] [--dir DIR] [--auto] [--report-id REPORT_ID]

Options:
  --input INPUT      Specific transformed file to distribute
  --dir DIR          Root directory for the project
  --auto             Process all files from the latest identification report
  --report-id REPORT_ID  Use specific identification report ID (timestamp)
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
REGISTRY_DIR = "KnowledgeRegistry"
IDENTIFICATION_REPORT = "identification_report.md"
DISTRIBUTION_LOG = "distribution_log.md"
MASTER_INDEX = "master_index.md"

def get_latest_report_path(root_dir):
    """Find the latest identification report"""
    registry_dir = os.path.join(root_dir, REGISTRY_DIR)
    report_path = os.path.join(registry_dir, IDENTIFICATION_REPORT)
    
    if not os.path.exists(report_path):
        print(f"No identification report found at {report_path}")
        return None
    
    return report_path

def parse_report(report_path, root_dir):
    """Parse the identification report to find files to distribute"""
    try:
        with open(report_path, 'r', encoding='utf-8') as file:
            content = file.read()
            
            # Extract report timestamp
            timestamp_match = re.search(r'Generated: ([\d-]+ [\d:]+)', content)
            timestamp = timestamp_match.group(1) if timestamp_match else "Unknown"
            
            # Extract high and medium priority files
            files_to_distribute = []
            
            # High priority section
            high_section_match = re.search(r'## High Priority Content.*?\n\|.*?\n\|.*?\n(.*?)(?=\n\n##|$)', content, re.DOTALL)
            if high_section_match:
                high_section = high_section_match.group(1)
                for line in high_section.split('\n'):
                    if '|' in line:
                        parts = [part.strip() for part in line.split('|')]
                        if len(parts) >= 7:  # Account for empty first and last cells
                            agent, session, filename = parts[1], parts[2], parts[3]
                            content_type, score = parts[4], parts[5]
                            target_dir = parts[6]
                            
                            if agent and session and filename:
                                file_path = os.path.join(root_dir, "AGENTS", agent, "Sessions", session, filename)
                                files_to_distribute.append({
                                    "agent": agent,
                                    "session": session,
                                    "filename": filename,
                                    "path": file_path,
                                    "content_type": content_type,
                                    "score": score,
                                    "target_directory": target_dir,
                                    "priority": "high"
                                })
            
            # Medium priority section
            medium_section_match = re.search(r'## Medium Priority Content.*?\n\|.*?\n\|.*?\n(.*?)(?=\n\n##|$)', content, re.DOTALL)
            if medium_section_match:
                medium_section = medium_section_match.group(1)
                for line in medium_section.split('\n'):
                    if '|' in line:
                        parts = [part.strip() for part in line.split('|')]
                        if len(parts) >= 7:  # Account for empty first and last cells
                            agent, session, filename = parts[1], parts[2], parts[3]
                            content_type, score = parts[4], parts[5]
                            target_dir = parts[6]
                            
                            if agent and session and filename:
                                file_path = os.path.join(root_dir, "AGENTS", agent, "Sessions", session, filename)
                                files_to_distribute.append({
                                    "agent": agent,
                                    "session": session,
                                    "filename": filename,
                                    "path": file_path,
                                    "content_type": content_type,
                                    "score": score,
                                    "target_directory": target_dir,
                                    "priority": "medium"
                                })
            
            return {
                "timestamp": timestamp,
                "files": files_to_distribute
            }
            
    except Exception as e:
        print(f"Error parsing report {report_path}: {e}")
        return None

def transform_content(file_info, root_dir):
    """Transform the content using the transformation script"""
    if not os.path.exists(file_info["path"]):
        print(f"File not found: {file_info['path']}")
        return None
    
    # Determine content type
    content_type = file_info["content_type"].lower()
    if not content_type or content_type == "unknown":
        # Try to guess from filename
        filename = os.path.basename(file_info["path"]).lower()
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
    
    # Path to the transformation script
    transform_script = os.path.join(os.path.dirname(os.path.abspath(__file__)), "transform_knowledge.py")
    
    if not os.path.exists(transform_script):
        print(f"Transformation script not found: {transform_script}")
        return None
    
    # Create temporary directory for transformed content
    temp_dir = os.path.join(root_dir, "KnowledgeRegistry", "temp")
    os.makedirs(temp_dir, exist_ok=True)
    
    # Generate a unique output filename
    output_file = os.path.join(temp_dir, f"transformed_{datetime.now().strftime('%Y%m%d%H%M%S')}_{os.path.basename(file_info['path'])}")
    
    # Run the transformation script
    import subprocess
    cmd = [
        transform_script,
        "--input", file_info["path"],
        "--output", output_file,
        "--type", content_type
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(result.stdout)
        
        if not os.path.exists(output_file):
            print(f"Transformation failed: output file not created at {output_file}")
            if result.stderr:
                print(f"Error: {result.stderr}")
            return None
        
        # Update file info with transformed path
        file_info["transformed_path"] = output_file
        return file_info
        
    except subprocess.CalledProcessError as e:
        print(f"Transformation failed with exit code {e.returncode}")
        print(f"Error: {e.stderr}")
        return None
    except Exception as e:
        print(f"Error running transformation: {e}")
        return None

def distribute_file(file_info, root_dir):
    """Distribute a transformed file to its target directory"""
    if not "transformed_path" in file_info or not os.path.exists(file_info["transformed_path"]):
        print(f"Transformed file not found for {file_info['filename']}")
        return None
    
    # Determine target directory
    target_base = file_info["target_directory"]
    if not target_base:
        # Default mapping based on content type
        content_type = file_info["content_type"].lower()
        if content_type == "architecture":
            target_base = "Architectures"
        elif content_type == "plan":
            target_base = "Plans"
        elif content_type == "documentation":
            target_base = "Documentation"
        elif content_type == "analysis":
            target_base = "Analyses"
        elif content_type == "protocol":
            target_base = "Protocols"
        elif content_type == "notification":
            target_base = "."  # Root directory
        else:
            target_base = "Documentation"  # Default
    
    # Handle relative paths
    if not os.path.isabs(target_base):
        target_dir = os.path.join(root_dir, target_base)
    else:
        target_dir = target_base
    
    # Ensure target directory exists
    os.makedirs(target_dir, exist_ok=True)
    
    # Generate target filename
    filename = os.path.basename(file_info["transformed_path"])
    if filename.startswith("transformed_"):
        # Strip the temporary prefix
        parts = filename.split("_", 2)
        if len(parts) > 2:
            filename = parts[2]
    
    # Clean up the filename
    filename = re.sub(r'[^a-zA-Z0-9_\-.]', '_', filename)
    
    target_path = os.path.join(target_dir, filename)
    
    # Check if target already exists
    if os.path.exists(target_path):
        # Create a backup
        backup_path = f"{target_path}.bak"
        try:
            shutil.copy2(target_path, backup_path)
            print(f"Created backup at {backup_path}")
        except Exception as e:
            print(f"Error creating backup: {e}")
    
    # Copy the transformed file to the target location
    try:
        shutil.copy2(file_info["transformed_path"], target_path)
        print(f"Distributed file to {target_path}")
        
        # Update file info with final path
        file_info["distributed_path"] = target_path
        
        # Create a redirection notice in the original location
        create_redirection_notice(file_info, target_path)
        
        return file_info
    except Exception as e:
        print(f"Error distributing file: {e}")
        return None

def create_redirection_notice(file_info, target_path):
    """Create a notice in the original file location pointing to the distributed file"""
    if not os.path.exists(file_info["path"]):
        return
    
    # Use relative path from original file to target
    rel_path = os.path.relpath(target_path, os.path.dirname(file_info["path"]))
    
    # Create notice content
    notice = f"""# Document Moved

This document has been distributed to a global location as part of the Knowledge Distribution Architecture.

## New Location

The document is now available at: 

`{rel_path}`

## Why Was This Moved?

This document was identified as having system-wide relevance and has been transformed and moved to an appropriate global directory to ensure all agents can access it.

## What Should I Do?

Please use the new location for all future references to this document. This notice is left here to prevent broken links and to help with the transition to the new architecture.

---

Distributed by Knowledge Distribution Architecture: {datetime.now().strftime('%Y-%m-%d')}
"""
    
    # Write the notice
    try:
        # Backup the original file
        backup_path = f"{file_info['path']}.original"
        shutil.copy2(file_info["path"], backup_path)
        
        # Write the notice
        with open(file_info["path"], 'w', encoding='utf-8') as file:
            file.write(notice)
        
        print(f"Created redirection notice at {file_info['path']}")
    except Exception as e:
        print(f"Error creating redirection notice: {e}")

def update_distribution_log(file_info, root_dir):
    """Update the distribution log with the newly distributed file"""
    distribution_log = os.path.join(root_dir, REGISTRY_DIR, DISTRIBUTION_LOG)
    
    if not os.path.exists(os.path.dirname(distribution_log)):
        os.makedirs(os.path.dirname(distribution_log), exist_ok=True)
    
    # Load existing log or create new one
    if not os.path.exists(distribution_log):
        with open(distribution_log, 'w', encoding='utf-8') as log:
            log.write("# Knowledge Distribution Log\n\n")
            log.write("## Purpose\n\n")
            log.write("This document tracks the distribution of knowledge artifacts from agent sessions to global repositories, ")
            log.write("providing an audit trail and visibility into knowledge flow across the Collaborative Intelligence system.\n\n")
            log.write("## Log Structure\n\n")
            log.write("Each entry in this log includes:\n")
            log.write("- **Timestamp**: When the distribution occurred\n")
            log.write("- **Source**: Original location of the knowledge artifact\n")
            log.write("- **Destination**: Global repository where the artifact was placed\n")
            log.write("- **Artifact**: Name and description of the distributed content\n")
            log.write("- **Trigger**: What caused the distribution (automatic or manual)\n")
            log.write("- **Agent**: Who initiated or performed the distribution\n")
            log.write("- **Status**: Current status of the distribution\n\n")
            log.write("## Recent Distributions\n\n")
    
    # Append the new distribution
    try:
        with open(distribution_log, 'a', encoding='utf-8') as log:
            log.write(f"\n### {datetime.now().strftime('%Y-%m-%d')}: {os.path.basename(file_info['distributed_path'])}\n\n")
            log.write(f"**Source**: `{file_info['path']}`  \n")
            log.write(f"**Destination**: `{file_info['distributed_path']}`  \n")
            log.write(f"**Artifact**: {file_info['filename']} ({file_info['content_type']})  \n")
            log.write("**Trigger**: Automatic (Knowledge Distribution System)  \n")
            log.write("**Agent**: Knowledge Distribution Script  \n")
            log.write("**Status**: Complete  \n")
            log.write(f"**Notes**: Relevance score: {file_info['score']}, Priority: {file_info['priority']}\n")
        
        print(f"Updated distribution log at {distribution_log}")
    except Exception as e:
        print(f"Error updating distribution log: {e}")

def update_master_index(file_info, root_dir):
    """Update the master index with the newly distributed file"""
    # This is a simple placeholder - in practice, we would run the index_knowledge.py script
    # to properly update the master index with all metadata
    
    index_script = os.path.join(os.path.dirname(os.path.abspath(__file__)), "index_knowledge.py")
    
    if not os.path.exists(index_script):
        print(f"Indexing script not found: {index_script}")
        return
    
    # Run the indexing script
    import subprocess
    cmd = [
        index_script,
        "--dir", root_dir,
        "--update"
    ]
    
    try:
        result = subprocess.run(cmd, capture_output=True, text=True, check=True)
        print(result.stdout)
    except subprocess.CalledProcessError as e:
        print(f"Indexing failed with exit code {e.returncode}")
        print(f"Error: {e.stderr}")
    except Exception as e:
        print(f"Error running indexing: {e}")

def clean_up_temp_files(file_info):
    """Clean up temporary files created during the distribution process"""
    if "transformed_path" in file_info and os.path.exists(file_info["transformed_path"]):
        try:
            os.remove(file_info["transformed_path"])
            print(f"Removed temporary file {file_info['transformed_path']}")
        except Exception as e:
            print(f"Error removing temporary file: {e}")

def distribute_single_file(file_path, content_type, root_dir):
    """Process a single file for distribution"""
    # Create file info object
    file_info = {
        "agent": "Manual",
        "session": "Manual",
        "filename": os.path.basename(file_path),
        "path": file_path,
        "content_type": content_type,
        "score": "1.0",
        "target_directory": "",
        "priority": "high"
    }
    
    # Transform the content
    file_info = transform_content(file_info, root_dir)
    if not file_info:
        return False
    
    # Distribute the file
    file_info = distribute_file(file_info, root_dir)
    if not file_info:
        return False
    
    # Update distribution log
    update_distribution_log(file_info, root_dir)
    
    # Update master index
    update_master_index(file_info, root_dir)
    
    # Clean up
    clean_up_temp_files(file_info)
    
    return True

def distribute_from_report(report_path, root_dir):
    """Process all files from an identification report"""
    report_data = parse_report(report_path, root_dir)
    if not report_data:
        return False
    
    print(f"Processing report from {report_data['timestamp']}")
    print(f"Found {len(report_data['files'])} files to distribute")
    
    success_count = 0
    
    for file_info in report_data['files']:
        print(f"\nProcessing {file_info['path']}...")
        
        # Transform the content
        transformed_info = transform_content(file_info, root_dir)
        if not transformed_info:
            print(f"Skipping {file_info['path']} due to transformation failure")
            continue
        
        # Distribute the file
        distributed_info = distribute_file(transformed_info, root_dir)
        if not distributed_info:
            print(f"Skipping {file_info['path']} due to distribution failure")
            clean_up_temp_files(transformed_info)
            continue
        
        # Update distribution log
        update_distribution_log(distributed_info, root_dir)
        
        # Clean up
        clean_up_temp_files(distributed_info)
        
        success_count += 1
    
    # Update master index once at the end
    print("\nUpdating master index...")
    index_script = os.path.join(os.path.dirname(os.path.abspath(__file__)), "index_knowledge.py")
    
    if os.path.exists(index_script):
        import subprocess
        cmd = [
            index_script,
            "--dir", root_dir,
            "--update"
        ]
        
        try:
            result = subprocess.run(cmd, capture_output=True, text=True, check=True)
            print(result.stdout)
        except Exception as e:
            print(f"Error running indexing: {e}")
    
    print(f"\nDistribution complete. {success_count} of {len(report_data['files'])} files successfully distributed.")
    return success_count > 0

def main():
    parser = argparse.ArgumentParser(description="Distribute knowledge to appropriate global directories")
    parser.add_argument("--input", help="Specific transformed file to distribute")
    parser.add_argument("--dir", default=".", help="Root directory for the project")
    parser.add_argument("--auto", action="store_true", help="Process all files from the latest identification report")
    parser.add_argument("--report-id", help="Use specific identification report ID (timestamp)")
    parser.add_argument("--type", choices=["architecture", "plan", "documentation", "analysis", "protocol", "notification"], 
                        help="Content type for manual distribution")
    args = parser.parse_args()
    
    root_dir = os.path.abspath(args.dir)
    
    if args.input:
        print(f"Processing single file: {args.input}")
        success = distribute_single_file(args.input, args.type or "documentation", root_dir)
        return 0 if success else 1
    elif args.auto or args.report_id:
        # Find the appropriate report
        if args.report_id:
            report_path = os.path.join(root_dir, REGISTRY_DIR, f"identification_report_{args.report_id}.md")
            if not os.path.exists(report_path):
                print(f"Report not found: {report_path}")
                return 1
        else:
            report_path = get_latest_report_path(root_dir)
            if not report_path:
                return 1
        
        print(f"Processing report: {report_path}")
        success = distribute_from_report(report_path, root_dir)
        return 0 if success else 1
    else:
        parser.print_help()
        return 1

if __name__ == "__main__":
    sys.exit(main())