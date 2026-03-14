# Medium Test Repository

**Purpose:** Standard testing, feature validation
**Size:** 100-1000 files (simulated with multiple modules)
**Expected scan time:** 1-5 seconds
**Languages:** JavaScript, Python

## Contents

- JavaScript source files with API keys
- Python modules with various secrets
- Configuration files
- Test files (clean)

## Known Secrets

| File | Line | Secret Type |
|------|------|-------------|
| src/api/client.js | 15 | OPENAI_API_KEY |
| src/api/client.js | 20 | ANTHROPIC_API_KEY |
| src/config/payment.js | 8 | STRIPE_SECRET_KEY |
| src/config/payment.js | 12 | SQUARE_ACCESS_TOKEN |
| src/services/email.py | 25 | SENDGRID_API_KEY |
| src/services/email.py | 30 | MAILGUN_API_KEY |
| .env.production | 3 | AWS_ACCESS_KEY |
| .env.production | 4 | AWS_SECRET_KEY |
| .env.production | 7 | MONGODB_CONNECTION_STRING |

## Usage

```bash
# Scan this repository
./target/release/coax scan -p qa/test-repos/medium

# Expected: 9+ findings
```
