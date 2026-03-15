# Homoglyph Attack Sample - Cyrillic 'а' vs Latin 'a'
# This file contains confusable characters for testing

# Variable with Cyrillic 'а' (U+0430) instead of Latin 'a'
pаssword = "secret123"  # Contains Cyrillic а

# Function name with Cyrillic characters
def lоgin():  # Contains Cyrillic о (U+043E)
    return True

# Multiple confusables
usеr_nаme = "admin"  # Cyrillic е and а
