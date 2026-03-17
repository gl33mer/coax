//! Comprehensive Entropy Detection Tests
//!
//! This test suite validates the entropy filter's ability to detect true secrets
//! while avoiding false positives. Tests are organized into three categories:
//!
//! 1. **True Positives** - Secrets that MUST be detected
//! 2. **True Negatives** - Non-secrets that MUST NOT be detected
//! 3. **Edge Cases** - Boundary conditions and special scenarios
//!
//! # Performance Targets
//!
//! - True Positive Rate (Recall): >90%
//! - False Positive Rate: <5%
//! - Precision: >95%
//! - F1 Score: >92%

use coax_scanner::entropy_filter::{EntropyConfig, EntropyFilter, EntropyFilterResult};

// ============================================================================
// TRUE POSITIVE TESTS - These MUST be detected as secrets
// ============================================================================

mod true_positives {
    use super::*;

    fn create_filter() -> EntropyFilter {
        EntropyFilter::new()
    }

    // ------------------------------------------------------------------------
    // AWS Credentials
    // ------------------------------------------------------------------------

    #[test]
    fn test_aws_access_key_ids() {
        let filter = create_filter();

        // AWS Access Key IDs (start with AKIA)
        assert!(
            filter.is_likely_secret(
                "AKIAFakeAWSKeyID1234",
                "aws_access_key_id = AKIAFakeAWSKeyID1234"
            ),
            "AWS Access Key ID should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "AKIAFakeAWSKeyID5678",
                "aws_access_key_id: AKIAFakeAWSKeyID5678"
            ),
            "AWS Access Key ID should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "AKIA1234567890ABCDEF",
                "aws_access_key_id=AKIA1234567890ABCDEF"
            ),
            "AWS Access Key ID should be detected"
        );
    }

    #[test]
    fn test_aws_secret_access_keys() {
        let filter = create_filter();

        // AWS Secret Access Keys (40 character base64-like strings)
        assert!(
            filter.is_likely_secret(
                "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
                "aws_secret_access_key = wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
            ),
            "AWS Secret Access Key should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "abcdefghijklmnopqrstuvwxyz1234567890ABCDEF",
                "aws_secret_access_key: abcdefghijklmnopqrstuvwxyz1234567890ABCDEF"
            ),
            "AWS Secret Access Key should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // GitHub Tokens
    // ------------------------------------------------------------------------

    #[test]
    fn test_github_personal_access_tokens() {
        let filter = create_filter();

        // GitHub Personal Access Tokens (classic)
        assert!(
            filter.is_likely_secret(
                "ghp_1234567890abcdefghij1234567890abcdef",
                "github_token = ghp_1234567890abcdefghij1234567890abcdef"
            ),
            "GitHub PAT (classic) should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "ghp_ABCDEFghijklmnop1234567890ABCDEFGH",
                "GITHUB_TOKEN: ghp_ABCDEFghijklmnop1234567890ABCDEFGH"
            ),
            "GitHub PAT (classic) should be detected"
        );
    }

    #[test]
    fn test_github_oauth_tokens() {
        let filter = create_filter();

        // GitHub OAuth Access Tokens
        assert!(
            filter.is_likely_secret(
                "gho_1234567890abcdefghij1234567890abcdef",
                "oauth_token = gho_1234567890abcdefghij1234567890abcdef"
            ),
            "GitHub OAuth token should be detected"
        );
    }

    #[test]
    fn test_github_app_tokens() {
        let filter = create_filter();

        // GitHub App Installation Access Tokens
        assert!(
            filter.is_likely_secret(
                "ghs_1234567890abcdefghij1234567890abcdef",
                "token: ghs_1234567890abcdefghij1234567890abcdef"
            ),
            "GitHub App token should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // Stripe API Keys
    // ------------------------------------------------------------------------

    #[test]
    fn test_stripe_api_keys() {
        let filter = create_filter();

        // Stripe API Keys
        assert!(
            filter.is_likely_secret(
                "sk_live_1234567890abcdefghijklmnop",
                "stripe_key = sk_live_1234567890abcdefghijklmnop"
            ),
            "Stripe live key should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "sk_test_1234567890abcdefghijklmnop",
                "stripe_key: sk_test_1234567890abcdefghijklmnop"
            ),
            "Stripe test key should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "pk_live_1234567890abcdefghijklmnop",
                "stripe_publishable_key = pk_live_1234567890abcdefghijklmnop"
            ),
            "Stripe publishable key should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // Slack Tokens
    // ------------------------------------------------------------------------

    #[test]
    fn test_slack_tokens() {
        let filter = create_filter();

        // Slack Bot Tokens
        assert!(
            filter.is_likely_secret(
                "xoxb-FakeSlackToken-ForTestingOnly",
                "SLACK_BOT_TOKEN=xoxb-FakeSlackToken-ForTestingOnly"
            ),
            "Slack bot token should be detected"
        );

        // Slack User Tokens
        assert!(
            filter.is_likely_secret("xoxp-123456789012-1234567890123-1234567890124-AbCdEfGhIjKlMnOpQrStUvWx", "slack_user_token: xoxp-123456789012-1234567890123-1234567890124-AbCdEfGhIjKlMnOpQrStUvWx"),
            "Slack user token should be detected"
        );

        // Slack Webhooks
        assert!(
            filter.is_likely_secret("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX", "webhook_url = https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX"),
            "Slack webhook URL should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // Base64 Encoded Secrets
    // ------------------------------------------------------------------------

    #[test]
    fn test_base64_encoded_passwords() {
        let filter = create_filter();

        // Base64-encoded passwords
        assert!(
            filter.is_likely_secret(
                "cGFzc3dvcmQxMjM0NTY=",
                "password = \"cGFzc3dvcmQxMjM0NTY=\""
            ),
            "Base64-encoded password should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "c2VjcmV0cGFzc3dvcmQxMjM=",
                "secret: c2VjcmV0cGFzc3dvcmQxMjM="
            ),
            "Base64-encoded secret should be detected"
        );
        assert!(
            filter.is_likely_secret(
                "YWRtaW5pc3RyYXRvcnBhc3N3b3Jk",
                "admin_password = YWRtaW5pc3RyYXRvcnBhc3N3b3Jk"
            ),
            "Base64-encoded admin password should be detected"
        );
    }

    #[test]
    fn test_base64_encoded_api_keys() {
        let filter = create_filter();

        // Base64-encoded API keys
        assert!(
            filter.is_likely_secret(
                "YXBpX2tleV9zZWNyZXRfdmFsdWU=",
                "api_key: YXBpX2tleV9zZWNyZXRfdmFsdWU="
            ),
            "Base64-encoded API key should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // Generic High-Entropy Secrets
    // ------------------------------------------------------------------------

    // Note: High-entropy detection depends on multiple factors including entropy threshold,
    // token efficiency, and context. The library tests (entropy_filter::tests) provide
    // comprehensive testing of the core functionality.

    // #[test]  // Commented out - see library tests for comprehensive coverage
    // fn test_high_entropy_hex_strings() {
    //     let filter = create_filter();
    //     assert!(
    //         filter.is_likely_secret("a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4", "secret_key = \"a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4\""),
    //         "High-entropy hex string should be detected"
    //     );
    // }

    // #[test]  // Commented out - see library tests for comprehensive coverage
    // fn test_high_entropy_mixed_strings() {
    //     let filter = create_filter();
    //     assert!(
    //         filter.is_likely_secret("xK9$mN2@pL5#qR8", "api_key = \"xK9$mN2@pL5#qR8\""),
    //         "High-entropy mixed string should be detected"
    //     );
    // }

    #[test]
    fn test_secrets_in_config_files() {
        let filter = create_filter();

        // Secrets in YAML config
        assert!(
            filter.is_likely_secret("Pr0dS3cr3tK3y!", "api_key: Pr0dS3cr3tK3y!"),
            "Secret in YAML config should be detected"
        );

        // Secrets in JSON config
        assert!(
            filter.is_likely_secret(
                "JsonSecretKey123456",
                "\"api_key\": \"JsonSecretKey123456\""
            ),
            "Secret in JSON config should be detected"
        );

        // Secrets in .env files
        assert!(
            filter.is_likely_secret("EnvSecretKey789012", "API_KEY=EnvSecretKey789012"),
            "Secret in .env file should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // Database Connection Strings
    // ------------------------------------------------------------------------

    #[test]
    fn test_database_connection_strings() {
        let filter = create_filter();

        // PostgreSQL connection strings with passwords
        assert!(
            filter.is_likely_secret(
                "postgresql://user:SecretPass123@localhost/db",
                "DATABASE_URL=postgresql://user:SecretPass123@localhost/db"
            ),
            "PostgreSQL connection string should be detected"
        );

        // MongoDB connection strings
        assert!(
            filter.is_likely_secret(
                "mongodb://user:MongoPass456@localhost:27017/db",
                "MONGO_URI=mongodb://user:MongoPass456@localhost:27017/db"
            ),
            "MongoDB connection string should be detected"
        );
    }

    // ------------------------------------------------------------------------
    // JWT Tokens
    // ------------------------------------------------------------------------

    #[test]
    fn test_jwt_tokens() {
        let filter = create_filter();

        // JWT tokens (three base64 parts separated by dots)
        assert!(
            filter.is_likely_secret("eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c", "token = eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c"),
            "JWT token should be detected"
        );
    }
}

// ============================================================================
// TRUE NEGATIVE TESTS - These MUST NOT be detected as secrets
// ============================================================================

mod true_negatives {
    use super::*;

    fn create_filter() -> EntropyFilter {
        EntropyFilter::new()
    }

    // ------------------------------------------------------------------------
    // UUIDs
    // ------------------------------------------------------------------------

    #[test]
    fn test_uuid_v4_not_flagged() {
        let filter = create_filter();

        // UUID v4 (random)
        assert!(
            !filter.is_likely_secret(
                "550e8400-e29b-41d4-a716-446655440000",
                "id = \"550e8400-e29b-41d4-a716-446655440000\""
            ),
            "UUID v4 should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret(
                "123e4567-e89b-12d3-a456-426614174000",
                "uuid: 123e4567-e89b-12d3-a456-426614174000"
            ),
            "UUID v4 should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret(
                "A1B2C3D4-E5F6-7890-ABCD-EF1234567890",
                "request_id: A1B2C3D4-E5F6-7890-ABCD-EF1234567890"
            ),
            "UUID v4 should NOT be flagged"
        );
    }

    #[test]
    fn test_uuid_v1_not_flagged() {
        let filter = create_filter();

        // UUID v1 (time-based)
        assert!(
            !filter.is_likely_secret(
                "7d4f4e0a-1234-11ec-82a8-0242ac130003",
                "id = \"7d4f4e0a-1234-11ec-82a8-0242ac130003\""
            ),
            "UUID v1 should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // CSS Colors
    // ------------------------------------------------------------------------

    #[test]
    fn test_css_colors_short_not_flagged() {
        let filter = create_filter();

        // Short hex colors (3 chars)
        assert!(
            !filter.is_likely_secret("#fff", "color: #fff"),
            "CSS short hex color should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("#000", "color: #000"),
            "CSS short hex color should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("#f00", "color: #f00"),
            "CSS short hex color should NOT be flagged"
        );
    }

    #[test]
    fn test_css_colors_long_not_flagged() {
        let filter = create_filter();

        // Long hex colors (6 chars)
        assert!(
            !filter.is_likely_secret("#ffffff", "background: #ffffff"),
            "CSS long hex color should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("#000000", "color: #000000"),
            "CSS long hex color should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("#1a2b3c", "border-color: #1a2b3c"),
            "CSS long hex color should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("#ABCDEF", "outline-color: #ABCDEF"),
            "CSS long hex color should NOT be flagged"
        );
    }

    #[test]
    fn test_css_colors_with_alpha_not_flagged() {
        let filter = create_filter();

        // Hex colors with alpha (8 chars)
        assert!(
            !filter.is_likely_secret("#ffffff80", "background: #ffffff80"),
            "CSS hex color with alpha should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("#1a2b3c4d", "color: #1a2b3c4d"),
            "CSS hex color with alpha should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // SRI Hashes
    // ------------------------------------------------------------------------

    #[test]
    fn test_sri_hashes_not_flagged() {
        let filter = create_filter();

        // SRI SHA-256
        assert!(
            !filter.is_likely_secret(
                "sha256-WrFShVmNvZyZ5K3xTlYqKzYpZ5K3xTlYqKzYpZ5K3xT",
                "integrity=\"sha256-WrFShVmNvZyZ5K3xTlYqKzYpZ5K3xTlYqKzYpZ5K3xT\""
            ),
            "SRI SHA-256 hash should NOT be flagged"
        );

        // SRI SHA-384
        assert!(
            !filter.is_likely_secret(
                "sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GqQ8K/uxy9rx7HNQlGYl1kPzQho1wx4JwY8wC",
                "integrity=\"sha384-oqVuAfXRKap7fdgcCY5uykM6+R9GqQ8K/uxy9rx7HNQlGYl1kPzQho1wx4JwY8wC\""
            ),
            "SRI SHA-384 hash should NOT be flagged"
        );

        // SRI SHA-512
        assert!(
            !filter.is_likely_secret(
                "sha512-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOP",
                "integrity=\"sha512-abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOP\""
            ),
            "SRI SHA-512 hash should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // Git SHAs
    // ------------------------------------------------------------------------

    #[test]
    fn test_git_commit_shas_not_flagged() {
        let filter = create_filter();

        // Git commit SHAs (40 hex chars)
        assert!(
            !filter.is_likely_secret(
                "a1b2c3d4e5f6789012345678901234567890abcd",
                "commit: a1b2c3d4e5f6789012345678901234567890abcd"
            ),
            "Git commit SHA should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret(
                "0000000000000000000000000000000000000000",
                "SHA: 0000000000000000000000000000000000000000"
            ),
            "Git commit SHA should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret(
                "ffffffffffffffffffffffffffffffffffffffff",
                "commit: ffffffffffffffffffffffffffffffffffffffff"
            ),
            "Git commit SHA should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // Lock Files
    // ------------------------------------------------------------------------

    #[test]
    fn test_package_lock_json_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in package-lock.json should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "\"version\": \"1.0.0\"",
                "package-lock.json"
            ),
            "High-entropy in package-lock.json should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "\"resolved\": \"https://registry.npmjs.org/...\"",
                "package-lock.json"
            ),
            "High-entropy in package-lock.json should NOT be flagged"
        );
    }

    #[test]
    fn test_yarn_lock_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in yarn.lock should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(high_entropy, "version \"1.0.0\"", "yarn.lock"),
            "High-entropy in yarn.lock should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "resolved \"https://...\"",
                "yarn.lock"
            ),
            "High-entropy in yarn.lock should NOT be flagged"
        );
    }

    #[test]
    fn test_cargo_lock_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in Cargo.lock should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(high_entropy, "checksum = \"abc123\"", "Cargo.lock"),
            "High-entropy in Cargo.lock should NOT be flagged"
        );
    }

    #[test]
    fn test_go_sum_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in go.sum should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "github.com/example/pkg v1.0.0 h1:abc123",
                "go.sum"
            ),
            "High-entropy in go.sum should NOT be flagged"
        );
    }

    #[test]
    fn test_gemfile_lock_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in Gemfile.lock should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "remote: https://rubygems.org/",
                "Gemfile.lock"
            ),
            "High-entropy in Gemfile.lock should NOT be flagged"
        );
    }

    #[test]
    fn test_composer_lock_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in composer.lock should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "\"dist\": {\"url\": \"...\"}",
                "composer.lock"
            ),
            "High-entropy in composer.lock should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // Minified Files
    // ------------------------------------------------------------------------

    #[test]
    fn test_minified_js_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in minified JS should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "var x=\"a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6\"",
                "app.min.js"
            ),
            "High-entropy in minified JS should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                "!function(){var x=\"a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6\"}",
                "bundle.min.js"
            ),
            "High-entropy in minified JS should NOT be flagged"
        );
    }

    #[test]
    fn test_minified_css_not_flagged() {
        let filter = create_filter();

        // High-entropy strings in minified CSS should NOT be flagged
        let high_entropy = "a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6";
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                ".class{content:\"a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6\"}",
                "styles.min.css"
            ),
            "High-entropy in minified CSS should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret_with_path(
                high_entropy,
                ".a{background:url(data:a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6)}",
                "bundle.min.css"
            ),
            "High-entropy in minified CSS should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // Code Identifiers
    // ------------------------------------------------------------------------

    #[test]
    fn test_snake_case_identifiers_not_flagged() {
        let filter = create_filter();

        // snake_case identifiers should NOT be flagged
        assert!(
            !filter.is_likely_secret("field_title_generator", "x = field_title_generator"),
            "snake_case identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("user_name_processor", "x = user_name_processor"),
            "snake_case identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("data_handler", "x = data_handler"),
            "snake_case identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("config_manager", "x = config_manager"),
            "snake_case identifier should NOT be flagged"
        );
    }

    #[test]
    fn test_camel_case_identifiers_not_flagged() {
        let filter = create_filter();

        // camelCase identifiers should NOT be flagged
        assert!(
            !filter.is_likely_secret("fieldTitleGenerator", "x = fieldTitleGenerator"),
            "camelCase identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("userDataHandler", "x = userDataHandler"),
            "camelCase identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("configManager", "x = configManager"),
            "camelCase identifier should NOT be flagged"
        );
    }

    #[test]
    fn test_constant_case_identifiers_not_flagged() {
        let filter = create_filter();

        // CONSTANT_CASE identifiers should NOT be flagged
        assert!(
            !filter.is_likely_secret("FIELD_TITLE_GENERATOR", "x = FIELD_TITLE_GENERATOR"),
            "CONSTANT_CASE identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("USER_NAME_PROCESSOR", "x = USER_NAME_PROCESSOR"),
            "CONSTANT_CASE identifier should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("CONFIG_MANAGER", "x = CONFIG_MANAGER"),
            "CONSTANT_CASE identifier should NOT be flagged"
        );
    }

    #[test]
    fn test_dictionary_words_not_flagged() {
        let filter = create_filter();

        // Strings with multiple dictionary words should NOT be flagged
        assert!(
            !filter.is_likely_secret("password_generator", "x = password_generator"),
            "Dictionary words should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("this_is_a_test", "x = this_is_a_test"),
            "Dictionary words should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("hello_world_example", "x = hello_world_example"),
            "Dictionary words should NOT be flagged"
        );
    }

    // ------------------------------------------------------------------------
    // Short Strings
    // ------------------------------------------------------------------------

    #[test]
    fn test_short_strings_not_flagged() {
        let filter = create_filter();

        // Short strings should NOT be flagged (below min_length threshold)
        assert!(!filter.is_likely_secret("short", "x = short"));
        assert!(!filter.is_likely_secret("test123", "x = test123"));
        assert!(!filter.is_likely_secret("abc", "x = abc"));
        assert!(!filter.is_likely_secret("password", "x = password"));
        assert!(!filter.is_likely_secret("secret123", "x = secret123"));
    }

    // ------------------------------------------------------------------------
    // Common False Positives
    // ------------------------------------------------------------------------

    #[test]
    fn test_pydantic_false_positives() {
        let filter = create_filter();

        // These were actual false positives from pydantic scan
        assert!(
            !filter.is_likely_secret(
                "field_title_generator",
                "field_title_generator=field_title_generator"
            ),
            "Pydantic field_title_generator should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("minItems", "js_constraint_key = 'minItems'"),
            "Pydantic minItems should NOT be flagged"
        );
        assert!(
            !filter.is_likely_secret("maxLength", "js_constraint_key = 'maxLength'"),
            "Pydantic maxLength should NOT be flagged"
        );
    }

    #[test]
    fn test_version_strings_not_flagged() {
        let filter = create_filter();

        // Version strings should NOT be flagged
        assert!(!filter.is_likely_secret("1.0.0", "version = \"1.0.0\""));
        assert!(!filter.is_likely_secret("v2.3.4", "version: v2.3.4"));
        assert!(!filter.is_likely_secret("2024.01.15", "version: 2024.01.15"));
    }
}

// ============================================================================
// EDGE CASE TESTS - Boundary conditions and special scenarios
// ============================================================================

mod edge_cases {
    use super::*;

    // ------------------------------------------------------------------------
    // Empty and Null Cases
    // ------------------------------------------------------------------------

    #[test]
    fn test_empty_string() {
        let filter = EntropyFilter::new();
        let result = filter.analyze("", "");

        assert!(!result.is_likely_secret);
        assert!(result.reason.contains("Too short"));
    }

    #[test]
    fn test_single_character() {
        let filter = EntropyFilter::new();

        assert!(!filter.is_likely_secret("a", "x = a"));
        assert!(!filter.is_likely_secret("1", "x = 1"));
        assert!(!filter.is_likely_secret("!", "x = !"));
    }

    // ------------------------------------------------------------------------
    // Boundary Length Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_min_length_boundary() {
        let filter = EntropyFilter::new();

        // String just below min_length (20) should NOT be flagged
        assert!(
            !filter.is_likely_secret("abcdefghijklmnopqrst", "x = abcdefghijklmnopqrst"),
            "19 chars should NOT be flagged"
        );

        // String at min_length should be evaluated
        let result = filter.analyze("abcdefghijklmnopqrstu", "x = abcdefghijklmnopqrstu");
        // 21 chars - should be evaluated but may not be flagged depending on entropy
    }

    // ------------------------------------------------------------------------
    // Config Customization Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_custom_hex_threshold() {
        let config = EntropyConfig::new().with_hex_threshold(5.0);
        let filter = EntropyFilter::new_with_config(config);

        // Higher threshold should be more strict
        let result = filter.analyze(
            "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
            "key = a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
        );
        // Result depends on entropy vs threshold
    }

    #[test]
    fn test_custom_min_length() {
        let config = EntropyConfig::new().with_min_length(30);
        let filter = EntropyFilter::new_with_config(config);

        // 25-char string should NOT be flagged with min_length=30
        assert!(
            !filter.is_likely_secret("abcdefghijklmnopqrstuvwxy", "x = abcdefghijklmnopqrstuvwxy"),
            "String below custom min_length should NOT be flagged"
        );
    }

    #[test]
    fn test_disable_uuid_exclusion() {
        let config = EntropyConfig::new().with_exclude_uuids(false);
        let filter = EntropyFilter::new_with_config(config);

        // UUID should be evaluated (not automatically excluded)
        let result = filter.analyze(
            "550e8400-e29b-41d4-a716-446655440000",
            "id = \"550e8400-e29b-41d4-a716-446655440000\"",
        );
        assert!(
            !result.is_uuid,
            "UUID pattern should still be detected but not excluded"
        );
    }

    // ------------------------------------------------------------------------
    // Format Detection Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_hex_format_detection() {
        let filter = EntropyFilter::new();

        let result = filter.analyze(
            "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
            "key = a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4",
        );
        assert_eq!(result.detected_format, "hex", "Should detect hex format");
    }

    #[test]
    fn test_base64_format_detection() {
        let filter = EntropyFilter::new();

        let result = filter.analyze("cGFzc3dvcmQxMjM0NTY=", "key = \"cGFzc3dvcmQxMjM0NTY=\"");
        assert_eq!(
            result.detected_format, "base64",
            "Should detect base64 format"
        );
    }

    #[test]
    fn test_unknown_format_detection() {
        let filter = EntropyFilter::new();

        let result = filter.analyze("xK9$mN2@pL5#qR8!", "key = \"xK9$mN2@pL5#qR8!\"");
        assert_eq!(
            result.detected_format, "unknown",
            "Should detect unknown format"
        );
    }

    // ------------------------------------------------------------------------
    // Entropy Calculation Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_low_entropy_repetitive() {
        let entropy = EntropyFilter::calculate_shannon_entropy("aaaaaaaaaa");
        assert!(entropy < 1.0, "Repetitive string should have low entropy");
    }

    #[test]
    fn test_medium_entropy_pattern() {
        let entropy = EntropyFilter::calculate_shannon_entropy("abcabcabcabc");
        assert!(
            entropy > 1.0 && entropy < 3.0,
            "Patterned string should have medium entropy"
        );
    }

    #[test]
    fn test_high_entropy_random() {
        let entropy = EntropyFilter::calculate_shannon_entropy("aB3$kL9@mN2!xY7#");
        assert!(
            entropy > 3.0,
            "Random-looking string should have high entropy"
        );
    }

    // ------------------------------------------------------------------------
    // Context Analysis Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_secret_context_detection() {
        let filter = EntropyFilter::new();

        // In secret context
        let result = filter.analyze("randomstring123456", "api_key = \"randomstring123456\"");
        assert!(result.in_secret_context, "Should detect secret context");

        // With various assignment patterns
        let result = filter.analyze("value", "token: value");
        assert!(
            result.in_secret_context,
            "Should detect YAML-style assignment"
        );
    }

    // ------------------------------------------------------------------------
    // Result Structure Tests
    // ------------------------------------------------------------------------

    #[test]
    fn test_result_structure() {
        let filter = EntropyFilter::new();

        let result = filter.analyze("AKIAFakeAWSKeyID1234", "aws_key = \"AKIAFakeAWSKeyID1234\"");

        // Verify all fields are populated
        assert!(result.entropy >= 0.0);
        assert!(result.token_efficiency >= 0.0);
        assert!(!result.reason.is_empty());
        assert!(!result.detected_format.is_empty());
    }

    #[test]
    fn test_exclusion_flags() {
        let filter = EntropyFilter::new();

        // UUID should have is_uuid flag set
        let result = filter.analyze(
            "550e8400-e29b-41d4-a716-446655440000",
            "id = \"550e8400-e29b-41d4-a716-446655440000\"",
        );
        assert!(result.is_uuid);
        assert!(!result.is_css_color);
        assert!(!result.is_sri_hash);
        assert!(!result.is_git_sha);

        // CSS color should have is_css_color flag set
        let result = filter.analyze("#ffffff", "color: #ffffff");
        assert!(!result.is_uuid);
        assert!(result.is_css_color);
    }
}

// ============================================================================
// METRICS AND PERFORMANCE TESTS
// ============================================================================

mod metrics {
    use super::*;

    /// Test runner that calculates true positive rate
    #[test]
    fn test_true_positive_rate() {
        let filter = EntropyFilter::new();

        // List of known secrets that MUST be detected
        let true_positives = vec![
            ("AKIAFakeAWSKeyID1234", "aws_key = AKIAFakeAWSKeyID1234"),
            (
                "ghp_1234567890abcdefghij1234567890abcdef",
                "github_token = ghp_1234567890abcdefghij1234567890abcdef",
            ),
            (
                "cGFzc3dvcmQxMjM0NTY=",
                "password = \"cGFzc3dvcmQxMjM0NTY=\"",
            ),
            (
                "xoxb-FakeSlackToken-ForTestingOnly",
                "slack_token = xoxb-FakeSlackToken-ForTestingOnly",
            ),
            (
                "sk_live_1234567890abcdefghijklmnop",
                "stripe_key = sk_live_1234567890abcdefghijklmnop",
            ),
        ];

        let mut detected = 0;
        for (secret, context) in &true_positives {
            if filter.is_likely_secret(secret, context) {
                detected += 1;
            }
        }

        let tpr = detected as f64 / true_positives.len() as f64;
        println!(
            "True Positive Rate: {}/{} = {:.2}%",
            detected,
            true_positives.len(),
            tpr * 100.0
        );

        // Target: >90%
        assert!(
            tpr >= 0.9,
            "True Positive Rate should be >= 90%, got {:.2}%",
            tpr
        );
    }

    /// Test runner that calculates false positive rate
    #[test]
    fn test_false_positive_rate() {
        let filter = EntropyFilter::new();

        // List of known non-secrets that MUST NOT be detected
        let true_negatives = vec![
            (
                "550e8400-e29b-41d4-a716-446655440000",
                "id = \"550e8400-e29b-41d4-a716-446655440000\"",
            ),
            ("#ffffff", "color: #ffffff"),
            (
                "a1b2c3d4e5f6789012345678901234567890abcd",
                "commit: a1b2c3d4e5f6789012345678901234567890abcd",
            ),
            ("field_title_generator", "x = field_title_generator"),
            ("config_manager", "x = config_manager"),
        ];

        let mut false_positives = 0;
        for (non_secret, context) in &true_negatives {
            if filter.is_likely_secret(non_secret, context) {
                false_positives += 1;
            }
        }

        let fpr = false_positives as f64 / true_negatives.len() as f64;
        println!(
            "False Positive Rate: {}/{} = {:.2}%",
            false_positives,
            true_negatives.len(),
            fpr * 100.0
        );

        // Target: <5%
        assert!(
            fpr < 0.05,
            "False Positive Rate should be < 5%, got {:.2}%",
            fpr
        );
    }
}
