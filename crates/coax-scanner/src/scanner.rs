//! Core Scanner Implementation
//!
//! This module provides the main Scanner struct with:
//! - Pattern compilation caching
//! - Parallel file scanning using rayon
//! - Configurable thread pool
//! - Context detection for false positive reduction

use crate::pattern_cache::{PatternCache, PatternConfig};
use crate::result::{ScanResult, ScanSummary, SeverityCounts};
use crate::secrets;
use crate::context::ContextAnalyzer;
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
        let pattern_cache = Arc::new(PatternCache::new(&config.patterns));

        // Initialize rayon thread pool if specified
        if config.num_threads > 0 {
            rayon::ThreadPoolBuilder::new()
                .num_threads(config.num_threads)
                .build_global()
                .ok();
        }

        Self {
            config,
            pattern_cache,
        }
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

    /// Scan a directory for secrets
    ///
    /// Uses parallel file scanning for optimal performance.
    pub fn scan_directory(&self, path: &Path) -> Vec<ScanResult> {
        let start = Instant::now();

        // Collect all files to scan
        let files = self.collect_files(path);

        // Scan files in parallel
        let results = self.scan_files_parallel(&files);

        let duration = start.elapsed();

        // Log performance metrics in debug mode
        tracing::debug!(
            "Scanned {} files in {:?} ({} patterns)",
            files.len(),
            duration,
            self.pattern_count()
        );

        results
    }

    /// Scan a single file
    pub fn scan_file(&self, path: &Path) -> Vec<ScanResult> {
        scan_file_internal(path, &self.pattern_cache, &self.config)
    }

    /// Scan content directly (for testing or custom use cases)
    pub fn scan_content(&self, content: &str, file_name: &str) -> Vec<ScanResult> {
        scan_content_internal(
            content.to_string(),
            PathBuf::from(file_name),
            &self.pattern_cache,
            &self.config,
        )
    }

    /// Scan multiple files in parallel
    fn scan_files_parallel(&self, files: &[PathBuf]) -> Vec<ScanResult> {
        let cache = Arc::clone(&self.pattern_cache);
        let config = self.config.clone();

        files
            .par_iter()
            .flat_map(move |path| {
                scan_file_internal(path, &cache, &config)
            })
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
                        if let Some(ext) = path.extension() {
                            if should_scan_extension(ext) {
                                files.push(path.to_path_buf());
                            }
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
        let start = Instant::now();
        let files = self.collect_files(path);
        let files_count = files.len();
        let results = self.scan_files_parallel(&files);
        let duration = start.elapsed();

        let summary = ScanSummary {
            files_scanned: files_count as u32,
            total_findings: results.len() as u32,
            by_severity: SeverityCounts::from_results(&results),
            top_patterns: get_top_patterns(&results, 10),
            scan_duration_ms: duration.as_millis() as u64,
        };

        (results, summary)
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
            "yml" | "yaml" | "json" | "env" | "js" | "py" | "rs" | "toml" | "xml" |
            "properties" | "conf" | "config" | "ini" | "sh" | "bash" | "zsh" |
            "tf" | "terraform" | "md" | "txt" | "ts" | "tsx" | "jsx" | "go" |
            "rb" | "php" | "java" | "cs" | "cpp" | "c" | "h" | "hpp" | "sql" |
            "graphql" | "proto" | "dockerfile" | "pem" | "key" | "cert" |
            "html" | "css" | "scss" | "less" | "vue" | "svelte" | "dart" |
            "kt" | "kts" | "swift" | "m" | "mm" | "r" | "R" | "jl" | "scala" |
            "sbt" | "ex" | "exs" | "erl" | "hrl" | "clj" | "cljs" | "edn" |
            "hs" | "lhs" | "fs" | "fsi" | "fsx" | "elm" |
            "ps1" | "psm1" | "psd1" | "bat" | "cmd" | "vbs" | "vb" | "lua" |
            "pl" | "pm" | "t" | "raku" | "rakumod" | "rakutest" | "nim" |
            "nix" | "re" | "rei" | "gql" | "lock" | "sum"
        )
    )
}

/// Internal file scanning function
fn scan_file_internal(
    path: &Path,
    cache: &Arc<PatternCache>,
    config: &ScannerConfig,
) -> Vec<ScanResult> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(_) => return Vec::new(),
    };

    scan_content_internal(content, path.to_path_buf(), cache, config)
}

/// Internal content scanning function with context detection
fn scan_content_internal(
    content: String,
    file: PathBuf,
    cache: &Arc<PatternCache>,
    config: &ScannerConfig,
) -> Vec<ScanResult> {
    let mut results = Vec::new();
    let context_analyzer = ContextAnalyzer {
        exclude_test_files: config.exclude_test_files,
        exclude_documentation: config.exclude_documentation,
        exclude_comments: config.exclude_comments,
        exclude_placeholders: config.exclude_placeholders,
        exclude_aws_examples: config.exclude_aws_examples,
    };

    for (line_num, line) in content.lines().enumerate() {
        for pattern in cache.patterns() {
            if pattern.is_match(line) {
                // Analyze context
                let context = context_analyzer.analyze(line, &file);

                // Skip excluded findings
                if context_analyzer.should_exclude(&context) {
                    continue;
                }

                let mut result = ScanResult::new(
                    file.clone(),
                    (line_num + 1) as u32,
                    pattern.name.to_string(),
                    pattern.severity.to_string(),
                    pattern.recommendation.to_string(),
                );

                // Add detected secret if pattern supports extraction
                if pattern.extract_secret {
                    if let Some(secret) = crate::context::extract_secret(line, &pattern.name) {
                        result = result.with_detected_secret(secret);
                    }
                }

                // Add line content if requested
                if config.include_line_content {
                    result = result.with_line_content(line.trim().to_string());
                }

                // Add context
                result = result.with_context(context);

                results.push(result);
            }
        }
    }

    results
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
        let patterns = vec![PatternConfig {
            name: "TEST",
            pattern: r"test\d+",
            severity: "low",
            recommendation: "Test",
        }];

        let scanner = Scanner::with_patterns(patterns);
        assert_eq!(scanner.pattern_count(), 1);
    }

    #[test]
    fn test_scan_directory() {
        let temp_dir = TempDir::new().unwrap();
        let scanner = Scanner::new();

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
        let scanner = Scanner::new();
        let content = "AWS_KEY=AKIAIOSFODNN7EXAMPLE";
        let results = scanner.scan_content(content, "test.txt");

        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.pattern.contains("AWS")));
    }

    #[test]
    fn test_parallel_scanning_performance() {
        let temp_dir = TempDir::new().unwrap();
        let scanner = Scanner::new();

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
        let scanner = Scanner::new();

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
}
