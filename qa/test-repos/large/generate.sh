#!/bin/bash
# Generate large test repository files

BASE_DIR="$(cd "$(dirname "$0")" && pwd)"
SRC_DIR="$BASE_DIR/src"

echo "Generating large test repository..."

# Create directory structure
mkdir -p "$SRC_DIR"/{components,services,utils,config,tests,models,controllers,middleware}

# Generate clean JavaScript files
for i in $(seq 1 200); do
    cat > "$SRC_DIR/components/component_$i.js" << 'EOF'
/**
 * Auto-generated component file
 * Clean code - no secrets
 */

import React from 'react';

const Component = ({ prop }) => {
    return (
        <div className="component">
            <h1>Component {ID}</h1>
            <p>Prop value: {prop}</p>
        </div>
    );
};

export default Component;
EOF
    sed -i "s/{ID}/$i/g" "$SRC_DIR/components/component_$i.js"
done

# Generate clean Python files
for i in $(seq 1 200); do
    cat > "$SRC_DIR/services/service_$i.py" << EOF
"""
Auto-generated service file
Clean code - no secrets
"""

class Service$i:
    """Service class $i."""
    
    def __init__(self):
        self.id = $i
        self.name = f"Service $i"
    
    def process(self, data):
        """Process data."""
        return {"id": self.id, "data": data}
    
    def validate(self, data):
        """Validate data."""
        return data is not None
EOF
done

# Generate config files with secrets (for testing detection)
for i in $(seq 1 10); do
    cat > "$SRC_DIR/config/config_$i.json" << EOF
{
    "name": "Config $i",
    "aws_access_key": "AKIAIOSFODNN7EXAMPLE$i",
    "github_token": "ghp_1234567890abcdefghij1234567890abcdefghij",
    "stripe_key": "sk_live_1234567890abcdefghij123456",
    "settings": {
        "debug": false,
        "log_level": "info"
    }
}
EOF
done

# Generate secret files
for i in $(seq 1 5); do
    cat > "$SRC_DIR/secrets_$i.py" << EOF
"""
Secret file $i - for testing detection
"""

AWS_KEY = "AKIAIOSFODNN7EXAMPLE$i"
GITHUB_TOKEN = "ghp_1234567890abcdefghij1234567890abcdefghij"
STRIPE_KEY = "sk_live_1234567890abcdefghij123456"
EOF
done

# Generate .env files
cat > "$BASE_DIR/.env.dev" << 'EOF'
NODE_ENV=development
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
DATABASE_URL=postgresql://user:password@localhost:5432/devdb
EOF

cat > "$BASE_DIR/.env.staging" << 'EOF'
NODE_ENV=staging
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
MONGODB_URI=mongodb://admin:password@cluster.example.com:27017/staging
EOF

cat > "$BASE_DIR/.env.test" << 'EOF'
NODE_ENV=test
TEST_API_KEY=test_key_1234567890
EOF

echo "Generated large test repository:"
find "$BASE_DIR" -type f | wc -l
echo "files created"
