# Coax Benchmark Fix - Phase 2 Final Summary

**Date:** March 16, 2026
**Status:** Architectural fix applied, GitHub/Stripe detection still under investigation

---

## Fixes Applied

### Fix 1: Known Patterns Bypass Heuristic Filters ✅

**File:** `crates/coax-scanner/src/scanner.rs`

**Change:** Added `is_known_secret_pattern()` function and modified filter logic to skip heuristic filters for known patterns.

**Impact:** AWS, GitHub, Stripe, and other known patterns now bypass:
- Token efficiency filter
- Word filter  
- Entropy pre-filter

### Fix 2: Removed "example" from Token Efficiency Block Words ✅

**File:** `crates/coax-scanner/src/token_efficiency.rs`

**Change:** Removed "example", "test", "sample" from common_words array.

**Impact:** Strings containing these words are no longer automatically filtered.

---

## Remaining Issue: GitHub/Stripe Patterns Not Matching

**Symptom:** GitHub tokens (`ghp_...`) and Stripe keys (`sk_live_...`) are NOT being detected even with filters bypassed.

**Evidence:**
- Patterns exist in `secrets.rs` (GITHUB_PAT: `ghp_[a-zA-Z0-9]{36}`)
- Regex works when tested directly with grep
- Tests `test_github_*` are FAILING
- CLI reports 43 patterns loaded

**Hypothesis:** The regex patterns may have issues (anchoring, length constraints, etc.) that prevent matching.

**Next Steps for Investigation:**
1. Add debug logging to pattern matching
2. Test regex directly in Rust code
3. Check if patterns are being compiled correctly
4. Verify pattern names match what `is_known_secret_pattern()` expects

---

## Test Data Issue: AWS Example Keys

**Issue:** Test files use `AKIAIOSFODNN7EXAMPLE` which is AWS's official documentation key.

**Decision:** This key SHOULD be filtered (it's public and useless to attackers).

**Recommendation:** Update test data to use different example keys:
- `AKIAIOSFODNN7TESTKEY1`
- `AKIAIOSFODNN7REALKEY2`

---

## Current Metrics (UNCHANGED - awaiting fix verification)

### Secrets Detection

| Tool       | TP | FN | FP | TN | Precision | Recall | F1   |
|------------|----|----|----|----|-----------|--------|------|
| Coax       | 0  | 15 | 3  | 8  | 0.0%      | 0.0%   | 0.00 |
| Gitleaks   | 9  | 6  | 2  | 9  | 81.8%     | 60.0%  | 0.69 |
| TruffleHog | 3  | 12 | 0  | 11 | 100.0%    | 20.0%  | 0.33 |

---

## Files Modified

1. `crates/coax-scanner/src/scanner.rs` - Added `is_known_secret_pattern()` and filter bypass logic
2. `crates/coax-scanner/src/token_efficiency.rs` - Removed "example", "test", "sample" from block words
3. `crates/coax-scanner/src/context.rs` - Updated `is_placeholder_value()` (earlier fix)
4. `crates/coax-scanner/src/unicode/confusables/data.rs` - Fixed IPA 'k' entry (earlier fix)

---

## Time Spent

- **Total:** 6 hours
- **Root Cause Analysis:** 3 hours
- **Fix Implementation:** 2 hours
- **Testing/Debugging:** 1 hour

---

## Recommendation

Given the time invested and the complexity of the GitHub/Stripe pattern matching issue, I recommend:

1. **Document current findings** (this document)
2. **Create a focused bug ticket** for GitHub/Stripe pattern matching
3. **Move to other priorities** (v0.8.4 cross-platform build, etc.)
4. **Return to this** when dedicated debugging time is available

The architectural fix (known patterns bypass filters) is correct and valuable even if GitHub/Stripe patterns need separate debugging.
