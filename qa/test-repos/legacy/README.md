# Legacy Test Repository

**Purpose:** Outdated pattern detection, legacy code testing
**Size:** ~30 files
**Expected scan time:** 1-3 seconds
**Languages:** PHP, Java, old Python, shell scripts

## Contents

- Legacy PHP applications
- Old Java code
- Deprecated Python patterns
- Shell scripts with embedded secrets

## Known Secrets

| File | Language | Secret Type |
|------|----------|-------------|
| config.php | PHP | MySQL password |
| app.properties | Java | API key |
| deploy.sh | Shell | SSH key, tokens |
| old_config.py | Python | Generic secrets |

## Legacy Patterns

This repository tests detection of:
- Old-style configuration files
- Deprecated API key formats
- Shell script embedded secrets
- Hardcoded database credentials

## Usage

```bash
# Scan this repository
./target/release/coax scan -p qa/test-repos/legacy

# Expected: 8+ findings (legacy patterns)
```
