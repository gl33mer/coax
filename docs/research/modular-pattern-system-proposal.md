# Modular Pattern System Proposal

**Date:** March 15, 2026  
**Author:** Coax Research Team  
**Status:** Proposal for Phase 2 Implementation

---

## Executive Summary

This proposal outlines a **modular, configurable pattern system** for Coax that enables:

1. **External pattern databases** (YAML/JSON format)
2. **User-defined custom patterns**
3. **Automatic pattern updates**
4. **Pattern versioning**
5. **Category-based filtering**

**Expected Benefits:**
- 37x pattern coverage increase (43 → 1,600+ patterns)
- Community-contributed patterns
- Faster pattern updates (no code changes required)
- Per-project pattern customization

---

## Current Architecture

### Hardcoded Patterns (Current)

```rust
// coax/crates/coax-scanner/src/secrets.rs
pub mod categories {
    pub const AWS: &[SecretPattern] = &[
        SecretPattern {
            name: "AWS_ACCESS_KEY",
            pattern: r"AKIA[0-9A-Z]{16}",
            severity: "critical",
            recommendation: "Remove immediately...",
            description: "AWS Access Key ID",
            cwe_id: Some("CWE-798"),
        },
        // ... more patterns
    ];
}

pub fn all_patterns() -> Vec<PatternConfig> {
    // Patterns compiled into binary
    // Requires code change + rebuild to update
}
```

**Limitations:**
- ❌ Patterns hardcoded in Rust source
- ❌ Requires code change + rebuild to add patterns
- ❌ No user customization
- ❌ No automatic updates
- ❌ No pattern versioning

---

## Proposed Architecture

### High-Level Design

```
┌─────────────────────────────────────────────────────────────┐
│                    Coax Scanner                             │
└─────────────────────────────────────────────────────────────┘
                            │
                            ▼
┌─────────────────────────────────────────────────────────────┐
│                  Pattern Manager                            │
│  • Load patterns from multiple sources                      │
│  • Merge and deduplicate                                    │
│  • Validate patterns                                        │
│  • Version tracking                                         │
└─────────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│  Built-in     │  │  External     │  │  User         │
│  Patterns     │  │  Databases    │  │  Custom       │
│  (compiled)   │  │  (YAML/JSON)  │  │  (YAML/JSON)  │
└───────────────┘  └───────────────┘  └───────────────┘
        │                   │                   │
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│ secrets.rs    │  │ secrets-      │  │ .coax/        │
│ (Rust code)   │  │ patterns-db   │  │ patterns.yaml │
│               │  │ (community)   │  │ (user custom) │
└───────────────┘  └───────────────┘  └───────────────┘
```

---

## Pattern Schema

### YAML Format (Recommended)

```yaml
# Pattern database metadata
metadata:
  name: "Coax Community Patterns"
  version: "2026.03.15"
  source: "https://github.com/coax/scanner-patterns"
  license: "MIT"
  last_updated: "2026-03-15T12:00:00Z"
  patterns_count: 1610

# Pattern categories
categories:
  - id: "cloud-providers"
    name: "Cloud Providers"
    enabled: true
    
  - id: "version-control"
    name: "Version Control"
    enabled: true

# Pattern definitions
patterns:
  - id: "aws-access-key"
    name: "AWS Access Key"
    description: "AWS Access Key ID"
    category: "cloud-providers"
    regex: "AKIA[0-9A-Z]{16}"
    severity: "critical"
    recommendation: "Remove immediately and rotate the key via AWS IAM Console"
    cwe_id: "CWE-798"
    enabled: true
    
    # Advanced filtering
    entropy:
      enabled: true
      min_threshold: 3.5
      
    token_efficiency:
      enabled: true
      threshold: 2.5
      
    keywords:
      - "AKIA"
      
    file_types:
      include: ["*.env", "*.yaml", "*.yml", "*.json", "*.py", "*.js"]
      exclude: ["*.lock", "*.min.js"]
      
    context:
      exclude_comments: true
      exclude_test_files: true
      exclude_example_keys: true

  - id: "github-pat"
    name: "GitHub Personal Access Token"
    description: "GitHub PAT (ghp_*)"
    category: "version-control"
    regex: "ghp_[a-zA-Z0-9]{36}"
    severity: "critical"
    recommendation: "Remove and regenerate the token in GitHub Settings"
    cwe_id: "CWE-798"
    enabled: true
    
    keywords:
      - "ghp_"
```

### JSON Format (Alternative)

```json
{
  "metadata": {
    "name": "Coax Community Patterns",
    "version": "2026.03.15",
    "source": "https://github.com/coax/scanner-patterns",
    "license": "MIT",
    "last_updated": "2026-03-15T12:00:00Z",
    "patterns_count": 1610
  },
  "categories": [
    {
      "id": "cloud-providers",
      "name": "Cloud Providers",
      "enabled": true
    }
  ],
  "patterns": [
    {
      "id": "aws-access-key",
      "name": "AWS Access Key",
      "description": "AWS Access Key ID",
      "category": "cloud-providers",
      "regex": "AKIA[0-9A-Z]{16}",
      "severity": "critical",
      "recommendation": "Remove immediately and rotate the key via AWS IAM Console",
      "cwe_id": "CWE-798",
      "enabled": true,
      "entropy": {
        "enabled": true,
        "min_threshold": 3.5
      },
      "token_efficiency": {
        "enabled": true,
        "threshold": 2.5
      },
      "keywords": ["AKIA"]
    }
  ]
}
```

---

## Implementation Design

### Rust Data Structures

```rust
// coax/crates/coax-scanner/src/patterns/mod.rs

use serde::{Deserialize, Serialize};
use regex::Regex;

/// Pattern database metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternMetadata {
    pub name: String,
    pub version: String,
    pub source: Option<String>,
    pub license: Option<String>,
    pub last_updated: Option<String>,
    pub patterns_count: usize,
}

/// Pattern category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternCategory {
    pub id: String,
    pub name: String,
    pub enabled: bool,
}

/// Entropy configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntropyConfig {
    pub enabled: bool,
    pub min_threshold: f64,
}

/// Token efficiency configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenEfficiencyConfig {
    pub enabled: bool,
    pub threshold: f64,
}

/// Context filtering configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConfig {
    pub exclude_comments: bool,
    pub exclude_test_files: bool,
    pub exclude_example_keys: bool,
}

/// Pattern definition
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDefinition {
    pub id: String,
    pub name: String,
    pub description: String,
    pub category: String,
    pub regex: String,
    pub severity: String,
    pub recommendation: String,
    pub cwe_id: Option<String>,
    pub enabled: bool,
    
    #[serde(default)]
    pub entropy: Option<EntropyConfig>,
    
    #[serde(default)]
    pub token_efficiency: Option<TokenEfficiencyConfig>,
    
    #[serde(default)]
    pub keywords: Vec<String>,
    
    #[serde(default)]
    pub file_types: Option<FileTypesConfig>,
    
    #[serde(default)]
    pub context: Option<ContextConfig>,
    
    #[serde(skip)]
    compiled_regex: Option<Regex>,
}

/// File types configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileTypesConfig {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
}

/// Complete pattern database
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatternDatabase {
    pub metadata: PatternMetadata,
    pub categories: Vec<PatternCategory>,
    pub patterns: Vec<PatternDefinition>,
}
```

### Pattern Manager

```rust
// coax/crates/coax-scanner/src/patterns/manager.rs

use std::path::{Path, PathBuf};
use std::fs;
use anyhow::Result;

pub struct PatternManager {
    /// Built-in patterns (compiled into binary)
    built_in_patterns: Vec<PatternDefinition>,
    
    /// External pattern databases
    external_databases: Vec<PatternDatabase>,
    
    /// User custom patterns
    user_patterns: Vec<PatternDefinition>,
    
    /// Merged and validated patterns
    merged_patterns: Vec<PatternDefinition>,
    
    /// Pattern cache directory
    cache_dir: PathBuf,
}

impl PatternManager {
    /// Create new pattern manager
    pub fn new() -> Self {
        Self {
            built_in_patterns: secrets::all_patterns(),
            external_databases: Vec::new(),
            user_patterns: Vec::new(),
            merged_patterns: Vec::new(),
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from(".cache"))
                .join("coax")
                .join("patterns"),
        }
    }
    
    /// Load patterns from all sources
    pub fn load_all(&mut self) -> Result<()> {
        // 1. Start with built-in patterns
        let mut all_patterns = self.built_in_patterns.clone();
        
        // 2. Load external databases
        self.load_external_databases()?;
        for db in &self.external_databases {
            for pattern in &db.patterns {
                if pattern.enabled {
                    all_patterns.push(pattern.clone());
                }
            }
        }
        
        // 3. Load user custom patterns
        self.load_user_patterns()?;
        for pattern in &self.user_patterns {
            if pattern.enabled {
                all_patterns.push(pattern.clone());
            }
        }
        
        // 4. Deduplicate by ID
        all_patterns = self.deduplicate_patterns(all_patterns);
        
        // 5. Validate and compile regexes
        all_patterns = self.validate_patterns(all_patterns)?;
        
        // 6. Store merged patterns
        self.merged_patterns = all_patterns;
        
        Ok(())
    }
    
    /// Load external pattern databases
    fn load_external_databases(&mut self) -> Result<()> {
        let db_paths = [
            // Community patterns (downloaded)
            self.cache_dir.join("community-patterns.yaml"),
            // Secrets-patterns-db
            self.cache_dir.join("secrets-patterns-db.yaml"),
        ];
        
        for path in &db_paths {
            if path.exists() {
                let content = fs::read_to_string(path)?;
                let db: PatternDatabase = serde_yaml::from_str(&content)?;
                self.external_databases.push(db);
            }
        }
        
        Ok(())
    }
    
    /// Load user custom patterns
    fn load_user_patterns(&mut self) -> Result<()> {
        let user_pattern_paths = [
            // Project-level patterns
            PathBuf::from(".coax/patterns.yaml"),
            // User-level patterns
            dirs::home_dir()
                .map(|d| d.join(".coax/patterns.yaml")),
        ];
        
        for path_opt in &user_pattern_paths {
            if let Some(path) = path_opt {
                if path.exists() {
                    let content = fs::read_to_string(path)?;
                    let db: PatternDatabase = serde_yaml::from_str(&content)?;
                    for pattern in db.patterns {
                        self.user_patterns.push(pattern);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Deduplicate patterns by ID
    fn deduplicate_patterns(&self, patterns: Vec<PatternDefinition>) -> Vec<PatternDefinition> {
        use std::collections::HashMap;
        
        let mut pattern_map: HashMap<String, PatternDefinition> = HashMap::new();
        
        for pattern in patterns {
            // User patterns override built-in patterns with same ID
            // External patterns are added if no conflict
            pattern_map.entry(pattern.id.clone())
                .or_insert(pattern);
        }
        
        pattern_map.into_values().collect()
    }
    
    /// Validate and compile patterns
    fn validate_patterns(&self, patterns: Vec<PatternDefinition>) -> Result<Vec<PatternDefinition>> {
        let mut validated = Vec::new();
        
        for mut pattern in patterns {
            // Compile regex
            match Regex::new(&pattern.regex) {
                Ok(regex) => {
                    pattern.compiled_regex = Some(regex);
                    validated.push(pattern);
                }
                Err(e) => {
                    eprintln!("Warning: Invalid regex for pattern '{}': {}", pattern.id, e);
                    // Skip invalid patterns
                }
            }
        }
        
        Ok(validated)
    }
    
    /// Get all patterns
    pub fn patterns(&self) -> &[PatternDefinition] {
        &self.merged_patterns
    }
    
    /// Get patterns by category
    pub fn patterns_by_category(&self, category: &str) -> Vec<&PatternDefinition> {
        self.merged_patterns
            .iter()
            .filter(|p| p.category == category)
            .collect()
    }
    
    /// Enable/disable category
    pub fn set_category_enabled(&mut self, category: &str, enabled: bool) -> Result<()> {
        for pattern in &mut self.merged_patterns {
            if pattern.category == category {
                pattern.enabled = enabled;
            }
        }
        Ok(())
    }
}
```

---

## Pattern Update System

### Update Mechanism

```rust
// coax/crates/coax-scanner/src/patterns/updater.rs

use reqwest;
use anyhow::Result;
use std::path::Path;

pub struct PatternUpdater {
    cache_dir: PathBuf,
}

impl PatternUpdater {
    pub fn new() -> Self {
        Self {
            cache_dir: dirs::cache_dir()
                .unwrap_or_else(|| PathBuf::from(".cache"))
                .join("coax")
                .join("patterns"),
        }
    }
    
    /// Check for pattern updates
    pub async fn check_for_updates(&self) -> Result<UpdateStatus> {
        // Check community patterns
        let community_status = self.check_community_patterns().await?;
        
        // Check secrets-patterns-db
        let spdb_status = self.check_secrets_patterns_db().await?;
        
        Ok(UpdateStatus {
            community_patterns: community_status,
            secrets_patterns_db: spdb_status,
        })
    }
    
    /// Download pattern updates
    pub async fn update_patterns(&self) -> Result<UpdateResult> {
        let mut result = UpdateResult::default();
        
        // Update community patterns
        if let Err(e) = self.download_community_patterns().await {
            result.errors.push(format!("Community patterns: {}", e));
        } else {
            result.updated.push("Community patterns");
        }
        
        // Update secrets-patterns-db
        if let Err(e) = self.download_secrets_patterns_db().await {
            result.errors.push(format!("Secrets-patterns-db: {}", e));
        } else {
            result.updated.push("Secrets-patterns-db");
        }
        
        Ok(result)
    }
    
    /// Download community patterns
    async fn download_community_patterns(&self) -> Result<()> {
        let url = "https://raw.githubusercontent.com/coax/scanner-patterns/main/patterns.yaml";
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        
        // Validate before saving
        let db: PatternDatabase = serde_yaml::from_str(&content)?;
        
        // Save to cache
        fs::create_dir_all(&self.cache_dir)?;
        let path = self.cache_dir.join("community-patterns.yaml");
        fs::write(path, content)?;
        
        Ok(())
    }
    
    /// Download secrets-patterns-db
    async fn download_secrets_patterns_db(&self) -> Result<()> {
        let url = "https://raw.githubusercontent.com/mazen160/secrets-patterns-db/main/db/rules-stable.yml";
        let response = reqwest::get(url).await?;
        let content = response.text().await?;
        
        // Convert format if needed
        // Save to cache
        fs::create_dir_all(&self.cache_dir)?;
        let path = self.cache_dir.join("secrets-patterns-db.yaml");
        fs::write(path, content)?;
        
        Ok(())
    }
}

#[derive(Debug)]
pub struct UpdateStatus {
    pub community_patterns: SourceStatus,
    pub secrets_patterns_db: SourceStatus,
}

#[derive(Debug)]
pub struct SourceStatus {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
}

#[derive(Debug, Default)]
pub struct UpdateResult {
    pub updated: Vec<String>,
    pub errors: Vec<String>,
}
```

### CLI Commands

```rust
// coax/crates/coax-cli/src/commands/patterns.rs

use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum PatternsCommand {
    /// List all available patterns
    List {
        /// Filter by category
        #[arg(short, long)]
        category: Option<String>,
        
        /// Show disabled patterns
        #[arg(long)]
        all: bool,
    },
    
    /// Check for pattern updates
    Check,
    
    /// Update pattern databases
    Update,
    
    /// Add custom pattern
    Add {
        /// Pattern file (YAML/JSON)
        #[arg(short, long)]
        file: Option<PathBuf>,
        
        /// Pattern name
        #[arg(long)]
        name: String,
        
        /// Pattern regex
        #[arg(long)]
        regex: String,
        
        /// Pattern severity
        #[arg(long, default_value = "high")]
        severity: String,
    },
    
    /// Enable/disable pattern category
    Category {
        /// Category name
        #[arg(short, long)]
        name: String,
        
        /// Enable or disable
        #[arg(short, long)]
        enable: bool,
    },
    
    /// Show pattern database info
    Info,
}

/// CLI handler
pub async fn run_patterns_command(cmd: PatternsCommand) -> Result<()> {
    match cmd {
        PatternsCommand::List { category, all } => {
            let manager = PatternManager::new();
            // List patterns...
        }
        PatternsCommand::Check => {
            let updater = PatternUpdater::new();
            let status = updater.check_for_updates().await?;
            // Show update status...
        }
        PatternsCommand::Update => {
            let updater = PatternUpdater::new();
            let result = updater.update_patterns().await?;
            // Show update result...
        }
        PatternsCommand::Add { file, name, regex, severity } => {
            // Add custom pattern...
        }
        PatternsCommand::Category { name, enable } => {
            // Enable/disable category...
        }
        PatternsCommand::Info => {
            // Show database info...
        }
    }
    Ok(())
}
```

---

## Directory Structure

```
~/.coax/
├── patterns/                    # Pattern cache directory
│   ├── community-patterns.yaml  # Downloaded community patterns
│   ├── secrets-patterns-db.yaml # Downloaded secrets-patterns-db
│   └── version.json             # Version tracking
│
└── config.yaml                  # User configuration

.project/
└── .coax/
    └── patterns.yaml            # Project-specific patterns
```

---

## Version Tracking

```yaml
# ~/.coax/patterns/version.json
{
  "community_patterns": {
    "version": "2026.03.15",
    "last_updated": "2026-03-15T12:00:00Z",
    "patterns_count": 500,
    "source": "https://github.com/coax/scanner-patterns"
  },
  "secrets_patterns_db": {
    "version": "2026.03.10",
    "last_updated": "2026-03-10T08:30:00Z",
    "patterns_count": 1610,
    "source": "https://github.com/mazen160/secrets-patterns-db"
  }
}
```

---

## Configuration File

```yaml
# ~/.coax/config.yaml

# Pattern sources
patterns:
  sources:
    - name: "community"
      url: "https://raw.githubusercontent.com/coax/scanner-patterns/main/patterns.yaml"
      enabled: true
      auto_update: true
      
    - name: "secrets-patterns-db"
      url: "https://raw.githubusercontent.com/mazen160/secrets-patterns-db/main/db/rules-stable.yml"
      enabled: true
      auto_update: false  # Manual update only
      
  # Category overrides
  categories:
    cloud-providers:
      enabled: true
    version-control:
      enabled: true
    payment:
      enabled: true
    ai-ml:
      enabled: false  # Disable AI/ML patterns
      
  # Pattern overrides
  overrides:
    aws-access-key:
      enabled: true
      severity: "critical"
    github-pat:
      enabled: true
      severity: "critical"
      
  # Custom patterns
  custom:
    - id: "my-company-api-key"
      name: "My Company API Key"
      regex: "MYCO_[a-zA-Z0-9]{32}"
      severity: "critical"
      recommendation: "Contact security team to rotate"

# Auto-update settings
auto_update:
  enabled: true
  frequency: "weekly"  # daily, weekly, monthly
  notify: true
```

---

## Migration Plan

### Phase 1: Foundation (Week 1)

**Goals:**
- Define pattern schema
- Implement pattern loader
- Add YAML/JSON support

**Tasks:**
1. Create `PatternDatabase` struct with serde
2. Implement `PatternManager::load_all()`
3. Add YAML loading support
4. Test with sample pattern files

**Deliverables:**
- Pattern schema defined
- Basic loader working
- Sample pattern files

### Phase 2: External Databases (Week 2)

**Goals:**
- Support secrets-patterns-db
- Implement pattern merging
- Add deduplication

**Tasks:**
1. Download secrets-patterns-db
2. Convert format (if needed)
3. Merge with built-in patterns
4. Handle conflicts

**Deliverables:**
- External database loading
- Pattern merging working
- 1,600+ patterns available

### Phase 3: User Customization (Week 3)

**Goals:**
- User custom patterns
- Category enable/disable
- Pattern overrides

**Tasks:**
1. Load `.coax/patterns.yaml`
2. Implement category filtering
3. Add pattern overrides
4. CLI commands for management

**Deliverables:**
- User patterns working
- Category filtering
- CLI commands

### Phase 4: Auto-Updates (Week 4)

**Goals:**
- Automatic pattern updates
- Version tracking
- Update notifications

**Tasks:**
1. Implement `PatternUpdater`
2. Add version tracking
3. CLI update commands
4. Auto-update scheduler

**Deliverables:**
- Auto-update working
- Version tracking
- Update notifications

---

## Testing Strategy

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_load_yaml_patterns() {
        let yaml = r#"
metadata:
  name: "Test Patterns"
  version: "1.0.0"
patterns:
  - id: "test-pattern"
    name: "Test Pattern"
    regex: "test\\d+"
    severity: "low"
"#;
        let db: PatternDatabase = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(db.patterns.len(), 1);
    }
    
    #[test]
    fn test_pattern_deduplication() {
        let manager = PatternManager::new();
        // Test deduplication logic
    }
    
    #[test]
    fn test_regex_validation() {
        let manager = PatternManager::new();
        // Test invalid regex handling
    }
}
```

### Integration Tests

```rust
#[cfg(test)]
mod integration_tests {
    #[test]
    fn test_load_secrets_patterns_db() {
        // Download and load secrets-patterns-db
        // Verify pattern count
    }
    
    #[test]
    fn test_pattern_update() {
        // Test update mechanism
    }
}
```

### Performance Tests

```rust
#[bench]
fn bench_load_1000_patterns(b: &mut test::Bencher) {
    b.iter(|| {
        let mut manager = PatternManager::new();
        manager.load_all().unwrap();
    })
}
```

---

## Security Considerations

### Pattern Validation

1. **Regex Validation**: All patterns validated before use
2. **ReDoS Protection**: Patterns checked for ReDoS vulnerabilities
3. **Source Verification**: Downloaded patterns from trusted sources only
4. **Sandboxing**: Patterns executed in sandboxed regex engine

### Supply Chain Security

1. **Source Pinning**: Pin pattern sources to specific commits/versions
2. **Signature Verification**: Verify pattern database signatures (future)
3. **Audit Trail**: Log pattern updates and changes
4. **Rollback Support**: Ability to rollback to previous pattern versions

---

## Performance Considerations

### Optimization Strategies

1. **Pattern Compilation Cache**: Compile regexes once, reuse across scans
2. **Lazy Loading**: Load patterns on-demand, not all at once
3. **Category Filtering**: Only load enabled categories
4. **Parallel Loading**: Load multiple databases in parallel

### Expected Performance

| Operation | Current | With Modular System | Notes |
|-----------|---------|---------------------|-------|
| Pattern load time | N/A (compiled) | 50-100ms | One-time cost |
| Pattern count | 43 | 1,600+ | 37x increase |
| Scan performance | Baseline | -5-10% | More patterns to check |
| Memory usage | ~100KB | ~5MB | Pattern storage |

---

## Success Metrics

### Adoption Metrics

- [ ] 1,600+ patterns available
- [ ] <100ms pattern load time
- [ ] <10% scan performance impact
- [ ] Zero ReDoS vulnerabilities

### User Experience Metrics

- [ ] Users can add custom patterns without code changes
- [ ] Pattern updates available via CLI command
- [ ] Category-based filtering works
- [ ] Pattern versioning tracked

### Quality Metrics

- [ ] <5% false positive rate with new patterns
- [ ] >90% detection rate on test dataset
- [ ] All patterns validated for ReDoS
- [ ] Pattern conflicts handled gracefully

---

## Conclusion

The modular pattern system will transform Coax from a static scanner with 43 patterns to a dynamic, community-driven platform with 1,600+ patterns. Key benefits:

1. **Massive coverage increase**: 43 → 1,600+ patterns
2. **User customization**: Add custom patterns without code changes
3. **Automatic updates**: Stay current with new secret types
4. **Community-driven**: Benefit from community contributions
5. **Flexible configuration**: Enable/disable categories per-project

**Implementation Timeline:** 4 weeks  
**Priority:** P0 for Phase 2  
**Dependencies:** serde_yaml, reqwest (for updates)

---

## Appendix: Sample Pattern Files

### Built-in Patterns (Rust → YAML)

```yaml
# Built-in patterns converted to YAML
metadata:
  name: "Coax Built-in Patterns"
  version: "2026.03.15"
  
patterns:
  - id: "aws-access-key"
    name: "AWS_ACCESS_KEY"
    description: "AWS Access Key ID"
    category: "cloud-providers"
    regex: "AKIA[0-9A-Z]{16}"
    severity: "critical"
    recommendation: "Remove immediately and rotate the key via AWS IAM Console"
    cwe_id: "CWE-798"
    enabled: true
    
  - id: "github-pat"
    name: "GITHUB_PAT"
    description: "GitHub Personal Access Token"
    category: "version-control"
    regex: "ghp_[a-zA-Z0-9]{36}"
    severity: "critical"
    recommendation: "Remove and regenerate the token in GitHub Settings"
    cwe_id: "CWE-798"
    enabled: true
```

### User Custom Patterns

```yaml
# ~/.coax/patterns.yaml
metadata:
  name: "My Custom Patterns"
  version: "1.0.0"
  
patterns:
  - id: "my-company-api-key"
    name: "My Company API Key"
    description: "Internal API key format"
    category: "custom"
    regex: "MYCO_[a-zA-Z0-9]{32}"
    severity: "critical"
    recommendation: "Contact security team to rotate"
    enabled: true
    
  - id: "my-company-db-password"
    name: "My Company DB Password"
    description: "Database password pattern"
    category: "custom"
    regex: "(?i)mycompany.*password.*[:=].{8,}"
    severity: "high"
    recommendation: "Use environment variables or secret manager"
    enabled: true
```
