# Coax QA Methodology

## Overview
This document describes the QA methodology for Coax scanner testing.

## Test Repository Selection

### Criteria
- Real-world codebases (not synthetic)
- Mix of languages (Rust, Python, JS, TS, Go)
- Various sizes (small <100 files, medium 100-1000, large >1000)
- Known secrets (for validation)
- Clean repos (for FP rate)

### Selected Repositories
| Repository | Files | Language | Purpose |
|------------|-------|----------|---------|
| serde-rs/serde | 376 | Rust | Small clean repo |
| psf/requests | 159 | Python | Small clean repo |
| expressjs/express | 242 | JS | Medium web framework |
| pallets/flask | 265 | Python | Medium web framework |
| kubernetes/kubernetes | 28K+ | Go | Large codebase |

## Test Scenarios

### 1. Clean Repository Scan
**Purpose:** Measure false positive rate
**Expected:** 0 findings (or very few)
**Metric:** FP per 1000 files

**Execution:**
```bash
coax scan -p clean-repo --format json > results.json
# Manually review findings
# Calculate: FP rate = (False Positives / Total Findings) * 100
```

### 2. Seeded Repository Scan
**Purpose:** Measure detection rate
**Expected:** All seeded secrets detected
**Metric:** Detection rate %

**Execution:**
```bash
# Create test repo with known secrets
mkdir test-seeded
cd test-seeded
echo "AWS_KEY=AKIAIOSFODNN7EXAMPLE" > config.txt
echo "GITHUB_TOKEN=ghp_1234567890abcdefghij1234567890abcdefghij" > .env

coax scan -p . --format json > results.json
# Verify all seeded secrets are detected
# Calculate: Detection Rate = (Detected / Total Seeded) * 100
```

### 3. Real-World Repository Scan
**Purpose:** Measure real-world performance
**Expected:** Unknown findings
**Metric:** Manual review required

**Execution:**
```bash
coax scan -p real-project --format json > results.json
# Categorize each finding as:
# - True Positive (TP): Actual secret
# - False Positive (FP): Not a secret
# - Uncertain: Needs more investigation
```

### 4. Performance Benchmark
**Purpose:** Measure scan speed
**Expected:** <1s per 100 files
**Metric:** Files/second, MB/second

**Execution:**
```bash
time coax scan -p test-repo --format json > /dev/null
# Record: real, user, sys time
# Calculate: files/second = total_files / real_time
```

### 5. Memory Benchmark
**Purpose:** Measure memory usage
**Expected:** <100MB peak
**Metric:** Peak RSS

**Execution:**
```bash
/usr/bin/time -v coax scan -p test-repo 2>&1 | grep "Maximum resident"
# Record peak memory in KB
```

## Success Criteria

| Metric | Target | Measurement |
|--------|--------|-------------|
| Detection Rate | >95% | Seeded repos |
| False Positive Rate | <5% | Clean repos |
| Scan Speed | <1s/100 files | Performance benchmark |
| Memory Usage | <100MB | Memory benchmark |
| Test Coverage | >80% | cargo tarpaulin |

## Test Execution

### Automated Tests
```bash
cargo test --workspace
```

### Manual QA
1. Clone test repositories
2. Run coax scan
3. Review findings manually
4. Categorize as TP/FP
5. Calculate metrics

### QA Checklist

- [ ] Clean repo scan completed
- [ ] Seeded repo scan completed
- [ ] Real-world repo scan completed
- [ ] Performance benchmarks recorded
- [ ] Memory benchmarks recorded
- [ ] All findings categorized
- [ ] FP analysis completed
- [ ] TP verification completed

## Reporting

### QA Report Structure
1. Executive Summary
2. Test Repositories
3. Detection Results
4. False Positive Analysis
5. Performance Metrics
6. Issues Found
7. Recommendations

### QA Report Template

```markdown
# QA Report - [Date]

## Executive Summary
- Total repos tested: X
- Detection rate: X%
- FP rate: X%
- Performance: X files/sec

## Test Repositories
| Repo | Files | Language | Findings |
|------|-------|----------|----------|
| ...  | ...   | ...      | ...      |

## Detection Results
- Seeded secrets: X/Y detected (Z%)
- Clean repo findings: X (all FPs)

## False Positive Analysis
| Pattern | FP Count | Root Cause |
|---------|----------|------------|
| ...     | ...      | ...        |

## Performance Metrics
| Repo Size | Time | Files/sec |
|-----------|------|-----------|
| Small     | ...  | ...       |
| Medium    | ...  | ...       |
| Large     | ...  | ...       |

## Recommendations
1. ...
2. ...
```

## Continuous QA

### Pre-Release QA
Before each release:
1. Run full test suite
2. Run benchmarks
3. Manual review of sample findings
4. Update QA report

### Regression Testing
When fixing FPs:
1. Add FP case to test corpus
2. Verify fix reduces FP
3. Verify no new FPs introduced
4. Verify detection rate unchanged

## Appendix: Test Secret Patterns

### AWS Keys
```
AKIAIOSFODNN7EXAMPLE
```

### GitHub Tokens
```
ghp_1234567890abcdefghij1234567890abcdefghij
```

### Stripe Keys
```
sk_live_1234567890abcdefghij1234
```

### Generic High-Entropy
```
aGVsbG8gd29ybGQgdGhpcyBpcyBhIHRlc3Qgc3RyaW5n
```
