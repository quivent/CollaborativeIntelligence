-- Migration 001: Initial Schema
-- This is a placeholder for the initial schema migration
-- The actual schemas are in internal/schema.sql and projects/schema.sql

-- Migration tracking table (applies to both databases)
CREATE TABLE IF NOT EXISTS schema_migrations (
    version INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    checksum TEXT
);