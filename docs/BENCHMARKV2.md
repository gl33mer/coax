

```
# Coax — Expanded Benchmark & Per-Pattern Tests

## Context

Coax now beats Gitleaks on our benchmark (F1=0.81 vs 0.69). But that
benchmark is only 15 TP + 11 TN files — a smoke test, not proof.
We need to validate that the scanner works on diverse, realistic inputs
before we can publish these numbers credibly.

This task has three parts:
1. Per-pattern unit tests (regression prevention)
2. Expanded test corpus (validation)
3. Real-world repo scan (FP rate at scale)

---

## Task 1: Per-Pattern Unit Tests

For EVERY pattern in secrets.rs, create a test that validates detection.
Follow TruffleHog's 5 canonical test cases per pattern:

| Case | Input | Expected |
|------|-------|----------|
| 1 | Realistic secret matching the pattern | DETECTED |
| 2 | Same pattern with different valid format | DETECTED |
| 3 | Similar-looking but invalid string | NOT detected |
| 4 | Pattern keyword present but no secret | NOT detected |
| 5 | Secret embedded in realistic code context | DETECTED |

Structure:
```
crates/coax-scanner/src/patterns/
  tests/
    aws_test.rs
    github_test.rs
    stripe_test.rs
    ... (one per pattern family)
```

Or if the current structure doesn't support per-file tests, create a
single `pattern_tests.rs` with clearly separated test blocks per pattern.

For Case 5, use realistic contexts like:
```python
# Python config
AWS_ACCESS_KEY_ID = "AKIAIOSFODNN7EXAMPLE1"
```
```javascript
// JavaScript .env loading
const apiKey = "ghp_ABCDEFGHIJKLMNOPQRSTUVWXYZabcdef12";
```
```yaml
# Terraform variables
stripe_key: "sk_live_abcdefghijklmnopqrstuvwx"
```
```json
{"datadog_api_key": "abcdef1234567890abcdef1234567890"}
```

These tests serve as regression guards — if a future refactor breaks
pattern matching, these tests catch it immediately.

---

## Task 2: Expanded Synthetic Corpus

Build a larger, more diverse test corpus. Target: 50+ TP, 30+ TN.

### 2a. Harvest competitor test data

Clone these repos (temporary, for test case reference only):

1. Gitleaks:
   git clone --depth 1 https://github.com/gitleaks/gitleaks.git /tmp/gitleaks-ref
   - Look in: cmd/generate/config/rules/ (TOML rule definitions)
   - Each rule has test cases (allowPatterns, test strings)
   - Extract: the test secret strings they use for each pattern type

2. detect-secrets (Yelp):
   git clone --depth 1 https://github.com/Yelp/detect-secrets.git /tmp/detect-secrets-ref
   - Look in: testing/ and tests/
   - Extract: test fixture files with labeled secrets

List what you find. Do NOT copy code — only extract test SECRET STRINGS
and the pattern types they test.

### 2b. Build expanded corpus

Create new test files in coax-benchmarks/datasets/secrets-expanded/

Directory structure:
```
secrets-expanded/
  true-positives/
    # Original 15 files (copy from existing)
    # NEW files by language/format:
    python-config.py          # AWS key in Python code
    javascript-env.js         # GitHub token in JS
    terraform-vars.tf         # Various cloud keys in HCL
    docker-compose.yml        # DB passwords in compose
    dotenv-file.env           # Classic .env with multiple secrets
    kubernetes-secret.yaml    # K8s secret manifest
    github-actions.yml        # Secrets in CI config
    gradle-properties.gradle  # Java build secrets
    npm-rc.npmrc              # npm auth tokens
    php-config.php            # DB credentials in PHP
    ruby-initializer.rb       # API keys in Rails initializer
    shell-script.sh           # Exported env vars with keys
    json-config.json          # Secrets in JSON config
    xml-config.xml            # Secrets in XML/Spring config
    toml-config.toml          # Secrets in TOML
    csv-dump.csv              # Accidental credential dump
    sql-migration.sql         # Hardcoded passwords in DDL
    jupyter-notebook.ipynb    # Keys in notebook cells
    markdown-doc.md           # Keys pasted in documentation
  true-negatives/
    # Original 11 files (copy from existing)
    # NEW files:
    placeholder-config.py     # CHANGE_ME, <your-key-here>, TODO
    hash-values.json          # SHA256 hashes (high entropy, not secrets)
    uuid-config.yaml          # UUIDs everywhere (not secrets)
    base64-content.txt        # Base64 encoded images/data (not secrets)
    encrypted-values.env      # ENC[...] encrypted vault values
    example-docs.md           # Keys with "example" or "sample" in them
    url-parameters.log        # URLs with long query params
    css-hex-colors.css        # #AABBCC hex colors (not secrets)
    jwt-documentation.md      # JWT format explanation (not real tokens)
    minified-code.min.js      # Minified JS (high entropy, not secrets)
    random-test-data.py       # Random string generation code
    package-lock.json         # npm integrity hashes
  ground-truth.yaml           # Labels for ALL files
```

Rules for building test data:
- EVERY secret in TP files should be a REALISTIC format (correct length,
  correct prefix, correct character set) but NEVER a real credential
- Use varied contexts: inline assignment, env vars, config objects,
  function args, string interpolation, comments
- Some files should contain MULTIPLE secrets of different types
- TN files should contain things that LOOK like secrets but aren't:
  hashes, UUIDs, encrypted blobs, placeholder values
- Include at least 3 files with secrets in formats Coax currently misses
  (encoded, multi-line, split across variables) — label these as
  "stretch goal" TPs

### 2c. Ground truth manifest

Create ground-truth.yaml:
```yaml
true_positives:
  python-config.py:
    secrets:
      - type: aws_access_key
        line: 3
        value_prefix: "AKIA..."
      - type: aws_secret_key
        line: 4
        value_prefix: "wJalr..."
    expected_finding_count: 2
  # ... etc

true_negatives:
  hash-values.json:
    description: "SHA256 hashes - high entropy but not secrets"
    expected_finding_count: 0
  # ... etc
```

---

## Task 3: Run Expanded Benchmark

### 3a. Run all three tools on expanded corpus

```bash
# Coax
coax scan --path coax-benchmarks/datasets/secrets-expanded \
  --format json > ~/coaxbenchmarking/results/raw/coax-expanded.json

# Gitleaks
gitleaks detect --source coax-benchmarks/datasets/secrets-expanded \
  --report-format json --report-path ~/coaxbenchmarking/results/raw/gitleaks-expanded.json \
  --no-git

# TruffleHog
trufflehog filesystem coax-benchmarks/datasets/secrets-expanded \
  --json > ~/coaxbenchmarking/results/raw/trufflehog-expanded.json
```

### 3b. Ground truth analysis

Update analyze-ground-truth.py to:
1. Load ground-truth.yaml
2. Match findings to expected secrets (by file + approximate line)
3. Generate the comparison table:
   | Tool | TP | FN | FP | TN | Precision | Recall | F1 |
4. ALSO generate a per-pattern-type breakdown:
   | Pattern Type | Coax | Gitleaks | TruffleHog |
   |---|---|---|---|
   | AWS Keys | ✓/✗ | ✓/✗ | ✓/✗ |
   | GitHub Tokens | ✓/✗ | ✓/✗ | ✓/✗ |
   | ... | | | |

### 3c. Real-world FP test (scale check)

Scan a real, large, open-source repo to check FP rate at scale:

```bash
# Clone a mid-size real repo (no known leaked secrets)
git clone --depth 1 https://github.com/fastify/fastify.git /tmp/fastify

# Scan with all three tools
coax scan --path /tmp/fastify --format json > ~/coaxbenchmarking/results/raw/coax-fastify.json
gitleaks detect --source /tmp/fastify --report-format json \
  --report-path ~/coaxbenchmarking/results/raw/gitleaks-fastify.json
trufflehog filesystem /tmp/fastify \
  --json > ~/coaxbenchmarking/results/raw/trufflehog-fastify.json

# Count findings per tool
echo "Coax: $(cat ~/coaxbenchmarking/results/raw/coax-fastify.json | python3 -c 'import json,sys; print(len(json.load(sys.stdin)))')"
echo "Gitleaks: $(cat ~/coaxbenchmarking/results/raw/gitleaks-fastify.json | python3 -c 'import json,sys; print(len(json.load(sys.stdin)))')"
echo "TruffleHog: $(wc -l < ~/coaxbenchmarking/results/raw/trufflehog-fastify.json)"
```

If Coax produces significantly more findings than Gitleaks on a clean
repo, those are likely FPs and need investigation.

Also scan one more repo for diversity:
```bash
git clone --depth 1 https://github.com/expressjs/express.git /tmp/express
# Same scan commands as above
```

Report findings counts for all tools on both repos.

---

## Task 4: Report

Create ~/coaxbenchmarking/results/EXPANDED-BENCHMARK-REPORT.md with:

1. **Original benchmark results** (for reference)
2. **Expanded benchmark results** (the new table)
3. **Per-pattern breakdown** (which patterns each tool catches)
4. **Real-world FP counts** (findings on clean repos)
5. **Analysis**: Where does Coax win? Where does it lose? What needs
   fixing before these numbers are publishable?

## Rules
- Do NOT tune the scanner to pass specific test cases during this task.
  This is a measurement exercise, not a fixing exercise.
- Report results honestly, including any regressions
- If a tool errors on any file, note the error, don't skip the file
- All results saved to ~/coaxbenchmarking/
- Commit test corpus: "test: expanded benchmark corpus (50+ TP, 30+ TN)"
```

---

## Why This Matters

The current F1 0.81 is real, but it's validated on a corpus small enough to memorize. Before you can put benchmark numbers in a README, on a website, or in front of a potential user, you need to know:

1. **Does it generalize?** — Do patterns work in `.py`, `.js`, `.yaml`, not just `.txt`?
2. **Does it scale?** — Is the FP rate acceptable on a 10,000-file repo?
3. **Is it honest?** — Would a skeptical security engineer reproduce these numbers?

If Coax holds F1 0.75+ on the expanded corpus and shows comparable or lower FP counts on real repos, that's a publishable, credible benchmark. If it drops to 0.50, you know exactly what to fix before going public.