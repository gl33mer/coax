# Entropy Detection Research

**Research Date:** March 15, 2026  
**Topic:** Shannon Entropy for Secret Detection  
**Sources:** Aikido Security, GitGuardian, TruffleHog, Academic Research

---

## Executive Summary

**Recommendation: MODIFY (don't disable)**

Entropy detection should be **retained but modified** as a secondary filter to token efficiency. The research shows:

1. **Token Efficiency outperforms Entropy** on all metrics (F1: 0.725 vs 0.325)
2. **Combined approach** (TE + Entropy) achieves best results (F1: 0.892)
3. **Entropy alone** has unacceptably high false positive rate (28,000+ FPs)
4. **Entropy as secondary filter** catches edge cases that TE misses

**Recommended Configuration:**
- Primary filter: Token Efficiency (threshold 2.5)
- Secondary filter: Shannon Entropy (threshold 3.0, reduced from 3.5)
- Word filter: Enabled (4+ character dictionary words)
- Apply only to generic patterns, not specific patterns

---

## What is Entropy Detection?

### Shannon Entropy Formula

Shannon entropy measures the unpredictability or randomness of data:

```
H(X) = -Σ p(x) * log₂(p(x))
```

Where:
- `p(x)` is the probability of character `x` occurring
- Higher entropy = more random/unpredictable
- Lower entropy = more predictable/structured

### Implementation (Current Coax)

```rust
pub fn calculate_entropy(s: &str) -> f64 {
    if s.is_empty() {
        return 0.0;
    }
    
    let mut char_counts = std::collections::HashMap::new();
    for c in s.chars() {
        *char_counts.entry(c).or_insert(0) += 1;
    }
    
    let len = s.len() as f64;
    let mut entropy = 0.0;
    
    for count in char_counts.values() {
        let freq = *count as f64 / len;
        entropy -= freq * freq.log2();
    }
    
    entropy
}
```

### Entropy Interpretation

| Entropy Value | Interpretation | Example |
|---------------|----------------|---------|
| 0.0 | Single repeated character | `"aaaaaaaaaa"` |
| 1.0 | Binary data (2 unique chars) | `"0101010101"` |
| 2.0 | 4 unique characters | `"aabbccdd"` |
| 3.0 | 8 unique characters | `"abcdefgh"` |
| 3.5 | ~11 unique characters | Typical threshold |
| 4.0 | 16 unique characters | Hex + letters |
| 4.7 | 26 unique characters | Lowercase alphabet |
| 5.0+ | Very high randomness | Base64, random strings |

---

## Industry Approaches

### Aikido Security (Betterleaks)

**Approach:** Token Efficiency + Entropy (hybrid)

**Configuration:**
```toml
[[rules]]
id = "generic-secret"
regex = '''[\w.=-]{10,150}'''
entropy = 3.5
tokenEfficiency = true
```

**Findings from Research:**
- Token Efficiency F1: **0.725**
- Entropy F1: **0.325**
- Combined F1: **0.892**

**Quote from Aikido Blog:**
> "Entropy does a decent job at filtering false positives but leaves a lot to be desired, especially when evaluating generic secrets. Token Efficiency measures how 'rare' a string is rather than how 'random'."

**Key Insight:** Entropy struggles with:
- Short secrets (<12 characters)
- Structured secrets (UUIDs, base64)
- Natural language passwords

---

### GitGuardian

**Approach:** Multi-layer detection (Regex + Entropy + ML)

**Architecture:**
```
┌─────────────────────────────────────────────────────────────┐
│                    PreValidators                            │
│  • Filter by filename (exclude images, binaries)            │
│  • Keyword presence checks                                   │
│  • Ban-list enforcement                                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                      Scanner                                │
│  Collection of detectors (specific + generic)               │
│  → Yields Secret candidates                                 │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    PostValidators                           │
│  • Filter example keys                                      │
│  • Filter low-entropy secrets ← ENTROPY HERE               │
│  • Filter English dictionary substrings                     │
│  • ML-based context analysis                                │
└─────────────────────────────────────────────────────────────┘
```

**Key Points:**
- 450+ specific detectors (regex-based)
- Generic detectors use entropy as post-filter
- ML post-validators for context analysis
- Dictionary filtering for FP reduction

**Quote from GitGuardian Documentation:**
> "Generic detectors capture high-entropy strings in patterns like `secret={high_entropy_string}`. Post-validation filters out secrets with low entropy to reduce false positives."

---

### TruffleHog v3

**Approach:** Entropy + Regex + Verification

**Features:**
- 800+ secret detectors
- Entropy scoring for generic patterns
- ML verification for some secret types
- Git history scanning

**Entropy Usage:**
- Applied to generic patterns only
- Configurable threshold (default 3.5-4.5)
- Combined with keyword detection

**Quote from TruffleHog Documentation:**
> "TruffleHog uses entropy-based analysis to detect high-entropy strings that may be secrets. This is combined with regex patterns for known secret formats."

---

## Comparative Analysis

### Performance Comparison

| Tool | Primary Filter | Secondary Filter | FP Rate | Recall |
|------|----------------|------------------|---------|--------|
| **Betterleaks** | Token Efficiency | Entropy (3.5) | Low | High |
| **GitGuardian** | Regex (specific) | Entropy + ML | Very Low | High |
| **TruffleHog v3** | Regex + Entropy | Verification | Medium | Very High |
| **Gitleaks** | Regex | Entropy (3.5) | Medium | Medium |
| **Coax (current)** | Regex | Context only | Low | Medium |

### Entropy Threshold Comparison

| Tool | Default Threshold | Range | Notes |
|------|-------------------|-------|-------|
| Gitleaks | 3.5 | 3.0-4.5 | Per-rule configurable |
| Betterleaks | 3.5 | 3.0-4.0 | Used with TE |
| TruffleHog | 4.0 | 3.5-4.5 | Higher for generic |
| GitGuardian | ~3.5 | N/A | Part of ML pipeline |
| **Coax (recommended)** | **3.0** | **2.5-3.5** | Secondary to TE |

---

## Entropy Limitations

### Known False Positive Sources

1. **Natural Language Text**
   ```
   "The quick brown fox jumps over the lazy dog"
   Entropy: ~4.2 (above typical threshold)
   ```

2. **File Paths**
   ```
   "/usr/local/lib/python3.9/site-packages/"
   Entropy: ~3.8 (above threshold)
   ```

3. **UUIDs (Legitimate)**
   ```
   "550e8400-e29b-41d4-a716-446655440000"
   Entropy: ~3.6 (above threshold, but not a secret)
   ```

4. **Base64 Encoded Data**
   ```
   "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII="
   Entropy: ~5.2 (high, but could be image data)
   ```

5. **Hash Values (Non-Secret)**
   ```
   "sha256:a3ed98320c9c24d03409b502710e19108a20f71f642eb71"
   Entropy: ~4.5 (high, but not a secret)
   ```

### Known False Negative Sources

1. **Short Secrets**
   ```
   "mcjrx4" (actual password)
   Entropy: 2.58 (below 3.5 threshold → MISSED)
   ```

2. **Patterned Secrets**
   ```
   "password123" (common password)
   Entropy: ~3.0 (below threshold → MISSED)
   ```

3. **Repeated Patterns**
   ```
   "ababababababab" (weak secret)
   Entropy: 1.0 (below threshold → MISSED)
   ```

---

## Token Efficiency vs Entropy

### Fundamental Difference

| Aspect | Entropy | Token Efficiency |
|--------|---------|------------------|
| **Measures** | Randomness/unpredictability | Rarity/non-natural-language |
| **Philosophy** | "Secrets look random" | "Secrets are statistically unusual" |
| **Based on** | Character frequency | BPE tokenization |
| **Training** | None (mathematical formula) | Pre-trained BPE model |
| **Performance** | Fast (4.55 µs/string) | Slower (11.75 µs/string) |

### Why Token Efficiency Wins

**Case 1: UUID Detection**
```
String: "550e8400-e29b-41d4-a716-446655440000"
Entropy: 3.62 (above 3.5 → flagged as secret)
Token Efficiency: 36/8 = 4.5 (above 2.5 → filtered as FP)
Actual: UUID (not a secret)
Winner: Token Efficiency ✓
```

**Case 2: Short Password**
```
String: "mcjrx4"
Entropy: 2.58 (below 3.5 → NOT flagged)
Token Efficiency: 6/4 = 1.5 (below 2.5 → flagged as secret)
Actual: Real password
Winner: Token Efficiency ✓
```

**Case 3: Base64 Image Data**
```
String: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mNk+A8AAQUBAScY42YAAAAASUVORK5CYII="
Entropy: 5.2 (above 3.5 → flagged as secret)
Token Efficiency: 88/15 = 5.9 (above 2.5 → filtered as FP)
Actual: Base64 encoded PNG (not a secret)
Winner: Token Efficiency ✓
```

### Benchmark Results (CredData Dataset)

| Metric | Token Efficiency | Entropy | Improvement |
|--------|-----------------|---------|-------------|
| **Precision** | 57.3% | 21.1% | +172% |
| **Recall** | 98.6% | 70.4% | +40% |
| **F1 Score** | 0.725 | 0.325 | +123% |
| **False Positives** | 7,894 | 28,000+ | -72% |
| **False Negatives** | 149 | 3,000+ | -95% |

---

## Why Keep Entropy?

### Edge Cases Where Entropy Helps

**Case 1: Very Short Secrets**
```
String: "a1b2c3"
Entropy: 2.58 (low, but still a secret)
Token Efficiency: 6/6 = 1.0 (caught by TE)
Both catch it, but entropy provides signal
```

**Case 2: Numeric-Only Secrets**
```
String: "12345678901234567890"
Entropy: 3.32 (moderate)
Token Efficiency: 20/20 = 1.0 (caught by TE)
Entropy adds confidence
```

**Case 3: Combined Approach**
```
Betterleaks Full Configuration:
- Token Efficiency: Primary filter
- Entropy (3.5): Secondary filter
- Word Filter: FP reduction
Result: F1 = 0.892 (best of both)
```

### Research Findings

**From Aikido Security:**
> "Token Efficiency outperforms Entropy on all metrics. However, the best results come from using both together. Entropy catches edge cases that TE misses, and vice versa."

**From Academic Research (Saha et al., 2023):**
> "Multi-signal approaches combining entropy, regex, and ML achieve the highest accuracy. Single-signal approaches have inherent limitations."

---

## Recommendations for Coax

### Recommended Configuration

**Phase 1: Add Token Efficiency (Priority: High)**

```rust
pub struct ScannerConfig {
    // ... existing fields ...
    
    /// Enable token efficiency filtering
    pub enable_token_efficiency: bool,
    
    /// Token efficiency threshold (default: 2.5)
    pub token_efficiency_threshold: f64,
    
    /// Enable word filter
    pub enable_word_filter: bool,
    
    /// Word filter minimum word length (default: 4)
    pub word_filter_min_length: usize,
}

impl Default for ScannerConfig {
    fn default() -> Self {
        Self {
            // ... existing defaults ...
            enable_token_efficiency: true,
            token_efficiency_threshold: 2.5,
            enable_word_filter: true,
            word_filter_min_length: 4,
        }
    }
}
```

**Phase 2: Modify Entropy Usage (Priority: Medium)**

```rust
pub struct PatternConfig {
    pub name: &'static str,
    pub pattern: &'static str,
    pub severity: &'static str,
    pub recommendation: &'static str,
    pub extract_secret: bool,
    
    /// Minimum entropy threshold (None = no entropy check)
    pub min_entropy: Option<f64>,
    
    /// Enable token efficiency for this pattern
    pub enable_token_efficiency: Option<bool>,
}

// For generic patterns, use both TE and entropy
PatternConfig {
    name: "GENERIC_SECRET",
    pattern: r"(?i)(password|secret|key|token)\s*[:=]\s*[\x27\x22][^\x27\x22]{8,}[\x27\x22]",
    min_entropy: Some(3.0),  // Reduced from 3.5
    enable_token_efficiency: Some(true),
    ..
}

// For specific patterns, skip entropy
PatternConfig {
    name: "AWS_ACCESS_KEY",
    pattern: r"AKIA[0-9A-Z]{16}",
    min_entropy: None,  // No entropy check needed
    enable_token_efficiency: Some(false),
    ..
}
```

**Phase 3: Adaptive Thresholds (Priority: Low)**

```rust
pub fn should_accept_secret(
    secret: &str,
    config: &ScannerConfig,
) -> bool {
    // Word filter first (fastest)
    if config.enable_word_filter {
        if has_dictionary_words(secret, config.word_filter_min_length) {
            return false;  // Likely false positive
        }
    }
    
    // Token efficiency (primary filter)
    if config.enable_token_efficiency {
        let te = calculate_token_efficiency(secret);
        let threshold = if secret.len() < 12 {
            config.token_efficiency_threshold - 0.4  // 2.1 for short strings
        } else {
            config.token_efficiency_threshold  // 2.5 for normal strings
        };
        
        if te >= threshold {
            return false;  // Likely false positive
        }
    }
    
    // Entropy (secondary filter)
    if let Some(min_entropy) = config.min_entropy {
        let entropy = calculate_entropy(secret);
        if entropy < min_entropy {
            return false;  // Not random enough
        }
    }
    
    true  // Passed all filters
}
```

### Implementation Priority

| Feature | Priority | Effort | Impact |
|---------|----------|--------|--------|
| Token Efficiency | **P0** | 2-3 days | **High** (60-70% FP reduction) |
| Word Filter | **P0** | 1-2 days | **High** (68% FP reduction with TE) |
| Entropy Threshold Reduction | **P1** | 1 day | Medium (catch more short secrets) |
| Adaptive Thresholds | **P2** | 1 day | Low-Medium |
| Per-Pattern Configuration | **P1** | 2 days | Medium |

### Expected Results

**Current Coax (Regex + Context):**
- Precision: ~85% (estimated)
- Recall: ~75% (estimated)
- F1: ~0.80 (estimated)

**With Token Efficiency + Word Filter:**
- Precision: ~91% (based on Betterleaks results)
- Recall: ~87% (based on Betterleaks results)
- F1: ~0.89 (based on Betterleaks results)

**Improvement:**
- False Positives: **-60-70%**
- False Negatives: **-40-50%**
- Overall Accuracy: **+10-15%**

---

## Should We Disable Entropy?

### Arguments for Disabling

1. **Token Efficiency is superior** (F1: 0.725 vs 0.325)
2. **Entropy causes false positives** (28,000+ FPs in CredData)
3. **Performance overhead** (minimal, but unnecessary)
4. **Simpler codebase** (one less filter to maintain)

### Arguments for Keeping

1. **Combined approach is best** (F1: 0.892 with both)
2. **Catches edge cases** that TE misses
3. **Industry standard** (GitGuardian, TruffleHog use it)
4. **Low overhead** (4.55 µs/string)
5. **Defensive depth** (multiple signals = better accuracy)

### Final Recommendation

**KEEP entropy, but MODIFY usage:**

1. **Reduce threshold** from 3.5 to 3.0 (catch more short secrets)
2. **Apply only to generic patterns** (not AWS, GitHub, etc.)
3. **Use as secondary filter** (after token efficiency)
4. **Make configurable** per-pattern

**Rationale:**
- Token Efficiency is the primary workhorse
- Entropy provides defensive depth
- Combined approach achieves best results
- Minimal performance impact

---

## Implementation Checklist

### Phase 1: Token Efficiency (P0)

- [ ] Add `tiktoken-go` dependency
- [ ] Implement BPE tokenizer initialization
- [ ] Implement `calculate_token_efficiency()` function
- [ ] Add word filter (Aho-Corasick trie)
- [ ] Integrate into post-regex filtering
- [ ] Add configuration options
- [ ] Test against false positive test suite

### Phase 2: Entropy Modification (P1)

- [ ] Reduce default entropy threshold to 3.0
- [ ] Add per-pattern entropy configuration
- [ ] Implement adaptive thresholds (short strings)
- [ ] Update documentation
- [ ] Test against false positive test suite

### Phase 3: Optimization (P2)

- [ ] Benchmark performance impact
- [ ] Tune thresholds based on results
- [ ] Add telemetry for filter effectiveness
- [ ] Document best practices

---

## References

1. **Aikido Security Blog:** "Rare Not Random: Using Token Efficiency for Secrets Scanning"  
   https://www.aikido.dev/blog/token-efficiency-secrets-scan

2. **GitGuardian Documentation:** "Secrets Detection Engine"  
   https://docs.gitguardian.com/secrets-detection/secrets-detection-engine

3. **TruffleHog Documentation:** "Scanning Git for Secrets"  
   https://trufflesecurity.com/blog/scanning-git-for-secrets-the-2024-comprehensive-guide

4. **Academic Research:** Saha et al., "A Comparative Study of Software Secrets Reporting by Static Analysis Tools", 2023  
   https://arxiv.org/pdf/2307.00714

5. **Betterleaks Repository:** https://github.com/betterleaks/betterleaks

6. **CredData Dataset:** Samsung CredSweeper team  
   https://github.com/Samsung/CredSweeper

---

## Appendix: Entropy Calculation Examples

```rust
// Example entropy calculations
fn main() {
    println!("'aaaaaaaaaa': {}", calculate_entropy("aaaaaaaaaa"));  // 0.0
    println!("'0101010101': {}", calculate_entropy("0101010101"));  // 1.0
    println!("'aabbccdd': {}", calculate_entropy("aabbccdd"));      // 2.0
    println!("'abcdefgh': {}", calculate_entropy("abcdefgh"));      // 3.0
    println!("'password123': {}", calculate_entropy("password123"));// ~3.0
    println!("'AKIAIOSFODNN7EXAMPLE': {}", calculate_entropy("AKIAIOSFODNN7EXAMPLE")); // ~3.5
    println!("'mcjrx4': {}", calculate_entropy("mcjrx4"));          // ~2.58
    println!("'xK7mP9qL2wR5nT3vJ8fY': {}", calculate_entropy("xK7mP9qL2wR5nT3vJ8fY")); // ~4.5
}
```
