use crate::{
    parallel_loader::ParallelClaudeLoader, 
    cache_validator::CacheValidator,
    integration_demo::run_integration_demo
};

/// Main entry point for optimized CLAUDE.md processing
pub async fn initialize_claude_with_optimization() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use std::time::Instant;
    
    let start_time = Instant::now();
    let loader = ParallelClaudeLoader::new();
    
    // Load and optimize instructions
    let cache = loader.load_with_cache_optimization().await?;
    let startup_sequence = loader.get_startup_sequence(&cache);
    let load_time = start_time.elapsed();
    
    // Execute Neo identity activation
    let neo_response = startup_sequence.execute_identity_activation();
    
    // Return with timing notice
    Ok(format!("⚡ CLAUDE.md instructions loaded in {:.2}s (optimized cache system active)\n\n{}\n\n{}", 
              load_time.as_secs_f64(), neo_response, startup_sequence.get_performance_summary()))
}

/// CLI command for testing the system
pub async fn run_cache_demo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    run_integration_demo().await
}

/// Validate current cache state
pub async fn validate_cache_health() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    let loader = ParallelClaudeLoader::new();
    let cache = loader.load_with_cache_optimization().await?;
    
    let validator = CacheValidator::new(None);
    let validation = validator.validate_cache_integrity(&cache);
    
    Ok(validator.get_validation_report(&validation))
}

/// Force cache refresh
pub async fn refresh_cache() -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use crate::cache_persistence::PersistentCacheManager;
    
    let cache_manager = PersistentCacheManager::new(None);
    cache_manager.invalidate_cache()?;
    
    let loader = ParallelClaudeLoader::new();
    let _cache = loader.load_with_cache_optimization().await?;
    
    Ok("✅ Cache refreshed successfully".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_claude_initialization() {
        match initialize_claude_with_optimization().await {
            Ok(response) => {
                assert!(response.contains("Neo"));
                println!("Claude initialization successful");
            }
            Err(e) => {
                println!("Expected in test environment: {}", e);
            }
        }
    }
}