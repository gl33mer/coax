# Large Test Repository

**Purpose:** Performance testing, stress testing
**Size:** >1000 files
**Expected scan time:** 5-30 seconds
**Languages:** Multiple (JavaScript, Python, Rust, Go)

## Contents

- Generated source files (simulating large codebase)
- Mix of clean files and files with secrets
- Various file types and extensions

## Known Secrets

| File Pattern | Count | Secret Types |
|--------------|-------|--------------|
| config_*.json | 10 | Various API keys |
| secrets_*.py | 5 | AWS, GitHub, Stripe |
| .env.* | 3 | Database URLs, tokens |

## Generation

Files are generated programmatically to simulate a large codebase.

## Usage

```bash
# Generate files (if needed)
bash qa/test-repos/large/generate.sh

# Scan this repository
./target/release/coax scan -p qa/test-repos/large

# Expected: 20+ findings, scan time <30s
```

## Performance Targets

| Metric | Target |
|--------|--------|
| Scan time | <30s |
| Memory | <100MB |
| Files/sec | >50 |
