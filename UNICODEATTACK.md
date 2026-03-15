# 📐 Coax Unicode Attack Detection System
## Architecture Specification Document v1.0

**Document Purpose:** Guide Qwen Code in architecting and implementing a comprehensive Unicode attack detection system for the Coax security scanner.

**Target Implementation:** Full production-ready system (2-3 week timeline)

**Date:** March 16, 2026

---

## 🎯 1. Executive Summary

### 1.1 Vision

Build a **best-in-class Unicode attack detection system** that positions Coax as the leading open-source security scanner for modern supply chain attacks. This system should detect not only current threats (Glassworm, homoglyphs, zero-width injections) but be **architecturally extensible** for future Unicode-based attack vectors.

### 1.2 Strategic Goals

| Goal | Success Metric | Timeline |
|------|---------------|----------|
| **Detect Glassworm-style attacks** | 100% detection of known patterns | Week 1 |
| **Detect homoglyph attacks** | 95%+ accuracy on confusables | Week 2 |
| **Zero false positives on legitimate Unicode** | <1% FP on i18n codebases | Week 2 |
| **Performance impact** | <10% scan time increase | Week 3 |
| **Market differentiation** | Feature parity/better than Aikido | Week 3 |

### 1.3 Key Design Principles

```
┌─────────────────────────────────────────────────────────────────┐
│                    ARCHITECTURE PRINCIPLES                       │
├─────────────────────────────────────────────────────────────────┤
│  1. MODULARITY      │ Each attack vector is an independent module │
│  2. EXTENSIBILITY   │ New attack types addable without core changes│
│  3. PERFORMANCE     │ Unicode scanning ≤10% of total scan time    │
│  4. ACCURACY        │ Prioritize precision over recall (low FP)   │
│  5. CONFIGURABILITY │ Users can tune sensitivity per project      │
│  6. EXPLAINABILITY  │ Every finding includes remediation guidance │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🌍 2. Threat Landscape Overview

### 2.1 Current Unicode Attack Vectors (2024-2026)

| Attack Type | Unicode Ranges | Real-World Examples | Detection Priority |
|-------------|---------------|---------------------|-------------------|
| **Glassworm** | U+FE00-U+FE0F, U+E0100-U+E01EF | GitHub, npm, VSCode (2025) | 🔴 CRITICAL |
| **Zero-Width Injection** | U+200B-U+200F | Shell scripts, configs | 🔴 CRITICAL |
| **Homoglyph/Spoofing** | Cyrillic, Greek, Armenian | Package names, variables | 🟡 HIGH |
| **Bidirectional Override** | U+202A-U+202E | Comments, strings | 🟡 HIGH |
| **Unicode Tags** | U+E0000-U+E007F | Metadata injection | 🟠 MEDIUM |
| **Emoji Obfuscation** | Various emoji | Variable names | 🟠 MEDIUM |
| **Normalization Attacks** | NFC/NFD/NFKC/NFKD | Path traversal | 🟠 MEDIUM |

### 2.2 Anticipated Future Attack Vectors (2026-2028)

| Attack Type | Description | Preparation Needed |
|-------------|-------------|-------------------|
| **AI-Generated Homoglyphs** | LLMs suggesting lookalike variables | Configurable confusables database |
| **Multi-Script Blending** | Mixing 3+ scripts in identifiers | Script boundary detection |
| **Emoji-Based Steganography** | Hidden data in emoji sequences | Emoji sequence analyzer |
| **Font-Dependent Attacks** | Rendering-based spoofing | Font context awareness |
| **IDE-Specific Exploits** | Editor-specific Unicode handling | IDE integration hooks |

### 2.3 Threat Intelligence Integration

```
┌─────────────────────────────────────────────────────────────────┐
│              THREAT INTELLIGENCE PIPELINE                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  [External Sources]          [Coax Unicode System]              │
│  ┌─────────────────┐        ┌─────────────────┐                │
│  │ GitHub Advisories│───────▶│  Pattern Updater│                │
│  │ NIST NVD        │        │                 │                │
│  │ Aikido Research │───────▶│  Config Generator│                │
│  │ Academic Papers │        │                 │                │
│  │ CVE Database    │───────▶│  Alert System   │                │
│  └─────────────────┘        └─────────────────┘                │
│                                  │                              │
│                                  ▼                              │
│                          ┌─────────────────┐                    │
│                          │  Pattern Cache  │                    │
│                          │  (Auto-Update)  │                    │
│                          └─────────────────┘                    │
└─────────────────────────────────────────────────────────────────┘
```

---

## 🏗️ 3. System Architecture

### 3.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────────┐
│                         COAX SCANNER CORE                                │
├─────────────────────────────────────────────────────────────────────────┤
│                                                                          │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐              │
│  │   File       │    │   Content    │    │   Report     │              │
│  │   Loader     │───▶│   Pipeline   │───▶│   Generator  │              │
│  └──────────────┘    └──────────────┘    └──────────────┘              │
│                           │                                              │
│                           ▼                                              │
│              ┌────────────────────────────┐                             │
│              │     DETECTION ENGINE       │                             │
│              ├────────────────────────────┤                             │
│              │ ┌────────────────────────┐ │                             │
│              │ │   Unicode Scanner      │ │ ◀── NEW                    │
│              │ │   (This Specification) │ │                             │
│              │ └────────────────────────┘ │                             │
│              │ ┌────────────────────────┐ │                             │
│              │ │   Secret Pattern Scan  │ │                             │
│              │ └────────────────────────┘ │                             │
│              │ ┌────────────────────────┐ │                             │
│              │ │   Entropy Filter       │ │                             │
│              │ └────────────────────────┘ │                             │
│              │ ┌────────────────────────┐ │                             │
│              │ │   Context Analyzer     │ │                             │
│              │ └────────────────────────┘ │                             │
│              └────────────────────────────┘                             │
│                                                                          │
└─────────────────────────────────────────────────────────────────────────┘
```

### 3.2 Module Structure

```
crates/
└── coax-scanner/
    ├── src/
    │   ├── lib.rs                    # Existing - add unicode module export
    │   ├── scanner.rs                # Existing - integrate unicode pipeline
    │   ├── config.rs                 # Existing - add unicode config options
    │   ├── findings.rs               # Existing - add UnicodeFinding type
    │   │
    │   ├── unicode/                  # NEW DIRECTORY
    │   │   ├── mod.rs                # Module root, re-exports
    │   │   ├── scanner.rs            # Main UnicodeScanner struct
    │   │   ├── detectors/            # Individual attack detectors
    │   │   │   ├── mod.rs
    │   │   │   ├── invisible.rs      # Glassworm, zero-width
    │   │   │   ├── homoglyph.rs      # Confusables detection
    │   │   │   ├── bidi.rs           # Bidirectional overrides
    │   │   │   ├── tags.rs           # Unicode tag detection
    │   │   │   ├── glassworm.rs      # Glassworm-specific patterns
    │   │   │   └── normalization.rs  # NFC/NFD attacks
    │   │   ├── confusables/          # Confusables data & utilities
    │   │   │   ├── mod.rs
    │   │   │   ├── data.rs           # Generated confusables table
    │   │   │   └── loader.rs         # Data loading utilities
    │   │   ├── ranges.rs             # Unicode range definitions
    │   │   ├── config.rs             # Unicode-specific configuration
    │   │   └── reporter.rs           # Unicode finding formatting
    │   │
    │   └── ... (existing modules)
    │
    └── Cargo.toml                    # Add unicode dependencies
```

### 3.3 Data Flow

```
┌──────────────────────────────────────────────────────────────────────────┐
│                        UNICODE SCAN PIPELINE                              │
├──────────────────────────────────────────────────────────────────────────┤
│                                                                           │
│  ┌─────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │
│  │  File   │───▶│  Pre-Filter │───▶│  Detector   │───▶│  Post-Filter│   │
│  │  Read   │    │  (Skip list)│    │  Pipeline   │    │  (Allowlist)│   │
│  └─────────┘    └─────────────┘    └─────────────┘    └─────────────┘   │
│       │                │                  │                  │           │
│       │                │                  │                  │           │
│       ▼                ▼                  ▼                  ▼           │
│  ┌─────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐   │
│  │ Content │    │ - File type │    │ - Invisible │    │ - Allowlist │   │
│  │ + Path  │    │ - File size │    │ - Homoglyph │    │ - Threshold │   │
│  │         │    │ - Encoding  │    │ - Bidi      │    │ - Severity  │   │
│  └─────────┘    └─────────────┘    │ - Tags      │    └─────────────┘   │
│                                    │ - Glassworm │          │           │
│                                    └─────────────┘          │           │
│                                           │                 │           │
│                                           ▼                 ▼           │
│                                    ┌─────────────────────────────┐     │
│                                    │      FINDING AGGREGATOR     │     │
│                                    │  - Deduplication            │     │
│                                    │  - Severity scoring         │     │
│                                    │  - Context enrichment       │     │
│                                    └─────────────────────────────┘     │
│                                                   │                     │
│                                                   ▼                     │
│                                    ┌─────────────────────────────┐     │
│                                    │       OUTPUT FORMATTER      │     │
│                                    │  - JSON / SARIF / TUI       │     │
│                                    └─────────────────────────────┘     │
│                                                                           │
└──────────────────────────────────────────────────────────────────────────┘
```

---

## 📦 4. Module Specifications

### 4.1 Core UnicodeScanner

**File:** `crates/coax-scanner/src/unicode/scanner.rs`

```rust
/// Primary entry point for Unicode attack detection
/// 
/// Architecture Notes:
/// - Thread-safe for parallel scanning (use Arc<Mutex<>> if needed)
/// - Configurable sensitivity levels
/// - Pluggable detector architecture
/// - Minimal memory footprint (<50MB for confusables table)

pub struct UnicodeScanner {
    config: UnicodeConfig,
    detectors: Vec<Box<dyn UnicodeDetector>>,
    confusables_db: ConfusablesDatabase,
    stats: UnicodeScanStats,
}

pub trait UnicodeDetector: Send + Sync {
    fn name(&self) -> &'static str;
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    fn severity(&self) -> Severity;
    fn is_enabled(&self, config: &UnicodeConfig) -> bool;
}

pub struct UnicodeFinding {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub code_point: u32,
    pub character: String,
    pub category: UnicodeCategory,
    pub severity: Severity,
    pub description: String,
    pub remediation: String,
    pub cwe_id: Option<String>,  // CWE classification if applicable
    pub references: Vec<String>,  // Links to research/advisories
}

pub enum UnicodeCategory {
    InvisibleCharacter,
    Homoglyph,
    BidirectionalOverride,
    UnicodeTag,
    NormalizationAttack,
    GlasswormPattern,
    EmojiObfuscation,
    Unknown,
}
```

**Key Requirements:**
- [ ] Implement `Send + Sync` for parallel scanning
- [ ] Support hot-reloading of detector configurations
- [ ] Include scan statistics (chars scanned, findings, time)
- [ ] Provide finding deduplication

---

### 4.2 Detector Modules

#### 4.2.1 Invisible Character Detector

**File:** `crates/coax-scanner/src/unicode/detectors/invisible.rs`

```rust
/// Detects invisible Unicode characters used in Glassworm-style attacks
/// 
/// Unicode Ranges to Monitor:
/// - U+FE00-U+FE0F: Variation Selectors (Glassworm primary)
/// - U+E0100-U+E01EF: Variation Selectors Supplement
/// - U+200B-U+200F: Zero-width space, joiner, non-joiner
/// - U+2060-U+206F: Word joiner, invisible operators
/// - U+E0000-U+E007F: Tags

pub struct InvisibleCharDetector {
    ranges: Vec<(u32, u32)>,
    skip_contexts: Vec<Regex>,  // Legitimate uses to skip
}

impl InvisibleCharDetector {
    /// Returns true if character should be flagged
    pub fn is_suspicious(&self, code_point: u32, context: &str) -> bool;
    
    /// Check if context suggests legitimate use (emoji, i18n, etc.)
    pub fn is_legitimate_context(&self, context: &str) -> bool;
}
```

**Configuration Options:**
```yaml
# config/unicode/invisible.yaml
detector:
  enabled: true
  severity: critical
  ranges:
    - start: 0xFE00
      end: 0xFE0F
      label: "Variation Selectors"
    - start: 0xE0100
      end: 0xE01EF
      label: "Variation Selectors Supplement"
  allowlist_contexts:
    - "emoji_variation"      # Emoji skin tone modifiers
    - "cjk_variation"        # CJK character variants
    - "mathematical_notation" # Math symbols
```

---

#### 4.2.2 Homoglyph Detector

**File:** `crates/coax-scanner/src/unicode/detectors/homoglyph.rs`

```rust
/// Detects confusable characters that could be used for spoofing
/// 
/// Data Source: unicode-confusables-data or custom generated table
/// Performance: O(1) lookup per character using HashMap

pub struct HomoglyphDetector {
    confusables: HashMap<char, Vec<char>>,  // char -> possible base chars
    reverse_map: HashMap<char, char>,        // confusable -> canonical
    min_confidence: f32,
}

pub struct ConfusableMatch {
    pub suspicious_char: char,
    pub base_char: char,
    pub confidence: f32,  // 0.0-1.0 similarity score
    pub script_source: String,  // e.g., "Cyrillic", "Greek"
    pub visual_similarity: f32,
}

impl HomoglyphDetector {
    /// Load confusables data from embedded or external source
    pub fn load_data() -> Result<Self>;
    
    /// Check if character is confusable with ASCII equivalent
    pub fn is_confusable(&self, ch: char) -> Option<ConfusableMatch>;
    
    /// Scan identifier for homoglyph attacks
    pub fn scan_identifier(&self, identifier: &str) -> Vec<ConfusableMatch>;
}
```

**Data Generation Script:**
```bash
# scripts/generate-confusables.py
# Generate confusables table from Unicode data
# Output: crates/coax-scanner/src/unicode/confusables/data.rs
```

---

#### 4.2.3 Bidirectional Override Detector

**File:** `crates/coax-scanner/src/unicode/detectors/bidi.rs`

```rust
/// Detects bidirectional text overrides that can reverse displayed text
/// 
/// Critical for: Comments, strings, documentation
/// Less critical: Binary files, images

pub struct BidiDetector {
    override_chars: Vec<char>,
    embedding_chars: Vec<char>,
    isolation_chars: Vec<char>,
}

// Unicode ranges:
// U+202A: Left-to-Right Embedding (LRE)
// U+202B: Right-to-Left Embedding (RLE)
// U+202C: Pop Directional Formatting (PDF)
// U+202D: Left-to-Right Override (LRO)
// U+202E: Right-to-Left Override (RLO) - MOST DANGEROUS
// U+2066-U+2069: Isolation characters
```

---

#### 4.2.4 Glassworm-Specific Detector

**File:** `crates/coax-scanner/src/unicode/detectors/glassworm.rs`

```rust
/// Specialized detector for Glassworm attack patterns
/// 
/// Glassworm Characteristics [[1]]:
/// 1. Uses Variation Selectors (U+FE00-U+FE0F) to hide payloads
/// 2. Includes decoder function to extract hidden content
/// 3. Often uses eval() or Function() constructor
/// 4. Payload encoded in Buffer.from() or similar

pub struct GlasswormDetector {
    decoder_patterns: Vec<Regex>,
    eval_patterns: Vec<Regex>,
    encoding_patterns: Vec<Regex>,
}

impl GlasswormDetector {
    /// Detect characteristic Glassworm decoder pattern
    pub fn detect_decoder(&self, content: &str) -> Option<GlasswormIndicator>;
    
    /// Calculate confidence score based on multiple indicators
    pub fn calculate_confidence(&self, indicators: &[GlasswormIndicator]) -> f32;
}

pub struct GlasswormIndicator {
    pub indicator_type: String,
    pub location: SourceLocation,
    pub snippet: String,
    pub confidence: f32,
}
```

**Decoder Patterns to Detect:**
```javascript
// Pattern 1: Code point extraction
.map(c => c.codePointAt(0))
.filter(c => c !== null)

// Pattern 2: Buffer decoding
Buffer.from(hidden, 'hex').toString()

// Pattern 3: eval with decoding
eval(Buffer.from(...))

// Pattern 4: String.fromCharCode with array
String.fromCharCode(...array)
```

---

### 4.3 Configuration System

**File:** `crates/coax-scanner/src/unicode/config.rs`

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnicodeConfig {
    /// Enable Unicode scanning
    pub enabled: bool,
    
    /// Sensitivity level: low, medium, high, critical
    pub sensitivity: SensitivityLevel,
    
    /// Enable specific detectors
    pub detectors: DetectorConfig,
    
    /// File types to scan (glob patterns)
    pub include_patterns: Vec<String>,
    
    /// File types to exclude
    pub exclude_patterns: Vec<String>,
    
    /// Allowlist for legitimate Unicode uses
    pub allowlist: AllowlistConfig,
    
    /// Performance tuning
    pub performance: PerformanceConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectorConfig {
    pub invisible_chars: bool,
    pub homoglyphs: bool,
    pub bidirectional: bool,
    pub unicode_tags: bool,
    pub glassworm: bool,
    pub normalization: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowlistConfig {
    /// Files to always skip
    pub files: Vec<String>,
    
    /// Directories to always skip
    pub directories: Vec<String>,
    
    /// Character ranges to allow (for i18n projects)
    pub character_ranges: Vec<(u32, u32)>,
    
    /// Scripts to allow (e.g., "Han", "Hangul" for Asian language projects)
    pub allowed_scripts: Vec<String>,
}
```

**Default Configuration File:**
```yaml
# config/unicode/default.yaml
unicode:
  enabled: true
  sensitivity: high
  
  detectors:
    invisible_chars: true
    homoglyphs: true
    bidirectional: true
    unicode_tags: true
    glassworm: true
    normalization: false  # Enable only for high-security projects
    
  include_patterns:
    - "**/*.js"
    - "**/*.ts"
    - "**/*.py"
    - "**/*.rs"
    - "**/*.go"
    - "**/*.java"
    - "**/*.sh"
    - "**/*.yaml"
    - "**/*.json"
    
  exclude_patterns:
    - "**/*.min.js"
    - "**/vendor/**"
    - "**/node_modules/**"
    - "**/*.lock"
    - "**/package-lock.json"
    
  allowlist:
    files:
      - "**/i18n/**"
      - "**/locales/**"
      - "**/translations/**"
    allowed_scripts:
      - "Han"      # Chinese
      - "Hangul"   # Korean
      - "Hiragana" # Japanese
      - "Katakana" # Japanese
```

---

### 4.4 Integration Points

#### 4.4.1 Scanner Integration

**File:** `crates/coax-scanner/src/scanner.rs`

```rust
/// Integration points for Unicode scanning in main pipeline

impl Scanner {
    pub fn scan_file(&self, file_path: &str) -> Result<ScanResult> {
        let content = self.read_file(file_path)?;
        
        // NEW: Unicode pre-scan (before pattern matching)
        let unicode_findings = if self.config.unicode.enabled {
            self.unicode_scanner.scan(&content, file_path)
        } else {
            Vec::new()
        };
        
        // Critical Unicode findings can short-circuit further scanning
        if unicode_findings.iter().any(|f| f.severity == Severity::Critical) {
            // Optionally return early for critical findings
        }
        
        // Existing secret detection pipeline
        let secret_findings = self.scan_content(&content, file_path)?;
        
        Ok(ScanResult {
            file: file_path.to_string(),
            unicode_findings,
            secret_findings,
            stats: self.collect_stats(),
        })
    }
}
```

#### 4.4.2 CLI Integration

**File:** `crates/coax-cli/src/main.rs`

```rust
/// New CLI flags for Unicode scanning

#[derive(Parser)]
struct Args {
    // ... existing args
    
    /// Enable Unicode attack detection
    #[arg(long, default_value = "true")]
    unicode_scan: bool,
    
    /// Unicode sensitivity level (low, medium, high, critical)
    #[arg(long, default_value = "high")]
    unicode_sensitivity: String,
    
    /// Only scan for Unicode attacks (skip secret detection)
    #[arg(long)]
    unicode_only: bool,
    
    /// Output Unicode findings separately
    #[arg(long)]
    unicode_output: Option<String>,
    
    /// List all Unicode detectors
    #[arg(long)]
    list_unicode_detectors: bool,
}
```

#### 4.4.3 TUI Integration

**File:** `crates/coax-cli/src/tui.rs`

```rust
/// TUI display enhancements for Unicode findings

impl TuiDisplay {
    fn render_unicode_finding(&self, finding: &UnicodeFinding) {
        // Show invisible characters with special markers
        // e.g., [U+FE00] displayed as ⚠️ or similar
        
        // Highlight the suspicious character position
        // Show the actual code point value
        
        // Display remediation guidance inline
    }
    
    fn render_character_visualization(&self, code_point: u32) -> String {
        // Show: U+XXXX [character if visible] (description)
        // e.g., "U+FE00 [︀] (Variation Selector-1)"
    }
}
```

#### 4.4.4 Output Format Integration

**File:** `crates/coax-scanner/src/findings.rs`

```rust
/// Extend existing finding types to include Unicode findings

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Finding {
    Secret(SecretFinding),
    Vulnerability(VulnerabilityFinding),
    Unicode(UnicodeFinding),  // NEW
}

/// SARIF output support
impl UnicodeFinding {
    pub fn to_sarif(&self) -> sarif::Result {
        sarif::Result {
            rule_id: format!("UNICODE-{}", self.category.as_str()),
            level: self.severity.to_sarif_level(),
            message: sarif::Message { text: self.description.clone() },
            locations: vec![self.to_sarif_location()],
            // ...
        }
    }
}
```

---

## 🧪 5. Testing Strategy

### 5.1 Test Categories

| Category | Description | Test Count Target |
|----------|-------------|-------------------|
| **Unit Tests** | Individual detector logic | 50+ |
| **Integration Tests** | Full pipeline scanning | 20+ |
| **False Positive Tests** | Legitimate Unicode usage | 30+ |
| **True Positive Tests** | Known attack patterns | 20+ |
| **Performance Tests** | Scan time, memory usage | 10+ |
| **Regression Tests** | Previously fixed issues | Ongoing |

### 5.2 Test Data Structure

```
qa/
└── unicode/
    ├── test-cases/
    │   ├── glassworm/
    │   │   ├── sample-001.js          # Known Glassworm pattern
    │   │   ├── sample-002.ts
    │   │   └── expected-findings.json
    │   ├── homoglyph/
    │   │   ├── cyrillic-a.py          # Cyrillic 'а' vs Latin 'a'
    │   │   ├── greek-o.rs             # Greek 'ο' vs Latin 'o'
    │   │   └── expected-findings.json
    │   ├── invisible/
    │   │   ├── zero-width.sh
    │   │   ├── variation-selector.css
    │   │   └── expected-findings.json
    │   ├── bidi/
    │   │   ├── rlo-attack.txt
    │   │   └── expected-findings.json
    │   └── legitimate/
    │       ├── emoji-readme.md        # Should NOT flag
    │       ├── chinese-comments.py    # Should NOT flag
    │       ├── japanese-vars.js       # Should NOT flag
    │       └── expected-findings.json # Empty
    │
    ├── benchmarks/
    │   ├── large-repo/                # 100K+ line codebase
    │   └── performance-expected.json
    │
    └── regression/
        └── previously-fixed-issues/
```

### 5.3 Test Implementation

```rust
// crates/coax-scanner/src/unicode/tests/mod.rs

#[cfg(test)]
mod unicode_tests {
    use super::*;
    
    #[test]
    fn test_glassworm_detection() {
        let scanner = UnicodeScanner::with_default_config();
        let content = include_str!("../../../../qa/unicode/test-cases/glassworm/sample-001.js");
        let findings = scanner.scan(content, "test.js");
        
        assert!(findings.iter().any(|f| f.category == UnicodeCategory::GlasswormPattern));
        assert!(findings.iter().any(|f| f.severity == Severity::Critical));
    }
    
    #[test]
    fn test_no_false_positives_on_legitimate_unicode() {
        let scanner = UnicodeScanner::with_default_config();
        let content = include_str!("../../../../qa/unicode/test-cases/legitimate/chinese-comments.py");
        let findings = scanner.scan(content, "test.py");
        
        // Should have 0 findings for legitimate i18n content
        assert_eq!(findings.len(), 0, "Legitimate Unicode should not be flagged");
    }
    
    #[test]
    fn test_homoglyph_detection_accuracy() {
        // Test known confusable pairs
        let test_cases = vec![
            ('а', 'a', "Cyrillic"),  // Cyrillic а vs Latin a
            ('ο', 'o', "Greek"),     // Greek ο vs Latin o
            ('е', 'e', "Cyrillic"),  // Cyrillic е vs Latin e
        ];
        
        let detector = HomoglyphDetector::load_data().unwrap();
        
        for (confusable, base, script) in test_cases {
            let result = detector.is_confusable(confusable);
            assert!(result.is_some(), "Should detect {} as confusable", confusable);
            assert_eq!(result.unwrap().base_char, base);
        }
    }
    
    #[test]
    fn test_performance_large_file() {
        let scanner = UnicodeScanner::with_default_config();
        let content = include_str!("../../../../qa/unicode/benchmarks/large-repo/main.js");
        
        let start = std::time::Instant::now();
        let findings = scanner.scan(content, "large.js");
        let elapsed = start.elapsed();
        
        // Should complete in <100ms for 10K lines
        assert!(elapsed < std::time::Duration::from_millis(100));
        
        // Memory should stay reasonable
        // (Add memory profiling if available)
    }
}
```

### 5.4 CI/CD Integration

```yaml
# .github/workflows/unicode-tests.yml

name: Unicode Detection Tests

on:
  push:
    paths:
      - "crates/coax-scanner/src/unicode/**"
      - "qa/unicode/**"
  pull_request:
    paths:
      - "crates/coax-scanner/src/unicode/**"
      - "qa/unicode/**"

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Run Unicode unit tests
        run: cargo test -p coax-scanner unicode -- --nocapture
      
      - name: Run Unicode integration tests
        run: cargo test -p coax-scanner unicode_integration -- --nocapture
      
      - name: Run false positive test suite
        run: ./scripts/run-fp-tests.sh unicode
      
      - name: Performance benchmark
        run: cargo bench -p coax-scanner unicode_bench
      
      - name: Upload test results
        uses: actions/upload-artifact@v4
        with:
          name: unicode-test-results
          path: target/criterion/
```

---

## ⚡ 6. Performance Requirements

### 6.1 Performance Targets

| Metric | Target | Maximum Acceptable |
|--------|--------|-------------------|
| **Scan overhead** | <5% | <10% |
| **Memory usage** | <50MB | <100MB |
| **Confusables lookup** | O(1) | O(log n) |
| **Large file (10K lines)** | <100ms | <500ms |
| **Repository (100K lines)** | <2s | <10s |

### 6.2 Optimization Strategies

```rust
/// Performance optimization checklist

// 1. Pre-compile all regexes (lazy_static or once_cell)
lazy_static! {
    static ref INVISIBLE_PATTERN: Regex = Regex::new(...).unwrap();
}

// 2. Use Aho-Corasick for multi-pattern matching
use aho_corasick::AhoCorasick;

// 3. Parallel scanning for large files
use rayon::prelude::*;
content.par_lines().for_each(|line| { ... });

// 4. Early exit on critical findings (configurable)
if config.exit_on_critical && finding.severity == Critical {
    return findings;
}

// 5. Cache confusables lookups
lru_cache! {
    confusables_cache: HashMap<char, Option<ConfusableMatch>>
}

// 6. Skip binary files early
if is_binary(content) {
    return Vec::new();
}
```

### 6.3 Benchmarking Suite

```rust
// crates/coax-scanner/benches/unicode_bench.rs

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_invisible_char_detection(c: &mut Criterion) {
    let scanner = UnicodeScanner::with_default_config();
    let content = include_str!("../../qa/unicode/benchmarks/large-repo/main.js");
    
    c.bench_function("invisible_char_10k_lines", |b| {
        b.iter(|| scanner.scan_invisible(black_box(content), "test.js"))
    });
}

fn bench_homoglyph_detection(c: &mut Criterion) {
    let detector = HomoglyphDetector::load_data().unwrap();
    let identifiers = vec!["variable_name", "функция", "συνάρτηση"];
    
    c.bench_function("homoglyph_lookup", |b| {
        b.iter(|| {
            for id in &identifiers {
                for ch in id.chars() {
                    detector.is_confusable(ch);
                }
            }
        })
    });
}

criterion_group!(benches, bench_invisible_char_detection, bench_homoglyph_detection);
criterion_main!(benches);
```

---

## 📚 7. Documentation Requirements

### 7.1 User Documentation

```
docs/
└── unicode-detection/
    ├── README.md              # Overview and quickstart
    ├── configuration.md       # Configuration options
    ├── detectors.md           # Detector descriptions
    ├── false-positives.md     # Handling false positives
    ├── remediation.md         # How to fix findings
    └── examples/
        ├── glassworm-example.md
        ├── homoglyph-example.md
        └── legitimate-unicode.md
```

### 7.2 Developer Documentation

```rust
/// Each module should include:
/// 
/// 1. Module-level doc comment explaining purpose
/// 2. Architecture diagram (ASCII or link to external)
/// 3. Performance characteristics
/// 4. Known limitations
/// 5. Extension points for future development
/// 
/// Example:
/// 
/// /// Invisible Character Detector
/// /// 
/// /// Detects Unicode characters that are invisible or near-invisible
/// /// when rendered, commonly used in Glassworm-style supply chain attacks.
/// /// 
/// /// ## Architecture
/// /// 
/// /// ```text
/// /// Input → Range Check → Context Analysis → Finding
/// /// ```
/// /// 
/// /// ## Performance
/// /// 
/// /// - Time: O(n) where n = number of characters
/// /// - Space: O(1) beyond input storage
/// /// 
/// /// ## Limitations
/// /// 
/// /// - May flag legitimate emoji variation selectors
/// /// - Does not detect font-dependent invisibility
/// /// 
/// /// ## Extension
/// /// 
/// /// To add new ranges, update `INVISIBLE_RANGES` constant.
```

### 7.3 Security Advisory Documentation

```markdown
# Unicode Attack Detection Advisory

## CWE Classification
- CWE-172: Encoding Error
- CWE-176: Exposure of Sensitive Information Through Unicode
- CWE-956: Improper Input Validation

## References
- Aikido Security: Glassworm Returns (2025) [1]
- GitHub Security Advisory: [Link when available]
- Unicode Security Considerations: UTR #36

## Detection Coverage
| Attack Type | Detection Status | Confidence |
|-------------|-----------------|------------|
| Glassworm | ✅ Detected | High |
| Zero-Width | ✅ Detected | High |
| Homoglyph | ✅ Detected | Medium-High |
| Bidi Override | ✅ Detected | High |
```

---

## 🔮 8. Future Extensibility

### 8.1 Plugin Architecture (v2.0)

```rust
/// Design for future plugin-based detector system

pub trait UnicodeDetectorPlugin: Send + Sync {
    fn name(&self) -> &'static str;
    fn version(&self) -> &'static str;
    fn detect(&self, content: &str, file_path: &str) -> Vec<UnicodeFinding>;
    fn config_schema(&self) -> serde_json::Value;
}

// External plugins can be loaded at runtime
// Plugins stored in: ~/.coax/plugins/unicode/
```

### 8.2 ML-Based Detection (v2.0)

```rust
/// Future: Machine learning classifier for ambiguous cases

pub struct MLUnicodeClassifier {
    model: onnx_runtime::Session,
    threshold: f32,
}

impl MLUnicodeClassifier {
    /// Classify whether a finding is likely malicious
    pub fn classify(&self, features: &UnicodeFeatures) -> MLPrediction;
}

// Features to extract:
// - Character distribution
// - Script mixing patterns
// - Context semantics
// - File type patterns
```

### 8.3 Threat Intelligence Integration (v2.0)

```rust
/// Future: Auto-update from threat intelligence sources

pub struct ThreatIntelUpdater {
    sources: Vec<Box<dyn ThreatIntelSource>>,
    update_interval: Duration,
}

pub trait ThreatIntelSource: Send + Sync {
    fn name(&self) -> &'static str;
    fn fetch_patterns(&self) -> Result<Vec<UnicodePattern>>;
    fn last_updated(&self) -> Option<DateTime<Utc>>;
}

// Sources to integrate:
// - GitHub Security Advisories API
// - NIST NVD
// - Aikido Research Feed
// - Academic paper summaries
```

### 8.4 IDE Integration (v2.0)

```rust
/// Future: Real-time scanning in IDEs

// VS Code Extension
// - Inline warnings for Unicode issues
// - Quick-fix to remove/replace suspicious characters
// - Visual highlighting of invisible characters

// JetBrains Plugin
// - Same features as VS Code
// - Integration with IntelliJ security tools
```

---

## 📊 9. Success Metrics

### 9.1 Technical Metrics

| Metric | Baseline | Target (v1.0) | Target (v2.0) |
|--------|----------|---------------|---------------|
| **Detection Rate (Glassworm)** | 0% | 100% | 100% |
| **Detection Rate (Homoglyph)** | 0% | 95% | 98% |
| **False Positive Rate** | N/A | <1% | <0.5% |
| **Scan Overhead** | 0% | <5% | <3% |
| **Memory Usage** | 0MB | <50MB | <30MB |

### 9.2 Adoption Metrics

| Metric | Target (3 months) | Target (12 months) |
|--------|------------------|-------------------|
| **GitHub Stars** | +500 | +2000 |
| **Weekly Downloads** | 1000 | 5000 |
| **Enterprise Users** | 5 | 25 |
| **Security Advisories Citing Coax** | 2 | 10 |

### 9.3 Quality Metrics

| Metric | Target |
|--------|--------|
| **Test Coverage** | >90% |
| **Documentation Coverage** | 100% of public APIs |
| **Issue Response Time** | <48 hours |
| **Security Vulnerability Response** | <24 hours |

---

## 🗓️ 10. Implementation Timeline

### Phase 1: Core Foundation (Week 1)

| Day | Task | Deliverable |
|-----|------|-------------|
| 1-2 | Module structure, UnicodeScanner base | `unicode/mod.rs`, `scanner.rs` |
| 3 | Invisible character detector | `detectors/invisible.rs` |
| 4 | Configuration system | `config.rs`, YAML configs |
| 5 | Basic CLI integration | `--unicode-scan` flag |

### Phase 2: Detector Implementation (Week 2)

| Day | Task | Deliverable |
|-----|------|-------------|
| 1-2 | Homoglyph detector + confusables DB | `detectors/homoglyph.rs` |
| 3 | Bidi override detector | `detectors/bidi.rs` |
| 4 | Glassworm-specific detector | `detectors/glassworm.rs` |
| 5 | Integration + TUI updates | Full pipeline integration |

### Phase 3: Testing & Polish (Week 3)

| Day | Task | Deliverable |
|-----|------|-------------|
| 1-2 | Test suite implementation | 100+ test cases |
| 3 | Performance optimization | Benchmarks passing |
| 4 | Documentation | All docs complete |
| 5 | QA + bug fixes | v1.0 release candidate |

---

## 📋 11. Checklist for Qwen Code

### Architecture Checklist

- [ ] Module structure follows existing Coax patterns
- [ ] All detectors implement `UnicodeDetector` trait
- [ ] Configuration is YAML-based and extensible
- [ ] Thread-safe for parallel scanning
- [ ] Memory-efficient (<50MB for confusables)

### Implementation Checklist

- [ ] Invisible character detector (Glassworm ranges)
- [ ] Homoglyph detector with confusables database
- [ ] Bidirectional override detector
- [ ] Glassworm-specific pattern detector
- [ ] CLI flags for Unicode scanning
- [ ] TUI visualization for Unicode findings
- [ ] SARIF output support
- [ ] JSON output support

### Testing Checklist

- [ ] 50+ unit tests
- [ ] 20+ integration tests
- [ ] 30+ false positive test cases
- [ ] 20+ true positive test cases
- [ ] Performance benchmarks
- [ ] CI/CD pipeline integration

### Documentation Checklist

- [ ] Module-level doc comments
- [ ] User guide (docs/unicode-detection/)
- [ ] Configuration reference
- [ ] Remediation guide
- [ ] Security advisory documentation
- [ ] Example test cases with explanations

### Quality Checklist

- [ ] >90% test coverage
- [ ] All clippy warnings resolved
- [ ] All rustfmt formatting applied
- [ ] No unsafe code (unless absolutely necessary)
- [ ] Error messages are actionable
- [ ] Finding descriptions include remediation

---

## 🎯 12. Summary for Qwen Code

### Key Priorities

1. **Accuracy First:** Prioritize low false positive rate over detection breadth
2. **Modularity:** Each detector should be independently testable and replaceable
3. **Performance:** Unicode scanning should add <10% overhead
4. **Extensibility:** Design for future attack vectors without breaking changes
5. **Documentation:** Every finding should explain what, why, and how to fix

### Architecture Decisions to Make

| Decision | Recommendation | Rationale |
|----------|---------------|-----------|
| **Confusables data source** | Embed generated Rust code | No runtime dependency, faster lookup |
| **Regex engine** | Continue using `regex` crate | Consistent with existing codebase |
| **Parallel scanning** | Use Rayon (existing dependency) | Consistent with existing patterns |
| **Configuration format** | YAML (existing pattern) | Consistent with pattern configs |
| **Error handling** | Result<T, UnicodeError> | Consistent with existing patterns |

### Questions to Clarify Before Starting

1. Should Unicode scanning be enabled by default or opt-in?
2. What's the acceptable false positive rate threshold?
3. Should we integrate with existing secret detection or separate output?
4. Do we need real-time confusables database updates?
5. What's the priority order if timeline needs compression?

---

## 📎 Appendix A: Unicode Reference Tables

### A.1 Critical Unicode Ranges

| Range | Name | Attack Use | Priority |
|-------|------|------------|----------|
| U+FE00-U+FE0F | Variation Selectors | Glassworm | 🔴 Critical |
| U+E0100-U+E01EF | Variation Selectors Supplement | Glassworm | 🔴 Critical |
| U+200B-U+200F | Zero-Width & Directional | Injection | 🔴 Critical |
| U+202A-U+202E | Bidirectional Overrides | Text reversal | 🟡 High |
| U+E0000-U+E007F | Tags | Metadata injection | 🟠 Medium |
| U+2060-U+206F | Invisible Operators | Hidden content | 🟠 Medium |

### A.2 Common Confusable Pairs

| Confusable | Base | Script | Visual Similarity |
|------------|------|--------|-------------------|
| а (U+0430) | a | Cyrillic | 100% |
| о (U+043E) | o | Cyrillic | 100% |
| е (U+0435) | e | Cyrillic | 100% |
| ο (U+03BF) | o | Greek | 100% |
| ε (U+03B5) | e | Greek | 95% |
| а (U+05D0) | a | Hebrew | 90% |

---

## 📎 Appendix B: References

1. Aikido Security. "Glassworm Returns: Unicode Attack on GitHub, npm, VSCode." 2025. https://www.aikido.dev/blog/glassworm-returns-unicode-attack-github-npm-vscode

2. Unicode Consortium. "Unicode Security Considerations, UTR #36." https://unicode.org/reports/tr36/

3. CWE-172: Encoding Error. https://cwe.mitre.org/data/definitions/172.html

4. GitHub Security. "Homoglyph Attack Prevention." https://docs.github.com/en/security

---

**Document Version:** 1.0  
**Last Updated:** March 16, 2026  
**Author:** Security Architecture Review  
**Review Status:** Ready for Implementation