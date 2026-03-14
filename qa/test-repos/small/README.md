# Small Test Repository

**Purpose:** Quick validation, development testing
**Size:** <100 files
**Expected scan time:** <1 second
**Languages:** Python

## Contents

- Python source files with intentional secrets
- Clean Python files (no secrets)
- Configuration files

## Known Secrets

| File | Line | Secret Type |
|------|------|-------------|
| config.py | 5 | AWS_ACCESS_KEY |
| config.py | 6 | AWS_SECRET_KEY |
| .env | 1 | GITHUB_PAT |
| .env | 2 | STRIPE_SECRET_KEY |
| database.py | 10 | POSTGRESQL_CONNECTION_STRING |

## Usage

```bash
# Scan this repository
./target/release/coax scan -p qa/test-repos/small

# Expected: 5 findings (2 critical, 2 critical, 1 high)
```
