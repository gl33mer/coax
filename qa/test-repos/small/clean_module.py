"""
Clean Python module - no secrets.
This file should not trigger any findings.
"""


def add(a: int, b: int) -> int:
    """Add two numbers."""
    return a + b


def multiply(a: int, b: int) -> int:
    """Multiply two numbers."""
    return a * b


def greet(name: str) -> str:
    """Greet someone by name."""
    return f"Hello, {name}!"


class Calculator:
    """Simple calculator class."""

    def __init__(self):
        self.result = 0

    def add(self, value: int) -> None:
        """Add value to result."""
        self.result += value

    def subtract(self, value: int) -> None:
        """Subtract value from result."""
        self.result -= value

    def get_result(self) -> int:
        """Get current result."""
        return self.result

    def reset(self) -> None:
        """Reset result to zero."""
        self.result = 0


if __name__ == "__main__":
    calc = Calculator()
    calc.add(10)
    calc.add(5)
    print(f"Result: {calc.get_result()}")
