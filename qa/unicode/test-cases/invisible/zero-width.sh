#!/bin/bash
# Zero-width character injection test

# Variable name with zero-width space (U+200B)
PASSW‌ORD="secret"  # Contains ZWSP between W and O

# Command with zero-width joiner (U+200D)
echo "test‍"  # Contains ZWJ

# Zero-width non-joiner (U+200C)
NAME‌="admin"  # Contains ZWNJ
