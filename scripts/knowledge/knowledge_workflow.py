#!/usr/bin/env python3
"""
Knowledge Workflow Script

This script coordinates the complete Knowledge Distribution Architecture workflow,
from identification to indexing.

Usage:
  python3 knowledge_workflow.py [--dir DIR] [--threshold THRESHOLD] [--report-only]

Options:
  --dir DIR              Root directory for the project
  --threshold THRESHOLD  Minimum relevance score (0.0-1.0) for content identification
  --report-only          Generate reports without making changes
"""

import os
import sys
import argparse
import subprocess
import time
from datetime import datetime

def run_command(command, description, capture_output=True):
    """Run a command and print its output"""
    print(f"\n=== {description} ===\n")
    
    start_time = time.time()
    try:
        result = subprocess.run(
            command,
            capture_output=capture_output,
            text=True,
            check=True
        )
        
        if capture_output:
            print(result.stdout)
            
        elapsed_time = time.time() - start_time
        print(f"\nCompleted in {elapsed_time:.2f} seconds")
        return True
    except subprocess.CalledProcessError as e:
        print(f"Command failed with exit code {e.returncode}")
        if e.stderr:
            print(f"Error output:\n{e.stderr}")
        return False
    except Exception as e:
        print(f"Error executing command: {e}")
        return False

def main():
    parser = argparse.ArgumentParser(description="Run the complete Knowledge Distribution workflow")
    parser.add_argument("--dir", default=".", help="Root directory for the project")
    parser.add_argument("--threshold", type=float, default=0.7, help="Minimum relevance score (0.0-1.0)")
    parser.add_argument("--report-only", action="store_true", help="Generate reports without making changes")
    args = parser.parse_args()
    
    root_dir = os.path.abspath(args.dir)
    scripts_dir = os.path.dirname(os.path.abspath(__file__))
    
    print(f"Starting Knowledge Distribution workflow in {root_dir}")
    print(f"Timestamp: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}")
    print(f"Mode: {'Report Only' if args.report_only else 'Full Processing'}")
    
    # Step 1: Create the Knowledge Registry if it doesn't exist
    registry_dir = os.path.join(root_dir, "KnowledgeRegistry")
    if not os.path.exists(registry_dir):
        print("\n=== Creating Knowledge Registry ===\n")
        os.makedirs(registry_dir, exist_ok=True)
        print(f"Created Knowledge Registry at {registry_dir}")
    
    # Step 2: Identify globally relevant content
    identification_script = os.path.join(scripts_dir, "identify_global_content.py")
    identification_cmd = [
        identification_script,
        "--dir", root_dir,
        "--threshold", str(args.threshold)
    ]
    
    if args.report_only:
        identification_cmd.append("--report-only")
    
    if not run_command(identification_cmd, "Identifying Globally Relevant Content"):
        print("Content identification failed, aborting workflow")
        return 1
    
    if args.report_only:
        print("\nReport-only mode: Skipping distribution steps")
    else:
        # Step 3: Distribute content from the identification report
        distribution_script = os.path.join(scripts_dir, "distribute_knowledge.py")
        distribution_cmd = [
            distribution_script,
            "--dir", root_dir,
            "--auto"
        ]
        
        if not run_command(distribution_cmd, "Distributing Knowledge"):
            print("Knowledge distribution failed, continuing with indexing")
    
    # Step 4: Update the Knowledge Registry index
    indexing_script = os.path.join(scripts_dir, "index_knowledge.py")
    indexing_cmd = [
        indexing_script,
        "--dir", root_dir,
        "--update"
    ]
    
    if not run_command(indexing_cmd, "Updating Knowledge Registry Index"):
        print("Knowledge indexing failed")
    
    # Final report
    report_path = os.path.join(root_dir, "KnowledgeRegistry", "workflow_report.md")
    with open(report_path, 'w', encoding='utf-8') as report:
        report.write(f"# Knowledge Distribution Workflow Report\n\n")
        report.write(f"Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}\n\n")
        report.write(f"## Workflow Parameters\n\n")
        report.write(f"- Root Directory: `{root_dir}`\n")
        report.write(f"- Relevance Threshold: {args.threshold}\n")
        report.write(f"- Mode: {'Report Only' if args.report_only else 'Full Processing'}\n\n")
        report.write(f"## Workflow Steps\n\n")
        report.write(f"1. **Knowledge Registry** - Ensured registry exists at `{registry_dir}`\n")
        report.write(f"2. **Content Identification** - Executed `{identification_script}`\n")
        if not args.report_only:
            report.write(f"3. **Knowledge Distribution** - Executed `{distribution_script}`\n")
        report.write(f"4. **Knowledge Indexing** - Executed `{indexing_script}`\n\n")
        report.write(f"## Next Steps\n\n")
        report.write(f"1. Review the identification report to confirm content relevance\n")
        report.write(f"2. Check distributed content for proper formatting and location\n")
        report.write(f"3. Verify Knowledge Registry is properly updated\n\n")
        report.write(f"For more details, see individual reports in the Knowledge Registry directory.\n")
    
    print(f"\nWorkflow complete. Report written to {report_path}")
    return 0

if __name__ == "__main__":
    sys.exit(main())