"""
Clean Python module - no secrets.
Utility functions for the backend.
"""

from typing import Any, Dict, List, Optional
from datetime import datetime


def parse_datetime(date_str: str) -> Optional[datetime]:
    """Parse a datetime string."""
    try:
        return datetime.fromisoformat(date_str)
    except ValueError:
        return None


def format_datetime(dt: datetime) -> str:
    """Format a datetime object."""
    return dt.isoformat()


def validate_email(email: str) -> bool:
    """Validate an email address."""
    return "@" in email and "." in email.split("@")[-1]


def sanitize_input(text: str) -> str:
    """Sanitize user input."""
    return text.strip().replace("<", "&lt;").replace(">", "&gt;")


def merge_configs(base: Dict, override: Dict) -> Dict:
    """Merge two configuration dictionaries."""
    result = base.copy()
    result.update(override)
    return result


class Cache:
    """Simple in-memory cache."""

    def __init__(self, ttl: int = 300):
        self._data: Dict[str, Any] = {}
        self._timestamps: Dict[str, float] = {}
        self.ttl = ttl

    def get(self, key: str) -> Optional[Any]:
        """Get a value from cache."""
        if key in self._data:
            return self._data[key]
        return None

    def set(self, key: str, value: Any) -> None:
        """Set a value in cache."""
        self._data[key] = value
        self._timestamps[key] = datetime.now().timestamp()

    def delete(self, key: str) -> bool:
        """Delete a value from cache."""
        if key in self._data:
            del self._data[key]
            del self._timestamps[key]
            return True
        return False

    def clear(self) -> None:
        """Clear all cache entries."""
        self._data.clear()
        self._timestamps.clear()
