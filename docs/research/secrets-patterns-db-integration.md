# Secrets-Patterns-DB Integration

## Overview

This document describes the integration of [mazen160/secrets-patterns-db](https://github.com/mazen160/secrets-patterns-db) into the Coax scanner, providing over 879 high-confidence secret detection patterns.

## Research Summary

### secrets-patterns-db Analysis

**Repository:** https://github.com/mazen160/secrets-patterns-db

**Key Statistics:**
- **Total patterns:** 1,610 (in rules-stable.yml)
- **High confidence:** 883 patterns
- **Low confidence:** 727 patterns
- **Format:** YAML
- **License:** CC BY-SA 4.0 (Creative Commons Attribution-ShareAlike)

**Pattern Categories:**
| Category | Count | Examples |
|----------|-------|----------|
| Generic API Keys | 650+ | Various service API keys |
| Cloud Providers | 22 | AWS, GCP, Azure, DigitalOcean |
| Communication | 24 | Slack, SendGrid, Twilio, Discord |
| Payment | 23 | Stripe, Square, PayPal |
| Version Control | 12 | GitHub, GitLab, Bitbucket |
| Private Keys | 7 | RSA, EC, SSH, PGP |
| Database | 3 | Connection strings |

**Pattern Format:**
```yaml
patterns:
  - pattern:
      name: AWS API Key
      regex: 'AKIA[0-9A-Z]{16}'
      confidence: high
```

**License Terms (CC BY-SA 4.0):**
- ✅ Commercial use allowed
- ✅ Modification allowed
- ✅ Distribution allowed
- ✅ Private use allowed
- ⚠️ Attribution required
- ⚠️ ShareAlike (derivatives must use same license)

## Architecture

### Pattern System Design

The Coax pattern system now supports:

1. **Built-in Patterns** - Hardcoded patterns in `secrets.rs`
2. **External YAML Patterns** - Load patterns from `.yml`/`.yaml` files
3. **secrets-patterns-db Import** - Converted patterns from the external database

### File Structure

```
coax/
├── crates/
│   └── coax-scanner/
│       └── src/
│           ├── pattern_cache.rs      # Pattern configuration & caching
│           ├── pattern_loader.rs     # YAML pattern loading
│           ├── scanner.rs            # Scanner with external pattern support
│           └── secrets.rs            # Built-in patterns
└── config/
    └── patterns/
        ├── README.md                 # Pattern format documentation
        ├── cloud_providers.yml       # AWS, GCP, Azure, etc.
        ├── version_control.yml       # GitHub, GitLab, Bitbucket
        ├── payment_processors.yml    # Stripe, Square, PayPal
        ├── communication_apis.yml    # Slack, SendGrid, Twilio
        ├── database_connections.yml  # PostgreSQL, MongoDB, MySQL
        ├── private_keys.yml          # RSA, EC, SSH, PGP
        ├── ai_ml_apis.yml            # OpenAI, Anthropic, HuggingFace
        └── secrets_patterns_db.yml   # Imported patterns (879 patterns)
```

### Pattern YAML Format

```yaml
patterns:
  - name: AWS_ACCESS_KEY
    regex: 'AKIA[0-9A-Z]{16}'
    severity: critical
    recommendation: "Remove immediately and rotate via AWS IAM Console"
    description: "AWS Access Key ID"
    cwe_id: "CWE-798"
    confidence: high
    category: cloud_provider
    enabled: true
```

**Fields:**
| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Unique identifier |
| `regex` | Yes | Regular expression pattern |
| `severity` | Yes | `critical`, `high`, `medium`, `low` |
| `recommendation` | Yes | Remediation guidance |
| `description` | No | Human-readable description |
| `cwe_id` | No | CWE identifier |
| `confidence` | No | `high`, `medium`, `low` |
| `category` | No | Pattern category |
| `enabled` | No | Enable/disable pattern (default: true) |

## Implementation

### PatternLoader Module

**Location:** `/home/shva/QwenDev/devshield-internal/coax/crates/coax-scanner/src/pattern_loader.rs`

**Key Features:**
- Load patterns from individual YAML files
- Load patterns from directories (recursive)
- Support for secrets-patterns-db format
- Pattern validation (regex compilation testing)
- Filtering by confidence, category, and enabled status
- Pattern merging from multiple sources

**API:**
```rust
use coax_scanner::PatternLoader;
use std::path::Path;

let mut loader = PatternLoader::new();

// Load from file
loader.load_from_file(Path::new("patterns/aws.yml"))?;

// Load from directory
loader.load_from_directory(Path::new("patterns/"))?;

// Validate patterns
let validation = loader.validate_patterns();
println!("Valid: {}/{}", validation.valid, validation.total);

// Filter by confidence
let high_conf = loader.filter_by_confidence("high");

// Get patterns
let patterns = loader.get_patterns();
```

### Scanner Configuration Updates

**New ScannerConfig Fields:**
```rust
pub struct ScannerConfig {
    // ... existing fields ...
    
    /// Load patterns from external YAML files
    pub load_external_patterns: bool,
    
    /// Pattern directory path
    pub pattern_directory: Option<PathBuf>,
    
    /// Minimum confidence level ("low", "medium", "high")
    pub min_confidence: String,
    
    /// Load secrets-patterns-db patterns
    pub enable_secrets_patterns_db: bool,
}
```

**Builder Methods:**
```rust
let config = ScannerConfig::default()
    .with_external_patterns(true)
    .with_pattern_directory(PathBuf::from("config/patterns/"))
    .with_min_confidence("high")
    .with_secrets_patterns_db(true);

let scanner = Scanner::with_config(config);
```

## Pattern Conversion

### Conversion Script

**Location:** `/home/shva/QwenDev/devshield-internal/coax/scripts/convert_spdb_to_coax.py`

**Usage:**
```bash
python3 scripts/convert_spdb_to_coax.py \
    /tmp/secrets-patterns-db/db/rules-stable.yml \
    config/patterns/secrets_patterns_db.yml \
    high
```

**Features:**
- Parses secrets-patterns-db YAML format
- Converts to Coax YAML format
- Adds severity, recommendations, categories
- Filters by confidence level
- Properly escapes regex patterns for YAML

### Conversion Results

**Input:** `rules-stable.yml` (1,610 patterns)
**Output:** `secrets_patterns_db.yml` (879 high-confidence patterns)

**Category Breakdown:**
| Category | Count |
|----------|-------|
| generic | 650 |
| api_key | 138 |
| communication | 24 |
| payment | 23 |
| cloud_provider | 22 |
| version_control | 12 |
| private_key | 7 |
| database | 3 |

**Validation Results:**
- Total: 879 patterns
- Valid: 876 patterns (99.7%)
- Invalid: 3 patterns (0.3% - regex syntax differences)

## Testing

### PatternLoader Tests

```bash
cargo test -p coax-scanner pattern_loader
```

**Test Coverage:**
- ✅ Pattern loader creation
- ✅ Load from single file
- ✅ Load from directory
- ✅ Pattern validation
- ✅ Filter by confidence
- ✅ Filter enabled patterns
- ✅ Merge loaders
- ✅ secrets-patterns-db format support
- ✅ Load actual secrets_patterns_db.yml file

### Test Results

```
running 9 tests
test pattern_loader::tests::test_pattern_loader_creation ... ok
test pattern_loader::tests::test_merge_loaders ... ok
test pattern_loader::tests::test_filter_enabled ... ok
test pattern_loader::tests::test_load_from_file ... ok
test pattern_loader::tests::test_filter_by_confidence ... ok
test pattern_loader::tests::test_load_from_directory ... ok
test pattern_loader::tests::test_secrets_patterns_db_format ... ok
test pattern_loader::tests::test_validate_patterns ... ok
test pattern_loader::tests::test_load_secrets_patterns_db_file ... ok

test result: ok. 9 passed; 0 failed
```

## Usage Examples

### Basic Usage - Load External Patterns

```rust
use coax_scanner::{Scanner, ScannerConfig};
use std::path::PathBuf;

// Load patterns from config/patterns/ directory
let config = ScannerConfig::default()
    .with_external_patterns(true)
    .with_pattern_directory(PathBuf::from("config/patterns/"))
    .with_min_confidence("high");

let scanner = Scanner::with_config(config);
let results = scanner.scan_directory(&PathBuf::from("./src"));

println!("Found {} secrets", results.len());
```

### Advanced Usage - Custom Pattern Loading

```rust
use coax_scanner::{Scanner, ScannerConfig, PatternLoader};
use std::path::Path;

// Load patterns manually
let mut loader = PatternLoader::new();
loader.load_from_file(Path::new("config/patterns/cloud_providers.yml"))?;
loader.load_from_file(Path::new("config/patterns/secrets_patterns_db.yml"))?;

// Filter to high-confidence only
let loader = loader.filter_by_confidence("high");

// Validate patterns
let validation = loader.validate_patterns();
if !validation.is_success() {
    eprintln!("Warning: {} invalid patterns", validation.invalid);
}

// Create scanner with loaded patterns
let config = ScannerConfig::default()
    .with_patterns(loader.into_patterns());

let scanner = Scanner::with_config(config);
```

### CLI Integration (Future)

```bash
# Scan with external patterns
coax scan --patterns config/patterns/ --min-confidence high ./src

# Scan with secrets-patterns-db
coax scan --spdb ./src

# Generate report
coax scan --patterns config/patterns/ --format sarif -o results.sarif ./src
```

## Maintenance

### Updating Patterns

1. **Update from secrets-patterns-db:**
   ```bash
   cd /tmp
   git clone https://github.com/mazen160/secrets-patterns-db.git
   cd coax
   python3 scripts/convert_spdb_to_coax.py \
       /tmp/secrets-patterns-db/db/rules-stable.yml \
       config/patterns/secrets_patterns_db.yml \
       high
   ```

2. **Validate patterns:**
   ```bash
   cargo test -p coax-scanner pattern_loader
   ```

3. **Test detection:**
   ```bash
   cargo test -p coax-scanner
   ```

### Adding New Patterns

1. Add to appropriate category file in `config/patterns/`
2. Or create new category file
3. Test regex compiles: `cargo test -p coax-scanner pattern_loader`
4. Test detection with sample data

### Pattern Quality Guidelines

- **High confidence:** Well-known patterns with low false positive rate
- **Medium confidence:** Patterns that may have some false positives
- **Low confidence:** Broad patterns, use with caution
- Always include clear recommendations
- Test against real codebases
- Document pattern source

## License Compliance

**secrets-patterns-db** is licensed under CC BY-SA 4.0.

**Requirements:**
1. ✅ Attribution: Include credit to mazen160/secrets-patterns-db
2. ✅ ShareAlike: Derivative works must use CC BY-SA 4.0
3. ✅ Notice: Include license text or link

**Implementation:**
- All converted patterns include header comments with source and license
- This documentation includes license information
- Pattern files include attribution comments

## Future Enhancements

1. **Pattern Updates:** Automated CI/CD for pattern updates from secrets-patterns-db
2. **Pattern Testing:** Test suite with known secrets samples
3. **False Positive Reduction:** Machine learning-based confidence scoring
4. **Custom Patterns:** User-defined pattern support via CLI
5. **Pattern Sharing:** Community pattern repository
6. **Performance:** Pattern optimization for faster matching

## References

- [secrets-patterns-db Repository](https://github.com/mazen160/secrets-patterns-db)
- [CC BY-SA 4.0 License](https://creativecommons.org/licenses/by-sa/4.0/)
- [Coax Scanner Documentation](../README.md)
- [Pattern Format Documentation](../config/patterns/README.md)
