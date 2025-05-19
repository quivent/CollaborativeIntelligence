#!/usr/bin/env python3
"""
Knowledge Registry Search Tool

This script provides search capabilities for the Knowledge Registry,
allowing users to find knowledge artifacts based on various criteria.

Usage:
  python3 search_knowledge.py query [options]
  python3 search_knowledge.py --category CATEGORY [options]
  python3 search_knowledge.py --tag TAG [options]
  python3 search_knowledge.py --recent [N] [options]

Arguments:
  query                Free-text search query

Options:
  --dir DIR            Root directory for the project (default: current directory)
  --category CATEGORY  Filter by category (e.g., architecture, plan, documentation)
  --tag TAG           Filter by tag/keyword
  --recent [N]         Show N most recent additions (default: 5)
  --full               Show full document descriptions in results
  --json               Output results in JSON format
  --md                 Output results in Markdown format (default)
"""

import os
import sys
import argparse
import re
import json
from datetime import datetime
import difflib

# Configuration
REGISTRY_DIR = "KnowledgeRegistry"
MASTER_INDEX = "master_index.md"
CROSS_REFERENCES = "cross_references.md"

def load_master_index(root_dir):
    """Load the master index file"""
    master_index_path = os.path.join(root_dir, REGISTRY_DIR, MASTER_INDEX)
    
    if not os.path.exists(master_index_path):
        print(f"Master index not found at {master_index_path}")
        return None
    
    try:
        with open(master_index_path, 'r', encoding='utf-8') as file:
            return file.read()
    except Exception as e:
        print(f"Error reading master index: {e}")
        return None

def load_cross_references(root_dir):
    """Load the cross-references file"""
    cross_references_path = os.path.join(root_dir, REGISTRY_DIR, CROSS_REFERENCES)
    
    if not os.path.exists(cross_references_path):
        print(f"Cross-references not found at {cross_references_path}")
        return None
    
    try:
        with open(cross_references_path, 'r', encoding='utf-8') as file:
            return file.read()
    except Exception as e:
        print(f"Error reading cross-references: {e}")
        return None

def extract_categories(master_index):
    """Extract all categories from the master index"""
    categories = {}
    
    # Find all category sections
    category_sections = re.findall(r'### ([^\n]+)\n\n\| Document \| Location \| Description \| Last Updated \|\n\|[^\n]+\n((?:\|[^\n]+\n)+)', master_index)
    
    for category_name, table_content in category_sections:
        categories[category_name] = []
        
        # Extract all rows in the table
        rows = table_content.strip().split('\n')
        for row in rows:
            parts = [part.strip() for part in row.split('|')]
            if len(parts) >= 6:  # Account for empty first and last parts
                document = parts[1].strip()
                location = parts[2].strip()
                description = parts[3].strip()
                last_updated = parts[4].strip()
                
                if document and location:
                    categories[category_name].append({
                        "document": document,
                        "location": location,
                        "description": description,
                        "last_updated": last_updated
                    })
    
    return categories

def extract_recent_additions(master_index, limit=5):
    """Extract recent additions from the master index"""
    recent_section = re.search(r'## Recent Additions\n\n\| Document \| Category \| Added \| Description \|\n\|[^\n]+\n((?:\|[^\n]+\n)+)', master_index)
    
    if not recent_section:
        return []
        
    recent_additions = []
    rows = recent_section.group(1).strip().split('\n')
    
    for i, row in enumerate(rows):
        if i >= limit:
            break
            
        parts = [part.strip() for part in row.split('|')]
        if len(parts) >= 6:  # Account for empty first and last parts
            document = parts[1].strip()
            category = parts[2].strip()
            added_date = parts[3].strip()
            description = parts[4].strip()
            
            if document:
                recent_additions.append({
                    "document": document,
                    "category": category,
                    "added_date": added_date,
                    "description": description
                })
    
    return recent_additions

def extract_relationships(cross_references, document):
    """Extract relationships for a specific document from cross-references"""
    relationships = {
        "implements": [],
        "extends": [],
        "references": [],
        "supersedes": [],
        "analyzes": [],
        "complements": []
    }
    
    if not cross_references:
        return relationships
    
    # Normalize document name for matching
    doc_normalized = document.lower()
    
    # Find sections that mention the document
    doc_sections = re.findall(r'### ([^\n]+)\n\n(?:.*?\n)+?((?:\*\*\w+\*\*:\n(?:- [^\n]+\n)+)+)', cross_references, re.DOTALL)
    
    for section_doc, relationships_section in doc_sections:
        # Check if this section is about our document
        if not similar_enough(section_doc, document):
            continue
            
        # Extract relationships
        rel_patterns = re.findall(r'\*\*(\w+)\*\*:\n((?:- [^\n]+\n)+)', relationships_section)
        for rel_type, rel_items in rel_patterns:
            rel_type_lower = rel_type.lower()
            if rel_type_lower in relationships:
                items = re.findall(r'- ([^\n]+)', rel_items)
                relationships[rel_type_lower].extend(items)
    
    # Also search for mentions of the document in other sections
    all_sections = re.findall(r'### ([^\n]+)\n\n(?:\*\*(\w+)\*\*:\n(?:(?:- ([^\n]+)\n)+))+', cross_references, re.DOTALL)
    for section_doc, rel_type, rel_item in all_sections:
        if section_doc == document:
            continue  # Skip the document's own section
            
        rel_type_lower = rel_type.lower() if rel_type else ""
        reverse_map = {
            "implements": "implemented by",
            "extends": "extended by",
            "references": "referenced by",
            "supersedes": "superseded by",
            "analyzes": "analyzed by",
            "complements": "complemented by"
        }
        
        # Check if this document is mentioned in other sections
        rel_items = re.findall(r'- ([^\n]+)', rel_item)
        for item in rel_items:
            if similar_enough(item, document):
                reverse_rel = reverse_map.get(rel_type_lower, "related to")
                if reverse_rel not in relationships:
                    relationships[reverse_rel] = []
                if section_doc not in relationships[reverse_rel]:
                    relationships[reverse_rel].append(section_doc)
    
    return relationships

def similar_enough(str1, str2, threshold=0.8):
    """Check if two strings are similar enough using difflib"""
    # Extract just the title part from potential paths or formatted strings
    str1_parts = re.search(r'(?:^|/)([^/]+?)(?:\.md)?$', str1)
    str2_parts = re.search(r'(?:^|/)([^/]+?)(?:\.md)?$', str2)
    
    s1 = str1_parts.group(1) if str1_parts else str1
    s2 = str2_parts.group(1) if str2_parts else str2
    
    # Lowercase for better matching
    s1 = s1.lower()
    s2 = s2.lower()
    
    # Remove common words that might confuse similarity
    common_words = ['the', 'and', 'for', 'with', 'in', 'on', 'of']
    for word in common_words:
        s1 = s1.replace(f" {word} ", " ")
        s2 = s2.replace(f" {word} ", " ")
    
    similarity = difflib.SequenceMatcher(None, s1, s2).ratio()
    return similarity >= threshold

def search_knowledge(master_index, cross_references, query=None, category=None, tag=None, recent=None, full=False):
    """Search the knowledge registry based on the provided criteria"""
    results = []
    
    # Extract all categories
    categories = extract_categories(master_index)
    
    # Process recent additions request
    if recent is not None:
        limit = recent if isinstance(recent, int) else 5
        return extract_recent_additions(master_index, limit)
    
    # Process category filter
    if category:
        category_matches = []
        for cat_name, documents in categories.items():
            if category.lower() in cat_name.lower():
                category_matches.extend([(cat_name, doc) for doc in documents])
        
        if not category_matches:
            print(f"No categories found matching '{category}'")
            # Show available categories
            print("Available categories:")
            for cat_name in categories.keys():
                print(f"  - {cat_name}")
            return []
        
        # Add all documents in matching categories to results
        for cat_name, doc in category_matches:
            doc["category"] = cat_name
            results.append(doc)
        
        return results
    
    # Process tag filter
    if tag:
        tag_matches = []
        for cat_name, documents in categories.items():
            for doc in documents:
                # Check if tag appears in document name or description
                if (tag.lower() in doc["document"].lower() or 
                    tag.lower() in doc["description"].lower()):
                    doc_copy = doc.copy()
                    doc_copy["category"] = cat_name
                    tag_matches.append(doc_copy)
        
        return tag_matches
    
    # Process search query
    if query:
        query_matches = []
        for cat_name, documents in categories.items():
            for doc in documents:
                # Calculate a score based on where the query appears
                score = 0
                
                # Highest score for document title matches
                if query.lower() in doc["document"].lower():
                    score += 5
                    # Even higher for exact match
                    if query.lower() == doc["document"].lower():
                        score += 5
                
                # Medium score for description matches
                if query.lower() in doc["description"].lower():
                    score += 3
                
                # Lower score for location matches
                if query.lower() in doc["location"].lower():
                    score += 1
                
                if score > 0:
                    doc_copy = doc.copy()
                    doc_copy["category"] = cat_name
                    doc_copy["score"] = score
                    
                    # Add relationship information if requested
                    if full:
                        doc_copy["relationships"] = extract_relationships(cross_references, doc["document"])
                    
                    query_matches.append(doc_copy)
        
        # Sort by score (highest first)
        query_matches.sort(key=lambda x: x["score"], reverse=True)
        return query_matches
    
    # If no specific criteria provided, return everything
    all_documents = []
    for cat_name, documents in categories.items():
        for doc in documents:
            doc_copy = doc.copy()
            doc_copy["category"] = cat_name
            all_documents.append(doc_copy)
    
    return all_documents

def format_results_json(results):
    """Format search results as JSON"""
    return json.dumps(results, indent=2)

def format_results_markdown(results, full=False):
    """Format search results as Markdown"""
    if not results:
        return "No results found."
    
    markdown = "# Knowledge Registry Search Results\n\n"
    markdown += f"Found {len(results)} results.\n\n"
    
    # Group by category
    by_category = {}
    for result in results:
        category = result.get("category", "Uncategorized")
        if category not in by_category:
            by_category[category] = []
        by_category[category].append(result)
    
    # Output results by category
    for category, docs in by_category.items():
        markdown += f"## {category}\n\n"
        
        for doc in docs:
            markdown += f"### {doc['document']}\n\n"
            markdown += f"**Location:** {doc['location']}\n\n"
            markdown += f"**Last Updated:** {doc.get('last_updated', 'Unknown')}\n\n"
            
            # Show score if available
            if "score" in doc:
                markdown += f"**Relevance Score:** {doc['score']}\n\n"
            
            # Show first part of description
            description = doc.get("description", "No description available.")
            if len(description) > 200 and not full:
                description = description[:200] + "..."
            markdown += f"**Description:** {description}\n\n"
            
            # Show relationships if available and full mode is enabled
            if full and "relationships" in doc:
                rel = doc["relationships"]
                if any(rel.values()):
                    markdown += "**Relationships:**\n\n"
                    for rel_type, rel_items in rel.items():
                        if rel_items:
                            markdown += f"- {rel_type.capitalize()}: "
                            markdown += ", ".join(rel_items)
                            markdown += "\n"
                    markdown += "\n"
            
            markdown += "---\n\n"
    
    return markdown

def main():
    parser = argparse.ArgumentParser(description="Search the Knowledge Registry")
    
    # Search criteria
    parser.add_argument("query", nargs="?", help="Free-text search query")
    parser.add_argument("--category", help="Filter by category")
    parser.add_argument("--tag", help="Filter by tag/keyword")
    parser.add_argument("--recent", nargs="?", const=5, type=int, help="Show N most recent additions")
    
    # Output options
    parser.add_argument("--dir", default=".", help="Root directory for the project")
    parser.add_argument("--full", action="store_true", help="Show full document descriptions")
    parser.add_argument("--json", action="store_true", help="Output results in JSON format")
    parser.add_argument("--md", action="store_true", help="Output results in Markdown format (default)")
    
    args = parser.parse_args()
    
    # Validate input
    if not (args.query or args.category or args.tag or args.recent is not None):
        parser.print_help()
        return 1
    
    root_dir = os.path.abspath(args.dir)
    
    # Load the master index
    master_index = load_master_index(root_dir)
    if not master_index:
        return 1
    
    # Load cross-references if needed
    cross_references = None
    if args.full:
        cross_references = load_cross_references(root_dir)
    
    # Perform the search
    results = search_knowledge(
        master_index, 
        cross_references,
        query=args.query, 
        category=args.category, 
        tag=args.tag, 
        recent=args.recent,
        full=args.full
    )
    
    # Output the results
    if args.json:
        print(format_results_json(results))
    else:  # Default to markdown
        print(format_results_markdown(results, args.full))
    
    return 0

if __name__ == "__main__":
    sys.exit(main())