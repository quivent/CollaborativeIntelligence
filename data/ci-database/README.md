# CI System Database

This directory contains the Collaborative Intelligence system's centralized database and related components.

## Directory Structure

```
data/ci-database/
├── master/           # Master database files
├── migrations/       # Schema migration scripts  
├── backups/          # Database backups
├── logs/             # Database operation logs
└── README.md         # This file
```

## Components

### Master Database
The central SQLite database containing:
- Agent registry
- Project metadata
- Operation logs
- Session records
- Knowledge entries

### Migrations
Version-controlled schema changes ensuring database evolution without data loss.

### Backups
Automated backups of the database for disaster recovery.

### Logs
Detailed logs of database operations for debugging and auditing.

## Usage

The database is managed by the Database agent. Direct manipulation should be avoided.

To interact with the database:
```bash
# Initialize database
ci database init

# Check status
ci database status

# Run migrations
ci database migrate
```

## Security

- Database files are excluded from version control
- Sensitive data is encrypted at rest
- Access is mediated through the Database agent API

---

Managed by Database Agent