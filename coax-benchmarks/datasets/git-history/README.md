# Git History Test Dataset

This directory contains a test repository with secrets embedded in git history.

## Setup Instructions

To create the test repository:

```bash
cd git-history
git init
git config user.email "test@example.com"
git config user.name "Test User"

# Commit 1: Initial commit with secret
echo 'AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE' > config.env
echo 'AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY' >> config.env
git add config.env
git commit -m "Initial commit with config"

# Commit 2: Add more code
echo 'console.log("Hello World");' > app.js
git add app.js
git commit -m "Add application code"

# Commit 3: Remove secret (but it's still in history!)
echo '# Configuration file' > config.env
git add config.env
git commit -m "Remove sensitive data"

# Commit 4: More development
echo 'function main() { return true; }' >> app.js
git add app.js
git commit -m "Add main function"
```

## Expected Findings

When scanning with `coax scan --git-history`:

| Commit | File | Secret Type | Still Present |
|--------|------|-------------|---------------|
| 1 | config.env | AWS Access Key | No |
| 1 | config.env | AWS Secret Key | No |

## Test Cases

1. **Historical Detection**: Secrets should be found even though removed
2. **Still-Present Flag**: Should correctly show `still_present: false`
3. **Commit Context**: Full commit info (SHA, author, date, message)
4. **Performance**: Should scan quickly even with many commits

## Metrics to Track

- Time to scan N commits
- Accuracy of `still_present` detection
- Correct commit attribution
