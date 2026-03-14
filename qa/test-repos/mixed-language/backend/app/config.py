"""
Backend Configuration
Python/FastAPI configuration module

WARNING: Contains intentional secrets for testing.
"""

import os
from typing import Optional

# Database Configuration (CRITICAL - should be detected)
DATABASE_URL = "postgresql://admin:SuperSecretPassword123@db.example.com:5432/production"
REDIS_URL = "redis://:RedisPassword123@redis.example.com:6379/0"

# AWS Configuration (CRITICAL - should be detected)
AWS_ACCESS_KEY_ID = os.getenv("AWS_ACCESS_KEY_ID", "AKIAIOSFODNN7EXAMPLE")
AWS_SECRET_ACCESS_KEY = os.getenv("AWS_SECRET_ACCESS_KEY", "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY")

# JWT Configuration
JWT_SECRET = "super-secret-jwt-key-that-should-be-in-env"
JWT_ALGORITHM = "HS256"
JWT_EXPIRATION = 3600


class Settings:
    """Application settings."""

    def __init__(self):
        self.database_url = DATABASE_URL
        self.redis_url = REDIS_URL
        self.aws_access_key = AWS_ACCESS_KEY_ID
        self.aws_secret_key = AWS_SECRET_ACCESS_KEY
        self.jwt_secret = JWT_SECRET
        self.debug = False
        self.log_level = "INFO"

    def get_database_url(self) -> str:
        """Get database URL."""
        return self.database_url

    def get_redis_url(self) -> str:
        """Get Redis URL."""
        return self.redis_url


settings = Settings()
