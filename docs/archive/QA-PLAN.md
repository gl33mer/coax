# Phase 3 QA Test Plan

**Date:** 2026-03-15
**Status:** Proposal
**Owner:** DevShield QA Team

---

## Executive Summary

This document defines a comprehensive QA test plan for Phase 3 features. We will test across **multiple repository sizes**, **various languages**, and **diverse scenarios** to ensure DevShield meets quality standards.

**QA Goals:**
1. Validate detection accuracy (>95%)
2. Measure false positive rate (<5%)
3. Verify performance targets (<1s per 100 files)
4. Test edge cases (binary files, huge files, etc.)
5. Validate Phase 3 features (verification, baseline, SARIF, etc.)

---

## 1. Test Repository Selection

### 1.1 Small Repositories (<100 files)

| Repository | Files | Language | Purpose | Secrets Expected |
|------------|-------|----------|---------|------------------|
| **serde** | ~50 | Rust | Serialization library | 0 (clean) |
| **left-pad** | ~10 | JavaScript | Utility library | 0 (clean) |
| **requests** | ~80 | Python | HTTP library | 0 (clean) |
| **test-seeded-small** | ~50 | Mixed | Seeded test repo | 10 (known) |
| **config-examples** | ~30 | YAML/JSON | Config files | 5 (known) |

**Selection Criteria:**
- Well-maintained projects
- Multiple languages
- Clean history (no accidental commits)
- Representative of typical libraries

### 1.2 Medium Repositories (100-1000 files)

| Repository | Files | Language | Purpose | Secrets Expected |
|------------|-------|----------|---------|------------------|
| **express** | ~500 | JavaScript | Web framework | 0 (clean) |
| **flask** | ~300 | Python | Web framework | 0 (clean) |
| **actix-web** | ~600 | Rust | Web framework | 0 (clean) |
| **test-seeded-medium** | ~500 | Mixed | Seeded test repo | 50 (known) |
| **demo-app** | ~400 | TypeScript | Full-stack app | 0 (clean) |

**Selection Criteria:**
- Real-world applications
- Multiple components
- Test files included
- Configuration files

### 1.3 Large Repositories (>1000 files)

| Repository | Files | Language | Purpose | Secrets Expected |
|------------|-------|----------|---------|------------------|
| **kubernetes** | ~15,000 | Go | Container orchestration | 0 (clean) |
| **tensorflow** | ~50,000 | C++/Python | ML framework | 0 (clean) |
| **vscode** | ~20,000 | TypeScript | Code editor | 0 (clean) |
| **test-seeded-large** | ~5,000 | Mixed | Seeded test repo | 200 (known) |
| **monorepo-example** | ~10,000 | Multi | Monorepo structure | 0 (clean) |

**Selection Criteria:**
- Enterprise-scale projects
- Multiple languages
- Complex directory structures
- Build artifacts (to be skipped)

### 1.4 Edge Case Repositories

| Repository | Description | Purpose |
|------------|-------------|---------|
| **binary-files** | Contains PDFs, images, executables | Test binary handling |
| **huge-files** | Contains files >100MB | Test size limits |
| **deep-nesting** | 100+ directory levels | Test recursion limits |
| **symlinks** | Circular and deep symlinks | Test symlink handling |
| **special-chars** | Files with unicode, spaces | Test path handling |
| **encoded-secrets** | Base64/hex/percent-encoded secrets | Test decoding |
| **archive-nested** | ZIP/TAR with nested archives | Test archive scanning |

---

## 2. Test Scenarios

### 2.1 Clean Repository Scans

**Objective:** Verify no false positives on clean repositories

**Test Cases:**

| ID | Repository | Expected Findings | Pass Criteria |
|----|------------|-------------------|---------------|
| TC-001 | serde | 0 | 0 findings |
| TC-002 | left-pad | 0 | 0 findings |
| TC-003 | requests | 0 | 0 findings |
| TC-004 | express | 0 | 0 findings |
| TC-005 | flask | 0 | 0 findings |
| TC-006 | kubernetes (subset) | 0 | 0 findings |
| TC-007 | tensorflow (subset) | 0 | 0 findings |

**Execution:**
```bash
for repo in serde left-pad requests express flask; do
    echo "Testing $repo..."
    result=$(devshield scan secrets --path ./test-repos/$repo --format json)
    findings=$(echo "$result" | jq '.findings | length')
    
    if [ "$findings" -eq 0 ]; then
        echo "✅ PASS: $repo"
    else
        echo "❌ FAIL: $repo ($findings findings)"
    fi
done
```

### 2.2 Seeded Repository Scans

**Objective:** Verify detection of known secrets

**Test Cases:**

| ID | Repository | Expected | Minimum Detection | Pass Criteria |
|----|------------|----------|-------------------|---------------|
| TC-010 | test-seeded-small | 10 | 9 (90%) | ≥9 findings |
| TC-011 | test-seeded-medium | 50 | 45 (90%) | ≥45 findings |
| TC-012 | test-seeded-large | 200 | 180 (90%) | ≥180 findings |
| TC-013 | config-examples | 5 | 5 (100%) | ≥5 findings |

**Seeded Secret Types:**
```
AWS Access Keys: 10
AWS Secret Keys: 10
GitHub PATs: 10
Stripe Keys: 5
Google API Keys: 5
Slack Tokens: 5
Private Keys: 5
Generic High-Entropy: 20
Encoded Secrets: 10
```

**Execution:**
```bash
# Scan seeded repo
result=$(devshield scan secrets --path ./test-repos/test-seeded-small --format json)

# Count detections by type
aws_keys=$(echo "$result" | jq '[.findings[] | select(.type == "AWS_ACCESS_KEY")] | length')
github_tokens=$(echo "$result" | jq '[.findings[] | select(.type == "GITHUB_PAT")] | length')

# Calculate detection rate
total=$(echo "$result" | jq '.findings | length')
expected=10
rate=$((total * 100 / expected))

echo "Detection rate: $rate% ($total/$expected)"
```

### 2.3 Real-World Repository Scans

**Objective:** Discover unknown secrets in active projects

**Test Cases:**

| ID | Repository | Scan Type | Expected Outcome |
|----|------------|-----------|------------------|
| TC-020 | Popular open-source | Full history | 0-5 findings (review manually) |
| TC-021 | Internal company repo | Full history | Unknown (investigate) |
| TC-022 | Personal projects | Full history | Unknown (investigate) |

**Process:**
1. Scan repository
2. Review each finding manually
3. Classify as: True Positive, False Positive, Unknown
4. Report findings to repository maintainers (if TP)

### 2.4 Edge Case Tests

**Objective:** Verify robust handling of unusual inputs

**Test Cases:**

| ID | Scenario | Input | Expected Behavior |
|----|----------|-------|-------------------|
| TC-030 | Binary files | PDF with embedded text | Skip or scan (configurable) |
| TC-031 | Huge files | 500MB log file | Skip (>100MB limit) |
| TC-032 | Deep nesting | 200-level directory | Stop at 100 levels |
| TC-033 | Circular symlinks | A -> B -> A | Detect and skip |
| TC-034 | Unicode paths | 文件/config.yml | Handle correctly |
| TC-035 | Empty files | Empty file | Skip (no content) |
| TC-036 | Permission denied | Root-owned file | Skip with warning |
| TC-037 | Network files | NFS/SMB mount | Handle timeouts |

**Execution:**
```bash
# Test binary file handling
echo "Testing binary file..."
devshield scan secrets --path ./test-repos/binary-files
# Should complete without crash

# Test huge file handling
echo "Testing huge file..."
devshield scan secrets --path ./test-repos/huge-files
# Should skip files >100MB

# Test symlink handling
echo "Testing symlinks..."
devshield scan secrets --path ./test-repos/symlinks
# Should not hang on circular symlinks
```

### 2.5 Phase 3 Feature Tests

#### 2.5.1 Live Verification

| ID | Test Case | Input | Expected |
|----|-----------|-------|----------|
| TC-040 | Valid secret | Real AWS key (test account) | Status: verified |
| TC-041 | Invalid secret | Fake AWS key | Status: unverified |
| TC-042 | Revoked secret | Previously valid, now revoked | Status: unverified |
| TC-043 | Network error | No internet connection | Status: unknown |
| TC-044 | Rate limit | Many rapid requests | Backoff and retry |
| TC-045 | Timeout | Slow API response | Status: unknown after timeout |

**Execution:**
```bash
# Test with known valid/invalid secrets
export AWS_ACCESS_KEY_ID=AKIAIOSFODNN7EXAMPLE  # Fake
export AWS_SECRET_ACCESS_KEY=wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY

result=$(devshield scan secrets --path ./test-repos/test-seeded-small --verify)

# Check verification status
verified=$(echo "$result" | jq '[.findings[] | select(.verification_status == "verified")] | length')
unverified=$(echo "$result" | jq '[.findings[] | select(.verification_status == "unverified")] | length')
unknown=$(echo "$result" | jq '[.findings[] | select(.verification_status == "unknown")] | length')

echo "Verified: $verified, Unverified: $unverified, Unknown: $unknown"
```

#### 2.5.2 Baseline Files

| ID | Test Case | Steps | Expected |
|----|-----------|-------|----------|
| TC-050 | Create baseline | `baseline create` | .baseline.json created |
| TC-051 | Scan with baseline | `scan --baseline` | Only new findings |
| TC-052 | Update baseline | `baseline update` | Baseline updated |
| TC-053 | Preserve labels | Add FP label, update | Label preserved |
| TC-054 | Slim mode | `baseline create --slim` | Minimal JSON |
| TC-055 | Migrate baseline | Format version change | Migration succeeds |

**Execution:**
```bash
# Create baseline
devshield baseline create --path ./test-repos/test-seeded-small --output .baseline.json

# Verify baseline created
if [ -f .baseline.json ]; then
    echo "✅ Baseline created"
else
    echo "❌ Baseline not created"
fi

# Add new secret to repo
echo "NEW_SECRET=sk_live_newkey123456" >> ./test-repos/test-seeded-small/config.env

# Scan with baseline (should only find new secret)
result=$(devshield scan secrets --path ./test-repos/test-seeded-small --baseline .baseline.json)
new_findings=$(echo "$result" | jq '.findings | length')

if [ "$new_findings" -eq 1 ]; then
    echo "✅ Only new findings reported"
else
    echo "❌ Expected 1 finding, got $new_findings"
fi
```

#### 2.5.3 SARIF Output

| ID | Test Case | Steps | Expected |
|----|-----------|-------|----------|
| TC-060 | Generate SARIF | `--format sarif` | Valid SARIF JSON |
| TC-061 | Schema validation | Validate against schema | Passes validation |
| TC-062 | GitHub upload | Upload to GH Advanced Security | Accepted |
| TC-063 | Multiple findings | Repo with 10+ findings | All in SARIF |
| TC-064 | Rule metadata | Check SARIF rules | All rules defined |

**Execution:**
```bash
# Generate SARIF
devshield scan secrets --path ./test-repos/test-seeded-small --format sarif --output results.sarif

# Validate schema
python scripts/validate-sarif.py results.sarif
# Should exit 0

# Check required fields
jq '.runs[0].tool.driver.name' results.sarif
# Should return "DevShield"

jq '.runs[0].results | length' results.sarif
# Should match finding count
```

#### 2.5.4 Pre-commit Hooks

| ID | Test Case | Steps | Expected |
|----|-----------|-------|----------|
| TC-070 | Install hook | `pre-commit install` | Hook installed |
| TC-071 | Clean commit | Commit without secrets | Commit succeeds |
| TC-072 | Secret commit | Commit with secrets | Commit blocked |
| TC-073 | Bypass | `--no-verify` | Commit succeeds |
| TC-074 | Uninstall | `pre-commit uninstall` | Hook removed |
| TC-075 | Config | Custom config file | Config applied |

**Execution:**
```bash
# Install hook
devshield pre-commit install

# Verify hook installed
if [ -f .git/hooks/pre-commit ]; then
    echo "✅ Hook installed"
else
    echo "❌ Hook not installed"
fi

# Test clean commit
echo "console.log('hello')" > test.js
git add test.js
if git commit -m "test"; then
    echo "✅ Clean commit succeeded"
else
    echo "❌ Clean commit failed"
fi

# Test secret commit
echo "AWS_KEY=AKIAIOSFODNN7EXAMPLE" > secret.env
git add secret.env
if git commit -m "secret" 2>/dev/null; then
    echo "❌ Secret commit should have been blocked"
else
    echo "✅ Secret commit blocked"
fi
```

---

## 3. Success Criteria

### 3.1 Detection Rate

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Overall detection** | >95% | Seeded repos |
| **AWS keys** | >98% | Seeded AWS keys |
| **GitHub tokens** | >98% | Seeded GitHub tokens |
| **Stripe keys** | >95% | Seeded Stripe keys |
| **Generic secrets** | >90% | High-entropy strings |
| **Encoded secrets** | >85% | Base64/hex encoded |

### 3.2 False Positive Rate

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Clean repos** | <5% | Findings / Files |
| **Verified secrets** | <1% | After verification |
| **Binary files** | 0% | Should skip |
| **Test fixtures** | <10% | Known test data |

### 3.3 Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| **Small repo (<100 files)** | <100ms | Mean scan time |
| **Medium repo (<1000 files)** | <500ms | Mean scan time |
| **Large repo (<10000 files)** | <5s | Mean scan time |
| **Pre-commit (normal)** | <3s | Hook execution |
| **Pre-commit (large)** | <10s | Hook with 100+ files |
| **Memory usage** | <100MB | Peak RSS |

### 3.4 Feature Completeness

| Feature | Required | Nice to Have |
|---------|----------|--------------|
| **Live verification** | AWS, GitHub, Stripe | All 20+ types |
| **Baseline files** | Create, update, diff | Slim mode, migrate |
| **SARIF output** | Valid schema | GitHub upload |
| **Pre-commit hooks** | Install, run, bypass | Pre-commit framework |
| **Encoded detection** | Base64, hex | Percent, nested |
| **Archive scanning** | ZIP, TAR | GZ, BZ2, nested |

---

## 4. Test Execution Plan

### 4.1 Phase 3 Test Schedule

| Week | Focus | Test Cases |
|------|-------|------------|
| 1 | Clean repos | TC-001 to TC-007 |
| 2 | Seeded repos | TC-010 to TC-013 |
| 3 | Edge cases | TC-030 to TC-037 |
| 4 | Live verification | TC-040 to TC-045 |
| 5 | Baseline files | TC-050 to TC-055 |
| 6 | SARIF output | TC-060 to TC-064 |
| 7 | Pre-commit hooks | TC-070 to TC-075 |
| 8 | Real-world scans | TC-020 to TC-022 |
| 9 | Performance | All performance tests |
| 10 | Regression | Full test suite |

### 4.2 Automated Test Suite

```yaml
# .github/workflows/qa-tests.yml
name: QA Tests

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        test-type: [clean, seeded, edge-cases, features]
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
      
      - name: Build DevShield
        run: cargo build --release
      
      - name: Run ${{ matrix.test-type }} tests
        run: ./scripts/run-qa-tests.sh ${{ matrix.test-type }}
      
      - name: Upload results
        uses: actions/upload-artifact@v4
        with:
          name: test-results-${{ matrix.test-type }}
          path: test-results/
```

### 4.3 Manual Test Checklist

**Before Phase 3 Release:**

- [ ] All automated tests passing
- [ ] Manual review of 100 random findings
- [ ] Performance benchmarks meet targets
- [ ] Documentation complete
- [ ] Edge cases tested
- [ ] Real-world repos scanned
- [ ] False positive rate <5%
- [ ] Detection rate >95%

---

## 5. Defect Classification

### 5.1 Severity Levels

| Level | Description | Response Time |
|-------|-------------|---------------|
| **P0 - Critical** | False negative on critical secret | Immediate |
| **P1 - High** | False positive rate >10% | 24 hours |
| **P2 - Medium** | Performance regression >50% | 1 week |
| **P3 - Low** | Minor UI/UX issue | Next release |

### 5.2 Defect Types

| Type | Description | Example |
|------|-------------|---------|
| **False Negative** | Missed secret | AWS key not detected |
| **False Positive** | Clean code flagged | Variable named `aws_config` |
| **Performance** | Slow scan | 10s for 100 files |
| **Crash** | Panic/error | Binary file causes panic |
| **Feature** | Missing functionality | Baseline update fails |

### 5.3 Defect Tracking

**Template:**
```markdown
## Defect Report

**ID:** DEF-001
**Severity:** P1 - High
**Type:** False Positive
**Date:** 2026-03-15

### Description
File `config.example.yml` flagged as containing AWS key

### Reproduction
```bash
devshield scan secrets --path ./test-repos/express
```

### Expected
No findings (clean repository)

### Actual
1 finding: AWS_ACCESS_KEY at line 42

### Analysis
Line 42 contains `aws_example_key: placeholder` which matches regex but is documentation

### Fix
Add `example`, `placeholder` to word filter
```

---

## 6. Test Data Management

### 6.1 Seeded Secret Format

**Security Note:** Never use real secrets in tests!

```python
# scripts/generate-test-secrets.py

FAKE_SECRETS = {
    "AWS_ACCESS_KEY": "AKIAIOSFODNN7EXAMPLE",  # AWS documented example
    "AWS_SECRET_KEY": "wJalrXUtnFEMI/K7MDENG/bPxRfiCYEXAMPLEKEY",
    "GITHUB_PAT": "ghp_" + "x" * 40,
    "STRIPE_KEY": "sk_live_" + "x" * 24,
    "GOOGLE_API": "AIza" + "x" * 35,
    "SLACK_TOKEN": "xoxb-" + "x" * 12 + "-" + "x" * 12 + "-" + "x" * 24,
}

def generate_test_file(path, secrets):
    with open(path, 'w') as f:
        for secret_type, value in secrets.items():
            f.write(f"{secret_type}={value}\n")
```

### 6.2 Test Repository Maintenance

**Weekly Tasks:**
- Update test repositories (git pull)
- Re-seed test repositories (ensure secrets present)
- Verify clean repositories still clean
- Add new edge cases as discovered

**Monthly Tasks:**
- Review false positive reports
- Add new secret patterns to seeded repos
- Update benchmark baselines
- Archive old test results

---

## 7. Reporting

### 7.1 Daily Test Report

```markdown
## QA Test Report - 2026-03-15

### Summary
- Tests Run: 50
- Passed: 48
- Failed: 2
- Skipped: 0

### Failures
1. TC-032 (Deep nesting): Expected skip at 100 levels, got 150
2. TC-045 (Timeout): Timeout not respected for slow API

### Metrics
- Detection Rate: 96% (target: >95%) ✅
- FP Rate: 3% (target: <5%) ✅
- Avg Scan Time (100 files): 85ms (target: <100ms) ✅

### Blockers
None

### Notes
Need to fix recursion limit in directory walker
```

### 7.2 Weekly Summary Report

```markdown
## QA Weekly Summary - Week 12

### Phase 3 Progress
- Live Verification: 100% complete, all tests passing
- Baseline Files: 90% complete, 1 failing test
- SARIF Output: 100% complete, GitHub validated
- Pre-commit Hooks: 80% complete, performance tuning needed

### Test Coverage
- Unit Tests: 150 (95% coverage)
- Integration Tests: 50
- E2E Tests: 20
- Manual Tests: 10

### Defects
- Open: 5 (0 critical, 1 high, 2 medium, 2 low)
- Closed: 15
- New This Week: 3

### Performance
- Speed: Within targets
- Memory: Within targets
- Accuracy: 96% detection, 3% FP

### Risks
- Pre-commit hook performance needs optimization
- Archive scanning memory usage high

### Next Week
- Fix pre-commit performance
- Complete baseline migration feature
- Start real-world repo scanning
```

---

## 8. References

- Test Repository Sources:
  - https://github.com/trufflesecurity/test_keys
  - https://github.com/gitleaks/gitleaks/tree/master/testdata
  - https://github.com/sergejey/majestic-12 (seeding examples)

- Testing Tools:
  - Criterion.rs (benchmarks)
  - assert_cmd (CLI testing)
  - tempfile (test fixtures)

- SARIF Validation:
  - https://sarifweb.azurewebsites.net/Validation

---

*Document created: 2026-03-15*
*Next: Update HANDOFF.md with Phase 3 research summary*
