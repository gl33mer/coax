# Homoglyph in Strings and Comments - True Positives
# Homoglyph attacks in string content and comments

# String with homoglyphs that spoof a URL
# The 'а' is Cyrillic, making this a phishing attempt
url = "https://exаmple.com"  # Cyrillic 'а' in "example"

# Email with homoglyph
email = "admin@cоmpany.com"  # Cyrillic 'о' in "company"

# Comment that looks legitimate but uses homoglyphs
# This is a sаfe function  # Cyrillic 'а'
def sаfe_function():  # Cyrillic 'а' in function name
    pass

# String that spoofs a command
command = "ls -la"  # Cyrillic 'а' that looks like Latin

# Path traversal with homoglyph
path = "/hоme/user/.ssh/id_rsa"  # Cyrillic 'о'

# JSON with homoglyph keys
config_json = '''
{
    "аpi_key": "stolen",
    "tοken": "fake",
    "sеcret": "exposed"
}
'''
