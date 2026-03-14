"""
Clean legacy module - no secrets.
Utility functions that follow old patterns but are secure.
"""

import os
import hashlib


def hash_password(password: str) -> str:
    """Hash a password (using modern practices)."""
    # In legacy code this might be MD5, but we use SHA256
    return hashlib.sha256(password.encode()).hexdigest()


def get_env_var(name: str, default: str = None) -> str:
    """Get environment variable safely."""
    return os.getenv(name, default)


def validate_input(data: str) -> bool:
    """Validate user input."""
    if not data:
        return False
    if len(data) > 1000:
        return False
    return True


def format_response(success: bool, message: str, data: dict = None) -> dict:
    """Format a standard API response."""
    response = {
        "success": success,
        "message": message
    }
    if data:
        response["data"] = data
    return response


class LegacyHandler:
    """Legacy request handler."""

    def __init__(self):
        self.requests_handled = 0

    def handle(self, request: dict) -> dict:
        """Handle a request."""
        self.requests_handled += 1
        return format_response(True, "Request handled")

    def get_stats(self) -> dict:
        """Get handler statistics."""
        return {
            "requests_handled": self.requests_handled
        }
