# Test Fixtures - True Negatives
# These are test data files that should NOT be flagged as secrets

# Test AWS credentials (clearly marked as test/fake)
TEST_AWS_ACCESS_KEY_ID = "AKIAIOSFODNN7TESTKEY"
TEST_AWS_SECRET_ACCESS_KEY = "wJalrXUtnFEMI/K7MDENG/TestKeyExample"

# Mock database credentials
MOCK_DB_PASSWORD = "test_password_123"
TEST_DATABASE_URL = "postgresql://test:test@localhost:5432/testdb"

# Fake tokens for unit tests
FAKE_GITHUB_TOKEN = "ghp_TestToken1234567890abcdefgh"
MOCK_STRIPE_KEY = "sk_test_MockKeyForTesting123456"

# Example values in documentation
EXAMPLE_API_KEY = "your_api_key_goes_here"
PLACEHOLDER_TOKEN = "<INSERT_TOKEN_HERE>"
SAMPLE_SECRET = "example_secret_value"

# Test configuration
test_config = {
    "api_key": "test_key_12345",
    "secret": "test_secret_67890",
    "token": "test_token_abcdef",
    "is_mock": True,
    "environment": "test"
}

# Fixture data with obviously fake values
FIXTURE_USER = {
    "id": "test-user-123",
    "api_key": "fixture_key_abc123",
    "auth_token": "fixture_token_xyz789"
}
