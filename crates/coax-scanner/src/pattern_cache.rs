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
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternConfig {
    /// Unique identifier for the pattern
    pub name: &'static str,
    /// Regex pattern string
    pub pattern: &'static str,
    /// Severity level: critical, high, medium, low
    pub severity: &'static str,
    /// Recommendation for remediation
    pub recommendation: &'static str,
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
}

impl CompiledPattern {
    /// Create a new compiled pattern from configuration
    ///
    /// # Panics
    ///
    /// Panics if the regex pattern is invalid. Use `try_from_config` for fallible construction.
    pub fn from_config(config: &PatternConfig) -> Self {
        Self {
            name: config.name.into(),
            regex: Regex::new(config.pattern)
                .unwrap_or_else(|e| panic!("Invalid regex pattern '{}': {}", config.name, e)),
            severity: config.severity.into(),
            recommendation: config.recommendation.into(),
        }
    }

    /// Try to create a compiled pattern from configuration
    ///
    /// Returns an error if the regex pattern is invalid.
    pub fn try_from_config(config: &PatternConfig) -> Result<Self, regex::Error> {
        Ok(Self {
            name: config.name.into(),
            regex: Regex::new(config.pattern)?,
            severity: config.severity.into(),
            recommendation: config.recommendation.into(),
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
        let compiled: Vec<CompiledPattern> = patterns
            .iter()
            .map(CompiledPattern::from_config)
            .collect();

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

        Ok(Self { patterns: compiled? })
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
        self.patterns
            .iter()
            .filter(|p| p.is_match(text))
            .collect()
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
        PatternConfig {
            name: "AWS_ACCESS_KEY",
            pattern: r"AKIA[0-9A-Z]{16}",
            severity: "critical",
            recommendation: "Remove immediately and rotate the key via AWS IAM Console",
        },
        PatternConfig {
            name: "AWS_SECRET_KEY",
            pattern: r"(?i)(aws_secret_access_key|aws_secret_key)\s*[:=]\s*[\x27\x22]?[0-9a-zA-Z\/+]{40}[\x27\x22]?",
            severity: "critical",
            recommendation: "Remove immediately and rotate the secret via AWS IAM Console",
        },
        // GitHub Tokens
        PatternConfig {
            name: "GITHUB_PAT",
            pattern: r"ghp_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and regenerate the token in GitHub Settings",
        },
        PatternConfig {
            name: "GITHUB_OAUTH",
            pattern: r"gho_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and revoke the OAuth token",
        },
        PatternConfig {
            name: "GITLAB_PAT",
            pattern: r"glpat-[a-zA-Z0-9_-]{20}",
            severity: "critical",
            recommendation: "Remove and revoke the GitLab token",
        },
        // Google Cloud
        PatternConfig {
            name: "GOOGLE_API_KEY",
            pattern: r"AIza[0-9A-Za-z\\-_]{35}",
            severity: "high",
            recommendation: "Remove and restrict API key usage in Google Cloud Console",
        },
        PatternConfig {
            name: "GCP_SERVICE_ACCOUNT",
            pattern: r#"(?i)"type"\s*:\s*"service_account""#,
            severity: "high",
            recommendation: "Remove and revoke service account credentials",
        },
        // Payment Processors
        PatternConfig {
            name: "STRIPE_SECRET_KEY",
            pattern: r"sk_live_[0-9a-zA-Z]{24,}",
            severity: "critical",
            recommendation: "Remove and rotate immediately in Stripe Dashboard",
        },
        PatternConfig {
            name: "SQUARE_ACCESS_TOKEN",
            pattern: r"sq0atp-[0-9a-zA-Z_-]{22}",
            severity: "critical",
            recommendation: "Remove and revoke in Square Developer Dashboard",
        },
        // Communication APIs
        PatternConfig {
            name: "SLACK_TOKEN",
            pattern: r"xox[baprs]-[0-9a-zA-Z]{10,48}",
            severity: "high",
            recommendation: "Remove and revoke the token in Slack Admin",
        },
        PatternConfig {
            name: "SENDGRID_API_KEY",
            pattern: r"SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}",
            severity: "critical",
            recommendation: "Remove and regenerate in SendGrid Settings",
        },
        PatternConfig {
            name: "TWILIO_API_KEY",
            pattern: r"SK[0-9a-fA-F]{32}",
            severity: "critical",
            recommendation: "Remove and revoke in Twilio Console",
        },
        PatternConfig {
            name: "MAILGUN_API_KEY",
            pattern: r"key-[0-9a-zA-Z]{32}",
            severity: "critical",
            recommendation: "Remove and rotate in Mailgun Dashboard",
        },
        PatternConfig {
            name: "DISCORD_BOT_TOKEN",
            pattern: r"MT[a-zA-Z0-9_-]{23}\.[a-zA-Z0-9_-]{6}\.[a-zA-Z0-9_-]{27}",
            severity: "critical",
            recommendation: "Remove and regenerate in Discord Developer Portal",
        },
        PatternConfig {
            name: "TELEGRAM_BOT_TOKEN",
            pattern: r"[0-9]{9,10}:[a-zA-Z0-9_-]{35}",
            severity: "high",
            recommendation: "Remove and revoke bot via @BotFather",
        },
        // Database Connection Strings
        PatternConfig {
            name: "POSTGRESQL_CONNECTION_STRING",
            pattern: r"(?i)postgresql://[^:]+:[^@]+@[^/]+/[a-zA-Z0-9_-]+",
            severity: "critical",
            recommendation: "Remove and rotate database credentials",
        },
        PatternConfig {
            name: "MONGODB_CONNECTION_STRING",
            pattern: r"(?i)mongodb(\+srv)?://[^:]+:[^@]+@[^/]+/?",
            severity: "critical",
            recommendation: "Remove and rotate MongoDB credentials",
        },
        PatternConfig {
            name: "MYSQL_CONNECTION_STRING",
            pattern: r"(?i)mysql://[^:]+:[^@]+@[^/]+/[a-zA-Z0-9_-]+",
            severity: "critical",
            recommendation: "Remove and rotate MySQL credentials",
        },
        PatternConfig {
            name: "REDIS_CONNECTION_STRING",
            pattern: r"(?i)redis://:[^@]+@[^/]+",
            severity: "critical",
            recommendation: "Remove and change Redis AUTH password",
        },
        PatternConfig {
            name: "AZURE_STORAGE_CONNECTION_STRING",
            pattern: r"(?i)DefaultEndpointsProtocol=https;AccountName=[^;]+;AccountKey=[A-Za-z0-9+/=]{88};",
            severity: "critical",
            recommendation: "Remove and regenerate key in Azure Portal",
        },
        // Package Manager Tokens
        PatternConfig {
            name: "NPM_ACCESS_TOKEN",
            pattern: r"npm_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and revoke in npm Access Tokens",
        },
        PatternConfig {
            name: "PYPI_API_TOKEN",
            pattern: r"pypi-[a-zA-Z0-9_-]{50,}",
            severity: "critical",
            recommendation: "Remove and revoke in PyPI Settings",
        },
        // AI/ML APIs
        PatternConfig {
            name: "OPENAI_API_KEY",
            pattern: r"sk-proj-[a-zA-Z0-9]{48}",
            severity: "critical",
            recommendation: "Remove and revoke in OpenAI Platform",
        },
        PatternConfig {
            name: "ANTHROPIC_API_KEY",
            pattern: r"sk-ant-api[0-9]{2}-[a-zA-Z0-9_-]{49}",
            severity: "critical",
            recommendation: "Remove and revoke in Anthropic Console",
        },
        PatternConfig {
            name: "HUGGINGFACE_TOKEN",
            pattern: r"hf_[a-zA-Z0-9]{34}",
            severity: "critical",
            recommendation: "Remove and delete in Hugging Face Settings",
        },
        // Cloud Providers
        PatternConfig {
            name: "HEROKU_API_KEY",
            pattern: r"(?i)heroku.*[\x27\x22][0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}[\x27\x22]",
            severity: "critical",
            recommendation: "Remove and regenerate in Heroku Settings",
        },
        PatternConfig {
            name: "DIGITALOCEAN_TOKEN",
            pattern: r"dop_v1_[a-f0-9]{64}",
            severity: "high",
            recommendation: "Remove and revoke in DigitalOcean Dashboard",
        },
        // Private Keys
        PatternConfig {
            name: "PRIVATE_KEY_HEADER",
            pattern: r"-----BEGIN (RSA |EC |DSA |OPENSSH |ENCRYPTED )?PRIVATE KEY-----",
            severity: "critical",
            recommendation: "Remove immediately - private key exposed",
        },
        PatternConfig {
            name: "SSH_PRIVATE_KEY",
            pattern: r"-----BEGIN OPENSSH PRIVATE KEY-----",
            severity: "critical",
            recommendation: "Remove and revoke SSH key from all servers",
        },
        // Tokens and Sessions
        PatternConfig {
            name: "JWT_TOKEN",
            pattern: r"eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*",
            severity: "high",
            recommendation: "Remove and invalidate session",
        },
        // Generic Secrets
        PatternConfig {
            name: "GENERIC_PASSWORD",
            pattern: r"(?i)(password|passwd|pwd)\s*[:=]\s*[\x27\x22][^\x27\x22]{8,}[\x27\x22]",
            severity: "high",
            recommendation: "Use environment variables or secret manager",
        },
        PatternConfig {
            name: "GENERIC_SECRET",
            pattern: r"(?i)(password|secret|key|token)\s*[:=]\s*[\x27\x22][^\x27\x22]{8,}[\x27\x22]",
            severity: "high",
            recommendation: "Use environment variables or secret manager",
        },
        PatternConfig {
            name: "HIGH_ENTROPY_STRING",
            pattern: r"[a-zA-Z0-9+/=_-]{40,}",
            severity: "medium",
            recommendation: "Review - may be false positive",
        },
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
        let patterns = vec![PatternConfig {
            name: "TEST_AWS",
            pattern: r"AKIA[0-9A-Z]{16}",
            severity: "critical",
            recommendation: "Test",
        }];

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
        let pattern = PatternConfig {
            name: "INVALID",
            pattern: r"[invalid(regex",
            severity: "high",
            recommendation: "Test",
        };

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
