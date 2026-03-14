# Coax QA Results

**Test Date:** YYYY-MM-DD HH:MM
**Binary Version:** 0.2.0
**Test ID:** QA-YYYY-MM-DD-XXX

---

## Repository Tested

| Property | Value |
|----------|-------|
| **Name** | [Repository name] |
| **Path** | `qa/test-repos/[name]/` |
| **Category** | [Small/Medium/Large/Mixed/Legacy] |
| **Files** | [X files] |
| **Languages** | [List languages] |

---

## Performance Results

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| **Scan Duration** | [X.XXs] | [<target] | ✅/⚠️/❌ |
| **Files Scanned** | [X] | - | - |
| **Total Findings** | [X] | - | - |
| **Critical** | [X] | - | - |
| **High** | [X] | - | - |
| **Medium** | [X] | - | - |
| **Low** | [X] | - | - |

### Performance History

| Date | Version | Duration | Files | Findings |
|------|---------|----------|-------|----------|
| YYYY-MM-DD | 0.2.0 | [X.XXs] | [X] | [X] |
| [Previous] | [Ver] | [X.XXs] | [X] | [X] |

---

## Findings

### Summary

```
🔍 X findings (X critical, X high, X medium, X low)
```

### Top Patterns Detected

| Rank | Pattern | Count | Severity |
|------|---------|-------|----------|
| 1 | [Pattern name] | [X] | [Severity] |
| 2 | [Pattern name] | [X] | [Severity] |
| 3 | [Pattern name] | [X] | [Severity] |
| 4 | [Pattern name] | [X] | [Severity] |
| 5 | [Pattern name] | [X] | [Severity] |

### Detailed Findings

| # | File | Line | Pattern | Severity | Recommendation |
|---|------|------|---------|----------|----------------|
| 1 | [file] | [X] | [pattern] | [sev] | [recommendation] |
| 2 | [file] | [X] | [pattern] | [sev] | [recommendation] |

---

## Issues

### False Positives

| # | File | Line | Pattern | Why False Positive |
|---|------|------|---------|-------------------|
| 1 | [file] | [X] | [pattern] | [Explanation] |

### False Negatives (Missed Secrets)

| # | File | Line | Expected Pattern | Why Missed |
|---|------|------|------------------|------------|
| 1 | [file] | [X] | [pattern] | [Explanation] |

### Other Issues

| # | Type | Description | Severity |
|---|------|-------------|----------|
| 1 | [Type] | [Description] | [Severity] |

---

## Verdict

| Criteria | Result |
|----------|--------|
| Detection Accuracy | [X%] |
| False Positive Rate | [X%] |
| Performance | ✅ Pass / ❌ Fail |
| Stability | ✅ Pass / ❌ Fail |

### Overall: ✅ PASS / ⚠️ PASS WITH ISSUES / ❌ FAIL

---

## Recommendations

1. [Recommendation 1]
2. [Recommendation 2]
3. [Recommendation 3]

---

## Raw Output

```json
{
  "version": "0.2.0",
  "scan_duration_ms": [X],
  "summary": {
    "files_scanned": [X],
    "total_findings": [X],
    "by_severity": {
      "critical": [X],
      "high": [X],
      "medium": [X],
      "low": [X]
    }
  },
  "findings": [...]
}
```

---

**Tested By:** [Name/Agent]
**Review Date:** YYYY-MM-DD
