# Coax Benchmark Methodology

## Overview
This document describes benchmark methodology for Coax scanner.

## Benchmark Categories

### 1. Speed Benchmarks

#### Small Repository (~100 files)
```bash
time coax scan -p small-repo --format json > /dev/null
```
**Target:** <100ms

#### Medium Repository (~1000 files)
```bash
time coax scan -p medium-repo --format json > /dev/null
```
**Target:** <500ms

#### Large Repository (~10000 files)
```bash
time coax scan -p large-repo --format json > /dev/null
```
**Target:** <5s

### 2. Memory Benchmarks

#### Peak Memory Usage
```bash
/usr/bin/time -v coax scan -p medium-repo 2>&1 | grep "Maximum resident"
```
**Target:** <100MB

#### Memory Per File
```bash
# Run with different repo sizes
# Plot memory vs file count
```
**Target:** <10KB per file

### 3. Accuracy Benchmarks

#### Precision
```
Precision = TP / (TP + FP)
```
**Target:** >95%

#### Recall
```
Recall = TP / (TP + FN)
```
**Target:** >95%

#### F1 Score
```
F1 = 2 * (Precision * Recall) / (Precision + Recall)
```
**Target:** >95%

### 4. Comparison Benchmarks

#### vs TruffleHog
```bash
# Same repo, both tools
# Compare: speed, FP rate, detection rate
```

#### vs Gitleaks
```bash
# Same repo, both tools
# Compare: speed, FP rate, detection rate
```

## Benchmark Automation

### GitHub Actions
```yaml
name: Benchmarks
on: [push, pull_request]
jobs:
  benchmark:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run benchmarks
        run: cargo bench
      - name: Upload results
        uses: benchmark-action/github-action-benchmark@v1
```

## Reporting

### Benchmark Report Structure
1. Executive Summary
2. Test Environment
3. Speed Results
4. Memory Results
5. Accuracy Results
6. Comparison Results
7. Trends (if historical data)
8. Recommendations

---

## Detailed Benchmark Procedures

### Environment Setup

Before running benchmarks, ensure consistent environment:

```bash
# Close unnecessary applications
# Use dedicated benchmark machine if possible
# Record system specs:

# CPU
lscpu | grep "Model name"

# Memory
free -h

# OS
uname -a

# Rust version
rustc --version

# Coax version
coax --version
```

### Speed Benchmark Script

```bash
#!/bin/bash
# benchmark_speed.sh

REPOS=(
    "small-repo"    # ~100 files
    "medium-repo"   # ~1000 files
    "large-repo"    # ~10000 files
)

for repo in "${REPOS[@]}"; do
    echo "Benchmarking $repo..."
    
    # Run 3 times and take average
    total=0
    for i in {1..3}; do
        # Clear caches
        sync && echo 3 | sudo tee /proc/sys/vm/drop_caches
        
        # Measure time
        start=$(date +%s%N)
        coax scan -p "$repo" --format json > /dev/null
        end=$(date +%s%N)
        
        elapsed=$(( (end - start) / 1000000 ))  # Convert to ms
        total=$((total + elapsed))
        echo "  Run $i: ${elapsed}ms"
    done
    
    avg=$((total / 3))
    echo "  Average: ${avg}ms"
    echo ""
done
```

### Memory Benchmark Script

```bash
#!/bin/bash
# benchmark_memory.sh

REPOS=(
    "small-repo"
    "medium-repo"
    "large-repo"
)

for repo in "${REPOS[@]}"; do
    echo "Memory benchmark: $repo..."
    
    /usr/bin/time -v coax scan -p "$repo" --format json > /dev/null 2>&1 | \
        grep -E "(Maximum resident|Elapsed|User time|System time)"
    echo ""
done
```

### Accuracy Benchmark Procedure

1. **Prepare Test Corpus**
   - Create seeded repositories with known secrets
   - Include various secret types (AWS, GitHub, Stripe, etc.)
   - Include clean repositories for FP testing

2. **Run Scanner**
   ```bash
   coax scan -p test-corpus --format json > coax_results.json
   ```

3. **Categorize Results**
   - True Positive (TP): Correctly identified secret
   - False Positive (FP): Incorrectly flagged as secret
   - False Negative (FN): Missed secret (from seeded list)

4. **Calculate Metrics**
   ```python
   precision = TP / (TP + FP)
   recall = TP / (TP + FN)
   f1 = 2 * (precision * recall) / (precision + recall)
   ```

### Comparison Benchmark Script

```bash
#!/bin/bash
# benchmark_comparison.sh

REPO="test-repo"

echo "=== Coax ==="
time coax scan -p "$REPO" --format json > coax_results.json

echo ""
echo "=== TruffleHog ==="
time trufflehog filesystem "$REPO" --json > trufflehog_results.json

echo ""
echo "=== Gitleaks ==="
time gitleaks detect --source "$REPO" --report-path gitleaks_results.json

# Compare results
echo ""
echo "=== Comparison ==="
echo "Coax findings:      $(jq length coax_results.json)"
echo "TruffleHog findings: $(jq length trufflehog_results.json)"
echo "Gitleaks findings:   $(jq length gitleaks_results.json)"
```

## Benchmark Results Template

```markdown
# Benchmark Results - [Date]

## Environment
- CPU: [CPU Model]
- Memory: [Total RAM]
- OS: [OS Version]
- Rust: [Version]
- Coax: [Version]

## Speed Results

| Repository | Files | Size | Time | Files/sec |
|------------|-------|------|------|-----------|
| Small      | 100   | 1MB  | 50ms | 2000      |
| Medium     | 1000  | 10MB | 400ms| 2500      |
| Large      | 10000 | 100MB| 4.5s | 2222      |

## Memory Results

| Repository | Peak Memory | Memory/File |
|------------|-------------|-------------|
| Small      | 30MB        | 300KB       |
| Medium     | 50MB        | 50KB        |
| Large      | 80MB        | 8KB         |

## Accuracy Results

| Metric | Score | Target | Status |
|--------|-------|--------|--------|
| Precision | 96% | >95% | ✅ |
| Recall | 97% | >95% | ✅ |
| F1 Score | 96.5% | >95% | ✅ |

## Comparison Results

| Tool | Speed (1000 files) | Memory | FP Rate | Detection |
|------|-------------------|--------|---------|-----------|
| Coax | 400ms | 50MB | 3% | 97% |
| TruffleHog | 2s | 200MB | 5% | 95% |
| Gitleaks | 1s | 100MB | 8% | 92% |

## Trends

[Include graphs showing performance over time if available]

## Recommendations

1. [Action items based on results]
2. [Optimization opportunities]
3. [Areas for improvement]
```

## Continuous Benchmarking

### Automated Benchmarks

Add to CI/CD pipeline:

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
        uses: dtolnay/rust-action@stable
      
      - name: Build release
        run: cargo build --release
      
      - name: Run speed benchmarks
        run: ./scripts/benchmark_speed.sh
      
      - name: Run memory benchmarks
        run: ./scripts/benchmark_memory.sh
      
      - name: Upload results
        uses: benchmark-action/github-action-benchmark@v1
        with:
          tool: 'cargo'
          output-file-path: benchmark_results.json
          github-token: ${{ secrets.GITHUB_TOKEN }}
          auto-push: true
```

### Performance Regression Detection

Alert if:
- Speed decreases by >10%
- Memory increases by >15%
- Detection rate decreases by >2%
- FP rate increases by >3%
