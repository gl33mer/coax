//! STRIDE Categorization Module
//!
//! This module provides STRIDE threat categorization for security findings.
//! STRIDE stands for: Spoofing, Tampering, Repudiation, Information Disclosure,
//! Denial of Service, and Elevation of Privilege.

use crate::model::{Impact, Likelihood, StrideCategory};
use coax_scanner::ScanResult;

/// Categorize a finding into STRIDE categories
pub fn categorize_finding_stride(pattern: &str) -> Vec<StrideCategory> {
    match pattern.to_uppercase().as_str() {
        // AWS credentials - can be used for spoofing and lead to info disclosure
        "AWS_ACCESS_KEY" | "AWS_SECRET_KEY" | "AWS_SESSION_TOKEN" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
            StrideCategory::ElevationOfPrivilege,
        ],

        // GitHub tokens - authentication bypass
        "GITHUB_PAT" | "GITHUB_TOKEN" | "GITHUB_OAUTH" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
            StrideCategory::Tampering,
        ],

        // Database credentials - data access and modification
        "DATABASE_URL" | "DATABASE_PASSWORD" | "MYSQL_PASSWORD" | "POSTGRES_PASSWORD" => vec![
            StrideCategory::Spoofing,
            StrideCategory::Tampering,
            StrideCategory::InformationDisclosure,
        ],

        // Private keys - authentication bypass
        "PRIVATE_KEY" | "RSA_PRIVATE_KEY" | "SSH_PRIVATE_KEY" | "EC_PRIVATE_KEY" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
            StrideCategory::ElevationOfPrivilege,
        ],

        // API keys - service impersonation
        "STRIPE_KEY" | "STRIPE_SECRET" | "TWILIO_API_KEY" | "SENDGRID_API_KEY" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
        ],

        // JWT secrets - token forgery
        "JWT_SECRET" | "JWT_KEY" | "SESSION_SECRET" => vec![
            StrideCategory::Spoofing,
            StrideCategory::Tampering,
            StrideCategory::ElevationOfPrivilege,
        ],

        // Encryption keys - data protection bypass
        "ENCRYPTION_KEY" | "AES_KEY" | "CRYPTO_KEY" => vec![
            StrideCategory::InformationDisclosure,
            StrideCategory::Tampering,
        ],

        // OAuth secrets - authentication bypass
        "OAUTH_CLIENT_SECRET" | "GOOGLE_CLIENT_SECRET" | "FACEBOOK_APP_SECRET" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
        ],

        // Slack/Discord webhooks - impersonation
        "SLACK_WEBHOOK" | "DISCORD_WEBHOOK" => vec![
            StrideCategory::Spoofing,
            StrideCategory::Tampering,
        ],

        // Mailgun/SendGrid - email spoofing
        "MAILGUN_API_KEY" | "SENDGRID_KEY" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
        ],

        // Cloud provider keys
        "AZURE_CLIENT_SECRET" | "GCP_CREDENTIALS" | "DIGITALOCEAN_TOKEN" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
            StrideCategory::ElevationOfPrivilege,
        ],

        // Generic passwords
        "PASSWORD" | "ADMIN_PASSWORD" | "ROOT_PASSWORD" => vec![
            StrideCategory::Spoofing,
            StrideCategory::ElevationOfPrivilege,
        ],

        // Certificates
        "CERTIFICATE" | "SSL_CERT" | "TLS_CERT" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
        ],

        // Webhook secrets
        "WEBHOOK_SECRET" | "HOOK_SECRET" => vec![
            StrideCategory::Spoofing,
            StrideCategory::Tampering,
        ],

        // Generic API keys
        "API_KEY" | "API_SECRET" | "SECRET_KEY" => vec![
            StrideCategory::Spoofing,
            StrideCategory::InformationDisclosure,
        ],

        // Default - no specific STRIDE category
        _ => vec![],
    }
}

/// Calculate risk score from likelihood and impact
pub fn calculate_risk_score(likelihood: Likelihood, impact: Impact) -> u32 {
    (likelihood as u32) * (impact as u32)
}

/// Determine likelihood based on finding characteristics
pub fn determine_likelihood(finding: &ScanResult, is_exposed: bool) -> Likelihood {
    // Base likelihood on severity and exposure
    let base_likelihood = match finding.severity.to_lowercase().as_str() {
        "critical" => Likelihood::High,
        "high" => Likelihood::Medium,
        "medium" => Likelihood::Low,
        _ => Likelihood::Low,
    };

    // Increase likelihood if exposed in accessible location
    if is_exposed {
        match base_likelihood {
            Likelihood::VeryLow => Likelihood::Low,
            Likelihood::Low => Likelihood::Medium,
            Likelihood::Medium => Likelihood::High,
            Likelihood::High => Likelihood::VeryHigh,
            Likelihood::VeryHigh => Likelihood::VeryHigh,
        }
    } else {
        base_likelihood
    }
}

/// Determine impact based on finding type
pub fn determine_impact(pattern: &str) -> Impact {
    match pattern.to_uppercase().as_str() {
        // Highest impact - full system compromise
        "AWS_ACCESS_KEY" | "AWS_SECRET_KEY" | "PRIVATE_KEY" | "RSA_PRIVATE_KEY" => {
            Impact::Catastrophic
        }

        // High impact - significant data access
        "DATABASE_URL" | "DATABASE_PASSWORD" | "JWT_SECRET" | "ENCRYPTION_KEY" => {
            Impact::Major
        }

        // Medium-high impact - service access
        "GITHUB_PAT" | "GITHUB_TOKEN" | "STRIPE_KEY" | "OAUTH_CLIENT_SECRET" => {
            Impact::Moderate
        }

        // Medium impact - limited scope
        "API_KEY" | "WEBHOOK_SECRET" | "SLACK_WEBHOOK" => Impact::Minor,

        // Default
        _ => Impact::Moderate,
    }
}

/// Convert severity to likelihood
pub fn severity_to_likelihood(severity: &str) -> Likelihood {
    match severity.to_lowercase().as_str() {
        "critical" => Likelihood::VeryHigh,
        "high" => Likelihood::High,
        "medium" => Likelihood::Medium,
        "low" => Likelihood::Low,
        _ => Likelihood::Medium,
    }
}

/// Convert severity to impact
pub fn severity_to_impact(severity: &str) -> Impact {
    match severity.to_lowercase().as_str() {
        "critical" => Impact::Catastrophic,
        "high" => Impact::Major,
        "medium" => Impact::Moderate,
        "low" => Impact::Minor,
        _ => Impact::Moderate,
    }
}

/// Get STRIDE description
pub fn stride_description(category: StrideCategory) -> &'static str {
    match category {
        StrideCategory::Spoofing => {
            "Attacker can impersonate a legitimate entity or user"
        }
        StrideCategory::Tampering => {
            "Attacker can modify data, code, or system configuration"
        }
        StrideCategory::Repudiation => {
            "Attacker can deny performing an action without proof"
        }
        StrideCategory::InformationDisclosure => {
            "Attacker can access sensitive or confidential information"
        }
        StrideCategory::DenialOfService => {
            "Attacker can disrupt service availability"
        }
        StrideCategory::ElevationOfPrivilege => {
            "Attacker can gain unauthorized access or privileges"
        }
    }
}

/// Get mitigation strategies for STRIDE category
pub fn stride_mitigations(category: StrideCategory) -> Vec<&'static str> {
    match category {
        StrideCategory::Spoofing => vec![
            "Implement strong authentication mechanisms",
            "Use multi-factor authentication",
            "Validate identity before granting access",
            "Rotate credentials regularly",
        ],
        StrideCategory::Tampering => vec![
            "Implement integrity checks (hashes, signatures)",
            "Use version control and audit logs",
            "Implement access controls",
            "Validate all inputs",
        ],
        StrideCategory::Repudiation => vec![
            "Implement comprehensive logging",
            "Use digital signatures",
            "Maintain audit trails",
            "Implement non-repudiation mechanisms",
        ],
        StrideCategory::InformationDisclosure => vec![
            "Encrypt sensitive data at rest and in transit",
            "Implement access controls",
            "Remove hardcoded secrets",
            "Use secret management solutions",
        ],
        StrideCategory::DenialOfService => vec![
            "Implement rate limiting",
            "Use load balancing",
            "Implement resource quotas",
            "Deploy DDoS protection",
        ],
        StrideCategory::ElevationOfPrivilege => vec![
            "Implement principle of least privilege",
            "Regular access reviews",
            "Separate duties",
            "Validate authorization on every request",
        ],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aws_key_stride() {
        let categories = categorize_finding_stride("AWS_ACCESS_KEY");
        assert!(categories.contains(&StrideCategory::Spoofing));
        assert!(categories.contains(&StrideCategory::InformationDisclosure));
        assert!(categories.contains(&StrideCategory::ElevationOfPrivilege));
    }

    #[test]
    fn test_github_token_stride() {
        let categories = categorize_finding_stride("GITHUB_PAT");
        assert!(categories.contains(&StrideCategory::Spoofing));
        assert!(categories.contains(&StrideCategory::InformationDisclosure));
        assert!(categories.contains(&StrideCategory::Tampering));
    }

    #[test]
    fn test_database_url_stride() {
        let categories = categorize_finding_stride("DATABASE_URL");
        assert!(categories.contains(&StrideCategory::Spoofing));
        assert!(categories.contains(&StrideCategory::Tampering));
        assert!(categories.contains(&StrideCategory::InformationDisclosure));
    }

    #[test]
    fn test_private_key_stride() {
        let categories = categorize_finding_stride("PRIVATE_KEY");
        assert!(categories.contains(&StrideCategory::Spoofing));
        assert!(categories.contains(&StrideCategory::InformationDisclosure));
        assert!(categories.contains(&StrideCategory::ElevationOfPrivilege));
    }

    #[test]
    fn test_risk_score_calculation() {
        let score = calculate_risk_score(Likelihood::High, Impact::Major);
        assert_eq!(score, 16); // 4 * 4

        let score = calculate_risk_score(Likelihood::VeryHigh, Impact::Catastrophic);
        assert_eq!(score, 25); // 5 * 5
    }

    #[test]
    fn test_impact_determination() {
        assert_eq!(determine_impact("AWS_ACCESS_KEY"), Impact::Catastrophic);
        assert_eq!(determine_impact("DATABASE_URL"), Impact::Major);
        assert_eq!(determine_impact("GITHUB_PAT"), Impact::Moderate);
        assert_eq!(determine_impact("API_KEY"), Impact::Minor);
    }

    #[test]
    fn test_stride_descriptions() {
        assert!(stride_description(StrideCategory::Spoofing).contains("impersonate"));
        assert!(stride_description(StrideCategory::Tampering).contains("modify"));
        assert!(stride_description(StrideCategory::InformationDisclosure).contains("access"));
    }

    #[test]
    fn test_stride_mitigations() {
        let mitigations = stride_mitigations(StrideCategory::InformationDisclosure);
        assert!(!mitigations.is_empty());
        assert!(mitigations.iter().any(|m| m.contains("Encrypt")));
        assert!(mitigations.iter().any(|m| m.contains("secret")));
    }
}
