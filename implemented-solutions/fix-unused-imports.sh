#!/bin/bash
# Fix unused imports in env.rs
# Also fixes the TypeScript environment definitions error

ENV_RS_PATH="/Users/joshkornreich/Documents/Projects/Terminals/Claude/src-tauri/src/env.rs"
BACKUP_DIR="/Users/joshkornreich/Documents/Projects/Terminals/Claude/src-tauri/backups"

# Create backup directory if it doesn't exist
mkdir -p "$BACKUP_DIR"

# Create backup of current file
BACKUP_FILE="$BACKUP_DIR/env.rs.backup.imports.$(date +%Y%m%d%H%M%S)"
cp "$ENV_RS_PATH" "$BACKUP_FILE"
echo "Created backup at $BACKUP_FILE"

# Create the fixed file content with unused imports removed
cat > "$ENV_RS_PATH" << 'EOF'
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs;
use tauri::api::path::{app_data_dir, resource_dir};
use tauri::Manager;

/// Represents environment variables that will be shared with the frontend
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EnvVars {
    pub variables: HashMap<String, String>,
}

impl Default for EnvVars {
    fn default() -> Self {
        EnvVars {
            variables: HashMap::new(),
        }
    }
}

pub struct EnvManager {
    env_vars: EnvVars,
}

impl EnvManager {
    /// Initialize the environment manager
    pub fn new(app_handle: &tauri::AppHandle) -> Result<Self, String> {
        // Load .env files with precedence:
        // 1. .env.local (highest priority, not checked into git)
        // 2. .env.development or .env.production (based on environment)
        // 3. .env (lowest priority, default)
        
        // Try to load from the resource directory first (where the app binary is located)
        if let Some(resource_dir) = resource_dir(&app_handle.package_info(), &app_handle.env()) {
            // Try to load environment-specific .env file
            let env_mode = if cfg!(debug_assertions) {
                ".env.development"
            } else {
                ".env.production"
            };
            
            let env_mode_path = resource_dir.join(env_mode);
            if env_mode_path.exists() {
                dotenv::from_path(&env_mode_path).ok();
            }
            
            // Load default .env file
            let env_default_path = resource_dir.join(".env");
            if env_default_path.exists() {
                dotenv::from_path(&env_default_path).ok();
            }
        }
        
        // Try to load from current working directory
        dotenv().ok();
        
        // Try to load .env.local from app data directory
        if let Some(app_dir) = app_data_dir(&app_handle.config()) {
            let local_env_path = app_dir.join(".env.local");
            if local_env_path.exists() {
                dotenv::from_path(&local_env_path).ok();
            }
        }
        
        // Collect environment variables
        let mut env_vars = EnvVars::default();
        
        // Only collect environment variables with specific prefixes
        // This prevents leaking all environment variables to the frontend
        let prefixes = ["CLAUDE_", "VITE_", "TAURI_"];
        
        for (key, value) in env::vars() {
            if prefixes.iter().any(|prefix| key.starts_with(prefix)) {
                env_vars.variables.insert(key, value);
            }
        }
        
        Ok(EnvManager { env_vars })
    }
    
    /// Get all environment variables
    pub fn get_env_vars(&self) -> &EnvVars {
        &self.env_vars
    }
    
    /// Get a specific environment variable
    pub fn get_env_var(&self, key: &str) -> Option<&String> {
        self.env_vars.variables.get(key)
    }
    
    /// Create a development .env.local file with template values if it doesn't exist
    pub fn ensure_local_env_file(app_handle: &tauri::AppHandle) -> Result<(), String> {
        if let Some(app_dir) = app_data_dir(&app_handle.config()) {
            fs::create_dir_all(&app_dir).map_err(|e| format!("Failed to create app directory: {}", e))?;
            
            let local_env_path = app_dir.join(".env.local");
            
            if !local_env_path.exists() {
                let template = r#"# Local environment variables - not checked into git
# This file is automatically created for you to add local overrides
# Variables must start with CLAUDE_, VITE_, or TAURI_ to be available in the frontend

CLAUDE_API_KEY=your_api_key_here
VITE_SOME_CLIENT_SETTING=example_setting
"#;
                
                fs::write(&local_env_path, template)
                    .map_err(|e| format!("Failed to write template .env.local file: {}", e))?;
            }
        }
        
        Ok(())
    }
    
    /// Generate TypeScript definitions for environment variables
    pub fn generate_env_types(app_handle: &tauri::AppHandle) -> Result<(), String> {
        if let Some(resource_dir) = resource_dir(&app_handle.package_info(), &app_handle.env()) {
            let env_files = [
                resource_dir.join(".env"),
                resource_dir.join(".env.development"),
                resource_dir.join(".env.production"),
            ];
            
            let mut env_keys = Vec::new();
            
            // Extract all environment keys from files
            for env_file in env_files {
                if env_file.exists() {
                    let content = fs::read_to_string(&env_file)
                        .map_err(|e| format!("Failed to read env file: {}", e))?;
                    
                    for line in content.lines() {
                        if line.starts_with('#') || line.trim().is_empty() {
                            continue;
                        }
                        
                        if let Some(key) = line.split('=').next() {
                            let key = key.trim();
                            if key.starts_with("VITE_") {
                                env_keys.push(key.to_string());
                            }
                        }
                    }
                }
            }
            
            // Remove duplicates
            env_keys.sort();
            env_keys.dedup();
            
            // Generate TypeScript interface
            let mut ts_content = "// Auto-generated TypeScript definitions for environment variables\n\n".to_string();
            ts_content.push_str("interface ImportMetaEnv {\n");
            
            for key in env_keys {
                ts_content.push_str(&format!("  {}: string;\n", key));
            }
            
            ts_content.push_str("}\n\n");
            ts_content.push_str("interface ImportMeta {\n");
            ts_content.push_str("  readonly env: ImportMetaEnv;\n");
            ts_content.push_str("}\n");
            
            // Create src directory if it doesn't exist (solves "No such file or directory" error)
            let src_dir = resource_dir.parent().unwrap().join("src");
            fs::create_dir_all(&src_dir).map_err(|e| format!("Failed to create src directory: {}", e))?;
            
            // Write to TypeScript file
            let ts_path = src_dir.join("env.d.ts");
            
            fs::write(&ts_path, ts_content)
                .map_err(|e| format!("Failed to write TypeScript env definitions: {}", e))?;
        }
        
        Ok(())
    }
}
EOF

echo "Successfully fixed unused imports in env.rs"
echo "Also updated TypeScript environment definitions to create the src directory if it doesn't exist"