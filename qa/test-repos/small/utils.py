"""
Utility functions for the small test repository.
All clean code - no secrets.
"""

import hashlib
import json
from typing import Any, Dict, List


def hash_string(s: str) -> str:
    """Hash a string using SHA256."""
    return hashlib.sha256(s.encode()).hexdigest()


def parse_json(json_str: str) -> Dict[str, Any]:
    """Parse a JSON string."""
    return json.loads(json_str)


def to_json(obj: Any) -> str:
    """Convert an object to JSON string."""
    return json.dumps(obj, indent=2)


def flatten_list(nested: List[Any]) -> List[Any]:
    """Flatten a nested list."""
    result = []
    for item in nested:
        if isinstance(item, list):
            result.extend(flatten_list(item))
        else:
            result.append(item)
    return result


def merge_dicts(dict1: Dict, dict2: Dict) -> Dict:
    """Merge two dictionaries."""
    return {**dict1, **dict2}


def safe_get(dictionary: Dict, key: str, default: Any = None) -> Any:
    """Safely get a value from a dictionary."""
    return dictionary.get(key, default)
