//! Pattern Loader Module
//!
//! This module provides functionality to load secret detection patterns from external YAML files.
//! It supports:
//! - Loading patterns from individual YAML files
//! - Loading patterns from directories (recursive)
//! - Merging patterns from multiple sources
//! - Pattern validation (regex compilation testing)
//! - Pattern filtering by confidence level, category, and enabled status
//!
//! # Pattern YAML Format
//!
//! ```yaml
//! patterns:
//!   - name: AWS_ACCESS_KEY
//!     regex: 'AKIA[0-9A-Z]{16}'
//!     severity: critical
//!     recommendation: "Remove immediately and rotate via AWS IAM Console"
//!     description: "AWS Access Key ID"
//!     cwe_id: "CWE-798"
//!     confidence: high
//!     category: cloud_provider
//!     enabled: true
//! ```
//!
//! # Example
//!
//! ```rust
//! use coax_scanner::PatternLoader;
//! use std::path::Path;
//!
//! let mut loader = PatternLoader::new();
//! loader.load_from_file(Path::new("patterns/aws.yml")).unwrap();
//! loader.load_from_directory(Path::new("patterns/")).unwrap();
//!
//! let patterns = loader.get_patterns();
//! println!("Loaded {} patterns", patterns.len());
//! ```

use crate::pattern_cache::PatternConfig;
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Error types for pattern loading
#[derive(Error, Debug)]
pub enum PatternLoaderError {
    #[error("Failed to read file '{path}': {source}")]
    FileReadError {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Failed to parse YAML from '{path}': {source}")]
    YamlParseError {
        path: PathBuf,
        #[source]
        source: serde_yaml::Error,
    },

    #[error("Invalid regex pattern '{name}': {source}")]
    InvalidRegex {
        name: String,
        #[source]
        source: regex::Error,
    },

    #[error("Directory not found: {0}")]
    DirectoryNotFound(PathBuf),

    #[error("Pattern validation failed: {0}")]
    ValidationError(String),
}

/// Result type for pattern loader operations
pub type Result<T> = std::result::Result<T, PatternLoaderError>;

/// Validation result for a single pattern
#[derive(Debug, Clone)]
pub struct PatternValidationResult {
    pub name: String,
    pub valid: bool,
    pub error: Option<String>,
}

/// Validation results for all patterns
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub total: usize,
    pub valid: usize,
    pub invalid: usize,
    pub results: Vec<PatternValidationResult>,
}

impl ValidationResult {
    pub fn is_success(&self) -> bool {
        self.invalid == 0
    }

    pub fn success_rate(&self) -> f64 {
        if self.total == 0 {
            100.0
        } else {
            (self.valid as f64 / self.total as f64) * 100.0
        }
    }
}

/// YAML structure for pattern files
#[derive(Debug, Deserialize, Clone)]
pub struct PatternsFile {
    pub patterns: Vec<PatternEntry>,
}

/// Pattern entry in YAML file
///
/// This struct matches the secrets-patterns-db format for easy conversion.
#[derive(Debug, Deserialize, Clone)]
pub struct PatternEntry {
    /// Pattern name/identifier
    pub name: String,

    /// Regex pattern string
    pub regex: String,

    /// Severity level: critical, high, medium, low
    #[serde(default = "default_severity")]
    pub severity: String,

    /// Recommendation for remediation
    #[serde(default)]
    pub recommendation: Option<String>,

    /// Optional description
    #[serde(default)]
    pub description: Option<String>,

    /// Optional CWE ID
    #[serde(default, rename = "cwe_id")]
    pub cwe_id: Option<String>,

    /// Confidence level: high, medium, low
    #[serde(default)]
    pub confidence: Option<String>,

    /// Category for grouping patterns
    #[serde(default)]
    pub category: Option<String>,

    /// Whether this pattern is enabled
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_severity() -> String {
    "medium".to_string()
}

fn default_true() -> bool {
    true
}

impl PatternEntry {
    /// Convert to PatternConfig
    pub fn to_pattern_config(&self) -> PatternConfig {
        PatternConfig::new_owned(
            self.name.clone(),
            self.regex.clone(),
            self.severity.clone(),
            self.recommendation.clone().unwrap_or_else(|| "Review this finding".to_string()),
        )
        .with_description_opt(self.description.clone())
        .with_cwe_id_opt(self.cwe_id.clone())
        .with_confidence_opt(self.confidence.clone())
        .with_category_opt(self.category.clone())
        .with_enabled(self.enabled)
    }
}

// Add helper methods to PatternConfig for optional fields
impl PatternConfig {
    pub fn with_description_opt(mut self, description: Option<String>) -> Self {
        self.description = description;
        self
    }

    pub fn with_cwe_id_opt(mut self, cwe_id: Option<String>) -> Self {
        self.cwe_id = cwe_id;
        self
    }

    pub fn with_confidence_opt(mut self, confidence: Option<String>) -> Self {
        self.confidence = confidence;
        self
    }

    pub fn with_category_opt(mut self, category: Option<String>) -> Self {
        self.category = category;
        self
    }
}

/// Pattern loader for external YAML files
///
/// Supports loading patterns from:
/// - Individual YAML files
/// - Directories (recursively)
/// - Multiple sources with merging
pub struct PatternLoader {
    patterns: Vec<PatternConfig>,
    loaded_files: Vec<PathBuf>,
}

impl Default for PatternLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PatternLoader {
    /// Create a new pattern loader
    pub fn new() -> Self {
        Self {
            patterns: Vec::new(),
            loaded_files: Vec::new(),
        }
    }

    /// Load patterns from a single YAML file
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut loader = PatternLoader::new();
    /// loader.load_from_file(Path::new("patterns/aws.yml"))?;
    /// ```
    pub fn load_from_file(&mut self, path: &Path) -> Result<usize> {
        let content = fs::read_to_string(path).map_err(|e| PatternLoaderError::FileReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

        let patterns_file: PatternsFile =
            serde_yaml::from_str(&content).map_err(|e| PatternLoaderError::YamlParseError {
                path: path.to_path_buf(),
                source: e,
            })?;

        let mut count = 0;
        for entry in patterns_file.patterns {
            // FP REDUCTION: Validate pattern before adding
            if Self::is_overly_broad_pattern(&entry.regex) {
                tracing::warn!(
                    "Skipping overly broad pattern '{}': {} (FP reduction)",
                    entry.name,
                    entry.regex
                );
                continue;
            }

            let config = entry.to_pattern_config();

            // FP REDUCTION: Add word boundary to patterns without proper boundaries
            let validated_config = Self::add_word_boundaries_if_needed(config);

            self.patterns.push(validated_config);
            count += 1;
        }

        self.loaded_files.push(path.to_path_buf());

        Ok(count)
    }

    /// FP REDUCTION: Check if a regex pattern is overly broad
    ///
    /// Returns true if the pattern is likely to cause false positives:
    /// - Contains .* without proper anchoring
    /// - Is too short (< 10 chars) with wildcards
    /// - Matches common programming constructs
    fn is_overly_broad_pattern(regex: &str) -> bool {
        // Skip empty patterns
        if regex.is_empty() {
            return true;
        }

        // Skip patterns that are just .* or similar
        if regex == ".*" || regex == ".+" || regex == "*" || regex == "+" {
            return true;
        }

        // Skip patterns with .* at both ends and very little content
        if regex.starts_with(".*") && regex.ends_with(".*") && regex.len() < 20 {
            return true;
        }

        // Skip patterns that match generic key=value without specificity
        if regex.contains(".*") && regex.len() < 15 && (regex.contains("key") || regex.contains("secret") || regex.contains("password")) {
            return true;
        }

        // Skip patterns with unanchored wildcards that match too much
        if regex.starts_with(".*") && !regex.contains("[") && !regex.contains("{") {
            if regex.matches(".*").count() > 2 {
                return true;
            }
        }

        false
    }

    /// FP REDUCTION: Add word boundaries to patterns if needed
    ///
    /// This helps prevent false positives from partial matches.
    fn add_word_boundaries_if_needed(mut config: crate::pattern_cache::PatternConfig) -> crate::pattern_cache::PatternConfig {
        // Don't modify already anchored patterns
        if config.pattern.starts_with('^') || config.pattern.starts_with(r"\b") {
            return config;
        }

        // Don't modify patterns with explicit boundaries
        if config.pattern.starts_with("(?") || config.pattern.starts_with('[') {
            return config;
        }

        // For short patterns (< 30 chars) without boundaries, add word boundary
        if config.pattern.len() < 30 && !config.pattern.contains("\\b") {
            // Only add if it doesn't already have implicit boundaries
            if !config.pattern.starts_with("(?") && !config.pattern.contains("(?:") {
                // Add word boundary at start for patterns that look like they need it
                if config.pattern.chars().next().map_or(false, |c| c.is_ascii_alphanumeric()) {
                    config.pattern = format!(r"\b{}", config.pattern);
                }
            }
        }

        config
    }

    /// Load patterns from all YAML files in a directory (recursively)
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut loader = PatternLoader::new();
    /// loader.load_from_directory(Path::new("patterns/"))?;
    /// ```
    pub fn load_from_directory(&mut self, dir: &Path) -> Result<usize> {
        if !dir.exists() {
            return Err(PatternLoaderError::DirectoryNotFound(dir.to_path_buf()));
        }

        let mut total = 0;

        for entry in walkdir::WalkDir::new(dir)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "yml" || ext == "yaml" {
                        if let Ok(count) = self.load_from_file(path) {
                            total += count;
                        }
                    }
                }
            }
        }

        Ok(total)
    }

    /// Load patterns from secrets-patterns-db format
    ///
    /// This method handles the specific format used by mazen160/secrets-patterns-db:
    /// ```yaml
    /// patterns:
    ///   - pattern:
    ///       name: AWS API Key
    ///       regex: AKIA[0-9A-Z]{16}
    ///       confidence: high
    /// ```
    pub fn load_secrets_patterns_db(&mut self, path: &Path) -> Result<usize> {
        let content = fs::read_to_string(path).map_err(|e| PatternLoaderError::FileReadError {
            path: path.to_path_buf(),
            source: e,
        })?;

        // Try to parse as secrets-patterns-db format
        let db_file: SecretsPatternsDbFile =
            serde_yaml::from_str(&content).map_err(|e| PatternLoaderError::YamlParseError {
                path: path.to_path_buf(),
                source: e,
            })?;

        let count = db_file.patterns.len();
        for entry in db_file.patterns {
            self.patterns.push(entry.to_pattern_config());
        }

        self.loaded_files.push(path.to_path_buf());

        Ok(count)
    }

    /// YAML structure for secrets-patterns-db format
    fn load_spdb_format(&mut self, path: &Path) -> Result<usize> {
        self.load_secrets_patterns_db(path)
    }

    /// Get all loaded patterns
    pub fn get_patterns(&self) -> &[PatternConfig] {
        &self.patterns
    }

    /// Get all loaded patterns (owned)
    pub fn into_patterns(self) -> Vec<PatternConfig> {
        self.patterns
    }

    /// Get list of loaded files
    pub fn get_loaded_files(&self) -> &[PathBuf] {
        &self.loaded_files
    }

    /// Get pattern count
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if loader is empty
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Validate all loaded patterns (test regex compilation)
    ///
    /// Returns validation results for each pattern.
    pub fn validate_patterns(&self) -> ValidationResult {
        let mut results = Vec::new();
        let mut valid = 0;
        let mut invalid = 0;

        for pattern in &self.patterns {
            match regex::Regex::new(&pattern.pattern) {
                Ok(_) => {
                    valid += 1;
                    results.push(PatternValidationResult {
                        name: pattern.name.clone(),
                        valid: true,
                        error: None,
                    });
                }
                Err(e) => {
                    invalid += 1;
                    results.push(PatternValidationResult {
                        name: pattern.name.clone(),
                        valid: false,
                        error: Some(e.to_string()),
                    });
                }
            }
        }

        ValidationResult {
            total: self.patterns.len(),
            valid,
            invalid,
            results,
        }
    }

    /// Filter patterns by confidence level
    ///
    /// # Arguments
    ///
    /// * `min_confidence` - Minimum confidence level ("low", "medium", "high")
    ///
    /// # Returns
    ///
    /// A new PatternLoader with filtered patterns
    pub fn filter_by_confidence(&self, min_confidence: &str) -> PatternLoader {
        let confidence_order = ["low", "medium", "high"];
        let min_level = confidence_order
            .iter()
            .position(|&c| c == min_confidence.to_lowercase().as_str())
            .unwrap_or(0);

        let patterns = self
            .patterns
            .iter()
            .filter(|p| {
                p.confidence
                    .as_ref()
                    .map(|c| {
                        confidence_order
                            .iter()
                            .position(|&level| level == c.to_lowercase().as_str())
                            .map(|level| level >= min_level)
                            .unwrap_or(true)
                    })
                    .unwrap_or(true)
            })
            .cloned()
            .collect();

        PatternLoader {
            patterns,
            loaded_files: self.loaded_files.clone(),
        }
    }

    /// Filter patterns by category
    pub fn filter_by_category(&self, category: &str) -> PatternLoader {
        let patterns = self
            .patterns
            .iter()
            .filter(|p| {
                p.category
                    .as_ref()
                    .map(|c| c == category)
                    .unwrap_or(false)
            })
            .cloned()
            .collect();

        PatternLoader {
            patterns,
            loaded_files: self.loaded_files.clone(),
        }
    }

    /// Filter to only enabled patterns
    pub fn filter_enabled(&self) -> PatternLoader {
        let patterns = self
            .patterns
            .iter()
            .filter(|p| p.enabled)
            .cloned()
            .collect();

        PatternLoader {
            patterns,
            loaded_files: self.loaded_files.clone(),
        }
    }

    /// Add a pattern directly (for programmatic use)
    pub fn add_pattern(&mut self, pattern: PatternConfig) {
        self.patterns.push(pattern);
    }

    /// Clear all loaded patterns
    pub fn clear(&mut self) {
        self.patterns.clear();
        self.loaded_files.clear();
    }

    /// Merge patterns from another loader
    pub fn merge(&mut self, other: PatternLoader) {
        self.patterns.extend(other.patterns);
        self.loaded_files.extend(other.loaded_files);
    }
}

/// YAML structure for secrets-patterns-db format
///
/// Format:
/// ```yaml
/// patterns:
///   - pattern:
///       name: AWS API Key
///       regex: AKIA[0-9A-Z]{16}
///       confidence: high
/// ```
#[derive(Debug, Deserialize, Clone)]
pub struct SecretsPatternsDbFile {
    pub patterns: Vec<SpdbPatternEntry>,
}

/// Pattern entry in secrets-patterns-db format
#[derive(Debug, Deserialize, Clone)]
pub struct SpdbPatternEntry {
    pub pattern: SpdbPatternDetails,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SpdbPatternDetails {
    pub name: String,
    pub regex: String,
    #[serde(default)]
    pub confidence: Option<String>,
}

impl SpdbPatternEntry {
    /// Convert to PatternConfig
    pub fn to_pattern_config(&self) -> PatternConfig {
        let severity = match self.pattern.confidence.as_deref() {
            Some("high") => "high",
            Some("low") => "low",
            _ => "medium",
        };

        PatternConfig::new_owned(
            self.pattern.name.clone(),
            self.pattern.regex.clone(),
            severity.to_string(),
            "Review this finding".to_string(),
        )
        .with_confidence_opt(self.pattern.confidence.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_pattern_loader_creation() {
        let loader = PatternLoader::new();
        assert!(loader.is_empty());
        assert_eq!(loader.len(), 0);
    }

    #[test]
    fn test_load_from_file() {
        let temp_dir = TempDir::new().unwrap();
        let pattern_file = temp_dir.path().join("test_patterns.yml");

        let content = r#"
patterns:
  - name: TEST_AWS
    regex: 'AKIA[0-9A-Z]{16}'
    severity: critical
    recommendation: "Test recommendation"
    description: "Test AWS pattern"
    cwe_id: "CWE-798"
    confidence: high
    category: cloud_provider
    enabled: true
"#;

        fs::write(&pattern_file, content).unwrap();

        let mut loader = PatternLoader::new();
        let count = loader.load_from_file(&pattern_file).unwrap();

        assert_eq!(count, 1);
        assert_eq!(loader.len(), 1);
        assert_eq!(loader.get_loaded_files().len(), 1);

        let patterns = loader.get_patterns();
        assert_eq!(patterns[0].name, "TEST_AWS");
        assert_eq!(patterns[0].severity, "critical");
        assert_eq!(patterns[0].confidence, Some("high".to_string()));
    }

    #[test]
    fn test_load_from_directory() {
        let temp_dir = TempDir::new().unwrap();

        // Create multiple pattern files
        let file1 = temp_dir.path().join("aws.yml");
        fs::write(
            &file1,
            r#"
patterns:
  - name: AWS_KEY
    regex: 'AKIA[0-9A-Z]{16}'
    severity: critical
"#,
        )
        .unwrap();

        let file2 = temp_dir.path().join("github.yml");
        fs::write(
            &file2,
            r#"
patterns:
  - name: GITHUB_TOKEN
    regex: 'ghp_[a-zA-Z0-9]{36}'
    severity: critical
"#,
        )
        .unwrap();

        let mut loader = PatternLoader::new();
        let count = loader.load_from_directory(temp_dir.path()).unwrap();

        assert_eq!(count, 2);
        assert_eq!(loader.len(), 2);
    }

    #[test]
    fn test_validate_patterns() {
        let temp_dir = TempDir::new().unwrap();
        let pattern_file = temp_dir.path().join("test.yml");

        let content = r#"
patterns:
  - name: VALID_PATTERN
    regex: 'AKIA[0-9A-Z]{16}'
    severity: critical
  - name: INVALID_PATTERN
    regex: '[invalid(regex'
    severity: high
"#;

        fs::write(&pattern_file, content).unwrap();

        let mut loader = PatternLoader::new();
        loader.load_from_file(&pattern_file).unwrap();

        let validation = loader.validate_patterns();

        assert_eq!(validation.total, 2);
        assert_eq!(validation.valid, 1);
        assert_eq!(validation.invalid, 1);
        assert!(!validation.is_success());

        let invalid = validation.results.iter().find(|r| !r.valid).unwrap();
        assert_eq!(invalid.name, "INVALID_PATTERN");
        assert!(invalid.error.is_some());
    }

    #[test]
    fn test_filter_by_confidence() {
        let temp_dir = TempDir::new().unwrap();
        let pattern_file = temp_dir.path().join("test.yml");

        let content = r#"
patterns:
  - name: HIGH_CONF
    regex: 'AKIA[0-9A-Z]{16}'
    confidence: high
  - name: LOW_CONF
    regex: 'test[0-9]+'
    confidence: low
  - name: NO_CONF
    regex: 'example'
"#;

        fs::write(&pattern_file, content).unwrap();

        let mut loader = PatternLoader::new();
        loader.load_from_file(&pattern_file).unwrap();

        // Filter to high confidence only (includes NO_CONF which has no confidence set)
        let filtered = loader.filter_by_confidence("high");
        assert_eq!(filtered.len(), 2); // HIGH_CONF + NO_CONF
        assert!(filtered.get_patterns().iter().any(|p| p.name == "HIGH_CONF"));

        // Filter to medium and above (includes NO_CONF which defaults to included)
        let filtered = loader.filter_by_confidence("medium");
        assert_eq!(filtered.len(), 2); // HIGH_CONF + NO_CONF

        // Filter to low and above (all)
        let filtered = loader.filter_by_confidence("low");
        assert_eq!(filtered.len(), 3); // All patterns
    }

    #[test]
    fn test_filter_enabled() {
        let temp_dir = TempDir::new().unwrap();
        let pattern_file = temp_dir.path().join("test.yml");

        let content = r#"
patterns:
  - name: ENABLED
    regex: 'AKIA[0-9A-Z]{16}'
    enabled: true
  - name: DISABLED
    regex: 'test[0-9]+'
    enabled: false
"#;

        fs::write(&pattern_file, content).unwrap();

        let mut loader = PatternLoader::new();
        loader.load_from_file(&pattern_file).unwrap();

        let filtered = loader.filter_enabled();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered.get_patterns()[0].name, "ENABLED");
    }

    #[test]
    fn test_merge_loaders() {
        let mut loader1 = PatternLoader::new();
        loader1.add_pattern(PatternConfig::new("PATTERN1", r"test1", "high", "Test 1"));

        let mut loader2 = PatternLoader::new();
        loader2.add_pattern(PatternConfig::new("PATTERN2", r"test2", "medium", "Test 2"));

        loader1.merge(loader2);

        assert_eq!(loader1.len(), 2);
        assert_eq!(loader1.get_patterns()[0].name, "PATTERN1");
        assert_eq!(loader1.get_patterns()[1].name, "PATTERN2");
    }

    #[test]
    fn test_secrets_patterns_db_format() {
        let temp_dir = TempDir::new().unwrap();
        let pattern_file = temp_dir.path().join("spdb.yml");

        // secrets-patterns-db format
        let content = r#"
patterns:
  - pattern:
      name: AWS API Key
      regex: 'AKIA[0-9A-Z]{16}'
      confidence: high
  - pattern:
      name: GitHub Token
      regex: 'ghp_[a-zA-Z0-9]{36}'
      confidence: high
"#;

        fs::write(&pattern_file, content).unwrap();

        let mut loader = PatternLoader::new();
        let count = loader.load_secrets_patterns_db(&pattern_file).unwrap();

        assert_eq!(count, 2);
        assert_eq!(loader.len(), 2);

        let patterns = loader.get_patterns();
        assert_eq!(patterns[0].name, "AWS API Key");
        assert_eq!(patterns[1].name, "GitHub Token");
    }

    #[test]
    fn test_load_secrets_patterns_db_file() {
        // Test loading the actual secrets_patterns_db.yml file if it exists
        let spdb_file = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../config/patterns/secrets_patterns_db.yml");
        
        if spdb_file.exists() {
            let mut loader = PatternLoader::new();
            let count = loader.load_from_file(&spdb_file).expect("Failed to load secrets_patterns_db.yml");
            
            assert!(count > 0, "Should load at least one pattern");
            
            // Validate all patterns
            let validation = loader.validate_patterns();
            assert!(validation.valid > 0, "Should have at least one valid pattern");
            
            println!("Loaded {} patterns, {} valid, {} invalid", 
                     validation.total, validation.valid, validation.invalid);
        }
    }
}
