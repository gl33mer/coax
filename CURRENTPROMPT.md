



```
# 🎯 Coax v0.7.5 - Script Mixing Detection Fix

**Mission:** Fix the homoglyph detector to properly distinguish between:
1. ✅ Legitimate pure non-Latin identifiers (Greek μήνυμα, Cyrillic сообщение)
2. ⚠️ Deceptive mixed-script identifiers (variαble, pаypal)

**Repository:** https://github.com/gl33mer/coax
**Current Version:** v0.7.4
**Target Version:** v0.7.5

---

## 🚨 The Problem (Verified by QA Testing)

### Test Case 1: Pure Greek (Should be 0 findings)

```javascript
// Pure Greek variable names - should NOT flag
const μήνυμα = "hello";
const α = 1;
const β = 2;
const γ = α + β;

// Greek in comments - should NOT flag
// ελληνικά σχόλια

// Mathematical notation - should NOT flag
const θ = Math.PI / 2;
const φ = (1 + Math.sqrt(5)) / 2;
const Δ = b * b - 4 * a * c;
```

**Expected:** 0 findings  
**Actual:** 12 findings (ALL Greek letters flagged)  
**Status:** 🔴 CRITICAL BUG

### Test Case 2: Mixed Script Attack (Should flag ~4 findings)

```javascript
// Mixed script attacks - SHOULD flag
const variαble = "attack";  // α is Greek, rest Latin
const pαypal = "fake";
const vаriable = "attack";  // а is Cyrillic, rest Latin
const pаypal = "attack2";   // а is Cyrillic
```

**Expected:** 4 findings (mixed script only)  
**Actual:** 13 findings (includes comments, false positives)  
**Status:** ⚠️ Detects attacks but massive FP

---

## 🏗️ Architecture Context

### What Exists (v0.7.4)

```
crates/coax-scanner/src/unicode/
├── script_detector.rs      # ✅ EXISTS - has correct logic
│   ├── has_mixed_scripts() # ✅ Implemented
│   ├── is_pure_non_latin() # ✅ Implemented
│   └── get_script()        # ✅ Uses unicode-script crate
│
├── confusables/
│   └── data.rs             # ✅ EXISTS - 74+ confusable mappings
│
└── detectors/
    └── homoglyph.rs        # ❌ PROBLEM - doesn't USE script_detector
```

### The Root Cause

**homoglyph.rs does NOT call script_detector functions.**

It flags ALL confusable characters without checking:
1. Is this a pure non-Latin identifier? (should skip)
2. Is this a mixed-script identifier? (should flag)
3. Is this in a comment? (should skip or be lenient)

---

## 🛠️ Required Fix

### Step 1: Update homoglyph.rs

**File:** `crates/coax-scanner/src/unicode/detectors/homoglyph.rs`

**Add imports:**
```rust
use crate::unicode::script_detector::{has_mixed_scripts, is_pure_non_latin};
```

**Update detect() method:**
```rust
pub fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding> {
    let mut findings = Vec::new();

    for (line_num, line) in content.lines().enumerate() {
        // Skip comment lines (or be lenient)
        let trimmed = line.trim();
        if trimmed.starts_with("//") || 
           trimmed.starts_with("#") || 
           trimmed.starts_with("/*") ||
           trimmed.starts_with("*") {
            continue;  // Skip comments entirely
        }

        // Extract identifiers for context-aware detection
        let identifiers = extract_identifiers(line);

        for (col_num, ch) in line.chars().enumerate() {
            if let Some(match_result) = self.is_confusable(ch) {
                // Find which identifier contains this character
                let identifier = find_identifier_at_position(line, col_num);
                
                if let Some(id) = identifier {
                    // ✅ SKIP pure non-Latin (legitimate i18n)
                    if is_pure_non_latin(id) {
                        continue;  // Greek μήνυμα, Cyrillic сообщение = OK
                    }
                    
                    // ✅ Only flag mixed-script (deceptive)
                    if !has_mixed_scripts(id) {
                        continue;  // Pure script = OK
                    }
                    
                    // FLAG: Mixed script identifier (the actual attack)
                    let finding = UnicodeFinding::new(
                        file_path,
                        line_num + 1,
                        col_num + 1,
                        match_result.suspicious_char as u32,
                        match_result.suspicious_char,
                        UnicodeCategory::Homoglyph,
                        Severity::High,
                        &format!("Mixed script identifier: '{}' contains {} ({} script)",
                            id, match_result.suspicious_char, match_result.script_source),
                        "Use pure Latin or pure non-Latin identifiers, not mixed scripts",
                    )
                    .with_cwe_id("CWE-172")
                    .with_context(id);
                    
                    findings.push(finding);
                }
                // If not in identifier (e.g., in string literal), don't flag
            }
        }
    }

    findings
}

/// Extract identifiers from a line of code
fn extract_identifiers(line: &str) -> Vec<&str> {
    let identifier_pattern = regex::Regex::new(
        r"\b[a-zA-Z_$][a-zA-Z0-9_$\u0080-\uFFFF]*\b"
    ).unwrap();
    
    identifier_pattern
        .find_iter(line)
        .map(|m| m.as_str())
        .collect()
}

/// Find the identifier containing a specific character position
fn find_identifier_at_position(line: &str, char_pos: usize) -> Option<&str> {
    let identifiers = extract_identifiers(line);
    let byte_pos = line.char_indices().nth(char_pos)?.0;
    
    for id in identifiers {
        if let Some(start) = line.find(id) {
            let end = start + id.len();
            if byte_pos >= start && byte_pos < end {
                return Some(id);
            }
        }
    }
    
    None
}
```

### Step 2: Fix Confusables Database

**File:** `crates/coax-scanner/src/unicode/confusables/data.rs`

**Remove IPA entries** that cause absurd false positives:
```rust
// REMOVE entries like:
// ('k', 'k', "IPA")  // ASCII 'k' confusable with itself? Absurd.
```

### Step 3: Add Integration Tests

**File:** `crates/coax-scanner/tests/unicode_tests.rs`

```rust
#[test]
fn test_greek_legitimate_no_false_positives() {
    let scanner = UnicodeScanner::with_default_config();
    let content = r#"
        const μήνυμα = "hello";
        const α = 1;
        const β = 2;
        const γ = α + β;
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();
    
    assert_eq!(homoglyph_findings.len(), 0, 
        "Pure Greek identifiers should not be flagged");
}

#[test]
fn test_mixed_script_attack_detection() {
    let scanner = UnicodeScanner::with_default_config();
    let content = r#"
        const variαble = "attack";
        const pαypal = "fake";
        const vаriable = "attack2";
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();
    
    assert!(homoglyph_findings.len() >= 3, 
        "Mixed script identifiers should be flagged");
}

#[test]
fn test_comments_not_flagged() {
    let scanner = UnicodeScanner::with_default_config();
    let content = r#"
        // ελληνικά σχόλια - Greek comments
        // comment with α beta γ
    "#;
    let findings = scanner.scan(content, "test.js");
    let homoglyph_findings: Vec<_> = findings.iter()
        .filter(|f| f.category == UnicodeCategory::Homoglyph)
        .collect();
    
    assert_eq!(homoglyph_findings.len(), 0, 
        "Comments should not be flagged");
}
```

---

## 🧪 Verification Commands

```bash
# Build release binary
cargo build --release

# Test 1: Pure Greek (should be 0 findings)
cat > /tmp/greek_legit.js << 'EOF'
const μήνυμα = "hello";
const α = 1;
const β = 2;
const γ = α + β;
EOF

./target/release/coax scan -p /tmp/greek_legit.js --unicode-only
# Expected: ✅ No findings

# Test 2: Mixed attack (should flag ~4 findings)
cat > /tmp/mixed_attack.js << 'EOF'
const variαble = "attack";
const pαypal = "fake";
const vаriable = "attack2";
const pаypal = "attack3";
EOF

./target/release/coax scan -p /tmp/mixed_attack.js --unicode-only
# Expected: ⚠️ 4+ findings (high severity)

# Test 3: All tests pass
cargo test -p coax-scanner unicode -- --nocapture
# Expected: 100% passing
```

---

## 📊 Success Criteria for v0.7.5

| Metric | v0.7.4 (Current) | v0.7.5 (Target) |
|--------|------------------|-----------------|
| Greek FP Rate | 100% (12/12 flagged) | 0% (0/12 flagged) |
| Mixed Attack Detection | 100% | 100% |
| Comment FP Rate | 100% | 0% |
| Overall FP Rate | ~50% | <1% |
| Test Coverage | 99.1% | 99.5% |

---

## 📁 Key Files to Review

| File | Purpose | Status |
|------|---------|--------|
| `crates/coax-scanner/src/unicode/script_detector.rs` | Script mixing logic | ✅ Exists, correct |
| `crates/coax-scanner/src/unicode/detectors/homoglyph.rs` | Homoglyph detection | ❌ Needs fix |
| `crates/coax-scanner/src/unicode/confusables/data.rs` | Confusables database | ⚠️ Remove IPA entries |
| `docs/HANDOFF.md` | Agent handoff docs | ✅ Current (v0.7.4) |
| `docs/QA-TESTING-PLAN-v0.7.4.md` | QA test plan | ✅ Exists |
| `qa/greek_legitimate_test.js` | Greek test case | ✅ Exists |
| `qa/mixed_script_attack_test.js` | Mixed attack test | ✅ Exists |

---

## 🎯 v0.7.5 Timeline

| Task | Time | Priority |
|------|------|----------|
| Fix homoglyph.rs integration | 2-3 hours | 🔴 P0 |
| Fix confusables database | 30 min | 🟡 P1 |
| Add integration tests | 1 hour | 🟡 P1 |
| Run full QA suite | 2-3 hours | 🟡 P1 |
| Update release notes | 30 min | 🟢 P2 |
| **Total** | **5-7 hours** | |

---

## 📚 Context from Previous Agent

### What Worked Well (v0.7.4)
- ✅ Unicode module architecture (5 detectors)
- ✅ script_detector.rs implementation (correct logic)
- ✅ Confusables database (74+ mappings)
- ✅ Test infrastructure

### What Needs Fix (v0.7.5)
- ❌ homoglyph.rs doesn't use script_detector
- ❌ Comments being scanned
- ❌ IPA confusables causing false positives
- ❌ Identifier extraction not implemented

### Known Issues Documented
- HANDOFF.md: "Script mixing detection needs refinement (planned for v0.7.5)"
- HANDOFF.md: "Need to check identifiers, not entire lines"

---

## 🚀 After v0.7.5 Complete

### v0.8.0: VS Code Extension (Ready to Start)
- **Spec:** `docs/VSCode-EXTENSION-SPEC.md`
- **Timeline:** 4-5 weeks
- **Features:** Real-time scanning, inline warnings, quick-fix actions

### v0.9.0: Threat Intelligence (After v0.8.0)
- Auto-pattern updates
- CVE integration
- Community pattern submissions

---

## 📞 Questions to Clarify

1. Should Cyrillic be treated same as Greek (allow pure, flag mixed)?
2. Should we add a `--unicode-strict-mode` flag for security-critical projects?
3. Should i18n files (locales/, translations/) be excluded entirely?

---

## ✅ Completion Checklist

- [ ] homoglyph.rs imports and uses script_detector functions
- [ ] Pure Greek test: 0 findings
- [ ] Mixed attack test: 4+ findings
- [ ] Comment test: 0 findings
- [ ] All unit tests pass (100%)
- [ ] QA testing on 5+ real repos
- [ ] RELEASE-NOTES-v0.7.5.md created
- [ ] HANDOFF.md updated to v0.7.5
- [ ] Ready for v0.8.0 VS Code Extension

---

**Ready to begin? Start by reviewing:**
1. `crates/coax-scanner/src/unicode/script_detector.rs` (existing logic)
2. `crates/coax-scanner/src/unicode/detectors/homoglyph.rs` (needs fix)
3. `qa/greek_legitimate_test.js` (test case)

**First task:** Implement the fix in homoglyph.rs and verify Greek test passes.
```

