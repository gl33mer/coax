# HANDOFF.md — Coax Scanner State

**Date:** March 17, 2026  
**Version:** v0.8.0 (benchmark complete)  
**Repository:** https://github.com/PropertySightlines/coax

---

## What Is Coax?

Coax is a **code trust scanner** written in Rust that answers: *"Is this code what it appears to be?"*

**Detects:**
- **Secrets & Credentials** - AWS keys, GitHub tokens, API keys, passwords (800+ patterns)
- **Unicode Attacks** - Glassworm, homoglyphs, bidirectional overrides, invisible characters
- **Vulnerabilities** - Common security misconfigurations via CFG analysis

**Components:**
- **Rust CLI** (`coax` binary) - Main scanner
- **TUI** - Terminal UI for interactive scanning
- **VS Code Extension** - In development (core functionality complete)

**Unique Value:**
- Only open-source scanner with comprehensive Unicode attack detection
- Known patterns bypass heuristic filters (prevents recall regressions)
- 0 real-world false positives on fastify + express repos

---

## Performance (as of March 17, 2026)

### Secrets Detection — Expanded Corpus (56 TP, 30 TN files)

| Tool | TP | FN | FP | TN | Precision | Recall | **F1** |
|------|----|----|----|----|-----------|--------|--------|
| Coax (before fixes) | 47 | 9 | 6 | 24 | 88.7% | 83.9% | 0.86 |
| **Coax (current)** | **53** | **3** | **1** | **29** | **98.1%** | **94.6%** | **0.96** |
| Gitleaks | 49 | 7 | 3 | 27 | 94.2% | 87.5% | 0.91 |

**✅ TARGET MET: Coax F1 (0.96) ≥ Gitleaks F1 (0.91)**  
**Improvement: +0.10 F1 points**

### Real-World False Positives

| Repo | Coax FPs | Gitleaks FPs |
|------|----------|--------------|
| fastify | **0** | 2 |
| express | **0** | 0 |

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
   ⚠️ CRITICAL: Known pattern matches BYPASS all heuristic filters
5. FP suppression — placeholder, hash context, PEM certs, vault encryption
6. File-type context — test/doc exclusion for non-pattern matches only
```

**Key Principle:** Known secret patterns are **PRIVILEGED** — they bypass steps 4 and 6. This was established after fixing three stacked bugs where heuristic filters silently discarded legitimate findings.

### Code Map

| File | Purpose | Key Functions |
|------|---------|---------------|
| `crates/coax-scanner/src/scanner.rs` | Main scan pipeline, filter hierarchy | `scan_content_internal()`, `is_known_secret_pattern()` |
| `crates/coax-scanner/src/secrets.rs` | Secret pattern definitions (organized by category) | `all_patterns()` |
| `crates/coax-scanner/src/context.rs` | Context analysis (test files, documentation, placeholders) | `is_placeholder()`, `is_aws_example()` |
| `crates/coax-scanner/src/source_provider.rs` | File enumeration with extension filtering | `should_scan_extension()`, `should_scan_extensionless_file()` |
| `crates/coax-scanner/src/token_efficiency.rs` | Entropy-based FP reduction | `is_likely_false_positive()` |
| `crates/coax-scanner/src/word_filter.rs` | Common word detection for FP reduction | `should_filter()` |
| `crates/coax-scanner/src/unicode/` | Unicode attack detection (5 detectors) | `UnicodeScanner`, `HomoglyphDetector`, etc. |
| `crates/coax-cli/src/main.rs` | CLI entry point | `run_scan()` |

---

## Known Gaps

### Remaining FN (3 files)

1. **slack_tokens.txt** - Pattern may not match test data format
   - **Why missed:** Pattern regex may be too strict
   - **Fix approach:** Review `SLACK_TOKEN` pattern against test data

2. **twilio_keys.txt** - Pattern may not match test data format
   - **Why missed:** Pattern regex may be too strict
   - **Fix approach:** Review `TWILIO_API_KEY` pattern against test data

3. **xml-config.xml** - XML context may not be handled correctly
   - **Why missed:** XML attribute patterns not covered
   - **Fix approach:** Add XML-specific pattern handling

### Remaining FP (1 file)

1. **hash-values.json** - HIGH_ENTROPY_STRING matching SHA256 hashes
   - **Why wrong:** Hash context detection doesn't handle JSON key-value format
   - **Fix approach:** Improve `is_non_secret_content()` to parse JSON context

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
gitleaks detect --source coax-benchmarks/datasets/secrets-expanded/ \
  --report-format json --report-path results/gitleaks-results.json --no-git

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
4. **Placeholder detection** - "your-*", "insert-*", "replace-*", "TODO", "CHANGE_ME", angle brackets
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

## Push Status

⚠️ **Push may be blocked by GitHub secret scanning** - Test fixtures contain intentional example secrets.

**Resolution:**
- `.github/secret_scanning.yml` configured with exclusions
- If push fails, contact GitHub support or approve secrets via the GitHub UI

---

## Contact

For questions about scanner state or architecture, refer to this document and the inline code comments in `scanner.rs`.
