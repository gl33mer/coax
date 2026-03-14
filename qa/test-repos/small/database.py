"""
Database connection module.
Contains intentional secret for testing.
"""

import os

# Clean code - no secrets here
def get_connection_string():
    """Build database connection string from environment."""
    host = os.getenv("DB_HOST", "localhost")
    port = os.getenv("DB_PORT", 5432)
    database = os.getenv("DB_NAME", "devshield")
    return f"postgresql://{host}:{port}/{database}"


# HARDCODED SECRET - This should be detected! (CRITICAL)
# In real code, this would come from environment variables
PRODUCTION_DB_URL = "postgresql://admin:SuperSecretPassword123@prod-db.example.com:5432/maindb"


def connect():
    """Connect to the database."""
    conn_string = get_connection_string()
    print(f"Connecting to {conn_string}")
    return conn_string
