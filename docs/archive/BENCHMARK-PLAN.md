# Phase 3 Benchmark Plan

**Date:** 2026-03-15
**Status:** Proposal
**Owner:** DevShield Performance Team

---

## Executive Summary

This document defines a comprehensive benchmark suite for measuring DevShield's performance against state-of-the-art security scanners. We will measure **speed**, **memory**, **accuracy**, and **false positive rate** across multiple scenarios.

**Benchmark Goals:**
1. Establish baseline performance metrics
2. Compare against TruffleHog, Gitleaks, GitGuardian
3. Identify performance bottlenecks
4. Track improvements over time
5. Validate Phase 3 feature impact

---

## 1. Speed Benchmarks

### 1.1 Test Scenarios

| Scenario | Files | Description | Target |
|----------|-------|-------------|--------|
| **Small repo** | ~100 | Typical library/package | <100ms |
| **Medium repo** | ~1,000 | Standard application | <500ms |
| **Large repo** | ~10,000 | Monorepo/large project | <5s |
| **Extra large** | ~100,000 | Enterprise codebase | <30s |

### 1.2 Test Repositories

| Name | Files | Language | Type |
|------|-------|----------|------|
| serde | 150 | Rust | Library |
| requests | 200 | Python | Library |
| express | 500 | JavaScript | Framework |
| kubernetes | 15,000 | Go | System |
| linux (subset) | 50,000 | C | Kernel |

### 1.3 Benchmark Commands

```bash
# DevShield
hyperfine --warmup 3 \
  "devshield scan secrets --path ./test-repos/serde" \
  "devshield scan secrets --path ./test-repos/kubernetes"

# TruffleHog
hyperfine --warmup 3 \
  "trufflehog filesystem ./test-repos/serde" \
  "trufflehog filesystem ./test-repos/kubernetes"

# Gitleaks
hyperfine --warmup 3 \
  "gitleaks dir ./test-repos/serde" \
  "gitleaks dir ./test-repos/kubernetes"
```

### 1.4 Metrics to Collect

| Metric | Unit | Tool |
|--------|------|------|
| **Mean time** | milliseconds | hyperfine |
| **Std deviation** | milliseconds | hyperfine |
| **Min/Max time** | milliseconds | hyperfine |
| **Files per second** | files/s | Calculated |
| **Lines per second** | lines/s | Calculated |

### 1.5 Expected Results

| Tool | 100 files | 1,000 files | 10,000 files |
|------|-----------|-------------|--------------|
| **DevShield** | <100ms | <500ms | <5s |
| **TruffleHog** | ~200ms | ~2s | ~20s |
| **Gitleaks** | ~150ms | ~1.5s | ~15s |
| **GitGuardian** | ~300ms | ~3s | ~30s |

**Note:** TruffleHog slower due to verification. DevShield should be fastest without verification.

---

## 2. Memory Benchmarks

### 2.1 Test Scenarios

| Scenario | Description | Target |
|----------|-------------|--------|
| **Peak memory** | Maximum RSS during scan | <100MB |
| **Memory per file** | Average memory per file scanned | <10KB |
| **Memory per finding** | Memory allocated per finding | <1KB |
| **Baseline memory** | Idle memory usage | <20MB |

### 2.2 Measurement Tools

```bash
# Linux
/usr/bin/time -v devshield scan secrets --path ./test-repos

# macOS
/usr/bin/time -l devshield scan secrets --path ./test-repos

# Cross-platform (Rust)
cargo install heaptrack
heaptrack devshield scan secrets --path ./test-repos
```

### 2.3 Metrics to Collect

| Metric | Unit | Measurement |
|--------|------|-------------|
| **Peak RSS** | MB | /usr/bin/time |
| **Max heap** | MB | heaptrack |
| **Allocations** | count | heaptrack |
| **GC pressure** | N/A | N/A (Rust has no GC) |

### 2.4 Expected Results

| Tool | Peak RSS | Memory/File | Allocations |
|------|----------|-------------|-------------|
| **DevShield** | <50MB | <5KB | <10,000 |
| **TruffleHog** | <100MB | <10KB | <50,000 |
| **Gitleaks** | <80MB | <8KB | <30,000 |
| **detect-secrets** | <150MB | <15KB | <100,000 |

---

## 3. Accuracy Benchmarks

### 3.1 Test Datasets

#### 3.1.1 TruffleHog Test Corpus

**Source:** https://github.com/trufflesecurity/test_keys

| File | Secrets | Types |
|------|---------|-------|
| test_keys.py | 50+ | AWS, GitHub, Stripe, etc. |
| test_keys.js | 30+ | API keys, tokens |
| test_keys.go | 20+ | Go-specific secrets |

**Expected:** Detect 100% of seeded secrets

#### 3.1.2 Gitleaks Test Corpus

**Source:** https://github.com/gitleaks/gitleaks/tree/master/testdata

| Directory | Secrets | Purpose |
|-----------|---------|---------|
| configs/ | 100+ | Configuration files |
| repos/ | 200+ | Git repositories |
| baseline/ | 50+ | Baseline testing |

**Expected:** Detect 95%+ of seeded secrets

#### 3.1.3 Custom Seeded Repositories

Create repositories with known secrets:

```bash
# Create test repository
mkdir test-seeded-repo
cd test-seeded-repo
git init

# Seed secrets (use fake values)
echo "AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE" > config.env
echo "GITHUB_TOKEN=ghp_1234567890abcdefghij1234567890abcdefghij" >> config.env
echo "STRIPE_KEY=sk_live_1234567890abcdefghij123456" >> config.env

# Add clean files
echo "console.log('hello')" > app.js
echo "def main(): pass" > app.py

git add .
git commit -m "Initial commit"
```

**Seeding Script:**
```python
#!/usr/bin/env python3
# scripts/seed-secrets.py

import os
import random

SECRET_TEMPLATES = [
    ("AWS", "AKIA{random:16}"),
    ("GitHub", "ghp_{random:40}"),
    ("Stripe", "sk_live_{random:24}"),
    ("Google", "AIza{random:35}"),
    ("Slack", "xoxb-{random:12}-{random:12}-{random:24}"),
]

def generate_secret(template):
    # Generate fake secret from template
    pass

def seed_repository(repo_path, num_secrets=50):
    # Seed repository with known secrets
    pass
```

### 3.2 Accuracy Metrics

| Metric | Formula | Target |
|--------|---------|--------|
| **True Positives (TP)** | Correctly identified secrets | Maximize |
| **False Positives (FP)** | Clean code flagged as secret | Minimize |
| **False Negatives (FN)** | Missed secrets | Minimize |
| **Precision** | TP / (TP + FP) | >95% |
| **Recall** | TP / (TP + FN) | >95% |
| **F1 Score** | 2 * (Precision * Recall) / (Precision + Recall) | >95% |

### 3.3 Benchmark Commands

```bash
# Run accuracy benchmark
cargo run --bin benchmark-accuracy -- \
  --dataset trufflehog-test-keys \
  --output results.json

# Compare tools
python scripts/compare-accuracy.py \
  --devshield results-devshield.json \
  --trufflehog results-trufflehog.json \
  --gitleaks results-gitleaks.json
```

### 3.4 Expected Results

| Tool | Precision | Recall | F1 Score |
|------|-----------|--------|----------|
| **DevShield (no verify)** | 85% | 95% | 90% |
| **DevShield (verified)** | 99% | 95% | 97% |
| **TruffleHog (verified)** | 99% | 98% | 98.5% |
| **Gitleaks** | 90% | 95% | 92.5% |
| **GitGuardian** | 98% | 97% | 97.5% |

---

## 4. False Positive Benchmarks

### 4.1 Test Repositories (Clean)

| Repository | Files | Language | Why Clean |
|------------|-------|----------|-----------|
| **Linux kernel** | 100,000+ | C | Heavily audited |
| **Rust compiler** | 50,000+ | Rust | Security-conscious |
| **Go standard lib** | 20,000+ | Go | No secrets expected |
| **Python stdlib** | 30,000+ | Python | No secrets expected |
| **React** | 5,000+ | JavaScript | Well-maintained |

### 4.2 False Positive Rate

| Metric | Formula | Target |
|--------|---------|--------|
| **FP per 1000 files** | (FP / Total Files) * 1000 | <10 |
| **FP percentage** | (FP / Total Findings) * 100% | <5% |
| **FP per line** | FP / Total Lines | <0.001% |

### 4.3 Benchmark Commands

```bash
# Scan clean repositories
for repo in linux rust go python react; do
    echo "Scanning $repo..."
    time devshield scan secrets --path ./clean-repos/$repo \
        --format json \
        --output results-$repo.json
done

# Calculate FP rate
python scripts/calculate-fp-rate.py \
    --results results-*.json \
    --expected 0
```

### 4.4 Expected Results

| Tool | FP/1000 files | FP % | Notes |
|------|---------------|------|-------|
| **DevShield (no verify)** | 50 | 10% | Entropy causes FPs |
| **DevShield (verified)** | 5 | 1% | Verification filters |
| **TruffleHog (verified)** | 2 | 0.5% | Best in class |
| **Gitleaks** | 30 | 5% | Good pattern matching |
| **detect-secrets** | 40 | 8% | High entropy sensitivity |

---

## 5. Feature-Specific Benchmarks

### 5.1 Live Verification

| Metric | Baseline | With Verification | Delta |
|--------|----------|-------------------|-------|
| **Scan time** | 100ms | 500ms | +400ms |
| **FP rate** | 10% | 1% | -9% |
| **API calls** | 0 | 50 | +50 |
| **Network latency** | 0ms | 300ms | +300ms |

**Benchmark:**
```bash
# Without verification
hyperfine "devshield scan --no-verify --path ./test-repo"

# With verification
hyperfine "devshield scan --verify --path ./test-repo"
```

### 5.2 Baseline Files

| Metric | Full Scan | Baseline Scan | Delta |
|--------|-----------|---------------|-------|
| **Scan time** | 500ms | 100ms | -80% |
| **Findings** | 100 | 5 (new only) | -95% |
| **Memory** | 50MB | 30MB | -40% |

**Benchmark:**
```bash
# Create baseline
devshield baseline create --path ./repo --output .baseline.json

# Full scan
hyperfine "devshield scan --path ./repo"

# Baseline scan
hyperfine "devshield scan --path ./repo --baseline .baseline.json"
```

### 5.3 Encoded Secret Detection

| Metric | Without Decode | With Decode | Delta |
|--------|---------------|-------------|-------|
| **Scan time** | 100ms | 120ms | +20% |
| **Detection** | 50 secrets | 60 secrets | +20% |
| **Memory** | 30MB | 35MB | +17% |

### 5.4 Archive Scanning

| Metric | Without Archives | With Archives | Delta |
|--------|-----------------|---------------|-------|
| **Scan time** | 100ms | 300ms | +200% |
| **Detection** | 50 secrets | 55 secrets | +10% |
| **Memory** | 30MB | 50MB | +67% |

---

## 6. Continuous Benchmarking

### 6.1 CI/CD Integration

```yaml
# .github/workflows/benchmarks.yml
name: Benchmarks

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run benchmarks
        run: cargo bench --bench speed --bench memory --bench accuracy
      
      - name: Upload results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: target/criterion
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
```

### 6.2 Benchmark Dashboard

**Tools:**
- GitHub Actions Benchmark (github-action-benchmark)
- CodSpeed (codspeed.io)
- Custom Grafana dashboard

**Metrics to Track:**
- Scan time (by repo size)
- Memory usage (peak, average)
- Detection rate (by secret type)
- False positive rate (over time)

### 6.3 Performance Budget

| Metric | Budget | Alert Threshold |
|--------|--------|-----------------|
| **Scan time (100 files)** | <100ms | >150ms |
| **Scan time (1000 files)** | <500ms | >750ms |
| **Scan time (10000 files)** | <5s | >7.5s |
| **Peak memory** | <100MB | >150MB |
| **FP rate** | <5% | >10% |
| **Detection rate** | >95% | <90% |

---

## 7. Benchmark Implementation

### 7.1 Benchmark Suite Structure

```
benches/
├── speed/
│   ├── small_repo.rs
│   ├── medium_repo.rs
│   └── large_repo.rs
├── memory/
│   ├── peak_memory.rs
│   └── memory_per_file.rs
├── accuracy/
│   ├── trufflehog_corpus.rs
│   ├── gitleaks_corpus.rs
│   └── seeded_repos.rs
└── false_positives/
    ├── clean_repos.rs
    └── fp_rate.rs
```

### 7.2 Example Benchmark

```rust
// benches/speed/small_repo.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use devshield::{Scanner, Config};

fn benchmark_small_repo(c: &mut Criterion) {
    let config = Config::default();
    let scanner = Scanner::new(config);
    
    c.bench_function("scan_small_repo", |b| {
        b.iter(|| {
            scanner.scan(black_box("./test-repos/serde"))
        })
    });
}

criterion_group!(benches, benchmark_small_repo);
criterion_main!(benches);
```

### 7.3 Comparison Script

```python
# scripts/compare-tools.py

import json
import subprocess
import tempfile
from pathlib import Path

TOOLS = {
    "devshield": ["devshield", "scan", "secrets", "--path"],
    "trufflehog": ["trufflehog", "filesystem"],
    "gitleaks": ["gitleaks", "dir"],
}

def run_benchmark(tool, repo_path):
    cmd = TOOLS[tool] + [repo_path]
    result = subprocess.run(cmd, capture_output=True, time=True)
    return {
        "time_ms": result.elapsed.total_seconds() * 1000,
        "findings": len(json.loads(result.stdout)),
        "memory_mb": get_memory_usage(result.pid),
    }

def compare_all_tools(repo_path):
    results = {}
    for tool in TOOLS:
        print(f"Running {tool}...")
        results[tool] = run_benchmark(tool, repo_path)
    
    # Generate comparison report
    generate_report(results)

if __name__ == "__main__":
    compare_all_tools("./test-repos/serde")
```

---

## 8. Success Criteria

### 8.1 Phase 3 Targets

| Metric | Current | Phase 3 Target | Industry Leader |
|--------|---------|----------------|-----------------|
| **Speed (100 files)** | ~50ms | <100ms | Gitleaks: ~150ms |
| **Speed (1000 files)** | ~500ms | <500ms | Gitleaks: ~1.5s |
| **Speed (10000 files)** | ~5s | <5s | Gitleaks: ~15s |
| **Memory (peak)** | ~30MB | <100MB | TruffleHog: ~100MB |
| **Detection rate** | ~90% | >95% | TruffleHog: ~98% |
| **FP rate (no verify)** | ~15% | <10% | Gitleaks: ~5% |
| **FP rate (verified)** | N/A | <1% | TruffleHog: ~0.5% |

### 8.2 Pass/Fail Criteria

**Phase 3 is successful if:**

✅ Speed: Within 2x of fastest tool (Gitleaks)
✅ Memory: <100MB peak for all scenarios
✅ Detection: >95% on test corpora
✅ FP rate: <10% without verification, <1% with verification
✅ SARIF: Valid output accepted by GitHub
✅ Pre-commit: <3s for normal commits

---

## 9. Timeline

| Week | Task | Deliverable |
|------|------|-------------|
| 1 | Set up benchmark infrastructure | Benchmark suite skeleton |
| 2 | Implement speed benchmarks | Speed benchmark results |
| 3 | Implement memory benchmarks | Memory benchmark results |
| 4 | Implement accuracy benchmarks | Accuracy comparison report |
| 5 | Implement FP benchmarks | FP rate analysis |
| 6 | CI/CD integration | Automated benchmark pipeline |
| 7 | Dashboard setup | Performance dashboard |
| 8 | Baseline measurements | Phase 2 baseline report |
| 9-12 | Continuous measurement | Weekly benchmark reports |

---

## 10. References

- Criterion.rs: https://github.com/bheisler/criterion.rs
- GitHub Action Benchmark: https://github.com/benchmark-action/github-action-benchmark
- TruffleHog test corpus: https://github.com/trufflesecurity/test_keys
- Gitleaks testdata: https://github.com/gitleaks/gitleaks/tree/master/testdata
- CodSpeed: https://codspeed.io

---

*Document created: 2026-03-15*
*Next: QA Plan (docs/QA-PLAN.md)*
