"""
Test utilities - clean code, no secrets.
"""

import unittest
from typing import Any, Dict, List


def assert_equal(actual: Any, expected: Any, msg: str = None) -> None:
    """Assert that two values are equal."""
    if actual != expected:
        raise AssertionError(msg or f"Expected {expected}, got {actual}")


def assert_not_none(value: Any, msg: str = None) -> None:
    """Assert that a value is not None."""
    if value is None:
        raise AssertionError(msg or "Value is None")


def assert_in(item: Any, container: List, msg: str = None) -> None:
    """Assert that an item is in a container."""
    if item not in container:
        raise AssertionError(msg or f"{item} not in {container}")


def create_test_data() -> Dict:
    """Create test data."""
    return {
        "name": "Test",
        "value": 42,
        "items": [1, 2, 3]
    }


class BaseTestCase(unittest.TestCase):
    """Base test case with common utilities."""

    def setUp(self):
        """Set up test fixtures."""
        self.test_data = create_test_data()

    def tearDown(self):
        """Tear down test fixtures."""
        pass

    def assert_valid_response(self, response: Dict) -> None:
        """Assert that a response is valid."""
        self.assertIn("success", response)
        self.assertTrue(response["success"])


if __name__ == "__main__":
    unittest.main()
