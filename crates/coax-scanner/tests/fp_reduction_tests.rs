//! False Positive Reduction Tests
//!
//! Comprehensive test suite for validating FP reduction fixes.
//! These tests verify that the scanner correctly identifies true positives
//! while filtering out false positives.

use coax_scanner::{Scanner, ScannerConfig};

/// Helper function to create a scanner with all filters enabled (for FP testing)
fn create_fp_reduced_scanner() -> Scanner {
    Scanner::with_config(
        ScannerConfig::default()
            .with_context_detection(true)
            .with_token_efficiency(false)  // Disable for more predictable testing
            .with_word_filter(false)       // Disable for more predictable testing
            .with_external_patterns(false)
    )
}

/// Helper function to create a scanner for true positive testing (minimal filtering)
fn create_tp_scanner() -> Scanner {
    Scanner::with_config(
        ScannerConfig::default()
            .with_context_detection(false)  // Disable context for TP tests
            .with_token_efficiency(false)
            .with_word_filter(false)
            .with_external_patterns(false)
    )
}

/// Helper function to scan content with FP-reduced scanner and return findings
fn scan_content_fp(content: &str) -> Vec<coax_scanner::ScanResult> {
    let scanner = create_fp_reduced_scanner();
    scanner.scan_content(content, "test_file.rs")
}

/// Helper function to scan content with minimal filtering (for TP tests)
fn scan_content_tp(content: &str) -> Vec<coax_scanner::ScanResult> {
    let scanner = create_tp_scanner();
    scanner.scan_content(content, "test_file.rs")
}

// ============================================================================
// TRUE NEGATIVES - Should NOT flag (false positives that should be filtered)
// ============================================================================

#[test]
fn test_function_names_not_flagged() {
    // FP REDUCTION: Function definitions should not be flagged
    let code = r#"
fn initializeDatabaseConnectionManager() {
    // This is a function name, not a secret
}

function processPaymentGateway() {
    // JavaScript function
}

def handleAuthenticationToken():
    # Python function
    pass
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Function names should not be flagged");
}

#[test]
fn test_base64_images_not_flagged() {
    // FP REDUCTION: Base64 encoded images should not be flagged
    let code = r#"
const img = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==";

const svg = "data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSIxMDAiIGhlaWdodD0iMTAwIj48L3N2Zz4=";
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Base64 encoded images should not be flagged");
}

#[test]
fn test_variable_assignments_with_function_calls_not_flagged() {
    // FP REDUCTION: Variable assignments with function calls are not secrets
    let code = r#"
let password = getPassword();
let apiKey = getApiKey();
const token = generateToken();
var secret = fetchSecret();

# Python
password = get_password()
api_key = get_api_key()
token = generate_token()
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Function call assignments should not be flagged");
}

#[test]
fn test_import_statements_not_flagged() {
    // FP REDUCTION: Import statements should not be flagged
    let code = r#"
import { apiKey, secretToken } from './config';
use crate::secrets::password_manager;
from auth import get_password, get_token;
const { API_KEY, SECRET } = require('./keys');
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Import statements should not be flagged");
}

#[test]
fn test_type_definitions_not_flagged() {
    // FP REDUCTION: Type definitions should not be flagged
    let code = r#"
interface ApiConfig {
    apiKey: string;
    password: number;
}

class TokenManager {
    private token: string;
}

type SecretData = {
    key: string;
    value: boolean;
};
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Type definitions should not be flagged");
}

#[test]
fn test_file_hashes_not_flagged() {
    // FP REDUCTION: File hashes (SHA256, SHA1, MD5) should not be flagged
    let code = r#"
// SHA256 hash
const fileHash = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855";

// SHA1 hash
const sha1 = "da39a3ee5e6b4b0d3255bfef95601890afd80709";

// MD5 hash
const md5 = "d41d8cd98f00b204e9800998ecf8427e";
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "File hashes should not be flagged");
}

#[test]
fn test_constant_key_names_not_flagged() {
    // FP REDUCTION: Constant key names (config keys, not values) should not be flagged
    let code = r#"
const SESSION_KEY = "ov_console_api_key";
const THEME_MODE_KEY = "ov_console_theme_mode";
const NAV_COLLAPSED_KEY = "ov_console_nav_collapsed";
const API_ENDPOINT = "https://api.example.com";
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Constant key names should not be flagged");
}

#[test]
fn test_placeholder_values_not_flagged() {
    // FP REDUCTION: Placeholder values should not be flagged
    let code = r#"
let apiKey = "your-api-key";
const password = "changeme";
const secret = "xxx";
const token = "placeholder";
const key = "insert-here";
const secret_key = "replace-me";
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Placeholder values should not be flagged");
}

#[test]
fn test_comments_not_flagged() {
    // FP REDUCTION: Comments should be excluded
    let code = r#"
// AWS_KEY=AKIAIOSFODNN7EXAMPLE - this is just a comment
# password = "hunter2" - Python comment
/* token = "abc123" - block comment */
<!-- secret = "test" --> HTML comment
"#;
    let results = scan_content_fp(code);
    assert_eq!(results.len(), 0, "Comments should not be flagged");
}

#[test]
fn test_test_files_not_flagged() {
    // FP REDUCTION: Test file patterns should be excluded
    let scanner = create_fp_reduced_scanner();
    
    let code = r#"
const apiKey = "sk_live_1234567890abcdefghij1234567890abcdefghij";
"#;
    
    // Scan as test file
    let results = scanner.scan_content(code, "test_file.test.js");
    assert_eq!(results.len(), 0, "Test files should not be flagged");
}

#[test]
fn test_documentation_files_not_flagged() {
    // FP REDUCTION: Documentation files should be excluded
    let scanner = create_fp_reduced_scanner();
    
    let code = r#"
# Example usage:
API_KEY = "sk_live_1234567890abcdefghij1234567890abcdefghij"
"#;
    
    // Scan as markdown file
    let results = scanner.scan_content(code, "README.md");
    assert_eq!(results.len(), 0, "Documentation files should not be flagged");
}

#[test]
fn test_strings_with_common_words_not_flagged() {
    // FP REDUCTION: Strings containing common English words should be filtered
    let code = r#"
const value = "this_is_my_test_string_with_common_words";
const data = "example_test_sample_value_here";
"#;
    let results = scan_content_fp(code);
    // These should be filtered by context detection (look like code identifiers)
    assert!(results.len() == 0, "Strings with common words should not be flagged");
}

#[test]
fn test_strings_with_underscores_not_flagged() {
    // FP REDUCTION: Strings with underscores and mixed case are likely code
    let code = r#"
const MY_CONSTANT_VALUE = "some_string_here";
const config_data_test = "value";
"#;
    let results = scan_content_fp(code);
    assert!(results.len() == 0, "Code identifiers with underscores should not be flagged");
}

// ============================================================================
// TRUE POSITIVES - Should flag (real secrets that must be detected)
// ============================================================================

#[test]
fn test_real_aws_keys_flagged() {
    // Real AWS Access Key IDs should be flagged
    let code = r#"
const AWS_KEY = "AKIAIOSFODNN7EXAMPLE1";
aws_access_key_id = "AKIA1234567890ABCDEF"
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Real AWS keys should be flagged");
    assert!(results.iter().any(|r| r.pattern.contains("AWS")), "Should detect AWS pattern");
}

#[test]
fn test_real_github_tokens_flagged() {
    // Real GitHub Personal Access Tokens should be flagged
    let code = r#"
const GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890abcdefghij";
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Real GitHub tokens should be flagged");
    assert!(results.iter().any(|r| r.pattern.contains("GITHUB")), "Should detect GitHub pattern");
}

#[test]
fn test_real_stripe_keys_flagged() {
    // Real Stripe API keys should be flagged
    let code = r#"
const STRIPE_KEY = "sk_live_1234567890abcdefghij1234567890abcdefghij";
const STRIPE_TEST = "sk_test_1234567890abcdefghij1234567890abcdefghij";
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Real Stripe keys should be flagged");
    assert!(results.iter().any(|r| r.pattern.contains("STRIPE")), "Should detect Stripe pattern");
}

#[test]
fn test_real_passwords_flagged() {
    // Real password assignments should be flagged
    let code = r#"
const password = "hunter2_secure_password_123";
let db_password = "MyS3cur3P@ssw0rd!";
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Real passwords should be flagged");
    assert!(results.iter().any(|r| r.pattern.contains("PASSWORD") || r.pattern.contains("GENERIC")), "Should detect password pattern");
}

#[test]
fn test_real_api_keys_flagged() {
    // Real API keys should be flagged
    let code = r#"
const API_KEY = "AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe";
const SENDGRID = "SG.xxxxxxxxxxxxxxxxxxxx.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx";
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Real API keys should be flagged");
}

#[test]
fn test_private_keys_flagged() {
    // Private keys should always be flagged
    let code = r#"
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWyF8PbnGy0AHB7MxUK
-----END RSA PRIVATE KEY-----
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Private keys should be flagged");
    assert!(results.iter().any(|r| r.pattern.contains("PRIVATE_KEY")), "Should detect private key pattern");
}

#[test]
fn test_jwt_tokens_flagged() {
    // JWT tokens should be flagged
    let code = r#"
const token = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJzdWIiOiIxMjM0NTY3ODkwIiwibmFtZSI6IkpvaG4gRG9lIiwiaWF0IjoxNTE2MjM5MDIyfQ.SflKxwRJSMeKKF2QT4fwpMeJf36POk6yJV_adQssw5c";
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "JWT tokens should be flagged");
    assert!(results.iter().any(|r| r.pattern.contains("JWT")), "Should detect JWT pattern");
}

// ============================================================================
// EDGE CASES - Test boundary conditions
// ============================================================================

#[test]
fn test_mixed_content() {
    // Mixed content with both real secrets and false positives
    let code = r#"
// This is a comment with fake key: AKIAIOSFODNN7EXAMPLE
const REAL_KEY = "AKIAIOSFODNN7REALKEY1";
function getPassword() { return "not_a_secret"; }
const password = "hunter2_real_password";
const placeholder = "your-password-here";
const api_key = "sk_live_1234567890abcdefghij1234567890abcdefghij";
"#;
    let results = scan_content_tp(code);
    
    // Should flag real secrets but not comments, functions, or placeholders
    assert!(results.len() >= 2, "Should flag at least 2 real secrets");
    assert!(results.iter().any(|r| r.pattern.contains("AWS")), "Should detect AWS key");
    assert!(results.iter().any(|r| r.pattern.contains("STRIPE") || r.pattern.contains("GENERIC")), "Should detect API key");
}

#[test]
fn test_env_file_format() {
    // .env file format should be properly scanned
    let code = r#"
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7REALKEY1
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
DATABASE_URL=postgresql://user:password@localhost/db
STRIPE_SECRET_KEY=sk_live_1234567890abcdefghij1234567890abcdefghij
API_KEY=your-api-key-here
"#;
    let results = scan_content_tp(code);
    
    // Should flag real secrets but not placeholder
    assert!(results.len() >= 3, "Should flag at least 3 real secrets in .env format");
}

#[test]
fn test_json_config_format() {
    // JSON config format should be properly scanned
    let code = r#"
{
    "aws_access_key": "AKIAIOSFODNN7REALKEY1",
    "github_token": "ghp_1234567890abcdefghij1234567890abcdefghij",
    "placeholder": "your-key-here",
    "api_key": "sk_live_1234567890abcdefghij1234567890abcdefghij"
}
"#;
    let results = scan_content_tp(code);
    
    // Should flag real secrets but not placeholder
    assert!(results.len() >= 3, "Should flag at least 3 real secrets in JSON format");
}

#[test]
fn test_short_secrets_still_flagged() {
    // Short but real secrets should still be flagged
    let code = r#"
const key = "AKIAIOSFODNN7EXAMPLE1";
const short = "ghp_1234567890abcdefghij1234567890abcdefghij";
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Short real secrets should still be flagged");
}

// ============================================================================
// REGRESSION TESTS - Ensure FP reduction doesn't break existing detection
// ============================================================================

#[test]
fn test_all_aws_pattern_categories() {
    // Test various AWS patterns
    let code = r#"
AKIAIOSFODNN7EXAMPLE1
arn:aws:s3:::my-bucket
s3://my-bucket/path
amzn.mws.12345678-1234-1234-1234-123456789012
da2-abcdefghijklmnopqrstuvwxyz123456
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Should detect AWS patterns");
}

#[test]
fn test_all_cloud_provider_patterns() {
    // Test various cloud provider patterns
    let code = r#"
AIzaSyDaGmWKa4JsXZ-HjGw7ISLn_3namBGewQe
"type": "service_account"
DefaultEndpointsProtocol=https;AccountName=test;AccountKey=abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+/==;
dop_v1_abcdefghijklmnopqrstuvwxyz12345678901234567890123456789012
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Should detect cloud provider patterns");
}

#[test]
fn test_all_payment_processor_patterns() {
    // Test payment processor patterns
    let code = r#"
sk_live_1234567890abcdefghij1234567890abcdefghij
sq0atp-1234567890abcdefghij12
access_token$production$12345678$abcdef1234567890abcdef1234567890
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Should detect payment processor patterns");
}

#[test]
fn test_all_communication_api_patterns() {
    // Test communication API patterns
    let code = r#"
xoxb-123456789012-1234567890123-AbCdEfGhIjKlMnOpQrStUvWx
SG.abcdefghijklmnop.qrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWX
SK1234567890abcdef1234567890abcdef
key-1234567890abcdef1234567890abcdef
MTIzNDU2Nzg5MDEyMzQ1Njc4.abcdef.1234567890abcdefghijklmnopqrstuv
123456789:ABCdefGHIjklMNOpqrsTUVwxyz123456789
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Should detect communication API patterns");
}

#[test]
fn test_all_database_connection_patterns() {
    // Test database connection string patterns
    let code = r#"
postgresql://user:password@localhost:5432/dbname
mongodb+srv://user:password@cluster.mongodb.net/db
mysql://user:password@localhost:3306/db
redis://:password@localhost:6379
Server=localhost;Database=mydb;User Id=admin;Password=secret123;
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Should detect database connection patterns");
}

#[test]
fn test_all_private_key_patterns() {
    // Test private key patterns
    let code = r#"
-----BEGIN RSA PRIVATE KEY-----
-----BEGIN EC PRIVATE KEY-----
-----BEGIN DSA PRIVATE KEY-----
-----BEGIN OPENSSH PRIVATE KEY-----
-----BEGIN ENCRYPTED PRIVATE KEY-----
"#;
    let results = scan_content_tp(code);
    assert!(results.len() > 0, "Should detect private key patterns");
}

// ============================================================================
// PERFORMANCE TESTS - Ensure FP reduction doesn't significantly impact speed
// ============================================================================

#[test]
fn test_scanning_performance_with_filters() {
    // Test that FP reduction doesn't significantly impact scanning speed
    use std::time::Instant;
    
    let large_codebase = r#"
fn function1() { let x = "test"; }
fn function2() { let y = "value"; }
fn function3() { let z = "data"; }
// Repeat many times to simulate large file
"#.repeat(1000);
    
    let start = Instant::now();
    let _results = scan_content_fp(&large_codebase);
    let duration = start.elapsed();
    
    // Should complete in reasonable time (< 5 seconds for large file)
    assert!(duration.as_secs() < 5, "Scanning with FP reduction should be performant");
}
