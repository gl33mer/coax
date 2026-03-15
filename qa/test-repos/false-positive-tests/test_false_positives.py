"""
Test file with various false positive scenarios.
This file should NOT trigger any detections (or very few).
"""

# =============================================================================
# FALSE POSITIVES - These should NOT be flagged
# =============================================================================

# 1. Placeholder values (should be excluded)
ACCESS_KEY = "your-access-key"
SECRET_KEY = "your-secret-key"
API_KEY = "your-api-key"
PASSWORD = "your-password"
TOKEN = "xxx"
DB_PASSWORD = "CHANGEME"
PRIVATE_KEY = "replace-me"
AUTH_TOKEN = "insert-here"
EXAMPLE_KEY = "example-value"
SAMPLE_KEY = "sample123"
TEST_KEY = "test-key-value"
FAKE_KEY = "fake-key"
DUMMY_KEY = "dummy-key"
PLACEHOLDER = "placeholder"

# 2. AWS Example Keys (should be excluded)
AWS_ACCESS_KEY_ID = "AKIAIOSFODNN7EXAMPLE"
AWS_SECRET_ACCESS_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"

# 3. Constant Key Names (should be excluded - these are config keys, not secrets)
SESSION_KEY = "ov_console_api_key"
THEME_MODE_KEY = "ov_console_theme_mode"
NAV_COLLAPSED_KEY = "ov_console_nav_collapsed"
CACHE_KEY = "my_app_cache"
CONFIG_KEY = "app_config_value"

# 4. Comments with example secrets (should be excluded)
# AWS_KEY=AKIAIOSFODNN7EXAMPLE
# api_key = "sk_live_1234567890abcdefghij1234567890abcdefghij"
# GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890abcdefghij"

/*
 * Multi-line comment with example secrets
 * AWS_KEY=AKIAIOSFODNN7EXAMPLE
 * api_key = "sk_live_1234567890"
 */

-- SQL comment with example
-- password = "secret123"

; Lisp comment
; key = "example-key"

# 5. Documentation-style examples
"""
Example usage:
    api_key = "your-api-key-here"
    secret = "CHANGEME"
    
Example AWS configuration:
    AWS_ACCESS_KEY_ID = "AKIAIOSFODNN7EXAMPLE"
    AWS_SECRET_ACCESS_KEY = "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY"
"""

# =============================================================================
# TEST FILE INDICATORS (should be excluded or low severity)
# =============================================================================

# This is a test file, so even if there were real-looking secrets,
# they should be treated with lower severity

TEST_API_KEY = "test_1234567890abcdefghij1234567890abcdefghij"
MOCK_TOKEN = "mock_token_1234567890"

# =============================================================================
# REAL SECRETS - These SHOULD be flagged (for testing detection)
# =============================================================================

# NOTE: These are FAKE but formatted like real secrets
# In a real scenario, these patterns should be detected

# Real-looking AWS key (not the example one)
# REAL_AWS_KEY = "AKIAIOSFODNN7REALKEY1"  # Commented out for testing

# Real-looking GitHub token
# REAL_GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890abcdefghij"  # Commented out

# Real-looking Stripe key
# REAL_STRIPE_KEY = "sk_live_1234567890abcdefghij1234567890abcdef"  # Commented out
