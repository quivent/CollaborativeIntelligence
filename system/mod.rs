pub mod instruction_cache;
pub mod parallel_loader;  
pub mod deduplication_engine;
pub mod cache_persistence;
pub mod cache_validator;
pub mod benchmark_suite;
pub mod integration_demo;
pub mod claude_startup;

pub use instruction_cache::{InstructionCache, InstructionDirective, DirectiveCategory, IdentityConfig};
pub use parallel_loader::{ParallelClaudeLoader, StartupSequence, IncrementalUpdater, PerformanceMetrics};
pub use deduplication_engine::{DeduplicationEngine, DeduplicationResult, RedundancyReport};
pub use cache_persistence::{PersistentCacheManager, CacheMetadata, CacheStatistics};
pub use cache_validator::{CacheValidator, ValidationResult, ValidationIssue, RepairResult};
pub use benchmark_suite::{BenchmarkSuite, BenchmarkResults};
pub use integration_demo::{run_integration_demo, run_performance_comparison};
pub use claude_startup::{initialize_claude_with_optimization, validate_cache_health, refresh_cache};