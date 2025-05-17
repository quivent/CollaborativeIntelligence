#!/bin/bash

# CI Database Migration Script
# Applies migrations to both internal and projects databases

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MIGRATIONS_DIR="$SCRIPT_DIR/migrations"
INTERNAL_DB="$SCRIPT_DIR/internal/ci_internal.db"
PROJECTS_DB="$SCRIPT_DIR/projects/ci_projects.db"

# Function to apply migrations to a database
apply_migrations() {
    local db_file="$1"
    local db_name="$2"
    
    echo "🔄 Applying migrations to $db_name database..."
    
    # Ensure migration tracking table exists
    sqlite3 "$db_file" "CREATE TABLE IF NOT EXISTS schema_migrations (
        version INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        checksum TEXT
    );"
    
    # Get list of applied migrations
    applied_versions=$(sqlite3 "$db_file" "SELECT version FROM schema_migrations ORDER BY version;" 2>/dev/null || echo "")
    
    # Process each migration file
    for migration in $(ls "$MIGRATIONS_DIR"/*.sql | sort); do
        filename=$(basename "$migration")
        version=$(echo "$filename" | cut -d'_' -f1 | sed 's/^0*//')
        name="${filename%.sql}"
        
        # Check if migration already applied
        if echo "$applied_versions" | grep -q "^$version$"; then
            echo "  ✓ Migration $version already applied: $name"
            continue
        fi
        
        echo "  📝 Applying migration $version: $name"
        
        # Calculate checksum
        checksum=$(md5sum "$migration" | cut -d' ' -f1)
        
        # Apply migration
        sqlite3 "$db_file" < "$migration"
        
        # Record migration
        sqlite3 "$db_file" "INSERT INTO schema_migrations (version, name, checksum) VALUES ($version, '$name', '$checksum');"
        
        echo "  ✅ Migration $version applied successfully"
    done
}

# Check if databases exist
if [ ! -f "$INTERNAL_DB" ]; then
    echo "❌ Internal database not found. Run init.sh first."
    exit 1
fi

if [ ! -f "$PROJECTS_DB" ]; then
    echo "❌ Projects database not found. Run init.sh first."
    exit 1
fi

# Apply migrations to both databases
echo "🚀 Starting database migrations..."
echo ""

apply_migrations "$INTERNAL_DB" "Internal"
echo ""
apply_migrations "$PROJECTS_DB" "Projects"

echo ""
echo "✨ All migrations completed successfully!"