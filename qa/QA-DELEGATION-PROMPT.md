# Coax QA Delegation Prompt

**Purpose:** Comprehensive QA testing of Coax security scanner
**Priority:** P0 - Must Complete
**Estimated Time:** 2-3 hours

---

## Task Overview

Run comprehensive QA tests on all 5 test repositories using the Coax scanner. Document results, identify issues, and provide performance analysis.

---

## Prerequisites

- Location: `/home/shva/QwenDev/coax-internal/coax`
- Binary: `./target/release/coax` (v0.2.0)
- Test Repos: `qa/test-repos/{small,medium,large,mixed-language,legacy}`
- Templates: `qa/qa-template.md`, `qa/results/TEMPLATE.md`

---

## Execution Steps

### Step 1: Verify Binary (5 min)

```bash
cd /home/shva/QwenDev/coax-internal/coax

# Verify binary exists and works
./target/release/coax version
./target/release/coax --help
```

**Expected:** Version 0.2.0, help displays correctly

---

### Step 2: Run Baseline Scans (30 min)

Scan each repository and save JSON results:

```bash
# Small repository
./target/release/coax scan -p qa/test-repos/small \
    -f json \
    -o qa/results/small.json \
    --quiet

# Medium repository
./target/release/coax scan -p qa/test-repos/medium \
    -f json \
    -o qa/results/medium.json \
    --quiet

# Large repository
./target/release/coax scan -p qa/test-repos/large \
    -f json \
    -o qa/results/large.json \
    --quiet

# Mixed-language repository
./target/release/coax scan -p qa/test-repos/mixed-language \
    -f json \
    -o qa/results/mixed-language.json \
    --quiet

# Legacy repository
./target/release/coax scan -p qa/test-repos/legacy \
    -f json \
    -o qa/results/legacy.json \
    --quiet
```

---

### Step 3: Performance Benchmarking (30 min)

Run timed scans with verbose output:

```bash
# Time each scan
echo "=== Small ===" && time ./target/release/coax scan -p qa/test-repos/small --quiet
echo "=== Medium ===" && time ./target/release/coax scan -p qa/test-repos/medium --quiet
echo "=== Large ===" && time ./target/release/coax scan -p qa/test-repos/large --quiet
echo "=== Mixed ===" && time ./target/release/coax scan -p qa/test-repos/mixed-language --quiet
echo "=== Legacy ===" && time ./target/release/coax scan -p qa/test-repos/legacy --quiet
```

**Record for each:**
- Scan duration (seconds)
- Files scanned
- Findings count
- Memory usage (if available)

---

### Step 4: Detailed Analysis (1 hour)

For each repository, complete a QA test report:

```bash
# Copy template for each repo
cp qa/qa-template.md qa/results/small-report.md
cp qa/qa-template.md qa/results/medium-report.md
cp qa/qa-template.md qa/results/large-report.md
cp qa/qa-template.md qa/results/mixed-language-report.md
cp qa/qa-template.md qa/results/legacy-report.md
```

**Fill in each report with:**

1. **Test Objective:** What is being validated
2. **Repository Details:** File count, languages, size
3. **Commands Run:** Exact commands used
4. **Performance Metrics:** Duration, files, findings
5. **Findings Summary:** By severity and pattern type
6. **Expected vs Actual:** Compare against known secrets
7. **Issues/Bugs:** False positives, false negatives, crashes
8. **Pass/Fail Verdict:** Overall assessment
9. **Recommendations:** Improvements suggested

---

### Step 5: Output Format Testing (15 min)

Test all output formats on small repository:

```bash
# Text format (default)
./target/release/coax scan -p qa/test-repos/small -f text > /tmp/text_output.txt

# JSON format
./target/release/coax scan -p qa/test-repos/small -f json > /tmp/json_output.json

# YAML format
./target/release/coax scan -p qa/test-repos/small -f yaml > /tmp/yaml_output.yaml

# Validate outputs
cat /tmp/text_output.txt | head -20
cat /tmp/json_output.json | python3 -m json.tool | head -20
cat /tmp/yaml_output.yaml | head -20
```

**Verify:**
- Text output is human-readable
- JSON is valid JSON
- YAML is valid YAML
- All formats contain same findings

---

### Step 6: Edge Case Testing (30 min)

Test edge cases and CLI options:

```bash
# Test with line content
./target/release/coax scan -p qa/test-repos/small --with-content --quiet

# Test exclude patterns
./target/release/coax scan -p qa/test-repos/small -e "*.py,test_*" --quiet

# Test thread count
./target/release/coax scan -p qa/test-repos/small -t 1 --quiet
./target/release/coax scan -p qa/test-repos/small -t 4 --quiet

# Test max file size
./target/release/coax scan -p qa/test-repos/small --max-file-size 1KB --quiet

# Test verbose mode
./target/release/coax scan -p qa/test-repos/small -v 2>&1 | head -20

# Test non-existent path (should error)
./target/release/coax scan -p /nonexistent/path 2>&1

# Test clean file (should have no findings)
echo "clean content" > /tmp/clean.txt
./target/release/coax scan -p /tmp/clean.txt --quiet
```

**Record:**
- Which options work correctly
- Any errors or unexpected behavior
- Performance differences with thread counts

---

### Step 7: False Positive Analysis (30 min)

Analyze findings for potential false positives:

```bash
# Review high entropy findings
cat qa/results/small.json | python3 -c "
import json, sys
data = json.load(sys.stdin)
for f in data['findings']:
    if 'HIGH_ENTROPY' in f['pattern']:
        print(f\"File: {f['file']}, Line: {f['line']}, Pattern: {f['pattern']}\")
"
```

**For each potential false positive:**
- Is it actually a secret?
- If not, why did it match?
- How can the pattern be improved?

---

### Step 8: Generate Summary Report (15 min)

Create overall QA summary:

```bash
cat > qa/results/QA-SUMMARY-$(date +%Y-%m-%d).md << 'EOF'
# Coax QA Summary

**Date:** YYYY-MM-DD
**Version:** 0.2.0
**Tester:** [Agent Name]

## Executive Summary

[Brief overview of QA results]

## Test Results

| Repository | Files | Duration | Findings | Status |
|------------|-------|----------|----------|--------|
| Small | | | | ✅/❌ |
| Medium | | | | ✅/❌ |
| Large | | | | ✅/❌ |
| Mixed-Language | | | | ✅/❌ |
| Legacy | | | | ✅/❌ |

## Performance Analysis

[Summary of performance metrics]

## Issues Found

[List any bugs or issues discovered]

## Recommendations

[Suggested improvements]

## Conclusion

[Overall assessment]
EOF
```

---

## Deliverables

1. ✅ **JSON Results:** `qa/results/*.json` for all 5 repos
2. ✅ **QA Reports:** `qa/results/*-report.md` for all 5 repos
3. ✅ **Performance Data:** Timing and metrics recorded
4. ✅ **Issue List:** Bugs, false positives, false negatives
5. ✅ **Summary Report:** `qa/results/QA-SUMMARY-YYYY-MM-DD.md`
6. ✅ **Recommendations:** Actionable improvements

---

## Success Criteria

- [ ] All 5 repositories scan without errors
- [ ] Expected secrets are detected in each repo
- [ ] Performance within targets (<30s for large)
- [ ] JSON/YAML outputs are valid
- [ ] False positives documented and analyzed
- [ ] All QA templates completed
- [ ] Summary report generated

---

## Known Test Secrets

### Small Repository (5 expected)
- AWS_ACCESS_KEY (config.py:7)
- AWS_SECRET_KEY (config.py:8)
- POSTGRESQL_CONNECTION_STRING (database.py:19)

### Medium Repository (9+ expected)
- OPENAI_API_KEY (src/api/client.js:15)
- ANTHROPIC_API_KEY (src/api/client.js:19)
- STRIPE_SECRET_KEY (src/config/payment.js:9)
- SQUARE_ACCESS_TOKEN (src/config/payment.js:13)
- SENDGRID_API_KEY (src/services/email.py:23)
- MAILGUN_API_KEY (src/services/email.py:30)
- AWS credentials (.env.production)
- MONGODB_CONNECTION_STRING (.env.production)

### Large Repository (20+ expected)
- Multiple AWS, GitHub, Stripe patterns in config_*.json files
- Secrets in secrets_*.py files

### Mixed-Language Repository (5+ expected)
- OPENAI_API_KEY (frontend/src/api.js)
- DISCORD_BOT_TOKEN (frontend/src/api.js)
- DATABASE_URL (backend/app/config.py)
- AWS credentials (backend/app/config.py, backend/.env)
- GITHUB_PAT (rust-cli/src/main.rs)

### Legacy Repository (8+ expected)
- MySQL passwords (config.php, old_config.py)
- Connection strings (multiple files)
- Private keys (scripts/deploy.sh)
- API keys (config/app.properties)

---

## Reporting Format

When reporting results, use this format:

```markdown
### Repository: [name]

**Status:** ✅ PASS / ❌ FAIL

**Performance:**
- Duration: X.XXs (target: <Ys)
- Files: X
- Findings: X

**Detection Accuracy:**
- Expected: X secrets
- Found: Y secrets
- False Positives: Z

**Issues:**
1. [Issue description]

**Recommendations:**
1. [Recommendation]
```

---

## Emergency Contacts

If issues occur:

1. **Binary crashes:** Check `./target/release/coax version` still works
2. **No findings:** Verify test files contain expected secrets
3. **Performance issues:** Try `-t 1` for single-threaded baseline
4. **Invalid output:** Check JSON/YAML with validators

---

**Created:** 2026-03-14
**Version:** 0.2.0
**Next Review:** After each major release
