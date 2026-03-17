//! Pattern Compilation Cache
//!
//! This module provides pre-compilation and caching of regex patterns
//! to eliminate recompilation overhead during scanning.
//!
//! # Performance Impact
//!
//! Pre-compiling patterns reduces scan time by:
//! - Eliminating regex compilation on every file
//! - Sharing compiled patterns across threads via Arc
//! - Reducing memory allocations during matching

use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Configuration for a single pattern
///
/// This struct supports both static patterns (using &'static str) and
/// dynamically loaded patterns (using String). For static patterns,
/// the String fields will contain the same data as the static references.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    /// Unique identifier for the pattern
    pub name: String,
    /// Regex pattern string
    pub pattern: String,
    /// Severity level: critical, high, medium, low
    pub severity: String,
    /// Recommendation for remediation
    pub recommendation: String,
    /// Whether to extract the matched secret string (default: true)
    #[serde(default = "default_true")]
    pub extract_secret: bool,
    /// Minimum entropy threshold for this pattern (optional, for high-entropy patterns)
    pub min_entropy: Option<f64>,
    /// Optional description of the pattern
    #[serde(default)]
    pub description: Option<String>,
    /// Optional CWE ID
    #[serde(default)]
    pub cwe_id: Option<String>,
    /// Optional confidence level (high, medium, low)
    #[serde(default)]
    pub confidence: Option<String>,
    /// Optional category for grouping patterns
    #[serde(default)]
    pub category: Option<String>,
    /// Whether this pattern is enabled (default: true)
    #[serde(default = "default_true")]
    pub enabled: bool,
}

fn default_true() -> bool {
    true
}

impl Default for PatternConfig {
    fn default() -> Self {
        Self {
            name: "UNKNOWN".to_string(),
            pattern: r".*".to_string(),
            severity: "low".to_string(),
            recommendation: "Review this finding".to_string(),
            extract_secret: true,
            min_entropy: None,
            description: None,
            cwe_id: None,
            confidence: None,
            category: None,
            enabled: true,
        }
    }
}

impl PatternConfig {
    /// Create a new PatternConfig with static strings (convenience for backward compatibility)
    pub fn new(
        name: &'static str,
        pattern: &'static str,
        severity: &'static str,
        recommendation: &'static str,
    ) -> Self {
        Self {
            name: name.to_string(),
            pattern: pattern.to_string(),
            severity: severity.to_string(),
            recommendation: recommendation.to_string(),
            extract_secret: true,
            min_entropy: None,
            description: None,
            cwe_id: None,
            confidence: None,
            category: None,
            enabled: true,
        }
    }

    /// Create a new PatternConfig with owned strings
    pub fn new_owned(
        name: String,
        pattern: String,
        severity: String,
        recommendation: String,
    ) -> Self {
        Self {
            name,
            pattern,
            severity,
            recommendation,
            extract_secret: true,
            min_entropy: None,
            description: None,
            cwe_id: None,
            confidence: None,
            category: None,
            enabled: true,
        }
    }

    /// Set the description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the CWE ID
    pub fn with_cwe_id(mut self, cwe_id: impl Into<String>) -> Self {
        self.cwe_id = Some(cwe_id.into());
        self
    }

    /// Set the confidence level
    pub fn with_confidence(mut self, confidence: impl Into<String>) -> Self {
        self.confidence = Some(confidence.into());
        self
    }

    /// Set the category
    pub fn with_category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    /// Set whether the pattern is enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Set the minimum entropy threshold
    pub fn with_min_entropy(mut self, min_entropy: f64) -> Self {
        self.min_entropy = Some(min_entropy);
        self
    }

    /// Set whether to extract the secret value
    pub fn with_extract_secret(mut self, extract_secret: bool) -> Self {
        self.extract_secret = extract_secret;
        self
    }
}

/// A compiled pattern ready for matching
///
/// This struct holds the pre-compiled regex and associated metadata.
/// It's designed to be cheaply cloneable (uses Arc internally).
#[derive(Debug, Clone)]
pub struct CompiledPattern {
    /// Pattern identifier
    pub name: Arc<str>,
    /// Pre-compiled regex
    pub regex: Regex,
    /// Severity level
    pub severity: Arc<str>,
    /// Remediation recommendation
    pub recommendation: Arc<str>,
    /// Whether to extract the matched secret string
    pub extract_secret: bool,
    /// Minimum entropy threshold for this pattern
    pub min_entropy: Option<f64>,
    /// Optional description of the pattern
    pub description: Option<Arc<str>>,
    /// Optional CWE ID
    pub cwe_id: Option<Arc<str>>,
    /// Optional confidence level
    pub confidence: Option<Arc<str>>,
    /// Optional category
    pub category: Option<Arc<str>>,
}

impl CompiledPattern {
    /// Create a new compiled pattern from configuration
    ///
    /// # Panics
    ///
    /// Panics if the regex pattern is invalid. Use `try_from_config` for fallible construction.
    pub fn from_config(config: &PatternConfig) -> Self {
        Self {
            name: config.name.clone().into(),
            regex: Regex::new(&config.pattern)
                .unwrap_or_else(|e| panic!("Invalid regex pattern '{}': {}", config.name, e)),
            severity: config.severity.clone().into(),
            recommendation: config.recommendation.clone().into(),
            extract_secret: config.extract_secret,
            min_entropy: config.min_entropy,
            description: config.description.as_ref().map(|s| s.clone().into()),
            cwe_id: config.cwe_id.as_ref().map(|s| s.clone().into()),
            confidence: config.confidence.as_ref().map(|s| s.clone().into()),
            category: config.category.as_ref().map(|s| s.clone().into()),
        }
    }

    /// Try to create a compiled pattern from configuration
    ///
    /// Returns an error if the regex pattern is invalid.
    pub fn try_from_config(config: &PatternConfig) -> Result<Self, regex::Error> {
        Ok(Self {
            name: config.name.clone().into(),
            regex: Regex::new(&config.pattern)?,
            severity: config.severity.clone().into(),
            recommendation: config.recommendation.clone().into(),
            extract_secret: config.extract_secret,
            min_entropy: config.min_entropy,
            description: config.description.as_ref().map(|s| s.clone().into()),
            cwe_id: config.cwe_id.as_ref().map(|s| s.clone().into()),
            confidence: config.confidence.as_ref().map(|s| s.clone().into()),
            category: config.category.as_ref().map(|s| s.clone().into()),
        })
    }

    /// Check if the pattern matches a line
    #[inline]
    pub fn is_match(&self, text: &str) -> bool {
        self.regex.is_match(text)
    }

    /// Find all matches in text
    #[inline]
    pub fn find_matches_in<'a>(&self, text: &'a str) -> Vec<regex::Match<'a>> {
        self.regex.find_iter(text).collect()
    }
}

/// Pattern compilation cache
///
/// Holds all compiled patterns for efficient reuse.
/// Thread-safe and designed to be shared via Arc.
///
/// # Example
///
/// ```rust
/// use coax_scanner::{PatternCache, PatternConfig};
/// use std::sync::Arc;
///
/// let patterns = vec![
///     PatternConfig {
///         name: "AWS_KEY",
///         pattern: r"AKIA[0-9A-Z]{16}",
///         severity: "critical",
///         recommendation: "Rotate immediately",
///     },
/// ];
///
/// let cache = Arc::new(PatternCache::new(&patterns));
///
/// // Share cache across threads
/// let cache_clone = Arc::clone(&cache);
/// ```
#[derive(Debug, Clone)]
pub struct PatternCache {
    /// Pre-compiled patterns
    patterns: Vec<CompiledPattern>,
}

impl PatternCache {
    /// Create a new pattern cache from configurations
    ///
    /// All patterns are compiled once during construction.
    ///
    /// # Panics
    ///
    /// Panics if any pattern has an invalid regex.
    pub fn new(patterns: &[PatternConfig]) -> Self {
        let compiled: Vec<CompiledPattern> =
            patterns.iter().map(CompiledPattern::from_config).collect();

        Self { patterns: compiled }
    }

    /// Try to create a pattern cache from configurations
    ///
    /// Returns an error if any pattern has an invalid regex.
    pub fn try_new(patterns: &[PatternConfig]) -> Result<Self, regex::Error> {
        let compiled: Result<Vec<CompiledPattern>, regex::Error> = patterns
            .iter()
            .map(CompiledPattern::try_from_config)
            .collect();

        Ok(Self {
            patterns: compiled?,
        })
    }

    /// Get all compiled patterns
    #[inline]
    pub fn patterns(&self) -> &[CompiledPattern] {
        &self.patterns
    }

    /// Get the number of cached patterns
    #[inline]
    pub fn len(&self) -> usize {
        self.patterns.len()
    }

    /// Check if cache is empty
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.patterns.is_empty()
    }

    /// Find a pattern by name
    pub fn get_by_name(&self, name: &str) -> Option<&CompiledPattern> {
        self.patterns.iter().find(|p| p.name.as_ref() == name)
    }

    /// Iterate over all patterns
    pub fn iter(&self) -> impl Iterator<Item = &CompiledPattern> {
        self.patterns.iter()
    }

    /// Check if any pattern matches the text
    ///
    /// Returns true if at least one pattern matches.
    #[inline]
    pub fn matches_any(&self, text: &str) -> bool {
        self.patterns.iter().any(|p| p.is_match(text))
    }

    /// Find all matching patterns for text
    ///
    /// Returns a vector of pattern names that match.
    pub fn find_matches<'a>(&'a self, text: &'a str) -> Vec<&'a CompiledPattern> {
        self.patterns.iter().filter(|p| p.is_match(text)).collect()
    }
}

/// Default patterns for secret detection
///
/// Based on research from GitGuardian, TruffleHog, and Gitleaks.
/// Includes patterns for:
/// - Cloud provider credentials (AWS, GCP, Azure)
/// - Version control tokens (GitHub, GitLab)
/// - Payment processors (Stripe, Square)
/// - Communication APIs (Slack, Twilio, SendGrid)
/// - Database connection strings
/// - Private keys and certificates
pub fn default_patterns() -> Vec<PatternConfig> {
    vec![
        // AWS Credentials
        PatternConfig::new(
            "AWS_ACCESS_KEY",
            r"AKIA[0-9A-Z]{16}",
            "critical",
            "Remove immediately and rotate the key via AWS IAM Console",
        ),
        PatternConfig::new(
            "AWS_SECRET_KEY",
            r#"(?i)(aws_secret_access_key|aws_secret_key)\s*[:=]\s*['"]?[0-9a-zA-Z/+=]{40}['"]?"#,
            "critical",
            "Remove immediately and rotate the secret via AWS IAM Console",
        ),
        // GitHub Tokens
        PatternConfig::new(
            "GITHUB_PAT",
            r"ghp_[a-zA-Z0-9]{36}",
            "critical",
            "Remove and regenerate the token in GitHub Settings",
        ),
        PatternConfig::new(
            "GITHUB_OAUTH",
            r"gho_[a-zA-Z0-9]{36}",
            "critical",
            "Remove and revoke the OAuth token",
        ),
        PatternConfig::new(
            "GITLAB_PAT",
            r"glpat-[a-zA-Z0-9_-]{20}",
            "critical",
            "Remove and revoke the GitLab token",
        ),
        // Google Cloud
        PatternConfig::new(
            "GOOGLE_API_KEY",
            r"AIza[0-9A-Za-z\\-_]{35}",
            "high",
            "Remove and restrict API key usage in Google Cloud Console",
        ),
        PatternConfig::new(
            "GCP_SERVICE_ACCOUNT",
            r#"(?i)"type"\s*:\s*"service_account""#,
            "high",
            "Remove and revoke service account credentials",
        ),
        // Payment Processors
        PatternConfig::new(
            "STRIPE_SECRET_KEY",
            r"sk_live_[0-9a-zA-Z]{24,}",
            "critical",
            "Remove and rotate immediately in Stripe Dashboard",
        ),
        PatternConfig::new(
            "SQUARE_ACCESS_TOKEN",
            r"sq0atp-[0-9a-zA-Z_-]{22}",
            "critical",
            "Remove and revoke in Square Developer Dashboard",
        ),
        // Communication APIs
        PatternConfig::new(
            "SLACK_TOKEN",
            r"xox[baprs]-[0-9a-zA-Z]{10,48}",
            "high",
            "Remove and revoke the token in Slack Admin",
        ),
        PatternConfig::new(
            "SENDGRID_API_KEY",
            r"SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}",
            "critical",
            "Remove and regenerate in SendGrid Settings",
        ),
        PatternConfig::new(
            "TWILIO_API_KEY",
            r"SK[0-9a-fA-F]{32}",
            "critical",
            "Remove and revoke in Twilio Console",
        ),
        PatternConfig::new(
            "MAILGUN_API_KEY",
            r"key-[0-9a-zA-Z]{32}",
            "critical",
            "Remove and rotate in Mailgun Dashboard",
        ),
        PatternConfig::new(
            "DISCORD_BOT_TOKEN",
            r"MT[a-zA-Z0-9_-]{23}\.[a-zA-Z0-9_-]{6}\.[a-zA-Z0-9_-]{27}",
            "critical",
            "Remove and regenerate in Discord Developer Portal",
        ),
        PatternConfig::new(
            "TELEGRAM_BOT_TOKEN",
            r"[0-9]{9,10}:[a-zA-Z0-9_-]{35}",
            "high",
            "Remove and revoke bot via @BotFather",
        ),
        // Database Connection Strings
        PatternConfig::new(
            "POSTGRESQL_CONNECTION_STRING",
            r"(?i)postgresql://[^:]+:[^@]+@[^/]+/[a-zA-Z0-9_-]+",
            "critical",
            "Remove and rotate database credentials",
        ),
        PatternConfig::new(
            "MONGODB_CONNECTION_STRING",
            r"(?i)mongodb(\+srv)?://[^:]+:[^@]+@[^/]+/?",
            "critical",
            "Remove and rotate MongoDB credentials",
        ),
        PatternConfig::new(
            "MYSQL_CONNECTION_STRING",
            r"(?i)mysql://[^:]+:[^@]+@[^/]+/[a-zA-Z0-9_-]+",
            "critical",
            "Remove and rotate MySQL credentials",
        ),
        PatternConfig::new(
            "REDIS_CONNECTION_STRING",
            r"(?i)redis://:[^@]+@[^/]+",
            "critical",
            "Remove and change Redis AUTH password",
        ),
        PatternConfig::new(
            "AZURE_STORAGE_CONNECTION_STRING",
            r"(?i)DefaultEndpointsProtocol=https;AccountName=[^;]+;AccountKey=[A-Za-z0-9+/=]{88};",
            "critical",
            "Remove and regenerate key in Azure Portal",
        ),
        // Package Manager Tokens
        PatternConfig::new(
            "NPM_ACCESS_TOKEN",
            r"npm_[a-zA-Z0-9]{36}",
            "critical",
            "Remove and revoke in npm Access Tokens",
        ),
        PatternConfig::new(
            "PYPI_API_TOKEN",
            r"pypi-[a-zA-Z0-9_-]{50,}",
            "critical",
            "Remove and revoke in PyPI Settings",
        ),
        // AI/ML APIs
        PatternConfig::new(
            "OPENAI_API_KEY",
            r"sk-proj-[a-zA-Z0-9]{48}",
            "critical",
            "Remove and revoke in OpenAI Platform",
        ),
        PatternConfig::new(
            "ANTHROPIC_API_KEY",
            r"sk-ant-api[0-9]{2}-[a-zA-Z0-9_-]{49}",
            "critical",
            "Remove and revoke in Anthropic Console",
        ),
        PatternConfig::new(
            "HUGGINGFACE_TOKEN",
            r"hf_[a-zA-Z0-9]{34}",
            "critical",
            "Remove and delete in Hugging Face Settings",
        ),
        // Cloud Providers
        PatternConfig::new(
            "HEROKU_API_KEY",
            r#"(?i)heroku.*['"][0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}['"]"#,
            "critical",
            "Remove and regenerate in Heroku Settings",
        ),
        PatternConfig::new(
            "DIGITALOCEAN_TOKEN",
            r"dop_v1_[a-f0-9]{64}",
            "high",
            "Remove and revoke in DigitalOcean Dashboard",
        ),
        // Private Keys
        PatternConfig::new(
            "PRIVATE_KEY_HEADER",
            r"-----BEGIN (RSA |EC |DSA |OPENSSH |ENCRYPTED )?PRIVATE KEY-----",
            "critical",
            "Remove immediately - private key exposed",
        ),
        PatternConfig::new(
            "SSH_PRIVATE_KEY",
            r"-----BEGIN OPENSSH PRIVATE KEY-----",
            "critical",
            "Remove and revoke SSH key from all servers",
        ),
        // Tokens and Sessions
        PatternConfig::new(
            "JWT_TOKEN",
            r"eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*",
            "high",
            "Remove and invalidate session",
        ),
        // Generic Secrets
        PatternConfig::new(
            "GENERIC_PASSWORD",
            r#"(?i)(password|passwd|pwd)\s*[:=]\s*['"][^'"]{8,}['"]"#,
            "high",
            "Use environment variables or secret manager",
        ),
        PatternConfig::new(
            "GENERIC_SECRET",
            r#"(?i)(password|secret|key|token)\s*[:=]\s*['"][^'"]{8,}['"]"#,
            "high",
            "Use environment variables or secret manager",
        ),
        // Note: High entropy detection is disabled by default due to high false positive rate
        // Enable only with manual review and tuned thresholds
        PatternConfig::new(
            "HIGH_ENTROPY_STRING",
            r"[a-zA-Z0-9+/=_-]{40,}",
            "low",
            "Review - may be false positive (entropy-based detection)",
        )
        .with_enabled(false)
        .with_min_entropy(4.5)
        .with_extract_secret(false),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pattern_cache_creation() {
        let patterns = default_patterns();
        let cache = PatternCache::new(&patterns);
        assert!(!cache.is_empty());
        assert!(cache.len() > 30);
    }

    #[test]
    fn test_pattern_matching() {
        let patterns = vec![PatternConfig::new(
            "TEST_AWS",
            r"AKIA[0-9A-Z]{16}",
            "critical",
            "Test",
        )];

        let cache = PatternCache::new(&patterns);

        assert!(cache.matches_any("AWS_KEY=AKIAIOSFODNN7EXAMPLE"));
        assert!(!cache.matches_any("clean content"));
    }

    #[test]
    fn test_find_matches() {
        let patterns = default_patterns();
        let cache = PatternCache::new(&patterns);

        let text = "AWS_KEY=AKIAIOSFODNN7EXAMPLE";
        let matches = cache.find_matches(text);

        assert!(!matches.is_empty());
        assert!(matches.iter().any(|p| p.name.as_ref() == "AWS_ACCESS_KEY"));
    }

    #[test]
    fn test_pattern_cache_cloning() {
        let patterns = default_patterns();
        let cache = PatternCache::new(&patterns);

        // Clone should be cheap (Arc-based)
        let cache_clone = cache.clone();
        assert_eq!(cache.len(), cache_clone.len());
    }

    #[test]
    fn test_try_from_config_invalid() {
        let pattern = PatternConfig::new("INVALID", r"[invalid(regex", "high", "Test");

        let result = CompiledPattern::try_from_config(&pattern);
        assert!(result.is_err());
    }

    #[test]
    fn test_get_by_name() {
        let patterns = default_patterns();
        let cache = PatternCache::new(&patterns);

        let aws_pattern = cache.get_by_name("AWS_ACCESS_KEY");
        assert!(aws_pattern.is_some());

        let nonexistent = cache.get_by_name("NONEXISTENT");
        assert!(nonexistent.is_none());
    }
}
