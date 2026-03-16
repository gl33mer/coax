

---

## Instructions for Qwen Code: Benchmark Analysis Fix

### Context

The current benchmark results have two critical issues that need fixing before they're publishable:

1. **Homoglyph findings are cross-contaminating the secrets results** — 250 of 260 "secret findings" are actually UNICODE-HOMOGLYPH detections, not secrets. This inflates Coax's numbers and makes the comparison misleading.
2. **The analysis doesn't use ground truth** — we have labeled TP/TN files but the script just counts raw findings instead of computing precision/recall against those labels.

### Task 1: Investigate the Homoglyph Cross-Fire

**Goal:** Understand why 250 homoglyph findings appear in the secrets dataset.

**Steps:**

1. Look at `coax-secrets.json` (the raw Coax output from scanning the secrets dataset).
2. Filter to findings where the type is `UNICODE-HOMOGLYPH`.
3. For each flagged file, answer:
   - What specific characters are being flagged?
   - Are they genuinely non-ASCII (e.g., Cyrillic `а` instead of Latin `a`), or is the detector misfiring on legitimate ASCII?
   - Were these characters introduced accidentally (e.g., copy-paste from a web page that introduced smart quotes or unicode dashes)?
4. If the test files contain accidental non-ASCII characters, fix the test files — they should be pure ASCII for the secrets test corpus.
5. If the homoglyph detector is firing on legitimate characters, that's a detector bug — document it for fixing.
6. After investigation, re-run the Coax scan on the secrets dataset and confirm the homoglyph count drops to near zero.

**Report back:** What caused the 250 homoglyph findings? Were they test data issues or detector issues? What was the fix?

---

### Task 2: Build a Ground-Truth Analysis Script

**Goal:** Create a script that computes precision, recall, and F1 for each tool using the labeled test data.

**File:** `coax-benchmarks/scripts/analyze-ground-truth.sh` (or `.py` if easier — Python is fine for analysis)

**Ground truth labels (from the test corpus structure):**

```
datasets/secrets/
  true-positives/    → 12 files, each SHOULD trigger ≥1 secret finding
  true-negatives/    → 11 files, each should trigger 0 secret findings
  encoded/           → 3 files, each SHOULD trigger ≥1 secret finding (treat as TP)

datasets/unicode/
  bidi/              → 5 files, each SHOULD trigger ≥1 unicode finding
  homoglyphs/        → 5 files, each SHOULD trigger ≥1 unicode finding
  invisible-chars/   → 5 files, each SHOULD trigger ≥1 unicode finding
  safe-i18n/         → 5 files, each should trigger 0 unicode findings
```

**Logic for secrets evaluation (per tool):**

```
For each file in true-positives/ and encoded/:
  findings = [f for f in tool_results if f.file == this_file AND f.type is a SECRET type (not unicode)]
  if len(findings) >= 1: count as TRUE POSITIVE
  if len(findings) == 0: count as FALSE NEGATIVE (miss)

For each file in true-negatives/:
  findings = [f for f in tool_results if f.file == this_file AND f.type is a SECRET type (not unicode)]
  if len(findings) == 0: count as TRUE NEGATIVE
  if len(findings) >= 1: count as FALSE POSITIVE
```

**CRITICAL:** When evaluating secrets, **exclude** any findings with a unicode type (UNICODE-HOMOGLYPH, UNICODE-INVISIBLE_CHARACTER, UNICODE-BIDIRECTIONAL_OVERRIDE, UNICODE-UNICODE_TAG). Only count findings that are actual secret detections (GENERIC_SECRET, GENERIC_PASSWORD, HIGH_ENTROPY_STRING, or any provider-specific pattern like AWS_ACCESS_KEY, etc.).

**Logic for unicode evaluation (per tool):**

```
For each file in bidi/, homoglyphs/, invisible-chars/:
  findings = [f for f in tool_results if f.file == this_file AND f.type is a UNICODE type]
  if len(findings) >= 1: count as TRUE POSITIVE
  if len(findings) == 0: count as FALSE NEGATIVE

For each file in safe-i18n/:
  findings = [f for f in tool_results if f.file == this_file AND f.type is a UNICODE type]
  if len(findings) == 0: count as TRUE NEGATIVE
  if len(findings) >= 1: count as FALSE POSITIVE
```

**Compute and output these metrics per tool, per category:**

```
Precision = TP / (TP + FP)
Recall    = TP / (TP + FN)
F1        = 2 * (Precision * Recall) / (Precision + Recall)
```

**Output format** — print a clean markdown table like:

```
## Secrets Detection (Ground Truth)

| Tool       | TP | FN | FP | TN | Precision | Recall | F1   |
|------------|----|----|----|----|-----------|--------|------|
| Coax       | ?  | ?  | ?  | ?  | ?%        | ?%     | ?    |
| Gitleaks   | ?  | ?  | ?  | ?  | ?%        | ?%     | ?    |
| TruffleHog | ?  | ?  | ?  | ?  | ?%        | ?%     | ?    |

## Unicode Detection (Ground Truth)

| Tool       | TP | FN | FP | TN | Precision | Recall | F1   |
|------------|----|----|----|----|-----------|--------|------|
| Coax       | ?  | ?  | ?  | ?  | ?%        | ?%     | ?    |
| Gitleaks   | ?  | ?  | ?  | ?  | ?%        | ?%     | ?    |
| TruffleHog | ?  | ?  | ?  | ?  | ?%        | ?%     | ?    |
```

**Important notes for parsing each tool's JSON:**

- **Coax:** Check the actual JSON structure — findings likely have a `rule_id` or `finding_type` field. Use that to categorize.
- **Gitleaks:** Findings have a `RuleID` field (e.g., `generic-api-key`). All Gitleaks findings are secret-type.
- **TruffleHog:** Findings have a `DetectorName` field. All TruffleHog findings are secret-type.
- File paths in findings will need to be matched against the directory structure to determine ground truth category.

---

### Task 3: Check Finding Granularity

**Goal:** Determine whether Coax reports one finding per attack/occurrence or one finding per character.

**Steps:**

1. Pick one homoglyph test file from `datasets/unicode/homoglyphs/`.
2. Count how many findings Coax reports for that single file.
3. If Coax reports, say, 40 findings for one file that contains 40 Cyrillic characters, that's per-character reporting — note this as a UX issue to address (findings should be grouped per line or per cluster, not per character).
4. Report the finding-to-file ratio for each unicode subcategory.

---

### Task 4: Update the Benchmark Report

**Goal:** Replace `BENCHMARK-RESULTS-v0.8.3.md` with corrected data.

After Tasks 1-3 are complete:

1. Replace the raw-count summary table with the ground-truth precision/recall table.
2. Report secrets and unicode findings separately — never sum them.
3. Remove or correct the "260 secrets found" claim to reflect actual secret-type findings only.
4. Add a "Finding Granularity" section noting the per-character vs. per-occurrence behavior.
5. Keep the honest tone, the performance comparison, and the QA targets.

---

### Deliverables

1. Root cause explanation for the 250 homoglyph findings in the secrets dataset
2. Fixed test files (if the issue was in the test data) or documented detector bug
3. `analyze-ground-truth.sh` (or `.py`) script that computes precision/recall/F1
4. Updated `BENCHMARK-RESULTS-v0.8.3.md` with corrected metrics
5. Finding granularity report (per-character vs. per-occurrence)

---

### What NOT To Do

- Do not sum secret findings and unicode findings together in any table
- Do not report "Unknown" for TP/FP rates — we have labeled data, compute them
- Do not change Gitleaks or TruffleHog numbers — only fix how Coax results are categorized and analyzed
- Do not delete or hide unfavorable results — if Coax's secret-only TP count is lower than Gitleaks, report that honestly

---

