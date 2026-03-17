//! Secret Pattern Definitions
//!
//! This module provides the secret pattern configurations used by the scanner.

use crate::pattern_cache::PatternConfig;

/// Secret pattern with full metadata
#[derive(Debug, Clone)]
pub struct SecretPattern {
    pub name: &'static str,
    pub pattern: &'static str,
    pub severity: &'static str,
    pub recommendation: &'static str,
    pub description: &'static str,
    pub cwe_id: Option<&'static str>,
}

impl SecretPattern {
    /// Convert to PatternConfig for scanner use
    pub fn to_config(&self) -> PatternConfig {
        PatternConfig::new(self.name, self.pattern, self.severity, self.recommendation)
    }
}

/// All secret patterns organized by category
pub mod categories {
    use super::SecretPattern;

    /// AWS credentials patterns
    pub const AWS: &[SecretPattern] = &[
        SecretPattern {
            name: "AWS_ACCESS_KEY",
            pattern: r"AKIA[0-9A-Z]{16}",
            severity: "critical",
            recommendation: "Remove immediately and rotate the key via AWS IAM Console",
            description: "AWS Access Key ID",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "AWS_SECRET_KEY",
            pattern: r"(?i)(aws_secret_access_key|aws_secret_key)\s*[:=]\s*[\x27\x22]?[0-9a-zA-Z\/+]{40}[\x27\x22]?",
            severity: "critical",
            recommendation: "Remove immediately and rotate the secret via AWS IAM Console",
            description: "AWS Secret Access Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "AWS_SESSION_TOKEN",
            pattern: r"(?i)aws_session_token\s*[:=]\s*[\x27\x22]?[A-Za-z0-9/+=]{100,}[\x27\x22]?",
            severity: "high",
            recommendation: "Remove and regenerate session token",
            description: "AWS Session Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// GitHub tokens patterns
    pub const GITHUB: &[SecretPattern] = &[
        SecretPattern {
            name: "GITHUB_PAT",
            pattern: r"ghp_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and regenerate the token in GitHub Settings",
            description: "GitHub Personal Access Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "GITHUB_OAUTH",
            pattern: r"gho_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and revoke the OAuth token",
            description: "GitHub OAuth Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "GITHUB_APP_TOKEN",
            pattern: r"ghu_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and revoke the app installation token",
            description: "GitHub App Installation Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "GITHUB_SSH_KEY",
            pattern: r"ghs_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and revoke the SSH key",
            description: "GitHub SSH Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "GITLAB_PAT",
            pattern: r"glpat-[a-zA-Z0-9_-]{20}",
            severity: "critical",
            recommendation: "Remove and revoke the GitLab token",
            description: "GitLab Personal Access Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Cloud provider patterns
    pub const CLOUD_PROVIDERS: &[SecretPattern] = &[
        SecretPattern {
            name: "GOOGLE_API_KEY",
            pattern: r"AIza[0-9A-Za-z\\-_]{35}",
            severity: "high",
            recommendation: "Remove and restrict API key usage in Google Cloud Console",
            description: "Google Cloud API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "GCP_SERVICE_ACCOUNT",
            pattern: r#"(?i)"type"\s*:\s*"service_account""#,
            severity: "high",
            recommendation: "Remove and revoke service account credentials",
            description: "Google Cloud Service Account Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "AZURE_STORAGE_CONNECTION_STRING",
            pattern: r"(?i)DefaultEndpointsProtocol=https;AccountName=[^;]+;AccountKey=[A-Za-z0-9+/=]{88};",
            severity: "critical",
            recommendation: "Remove and regenerate key in Azure Portal",
            description: "Azure Storage Connection String",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "AZURE_CLIENT_SECRET",
            pattern: r"(?i)azure.*client.*secret.*[:=]\s*[\x27\x22]?[^\x27\x22\s]{8,}",
            severity: "critical",
            recommendation: "Remove and rotate in Azure AD",
            description: "Azure Client Secret",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "AZURE_SAS_TOKEN",
            pattern: r"\?sv=[0-9-]+&ss=[a-z]+&srt=[a-z]+&sp=[a-z]+&se=[0-9T:Z-]+&st=[0-9T:Z-]+&spr=https&sig=[a-zA-Z0-9]+",
            severity: "critical",
            recommendation: "Remove and regenerate SAS token in Azure Portal",
            description: "Azure Shared Access Signature Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "HEROKU_API_KEY",
            pattern: r"(?i)heroku.*[\x27\x22][0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}[\x27\x22]",
            severity: "critical",
            recommendation: "Remove and regenerate in Heroku Settings",
            description: "Heroku API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "DIGITALOCEAN_TOKEN",
            pattern: r"dop_v1_[a-f0-9]{64}",
            severity: "high",
            recommendation: "Remove and revoke in DigitalOcean Dashboard",
            description: "DigitalOcean API Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Payment processor patterns
    pub const PAYMENT: &[SecretPattern] = &[
        SecretPattern {
            name: "STRIPE_SECRET_KEY",
            pattern: r"sk_live_[0-9a-zA-Z]{24,}",
            severity: "critical",
            recommendation: "Remove and rotate immediately in Stripe Dashboard",
            description: "Stripe Live Secret Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "STRIPE_TEST_KEY",
            pattern: r"sk_test_[0-9a-zA-Z]{24,}",
            severity: "low",
            recommendation: "Remove test key (not production sensitive)",
            description: "Stripe Test Secret Key",
            cwe_id: None,
        },
        SecretPattern {
            name: "SQUARE_ACCESS_TOKEN",
            pattern: r"sq0atp-[0-9a-zA-Z_-]{22}",
            severity: "critical",
            recommendation: "Remove and revoke in Square Developer Dashboard",
            description: "Square Access Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "PAYPAL_ACCESS_TOKEN",
            pattern: r"(?i)access_token\$production\$[0-9a-z]{16}\$[0-9a-f]{32}",
            severity: "critical",
            recommendation: "Remove and revoke in PayPal Developer Dashboard",
            description: "PayPal Access Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Communication API patterns
    pub const COMMUNICATION: &[SecretPattern] = &[
        SecretPattern {
            name: "SLACK_TOKEN",
            pattern: r"xox[baprs]-[0-9a-zA-Z]{10,48}",
            severity: "high",
            recommendation: "Remove and revoke the token in Slack Admin",
            description: "Slack Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "SENDGRID_API_KEY",
            pattern: r"SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}",
            severity: "critical",
            recommendation: "Remove and regenerate in SendGrid Settings",
            description: "SendGrid API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "TWILIO_API_KEY",
            pattern: r"SK[0-9a-fA-F]{32}",
            severity: "critical",
            recommendation: "Remove and revoke in Twilio Console",
            description: "Twilio API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "TWILIO_ACCOUNT_SID",
            pattern: r"AC[a-zA-Z0-9_]{32}",
            severity: "high",
            recommendation: "Remove and review account access in Twilio Console",
            description: "Twilio Account SID",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "DATADOG_API_KEY",
            pattern: r"(?i)(?:DD|DATADOG)[_-]?(?:API)[_-]?KEY\s*[:=]\s*[0-9a-f]{32}",
            severity: "high",
            recommendation: "Remove and regenerate in Datadog API Settings",
            description: "Datadog API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "DATADOG_APP_KEY",
            pattern: r"(?i)(?:DD|DATADOG)[_-]?(?:APP|APPLICATION)[_-]?KEY\s*[:=]\s*[a-f0-9]{40}",
            severity: "high",
            recommendation: "Remove and regenerate in Datadog API Settings",
            description: "Datadog Application Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "MAILGUN_API_KEY",
            pattern: r"key-[0-9a-zA-Z]{32}",
            severity: "critical",
            recommendation: "Remove and rotate in Mailgun Dashboard",
            description: "Mailgun API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "DISCORD_BOT_TOKEN",
            pattern: r"MT[a-zA-Z0-9_-]{23}\.[a-zA-Z0-9_-]{6}\.[a-zA-Z0-9_-]{27}",
            severity: "critical",
            recommendation: "Remove and regenerate in Discord Developer Portal",
            description: "Discord Bot Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "TELEGRAM_BOT_TOKEN",
            pattern: r"[0-9]{9,10}:[a-zA-Z0-9_-]{35}",
            severity: "high",
            recommendation: "Remove and revoke bot via @BotFather",
            description: "Telegram Bot Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Database connection string patterns
    pub const DATABASE: &[SecretPattern] = &[
        SecretPattern {
            name: "POSTGRESQL_CONNECTION_STRING",
            pattern: r"(?i)postgresql://[^:]+:[^@]+@[^/]+/[a-zA-Z0-9_-]+",
            severity: "critical",
            recommendation: "Remove and rotate database credentials",
            description: "PostgreSQL Connection String",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "MONGODB_CONNECTION_STRING",
            pattern: r"(?i)mongodb(\+srv)?://[^:]+:[^@]+@[^/]+/?",
            severity: "critical",
            recommendation: "Remove and rotate MongoDB credentials",
            description: "MongoDB Connection String",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "MYSQL_CONNECTION_STRING",
            pattern: r"(?i)mysql://[^:]+:[^@]+@[^/]+/[a-zA-Z0-9_-]+",
            severity: "critical",
            recommendation: "Remove and rotate MySQL credentials",
            description: "MySQL Connection String",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "REDIS_CONNECTION_STRING",
            pattern: r"(?i)redis://:[^@]+@[^/]+",
            severity: "critical",
            recommendation: "Remove and change Redis AUTH password",
            description: "Redis Connection String",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "MSSQL_CONNECTION_STRING",
            pattern: r"(?i)Server=.*;Database=.*;User Id=.*;Password=.*;",
            severity: "critical",
            recommendation: "Remove and rotate SQL Server credentials",
            description: "Microsoft SQL Server Connection String",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Package manager token patterns
    pub const PACKAGE_MANAGER: &[SecretPattern] = &[
        SecretPattern {
            name: "NPM_ACCESS_TOKEN",
            pattern: r"npm_[a-zA-Z0-9]{36}",
            severity: "critical",
            recommendation: "Remove and revoke in npm Access Tokens",
            description: "npm Access Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "PYPI_API_TOKEN",
            pattern: r"pypi-[a-zA-Z0-9_-]{50,}",
            severity: "critical",
            recommendation: "Remove and revoke in PyPI Settings",
            description: "PyPI API Token",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "RUBY_GEMS_TOKEN",
            pattern: r"rubygems_[a-zA-Z0-9]{48}",
            severity: "critical",
            recommendation: "Remove and revoke in RubyGems Settings",
            description: "RubyGems API Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// AI/ML API patterns
    pub const AI_ML: &[SecretPattern] = &[
        SecretPattern {
            name: "OPENAI_API_KEY",
            pattern: r"sk-proj-[a-zA-Z0-9]{48}",
            severity: "critical",
            recommendation: "Remove and revoke in OpenAI Platform",
            description: "OpenAI API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "ANTHROPIC_API_KEY",
            pattern: r"sk-ant-api[0-9]{2}-[a-zA-Z0-9_-]{49}",
            severity: "critical",
            recommendation: "Remove and revoke in Anthropic Console",
            description: "Anthropic API Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "HUGGINGFACE_TOKEN",
            pattern: r"hf_[a-zA-Z0-9]{34}",
            severity: "critical",
            recommendation: "Remove and delete in Hugging Face Settings",
            description: "Hugging Face Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Private key patterns
    pub const PRIVATE_KEYS: &[SecretPattern] = &[
        SecretPattern {
            name: "PRIVATE_KEY_HEADER",
            pattern: r"-----BEGIN (RSA |EC |DSA |OPENSSH |ENCRYPTED )?PRIVATE KEY-----",
            severity: "critical",
            recommendation: "Remove immediately - private key exposed",
            description: "Private Key Header",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "SSH_PRIVATE_KEY",
            pattern: r"-----BEGIN OPENSSH PRIVATE KEY-----",
            severity: "critical",
            recommendation: "Remove and revoke SSH key from all servers",
            description: "SSH Private Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "RSA_PRIVATE_KEY",
            pattern: r"-----BEGIN RSA PRIVATE KEY-----",
            severity: "critical",
            recommendation: "Remove and regenerate RSA key pair",
            description: "RSA Private Key",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "EC_PRIVATE_KEY",
            pattern: r"-----BEGIN EC PRIVATE KEY-----",
            severity: "critical",
            recommendation: "Remove and regenerate EC key pair",
            description: "EC Private Key",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Token and session patterns
    pub const TOKENS: &[SecretPattern] = &[
        SecretPattern {
            name: "JWT_TOKEN",
            pattern: r"eyJ[a-zA-Z0-9_-]*\.eyJ[a-zA-Z0-9_-]*\.[a-zA-Z0-9_-]*",
            severity: "high",
            recommendation: "Remove and invalidate session",
            description: "JWT Token",
            cwe_id: Some("CWE-798"),
        },
    ];

    /// Generic secret patterns
    pub const GENERIC: &[SecretPattern] = &[
        SecretPattern {
            name: "GENERIC_PASSWORD",
            // FP REDUCTION: Require actual quoted value, don't match function calls or variable references
            pattern: r"(?i)(password|passwd|pwd)\s*[:=]\s*[\x27\x22][^\x27\x22]{8,}[\x27\x22]",
            severity: "medium",  // FP REDUCTION: Reduced from high
            recommendation: "Use environment variables or secret manager",
            description: "Generic Password Assignment",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "GENERIC_SECRET",
            // FP REDUCTION: Require actual quoted value
            pattern: r"(?i)(password|secret|key|token)\s*[:=]\s*[\x27\x22][^\x27\x22]{8,}[\x27\x22]",
            severity: "medium",  // FP REDUCTION: Reduced from high
            recommendation: "Use environment variables or secret manager",
            description: "Generic Secret Assignment",
            cwe_id: Some("CWE-798"),
        },
        SecretPattern {
            name: "HIGH_ENTROPY_STRING",
            // FP REDUCTION: Simpler pattern without lookbehind (not supported by regex crate)
            // Matches: long alphanumeric strings (40+ chars) - reduced severity and disabled by default
            // Use entropy filter in token_efficiency.rs for more sophisticated detection
            pattern: r"[a-zA-Z0-9]{40,}",
            severity: "low",  // FP REDUCTION: Reduced from medium
            recommendation: "Review - likely false positive, check context and entropy",
            description: "High Entropy String (potential secret) - DISABLED BY DEFAULT",
            cwe_id: Some("CWE-798"),
        },
    ];
}

/// Get all secret patterns as PatternConfigs
pub fn all_patterns() -> Vec<crate::pattern_cache::PatternConfig> {
    use categories::*;

    let all_patterns: Vec<&SecretPattern> = AWS
        .iter()
        .chain(GITHUB.iter())
        .chain(CLOUD_PROVIDERS.iter())
        .chain(PAYMENT.iter())
        .chain(COMMUNICATION.iter())
        .chain(DATABASE.iter())
        .chain(PACKAGE_MANAGER.iter())
        .chain(AI_ML.iter())
        .chain(PRIVATE_KEYS.iter())
        .chain(TOKENS.iter())
        .chain(GENERIC.iter())
        .collect();

    all_patterns.iter().map(|p| p.to_config()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_patterns_count() {
        let patterns = all_patterns();
        assert!(patterns.len() > 40); // Should have many patterns
    }

    #[test]
    fn test_category_counts() {
        assert!(!categories::AWS.is_empty());
        assert!(!categories::GITHUB.is_empty());
        assert!(!categories::CLOUD_PROVIDERS.is_empty());
        assert!(!categories::PAYMENT.is_empty());
        assert!(!categories::COMMUNICATION.is_empty());
        assert!(!categories::DATABASE.is_empty());
        assert!(!categories::PACKAGE_MANAGER.is_empty());
        assert!(!categories::AI_ML.is_empty());
        assert!(!categories::PRIVATE_KEYS.is_empty());
        assert!(!categories::TOKENS.is_empty());
        assert!(!categories::GENERIC.is_empty());
    }

    #[test]
    fn test_pattern_conversion() {
        let pattern = &categories::AWS[0];
        let config = pattern.to_config();

        assert_eq!(config.name, pattern.name);
        assert_eq!(config.pattern, pattern.pattern);
        assert_eq!(config.severity, pattern.severity);
        assert_eq!(config.recommendation, pattern.recommendation);
    }
}
