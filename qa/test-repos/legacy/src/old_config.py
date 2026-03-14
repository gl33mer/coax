"""
Legacy Python Configuration
Old-style configuration with hardcoded secrets

WARNING: Contains intentional secrets for testing.
"""

# Database Configuration (CRITICAL - should be detected)
DB_HOST = "localhost"
DB_PORT = 3306
DB_USER = "root"
DB_PASSWORD = "LegacyPassword123!"
DB_NAME = "legacy_db"

# Connection string format (CRITICAL - should be detected)
DATABASE_URL = "mysql://root:LegacyPassword123!@localhost:3306/legacy_db"

# Legacy API configuration
API_KEY = "legacy_api_key_1234567890abcdef"
API_SECRET = "legacy_secret_abcdefghijklmnop123456"

# Generic password assignment (HIGH - should be detected)
password = "admin123456"
passwd = "rootpassword"

# Old-style token
AUTH_TOKEN = "token_1234567890abcdefghijklmnopqrstuvwxyz"

# Redis configuration (CRITICAL - should be detected)
REDIS_URL = "redis://:RedisLegacyPassword@localhost:6379/0"


def get_connection():
    """Get database connection string."""
    return DATABASE_URL


def get_api_credentials():
    """Get API credentials."""
    return {
        "key": API_KEY,
        "secret": API_SECRET
    }


# Legacy authentication
def authenticate(username, password):
    """Legacy authentication function."""
    # Hardcoded credentials (bad practice!)
    admin_password = "admin123456"
    if username == "admin" and password == admin_password:
        return True
    return False
