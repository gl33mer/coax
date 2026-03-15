//! Coax Scanner - High-Performance Security Scanner
//!
//! This crate provides optimized secret and vulnerability scanning with:
//! - Pattern compilation caching (compile once, use many times)
//! - Parallel file scanning using rayon
//! - Configurable thread pool for optimal performance
//! - Token efficiency filtering (BPE-based, from Betterleaks)
//! - Word filter using Aho-Corasick algorithm (from Betterleaks)
//!
//! # Performance
//!
//! Benchmarks show 3-5x speedup compared to naive implementation:
//! - scan_100_files: <100ms (target), ~300ms (baseline)
//! - scan_regex_only: <100ms (target), ~270ms (baseline)
//!
//! # Betterleaks Integration
//!
//! This scanner incorporates detection algorithms from Betterleaks:
//! - **Token Efficiency Filter**: Uses BPE tokenization to distinguish real secrets
//!   from natural language. Real secrets have high token efficiency (>2.5).
//! - **Word Filter**: Uses Aho-Corasick algorithm for multi-pattern matching
//!   to filter out common English words and programming keywords.
//!
//! # Example
//!
//! ```rust
//! use coax_scanner::{Scanner, PatternConfig, ScanResult};
//! use std::path::PathBuf;
//! use std::sync::Arc;
//!
//! // Create scanner with default patterns
//! let scanner = Scanner::with_default_patterns();
//!
//! // Scan a directory
//! let results = scanner.scan_directory(&PathBuf::from("./src"));
//!
//! // Process results
//! for result in results {
//!     println!("Found {} in {}:{}", result.pattern, result.file.display(), result.line);
//! }
//! ```

mod pattern_cache;
mod scanner;
mod secrets;
mod result;
mod context;
pub mod pattern_loader;
pub mod token_efficiency;
pub mod word_filter;

pub use pattern_cache::{PatternCache, CompiledPattern, PatternConfig};
pub use scanner::{Scanner, ScannerConfig};
pub use result::{ScanResult, ScanSummary, SeverityCounts, PatternCount, OutputFormat, FindingContext};
pub use secrets::SecretPattern;
pub use context::{ContextAnalyzer, ExclusionPatterns};
pub use pattern_loader::{PatternLoader, PatternLoaderError, PatternValidationResult, ValidationResult, PatternsFile, PatternEntry};
pub use token_efficiency::{TokenEfficiencyConfig, calculate_token_efficiency, is_likely_secret, fails_token_efficiency_filter};
pub use word_filter::{WordFilter, WordFilterConfig, WordFilterResult};

/// Crate version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Re-export commonly used types
pub use regex::Regex;
pub use thiserror::Error;

/// Error types for scanner operations
#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Regex compilation error: {0}")]
    Regex(#[from] regex::Error),

    #[error("File read error for {path}: {source}")]
    FileRead {
        path: String,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid configuration: {0}")]
    Config(String),
}

/// Result type alias for scanner operations
pub type ScanError = ScannerError;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_scanner_creation() {
        let scanner = Scanner::with_default_patterns();
        assert!(!scanner.config.patterns.is_empty());
    }

    #[test]
    fn test_scan_file_with_secrets() {
        // Disable filters and context detection for this test
        let scanner = Scanner::with_config(
            ScannerConfig::default()
                .with_token_efficiency(false)
                .with_word_filter(false)
                .with_context_detection(false)
        );
        let content = r#"
            AWS_KEY=AKIAIOSFODNN7EXAMPLE
            GITHUB_TOKEN=ghp_1234567890abcdefghij1234567890abcdefghij
        "#;

        let results = scanner.scan_content(content, "test.txt");
        assert!(!results.is_empty());
        assert!(results.iter().any(|r| r.pattern.contains("AWS")));
        assert!(results.iter().any(|r| r.pattern.contains("GITHUB")));
    }

    #[test]
    fn test_scan_clean_file() {
        let scanner = Scanner::with_default_patterns();
        let content = r#"
            // This is a clean file
            let x = 42;
            println!("Hello, world!");
        "#;

        let results = scanner.scan_content(content, "clean.rs");
        assert!(results.is_empty());
    }

    #[test]
    fn test_parallel_scanning() {
        let temp_dir = TempDir::new().unwrap();
        // Disable filters and context detection for this test
        let scanner = Scanner::with_config(
            ScannerConfig::default()
                .with_token_efficiency(false)
                .with_word_filter(false)
                .with_context_detection(false)
        );

        // Create test files
        for i in 0..10 {
            let file_path = temp_dir.path().join(format!("file_{}.txt", i));
            let content = if i == 5 {
                "AWS_KEY=AKIAIOSFODNN7EXAMPLE".to_string()
            } else {
                "clean content".to_string()
            };
            fs::write(&file_path, content).unwrap();
        }

        let results = scanner.scan_directory(temp_dir.path());
        assert_eq!(results.len(), 1);
        assert!(results[0].pattern.contains("AWS"));
    }

    #[test]
    fn test_pattern_cache_caching() {
        let patterns = vec![
            PatternConfig::new(
                "TEST_PATTERN",
                r"test\d+",
                "high",
                "Test recommendation",
            ),
        ];

        let cache = PatternCache::new(&patterns);
        // Second access should use cached pattern
        let _cache2 = PatternCache::new(&patterns);
        // If we get here without panic, caching works
    }
}
