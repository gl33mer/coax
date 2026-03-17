//! Core Scanner Implementation
//!
//! This module provides the main Scanner struct with:
//! - Pattern compilation caching
//! - Parallel file scanning using rayon
//! - Configurable thread pool
//! - Context detection for false positive reduction
//! - Token efficiency filtering (Betterleaks algorithm)
//! - Word filter using Aho-Corasick (Betterleaks algorithm)
//! - Source Provider abstraction for scanning multiple content sources

use crate::context::ContextAnalyzer;
use crate::pattern_cache::{PatternCache, PatternConfig};
use crate::result::{FindingContext, ScanResult, ScanSummary, SeverityCounts};
use crate::secrets;
use crate::source_provider::{
    ContentLoader, ContentType, FileSystemProvider, ScanTarget, SourceProvider, SourceProviderError,
};
use crate::token_efficiency::TokenEfficiencyConfig;
use crate::unicode::{UnicodeConfig, UnicodeScanner};
use crate::word_filter::{WordFilter, WordFilterConfig};
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;
use walkdir::WalkDir;

/// Scanner configuration
#[derive(Debug, Clone)]
pub struct ScannerConfig {
    /// Patterns to use for scanning
    pub patterns: Vec<PatternConfig>,
    /// Maximum file size to scan (bytes), default 10MB
    pub max_file_size: u64,
    /// Files to exclude by pattern
    pub exclude_patterns: Vec<String>,
    /// Number of threads for parallel scanning (0 = auto)
    pub num_threads: usize,
    /// Include line content in results
    pub include_line_content: bool,
    /// Scan hidden files and directories
    pub scan_hidden: bool,
    /// Follow symlinks
    pub follow_symlinks: bool,
    /// Enable context-aware scanning (reduce false positives)
    pub enable_context_detection: bool,
    /// Exclude test files from results
    pub exclude_test_files: bool,
    /// Exclude documentation files from results
    pub exclude_documentation: bool,
    /// Exclude comments from results
    pub exclude_comments: bool,
    /// Exclude placeholder values
    pub exclude_placeholders: bool,
    /// Exclude AWS example keys
    pub exclude_aws_examples: bool,
    /// Enable token efficiency filtering (Betterleaks algorithm)
    pub enable_token_efficiency: bool,
    /// Token efficiency configuration
    pub token_efficiency_config: TokenEfficiencyConfig,
    /// Enable word filter (Betterleaks Aho-Corasick algorithm)
    pub enable_word_filter: bool,
    /// Word filter configuration
    pub word_filter_config: WordFilterConfig,
    /// Load patterns from external YAML files
    pub load_external_patterns: bool,
    /// Pattern directory path for external patterns
    pub pattern_directory: Option<PathBuf>,
    /// Minimum confidence level for external patterns ("low", "medium", "high")
    pub min_confidence: String,
    /// Load secrets-patterns-db patterns
    pub enable_secrets_patterns_db: bool,
    /// Unicode scanning configuration
    pub unicode: UnicodeConfig,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            patterns: secrets::all_patterns(),
            max_file_size: 10 * 1024 * 1024, // 10MB
            exclude_patterns: vec![
                ".git".to_string(),
                "node_modules".to_string(),
                "target".to_string(),
                "build".to_string(),
                "dist".to_string(),
                "vendor".to_string(),
                ".venv".to_string(),
                "__pycache__".to_string(),
                "*.min.js".to_string(),
                "*.bundle.js".to_string(),
                "*.lock".to_string(),
                "Cargo.lock".to_string(),
                "package-lock.json".to_string(),
                "yarn.lock".to_string(),
                "pnpm-lock.yaml".to_string(),
                "go.sum".to_string(),
                "Gemfile.lock".to_string(),
                "composer.lock".to_string(),
            ],
            num_threads: 0, // Auto-detect
            include_line_content: false,
            scan_hidden: false,
            follow_symlinks: false,
            enable_context_detection: true,
            exclude_test_files: true,
            exclude_documentation: true,
            exclude_comments: true,
            exclude_placeholders: true,
            exclude_aws_examples: true,
            enable_token_efficiency: true,
            token_efficiency_config: TokenEfficiencyConfig::default(),
            enable_word_filter: true,
            word_filter_config: WordFilterConfig::default(),
            load_external_patterns: false,
            pattern_directory: None,
            min_confidence: "high".to_string(),
            enable_secrets_patterns_db: false,
            unicode: UnicodeConfig::default(),
        }
    }
}

impl ScannerConfig {
    /// Create a new config with default patterns
    pub fn new() -> Self {
        Self::default()
    }

    /// Create a new config with custom patterns
    pub fn with_patterns(patterns: Vec<PatternConfig>) -> Self {
        Self {
            patterns,
            ..Default::default()
        }
    }

    /// Set the number of threads
    pub fn with_threads(mut self, num_threads: usize) -> Self {
        self.num_threads = num_threads;
        self
    }

    /// Set max file size
    pub fn with_max_file_size(mut self, max_file_size: u64) -> Self {
        self.max_file_size = max_file_size;
        self
    }

    /// Add exclude pattern
    pub fn with_exclude(mut self, pattern: String) -> Self {
        self.exclude_patterns.push(pattern);
        self
    }

    /// Enable line content in results
    pub fn with_line_content(mut self) -> Self {
        self.include_line_content = true;
        self
    }

    /// Enable token efficiency filtering
    pub fn with_token_efficiency(mut self, enabled: bool) -> Self {
        self.enable_token_efficiency = enabled;
        self
    }

    /// Set token efficiency configuration
    pub fn with_token_efficiency_config(mut self, config: TokenEfficiencyConfig) -> Self {
        self.token_efficiency_config = config;
        self
    }

    /// Enable word filter
    pub fn with_word_filter(mut self, enabled: bool) -> Self {
        self.enable_word_filter = enabled;
        self
    }

    /// Set word filter configuration
    pub fn with_word_filter_config(mut self, config: WordFilterConfig) -> Self {
        self.word_filter_config = config;
        self
    }

    /// Enable or disable context detection
    pub fn with_context_detection(mut self, enabled: bool) -> Self {
        self.enable_context_detection = enabled;
        self
    }

    /// Enable loading patterns from external YAML files
    pub fn with_external_patterns(mut self, enabled: bool) -> Self {
        self.load_external_patterns = enabled;
        self
    }

    /// Set the pattern directory for external patterns
    pub fn with_pattern_directory(mut self, path: PathBuf) -> Self {
        self.pattern_directory = Some(path);
        self
    }

    /// Set minimum confidence level for external patterns
    pub fn with_min_confidence(mut self, level: &str) -> Self {
        self.min_confidence = level.to_string();
        self
    }

    /// Enable secrets-patterns-db patterns
    pub fn with_secrets_patterns_db(mut self, enabled: bool) -> Self {
        self.enable_secrets_patterns_db = enabled;
        self
    }

    /// Enable Unicode attack detection
    pub fn with_unicode_enabled(mut self, enabled: bool) -> Self {
        self.unicode.enabled = enabled;
        self
    }

    /// Set Unicode sensitivity level
    pub fn with_unicode_sensitivity(
        mut self,
        sensitivity: crate::unicode::SensitivityLevel,
    ) -> Self {
        self.unicode.sensitivity = sensitivity;
        self
    }
}

/// High-performance security scanner
///
/// Uses pattern caching and parallel scanning for optimal performance.
///
/// # Example
///
/// ```rust
/// use coax_scanner::{Scanner, ScannerConfig};
/// use std::path::PathBuf;
///
/// // Create scanner with default config
/// let scanner = Scanner::new();
///
/// // Or with custom config
/// let config = ScannerConfig::default()
///     .with_threads(4)
///     .with_max_file_size(5 * 1024 * 1024);
/// let scanner = Scanner::with_config(config);
///
/// // Scan a directory
/// let results = scanner.scan_directory(&PathBuf::from("./src"));
/// ```
pub struct Scanner {
    /// Pattern cache (shared across threads)
    pub(crate) config: ScannerConfig,
    /// Pre-compiled patterns (thread-safe)
    pattern_cache: Arc<PatternCache>,
    /// Unicode scanner (optional, based on config)
    unicode_scanner: Option<UnicodeScanner>,
}

impl Scanner {
    /// Create a new scanner with default configuration
    pub fn new() -> Self {
        Self::with_config(ScannerConfig::default())
    }

    /// Create a scanner with default patterns (convenience method)
    pub fn with_default_patterns() -> Self {
        Self::new()
    }

    /// Create a scanner with custom configuration
    pub fn with_config(config: ScannerConfig) -> Self {
        // Load external patterns if configured
        let mut final_config = config.clone();
        if config.load_external_patterns {
            if let Some(ref pattern_dir) = config.pattern_directory {
                if let Ok(loader) =
                    Self::load_patterns_from_directory(pattern_dir, &config.min_confidence)
                {
                    // Merge with existing patterns
                    let mut all_patterns = final_config.patterns.clone();
                    all_patterns.extend(loader.into_patterns());
                    final_config.patterns = all_patterns;
                }
            }
        }

        let pattern_cache = Arc::new(PatternCache::new(&final_config.patterns));

        // Initialize rayon thread pool if specified
        if final_config.num_threads > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(final_config.num_threads)
                .build_global()
                .ok();
        }

        let unicode_scanner = if final_config.unicode.enabled {
            Some(UnicodeScanner::new(final_config.unicode.clone()))
        } else {
            None
        };

        Self {
            config: final_config,
            pattern_cache,
            unicode_scanner,
        }
    }

    /// Load patterns from a directory
    fn load_patterns_from_directory(
        dir: &Path,
        min_confidence: &str,
    ) -> Result<crate::pattern_loader::PatternLoader, crate::pattern_loader::PatternLoaderError>
    {
        use crate::pattern_loader::PatternLoader;

        let mut loader = PatternLoader::new();
        loader.load_from_directory(dir)?;

        // Filter by confidence
        let loader = loader.filter_by_confidence(min_confidence);

        Ok(loader)
    }

    /// Create a scanner with custom patterns
    pub fn with_patterns(patterns: Vec<PatternConfig>) -> Self {
        let config = ScannerConfig::with_patterns(patterns);
        Self::with_config(config)
    }

    /// Get the pattern cache
    pub fn pattern_cache(&self) -> &Arc<PatternCache> {
        &self.pattern_cache
    }

    /// Get the number of patterns
    pub fn pattern_count(&self) -> usize {
        self.pattern_cache.len()
    }

    /// Scan a single file
    pub fn scan_file(&self, path: &Path) -> Vec<ScanResult> {
        scan_file_internal(
            path,
            &self.pattern_cache,
            &self.config,
            self.unicode_scanner.as_ref(),
        )
    }

    /// Scan content directly (for testing or custom use cases)
    pub fn scan_content(&self, content: &str, file_name: &str) -> Vec<ScanResult> {
        scan_content_internal(
            content.to_string(),
            PathBuf::from(file_name),
            &self.pattern_cache,
            &self.config,
            self.unicode_scanner.as_ref(),
        )
    }

    /// Scan multiple files in parallel
    fn scan_files_parallel(&self, files: &[PathBuf]) -> Vec<ScanResult> {
        let cache = Arc::clone(&self.pattern_cache);
        let config = self.config.clone();
        let unicode_scanner = self.unicode_scanner.as_ref().map(|s| s as &UnicodeScanner);

        files
            .par_iter()
            .flat_map(move |path| scan_file_internal(path, &cache, &config, unicode_scanner))
            .collect()
    }

    /// Collect files to scan from a directory
    fn collect_files(&self, root: &Path) -> Vec<PathBuf> {
        let mut files = Vec::new();

        for entry in WalkDir::new(root)
            .into_iter()
            .filter_entry(|e: &walkdir::DirEntry| {
                // First check if path should be excluded
                if self.should_exclude(e.path()) {
                    return false;
                }

                // Then check hidden files (but only for the root directory itself)
                if !self.config.scan_hidden && e.path() != root {
                    if let Some(name) = e.file_name().to_str() {
                        if name.starts_with('.') {
                            return false;
                        }
                    }
                }

                true
            })
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    if metadata.len() <= self.config.max_file_size {
                        // Check if file should be scanned
                        let should_scan = if let Some(ext) = path.extension() {
                            // File has extension - check allowlist
                            should_scan_extension(ext)
                        } else {
                            // File has no extension - check known names or binary check
                            should_scan_extensionless_file(&path)
                        };

                        if should_scan {
                            files.push(path.to_path_buf());
                        }
                    }
                }
            }
        }

        files
    }

    /// Check if a path should be excluded
    fn should_exclude(&self, path: &Path) -> bool {
        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
            for pattern in &self.config.exclude_patterns {
                if pattern.starts_with("*.") {
                    // Extension pattern
                    if name.ends_with(&pattern[1..]) {
                        return true;
                    }
                } else if name == pattern {
                    // Exact match
                    return true;
                }
            }
        }

        false
    }

    /// Get scan summary with performance metrics
    pub fn scan_with_summary(&self, path: &Path) -> (Vec<ScanResult>, ScanSummary) {
        // Use the SourceProvider abstraction internally
        let provider = FileSystemProvider::new(path.to_path_buf())
            .with_max_file_size(self.config.max_file_size)
            .with_skip_binary(true)
            .with_exclude_patterns(self.config.exclude_patterns.clone())
            .with_scan_hidden(self.config.scan_hidden);

        self.scan_source_provider_with_summary(&provider)
    }
}

impl Scanner {
    /// Scan only for Unicode attacks (skip secret detection)
    pub fn scan_unicode_only(&self, path: &Path) -> (Vec<ScanResult>, ScanSummary) {
        let start = Instant::now();
        let files = self.collect_files(path);
        let files_count = files.len();
        let all_results = self.scan_files_parallel(&files);

        // Filter to only Unicode findings
        let unicode_results: Vec<ScanResult> = all_results
            .into_iter()
            .filter(|r| r.pattern.starts_with("UNICODE-"))
            .collect();

        let duration = start.elapsed();

        let summary = ScanSummary {
            files_scanned: files_count as u32,
            total_findings: unicode_results.len() as u32,
            by_severity: SeverityCounts::from_results(&unicode_results),
            top_patterns: get_top_patterns(&unicode_results, 10),
            scan_duration_ms: duration.as_millis() as u64,
        };

        (unicode_results, summary)
    }

    /// Scan using a SourceProvider abstraction
    ///
    /// This method allows scanning from various sources (filesystem, git history, etc.)
    /// by accepting any type that implements the `SourceProvider` and `ContentLoader` traits.
    ///
    /// # Arguments
    ///
    /// * `provider` - An Arc-wrapped SourceProvider that also implements ContentLoader
    ///
    /// # Returns
    ///
    /// A vector of scan results
    ///
    /// # Example
    ///
    /// ```rust
    /// use coax_scanner::{Scanner, FileSystemProvider};
    /// use std::sync::Arc;
    /// use std::path::PathBuf;
    ///
    /// let scanner = Scanner::new();
    /// let provider = Arc::new(FileSystemProvider::new(PathBuf::from("./src")));
    /// let results = scanner.scan_source_provider(&provider);
    /// ```
    pub fn scan_source_provider<P>(&self, provider: &P) -> Vec<ScanResult>
    where
        P: SourceProvider + ContentLoader,
    {
        let start = Instant::now();

        // Collect all targets from the provider
        let targets: Vec<ScanTarget> = provider.enumerate().collect();
        let targets_count = targets.len();

        // Scan targets in parallel
        let results = self.scan_targets_parallel(provider, &targets);

        let duration = start.elapsed();

        // Log performance metrics in debug mode
        tracing::debug!(
            "Scanned {} targets in {:?} ({} patterns)",
            targets_count,
            duration,
            self.pattern_count()
        );

        results
    }

    /// Scan using a SourceProvider with summary
    ///
    /// Similar to `scan_source_provider` but also returns a summary with metrics.
    pub fn scan_source_provider_with_summary<P>(
        &self,
        provider: &P,
    ) -> (Vec<ScanResult>, ScanSummary)
    where
        P: SourceProvider + ContentLoader,
    {
        let start = Instant::now();

        let targets: Vec<ScanTarget> = provider.enumerate().collect();
        let targets_count = targets.len();
        let results = self.scan_targets_parallel(provider, &targets);
        let duration = start.elapsed();

        let summary = ScanSummary {
            files_scanned: targets_count as u32,
            total_findings: results.len() as u32,
            by_severity: SeverityCounts::from_results(&results),
            top_patterns: get_top_patterns(&results, 10),
            scan_duration_ms: duration.as_millis() as u64,
        };

        (results, summary)
    }

    /// Scan multiple targets in parallel
    fn scan_targets_parallel<P>(&self, provider: &P, targets: &[ScanTarget]) -> Vec<ScanResult>
    where
        P: SourceProvider + ContentLoader,
    {
        let cache = Arc::clone(&self.pattern_cache);
        let config = self.config.clone();
        let unicode_scanner = self.unicode_scanner.as_ref().map(|s| s as &UnicodeScanner);

        targets
            .par_iter()
            .flat_map(move |target| {
                // Skip binary content if configured
                if provider.skip_binary() {
                    if let Some(ContentType::Binary) = target.content_type {
                        tracing::debug!("Skipping binary target: {}", target.display_path());
                        return Vec::new();
                    }
                }

                // Skip oversized content
                if let Some(size) = target.size_hint {
                    if size > provider.max_content_size() {
                        tracing::debug!(
                            "Skipping oversized target ({} bytes): {}",
                            size,
                            target.display_path()
                        );
                        return Vec::new();
                    }
                }

                // Load content
                match provider.load(target) {
                    Ok(content) => {
                        // Convert to string for scanning
                        match content.into_string() {
                            Ok(content_str) => scan_content_internal(
                                content_str,
                                PathBuf::from(target.display_path()),
                                &cache,
                                &config,
                                unicode_scanner,
                            ),
                            Err(e) => {
                                tracing::debug!(
                                    "Failed to decode content as UTF-8 for {}: {}",
                                    target.display_path(),
                                    e
                                );
                                Vec::new()
                            }
                        }
                    }
                    Err(e) => {
                        match e {
                            SourceProviderError::BinaryContent(_) => {
                                // Expected, already logged above
                            }
                            SourceProviderError::TooLarge { .. } => {
                                // Expected, already logged above
                            }
                            _ => {
                                tracing::debug!(
                                    "Failed to load target {}: {}",
                                    target.display_path(),
                                    e
                                );
                            }
                        }
                        Vec::new()
                    }
                }
            })
            .collect()
    }

    /// Scan a directory using the SourceProvider abstraction (convenience method)
    ///
    /// This is a convenience wrapper that creates a FileSystemProvider internally.
    /// For more control, use `scan_source_provider` directly.
    pub fn scan_directory(&self, path: &Path) -> Vec<ScanResult> {
        // Use the SourceProvider abstraction internally
        let provider = FileSystemProvider::new(path.to_path_buf())
            .with_max_file_size(self.config.max_file_size)
            .with_skip_binary(true)
            .with_exclude_patterns(self.config.exclude_patterns.clone())
            .with_scan_hidden(self.config.scan_hidden);

        self.scan_source_provider(&provider)
    }
}

impl Default for Scanner {
    fn default() -> Self {
        Self::new()
    }
}

/// Check if a file extension should be scanned
fn should_scan_extension(ext: &std::ffi::OsStr) -> bool {
    matches!(
        ext.to_str(),
        Some(
            // Core programming languages
            "js" | "ts" | "tsx" | "jsx" | "py" | "rs" | "go" | "rb" | "php" |
            "java" | "cs" | "cpp" | "c" | "h" | "hpp" | "kt" | "kts" | "swift" |
            "scala" | "sbt" | "dart" | "r" | "R" | "jl" | "nim" | "hs" | "lhs" |
            "fs" | "fsi" | "fsx" | "elm" | "vue" | "svelte" |
            // Web technologies
            "html" | "css" | "scss" | "less" | "graphql" | "gql" |
            // Config files
            "yml" | "yaml" | "json" | "toml" | "xml" | "ini" | "conf" | "config" |
            "properties" | "plist" | "cfg" | "hcl" | "tf" | "tfvars" | "terraform" |
            "env" | "envrc" | "htaccess" | "htpasswd" |
            // Build/CI files
            "gradle" | "cmake" | "mk" | "bzl" | "bazel" | "makefile" |
            // Data files
            "csv" | "tsv" | "sql" | "sqlite" |
            // Package manager configs
            "npmrc" | "pypirc" | "gemrc" | "yarnrc" | "lock" | "sum" |
            // Notebooks
            "ipynb" | "rmd" |
            // IaC / Config management
            "pp" | "sls" | "erb" |
            // Shell scripts
            "sh" | "bash" | "zsh" | "ps1" | "psm1" | "psd1" | "bat" | "cmd" | "vbs" | "vb" | "lua" |
            // Perl/Raku
            "pl" | "pm" | "t" | "raku" | "rakumod" | "rakutest" |
            // Erlang/Elixir
            "erl" | "hrl" | "ex" | "exs" |
            // Clojure
            "clj" | "cljs" | "edn" |
            // Other
            "md" | "txt" | "proto" | "dockerfile" | "pem" | "key" | "cert" |
            "m" | "mm" | "nix" | "re" | "rei"
        )
    )
}

/// Check if an extensionless file should be scanned
/// Known text-based files without extensions are scanned
/// Unknown extensionless files are checked for binary content
fn should_scan_extensionless_file(path: &Path) -> bool {
    // Known text-based filenames without extensions
    const KNOWN_TEXT_FILES: &[&str] = &[
        "Jenkinsfile",
        "Makefile",
        "Vagrantfile",
        "Dockerfile",
        "Gemfile",
        "Rakefile",
        "Procfile",
        "Brewfile",
        "Berksfile",
        "Podfile",
        ".env",
        ".envrc",
        ".gitignore",
        ".dockerignore",
        ".npmignore",
        ".pypirc",
        "README",
        "LICENSE",
        "CHANGELOG",
        "INSTALL",
        "TODO",
        "NOTICE",
        "AUTHORS",
        "CONTRIBUTORS",
    ];

    // Check if filename matches known text files
    if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
        if KNOWN_TEXT_FILES.contains(&name) {
            return true;
        }
    }

    // For unknown extensionless files, do a quick binary check
    // Read first 512 bytes - if null bytes present, it's binary
    if let Ok(mut file) = std::fs::File::open(path) {
        use std::io::Read;
        let mut buffer = [0u8; 512];
        if let Ok(bytes_read) = file.read(&mut buffer) {
            if bytes_read == 0 {
                return true; // Empty file, treat as text
            }
            // Check for null bytes (binary indicator)
            for &byte in &buffer[..bytes_read] {
                if byte == 0 {
                    return false; // Binary file
                }
            }
            return true; // No null bytes, treat as text
        }
    }

    // Can't read file, default to scanning
    true
}

/// Internal file scanning function
fn scan_file_internal(
    path: &Path,
    cache: &Arc<PatternCache>,
    config: &ScannerConfig,
    unicode_scanner: Option<&UnicodeScanner>,
) -> Vec<ScanResult> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    // 4A: Binary file detection - skip files with null bytes
    if content.contains('\0') {
        return Vec::new();
    }

    scan_content_internal(content, path.to_path_buf(), cache, config, unicode_scanner)
}

/// Check if a pattern name indicates a known secret type (not generic)
/// Known patterns have very low false positive rates and should bypass heuristic filters
fn is_known_secret_pattern(pattern_name: &str) -> bool {
    // AWS patterns
    if pattern_name.starts_with("AWS_") {
        return true;
    }
    // GitHub patterns
    if pattern_name.starts_with("GITHUB_") {
        return true;
    }
    // Stripe patterns
    if pattern_name.starts_with("STRIPE_") {
        return true;
    }
    // Google patterns
    if pattern_name.starts_with("GOOGLE_") {
        return true;
    }
    // Slack patterns
    if pattern_name.starts_with("SLACK_") {
        return true;
    }
    // Twilio patterns
    if pattern_name.starts_with("TWILIO_") {
        return true;
    }
    // Datadog patterns
    if pattern_name.starts_with("DATADOG_") || pattern_name.starts_with("DD_") {
        return true;
    }
    // SendGrid patterns
    if pattern_name.starts_with("SENDGRID_") {
        return true;
    }
    // npm patterns
    if pattern_name.starts_with("NPM_") {
        return true;
    }
    // Private keys
    if pattern_name.contains("PRIVATE_KEY")
        || pattern_name.contains("RSA_")
        || pattern_name.contains("SSH_")
    {
        return true;
    }
    // Database connection strings
    if pattern_name.contains("CONNECTION")
        || pattern_name.contains("MONGO")
        || pattern_name.contains("POSTGRES")
    {
        return true;
    }
    // Payment processors
    if pattern_name.starts_with("PAYPAL_") || pattern_name.starts_with("SQUARE_") {
        return true;
    }
    // Cloud providers
    if pattern_name.starts_with("AZURE_") || pattern_name.starts_with("GCP_") {
        return true;
    }
    // Communication APIs
    if pattern_name.starts_with("MAILGUN_") || pattern_name.starts_with("MAILCHIMP_") {
        return true;
    }
    // AI/ML APIs
    if pattern_name.starts_with("OPENAI_") || pattern_name.starts_with("ANTHROPIC_") {
        return true;
    }
    // XML-specific patterns
    if pattern_name.starts_with("XML_") {
        return true;
    }

    // Generic patterns should go through filters
    false
}

/// 4B: Check if a value appears to be a placeholder rather than a real secret
/// For known secret patterns, only filter exact matches (not substrings)
/// For generic patterns, filter more aggressively
fn is_placeholder_value(value: &str, is_known_pattern: bool) -> bool {
    let lower = value.to_lowercase();

    // Exact placeholder matches - these are always filtered
    const EXACT_PLACEHOLDERS: &[&str] = &[
        "example",
        "test",
        "sample",
        "dummy",
        "fake",
        "placeholder",
        "changeme",
        "change_me",
        "replace_me",
        "your_key_here",
        "your-key-here",
        "xxx",
        "todo",
        "fixme",
        "insert",
        "redacted",
        "none",
        "null",
        "your_api_key",
        "your_secret",
        "your_token",
        "your_password",
        "your_access_key",
        "your_secret_key",
        "insert_here",
        "insert-here",
    ];

    // For known patterns, only filter if the ENTIRE value is a placeholder
    if is_known_pattern {
        for placeholder in EXACT_PLACEHOLDERS {
            if lower == *placeholder {
                return true;
            }
        }
        return false;
    }

    // For generic/unknown patterns, filter more aggressively (substring match)
    for placeholder in EXACT_PLACEHOLDERS {
        if lower.contains(placeholder) {
            return true;
        }
    }

    // Check for all same character (e.g., "AAAAAAA", "0000000")
    if value.len() > 3 {
        let first_char = value.chars().next();
        if let Some(first) = first_char {
            if value.chars().all(|c| c == first) {
                return true;
            }
        }
    }

    // Check for sequential characters (e.g., "1234567890", "abcdefgh")
    if value.len() > 5 {
        let is_sequential = value
            .chars()
            .zip(value.chars().skip(1))
            .all(|(a, b)| (a as u8 + 1 == b as u8) || (a as u8 == b as u8));
        if is_sequential {
            return true;
        }
    }

    false
}

/// 4C/4D/4E: Check if content appears to be non-secret data (PEM certs, encrypted vaults, hashes)
fn is_non_secret_content(value: &str, line: &str) -> bool {
    let lower_line = line.to_lowercase();
    let lower_value = value.to_lowercase();

    // 4C: PEM certificate markers - public certs are not secrets
    // Note: Private keys (BEGIN RSA PRIVATE KEY, BEGIN PRIVATE KEY) should still be flagged
    if lower_line.contains("-----begin certificate-----")
        || lower_line.contains("-----begin public key-----")
    {
        return true;
    }

    // 4D: Encrypted vault formats
    if value.starts_with("ENC[") ||  // Ansible Vault, SOPS
       lower_value.starts_with("$ansible_vault") ||
       lower_value.starts_with("vault:")
    {
        return true;
    }

    // 4E: Hash values with context indicators
    // Only suppress if BOTH conditions are true:
    // 1. Value is hex string (optionally with hash algorithm prefix like sha256:)
    // 2. Context indicates it's a hash (key name contains hash-related words)

    // Check if line contains hash algorithm markers
    let hash_markers: &[&str] = &["sha256:", "sha512:", "sha384:", "sha1:", "md5:"];
    let has_hash_marker = hash_markers
        .iter()
        .any(|m| lower_value.contains(*m) || lower_line.contains(*m));

    if has_hash_marker {
        // Extract hex portion after hash marker
        let hex_value = if let Some(marker) = hash_markers.iter().find(|m| lower_value.contains(*m))
        {
            let idx = lower_value.find(*marker).unwrap() + marker.len();
            &value[idx..]
        } else if let Some(marker) = hash_markers.iter().find(|m| lower_line.contains(*m)) {
            let idx = lower_line.find(*marker).unwrap() + marker.len();
            &line[idx..]
        } else {
            value
        };

        // Remove quotes and whitespace
        let clean_hex =
            hex_value.trim_matches(|c: char| c == '"' || c == '\'' || c.is_whitespace());

        if clean_hex.chars().all(|c| c.is_ascii_hexdigit()) && clean_hex.len() >= 32 {
            const HASH_CONTEXTS: &[&str] = &[
                "hash",
                "digest",
                "checksum",
                "sha256",
                "sha512",
                "sha384",
                "sha1",
                "md5",
                "fingerprint",
                "signature",
                "hmac",
                "integrity",
                "etag",
            ];

            for context in HASH_CONTEXTS {
                if lower_line.contains(context) {
                    return true;
                }
            }
        }
    }

    false
}

/// Internal content scanning function with context detection and Betterleaks filters
///
/// # SCAN PIPELINE — FILTER HIERARCHY
///
/// 1. Extension filter — skips binary/irrelevant file types
/// 2. Binary check — skips files with null bytes in first 512 bytes
/// 3. Pattern matching — ALL known patterns run against content
/// 4. Heuristic filters — entropy, word filter, token efficiency
///    CRITICAL: Known pattern matches BYPASS all heuristic filters.
/// 5. FP suppression — placeholder, hash context, PEM certs, vault encryption
/// 6. File-type context — test/doc exclusion for non-pattern matches only
///
/// Known patterns are PRIVILEGED — they bypass steps 4 and 6.
/// This prevents recall regressions from heuristic filtering.
fn scan_content_internal(
    content: String,
    file: PathBuf,
    cache: &Arc<PatternCache>,
    config: &ScannerConfig,
    unicode_scanner: Option<&UnicodeScanner>,
) -> Vec<ScanResult> {
    let mut results = Vec::new();
    let context_analyzer = ContextAnalyzer {
        exclude_test_files: config.exclude_test_files,
        exclude_documentation: config.exclude_documentation,
        exclude_comments: config.exclude_comments,
        exclude_placeholders: config.exclude_placeholders,
        exclude_aws_examples: config.exclude_aws_examples,
    };

    // Initialize Betterleaks filters if enabled
    let word_filter = if config.enable_word_filter {
        Some(WordFilter::with_min_length(
            config.word_filter_config.min_word_length,
        ))
    } else {
        None
    };

    for (line_num, line) in content.lines().enumerate() {
        // FP REDUCTION: Analyze context FIRST, before pattern matching
        let context = if config.enable_context_detection {
            context_analyzer.analyze(line, &file)
        } else {
            FindingContext::default()
        };

        // CRITICAL FIX: Known secret patterns should NEVER be suppressed by file-type heuristics.
        // A secret is a secret whether it's in a test file, documentation, or /tmp.
        // We check if ANY known pattern matches this line BEFORE deciding to skip.
        let has_known_pattern_match = cache
            .patterns()
            .iter()
            .filter(|p| is_known_secret_pattern(&p.name))
            .any(|p| p.is_match(line));

        // FP REDUCTION: Skip excluded findings EARLY (before pattern matching)
        // BUT: Never skip if a known secret pattern matches this line
        if !has_known_pattern_match
            && config.enable_context_detection
            && context_analyzer.should_exclude(&context)
        {
            continue;
        }

        for pattern in cache.patterns() {
            if pattern.is_match(line) {
                // Extract the potential secret
                let secret = if pattern.extract_secret {
                    crate::context::extract_secret(line, &pattern.name)
                } else {
                    None
                };

                // FIX: Known secret patterns bypass heuristic filters
                // If a string matches a KNOWN pattern (AWS, GitHub, Stripe, etc.),
                // it should NOT be discarded by generic heuristic filters.
                // Heuristic filters are for GENERIC/UNKNOWN strings only.
                let is_known_pattern = is_known_secret_pattern(&pattern.name);

                // CRITICAL: AWS examples and placeholders should ALWAYS be excluded,
                // even for known patterns. These are documentation/examples, not real secrets.
                if context.is_aws_example || context.is_placeholder {
                    tracing::debug!(
                        "Filtered by context (AWS example/placeholder): {} in {}:{}",
                        pattern.name,
                        file.display(),
                        line_num + 1
                    );
                    continue;
                }

                // FP REDUCTION: Apply entropy pre-filter ONLY for unknown patterns
                if !is_known_pattern {
                    if let Some(ref secret_value) = secret {
                        if crate::token_efficiency::is_likely_false_positive(secret_value) {
                            tracing::debug!(
                                "Filtered by entropy pre-filter: {} in {}:{}",
                                pattern.name,
                                file.display(),
                                line_num + 1
                            );
                            continue;
                        }
                    }
                }

                // Apply Betterleaks Token Efficiency Filter ONLY for unknown patterns
                if !is_known_pattern && config.enable_token_efficiency {
                    if let Some(ref secret_value) = secret {
                        if !config.token_efficiency_config.passes_filter(secret_value) {
                            tracing::debug!(
                                "Filtered by token efficiency: {} in {}:{}",
                                pattern.name,
                                file.display(),
                                line_num + 1
                            );
                            continue;
                        }
                    }
                }

                // Apply Betterleaks Word Filter ONLY for unknown patterns
                if !is_known_pattern {
                    if let Some(ref _filter) = word_filter {
                        if let Some(ref secret_value) = secret {
                            if !config.word_filter_config.passes_filter(secret_value) {
                                tracing::debug!(
                                    "Filtered by word filter: {} in {}:{}",
                                    pattern.name,
                                    file.display(),
                                    line_num + 1
                                );
                                continue;
                            }
                        }
                    }
                }

                // 4B: Placeholder detection - suppress findings with placeholder values
                // For known patterns, only filter exact matches (not substrings)
                // For generic patterns, filter more aggressively
                // Check both extracted secret value AND the full line content
                let should_filter_placeholder = if let Some(ref secret_value) = secret {
                    is_placeholder_value(secret_value, is_known_pattern)
                } else {
                    // If no secret was extracted, check the line content itself
                    is_placeholder_value(line, is_known_pattern)
                };

                if should_filter_placeholder {
                    tracing::debug!(
                        "Filtered by placeholder detection: {} in {}:{}",
                        pattern.name,
                        file.display(),
                        line_num + 1
                    );
                    continue;
                }

                // 4C/4D/4E: Non-secret content detection (PEM certs, encrypted vaults, hashes)
                // This runs for ALL patterns (known and unknown)
                let should_filter_non_secret = if let Some(ref secret_value) = secret {
                    is_non_secret_content(secret_value, line)
                } else {
                    // If no secret was extracted, check the line content itself
                    is_non_secret_content(line, line)
                };

                if should_filter_non_secret {
                    tracing::debug!(
                        "Filtered by non-secret content detection: {} in {}:{}",
                        pattern.name,
                        file.display(),
                        line_num + 1
                    );
                    continue;
                }

                let mut result = ScanResult::new(
                    file.clone(),
                    (line_num + 1) as u32,
                    pattern.name.to_string(),
                    pattern.severity.to_string(),
                    pattern.recommendation.to_string(),
                );

                // Add description and CWE ID from pattern metadata
                if let Some(ref desc) = pattern.description {
                    result = result.with_description(desc.as_ref());
                }
                if let Some(ref cwe) = pattern.cwe_id {
                    result = result.with_cwe_id(cwe.as_ref());
                }

                // Add detected secret if pattern supports extraction
                if pattern.extract_secret {
                    if let Some(secret_value) = &secret {
                        result = result.with_detected_secret(secret_value.clone());
                    }
                }

                // Add line content if requested
                if config.include_line_content {
                    result = result.with_line_content(line.trim().to_string());
                }

                // CRITICAL: Known secret patterns bypass documentation/test file exclusion
                // A real secret pattern match is a finding regardless of file type
                let mut final_context = context.clone();
                if is_known_pattern {
                    // Clear exclusion for known patterns
                    if final_context.is_documentation || final_context.is_test_file {
                        final_context.adjusted_severity = None;
                        final_context.note = Some(format!(
                            "Known pattern {} - bypasses file-type exclusion",
                            pattern.name
                        ));
                    }
                }

                // Add context
                result = result.with_context(final_context);

                // Add Betterleaks filter metadata
                if config.enable_token_efficiency || config.enable_word_filter {
                    let mut notes = Vec::new();
                    if config.enable_token_efficiency {
                        notes.push("token_efficiency=enabled");
                    }
                    if config.enable_word_filter {
                        notes.push("word_filter=enabled");
                    }
                    if let Some(existing_note) = &result.context.note {
                        notes.push(existing_note.as_str());
                    }
                    result.context.note = Some(notes.join(", "));
                }

                results.push(result);
            }
        }
    }

    // Unicode scanning
    let mut all_results = results;
    if let Some(scanner) = unicode_scanner {
        let unicode_findings = scanner.scan(&content, file.to_str().unwrap_or(""));
        for finding in unicode_findings {
            all_results.push(finding.to_scan_result());
        }
    }

    all_results
}

/// Get top patterns by occurrence count
fn get_top_patterns(results: &[ScanResult], limit: usize) -> Vec<crate::result::PatternCount> {
    use std::collections::HashMap;

    let mut counts: HashMap<String, u32> = HashMap::new();
    for result in results {
        *counts.entry(result.pattern.clone()).or_insert(0) += 1;
    }

    let mut pattern_counts: Vec<crate::result::PatternCount> = counts
        .into_iter()
        .map(|(pattern, count)| crate::result::PatternCount { pattern, count })
        .collect();

    pattern_counts.sort_by(|a, b| b.count.cmp(&a.count));
    pattern_counts.truncate(limit);
    pattern_counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::new();
        assert!(scanner.pattern_count() > 30);
    }

    #[test]
    fn test_scanner_with_custom_patterns() {
        let patterns = vec![PatternConfig::new("TEST", r"test\d+", "low", "Test")];

        let scanner = Scanner::with_patterns(patterns);
        assert_eq!(scanner.pattern_count(), 1);
    }

    #[test]
    fn test_scan_directory() {
        let temp_dir = TempDir::new().unwrap();
        // Disable filters and context detection for this test
        let scanner = Scanner::with_config(
            ScannerConfig::default()
                .with_token_efficiency(false)
                .with_word_filter(false)
                .with_context_detection(false),
        );

        // Create test files
        let clean_file = temp_dir.path().join("clean.rs");
        fs::write(&clean_file, "clean content").unwrap();

        let secret_file = temp_dir.path().join("secret.rs");
        fs::write(&secret_file, "AWS_KEY=AKIAIOSFODNN7EXAMPLE").unwrap();

        let results = scanner.scan_directory(temp_dir.path());
        assert_eq!(results.len(), 1);
        assert!(results[0].pattern.contains("AWS"));
    }

    #[test]
    fn test_scan_content() {
        // Disable filters and context detection for this test
        let scanner = Scanner::with_config(
            ScannerConfig::default()
                .with_token_efficiency(false)
                .with_word_filter(false)
                .with_context_detection(false),
        );
        let content = "AWS_KEY=AKIAIOSFODNN7EXAMPLE";
        let results = scanner.scan_content(content, "test.txt");

        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.pattern.contains("AWS")));
    }

    #[test]
    fn test_parallel_scanning_performance() {
        let temp_dir = TempDir::new().unwrap();
        // Disable filters and context detection for this test
        let scanner = Scanner::with_config(
            ScannerConfig::default()
                .with_token_efficiency(false)
                .with_word_filter(false)
                .with_context_detection(false),
        );

        // Create 100 test files
        for i in 0..100 {
            let file = temp_dir.path().join(format!("file_{}.txt", i));
            let content = if i % 10 == 0 {
                "AWS_KEY=AKIAIOSFODNN7EXAMPLE"
            } else {
                "clean content here"
            };
            fs::write(&file, content).unwrap();
        }

        let start = Instant::now();
        let results = scanner.scan_directory(temp_dir.path());
        let duration = start.elapsed();

        // Should find 10 secrets (files 0, 10, 20, etc.)
        assert_eq!(results.len(), 10);

        // Should complete in reasonable time (< 1 second for 100 files)
        assert!(
            duration.as_millis() < 1000,
            "Scan took too long: {:?}",
            duration
        );
    }

    #[test]
    fn test_scan_with_summary() {
        let temp_dir = TempDir::new().unwrap();
        // Disable filters and context detection for this test
        let scanner = Scanner::with_config(
            ScannerConfig::default()
                .with_token_efficiency(false)
                .with_word_filter(false)
                .with_context_detection(false),
        );

        // Create test files
        for i in 0..5 {
            let file = temp_dir.path().join(format!("file_{}.txt", i));
            let content = if i < 2 {
                "AWS_KEY=AKIAIOSFODNN7EXAMPLE"
            } else {
                "clean content"
            };
            fs::write(&file, content).unwrap();
        }

        let (results, summary) = scanner.scan_with_summary(temp_dir.path());

        assert_eq!(results.len(), 2);
        assert_eq!(summary.files_scanned, 5);
        assert_eq!(summary.total_findings, 2);
        assert!(summary.scan_duration_ms < 1000);
    }

    #[test]
    fn test_exclude_patterns() {
        let temp_dir = TempDir::new().unwrap();
        let scanner = Scanner::new();

        // Create files in excluded directory
        let git_dir = temp_dir.path().join(".git");
        fs::create_dir(&git_dir).unwrap();
        let git_file = git_dir.join("config");
        fs::write(&git_file, "AWS_KEY=AKIAIOSFODNN7EXAMPLE").unwrap();

        let results = scanner.scan_directory(temp_dir.path());
        assert!(results.is_empty()); // .git should be excluded
    }

    #[test]
    fn test_max_file_size() {
        let config = ScannerConfig::default().with_max_file_size(100); // 100 bytes
        let scanner = Scanner::with_config(config);

        let temp_dir = TempDir::new().unwrap();
        let large_file = temp_dir.path().join("large.txt");
        fs::write(&large_file, "x".repeat(1000)).unwrap(); // 1000 bytes

        let results = scanner.scan_directory(temp_dir.path());
        assert!(results.is_empty()); // Large file should be skipped
    }

    #[test]
    fn test_should_scan_extension() {
        // Test newly added extensions
        assert!(should_scan_extension(std::ffi::OsStr::new("csv")));
        assert!(should_scan_extension(std::ffi::OsStr::new("gradle")));
        assert!(should_scan_extension(std::ffi::OsStr::new("ipynb")));
        assert!(should_scan_extension(std::ffi::OsStr::new("npmrc")));
        assert!(should_scan_extension(std::ffi::OsStr::new("pp")));
        assert!(should_scan_extension(std::ffi::OsStr::new("sls")));

        // Test existing extensions still work
        assert!(should_scan_extension(std::ffi::OsStr::new("py")));
        assert!(should_scan_extension(std::ffi::OsStr::new("js")));
        assert!(should_scan_extension(std::ffi::OsStr::new("json")));
        assert!(should_scan_extension(std::ffi::OsStr::new("yml")));

        // Test binary extensions are still rejected
        assert!(!should_scan_extension(std::ffi::OsStr::new("jpg")));
        assert!(!should_scan_extension(std::ffi::OsStr::new("png")));
        assert!(!should_scan_extension(std::ffi::OsStr::new("exe")));
        assert!(!should_scan_extension(std::ffi::OsStr::new("zip")));
        assert!(!should_scan_extension(std::ffi::OsStr::new("wasm")));
        assert!(!should_scan_extension(std::ffi::OsStr::new("bin")));
    }

    #[test]
    fn test_should_scan_extensionless_file() {
        use std::io::Write;

        let temp_dir = TempDir::new().unwrap();

        // Test known text files
        let jenkinsfile = temp_dir.path().join("Jenkinsfile");
        let mut f = std::fs::File::create(&jenkinsfile).unwrap();
        writeln!(f, "pipeline {{ }}").unwrap();
        assert!(should_scan_extensionless_file(&jenkinsfile));

        let makefile = temp_dir.path().join("Makefile");
        let mut f = std::fs::File::create(&makefile).unwrap();
        writeln!(f, "all: build").unwrap();
        assert!(should_scan_extensionless_file(&makefile));

        let vagrantfile = temp_dir.path().join("Vagrantfile");
        let mut f = std::fs::File::create(&vagrantfile).unwrap();
        writeln!(f, "Vagrant.configure").unwrap();
        assert!(should_scan_extensionless_file(&vagrantfile));

        // Test binary file detection
        let binary_file = temp_dir.path().join("binary_no_ext");
        let mut f = std::fs::File::create(&binary_file).unwrap();
        f.write_all(&[0x00, 0x01, 0x02, 0x03]).unwrap(); // Null bytes = binary
        assert!(!should_scan_extensionless_file(&binary_file));

        // Test text file without extension
        let text_file = temp_dir.path().join("text_no_ext");
        let mut f = std::fs::File::create(&text_file).unwrap();
        writeln!(f, "This is plain text").unwrap();
        assert!(should_scan_extensionless_file(&text_file));
    }
}
