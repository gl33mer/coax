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
