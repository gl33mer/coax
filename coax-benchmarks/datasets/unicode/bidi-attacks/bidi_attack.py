# BiDi Attacks in Python - True Positives
# These contain BiDi override characters that should be detected

# U+202E (RIGHT-TO-LEFT OVERRIDE) attack
def authenticate(user_input):
    is_admin = False  #‮//‭True

# U+202B (RIGHT-TO-LEFT EMBEDDING)
access_level = "user"  #‮//‬"admin"

# U+202D (LEFT-TO-RIGHT OVERRIDE) in assignment
permissions = []  #‭//‬["read", "write", "delete"]

# BiDi in docstring that affects rendering
"""
This function checks‮‮‮
the user permissions
"""
def check_perms():
    return True

# Multiple BiDi characters
secret_key = "abc123"  #‮‮‮//‭‭‭"xyz789"

# BiDi in conditional (makes condition appear different)
if user_role == "admin":  #‮//‭user_role == "guest"
    grant_access()

# BiDi in function name display
def‮‮‮execute_command‮‮‮(cmd):
    pass  # Function name appears different
