# Mixed-Language Test Repository

**Purpose:** Multi-language support validation
**Size:** ~50 files
**Expected scan time:** 2-5 seconds
**Languages:** JavaScript, Python, Rust

## Contents

- JavaScript frontend (React components)
- Python backend (FastAPI)
- Rust utilities (CLI tools)
- Shared configuration

## Known Secrets

| File | Language | Secret Type |
|------|----------|-------------|
| frontend/src/api.js | JavaScript | OPENAI_API_KEY |
| backend/app/config.py | Python | DATABASE_URL |
| backend/.env | Python | AWS credentials |
| rust-cli/src/main.rs | Rust | GitHub token |

## Usage

```bash
# Scan this repository
./target/release/coax scan -p qa/test-repos/mixed-language

# Expected: 5+ findings across multiple languages
```
