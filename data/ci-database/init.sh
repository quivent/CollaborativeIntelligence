#!/bin/bash

# CI Database Initialization Script
# Initializes both internal and projects databases

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INTERNAL_DIR="$SCRIPT_DIR/internal"
PROJECTS_DIR="$SCRIPT_DIR/projects"

echo "🗄️  Initializing CI Databases..."

# Function to initialize a database
init_database() {
    local db_type="$1"
    local db_dir="$2"
    local db_file="$3"
    local schema_file="$4"
    
    echo ""
    echo "📊 Initializing $db_type database..."
    
    # Check if database already exists
    if [ -f "$db_file" ]; then
        echo "⚠️  Database already exists: $db_file"
        read -p "Do you want to recreate it? (y/N): " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Skipping $db_type database initialization"
            return
        fi
        
        # Backup existing database
        backup_file="$db_dir/backups/$(basename "$db_file").$(date +%Y%m%d_%H%M%S).backup"
        echo "📦 Creating backup: $backup_file"
        cp "$db_file" "$backup_file"
        rm "$db_file"
    fi
    
    # Create database and apply schema
    echo "🔨 Creating database: $db_file"
    sqlite3 "$db_file" < "$schema_file"
    
    # Set permissions
    chmod 644 "$db_file"
    
    echo "✅ $db_type database initialized successfully"
}

# Initialize internal database
init_database "Internal" "$INTERNAL_DIR" "$INTERNAL_DIR/ci_internal.db" "$INTERNAL_DIR/schema.sql"

# Initialize projects database
init_database "Projects" "$PROJECTS_DIR" "$PROJECTS_DIR/ci_projects.db" "$PROJECTS_DIR/schema.sql"

# Create initial agent entries in internal database
echo ""
echo "🤖 Populating initial agent registry..."
sqlite3 "$INTERNAL_DIR/ci_internal.db" << EOF
INSERT OR IGNORE INTO agents (name, type, capabilities) VALUES
    ('Database', 'specialist', '{"domains": ["data", "schema", "sqlite"]}'),
    ('Athena', 'primary', '{"domains": ["memory", "learning", "architecture"]}'),
    ('Topologist', 'specialist', '{"domains": ["git", "repository", "version-control"]}'),
    ('Architect', 'specialist', '{"domains": ["design", "structure", "patterns"]}'),
    ('Scholar', 'specialist', '{"domains": ["learning", "distributed-systems", "coordination"]}');
EOF

echo ""
echo "🎉 CI Database initialization complete!"
echo ""
echo "Database locations:"
echo "  Internal: $INTERNAL_DIR/ci_internal.db"
echo "  Projects: $PROJECTS_DIR/ci_projects.db"
echo ""
echo "To check database status, run:"
echo "  sqlite3 $INTERNAL_DIR/ci_internal.db '.tables'"
echo "  sqlite3 $PROJECTS_DIR/ci_projects.db '.tables'"