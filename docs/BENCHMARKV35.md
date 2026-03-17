 Close All Gaps — Phase 2-8 Implementation

---

**You completed Phase 1 analysis. Here's what we learned and the implementation plan.**

### What Phase 1 Revealed

**FN Root Cause (all 9 files):** Every single FN is caused by `should_scan_extension()` filtering files out before scanning. This is the same class of bug we fixed for `is_test_file()` and `is_documentation()` — but in a different filter we missed. One fix recovers all 9 recalls.

**FP Root Cause (6 files):** 4 of 6 are `HIGH_ENTROPY_STRING` false alarms (binary data, encrypted vaults, PEM certs, hash values). 2 are pattern matches on placeholder text.

**Ground Truth:** `example-docs.md` is flagged by all 3 tools — likely mislabeled.

---

### PHASE 2: Ground Truth Correction

Open `example-docs.md` and inspect the actual strings that get flagged.

**Decision criteria:**
- If the flagged strings look like realistic key formats with random-looking characters (e.g., `sk_test_4eC39HqLyjWDarjtT1zdp7dc`) → reclassify as **TP** in `ground-truth.yaml`. Real key formats should be flagged even in documentation — humans copy-paste these into code constantly.
- If the flagged strings are obviously fake placeholders (e.g., `sk_test_YOUR_KEY_HERE`, `sk_test_example123`, `REPLACE_ME`) → keep as **TN**. The scanner is wrong to flag these.

Document your decision and reasoning.

---

### PHASE 3: Fix the Extension Filter (Recall — All 9 FN)

**CRITICAL ARCHITECTURAL GUIDANCE: Do NOT bypass `should_scan_extension()` for known patterns.** Unlike `is_test_file()` and `is_documentation()` which run AFTER reading file content, the extension filter runs BEFORE reading the file. Bypassing it would mean reading every `.jpg`, `.zip`, `.exe`, `.png`, `.wasm` in a repo, which is unacceptable for performance.

**Instead, do these three things:**

**3A. Expand the extension allowlist.** Add all missing text-based file extensions. At minimum, add every extension from the 9 FN files:
- `.csv` (data files)
- `.gradle` (build config)
- `.ipynb` (Jupyter notebooks)
- `.npmrc` (npm config)
- `.pp` (Puppet manifests)
- `.sls` (Salt states)

But don't stop there. Audit the full list against common file types a secret could appear in. Make sure the allowlist includes (if not already present):
- Build/CI: `.gradle`, `.sbt`, `.cmake`, `.mk`, `.bzl`, `.bazel`
- Config: `.pp`, `.sls`, `.hcl`, `.tf`, `.tfvars`, `.conf`, `.cfg`, `.ini`, `.properties`, `.plist`
- Data: `.csv`, `.tsv`, `.sql`, `.sqlite`
- Package manager: `.npmrc`, `.pypirc`, `.gemrc`, `.yarnrc`
- Notebooks: `.ipynb`, `.rmd`
- IaC: `.pp`, `.sls`, `.erb`
- Other: `.env`, `.envrc`, `.htaccess`, `.htpasswd`

**3B. Handle extensionless files.** Files like `Jenkinsfile`, `Makefile`, `Vagrantfile`, `Dockerfile`, `Gemfile`, `Rakefile`, `Procfile`, `Brewfile` have no extension but are text files that commonly contain secrets.

Implement a known-names list for common extensionless files that should always be scanned. Include at least:
- `Jenkinsfile`
- `Makefile`
- `Vagrantfile`
- `Dockerfile`
- `Gemfile`
- `Rakefile`
- `Procfile`
- `Brewfile`
- `.env` (if treated as extensionless)

As a fallback for unknown extensionless files: do a quick binary check (read first 512 bytes, if no null bytes, treat as text and scan).

**3C. Add unit tests** for the extension filter:
- Test that all newly added extensions are accepted
- Test that extensionless known files (Jenkinsfile, Makefile, Vagrantfile) are accepted
- Test that binary extensions (.jpg, .png, .exe, .zip, .wasm, .bin) are still rejected
- Test that an unknown extensionless text file is accepted

Run `cargo test` after this phase.

---

### PHASE 4: Fix False Positives (Precision — 6 FP)

Fix each FP with a targeted, narrow fix. Work through them in this order:

**4A. Binary file detection (binary-signature.bin)**
Add a binary file check early in the scan pipeline. Read the first 512 bytes — if null bytes are present, skip the file entirely. This is a zero-risk fix; binary files never contain text-format secrets.

**4B. Placeholder detection (example-docs.md, placeholder-config.py)**
Note: If you reclassified `example-docs.md` as TP in Phase 2, this only applies to `placeholder-config.py`.

Add a placeholder value filter that runs AFTER pattern matching. When a pattern match is found, check if the matched secret value contains common placeholder indicators:
- Exact matches: `example`, `test`, `sample`, `dummy`, `fake`, `placeholder`, `changeme`, `change_me`, `replace_me`, `your_key_here`, `your-key-here`, `xxx`, `todo`, `fixme`, `insert`, `redacted`
- Pattern: all same character repeated (e.g., `AAAAAAA`, `0000000`)
- Pattern: sequential characters (e.g., `1234567890`, `abcdefgh`)

**IMPORTANT:** This filter should only suppress matches where the SECRET VALUE itself contains placeholder text — not where the variable name or surrounding context does. A line like `test_api_key = "sk_live_abc123real"` should still match because the value is real even though the variable name contains "test".

**4C. PEM certificate marker exclusion (fake-ssl-cert.pem)**
When scanning `.pem`, `.crt`, `.cert`, `.key` files, check for `-----BEGIN CERTIFICATE-----` or `-----BEGIN PUBLIC KEY-----` markers. Content between PEM markers is encoded certificate data, not a leaked secret. Suppress entropy findings within PEM blocks.

Note: `-----BEGIN RSA PRIVATE KEY-----` and `-----BEGIN PRIVATE KEY-----` SHOULD still be flagged — private keys in repos are a real finding.

**4D. Encrypted vault exclusion (encrypted-values.env)**
Detect and skip encrypted/vault values. Suppress findings where the matched value:
- Is wrapped in `ENC[...]` (e.g., Ansible Vault, SOPS)
- Starts with `$ANSIBLE_VAULT;`
- Starts with `vault:` prefix
- Matches SOPS-encrypted format

These are already-encrypted values, not cleartext secrets.

**4E. Hash value exclusion (hash-values.json)**
**BE CAREFUL HERE.** Some real API keys are 64-character hex strings. Do NOT exclude based on hex string length alone.

Instead, suppress findings where BOTH conditions are true:
1. The matched value is a hex string (only characters 0-9, a-f, A-F)
2. AND the surrounding context indicates it's a hash — the JSON key, variable name, or preceding comment contains words like: `hash`, `digest`, `checksum`, `sha1`, `sha256`, `sha384`, `sha512`, `md5`, `fingerprint`, `signature`, `hmac`, `integrity`, `etag`

A 64-char hex string assigned to a key called `api_key` or `secret` must still be flagged.

**4F. Unit tests for each FP fix:**
For each fix above, add a test case that verifies:
- The specific FP case is now suppressed
- A similar-looking real secret is NOT suppressed (regression guard)

Example: test that `"checksum": "a1b2c3..."` is suppressed but `"api_key": "a1b2c3..."` is NOT.

Run `cargo test` after all FP fixes.

---

### PHASE 5: Unicode FP Fix

From the original benchmark, Coax had 4 FPs on safe internationalization files (legitimate CJK, Arabic, Cyrillic, Devanagari, emoji characters flagged as Trojan Source attacks).

**Fix approach — script/character allowlisting:**

Only flag Unicode that is genuinely suspicious in code contexts:
- **BiDi override characters:** U+202A through U+202E, U+2066 through U+2069
- **Zero-width characters in code:** U+200B (zero-width space), U+200C (ZWNJ), U+200D (ZWJ), U+200E/F (directional marks), U+FEFF (BOM, except at position 0)
- **Homoglyph substitutions in identifiers** (if currently detected)

Do NOT flag:
- Characters from legitimate scripts (Latin, CJK, Arabic, Hebrew, Cyrillic, Devanagari, Thai, Greek, Korean Hangul, Japanese Kana, emoji)
- Normal use of accented Latin characters
- Unicode in string literals and comments (only flag in identifiers if homoglyph detection exists)

**Test:**
- Run existing unicode test corpus — all 15 TP must still be caught
- The 4 safe-i18n FP files should now pass clean
- Run `cargo test`

---

### PHASE 6: Skip Encoded Detection Phase

The original encoded secret files (`base64_secrets.txt`, `hex_url_secrets.txt`) are NOT in the expanded benchmark corpus. Skip this phase — do not let it block the benchmark re-run. We can harden encoded detection in Phase 1 of the roadmap.

---

### PHASE 7: Re-Run Full Benchmark

After all fixes are complete:

1. **Run `cargo test`** — every test must pass (existing + all new tests from Phases 3-5)

2. **Re-run Coax on the expanded corpus** (with any ground truth corrections applied):
```bash
cargo build --release
./target/release/coax scan ~/coaxbenchmarking/expanded-corpus/ --format json > ~/coaxbenchmarking/results/expanded/coax-expanded-v2.json
```

3. **Re-classify results** against the (possibly updated) ground-truth.yaml. Produce the per-file classification table.

4. **If ground truth changed** (e.g., example-docs.md reclassified), also re-classify the existing Gitleaks and TruffleHog results against the updated ground truth.

5. **Re-run real-world FP test:**
```bash
./target/release/coax scan ~/coaxbenchmarking/repos/fastify/ --format json > ~/coaxbenchmarking/results/expanded/coax-fastify-v2.json
./target/release/coax scan ~/coaxbenchmarking/repos/express/ --format json > ~/coaxbenchmarking/results/expanded/coax-express-v2.json
```
Count findings — target: 0 FPs on both repos.

6. **Produce the before/after comparison table:**

```
SECRETS DETECTION — EXPANDED CORPUS (updated ground truth):

| Tool            | TP | FN | FP | TN | Precision | Recall | F1   |
|-----------------|----|----|----|----|-----------|--------|------|
| Coax v1 (before)| 47 |  9 |  6 | 24 |   88.7%   | 83.9%  | 0.86 |
| Coax v2 (after) |  ? |  ? |  ? |  ? |     ?     |   ?    |  ?   |
| Gitleaks        | 49 |  7 |  3 | 27 |   94.2%   | 87.5%  | 0.91 |
| TruffleHog      |  6 | 50 |  0 | 30 |  100.0%   | 10.7%  | 0.19 |

REAL-WORLD FALSE POSITIVES:

| Repo     | Coax v1 | Coax v2 | Gitleaks | TruffleHog |
|----------|---------|---------|----------|------------|
| fastify  |    0    |    ?    |     2    |     0      |
| express  |    0    |    ?    |     0    |     0      |

UNICODE DETECTION (if Phase 5 was applied):

| Tool  | TP | FN | FP | TN | Precision | Recall | F1   |
|-------|----|----|----|----|-----------|--------|------|
| Before| 15 |  0 |  4 |  1 |   78.9%   | 100.0% | 0.88 |
| After |  ? |  ? |  ? |  ? |     ?     |   ?    |  ?   |
```

7. **If secrets F1 < 0.91 after fixes:** List every remaining FN and FP with explanation of what it would take to fix each one.

---

### PHASE 8: Cleanup

After benchmark results are satisfactory:

1. **Remove dead code** — commented-out code, unused functions, debug `println!` or `eprintln!` statements left from debugging
2. **Organize patterns in `secrets.rs`** — group by provider/category with clear section comments (e.g., `// === Cloud Providers ===`, `// === API Keys ===`, `// === Tokens ===`)
3. **Add comments to non-obvious logic:**
   - The known-pattern filter bypass in `scanner.rs` — explain WHY patterns bypass heuristic filters
   - The file-type exclusion bypass — explain WHY test/doc files are still scanned for known patterns
   - The extension allowlist — note that this is intentionally broad
   - Any new FP suppression logic — explain the criteria and why it's safe
4. **Run `cargo clippy`** — fix all warnings
5. **Run `cargo fmt`** — consistent formatting
6. **Run `cargo test`** — everything green, final confirmation
7. **Update `.gitignore`** — ensure build artifacts and temp files are excluded, but benchmark corpus and ground truth are version-controlled

---

### Deliverables Checklist

When complete, print this checklist with status:

```
DELIVERABLES:
- [ ] Ground truth reviewed and corrected (example-docs.md decision documented)
- [ ] Extension filter expanded (all 9 FN file types covered + extensionless files)
- [ ] Extension filter unit tests added and passing
- [ ] Binary file detection added (null byte check)
- [ ] Placeholder detection added
- [ ] PEM cert exclusion added (public certs only — private keys still flagged)
- [ ] Encrypted vault exclusion added
- [ ] Hash context exclusion added (context-aware, not length-based)
- [ ] FP unit tests added (each with regression guard)
- [ ] Unicode FP fixed (safe scripts allowlisted)
- [ ] Unicode TP preserved (all 15 still detected)
- [ ] cargo test — all passing (count: ___)
- [ ] cargo clippy — no warnings
- [ ] cargo fmt — clean
- [ ] Expanded benchmark re-run complete
- [ ] Secrets F1: ___ (target: ≥ 0.91)
- [ ] Unicode F1: ___ (target: ≥ 0.95)
- [ ] Real-world FPs: fastify=___, express=___ (target: 0, 0)
- [ ] Before/after comparison table produced
- [ ] Dead code removed
- [ ] Patterns organized with section comments
- [ ] Key logic commented
- [ ] All results saved to results/expanded/
```

---

**Work through the phases sequentially. Do not skip ahead. Run `cargo test` after each phase to catch regressions early. Print the benchmark comparison table as the final output so we can see exactly where we landed.**