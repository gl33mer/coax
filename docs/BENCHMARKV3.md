Context: We ran an expanded benchmark (56 TP, 30 TN, 86 files total) against Coax, Gitleaks, and TruffleHog. Results:

Tool	TP	FN	FP	TN	Precision	Recall	F1
Coax	47	9	6	24	88.7%	83.9%	0.86
Gitleaks	49	7	3	27	94.2%	87.5%	0.91
TruffleHog	6	50	0	30	100.0%	10.7%	0.19
Coax also has 0 real-world FPs on fastify + express (Gitleaks has 2). But Gitleaks beats Coax on the synthetic corpus. We need to close this gap.

Goal: Close all FN and FP gaps, re-run the benchmark, and produce final numbers that we can confidently ship. Target: F1 ≥ 0.91 (match or beat Gitleaks).

Architecture rule (CRITICAL): Known secret patterns are privileged — they bypass all heuristic filters (entropy pre-filter, word filter, token efficiency) and file-type exclusions. This was a hard-won fix. Do NOT reintroduce any filtering that runs before pattern matching on known patterns.

Results are at: coax-benchmarks/results/expanded/ (or ~/coaxbenchmarking/results/expanded/ — check both). Key files: coax-expanded.json, gitleaks-expanded.json, trufflehog-expanded.json, per-file-classification.txt, ground-truth.yaml.

PHASE 1: Diagnostic Analysis (Do Not Change Any Code Yet)
Task 1A: FN Cross-Reference (9 files Coax missed)

For each of the 9 files Coax classified as FN (missed):

What is the filename and what secret type does it contain (from ground-truth.yaml)?
Did Gitleaks detect it? If yes, what Gitleaks rule ID/description caught it?
Did TruffleHog detect it? If yes, what detector?
Open the file and find the actual secret string. What does it look like?
Check Coax's existing patterns in secrets.rs — is there a pattern that SHOULD match? If yes, why didn't it? (Regex too narrow? Filter suppressing it? File excluded?)
If no existing pattern covers it, what new pattern would be needed?
Output a table:

| # | File | Secret Type | Gitleaks Rule | Coax Pattern Exists? | Root Cause | Fix | |---|------|-------------|---------------|---------------------|------------|-----| | 1 | ... | ... | ... | Yes/No | regex too narrow / no pattern / filter | add pattern X / widen regex Y |
Task 1B: FP Analysis (6 files Coax falsely flagged)

For each of the 6 files Coax classified as FP (false alarm):

What is the filename and what did Coax flag (pattern name, matched text)?
Why is this a false positive? (placeholder value? hash? UUID? encrypted data? hex color?)
What Coax detection mechanism triggered? (specific pattern match? entropy? encoded detection?)
Did Gitleaks or TruffleHog also flag this file?
What fix would suppress this FP without suppressing real positives?
Output a table:

| # | File | Coax Flagged As | Actual Content | Trigger Mechanism | Gitleaks Also FP? | Fix | |---|------|-----------------|----------------|-------------------|-------------------|-----| | 1 | ... | aws_key | SHA-256 hash | entropy detection | No | raise entropy threshold for hashes |
Task 1C: Ground Truth Audit

Review attention-flags.txt. Are there any files where the ground truth label seems wrong? Specifically:

Any TP file that ALL THREE tools miss → likely mislabeled (not actually a secret)
Any TN file that ALL THREE tools flag → likely mislabeled (actually contains a secret)
Any file where only Gitleaks flags it and it looks like a Gitleaks FP rather than a Coax FN
If you find mislabeled files, note them with your recommended correction. We will correct ground truth before re-benchmarking.

Task 1D: Encoded Detection Review

From the original (smaller) benchmark, we had 2 FN from encoded secrets: base64_secrets.txt and hex_url_secrets.txt. Check:

Are these files in the expanded corpus? If so, does Coax still miss them?
Look at the encoded detection module — what's preventing detection?
Is this a quick fix or a deeper module issue?
STOP after Phase 1. Print all four analysis tables. Do not proceed to fixes until the analysis is complete.

PHASE 2: Ground Truth Corrections
Based on Phase 1C findings:

Correct any mislabeled files in ground-truth.yaml
If a file is genuinely mislabeled (e.g., a "TP" that contains no real secret pattern), either fix the file content or reclassify it
Document every ground truth change with justification
PHASE 3: Pattern Fixes (Recall)
For each FN identified in Phase 1A, implement the fix:

If a new pattern is needed:

Add it to secrets.rs following the existing pattern structure
Make sure it uses the same named capture groups as other patterns
Add a descriptive comment with an example of what it matches
If an existing pattern needs widening:

Modify the regex to cover the missed variant
Verify the widened regex still matches all existing test cases (run cargo test)
If a filter is suppressing a known pattern:

This should not be possible given the architectural rule (known patterns bypass filters)
If it IS happening, there's a regression — find and fix it without changing the bypass architecture
For each fix, immediately add unit tests following the 5-case standard:

Realistic match (secret embedded in code context — Python assignment, YAML config, JSON value, etc.)
Variation match (different quote style, different prefix, different spacing)
Invalid/should-not-match (too short, wrong character set, placeholder value)
Keyword-only (keyword present but no secret value — should not match)
Context match (secret in a realistic multi-line code block)
Run cargo test after each pattern addition to ensure nothing breaks.

PHASE 4: FP Fixes (Precision)
For each FP identified in Phase 1B, implement the targeted fix:

Common FP fixes (choose the narrowest fix that works):

Hash values triggering patterns: Add hex-string length checks or hash-prefix exclusions
Placeholders: Add placeholder string detection (e.g., example, test, dummy, xxx, your-key-here, CHANGE_ME, TODO)
UUIDs: Ensure UUID format (8-4-4-4-12 hex) is excluded from generic hex matchers
Encoded non-secrets: Improve encoded detection to check decoded content, not just encoding format
Entropy false alarms: Adjust entropy thresholds for specific content types
Rules for FP fixes:

Every FP fix must be narrow and targeted — do not add broad exclusions that could suppress real secrets
After each fix, verify it doesn't break any existing TP detection by running cargo test
Add a unit test for each FP case (the "false positive" case in the 5-case standard)
PHASE 5: Encoded Detection Fix
Based on Phase 1D analysis, fix the encoded detection module for base64 and hex-encoded secrets. This addresses the 2 FN from the original benchmark.

Requirements:

Base64-encoded AWS keys, API tokens, etc. should be detected
Hex-encoded secrets should be detected
URL-encoded secrets should be detected
Do NOT flag base64-encoded non-secrets (images, certificates, compressed data)
Add unit tests for encoded detection covering: encoded AWS key, encoded GitHub token, encoded random data (should not flag), encoded certificate (should not flag)
PHASE 6: Unicode FP Fix
From the original benchmark, Coax had 4 FPs on safe internationalization files. These are legitimate Unicode characters (CJK, Arabic, Cyrillic, Devanagari, emoji, etc.) being flagged as Trojan Source attacks.

Fix approach:

Implement script allowlisting — common legitimate scripts (Latin, CJK, Arabic, Cyrillic, Devanagari, Thai, Greek, etc.) should not trigger alerts
Only flag Unicode that is genuinely suspicious: BiDi override characters (U+202A-U+202E, U+2066-U+2069), zero-width characters (U+200B-U+200F, U+FEFF) in code contexts, homoglyph substitutions in identifiers
Preserve detection of actual Trojan Source attacks (the current 15 TP should still be caught)
Run existing unicode tests to verify no regressions
PHASE 7: Re-Run Full Benchmark
After all fixes are complete:

Run cargo test — all tests must pass (existing + new)
Re-run expanded benchmark on all 86 files (with any ground truth corrections from Phase 2):
coax scan expanded-corpus/ --format json > results/coax-expanded-v2.json
Re-run Gitleaks and TruffleHog only if ground truth changed (otherwise reuse existing results)
Re-run real-world FP test on fastify + express
Produce updated results table:
| Tool | TP | FN | FP | TN | Precision | Recall | F1 |
Produce a before/after comparison:
| Metric | Coax v1 (before) | Coax v2 (after) | Gitleaks | Change | |--------|-------------------|-----------------|----------|--------| | F1 | 0.86 | ??? | 0.91 | +??? | | Recall | 83.9% | ??? | 87.5% | +??? | | Precision | 88.7% | ??? | 94.2% | +??? | | Real-world FPs | 0 | ??? | 2 | ??? |
If F1 < 0.91 after fixes, list remaining gaps and what it would take to close them.
PHASE 8: Cleanup
After benchmark results are satisfactory:

Remove dead code — any commented-out code, unused functions, debug prints left from the fix journey
Organize patterns — ensure patterns in secrets.rs are grouped by provider/family with clear section comments
Code comments — ensure non-obvious logic has comments, especially:
The known-pattern filter bypass in scanner.rs
The file-type exclusion bypass logic
Any new FP suppression logic
Run cargo clippy — fix any warnings
Run cargo fmt — ensure consistent formatting
Run full test suite — cargo test — everything green
Update .gitignore — ensure benchmark results, temp files, build artifacts are excluded appropriately (but keep the benchmark corpus and ground truth in the repo if we want it versioned)
Deliverables Checklist
When complete, confirm each item:

 Phase 1 analysis tables printed (FN cross-reference, FP analysis, ground truth audit, encoded detection review)
 Ground truth corrections documented and applied
 All identified FN patterns fixed with unit tests
 All identified FPs fixed with unit tests
 Encoded detection working (base64/hex/URL)
 Unicode FP on safe i18n fixed
 cargo test — all passing
 cargo clippy — no warnings
 cargo fmt — clean
 Expanded benchmark re-run with updated results
 Real-world FP re-run confirmed 0 FPs
 Before/after comparison table produced
 Dead code removed, patterns organized, comments added
 All results saved to results/expanded/
Take this phase by phase. Complete Phase 1 analysis before making any code changes. Print the analysis tables so we can validate the plan before you start fixing.