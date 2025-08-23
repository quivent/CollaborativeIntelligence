use std::path::{Path, PathBuf};
use std::fs;
use std::io::{Write, Read};
use std::time::{SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use flate2::{Compression, write::GzEncoder, read::GzDecoder};
use crate::instruction_cache::InstructionCache;

#[derive(Debug, Serialize, Deserialize)]
pub struct CacheMetadata {
    pub version: String,
    pub created_at: SystemTime,
    pub last_accessed: SystemTime,
    pub file_count: usize,
    pub compression_ratio: f32,
    pub checksum: String,
}

pub struct PersistentCacheManager {
    cache_dir: PathBuf,
    compression_enabled: bool,
    retention_days: u64,
}

impl PersistentCacheManager {
    pub fn new(cache_dir: Option<&str>) -> Self {
        let cache_dir = cache_dir
            .map(PathBuf::from)
            .unwrap_or_else(|| {
                let mut path = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
                path.push(".claude");
                path.push("cache");
                path
            });

        Self {
            cache_dir,
            compression_enabled: true,
            retention_days: 30,
        }
    }

    pub fn ensure_cache_directory(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if !self.cache_dir.exists() {
            fs::create_dir_all(&self.cache_dir)?;
            println!("Created cache directory: {:?}", self.cache_dir);
        }
        Ok(())
    }

    pub fn save_cache_with_metadata(&self, cache: &InstructionCache) -> Result<CacheMetadata, Box<dyn std::error::Error + Send + Sync>> {
        self.ensure_cache_directory()?;

        let cache_file = self.cache_dir.join("instruction_cache.bin");
        let metadata_file = self.cache_dir.join("cache_metadata.json");

        // Serialize cache
        let serialized_cache = bincode::serialize(cache)?;
        let original_size = serialized_cache.len();

        // Save with or without compression
        let final_size = if self.compression_enabled {
            self.save_compressed(&cache_file, &serialized_cache)?
        } else {
            fs::write(&cache_file, &serialized_cache)?;
            serialized_cache.len()
        };

        // Generate checksum
        let checksum = self.compute_checksum(&serialized_cache);

        // Create metadata
        let metadata = CacheMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: SystemTime::now(),
            last_accessed: SystemTime::now(),
            file_count: cache.file_hashes.len(),
            compression_ratio: if original_size > 0 {
                final_size as f32 / original_size as f32
            } else {
                1.0
            },
            checksum,
        };

        // Save metadata
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_file, metadata_json)?;

        println!("Cache saved: {} bytes -> {} bytes ({:.1}% compression)", 
                 original_size, final_size, (1.0 - metadata.compression_ratio) * 100.0);

        Ok(metadata)
    }

    pub fn load_cache_with_validation(&self) -> Result<InstructionCache, Box<dyn std::error::Error + Send + Sync>> {
        let cache_file = self.cache_dir.join("instruction_cache.bin");
        let metadata_file = self.cache_dir.join("cache_metadata.json");

        // Check if cache exists
        if !cache_file.exists() || !metadata_file.exists() {
            return Err("Cache files not found".into());
        }

        // Load and validate metadata
        let metadata_json = fs::read_to_string(&metadata_file)?;
        let mut metadata: CacheMetadata = serde_json::from_str(&metadata_json)?;

        // Check cache age
        if self.is_cache_expired(&metadata)? {
            return Err("Cache expired".into());
        }

        // Load cache data
        let serialized_cache = if self.compression_enabled {
            self.load_compressed(&cache_file)?
        } else {
            fs::read(&cache_file)?
        };

        // Validate checksum
        let current_checksum = self.compute_checksum(&serialized_cache);
        if current_checksum != metadata.checksum {
            return Err("Cache checksum validation failed".into());
        }

        // Deserialize cache
        let cache: InstructionCache = bincode::deserialize(&serialized_cache)?;

        // Update last accessed time
        metadata.last_accessed = SystemTime::now();
        let metadata_json = serde_json::to_string_pretty(&metadata)?;
        fs::write(&metadata_file, metadata_json)?;

        println!("Cache loaded successfully with {} directives", cache.compiled_directives.len());

        Ok(cache)
    }

    fn save_compressed(&self, file_path: &Path, data: &[u8]) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let file = fs::File::create(file_path)?;
        let mut encoder = GzEncoder::new(file, Compression::default());
        encoder.write_all(data)?;
        let compressed_file = encoder.finish()?;
        
        Ok(compressed_file.metadata()?.len() as usize)
    }

    fn load_compressed(&self, file_path: &Path) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let file = fs::File::open(file_path)?;
        let mut decoder = GzDecoder::new(file);
        let mut buffer = Vec::new();
        decoder.read_to_end(&mut buffer)?;
        Ok(buffer)
    }

    fn compute_checksum(&self, data: &[u8]) -> String {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    fn is_cache_expired(&self, metadata: &CacheMetadata) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
        let cache_age = SystemTime::now().duration_since(metadata.created_at)?;
        let retention_duration = std::time::Duration::from_secs(self.retention_days * 24 * 60 * 60);
        Ok(cache_age > retention_duration)
    }

    pub fn cleanup_old_caches(&self) -> Result<usize, Box<dyn std::error::Error + Send + Sync>> {
        let mut cleaned_count = 0;
        
        if !self.cache_dir.exists() {
            return Ok(0);
        }

        for entry in fs::read_dir(&self.cache_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_file() {
                if let Some(file_name) = path.file_name() {
                    if file_name.to_string_lossy().starts_with("instruction_cache") {
                        // Check if this cache is expired by looking at file modified time
                        let metadata = fs::metadata(&path)?;
                        if let Ok(modified) = metadata.modified() {
                            let age = SystemTime::now().duration_since(modified)?;
                            let retention_duration = std::time::Duration::from_secs(self.retention_days * 24 * 60 * 60);
                            
                            if age > retention_duration {
                                fs::remove_file(&path)?;
                                cleaned_count += 1;
                                println!("Cleaned expired cache: {:?}", path);
                            }
                        }
                    }
                }
            }
        }

        Ok(cleaned_count)
    }

    pub fn get_cache_statistics(&self) -> Result<CacheStatistics, Box<dyn std::error::Error + Send + Sync>> {
        let cache_file = self.cache_dir.join("instruction_cache.bin");
        let metadata_file = self.cache_dir.join("cache_metadata.json");

        let mut stats = CacheStatistics {
            exists: cache_file.exists() && metadata_file.exists(),
            size_bytes: 0,
            compressed_size_bytes: 0,
            compression_ratio: 0.0,
            age_seconds: 0,
            file_count: 0,
            last_accessed: None,
        };

        if !stats.exists {
            return Ok(stats);
        }

        // Get file sizes
        if let Ok(cache_metadata) = fs::metadata(&cache_file) {
            stats.compressed_size_bytes = cache_metadata.len();
        }

        // Load metadata for detailed stats
        if let Ok(metadata_json) = fs::read_to_string(&metadata_file) {
            if let Ok(metadata) = serde_json::from_str::<CacheMetadata>(&metadata_json) {
                stats.compression_ratio = metadata.compression_ratio;
                stats.file_count = metadata.file_count;
                stats.last_accessed = Some(metadata.last_accessed);
                
                if let Ok(age) = SystemTime::now().duration_since(metadata.created_at) {
                    stats.age_seconds = age.as_secs();
                }
            }
        }

        Ok(stats)
    }

    pub fn invalidate_cache(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let cache_file = self.cache_dir.join("instruction_cache.bin");
        let metadata_file = self.cache_dir.join("cache_metadata.json");

        if cache_file.exists() {
            fs::remove_file(&cache_file)?;
        }
        if metadata_file.exists() {
            fs::remove_file(&metadata_file)?;
        }

        println!("Cache invalidated successfully");
        Ok(())
    }

    pub fn backup_cache(&self, backup_name: Option<&str>) -> Result<PathBuf, Box<dyn std::error::Error + Send + Sync>> {
        let cache_file = self.cache_dir.join("instruction_cache.bin");
        let metadata_file = self.cache_dir.join("cache_metadata.json");

        if !cache_file.exists() || !metadata_file.exists() {
            return Err("No cache to backup".into());
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)?
            .as_secs();
        
        let default_backup_name = format!("backup_{}", timestamp);
        let backup_name = backup_name.unwrap_or(&default_backup_name);
        let backup_dir = self.cache_dir.join("backups").join(backup_name);
        
        fs::create_dir_all(&backup_dir)?;
        
        fs::copy(&cache_file, backup_dir.join("instruction_cache.bin"))?;
        fs::copy(&metadata_file, backup_dir.join("cache_metadata.json"))?;

        println!("Cache backed up to: {:?}", backup_dir);
        Ok(backup_dir)
    }
}

#[derive(Debug)]
pub struct CacheStatistics {
    pub exists: bool,
    pub size_bytes: u64,
    pub compressed_size_bytes: u64,
    pub compression_ratio: f32,
    pub age_seconds: u64,
    pub file_count: usize,
    pub last_accessed: Option<SystemTime>,
}

impl CacheStatistics {
    pub fn format_summary(&self) -> String {
        if !self.exists {
            return "Cache: Not found".to_string();
        }

        let age_hours = self.age_seconds / 3600;
        let compression_pct = (1.0 - self.compression_ratio) * 100.0;
        
        format!(
            "Cache Statistics:\n\
            • Status: Active\n\
            • Size: {} bytes (compressed: {} bytes)\n\
            • Compression: {:.1}% space savings\n\
            • Age: {} hours\n\
            • Files tracked: {}\n\
            • Last accessed: {}",
            self.size_bytes,
            self.compressed_size_bytes,
            compression_pct,
            age_hours,
            self.file_count,
            if let Some(accessed) = self.last_accessed {
                format!("{:.1} hours ago", 
                       SystemTime::now().duration_since(accessed)
                           .unwrap_or_default().as_secs() as f32 / 3600.0)
            } else {
                "Unknown".to_string()
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_cache_persistence() {
        let temp_dir = tempdir().unwrap();
        let cache_manager = PersistentCacheManager::new(Some(temp_dir.path().to_str().unwrap()));
        
        // Create a test cache
        let mut cache = InstructionCache::new();
        cache.file_hashes.insert("test.md".to_string(), "hash123".to_string());
        
        // Save cache
        let metadata = cache_manager.save_cache_with_metadata(&cache).unwrap();
        assert!(metadata.file_count > 0);
        
        // Load cache
        let loaded_cache = cache_manager.load_cache_with_validation().unwrap();
        assert_eq!(loaded_cache.file_hashes.len(), cache.file_hashes.len());
    }

    #[test]
    fn test_cache_statistics() {
        let temp_dir = tempdir().unwrap();
        let cache_manager = PersistentCacheManager::new(Some(temp_dir.path().to_str().unwrap()));
        
        let stats = cache_manager.get_cache_statistics().unwrap();
        assert!(!stats.exists); // Should not exist initially
        
        // Create and save a cache
        let cache = InstructionCache::new();
        cache_manager.save_cache_with_metadata(&cache).unwrap();
        
        let stats = cache_manager.get_cache_statistics().unwrap();
        assert!(stats.exists);
        assert!(stats.compressed_size_bytes > 0);
    }

    #[test]
    fn test_cache_cleanup() {
        let temp_dir = tempdir().unwrap();
        let mut cache_manager = PersistentCacheManager::new(Some(temp_dir.path().to_str().unwrap()));
        cache_manager.retention_days = 0; // Force immediate expiration for testing
        
        // Create a cache
        let cache = InstructionCache::new();
        cache_manager.save_cache_with_metadata(&cache).unwrap();
        
        // Wait a bit to ensure age > 0
        std::thread::sleep(std::time::Duration::from_millis(10));
        
        // Cleanup should remove the expired cache
        let cleaned = cache_manager.cleanup_old_caches().unwrap();
        assert!(cleaned > 0);
    }
}