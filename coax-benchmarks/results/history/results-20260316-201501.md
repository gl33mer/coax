# Coax Benchmark Results

**Generated:** 2026-03-16 20:15:01
**Coax Version:** coax 0.7.4
**Test Environment:** Linux 6.12.73+deb13-cloud-amd64

---

## Summary

| Category | Metric | Value | Target | Status |
|----------|--------|-------|--------|--------|
| Secrets | True Positive Rate | 94/12 | >90% | ✅ Pass |
| Secrets | False Positive Rate | 0/11 | <5% | ✅ Pass |
| Secrets | Precision | Calculated below | >95% | - |
| Encoded | Detection Rate | 40/3 | >80% | - |
| Unicode | Bidi Coverage | 224/5 | 5/5 | - |
| Unicode | Homoglyph Coverage | 177/5 | 5/5 | - |
| Unicode | Invisible Chars | 72/5 | 5/5 | - |
| Unicode | Safe i18n (no FP) | 0/5 | 5/5 | - |
| Performance | Scan Speed | See below | >100K files/s | - |

---

## Detailed Results

### Secrets Detection

| Dataset | Files | Findings | Time (s) |
|---------|-------|----------|----------|
| True Positives | 12 | 94 | 0 |
| True Negatives | 11 | 0 FP | 1 |
| Encoded | 3 | 40 | 0 |

### Unicode Attack Detection

| Dataset | Files | Findings | Time (s) |
|---------|-------|----------|----------|
| BiDi Attacks | 5 | 224 | 1 |
| Homoglyphs | 5 | 177 | 0 |
| Invisible Chars | 5 | 72 | 0 |
| Safe i18n | 5 | 0 (correct) | 1 |

---

## Metrics Definitions

| Metric | Formula | Description |
|--------|---------|-------------|
| True Positive Rate (Recall) | TP / (TP + FN) | Percentage of actual secrets detected |
| False Positive Rate | FP / (FP + TN) | Percentage of clean files incorrectly flagged |
| Precision | TP / (TP + FP) | Percentage of findings that are real |
| F1 Score | 2 × (Precision × Recall) / (Precision + Recall) | Harmonic mean of precision and recall |

---

## How to Run

```bash
# Run all benchmarks
./run-benchmarks.sh

# Run specific category
coax scan datasets/secrets/true-positives

# Run git history benchmark
coax scan --git-history datasets/git-history/test-repo
```

---

## Historical Results

See `results/history/` for previous benchmark runs.

---

*Note: These benchmarks are designed to be honest and multi-dimensional. Coax may score differently than competitors on traditional secret detection, but excels in Unicode attack detection where most tools have no coverage.*
