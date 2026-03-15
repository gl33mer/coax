# Advanced Secret Detection - Research Recommendations

**Research Date:** March 15, 2026  
**Researcher:** Coax Research Team  
**Status:** Ready for Phase 2 Implementation

---

## Executive Summary

This research analyzed advanced secret detection strategies from industry leaders (Betterleaks, GitGuardian, TruffleHog) and open-source pattern databases. The findings provide a clear roadmap for improving Coax's detection accuracy and coverage.

### Key Findings

| Area | Current Coax | Research Recommendation | Expected Impact |
|------|--------------|------------------------|-----------------|
| **Pattern Count** | 43 patterns | 1,600+ patterns | **37x coverage increase** |
| **False Positive Rate** | ~15% (estimated) | <5% | **60-70% reduction** |
| **Detection Accuracy (F1)** | ~0.80 (estimated) | ~0.89 | **+10-15% improvement** |
| **Pattern Updates** | Code changes required | Automatic (YAML) | **No rebuild needed** |

---

## Research Deliverables

### Files Created

All research files are located in `/home/shva/QwenDev/devshield-internal/coax/docs/research/`:

1. **betterleaks-analysis.md** - Complete analysis of Betterleaks detection strategies
2. **secrets-patterns-db-analysis.md** - Analysis of secrets-patterns-db integration
3. **entropy-detection-research.md** - Research on entropy detection best practices
4. **modular-pattern-system-proposal.md** - Architecture proposal for modular patterns
5. **recommendations.md** - This summary document

---

## Prioritized Recommendations

### P0: Immediate Implementation (Week 1-2)

#### 1. Token Efficiency Filter

**What:** Implement BPE-based token efficiency filtering (from Betterleaks)

**Why:** 
- Reduces false positives by **60-70%**
- F1 score improvement: **0.325 → 0.725** (123% improvement)
- Catches short secrets that entropy misses

**How:**
```rust
// Add tiktoken-go dependency
// Implement fails_token_efficiency_filter()
// Apply as post-regex filter
```

**Effort:** 2-3 days  
**Impact:** High  
**Risk:** Low

---

#### 2. Word Filter (Dictionary-Based)

**What:** Aho-Corasick trie-based dictionary matching

**Why:**
- Reduces false positives by **68%** (when combined with TE)
- Fast (~2 µs/string)
- Filters natural language false positives

**How:**
```rust
// Add aho-corasick dependency
// Build trie from NLTK word list (777KB)
// Filter secrets containing 4+ char dictionary words
```

**Effort:** 1-2 days  
**Impact:** High  
**Risk:** Low

---

#### 3. Secrets-Patterns-DB Integration (High-Confidence Only)

**What:** Integrate high-confidence patterns from secrets-patterns-db

**Why:**
- Increases pattern count from **43 → 400+** (high-confidence only)
- Broader service coverage
- Community-maintained patterns

**How:**
```bash
# Download high-confidence patterns
git clone https://github.com/mazen160/secrets-patterns-db.git
# Filter to high-confidence only
# Convert to YAML format
# Load via pattern manager
```

**Effort:** 2-3 days  
**Impact:** High  
**Risk:** Low (high-confidence patterns only)

---

### P1: Short-Term Implementation (Week 3-4)

#### 4. Modular Pattern System

**What:** YAML/JSON-based pattern loading system

**Why:**
- No code changes required for new patterns
- User-customizable patterns
- Automatic pattern updates
- Pattern versioning

**How:**
```rust
// Implement PatternManager
// Add serde_yaml dependency
// Load patterns from:
//   - Built-in (compiled)
//   - External databases (YAML)
//   - User custom patterns
```

**Effort:** 3-4 days  
**Impact:** High  
**Risk:** Medium

---

#### 5. Entropy Threshold Modification

**What:** Reduce entropy threshold from 3.5 to 3.0, apply only to generic patterns

**Why:**
- Catches more short secrets
- Reduces false negatives
- Keep as secondary filter to token efficiency

**How:**
```rust
// Update default threshold: 3.5 → 3.0
// Apply only to GENERIC patterns
// Keep as secondary filter (after TE)
```

**Effort:** 1 day  
**Impact:** Medium  
**Risk:** Low

---

#### 6. Adaptive Thresholds

**What:** Lower threshold (2.1) for short strings <12 chars

**Why:**
- Better detection of short secrets
- Based on Betterleaks research
- Minimal implementation effort

**How:**
```rust
let threshold = if secret.len() < 12 {
    2.1  // Stricter for short strings
} else {
    2.5  // Default threshold
};
```

**Effort:** 1 day  
**Impact:** Medium  
**Risk:** Low

---

### P2: Medium-Term Implementation (Month 2)

#### 7. Full Secrets-Patterns-DB Integration

**What:** Integrate all 1,600+ patterns (not just high-confidence)

**Why:**
- Maximum pattern coverage
- Detects rare/exotic secret types
- Community-maintained

**How:**
```rust
// Load full rules-stable.yml
// Add confidence-based filtering
// Allow users to enable medium/low confidence patterns
```

**Effort:** 2 days  
**Impact:** Medium  
**Risk:** Medium (higher FP rate)

---

#### 8. Automatic Pattern Updates

**What:** Automated pattern database updates

**Why:**
- Stay current with new secret types
- No manual intervention required
- Version tracking

**How:**
```rust
// Implement PatternUpdater
// Download from GitHub weekly
// Validate before applying
// Track versions
```

**Effort:** 2-3 days  
**Impact:** Medium  
**Risk:** Low

---

#### 9. CEL Validation (Live Secret Verification)

**What:** HTTP-based secret validation (from Betterleaks)

**Why:**
- Verify if detected secrets are actually live
- Reduce false positives from revoked/expired secrets
- Priority scoring based on validation status

**How:**
```rust
// Add CEL engine dependency
// Define validation expressions per-pattern
// Make HTTP requests to verify secrets
// Filter by validation status
```

**Effort:** 5-7 days  
**Impact:** Medium  
**Risk:** Medium (network dependencies)

---

## Implementation Roadmap

### Phase 2A: Core Improvements (Week 1-2)

```
Week 1:
├── Day 1-2: Token Efficiency implementation
├── Day 3-4: Word Filter implementation
└── Day 5: Testing and tuning

Week 2:
├── Day 1-2: Secrets-patterns-db integration (high-confidence)
├── Day 3-4: Pattern merging and deduplication
└── Day 5: False positive testing
```

**Deliverables:**
- Token efficiency filter working
- Word filter working
- 400+ high-confidence patterns loaded
- FP rate reduced by 60%+

---

### Phase 2B: Modular System (Week 3-4)

```
Week 3:
├── Day 1-2: Pattern schema definition
├── Day 3-4: PatternManager implementation
└── Day 5: YAML loading support

Week 4:
├── Day 1-2: CLI commands for pattern management
├── Day 3-4: Entropy threshold modification
└── Day 5: Integration testing
```

**Deliverables:**
- Modular pattern system working
- User custom patterns supported
- CLI commands for management
- Automatic updates ready

---

## Expected Results

### Detection Accuracy

| Metric | Current | After P0 | After P1 | Target |
|--------|---------|----------|----------|--------|
| **Precision** | ~85% | ~91% | ~93% | 95% |
| **Recall** | ~75% | ~87% | ~90% | 92% |
| **F1 Score** | ~0.80 | ~0.89 | ~0.91 | 0.93 |
| **False Positives** | Baseline | -60% | -70% | -75% |

### Pattern Coverage

| Category | Current | After P0 | After P1 |
|----------|---------|----------|----------|
| **Total Patterns** | 43 | 400+ | 1,600+ |
| **Cloud Providers** | 6 | 50+ | 150+ |
| **Version Control** | 5 | 20+ | 40+ |
| **Payment** | 4 | 20+ | 50+ |
| **Communication** | 6 | 30+ | 80+ |
| **Database** | 5 | 25+ | 60+ |
| **AI/ML** | 3 | 10+ | 30+ |
| **Generic** | 3 | 50+ | 200+ |

### Performance Impact

| Operation | Current | After P0 | After P1 |
|-----------|---------|----------|----------|
| **Pattern Load Time** | N/A | 50-100ms | 100-200ms |
| **Scan Performance** | Baseline | -5% | -10% |
| **Memory Usage** | ~100KB | ~2MB | ~5MB |

---

## Risk Assessment

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Token efficiency performance overhead | Low | Low | Benchmark, optimize if needed |
| Pattern database format changes | Low | Medium | Version pinning, validation |
| ReDoS vulnerabilities in patterns | Medium | High | Pattern validation, testing |
| False positive increase with more patterns | Medium | Medium | Confidence-based filtering |

### Mitigation Strategies

1. **Performance Testing**: Benchmark after each change
2. **Pattern Validation**: Validate all patterns before use
3. **Gradual Rollout**: Start with high-confidence patterns only
4. **FP Monitoring**: Track false positive rates per pattern
5. **Rollback Support**: Ability to revert pattern updates

---

## Testing Strategy

### False Positive Test Suite

**Test Files:**
```
qa/test-repos/false-positive-tests/
├── src/
│   ├── clean_code.py         # Should have 0 findings
│   ├── config_examples.py    # Should have 0 findings
│   └── documentation.md      # Should have 0 findings
├── actual_secrets.py          # Should detect all secrets
└── edge_cases/
    ├── uuids.py              # UUIDs (not secrets)
    ├── base64_data.py        # Base64 encoded data
    └── hashes.py             # Hash values
```

**Expected Results:**
- Clean code: 0 findings
- Actual secrets: All detected
- Edge cases: Correctly classified

### Performance Benchmarks

**Test Scenarios:**
1. Small codebase (100 files)
2. Medium codebase (1,000 files)
3. Large codebase (10,000 files)
4. Git history scanning

**Metrics:**
- Scan time
- Memory usage
- CPU usage
- Findings accuracy

---

## Success Criteria

### Phase 2A (P0 Items)

- [ ] Token efficiency filter implemented and tested
- [ ] Word filter implemented and tested
- [ ] 400+ high-confidence patterns loaded
- [ ] False positive rate reduced by 60%+
- [ ] F1 score improved to 0.89+
- [ ] No performance regression >10%

### Phase 2B (P1 Items)

- [ ] Modular pattern system implemented
- [ ] User custom patterns supported
- [ ] CLI commands for pattern management
- [ ] Entropy threshold modified
- [ ] 1,600+ patterns available
- [ ] Pattern versioning implemented

### Overall Phase 2

- [ ] All P0 and P1 items completed
- [ ] Detection accuracy (F1) > 0.90
- [ ] False positive rate < 5%
- [ ] Pattern updates available via CLI
- [ ] Documentation complete
- [ ] User guide for custom patterns

---

## Resource Requirements

### Development Resources

- **Developers:** 2 engineers for 4 weeks
- **QA:** 1 engineer for 2 weeks (testing)
- **Total Effort:** ~10 engineer-weeks

### Infrastructure Resources

- **GitHub Repository:** For pattern hosting (new repo: `coax/scanner-patterns`)
- **CDN/Storage:** For pattern distribution (GitHub Releases)
- **CI/CD:** Pattern validation pipeline

### External Dependencies

| Dependency | Purpose | License |
|------------|---------|---------|
| `tiktoken-go` | BPE tokenization | MIT |
| `aho-corasick` | Word filter | MIT |
| `serde_yaml` | YAML loading | MIT/Apache-2.0 |
| `reqwest` | Pattern updates | MIT/Apache-2.0 |
| `secrets-patterns-db` | Pattern source | CC BY-SA 4.0 |

---

## Conclusion

This research provides a clear, actionable roadmap for significantly improving Coax's secret detection capabilities. The recommended changes will:

1. **Increase pattern coverage by 37x** (43 → 1,600+ patterns)
2. **Reduce false positives by 60-70%** (token efficiency + word filter)
3. **Improve detection accuracy by 10-15%** (F1: 0.80 → 0.89+)
4. **Enable user customization** (modular pattern system)
5. **Support automatic updates** (no rebuild required)

**Implementation Priority:**
- **P0 (Week 1-2):** Token Efficiency, Word Filter, High-Confidence Patterns
- **P1 (Week 3-4):** Modular System, Entropy Modification, Adaptive Thresholds
- **P2 (Month 2):** Full Pattern DB, Auto-Updates, CEL Validation

**Expected Timeline:** 4 weeks for P0+P1, 8 weeks for full implementation

**Expected Outcome:** Industry-leading secret detection with F1 score > 0.90

---

## References

1. **Betterleaks Repository:** https://github.com/betterleaks/betterleaks
2. **Secrets-Patterns-DB:** https://github.com/mazen160/secrets-patterns-db
3. **Aikido Security Blog:** https://www.aikido.dev/blog/token-efficiency-secrets-scan
4. **GitGuardian Documentation:** https://docs.gitguardian.com/secrets-detection/secrets-detection-engine
5. **TruffleHog Documentation:** https://trufflesecurity.com/blog/scanning-git-for-secrets-the-2024-comprehensive-guide

---

## Appendix: Quick Start Implementation

### Week 1 Day 1: Token Efficiency

```bash
# Add dependency
cd coax/crates/coax-scanner
cargo add tiktoken-go

# Implement in scanner.rs
# See betterleaks-analysis.md for implementation details
```

### Week 1 Day 3: Word Filter

```bash
# Add dependency
cargo add aho-corasick

# Download word list
wget https://raw.githubusercontent.com/betterleaks/betterleaks/master/words/words.go
# Extract word list, convert to Rust format

# Implement word filter
# See betterleaks-analysis.md for implementation details
```

### Week 2 Day 1: Patterns Integration

```bash
# Clone secrets-patterns-db
git clone https://github.com/mazen160/secrets-patterns-db.git /tmp/spdb

# Filter high-confidence patterns
cd /tmp/spdb
python3 scripts/convert-rules.py --db db/rules-stable.yml --type gitleaks --export high-confidence

# Convert to YAML format for Coax
# Implement pattern loader
```

---

*Research completed: March 15, 2026*  
*Ready for Phase 2 implementation*
