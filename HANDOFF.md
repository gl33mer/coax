# HANDOFF.md — Coax Scanner State

**Date:** March 17, 2026  
**Version:** v0.8.0 (benchmark complete)

---

## What Is Coax?

Coax is a code trust scanner written in Rust that detects:
- **Secrets & Credentials** - AWS keys, GitHub tokens, API keys, passwords
- **Unicode Attacks** - Glassworm, homoglyphs, bidirectional overrides, invisible characters
- **Vulnerabilities** - Common security misconfigurations

**Components:**
- Rust CLI (`coax` binary)
- TUI (terminal UI)
- VS Code extension (in development)

**Mission:** Answer "Is this code what it appears to be?"

---

## Performance (as of March 17, 2026)

### Secrets Detection — Expanded Corpus (56 TP, 30 TN files)

| Tool | TP | FN | FP | TN | Precision | Recall | F1 |
|------|----|----|----|----|-----------|--------|----|
| Coax (before fixes) | 47 | 9 | 6 | 24 | 88.7% | 83.9% | 0.86 |
| **Coax (current)** | **53** | **3** | **1** | **29** | **98.1%** | **94.6%** | **0.96** |
| Gitleaks | 49 | 7 | 3 | 27 | 94.2% | 87.5% | 0.91 |

**✅ TARGET MET: Coax F1 (0.96) ≥ Gitleaks F1 (0.91)**

### Real-World False Positives

| Repo | Coax FPs | Gitleaks FPs |
|------|----------|--------------|
| fastify | 0 | 2 |
| express | 0 | 0 |

### Unicode Detection

| Metric | Before | After |
|--------|--------|-------|
| TP | 15 | 15 |
| FN | 0 | 0 |
| FP | 4 | 1 |
| TN | 1 | 4 |
| F1 | 0.88 | 0.96 |

### Tests

**185 tests passing.** Run: `cargo test`

---

## Architecture

### Scan Pipeline — Filter Hierarchy

```
1. Extension filter — skips binary/irrelevant file types
2. Binary check — skips files with null bytes in first 512 bytes
3. Pattern matching — ALL known patterns run against content
4. Heuristic filters — entropy, word filter, token efficiency
   CRITICAL: Known pattern matches BYPASS all heuristic filters.
5. FP suppression — placeholder, hash context, PEM certs, vault encryption
6. File-type context — test/doc exclusion for non-pattern matches only
```

**Key Principle:** Known secret patterns are PRIVILEGED — they bypass steps 4 and 6. This prevents recall regressions from heuristic filtering.

### Code Map

| File | Purpose |
|------|---------|
| `crates/coax-scanner/src/scanner.rs` | Main scan pipeline, filter hierarchy |
| `crates/coax-scanner/src/secrets.rs` | Secret pattern definitions (organized by category) |
| `crates/coax-scanner/src/context.rs` | Context analysis (test files, documentation, placeholders) |
| `crates/coax-scanner/src/source_provider.rs` | File enumeration with extension filtering |
| `crates/coax-scanner/src/token_efficiency.rs` | Entropy-based FP reduction |
| `crates/coax-scanner/src/word_filter.rs` | Common word detection for FP reduction |
| `crates/coax-scanner/src/unicode/` | Unicode attack detection (5 detectors) |
| `crates/coax-cli/src/main.rs` | CLI entry point |

---

## Known Gaps

### Remaining FN (3 files)

1. **slack_tokens.txt** - Pattern may not match test data format
2. **twilio_keys.txt** - Pattern may not match test data format
3. **xml-config.xml** - XML context may not be handled correctly

**Fix approach:** Review pattern regexes against test data, add XML-specific handling if needed.

### Remaining FP (1 file)

1. **hash-values.json** - HIGH_ENTROPY_STRING matching SHA256 hashes despite context detection

**Fix approach:** Improve hash context detection to handle JSON key-value format.

### Encoded Detection

Base64/hex/URL-encoded secret detection exists but has gaps. Not yet benchmarked against a dedicated corpus.

### Unicode Finding Grouping

Per-character findings, not per-line. UX issue, not detection issue.

---

## Benchmark Infrastructure

- **Corpus location:** `coax-benchmarks/datasets/secrets-expanded/`
- **Ground truth:** `coax-benchmarks/datasets/secrets-expanded/ground-truth.yaml`
- **Results:** `coax-benchmarks/results/expanded/`

### How to Reproduce

```bash
# Run Coax on expanded corpus
./target/release/coax scan coax-benchmarks/datasets/secrets-expanded/ --format json > results/coax-results.json

# Run Gitleaks for comparison
gitleaks detect --source coax-benchmarks/datasets/secrets-expanded/ --report-format json --report-path results/gitleaks-results.json --no-git

# Analyze results (Python script)
python3 analyze-benchmark.py
```

---

## Development

```bash
# Build
cargo build --release

# Test
cargo test

# Lint
cargo clippy

# Format
cargo fmt
```

---

## Key Fixes Implemented (v0.7 → v0.8)

1. **Extension filter expanded** - Added .csv, .gradle, .ipynb, .npmrc, .pp, .sls
2. **Extensionless file handling** - Jenkinsfile, Makefile, Vagrantfile, Dockerfile
3. **Binary file detection** - Null byte check in first 512 bytes
4. **Placeholder detection** - "your-*", "insert-*", "replace-*", "TODO", "CHANGE_ME", etc.
5. **PEM certificate exclusion** - Public certs excluded, private keys still flagged
6. **Encrypted vault exclusion** - ENC[...] format (Ansible Vault, SOPS)
7. **Hash context exclusion** - SHA256/SHA512 hashes with context indicators
8. **Unicode FP fix** - Underscore not treated as Latin script character
9. **NPM token pattern** - Changed from `{36}` to `{32,}` for flexibility

---

## Next Steps (Phase 2 - Not Yet Started)

1. Fix remaining 3 FN (Slack, Twilio, XML patterns)
2. Fix remaining 1 FP (hash-values.json)
3. Encoded detection hardening (base64/hex/URL)
4. Unicode finding grouping (per-line instead of per-character)
5. VS Code extension completion

---

## Contact

For questions about scanner state or architecture, refer to this document and the inline code comments.
