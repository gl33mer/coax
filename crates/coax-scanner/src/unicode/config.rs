//! Unicode Scanner Configuration
//!
//! This module provides configuration options for Unicode attack detection.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Unicode scanner configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodeConfig {
    /// Enable Unicode scanning
    pub enabled: bool,
    
    /// Sensitivity level: low, medium, high, critical
    pub sensitivity: SensitivityLevel,
    
    /// Enable/disable individual detectors
    pub detectors: DetectorConfig,
    
    /// File patterns to include
    pub include_patterns: Vec<String>,
    
    /// File patterns to exclude
    pub exclude_patterns: Vec<String>,
    
    /// Allowlist configuration
    pub allowlist: AllowlistConfig,
    
    /// Performance tuning
    pub performance: PerformanceConfig,
}

/// Sensitivity levels for Unicode detection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SensitivityLevel {
    Low,
    Medium,
    High,
    Critical,
}

impl SensitivityLevel {
    pub fn as_str(&self) -> &'static str {
        match self {
            SensitivityLevel::Low => "low",
            SensitivityLevel::Medium => "medium",
            SensitivityLevel::High => "high",
            SensitivityLevel::Critical => "critical",
        }
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "critical" => SensitivityLevel::Critical,
            "high" => SensitivityLevel::High,
            "medium" => SensitivityLevel::Medium,
            _ => SensitivityLevel::Low,
        }
    }
}

impl Default for SensitivityLevel {
    fn default() -> Self {
        SensitivityLevel::High
    }
}

/// Configuration for individual detectors
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorConfig {
    /// Detect invisible characters (zero-width, variation selectors)
    pub invisible_chars: bool,
    
    /// Detect homoglyph/confusable characters
    pub homoglyphs: bool,
    
    /// Detect bidirectional overrides
    pub bidirectional: bool,
    
    /// Detect Unicode tags
    pub unicode_tags: bool,
    
    /// Detect Glassworm-specific patterns
    pub glassworm: bool,
    
    /// Detect normalization attacks
    pub normalization: bool,
    
    /// Detect emoji obfuscation
    pub emoji_obfuscation: bool,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            invisible_chars: true,
            homoglyphs: true,
            bidirectional: true,
            unicode_tags: true,
            glassworm: true,
            normalization: false, // Only for high-security projects
            emoji_obfuscation: true,
        }
    }
}

/// Allowlist configuration for legitimate Unicode uses
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AllowlistConfig {
    /// Files to always skip (glob patterns)
    pub files: Vec<String>,
    
    /// Directories to always skip (glob patterns)
    pub directories: Vec<String>,
    
    /// Character ranges to allow (for i18n projects)
    pub character_ranges: Vec<(u32, u32)>,
    
    /// Scripts to allow (e.g., "Han", "Hangul" for Asian language projects)
    pub allowed_scripts: Vec<String>,
}

/// Performance tuning configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceConfig {
    /// Maximum file size to scan (bytes)
    pub max_file_size: u64,
    
    /// Skip binary files
    pub skip_binary: bool,
    
    /// Early exit on critical findings
    pub exit_on_critical: bool,
    
    /// Parallel scanning enabled
    pub parallel: bool,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            max_file_size: 10 * 1024 * 1024, // 10MB
            skip_binary: true,
            exit_on_critical: false,
            parallel: true,
        }
    }
}

impl Default for UnicodeConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: SensitivityLevel::default(),
            detectors: DetectorConfig::default(),
            include_patterns: vec![
                "**/*.js".to_string(),
                "**/*.ts".to_string(),
                "**/*.py".to_string(),
                "**/*.rs".to_string(),
                "**/*.go".to_string(),
                "**/*.java".to_string(),
                "**/*.sh".to_string(),
                "**/*.yaml".to_string(),
                "**/*.json".to_string(),
                "**/*.md".to_string(),
                "**/*.txt".to_string(),
            ],
            exclude_patterns: vec![
                "**/*.min.js".to_string(),
                "**/vendor/**".to_string(),
                "**/node_modules/**".to_string(),
                "**/*.lock".to_string(),
                "**/package-lock.json".to_string(),
                "**/yarn.lock".to_string(),
                "**/pnpm-lock.yaml".to_string(),
                "**/go.sum".to_string(),
            ],
            allowlist: AllowlistConfig {
                files: vec![
                    "**/i18n/**".to_string(),
                    "**/locales/**".to_string(),
                    "**/translations/**".to_string(),
                ],
                directories: vec![],
                character_ranges: vec![],
                allowed_scripts: vec![
                    "Han".to_string(),      // Chinese
                    "Hangul".to_string(),   // Korean
                    "Hiragana".to_string(), // Japanese
                    "Katakana".to_string(), // Japanese
                ],
            },
            performance: PerformanceConfig::default(),
        }
    }
}

impl UnicodeConfig {
    /// Create config with default settings
    pub fn new() -> Self {
        Self::default()
    }

    /// Create config optimized for i18n projects
    pub fn for_i18n_project() -> Self {
        Self {
            sensitivity: SensitivityLevel::Medium,
            allowlist: AllowlistConfig {
                allowed_scripts: vec![
                    "Han".to_string(),
                    "Hangul".to_string(),
                    "Hiragana".to_string(),
                    "Katakana".to_string(),
                    "Arabic".to_string(),
                    "Hebrew".to_string(),
                    "Thai".to_string(),
                ],
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Create config for high-security projects
    pub fn for_high_security() -> Self {
        Self {
            sensitivity: SensitivityLevel::Critical,
            detectors: DetectorConfig {
                normalization: true,
                ..DetectorConfig::default()
            },
            ..Default::default()
        }
    }

    /// Check if a file path should be excluded
    pub fn should_exclude(&self, file_path: &str) -> bool {
        // Check exclude patterns
        for pattern in &self.exclude_patterns {
            if self.matches_glob(pattern, file_path) {
                return true;
            }
        }
        
        // Check allowlist files
        for pattern in &self.allowlist.files {
            if self.matches_glob(pattern, file_path) {
                return false; // Explicitly allowed
            }
        }
        
        false
    }

    /// Simple glob matching (supports ** and *)
    fn matches_glob(&self, pattern: &str, text: &str) -> bool {
        // Convert glob to regex-like matching
        let pattern = pattern.replace("**", ".*").replace("*", "[^/]*");
        let regex_pattern = format!("^{}$", pattern);
        
        if let Ok(re) = regex::Regex::new(&regex_pattern) {
            return re.is_match(text);
        }
        
        false
    }

    /// Check if sensitivity allows detection at a given level
    pub fn sensitivity_allows(&self, required: SensitivityLevel) -> bool {
        self.sensitivity as u8 >= required as u8
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = UnicodeConfig::default();
        assert!(config.enabled);
        assert_eq!(config.sensitivity, SensitivityLevel::High);
        assert!(config.detectors.invisible_chars);
        assert!(config.detectors.homoglyphs);
    }

    #[test]
    fn test_i18n_config() {
        let config = UnicodeConfig::for_i18n_project();
        assert_eq!(config.sensitivity, SensitivityLevel::Medium);
        assert!(config.allowlist.allowed_scripts.contains(&"Han".to_string()));
    }

    #[test]
    fn test_high_security_config() {
        let config = UnicodeConfig::for_high_security();
        assert_eq!(config.sensitivity, SensitivityLevel::Critical);
        assert!(config.detectors.normalization);
    }

    #[test]
    fn test_sensitivity_comparison() {
        let config = UnicodeConfig {
            sensitivity: SensitivityLevel::High,
            ..Default::default()
        };
        
        assert!(config.sensitivity_allows(SensitivityLevel::Low));
        assert!(config.sensitivity_allows(SensitivityLevel::Medium));
        assert!(config.sensitivity_allows(SensitivityLevel::High));
        assert!(!config.sensitivity_allows(SensitivityLevel::Critical));
    }
}
