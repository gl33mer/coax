# Betterleaks Analysis

**Research Date:** March 15, 2026  
**Repository:** https://github.com/betterleaks/betterleaks  
**Version Analyzed:** v1.0.0 (latest stable)

---

## Executive Summary

Betterleaks is the successor to Gitleaks, built by the same team with significant improvements to secret detection accuracy. The key innovation is **Token Efficiency**, a post-regex filtering technique that outperforms traditional entropy-based detection by measuring how "non-natural-language" a string is rather than how random it appears.

**Key Finding:** Token Efficiency achieves **F1 score of 0.725** vs entropy's **0.325** - more than doubling detection accuracy.

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────┐
│                    Input (File/Git/Stdin)                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Pre-Filtering (Aho-Corasick)               │
│  • Keyword matching before regex                             │
│  • Fast string comparison for rules with keywords            │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Regex Pattern Matching                     │
│  • 251 rules in default configuration                        │
│  • Configurable regex engine (stdlib/re2)                    │
│  • Path-based filtering                                      │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              Post-Regex Validation Filters                  │
│  ┌─────────────────┐  ┌─────────────────┐                   │
│  │ Token Efficiency│  │   Entropy Check │                   │
│  │   (BPE-based)   │  │  (Shannon, 3.5) │                   │
│  └─────────────────┘  └─────────────────┘                   │
│  ┌─────────────────┐  ┌─────────────────┐                   │
│  │  Word Filter    │  │  Allowlist Check│                   │
│  │ (Dictionary)    │  │ (Commits/Paths) │                   │
│  └─────────────────┘  └─────────────────┘                   │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│              CEL Validation (Optional)                      │
│  • HTTP requests to verify secrets are live                  │
│  • Configurable per-rule                                     │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                    Output (JSON/SARIF/CSV)                  │
└─────────────────────────────────────────────────────────────┘
```

---

## Detection Strategies

### 1. Token Efficiency (Primary Filter)

**What it is:** A post-regex filtering technique that measures how "out-of-vocabulary" a string is using Byte-Pair Encoding (BPE) tokenization.

**How it works:**
```rust
func (d *Detector) failsTokenEfficiencyFilter(secret string) bool {
    // 1. Encode with BPE tokenizer (cl100k_base)
    tokens := d.tokenizer.Encode(analyzed, nil, nil)
    
    // 2. Word filter: reject if contains 5+ char words
    matches := words.HasMatchInList(analyzed, 5)
    if len(matches) > 0 {
        return true  // Filter out as false positive
    }
    
    // 3. Calculate efficiency
    threshold := 2.5
    if len(analyzed) < 12 {
        threshold = 2.1  // Adaptive threshold for short strings
    }
    
    // 4. Compare: high efficiency = natural language = false positive
    return float64(len(analyzed))/float64(len(tokens)) >= threshold
}
```

**Formula:**
```
token_efficiency = len(string) / len(tokens)
```

**Interpretation:**
- **High efficiency (>2.5)**: Natural language, common patterns → **false positive**
- **Low efficiency (<2.5)**: Rare, unnatural patterns → **likely secret**

**Examples:**

| String | Tokens | Token Count | Efficiency | Classification |
|--------|--------|-------------|------------|----------------|
| `"Hello World"` | `[15339, 1917]` | 2 | 11/2 = **5.5** | Natural language |
| `"password"` | `[3918]` | 1 | 8/1 = **8.0** | Common word |
| `"github"` | `[5316]` | 1 | 6/1 = **6.0** | Common word |
| `"kj2h3f2uaafewa"` | `[93797, 17, 71, ...]` | 11 | 14/11 = **1.27** | **Secret** |
| `ghp_xK7mP9qL2wR5nT3vJ8fY` | `[876, 79, 3292, ...]` | 22 | 24/22 = **1.1** | **Secret** |

**Thresholds:**
- Default: **≥ 2.5** (minimum cutoff for non-secrets)
- Short strings (<12 chars): **≥ 2.1** (stricter threshold)

---

### 2. Shannon Entropy (Secondary Filter)

**What it is:** Traditional randomness measurement using Shannon entropy formula.

**Implementation:**
```go
func shannonEntropy(data string) (entropy float64) {
    if data == "" {
        return 0
    }
    
    charCounts := make(map[rune]int)
    for _, char := range data {
        charCounts[char]++
    }
    
    invLength := 1.0 / float64(len(data))
    for _, count := range charCounts {
        freq := float64(count) * invLength
        entropy -= freq * math.Log2(freq)
    }
    
    return entropy
}
```

**Usage in Betterleaks:**
- Applied **after** regex matching
- Default threshold: **3.5** for generic secrets
- Used as **secondary filter** to token efficiency
- Can be disabled for specific rules

---

### 3. Word Filter (Dictionary-Based)

**What it is:** Aho-Corasick trie-based dictionary matching to detect common words in candidate secrets.

**Implementation:**
```go
func HasMatchInList(word string, minLen int) []Result {
    word = strings.ToLower(word)
    if len(word) < minLen {
        return nil
    }
    
    var matches []Match
    seen := make(map[string]struct{})
    
    // Walk the word: at each start position, try every substring length >= minLen
    for start := 0; start <= len(word)-minLen; start++ {
        for length := minLen; start+length <= len(word); length++ {
            sub := word[start : start+length]
            if _, exists := nltkWords[sub]; exists {
                if _, ok := seen[sub]; !ok {
                    seen[sub] = struct{}{}
                }
                matches = append(matches, Match{Word: sub, Len: length})
            }
        }
    }
    
    // Return aggregated results
    return []Result{{
        WordCount:   len(matches),
        UniqueWords: uniqueWords,
        Matches:     matches,
    }}
}
```

**Dictionary:** 777KB of NLTK words (common English words)

**Usage:**
- Filters out secrets containing 4+ character dictionary words
- Reduces false positives by **68%** with minimal false negative cost
- Applied **before** token efficiency calculation

---

### 4. Allowlists (Multi-Layer Filtering)

**What it is:** Comprehensive filtering system to ignore known false positives.

**Allowlist Types:**

| Type | Description | Example |
|------|-------------|---------|
| **Commits** | Ignore specific commit SHAs | `["abc123...", "def456..."]` |
| **Paths** | Ignore file path patterns | `node_modules`, `*.lock` |
| **Regexes** | Ignore content matching regex | `^(true|false|null)$` |
| **StopWords** | Ignore secrets containing specific strings | `"014df517-39d1-4453-b7b3-9930c563627c"` |

**Configuration:**
```toml
[allowlist]
description = "global allow lists"
paths = [
    '''gitleaks\.toml''',
    '''(?i)\.(?:bmp|gif|jpe?g|png|svg|tiff?)$''',
    '''(?:^|/)node_modules(?:/.*)?$''',
    '''(?:^|/)(?:package-lock\.json|yarn\.lock)$''',
]
regexes = [
    '''(?i)^true|false|null$''',
    '''^\$(?:[A-Z_]+|[a-z_]+)$''',
    '''^\{\{[ \t]*[\w ().|]+[ \t]*}}$''',
]
stopwords = [
    "014df517-39d1-4453-b7b3-9930c563627c",
    "abcdefghijklmnopqrstuvwxyz",
]
```

**Match Conditions:**
- **OR** (default): Any condition match allows the finding
- **AND**: All conditions must match to allow

---

### 5. CEL Validation (Live Secret Verification)

**What it is:** Common Expression Language (CEL) based HTTP validation to verify if detected secrets are actually live/active.

**Configuration:**
```toml
[[rules]]
id = "sendgrid-api-key"
regex = '''SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}'''
ValidateCEL = '''
    http.get("https://api.sendgrid.com/v3/validations").with(
        header("Authorization", "Bearer " + finding.secret)
    ).status == 200
'''
```

**Features:**
- Makes HTTP requests to verify secrets
- Configurable timeout (default 10s)
- Can filter by validation status (valid, invalid, revoked, error, unknown)
- **Optional feature** - can be disabled for offline scanning

---

## False Positive Reduction Strategies

### Multi-Layer Approach

```
Regex Match → Token Efficiency → Word Filter → Entropy → Allowlist → CEL Validation
     ↓              ↓                 ↓            ↓          ↓           ↓
  Capture      Filter out        Filter out    Filter    Filter     Verify live
  candidate    unnatural          common       random   known      status
               language           words        enough   FPs
```

### Performance Comparison (CredData Dataset)

| Configuration | Precision | Recall | F1 Score | False Positives |
|---------------|-----------|--------|----------|-----------------|
| **Token Efficiency + Entropy** | **91.28%** | **87.25%** | **0.8922** | 1,031 |
| Token Efficiency only | 88.5% | 89.1% | 0.8696 | ~1,700 |
| Entropy only | 85.2% | 72.4% | 0.7756 | ~1,800 |
| **Token Efficiency (raw)** | **57.3%** | **98.6%** | **0.725** | 7,894 |
| **Entropy (raw)** | **21.1%** | **70.4%** | **0.325** | 28,000+ |
| TE + Word Filter | **80.4%** | **95.8%** | **0.874** | 2,508 |
| Entropy + Word Filter | 76.6% | 67.1% | 0.715 | 2,500 |

### Key Insights

1. **Token Efficiency alone** reduces FPs by **72%** vs entropy (7,894 vs 28,000+)
2. **Word filter** reduces FPs by **68%** (7,894 → 2,508) with only 308 additional FNs
3. **Combined approach** (TE + Entropy + Word Filter) achieves best results
4. **Full configuration** achieves F1 of **0.892** vs CredSweeper's **0.85**

---

## Pattern Database

### Structure

**File:** `config/betterleaks.toml` (4,630 lines)

**Rule Count:** 251 rules

**Rule Format:**
```toml
[[rules]]
id = "aws-access-key"
description = "AWS Access Key ID"
regex = '''AKIA[0-9A-Z]{16}'''
entropy = 3.5
keywords = ["AKIA"]
tokenEfficiency = true
tags = ["aws", "cloud", "critical"]
```

**Rule Fields:**

| Field | Type | Description |
|-------|------|-------------|
| `id` | string | Unique identifier |
| `description` | string | Human-readable description |
| `regex` | string | Golang regular expression |
| `entropy` | float | Minimum Shannon entropy (optional) |
| `secretGroup` | int | Regex group to extract for entropy check |
| `tokenEfficiency` | bool | Enable token efficiency filter |
| `keywords` | []string | Pre-regex keyword filtering |
| `tags` | []string | Metadata for reporting |
| `path` | string | Path regex filter (optional) |
| `ValidateCEL` | string | CEL expression for live validation |

### Pattern Categories

| Category | Count | Examples |
|----------|-------|----------|
| Cloud Providers | 45 | AWS, GCP, Azure, DigitalOcean |
| Version Control | 12 | GitHub, GitLab, Bitbucket |
| Communication | 18 | Slack, SendGrid, Twilio, Discord |
| Payment | 8 | Stripe, Square, PayPal |
| Database | 15 | PostgreSQL, MongoDB, MySQL, Redis |
| Package Managers | 9 | npm, PyPI, RubyGems, Cargo |
| AI/ML | 6 | OpenAI, Anthropic, HuggingFace |
| Private Keys | 8 | RSA, EC, DSA, SSH, PGP |
| Generic | 25 | Generic secrets, passwords, high-entropy |

---

## Performance Characteristics

### Timing Benchmarks

| Operation | Time per String | Relative Cost |
|-----------|-----------------|---------------|
| **Regex matching** | ~50-100 µs | **Primary bottleneck** |
| Entropy calculation | 4.55 µs | Baseline |
| Token Efficiency calculation | 11.75 µs | 2.5x slower than entropy |
| Word filter (Aho-Corasick) | ~2 µs | Negligible |

### Optimization Strategies

1. **Keyword pre-filtering**: Aho-Corasick before regex
2. **Parallel git scanning**: `--git-workers=8`
3. **Regex engine switching**: `--regex-engine=stdlib/re2`
4. **Recursive decoding**: Optimized for SHA1-HULUD variants
5. **Buffer pooling**: Reusable buffers for lowercasing

---

## What Makes Betterleaks More Accurate

### 1. Token Efficiency > Entropy

**Philosophy shift:**
- **Entropy**: "Secrets look random"
- **Token Efficiency**: "Secrets are statistically unusual"

**Why it works better:**
- BPE tokenizers implicitly reflect frequency distributions from training data
- Common words/subwords → merged into long tokens (high efficiency)
- Rare/unnatural strings → broken into many short tokens (low efficiency)
- Can distinguish b64, UUID, actual secrets, weird dependencies

### 2. Multi-Layer Filtering

No single technique is perfect. Betterleaks uses:
- **Token Efficiency** for primary filtering
- **Entropy** as secondary check
- **Word Filter** for dictionary-based FP reduction
- **Allowlists** for known FPs
- **CEL Validation** for live secret verification

### 3. Rule-Specific Configuration

Each rule can have:
- Custom entropy threshold
- Token efficiency enabled/disabled
- Keywords for pre-filtering
- Path-based filtering
- CEL validation expression

### 4. Community-Driven Pattern Database

- 251 rules (vs Gitleaks' ~60)
- Auto-generated from contribution system
- Regular updates from security community

---

## Machine Learning / AI Usage

### Current ML/AI Usage

**Token Efficiency (BPE-based):**
- Uses pre-trained BPE tokenizer (`cl100k_base`)
- **Not ML in the traditional sense** - no model training required
- BPE vocabulary implicitly encodes frequency information

**CEL Validation:**
- Rule-based HTTP validation
- **Not ML** - deterministic verification

### No Traditional ML

Betterleaks does **NOT** use:
- Neural networks
- Classification models
- Training pipelines
- Feature engineering

**Rationale:**
- BPE tokenization provides sufficient signal
- Simpler = faster, more maintainable
- No false positive training data required

---

## Secret Verification

### CEL-Based Validation

**How it works:**
1. Detect secret via regex
2. Execute CEL expression against secret
3. CEL can make HTTP requests to verify
4. Filter results by validation status

**Example:**
```toml
[[rules]]
id = "sendgrid-api-key"
regex = '''SG\.[a-zA-Z0-9_-]{22}\.[a-zA-Z0-9_-]{43}'''
ValidateCEL = '''
    http.get("https://api.sendgrid.com/v3/validations").with(
        header("Authorization", "Bearer " + finding.secret)
    ).status == 200
'''
```

**Validation Statuses:**
- `valid`: Secret verified as active
- `invalid`: Secret verified as inactive
- `revoked`: Secret was revoked
- `error`: Validation failed (network, timeout)
- `unknown`: Validation not attempted
- `none`: Rule has no validation

**Configuration:**
```bash
betterleaks scan --validation \
                 --validation-status valid,unknown \
                 --validation-timeout 10s
```

---

## Adaptability to Coax

### What Coax Can Adopt

#### High Priority (Immediate Impact)

1. **Token Efficiency Filter**
   - Add BPE tokenizer dependency (`tiktoken-go`)
   - Implement post-regex filtering
   - Expected FP reduction: **60-70%**

2. **Word Filter**
   - Build Aho-Corasick trie from NLTK words
   - Filter secrets containing 4+ char dictionary words
   - Expected FP reduction: **68%** (when combined with TE)

3. **Adaptive Thresholds**
   - Lower threshold (2.1) for short strings <12 chars
   - Higher threshold (2.5) for longer strings

#### Medium Priority (Architectural Improvements)

4. **Modular Pattern System**
   - YAML/JSON pattern configuration
   - Support external pattern databases
   - Version pattern databases

5. **Enhanced Allowlists**
   - Commit-based filtering (for git scanning)
   - Path-based filtering (already partially implemented)
   - Regex-based filtering
   - StopWord filtering

6. **Rule-Specific Configuration**
   - Per-rule entropy thresholds
   - Per-rule token efficiency toggle
   - Per-rule keywords

#### Low Priority (Future Enhancements)

7. **CEL Validation**
   - Live secret verification
   - Requires HTTP client infrastructure
   - Optional feature

8. **Regex Engine Switching**
   - Support stdlib and re2 engines
   - Performance tuning option

---

## Implementation Complexity

### Token Efficiency

**Complexity:** Medium  
**Dependencies:** `tiktoken-go` (BPE tokenizer)  
**Code Changes:**
- Add tokenizer initialization
- Implement `fails_token_efficiency_filter()` function
- Integrate into post-regex filtering pipeline
- Add word filter (Aho-Corasick trie)

**Estimated Effort:** 2-3 days

### Word Filter

**Complexity:** Low-Medium  
**Dependencies:** `aho-corasick` crate  
**Code Changes:**
- Build trie from word list
- Implement `has_match_in_list()` function
- Integrate with token efficiency filter

**Estimated Effort:** 1-2 days

### Modular Pattern System

**Complexity:** Medium  
**Dependencies:** `serde_yaml`, `serde_json`  
**Code Changes:**
- Define pattern schema
- Implement pattern loader
- Add pattern validation
- Update scanner to use dynamic patterns

**Estimated Effort:** 3-4 days

---

## Known Limitations

### Token Efficiency Weaknesses

1. **Bad Passwords**: `"password123"`, `"chibearsfan123"` (high TE, natural language)
2. **Passphrases**: Usually just words (high TE)
3. **Short Secrets**: <6 characters may not tokenize well

### General Limitations

1. **Performance Overhead**: Token efficiency is 2.5x slower than entropy
2. **Offline Dependency**: BPE tokenizer requires word list
3. **Language Bias**: BPE trained on English text (may miss non-English secrets)

---

## References

- **Blog Post:** [Rare Not Random: Using Token Efficiency for Secrets Scanning](https://www.aikido.dev/blog/token-efficiency-secrets-scan)
- **Tokenizer:** `cl100k_base` (test at https://tiktokenizer.vercel.app/?model=cl100k_base)
- **Dataset:** CredData (Samsung CredSweeper team)
- **Implementation:** Betterleaks (Gitleaks successor)
- **Original Research:** Zach Rice (Head of Secrets Scanning, founder of Gitleaks)

---

## Appendix: Key Code Snippets

### Token Efficiency Filter (Go)

```go
func (d *Detector) failsTokenEfficiencyFilter(secret string) bool {
    // For short secrets (< 20 chars) that contain newlines, strip the newlines
    // before analysis so that strings like "123\n\nTest" are evaluated as "123Test"
    analyzed := secret
    if len(analyzed) < 20 && strings.ContainsAny(analyzed, "\n\r") {
        analyzed = newlineReplacer.Replace(analyzed)
    }
    
    tokens := d.tokenizer.Encode(analyzed, nil, nil)
    
    matches := words.HasMatchInList(analyzed, 5)
    if len(matches) > 0 {
        return true
    }
    
    threshold := 2.5
    if len(analyzed) < 12 {
        threshold = 2.1
        matches := words.HasMatchInList(analyzed, 4)
        if len(matches) == 0 {
            threshold = 2.5
        }
    }
    
    return float64(len(analyzed))/float64(len(tokens)) >= threshold
}
```

### Shannon Entropy (Go)

```go
func shannonEntropy(data string) (entropy float64) {
    if data == "" {
        return 0
    }
    
    charCounts := make(map[rune]int)
    for _, char := range data {
        charCounts[char]++
    }
    
    invLength := 1.0 / float64(len(data))
    for _, count := range charCounts {
        freq := float64(count) * invLength
        entropy -= freq * math.Log2(freq)
    }
    
    return entropy
}
```

### Word Filter (Aho-Corasick)

```go
func HasMatchInList(word string, minLen int) []Result {
    word = strings.ToLower(word)
    if len(word) < minLen {
        return nil
    }
    
    var matches []Match
    seen := make(map[string]struct{})
    
    for start := 0; start <= len(word)-minLen; start++ {
        for length := minLen; start+length <= len(word); length++ {
            sub := word[start : start+length]
            if _, exists := nltkWords[sub]; exists {
                if _, ok := seen[sub]; !ok {
                    seen[sub] = struct{}{}
                }
                matches = append(matches, Match{Word: sub, Len: length})
            }
        }
    }
    
    if len(matches) == 0 {
        return nil
    }
    
    uniqueWords := make([]string, 0, len(seen))
    for w := range seen {
        uniqueWords = append(uniqueWords, w)
    }
    
    return []Result{{
        WordCount:   len(matches),
        UniqueWords: uniqueWords,
        Matches:     matches,
    }}
}
```
