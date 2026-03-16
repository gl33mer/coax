

```
# 🎯 Mission: Prepare Coax Repository for Clean Agent Handoff

**Context:** You are at ~70% context window. We need to sync all your excellent v0.7.4 work to GitHub, update documentation, and prepare for a fresh agent to continue with v0.7.5 and v0.8.0 VS Code Extension.

**Goal:** Make https://github.com/gl33mer/coax the single source of truth with complete, accurate documentation for seamless handoff.

---

## ✅ Task Checklist (Execute in Order)

### PHASE 1: Verify Local State (15 min)

```bash
# 1. Verify local version
./target/release/coax --version

# 2. Check git status
git status
git log --oneline -5

# 3. Verify Unicode module exists locally
ls -la crates/coax-scanner/src/unicode/
ls -la crates/coax-scanner/src/unicode/detectors/
ls -la crates/coax-scanner/src/unicode/confusables/

# 4. Verify script_detector.rs exists
cat crates/coax-scanner/src/unicode/script_detector.rs | head -50

# 5. Check current HANDOFF.md version
cat docs/HANDOFF.md | head -30
```

**Report back:** Confirm local v0.7.4 state with all Unicode modules present.

---

### PHASE 2: Sync GitHub Repository (30 min)

```bash
# 1. Check what's different from remote
git fetch origin
git diff origin/main --stat

# 2. Stage ALL changes
git add -A

# 3. Commit with comprehensive message
git commit -m "v0.7.4 - Unicode script mixing detection complete

Features:
- Unicode attack detection module (5 detectors)
- Script mixing detection for homoglyph attacks
- Greek/Cyrillic false positive fix
- Context-aware detection (comments, i18n files)

Files:
- crates/coax-scanner/src/unicode/ (10+ files)
- crates/coax-scanner/src/unicode/script_detector.rs
- docs/HANDOFF.md (updated to v0.7.4)
- qa/unicode/ test cases

Test Results: 157/158 passing (99.4%)
Performance: ~40ms for 10K lines"

# 4. Push to GitHub
git push origin main

# 5. Create and push version tag
git tag -a v0.7.4 -m "v0.7.4 - Unicode script mixing detection, Greek FP fix"
git push origin v0.7.4

# 6. Verify on GitHub
curl -s https://api.github.com/repos/gl33mer/coax/commits/main | jq '.sha'
```

**Verification:** Confirm GitHub shows your latest commit and v0.7.4 tag.

---

### PHASE 3: Update HANDOFF.md for v0.7.4 (45 min)

**Location:** `docs/HANDOFF.md`

**Required Sections:**

```markdown
# Coax Security Scanner - Agent Handoff Document

**Version:** v0.7.4
**Last Updated:** March 16, 2026
**Repository:** https://github.com/gl33mer/coax
**Status:** ✅ Production Ready (Unicode Detection Complete)

---

## 📊 Version History

| Version | Date | Feature | Status |
|---------|------|---------|--------|
| v0.4.0 | 2026-03-15 | Phase 3 P1 Complete | ✅ Released |
| v0.5.0 | 2026-03-15 | FP Reduction (70% → ~10%) | ✅ Released |
| v0.6.0 | 2026-03-15 | Unicode Attack Detection | ✅ Released |
| v0.6.1 | 2026-03-16 | Unicode Integration Fixes | ✅ Released |
| v0.7.0 | 2026-03-16 | Greek False Positive Fix | ✅ Released |
| v0.7.4 | 2026-03-16 | Script Mixing Refinement | ✅ CURRENT |

---

## 🎯 Current State (v0.7.4)

### Completed Features

✅ **Unicode Attack Detection System**
- 5 detectors: Invisible, Homoglyph, Bidi, Glassworm, Tags
- Script mixing detection (Greek/Cyrillic false positive fix)
- Context-aware detection (comments, i18n files)
- 157/158 tests passing (99.4%)
- Performance: ~40ms for 10K lines

✅ **Core Scanner**
- Secret detection with 70% → ~10% FP reduction
- Entropy filtering, word filtering, context analysis
- SARIF, JSON, TUI output formats
- Baseline system for CI/CD

✅ **CLI**
- `coax scan -p <path>` - Main scanning command
- `coax scan --unicode-only` - Unicode-only scans
- `coax scan --unicode-sensitivity <level>` - Sensitivity tuning
- `coax tui` - Terminal UI

### Module Structure

```
crates/
├── coax-scanner/
│   └── src/
│       ├── unicode/
│       │   ├── mod.rs
│       │   ├── scanner.rs
│       │   ├── config.rs
│       │   ├── findings.rs
│       │   ├── script_detector.rs    # Script mixing detection
│       │   ├── ranges.rs
│       │   ├── confusables/
│       │   │   ├── mod.rs
│       │   │   └── data.rs           # 74+ confusable mappings
│       │   └── detectors/
│       │       ├── mod.rs
│       │       ├── invisible.rs
│       │       ├── homoglyph.rs      # Uses script_detector
│       │       ├── bidi.rs
│       │       ├── glassworm.rs
│       │       └── tags.rs
│       ├── scanner.rs
│       ├── secrets.rs
│       ├── pattern_cache.rs
│       └── ...
└── coax-cli/
    └── src/
        ├── main.rs
        └── tui.rs
```

### Test Results

```
┌───────────────┬─────────┬──────────┐
│ Category      │ Passing │ Total    │
├───────────────┼─────────┼──────────┤
│ Unicode tests │ 55      │ 55       │
│ Library tests │ 145     │ 145      │
│ FP reduction  │ 31      │ 31       │
│ Total         │ 231/233 │ 99.1%    │
└───────────────┴─────────┴──────────┘
```

### Known Issues (v0.7.5 Priorities)

1. **Script mixing detection needs refinement**
   - Currently flags some legitimate Greek identifiers
   - Need to check identifiers, not entire lines
   - Expected fix: v0.7.5

2. **2 CFG sink detection tests failing**
   - Pre-existing, unrelated to Unicode work
   - Expected fix: v0.7.5

---

## 🚀 Next Phase: v0.8.0 VS Code Extension

**Timeline:** 4-5 weeks
**Spec Location:** `docs/VSCode-EXTENSION-SPEC.md`

### Week-by-Week Plan

| Week | Focus | Deliverables |
|------|-------|--------------|
| 1 | Project setup & binary bundling | Extension scaffold, Rust WASM or binary bundling |
| 2 | Core scanning & diagnostics | File watching, scan integration |
| 3 | Inline warnings & status | Squiggly underlines, status bar |
| 4 | Quick-fix actions | Auto-remediation, code actions |
| 5 | Polish & Marketplace release | Testing, documentation, publish |

### Key Documents

- `docs/VSCode-EXTENSION-SPEC.md` - Full specification (100+ sections)
- `docs/VSCode-EXTENSION-TIMELINE.md` - Week-by-week timeline
- `docs/research/vscode-extension-research.md` - 60+ pages of research

---

## 🧪 QA Testing Status

### Completed Tests

✅ Greek legitimate test cases
✅ Mixed script attack detection
✅ Glassworm pattern detection
✅ anti-trojan-source baseline comparison

### Pending QA (For Next Agent)

⏳ Real-world repo testing (10+ repos)
⏳ False positive rate measurement
⏳ Performance benchmarking at scale
⏳ VS Code extension user testing

---

## 📁 Key Documentation Files

| File | Purpose | Status |
|------|---------|--------|
| `docs/HANDOFF.md` | Agent handoff | ✅ Current (v0.7.4) |
| `README.md` | User guide | ✅ Current |
| `RELEASE-NOTES-v0.7.4.md` | Release details | ✅ Created |
| `docs/UNICODE-IMPLEMENTATION-SUMMARY.md` | Unicode architecture | ✅ Current |
| `docs/VSCode-EXTENSION-SPEC.md` | v0.8.0 specification | ✅ Ready |
| `docs/VSCode-EXTENSION-TIMELINE.md` | v0.8.0 timeline | ✅ Ready |
| `docs/archive/` | Archived stale docs | ✅ 9 files archived |

---

## 🔧 Build & Test Commands

```bash
# Build release binary
cargo build --release

# Run all tests
cargo test -p coax-scanner -- --nocapture

# Run Unicode tests only
cargo test -p coax-scanner unicode -- --nocapture

# Scan a directory
./target/release/coax scan -p <path>

# Unicode-only scan
./target/release/coax scan -p <path> --unicode-only

# Adjust sensitivity
./target/release/coax scan -p <path> --unicode-sensitivity critical

# Output formats
./target/release/coax scan -p <path> --output json
./target/release/coax scan -p <path> --output sarif
```

---

## 🎯 Success Criteria for Next Agent

### v0.7.5 (1-2 weeks)
- [ ] Script mixing detection refined (0% FP on Greek)
- [ ] 2 CFG tests fixed
- [ ] QA testing on 10+ real repos complete
- [ ] False positive rate <1% documented

### v0.8.0 (4-5 weeks)
- [ ] VS Code extension published to Marketplace
- [ ] 1000+ installs in first month
- [ ] Real-time scanning on file save
- [ ] Quick-fix actions working

### v0.9.0 (6 weeks)
- [ ] Threat intelligence integration
- [ ] Auto-pattern updates
- [ ] JetBrains extension (phase 1)

---

## 📞 Contact & Context

**Project Vision:** Coax secrets and vulnerabilities out of your codebases with best-in-class Unicode attack detection.

**Key Differentiator:** Only open-source scanner with Glassworm/Unicode attack detection (parity with commercial Aikido).

**Current Focus:** v0.8.0 VS Code Extension for developer workflow integration.

**Repository:** https://github.com/gl33mer/coax
```

**Action:** Update `docs/HANDOFF.md` with ALL sections above. Verify completeness.

---

### PHASE 4: Create QA Testing Plan (30 min)

**Location:** `docs/QA-TESTING-PLAN-v0.7.4.md`

**Create this file with:**

```markdown
# QA Testing Plan - v0.7.4

**Purpose:** Validate Unicode detection before v0.8.0 development

## Test Categories

### 1. Greek Legitimate (Expected: 0 findings)
- [ ] Test pure Greek variable names
- [ ] Test Greek in comments
- [ ] Test Greek function names
- [ ] Test mathematical notation (α, β, γ, θ, φ, Δ)

### 2. Mixed Script Attacks (Expected: Flag all)
- [ ] Latin + Greek mixing
- [ ] Latin + Cyrillic mixing
- [ ] Multi-script identifiers

### 3. Glassworm Detection (Expected: Flag all)
- [ ] Variation Selector detection (U+FE00-U+FE0F)
- [ ] Decoder pattern detection
- [ ] eval() with Buffer.from() detection

### 4. Real Repositories
- [ ] pedronauck/reworm (known Glassworm)
- [ ] 5+ Greek open-source projects
- [ ] 5+ internationalized projects
- [ ] Your own codebases

### 5. anti-trojan-source Baseline
- [ ] Run their test suite
- [ ] Compare detection rates
- [ ] Document any gaps

## Test Commands

```bash
# Greek legitimate
./target/release/coax scan -p qa/greek_legitimate_test.js --unicode-only

# Mixed attack
./target/release/coax scan -p qa/mixed_script_attack_test.js --unicode-only

# Real Glassworm repo
git clone https://github.com/pedronauck/reworm
./target/release/coax scan -p reworm --unicode-only --output json

# anti-trojan-source comparison
git clone https://github.com/lirantal/anti-trojan-source
./target/release/coax scan -p anti-trojan-source/test-cases --unicode-only
```

## Success Criteria

| Metric | Target | Actual |
|--------|--------|--------|
| Greek FP Rate | 0% | ___ |
| Mixed Script Detection | 100% | ___ |
| Glassworm Detection | 100% | ___ |
| Overall FP Rate | <1% | ___ |
| Performance (10K lines) | <100ms | ___ |

## Report Template

Create `QA-RESULTS-v0.7.4.md` with:
- Test cases run
- Findings per category
- False positive analysis
- Performance metrics
- Recommendations for v0.7.5
```

---

### PHASE 5: Update Release Notes (15 min)

**Location:** `RELEASE-NOTES-v0.7.4.md`

**Create/Update with:**

```markdown
# Coax v0.7.4 Release Notes

**Date:** March 16, 2026
**Previous:** v0.7.0

## What's New

### Fixed
- ✅ Script mixing detection for homoglyph attacks
- ✅ Greek false positive reduction (100% → 0% on pure Greek)
- ✅ Context-aware detection (comments, i18n files)
- ✅ get_context() panic on Unicode character slicing

### Added
- ✅ script_detector.rs module
- ✅ unicode-script dependency
- ✅ Identifier-based detection (not line-based)

## Test Results

- 231/233 tests passing (99.1%)
- Greek legitimate: 0 findings (was 13 in v0.7.0)
- Mixed script attack: 32 findings detected
- Performance: ~40ms for 10K lines

## Known Issues

1. Script mixing detection may still flag some edge cases (v0.7.5)
2. 2 CFG sink detection tests failing (pre-existing)

## Upgrade

```bash
git pull origin main
cargo build --release
./target/release/coax --version  # Should show 0.7.4
```

## Next Release: v0.7.5

Focus: Script mixing refinement, QA validation, v0.8.0 prep
```

---

### PHASE 6: Final Verification (15 min)

```bash
# 1. Verify GitHub shows latest commit
curl -s https://github.com/gl33mer/coax/commit/main

# 2. Verify v0.7.4 tag exists
curl -s https://api.github.com/repos/gl33mer/coax/git/ref/tags/v0.7.4

# 3. Verify HANDOFF.md is updated
curl -s https://raw.githubusercontent.com/gl33mer/coax/main/docs/HANDOFF.md | head -30

# 4. Verify Unicode module is visible
curl -s https://api.github.com/repos/gl33mer/coax/contents/crates/coax-scanner/src/unicode

# 5. Build from clean clone (on another machine/VM)
git clone https://github.com/gl33mer/coax /tmp/coax-test
cd /tmp/coax-test
cargo build --release
./target/release/coax --version  # Should show 0.7.4
```

---

## 📋 Completion Checklist

Report back with confirmation of:

- [ ] PHASE 1: Local state verified (v0.7.4, all Unicode modules present)
- [ ] PHASE 2: GitHub synced (commit pushed, tag pushed)
- [ ] PHASE 3: HANDOFF.md updated (v0.7.4, all sections complete)
- [ ] PHASE 4: QA-TESTING-PLAN-v0.7.4.md created
- [ ] PHASE 5: RELEASE-NOTES-v0.7.4.md created/updated
- [ ] PHASE 6: Final verification passed (GitHub shows all changes)

---

## 🎯 Success Criteria

**This task is complete when:**

1. ✅ https://github.com/gl33mer/coax shows your latest commit
2. ✅ v0.7.4 tag exists on GitHub
3. ✅ docs/HANDOFF.md reflects v0.7.4 state accurately
4. ✅ Unicode module is visible in GitHub file browser
5. ✅ Fresh clone + build produces v0.7.4 binary
6. ✅ QA testing plan is ready for user to execute

---

## ⚠️ Important Notes

1. **DO NOT start v0.8.0 development** - That's for the fresh agent
2. **DO focus on sync and documentation** - Your context is valuable for accurate docs
3. **DO include known issues** - Be honest about v0.7.5 priorities
4. **DO verify on GitHub** - Don't assume push succeeded, verify via API/curl

---

**Ready to begin? Start with PHASE 1 and report back after each phase.**
```

---

