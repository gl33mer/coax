# Coax QA Test Template

**Test ID:** QA-YYYY-MM-DD-XXX
**Date:** YYYY-MM-DD HH:MM
**Tester:** [Name/Agent]
**Status:** ⏳ In Progress / ✅ Pass / ❌ Fail

---

## Test Objective

[Describe what this test is validating, e.g., "Validate secret detection in Python projects" or "Performance benchmark for large repositories"]

---

## Repository Details

| Property | Value |
|----------|-------|
| **Name** | [Repository name] |
| **Location** | `qa/test-repos/[name]/` |
| **Size** | [X files, Y MB] |
| **Languages** | [e.g., Python, JavaScript, Rust] |
| **Type** | [Small/Medium/Large/Mixed/Legacy] |
| **Source** | [Created/Cloned from URL] |
| **Known Secrets** | [Number and types of intentional test secrets] |

### File Breakdown

| Language | Files | Lines of Code |
|----------|-------|---------------|
| [e.g., Python] | [X] | [Y] |
| [e.g., JavaScript] | [X] | [Y] |
| **Total** | **[X]** | **[Y]** |

---

## Commands Run

```bash
# Primary scan command
./target/release/coax scan -p qa/test-repos/[name] [options]

# Additional commands (if any)
./target/release/coax scan -p qa/test-repos/[name] -f json -o results.json
./target/release/coax scan -p qa/test-repos/[name] -v
```

### Command Options Used

| Option | Value | Purpose |
|--------|-------|---------|
| `--path` | `[path]` | Target repository |
| `--format` | `[text/json/yaml]` | Output format |
| `--threads` | `[N]` | Thread count |
| `--exclude` | `[patterns]` | Excluded patterns |
| `--with-content` | `[true/false]` | Include line content |
| `--verbose` | `[true/false]` | Verbose output |

---

## Performance Metrics

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| **Scan Duration** | [X.XXs] | [<target] | ✅/❌ |
| **Files Scanned** | [X] | - | - |
| **Lines Processed** | [X] | - | - |
| **Memory Usage** | [X MB] | [<target] | ✅/❌ |
| **CPU Utilization** | [X%] | - | - |
| **Findings Count** | [X] | - | - |

### Timing Breakdown

| Phase | Duration |
|-------|----------|
| Pattern compilation | [X ms] |
| File collection | [X ms] |
| Parallel scanning | [X ms] |
| Result formatting | [X ms] |
| **Total** | **[X ms]** |

---

## Findings Summary

### By Severity

| Severity | Count | Percentage |
|----------|-------|------------|
| 🔴 Critical | [X] | [X%] |
| 🟠 High | [X] | [X%] |
| 🟡 Medium | [X] | [X%] |
| 🟢 Low | [X] | [X%] |
| **Total** | **[X]** | **100%** |

### By Pattern Type

| Pattern | Count | Files Affected |
|---------|-------|----------------|
| [e.g., AWS_ACCESS_KEY] | [X] | [file1, file2] |
| [e.g., GITHUB_PAT] | [X] | [file1] |
| [e.g., HIGH_ENTROPY_STRING] | [X] | [file1, file2, file3] |

### Expected vs Actual Findings

| Expected | Actual | Match |
|----------|--------|-------|
| [X] secrets | [Y] found | ✅/❌ |

**Missing Findings:** [List any expected secrets not detected]

**Unexpected Findings:** [List any false positives]

---

## Issues/Bugs Found

### Issue 1: [Title]

**Severity:** [Critical/High/Medium/Low]
**Type:** [False Positive/False Negative/Performance/Crash/Other]

**Description:**
[Detailed description of the issue]

**Steps to Reproduce:**
1. [Step 1]
2. [Step 2]
3. [Step 3]

**Expected Behavior:**
[What should happen]

**Actual Behavior:**
[What actually happened]

**Evidence:**
```
[Relevant output, screenshots, or logs]
```

**Suggested Fix:**
[Recommendation for fixing]

---

### Issue 2: [Title]

[Repeat structure as above]

---

## Output Samples

### Text Output
```
[Paste relevant text output]
```

### JSON Output (excerpt)
```json
{
  "findings": [...],
  "summary": {...}
}
```

---

## Pass/Fail Verdict

| Criteria | Status |
|----------|--------|
| All expected secrets detected | ✅/❌ |
| No false positives in clean files | ✅/❌ |
| Performance within targets | ✅/❌ |
| No crashes or errors | ✅/❌ |
| Output formats valid | ✅/❌ |
| Exit codes correct | ✅/❌ |

### Overall Verdict: ✅ PASS / ❌ FAIL

**Confidence Level:** [High/Medium/Low]

---

## Recommendations

### Immediate Actions

1. [Action 1]
2. [Action 2]
3. [Action 3]

### Pattern Improvements

| Pattern | Issue | Suggestion |
|---------|-------|------------|
| [Pattern name] | [Issue] | [Suggestion] |

### Performance Optimizations

1. [Optimization 1]
2. [Optimization 2]

### Future Testing

1. [Test case to add]
2. [Repository type to include]

---

## Appendix

### Environment

| Property | Value |
|----------|-------|
| **OS** | [e.g., Ubuntu 22.04] |
| **CPU** | [e.g., AMD Ryzen 7 5800X] |
| **RAM** | [e.g., 32GB DDR4] |
| **Coax Version** | [e.g., 0.2.0] |
| **Rust Version** | [e.g., 1.76.0] |

### Test Files

| File | Purpose | Secrets |
|------|---------|---------|
| `config.py` | Config file with AWS key | 1 AWS_ACCESS_KEY |
| `.env` | Environment variables | 2 API keys |
| `clean.py` | Clean file (no secrets) | 0 |

### Related Tests

- [Link to related QA test]
- [Link to performance benchmark]

---

**Test Completed:** YYYY-MM-DD HH:MM
**Next Review:** YYYY-MM-DD
**Approved By:** [Name]
