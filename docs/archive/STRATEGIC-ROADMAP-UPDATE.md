# Strategic Roadmap Update - v0.7.0 to v0.9.0

**Date:** 2026-03-16  
**Based on:** QAANDSTRATEGY.md analysis  
**Status:** ✅ APPROVED

---

## Executive Summary

Based on comprehensive market research and QA results, we're pivoting the roadmap to prioritize **IDE integration** over ML-based features. This aligns with market reality:

1. **Unicode detection is our differentiator** - Gitleaks/TruffleHog don't have it
2. **IDE integration drives adoption** - 85% of devs use VS Code
3. **ML requires training data** - We need 10K+ scans first
4. **Threat intel is table stakes** - Can be added after IDE plugin

---

## Updated Roadmap

### v0.7.0: Polish & QA Integration (2 weeks)
**Status:** IN PROGRESS

**Tasks:**
- [x] Unicode panic bug fixed
- [x] QA testing complete (231/233 tests passing)
- [ ] Complete `--unicode-only` flag (1 day)
- [ ] Fix Greek i18n false positives (2 days)
- [ ] Performance optimization (2 days)
- [ ] Documentation updates (2 days)

**Success Criteria:**
- 100% test coverage on Unicode detection
- Zero false positives on legitimate i18n
- Detection parity with anti-trojan-source

---

### v0.8.0: VS Code Extension (4-5 weeks) - NEW P0 PRIORITY
**Status:** SPEC COMPLETE, READY FOR DEV

**Features:**
- Real-time scanning on file save
- Inline squiggly underlines
- Problems panel integration
- Quick-fix actions (remove, replace, ignore)
- Status bar indicator
- Command palette commands

**Timeline:**
- Week 1: Project setup & binary bundling
- Week 2: Core scanning & diagnostics
- Week 3: Inline warnings & status
- Week 4: Quick-fix actions
- Week 5: Polish & Marketplace release

**Resources:**
- 1 Rust/TypeScript developer
- docs/VSCode-EXTENSION-SPEC.md (complete)
- docs/VSCode-EXTENSION-TIMELINE.md (complete)

**Success Criteria:**
- Install from VS Code Marketplace
- Scan on file save works
- Findings show in Problems panel
- Inline warnings visible
- Quick-fix actions work

---

### v0.9.0: Threat Intelligence (2-3 weeks)
**Status:** PLANNED

**Features:**
- Auto-update pattern database
- CVE integration
- Glassworm pattern feed
- Community-sourced patterns

**Timeline:** After VS Code Extension

---

### v1.0.0: ML Classifier (R&D, 8-12 weeks)
**Status:** DEFERRED

**Reason:** Requires training data we don't have yet. Wait until we have 10K+ scans from VS Code Extension users.

**When to Start:**
- After 10K+ scans collected
- After 1K+ labeled findings
- After v0.9.0 release

---

## Why This Order?

| Priority | Rationale |
|----------|-----------|
| **1. IDE Plugin First** | 85% of developers use VS Code. CLI-only tools have limited adoption. IDE integration = daily usage = word-of-mouth growth. |
| **2. Threat Intelligence** | Glassworm patterns evolve. Auto-updates keep Coax relevant without manual pattern maintenance. |
| **3. ML Classifier Last** | Requires training data you don't have yet. Wait until you have 10K+ scans to learn from. |

---

## Market Validation

### Competitive Landscape

| Tool | Unicode Detection | IDE Plugin | Threat Intel | ML Classifier |
|------|------------------|------------|--------------|---------------|
| **Coax** | ✅ YES | 🔄 v0.8.0 | ⏳ v0.9.0 | ⏳ v1.0.0 |
| Gitleaks | ❌ NO | ❌ NO | ❌ NO | ❌ NO |
| TruffleHog | ❌ NO | ❌ NO | ✅ YES | ❌ NO |
| GitGuardian | ✅ YES | ✅ YES | ✅ YES | ✅ YES |
| Aikido | ✅ YES | ✅ YES | ✅ YES | ✅ YES |
| anti-trojan-source | ✅ YES | ❌ NO | ❌ NO | ❌ NO |

**Our Niche:** Unicode detection + IDE integration (unique combination)

---

## QA Results Summary

**v0.6.2 QA Report:** `qa/QA-REPORT-v0.6.2.md`

**Results:**
- anti-trojan-source: 197 findings detected ✅
- Glassworm patterns: 100% detection ✅
- Homoglyphs: 100% detection ✅
- Bidirectional overrides: 100% detection ✅
- Performance: 54ms for 10K lines (82% faster than target) ✅
- False positives: 0% on emoji/CJK, ~100% on Greek i18n ⚠️

**Overall Verdict:** PASS - Ready for production with documented limitations

---

## Next Steps

### Immediate (This Week)
1. Fix Greek i18n false positives
2. Complete `--unicode-only` flag
3. Update documentation
4. Release v0.7.0

### Short-term (Next 4-5 weeks)
1. Initialize VS Code extension project
2. Bundle coax binaries for all platforms
3. Implement core scanning
4. Add inline warnings
5. Implement quick-fixes
6. Submit to Marketplace

### Medium-term (8-12 weeks)
1. Collect scan data from users
2. Build labeled dataset
3. Research ML approaches
4. Start ML classifier R&D

---

## Resources

- `docs/VSCode-EXTENSION-SPEC.md` - Complete specification
- `docs/VSCode-EXTENSION-TIMELINE.md` - Week-by-week plan
- `docs/research/vscode-extension-research.md` - Technical research
- `qa/QA-REPORT-v0.6.2.md` - QA results
- `VSCode-EXTENSION-SUMMARY.md` - Executive summary

---

**Approved by:** Security Architecture Analysis  
**Date:** 2026-03-16  
**Next Review:** v0.7.0 release (2 weeks)
