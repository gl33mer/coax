#!/bin/bash
# Script to create the git-history test repository
# Run this to set up the test data for git-history benchmarking

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_DIR="$SCRIPT_DIR/test-repo"

# Remove existing test repo if present
rm -rf "$REPO_DIR"
mkdir -p "$REPO_DIR"
cd "$REPO_DIR"

# Initialize git repo
git init
git config user.email "test@example.com"
git config user.name "Test User"

# Commit 1: Add AWS credentials (should be detected in history)
cat > config.env << 'EOF'
# Application Configuration
AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE
AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY
DATABASE_URL=postgresql://admin:SuperSecretPassword123!@db.example.com:5432/production
EOF
git add config.env
git commit -m "Initial commit with configuration"
COMMIT_1=$(git rev-parse HEAD)

# Commit 2: Add GitHub token
cat > .github-token << 'EOF'
GITHUB_TOKEN=ghp_1234567890abcdefghij1234567890ABCDEF
EOF
git add .github-token
git commit -m "Add GitHub token for CI"
COMMIT_2=$(git rev-parse HEAD)

# Commit 3: Add application code
cat > app.js << 'EOF'
console.log("Hello World");
function main() {
    return true;
}
EOF
git add app.js
git commit -m "Add application code"
COMMIT_3=$(git rev-parse HEAD)

# Commit 4: Add Stripe key
cat > payment.js << 'EOF'
const stripe = require('stripe');
const stripeClient = stripe('sk_live_1234567890abcdefghijklmnopqrstuv');
module.exports = stripeClient;
EOF
git add payment.js
git commit -m "Add payment processing"
COMMIT_4=$(git rev-parse HEAD)

# Commit 5: Remove sensitive files (but they remain in history!)
git rm config.env .github-token
git commit -m "Remove sensitive configuration files"
COMMIT_5=$(git rev-parse HEAD)

# Commit 6: More development
cat >> app.js << 'EOF'

function validate(input) {
    return input !== null;
}
EOF
git add app.js
git commit -m "Add validation function"
COMMIT_6=$(git rev-parse HEAD)

# Commit 7: Add private key (then remove)
cat > deploy.key << 'EOF'
-----BEGIN RSA PRIVATE KEY-----
MIIEpAIBAAKCAQEA0Z3VS5JJcds3xfn/ygWyF8PbnGy0AHB7MvXB8mFvNzXvZ5Jz
EXAMPLE_KEY_CONTENT_HERE_REPLACE_WITH_ACTUAL_KEY_DATA_BUT_HEADER_SHOULD_TRIGGER
-----END RSA PRIVATE KEY-----
EOF
git add deploy.key
git commit -m "Add deployment key"
COMMIT_7=$(git rev-parse HEAD)

# Commit 8: Remove private key
git rm deploy.key
git commit -m "Remove deployment key (security fix)"
COMMIT_8=$(git rev-parse HEAD)

# Commit 9: Current state - clean but history has secrets
cat >> app.js << 'EOF'

module.exports = { main, validate };
EOF
git add app.js
git commit -m "Finalize application"
COMMIT_9=$(git rev-parse HEAD)

echo "Git history test repository created successfully!"
echo "Total commits: 9"
echo ""
echo "Secrets in history (should be detected):"
echo "  - Commit $COMMIT_1: AWS credentials in config.env"
echo "  - Commit $COMMIT_2: GitHub token in .github-token"
echo "  - Commit $COMMIT_4: Stripe key in payment.js"
echo "  - Commit $COMMIT_7: RSA private key in deploy.key"
echo ""
echo "All secrets were removed in later commits but remain in git history."
echo "Run: coax scan --git-history $REPO_DIR"
