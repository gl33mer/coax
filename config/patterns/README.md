# Coax Pattern Configuration Files

This directory contains YAML pattern configuration files for the Coax security scanner.

## Pattern File Format

Each pattern file follows this YAML structure:

```yaml
patterns:
  - name: PATTERN_NAME
    regex: 'REGEX_PATTERN'
    severity: critical|high|medium|low
    recommendation: "Remediation recommendation"
    description: "Pattern description"
    cwe_id: "CWE-XXX"  # Optional
    confidence: high|medium|low  # Optional
    category: category_name  # Optional
    enabled: true|false
```

### Field Descriptions

| Field | Required | Description |
|-------|----------|-------------|
| `name` | Yes | Unique identifier for the pattern |
| `regex` | Yes | Regular expression pattern to match |
| `severity` | Yes | Severity level: `critical`, `high`, `medium`, `low` |
| `recommendation` | Yes | Action to take when pattern is detected |
| `description` | No | Human-readable description of what the pattern detects |
| `cwe_id` | No | Common Weakness Enumeration identifier |
| `confidence` | No | Confidence level: `high`, `medium`, `low` |
| `category` | No | Category for grouping related patterns |
| `enabled` | No | Whether the pattern is active (default: `true`) |

## Pattern Categories

| File | Category | Description |
|------|----------|-------------|
| `cloud_providers.yml` | cloud_provider | AWS, GCP, Azure, DigitalOcean, Heroku |
| `version_control.yml` | version_control | GitHub, GitLab, Bitbucket tokens |
| `payment_processors.yml` | payment | Stripe, Square, PayPal, Braintree |
| `communication_apis.yml` | communication | Slack, SendGrid, Twilio, Discord |
| `database_connections.yml` | database | PostgreSQL, MongoDB, MySQL, Redis |
| `private_keys.yml` | private_key | RSA, EC, SSH, PGP private keys |
| `ai_ml_apis.yml` | ai_ml | OpenAI, Anthropic, Hugging Face |

## Usage

### Loading Patterns Programmatically

```rust
use coax_scanner::PatternLoader;
use std::path::Path;

let mut loader = PatternLoader::new();

// Load from single file
loader.load_from_file(Path::new("config/patterns/cloud_providers.yml"))?;

// Load from directory (all .yml/.yaml files)
loader.load_from_directory(Path::new("config/patterns/"))?;

// Get patterns
let patterns = loader.get_patterns();
println!("Loaded {} patterns", patterns.len());

// Validate patterns
let validation = loader.validate_patterns();
if !validation.is_success() {
    for result in &validation.results {
        if !result.valid {
            eprintln!("Invalid pattern {}: {}", result.name, result.error.as_ref().unwrap());
        }
    }
}

// Filter by confidence
let high_confidence = loader.filter_by_confidence("high");

// Filter by category
let cloud_patterns = loader.filter_by_category("cloud_provider");

// Filter enabled patterns only
let enabled_patterns = loader.filter_enabled();
```

### Using with Scanner

```rust
use coax_scanner::{Scanner, PatternLoader, ScannerConfig};
use std::path::Path;

// Load patterns
let mut loader = PatternLoader::new();
loader.load_from_directory(Path::new("config/patterns/"))?;

// Filter to high-confidence patterns only
let loader = loader.filter_by_confidence("high");

// Create scanner with loaded patterns
let config = ScannerConfig::default().with_patterns(loader.into_patterns());
let scanner = Scanner::with_config(config);

// Scan
let results = scanner.scan_directory(Path::new("./src"));
```

## Adding New Patterns

1. Create a new YAML file or add to an existing category file
2. Follow the pattern format above
3. Test the regex pattern compiles correctly
4. Run `cargo test -p coax-scanner pattern_loader` to validate

### Example: Adding a New Pattern

```yaml
patterns:
  - name: MY_NEW_PATTERN
    regex: 'MY_REGEX_PATTERN'
    severity: high
    recommendation: "Remove and rotate the credential"
    description: "Description of what this detects"
    cwe_id: "CWE-798"
    confidence: high
    category: my_category
    enabled: true
```

## Pattern Sources

Patterns in this directory are sourced from:

1. **Coax Default Patterns** - Curated patterns from the Coax project
2. **secrets-patterns-db** - Imported from [mazen160/secrets-patterns-db](https://github.com/mazen160/secrets-patterns-db) (CC BY-SA 4.0)
3. **Community Contributions** - Patterns contributed by the community

## License

Patterns imported from secrets-patterns-db are licensed under CC BY-SA 4.0.
Original Coax patterns are licensed under the project's main license.

## Maintenance

- Regularly update patterns from secrets-patterns-db
- Review and remove patterns with high false positive rates
- Add new patterns for emerging services and credential types
- Test all patterns against known test cases
