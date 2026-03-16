// Code Identifiers - True Negatives
// These are code identifiers that should NOT be flagged as secrets

// Variable names that might look like tokens
const apiKey = "my_api_key_variable";
const authToken = "this_is_just_a_variable_name";
const secretKey = "not_a_real_secret";
const accessToken = "placeholder_value";

// Function names
function generateToken() {
  return "token_generator_function";
}

function validateApiKey(key) {
  return key !== null;
}

// Class names and constants
const API_KEY_LENGTH = 32;
const TOKEN_EXPIRY = 3600;
const SECRET_MANAGER = "SecretManagerService";

// Test fixtures
const MOCK_API_KEY = "test_api_key_12345";
const MOCK_TOKEN = "mock_token_for_testing";
const FAKE_SECRET = "this_is_fake_data";

// Documentation examples
/**
 * @param {string} apiKey - Your API key (example: "your_api_key_here")
 * @param {string} token - Auth token (example: "your_token_here")
 */

// TypeScript interfaces
interface AuthConfig {
  apiKey: string;
  token: string;
  secret?: string;
}
