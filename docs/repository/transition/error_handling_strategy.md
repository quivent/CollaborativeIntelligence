# Error Handling Strategy for CI Migration Transition Period

## Overview

This document outlines a comprehensive error handling strategy for the transition period during the migration of CI functionality to an external repository. The strategy ensures robustness, graceful degradation, and a smooth experience for users during this transitional phase.

## Transition Period Challenges

The transition period presents unique challenges that require special error handling considerations:

1. **Dual Implementation**: Both local and external implementations may exist simultaneously
2. **Version Mismatches**: Local and external versions may differ
3. **Path Resolution**: Multiple potential locations for the external implementation
4. **Configuration Conflicts**: Different configuration sources with potentially conflicting settings
5. **User Experience**: Need to provide clear guidance without disrupting workflows
6. **Recovery Mechanisms**: Ability to recover from failures in either implementation

## Core Principles

The error handling strategy is built on these core principles:

1. **Graceful Degradation**: Fall back to local implementation when external fails
2. **Clear Communication**: Provide explicit error messages about what failed and why
3. **Actionable Guidance**: Include steps to resolve issues in error messages
4. **Minimized Disruption**: Prevent errors from blocking critical workflows
5. **Consistent Experience**: Maintain consistent behavior regardless of implementation
6. **Transparency**: Make it clear which implementation is being used

## Error Categories and Handling Strategies

### 1. External Tool Detection Errors

| Error Scenario | Strategy | Implementation |
|----------------|----------|----------------|
| External tool not found | Fall back to local, show rate-limited notice | Check multiple paths, fall back gracefully |
| External tool not executable | Fall back to local, show specific error | Verify permissions before execution |
| External tool path resolution failures | Try alternative paths, fall back to local | Implement multiple detection strategies |
| Environment variable misconfiguration | Use default locations, log warning | Validate environment variables before use |

```rust
/// Detect external CI with robust error handling
pub fn detect_external_ci() -> Result<Option<ExternalCi>> {
    // First check if local implementation is forced
    if let Ok(force_local) = env::var("CI_FORCE_LOCAL") {
        if force_local == "1" || force_local.to_lowercase() == "true" {
            return Ok(None);
        }
    }
    
    // Try multiple detection strategies in sequence
    match detect_external_ci_from_path() {
        Ok(Some(tool)) => {
            // Verify the tool is executable
            if let Err(err) = verify_executable(&tool.path) {
                log::warn!(
                    "Found external CI at {} but it's not executable: {}",
                    tool.path.display(), err
                );
                return Ok(None);
            }
            return Ok(Some(tool));
        }
        Ok(None) => { /* Continue to next strategy */ }
        Err(err) => {
            log::debug!("PATH-based external CI detection failed: {}", err);
            // Don't return error, try next strategy
        }
    }
    
    // Try home directory location
    match detect_external_ci_from_home_dir() {
        Ok(Some(tool)) => return Ok(Some(tool)),
        Ok(None) => { /* Continue to next strategy */ }
        Err(err) => {
            log::debug!("Home directory external CI detection failed: {}", err);
            // Don't return error, try next strategy
        }
    }
    
    // Try environment variable location
    match detect_external_ci_from_env_var() {
        Ok(Some(tool)) => return Ok(Some(tool)),
        Ok(None) => { /* Continue to next strategy */ }
        Err(err) => {
            log::debug!("Environment variable external CI detection failed: {}", err);
            // Don't return error, try next strategy
        }
    }
    
    // No external CI found, but that's not an error
    Ok(None)
}
```

### 2. Version Compatibility Errors

| Error Scenario | Strategy | Implementation |
|----------------|----------|----------------|
| External tool version too old | Show warning, use if compatible or fall back | Version check with minimum version requirement |
| External tool version too new | Show notice, use with compatibility warning | Version check with feature compatibility check |
| Version parsing failure | Log warning, assume compatible | Robust version parsing with fallbacks |
| API incompatibility | Detect during execution, fall back if needed | Runtime compatibility checks |

```rust
/// Check compatibility between local and external implementations
pub fn check_compatibility(external_version: &Version) -> CompatibilityResult {
    let min_compatible_version = Version::parse("0.1.0").unwrap();
    let max_tested_version = Version::parse("0.3.0").unwrap();
    
    if external_version < &min_compatible_version {
        CompatibilityResult::Incompatible(format!(
            "External CI version {} is too old. Minimum supported version is {}.",
            external_version, min_compatible_version
        ))
    } else if external_version > &max_tested_version {
        CompatibilityResult::WarningCompatible(format!(
            "External CI version {} is newer than the maximum tested version {}. Some features may not work as expected.",
            external_version, max_tested_version
        ))
    } else {
        CompatibilityResult::FullyCompatible
    }
}

/// Execute external CI with version compatibility check
pub fn execute_external_ci(cmd: &str, args: &[String], config: &Config) -> Result<()> {
    let external_ci = detect_external_ci()
        .map_err(|e| Error::ExternalCi(format!("Failed to detect external CI: {}", e)))?
        .ok_or_else(|| Error::ExternalCi("External CI not found".into()))?;
    
    // Check version compatibility
    match check_compatibility(&external_ci.version) {
        CompatibilityResult::FullyCompatible => {
            // Proceed normally
        }
        CompatibilityResult::WarningCompatible(warning) => {
            // Show warning but proceed
            if !config.suppress_warnings {
                eprintln!("Warning: {}", warning);
            }
        }
        CompatibilityResult::Incompatible(error) => {
            // Fall back to local implementation
            log::warn!("Incompatible external CI: {}", error);
            return Err(Error::ExternalCi(format!(
                "Incompatible external CI: {}. Falling back to local implementation.",
                error
            )));
        }
    }
    
    // Execute external CI
    let mut command = Command::new(&external_ci.path);
    command.arg(cmd).args(args);
    
    match command.status() {
        Ok(status) if status.success() => Ok(()),
        Ok(status) => Err(Error::ExternalCi(format!(
            "External CI command failed with exit code: {}",
            status.code().unwrap_or(-1)
        ))),
        Err(e) => Err(Error::ExternalCi(format!(
            "Failed to execute external CI: {}",
            e
        ))),
    }
}
```

### 3. Configuration Errors

| Error Scenario | Strategy | Implementation |
|----------------|----------|----------------|
| Conflicting configurations | Use priority order, log conflicts | Define clear precedence rules |
| Missing configuration | Use defaults, show guidance | Define sensible defaults for all settings |
| Invalid configuration | Ignore invalid parts, use defaults | Validate configuration before use |
| Configuration format changes | Automatically convert when possible | Include format converters |

```rust
/// Load configuration with conflict resolution
pub fn load_config() -> Result<Config> {
    // Define prioritized configuration sources
    let mut config = Config::default();
    
    // 1. Load system-wide configuration
    match load_system_config() {
        Ok(system_config) => {
            log::debug!("Loaded system configuration");
            config.merge(system_config);
        }
        Err(err) => {
            log::debug!("Failed to load system configuration: {}", err);
            // Continue with defaults
        }
    }
    
    // 2. Load user configuration (overrides system)
    match load_user_config() {
        Ok(user_config) => {
            log::debug!("Loaded user configuration");
            config.merge(user_config);
        }
        Err(err) => {
            log::debug!("Failed to load user configuration: {}", err);
            // Continue with previous configuration
        }
    }
    
    // 3. Load project configuration (overrides user)
    match load_project_config() {
        Ok(project_config) => {
            log::debug!("Loaded project configuration");
            config.merge(project_config);
        }
        Err(err) => {
            log::debug!("Failed to load project configuration: {}", err);
            // Continue with previous configuration
        }
    }
    
    // 4. Apply environment variables (highest priority)
    apply_environment_variables(&mut config);
    
    // Validate the final configuration
    if let Err(validation_errors) = config.validate() {
        // Log validation errors but use what we can
        log::warn!("Configuration validation errors: {}", validation_errors);
    }
    
    Ok(config)
}
```

### 4. Command Execution Errors

| Error Scenario | Strategy | Implementation |
|----------------|----------|----------------|
| External command failure | Fall back to local, show context | Catch and handle execution errors |
| Permission issues | Show clear error with resolution steps | Check permissions before execution |
| Missing dependencies | Check prerequisites, show installation guide | Dependency validation |
| Timeout issues | Implement timeout, provide diagnostic info | Set reasonable timeouts with context |

```rust
/// Execute a command with automatic fallback
pub fn execute_command<F, G>(
    external_fn: F,
    local_fn: G,
    config: &Config,
) -> Result<()>
where
    F: FnOnce() -> Result<()>,
    G: FnOnce() -> Result<()>,
{
    // Always prefer external implementation unless forced local
    if config.force_local {
        return local_fn();
    }
    
    // Try external implementation first
    match external_fn() {
        Ok(()) => Ok(()),
        Err(err) => {
            // Log the external error
            log::warn!("External implementation failed: {}", err);
            
            // Show fallback notice if not suppressed
            if !config.suppress_notices && should_show_fallback_notice() {
                eprintln!("Notice: External CI command failed, falling back to local implementation.");
                eprintln!("Error details: {}", err);
            }
            
            // Fall back to local implementation
            local_fn()
        }
    }
}
```

### 5. User Experience Errors

| Error Scenario | Strategy | Implementation |
|----------------|----------|----------------|
| Transition notices too frequent | Rate limit notices | Track notice frequency in temp file |
| Unclear error messages | Structured error format with context | Define error message templates |
| Workflow disruption | Non-blocking warnings when possible | Categorize errors by severity |
| Missing guidance | Include documentation links | Add help resources to messages |

```rust
/// Display a rate-limited notice to the user
pub fn show_rate_limited_notice(notice_type: NoticeType, message: &str) -> Result<()> {
    let notice_file = get_notice_tracking_file(notice_type)?;
    
    // Check if we've shown this notice recently
    if let Ok(last_shown) = fs::read_to_string(&notice_file) {
        if let Ok(timestamp) = last_shown.trim().parse::<u64>() {
            let now = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(|e| Error::System(format!("Failed to get system time: {}", e)))?
                .as_secs();
            
            // Only show notice once per day
            if now - timestamp < 24 * 60 * 60 {
                return Ok(());
            }
        }
    }
    
    // Show the notice
    match notice_type {
        NoticeType::Deprecation => {
            eprintln!("DEPRECATION NOTICE: {}", message);
            eprintln!("For more information, visit: https://github.com/anthropics/ci-tools");
        }
        NoticeType::Compatibility => {
            eprintln!("COMPATIBILITY NOTICE: {}", message);
        }
        NoticeType::Fallback => {
            eprintln!("FALLBACK NOTICE: {}", message);
        }
    }
    
    // Update the notice tracking file
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|e| Error::System(format!("Failed to get system time: {}", e)))?
        .as_secs();
    
    fs::write(notice_file, now.to_string())
        .map_err(|e| Error::Io(format!("Failed to update notice tracking file: {}", e)))?;
    
    Ok(())
}
```

### 6. Recovery Mechanism Errors

| Error Scenario | Strategy | Implementation |
|----------------|----------|----------------|
| Local fallback also fails | Provide emergency recovery mode | Add `--emergency-mode` flag |
| Partial execution failure | Provide rollback capabilities | Implement transaction-like operations |
| Data corruption | Backup before changes, provide restore | Create backups of critical files |
| Unrecoverable failure | Provide manual recovery instructions | Include recovery guidance |

```rust
/// Execute a command with automatic fallback and recovery
pub fn execute_with_recovery<F, G, R>(
    external_fn: F,
    local_fn: G,
    recovery_fn: R,
    config: &Config,
) -> Result<()>
where
    F: FnOnce() -> Result<()>,
    G: FnOnce() -> Result<()>,
    R: FnOnce() -> Result<()>,
{
    // Check for emergency mode
    if config.emergency_mode {
        log::info!("Running in emergency mode");
        return recovery_fn();
    }
    
    // Create backup if appropriate
    if config.create_backups {
        create_backup()?;
    }
    
    // Try external implementation first
    let external_result = if !config.force_local {
        external_fn()
    } else {
        Err(Error::Skipped("External implementation skipped due to force_local".into()))
    };
    
    // If external succeeds, we're done
    if external_result.is_ok() {
        return Ok(());
    }
    
    // Log the external error
    log::warn!("External implementation failed: {}", external_result.unwrap_err());
    
    // Try local implementation
    let local_result = local_fn();
    
    // If local succeeds, we're done
    if local_result.is_ok() {
        // Show fallback notice if not suppressed
        if !config.suppress_notices && should_show_fallback_notice() {
            eprintln!("Notice: Used local implementation as fallback.");
        }
        return Ok(());
    }
    
    // Both implementations failed, try recovery
    log::error!(
        "Both external and local implementations failed. External error: {}, Local error: {}",
        external_result.unwrap_err(),
        local_result.unwrap_err()
    );
    
    // Show recovery notice
    eprintln!("Error: Command failed in both external and local implementations.");
    eprintln!("Attempting recovery procedure...");
    
    recovery_fn()
}
```

## Implementation Strategy

### Error Type Definitions

A comprehensive error type system will be implemented to handle all error scenarios:

```rust
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("External CI error: {0}")]
    ExternalCi(String),
    
    #[error("Command error: {0}")]
    Command(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Version error: {0}")]
    Version(String),
    
    #[error("System error: {0}")]
    System(String),
    
    #[error("Operation skipped: {0}")]
    Skipped(String),
    
    #[error("{0}")]
    Other(String),
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::Io(err.to_string())
    }
}

impl From<semver::Error> for Error {
    fn from(err: semver::Error) -> Self {
        Error::Version(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
```

### Error Context Enrichment

Errors will be enriched with context to provide more helpful information:

```rust
/// Add context to an error
pub trait ErrorExt<T> {
    fn with_context<C, F>(self, context: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C;
}

impl<T, E> ErrorExt<T> for std::result::Result<T, E>
where
    E: Into<Error>,
{
    fn with_context<C, F>(self, context: F) -> Result<T>
    where
        C: Display + Send + Sync + 'static,
        F: FnOnce() -> C,
    {
        self.map_err(|err| {
            let err = err.into();
            let context = context();
            match err {
                Error::Io(msg) => Error::Io(format!("{}: {}", context, msg)),
                Error::Config(msg) => Error::Config(format!("{}: {}", context, msg)),
                Error::ExternalCi(msg) => Error::ExternalCi(format!("{}: {}", context, msg)),
                Error::Command(msg) => Error::Command(format!("{}: {}", context, msg)),
                Error::Validation(msg) => Error::Validation(format!("{}: {}", context, msg)),
                Error::Version(msg) => Error::Version(format!("{}: {}", context, msg)),
                Error::System(msg) => Error::System(format!("{}: {}", context, msg)),
                Error::Skipped(msg) => Error::Skipped(format!("{}: {}", context, msg)),
                Error::Other(msg) => Error::Other(format!("{}: {}", context, msg)),
            }
        })
    }
}
```

### Logging Strategy

A structured logging approach will be used to track errors and diagnose issues:

```rust
/// Initialize logging with appropriate levels
pub fn init_logging(config: &Config) -> Result<()> {
    let level = if config.verbose {
        log::LevelFilter::Debug
    } else {
        log::LevelFilter::Info
    };
    
    let mut builder = env_logger::Builder::new();
    builder.filter_level(level);
    
    // Check for CI_LOG_FILE environment variable
    if let Ok(log_file) = std::env::var("CI_LOG_FILE") {
        let file = fs::File::create(&log_file)
            .map_err(|e| Error::Io(format!("Failed to create log file: {}", e)))?;
        builder.target(env_logger::Target::Pipe(Box::new(file)));
    }
    
    builder.init();
    
    Ok(())
}

/// Log an error with appropriate level and context
pub fn log_error(err: &Error, context: &str) {
    match err {
        Error::Io(_) | Error::System(_) => {
            log::error!("{}: {}", context, err);
        }
        Error::ExternalCi(_) | Error::Command(_) => {
            log::warn!("{}: {}", context, err);
        }
        Error::Config(_) | Error::Validation(_) | Error::Version(_) => {
            log::info!("{}: {}", context, err);
        }
        Error::Skipped(_) => {
            log::debug!("{}: {}", context, err);
        }
        Error::Other(_) => {
            log::error!("{}: {}", context, err);
        }
    }
}
```

### User Notification System

A structured notification system will provide consistent user messaging:

```rust
pub enum NoticeType {
    Deprecation,
    Compatibility,
    Fallback,
}

pub enum NoticeSeverity {
    Info,
    Warning,
    Error,
}

pub struct Notice {
    pub notice_type: NoticeType,
    pub severity: NoticeSeverity,
    pub message: String,
    pub help_text: Option<String>,
    pub doc_link: Option<String>,
}

impl Notice {
    pub fn new(notice_type: NoticeType, severity: NoticeSeverity, message: String) -> Self {
        Self {
            notice_type,
            severity,
            message,
            help_text: None,
            doc_link: None,
        }
    }
    
    pub fn with_help(mut self, help_text: String) -> Self {
        self.help_text = Some(help_text);
        self
    }
    
    pub fn with_doc_link(mut self, doc_link: String) -> Self {
        self.doc_link = Some(doc_link);
        self
    }
    
    pub fn display(&self) {
        let prefix = match self.severity {
            NoticeSeverity::Info => "Notice",
            NoticeSeverity::Warning => "Warning",
            NoticeSeverity::Error => "Error",
        };
        
        let type_str = match self.notice_type {
            NoticeType::Deprecation => "DEPRECATION",
            NoticeType::Compatibility => "COMPATIBILITY",
            NoticeType::Fallback => "FALLBACK",
        };
        
        eprintln!("{} {}: {}", prefix, type_str, self.message);
        
        if let Some(ref help_text) = self.help_text {
            eprintln!("Help: {}", help_text);
        }
        
        if let Some(ref doc_link) = self.doc_link {
            eprintln!("Documentation: {}", doc_link);
        }
    }
}
```

## Phase-Specific Error Handling

### Phase 1: Initial Deployment

During initial deployment of the external repository, the focus is on detecting major issues:

1. **Conservative Fallback**: Quickly fall back to local implementation
2. **Minimal Disruption**: Limit user-facing errors and notices
3. **Telemetry**: Gather error data for improvement
4. **Easy Rollback**: Make it easy to disable external implementation

### Phase 2: Stabilization

As the external implementation stabilizes:

1. **Refined Detection**: Improve detection of edge cases
2. **More Detailed Errors**: Provide more specific error messages
3. **Better Recovery**: Implement more sophisticated recovery mechanisms
4. **Encourage Migration**: Start showing deprecation notices for local implementation

### Phase 3: Full Deployment

In the final phase:

1. **Local Implementation Deprecated**: Show clear deprecation notices
2. **Migration Guidance**: Provide detailed migration instructions
3. **Fallback as Last Resort**: Local implementation as emergency fallback only
4. **Full Error Handling**: Complete error handling for all scenarios

## Error Handling for Installation Scenarios

Special consideration for installation scenarios:

### New Installation

For users installing for the first time:

1. **Clear Success/Failure**: Clearly indicate installation success or failure
2. **Verification Step**: Verify installation with simple test command
3. **Configuration Guidance**: Help with initial configuration
4. **Dependency Checks**: Verify all dependencies are available

### Upgrade Installation

For users upgrading:

1. **Version Compatibility Check**: Verify compatibility with existing projects
2. **Configuration Migration**: Migrate configuration if format changed
3. **Backup Creation**: Create backups of critical files before upgrade
4. **Rollback Instructions**: Provide rollback instructions for failures

### Uninstallation

For users uninstalling:

1. **Complete Cleanup**: Ensure all components are removed
2. **Configuration Preservation Option**: Allow preserving configuration
3. **Feedback Collection**: Gather feedback on reasons for uninstallation
4. **Reinstallation Guidance**: Provide guidance for future reinstallation

## Monitoring and Improvement

To continuously improve error handling:

1. **Error Frequency Analysis**: Track most common errors
2. **User Feedback Integration**: Incorporate user feedback into error messages
3. **Error Resolution Time**: Track time from error to resolution
4. **Documentation Updates**: Update documentation based on common issues

## Implementation Checklist

- [ ] Define comprehensive error types
- [ ] Implement context enrichment
- [ ] Create structured logging system
- [ ] Develop user notification system
- [ ] Implement detection fallback mechanisms
- [ ] Create configuration conflict resolution
- [ ] Develop version compatibility checking
- [ ] Implement command execution with recovery
- [ ] Create rate-limited notices
- [ ] Develop clear error display formats
- [ ] Create emergency recovery mode
- [ ] Document common errors and resolutions

## Conclusion

This error handling strategy ensures a smooth transition period by focusing on:

1. **Robustness**: Multiple fallback mechanisms
2. **Clarity**: Clear error messages and guidance
3. **Consistency**: Uniform error handling across implementations
4. **Recovery**: Strong recovery capabilities
5. **User Experience**: Minimal disruption to workflows

By following this strategy, the migration of CI functionality to the external repository can proceed with minimal risk and maximal user satisfaction.