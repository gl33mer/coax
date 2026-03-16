# Coax Benchmark Suite

**Version:** 0.8.3
**Date:** March 16, 2026

Honest, multi-dimensional benchmarks for Coax — Code Trust & Supply-Chain Attack Scanner.

---

## Mission

> "When a measure becomes a target, it ceases to be a good measure."

This benchmark suite is designed to provide **honest, credible measurements** of Coax's capabilities across multiple dimensions. We do not optimize for a single score. We report results transparently, including areas where competitors may outperform us.

**Key Principle:** If Coax loses on secrets recall against TruffleHog, that's fine — we show the Unicode categories where no competitor even has a score.

---

## Quick Start

```bash
# Run all benchmarks
./run-benchmarks.sh

# Create git history test repository first
./datasets/git-history/create-test-repo.sh

# Run specific dataset
coax scan datasets/secrets/true-positives

# Run git history benchmark
coax scan --git-history datasets/git-history/test-repo
```

---

## Directory Structure

```
coax-benchmarks/
├── README.md                    # This file
├── run-benchmarks.sh           # Automation script
├── datasets/
│   ├── secrets/
│   │   ├── true-positives/     # Known secrets (labeled)
│   │   ├── true-negatives/     # Look suspicious but aren't
│   │   └── encoded/            # Base64, hex, URL-encoded
│   ├── unicode/
│   │   ├── bidi-attacks/
│   │   ├── homoglyphs/
│   │   ├── invisible-chars/
│   │   └── safe-i18n/          # Legitimate multilingual code
│   └── git-history/            # Repo with secrets in old commits
└── results/
    ├── latest.md               # Auto-generated comparison
    └── history/                # Trend tracking
```

---

## Dataset Categories

### Secrets Detection

| Dataset | Description | Files | Expected |
|---------|-------------|-------|----------|
| `true-positives` | Real credential patterns | 12 | Detect all |
| `true-negatives` | UUIDs, colors, hashes, etc. | 11 | Flag none |
| `encoded` | Base64/hex/URL-encoded secrets | 3 | Detect when decoded |

**True Positives Include:**
- AWS credentials (AKIA...)
- GitHub tokens (ghp_...)
- Stripe keys (sk_live_...)
- Database connection strings
- Private keys (RSA, SSH, PGP)
- Slack tokens (xoxb-...)
- Google API keys (AIza...)
- Twilio credentials
- NPM tokens
- Azure credentials
- Mailgun keys
- Datadog keys

**True Negatives Include:**
- UUIDs in configuration
- CSS hex colors
- Code identifiers (variable names)
- Test fixtures and mock data
- Lock file hashes (package-lock.json, Cargo.lock)
- Git commit SHAs
- Base64-encoded non-secrets (images, public data)
- Cryptographic hashes/checksums
- Minified JavaScript
- Placeholder/documentation values
- SRI hashes in HTML

### Unicode Attack Detection

| Dataset | Description | Files | Attack Type |
|---------|-------------|-------|-------------|
| `bidi-attacks` | BiDi override/reorder | 5 | U+202A-U+202E, U+2066-U+2069 |
| `homoglyphs` | Confusable characters | 5 | Cyrillic/Greek vs Latin |
| `invisible-chars` | Zero-width characters | 5 | ZWSP, ZWJ, ZWNJ, etc. |
| `safe-i18n` | Legitimate i18n code | 5 | Should NOT flag |

**Unicode Attack Patterns:**
- **BiDi (Trojan Source):** RIGHT-TO-LEFT OVERRIDE (U+202E), LEFT-TO-RIGHT OVERRIDE (U+202D), etc.
- **Homoglyphs:** Cyrillic 'а' (U+0430) vs Latin 'a' (U+0061), Greek 'ο' (U+03BF) vs Latin 'o', etc.
- **Invisible Characters:** Zero-Width Space (U+200B), Zero-Width Joiner (U+200D), Variation Selectors (U+FE00-U+FE0F)

**Safe i18n (Should NOT Flag):**
- Greek variables and comments
- Cyrillic (Russian, Ukrainian, Bulgarian) code
- Chinese strings and identifiers
- Japanese (Hiragana, Katakana, Kanji) code
- Arabic and Hebrew text (RTL without overrides)

### Git History Detection

| Dataset | Description | Commits | Expected |
|---------|-------------|---------|----------|
| `test-repo` | Secrets committed then removed | 9 | Find historical secrets |

Tests Coax's ability to find secrets that were:
- Committed to git history
- Removed in later commits
- Still present in `.git/` objects

---

## Metrics Definitions

| Metric | Formula | Target | Description |
|--------|---------|--------|-------------|
| **True Positive Rate (Recall)** | TP / (TP + FN) | >90% | Percentage of actual secrets detected |
| **False Positive Rate** | FP / (FP + TN) | <5% | Percentage of clean files incorrectly flagged |
| **Precision** | TP / (TP + FP) | >95% | Percentage of findings that are real |
| **F1 Score** | 2 × (Precision × Recall) / (Precision + Recall) | >92% | Harmonic mean of precision and recall |
| **Scan Speed** | Files / Time | >100K files/s | Throughput on clean files |
| **Unicode Coverage** | Categories detected | 5/5 | Number of Unicode attack categories detected |

---

## Results Table Template

### Secrets Detection Comparison

| Tool | TP Rate | FP Rate | Precision | F1 Score |
|------|---------|---------|-----------|----------|
| Coax v0.8.3 | - | - | - | - |
| TruffleHog | - | - | - | - |
| Gitleaks | - | - | - | - |
| detect-secrets | - | - | - | - |

### Unicode Attack Detection Comparison

| Tool | BiDi | Homoglyphs | Invisible | Safe i18n (no FP) |
|------|------|------------|-----------|-------------------|
| Coax v0.8.3 | - | - | - | - |
| TruffleHog | — | — | — | — |
| Gitleaks | — | — | — | — |
| detect-secrets | — | — | — | — |

*Note: "—" indicates the tool does not support this detection category.*

### Performance Comparison

| Tool | Scan Speed (files/s) | Memory Usage | Git History (10K commits) |
|------|---------------------|--------------|---------------------------|
| Coax v0.8.3 | - | - | - |
| TruffleHog | - | - | - |
| Gitleaks | - | - | - |
| detect-secrets | - | - | - |

---

## Methodology

### Held-Out Test Set

To prevent overfitting, this suite includes:
- **Development set:** Used for tuning and debugging
- **Held-out set:** Sealed, only run for release validation

### Real-World Samples

Where possible, datasets include:
- Sanitized real leaked credentials
- Patterns from public breach notifications
- Actual code structures from open-source projects

### Adversarial Negatives

The true-negatives dataset includes strings designed to look like secrets:
- Password validation regex patterns
- Documentation examples with placeholder keys
- Base64-encoded non-secret data
- High-entropy minified code

---

## Running Benchmarks

### Prerequisites

```bash
# Install Coax
cargo install --path .

# Or build from source
cargo build --release

# Ensure coax is in PATH
export PATH="$PATH:$(pwd)/target/release"
```

### Full Benchmark Run

```bash
# Create git history test repo
cd coax-benchmarks
./datasets/git-history/create-test-repo.sh

# Run all benchmarks
./run-benchmarks.sh

# View results
cat results/latest.md
```

### Individual Dataset

```bash
# Scan secrets
coax scan datasets/secrets/true-positives --format json

# Scan unicode attacks
coax scan datasets/unicode/bidi-attacks --format json

# Scan git history
coax scan --git-history datasets/git-history/test-repo --format json
```

---

## Contributing

### Adding New Test Cases

1. Add file to appropriate dataset directory
2. Update dataset counts in this README
3. Run benchmarks to verify behavior
4. Document expected findings

### Reporting Issues

If you find:
- **False negatives:** Secrets not detected → Add to true-positives
- **False positives:** Clean files flagged → Add to true-negatives
- **Missing attack patterns:** → Add to unicode dataset

---

## Version History

| Version | Date | Changes |
|---------|------|---------|
| 0.8.3 | 2026-03-16 | Initial benchmark suite |

---

## License

Same as Coax main project.

---

*These benchmarks are designed for honesty, not marketing. Results may show Coax losing in some categories — that's intentional and builds credibility. Our differentiation is Unicode detection where competitors have no coverage.*
