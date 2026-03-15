"""
Clean source file - should have no detections.
"""

import os
import sys

def hello_world():
    """Print hello world."""
    print("Hello, World!")

def calculate_sum(a, b):
    """Calculate sum of two numbers."""
    return a + b

class MyClass:
    """A simple class."""
    
    def __init__(self, name):
        self.name = name
    
    def greet(self):
        """Greet the user."""
        return f"Hello, {self.name}!"

if __name__ == "__main__":
    hello_world()
    obj = MyClass("Test")
    print(obj.greet())
