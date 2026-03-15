# Threat Modeling Integration Research

**Date:** 2026-03-15
**Author:** Coax Research Team
**Status:** Complete

---

## Executive Summary

This document researches integrating the `opendev-threat-model` crate into Coax for Phase 3 P1. The integration enables automated STRIDE threat modeling, correlation of scan findings with threat categories, and enhanced risk scoring.

**Key Finding:** Threat model integration provides context-aware security analysis, transforming Coax from a pattern scanner to a comprehensive security assessment tool.

---

## Background: Threat Modeling

### What is Threat Modeling?

**Definition:** A structured approach to identifying, quantifying, and addressing security threats in a system.

**Benefits:**
- Proactive security (find threats before exploitation)
- Risk prioritization (focus on high-impact threats)
- Compliance support (meet regulatory requirements)
- Developer awareness (security mindset)

---

### STRIDE Threat Model

**Origin:** Microsoft (2000s)
**Categories:** 6 threat types

| Category | Description | Example | Mitigation |
|----------|-------------|---------|------------|
| **S**poofing | Impersonating legitimate entities | Fake login page, credential theft | Authentication, MFA |
| **T**ampering | Modifying data/code maliciously | SQL injection, config tampering | Integrity checks, signatures |
| **R**epudiation | Denying actions without proof | Log deletion, anonymous actions | Audit logs, non-repudiation |
| **I**nformation Disclosure | Exposing sensitive information | Secret leakage, data breach | Encryption, access control |
| **D**enial of Service | Disrupting service availability | DDoS, resource exhaustion | Rate limiting, redundancy |
| **E**levation of Privilege | Gaining unauthorized access | Privilege escalation, auth bypass | Least privilege, RBAC |

---

### Data Flow Diagrams (DFD)

**Purpose:** Visual representation of system components and data flows for threat analysis.

**DFD Components:**
```
┌─────────────┐     Data Flow      ┌─────────────┐
│   External  │ ──────────────────▶│   Process   │
│   Entity    │    (data movement) │   (action)  │
└─────────────┘                    └──────┬──────┘
                                          │
                                          ▼
                                   ┌─────────────┐
                                   │ Data Store  │
                                   │ (database)  │
                                   └─────────────┘
```

**Threat Analysis per Component:**
- **External Entity:** Spoofing, Repudiation
- **Process:** Tampering, Information Disclosure, DoS
- **Data Store:** Tampering, Information Disclosure, Elevation of Privilege
- **Data Flow:** Spoofing, Tampering, Information Disclosure

---

## opendev-threat-model Crate Analysis

### Note: Crate Status

Based on research, `opendev-threat-model` was mentioned in the Phase 2 planning documents but was not found in the current workspace. This document assumes the crate will be developed or integrated as part of Phase 3 P1.

**Assumed Location:** `crates/opendev-threat-model/`
**Assumed License:** MIT or Apache-2.0 (compatible with Coax)

---

### Assumed API Design

Based on threat modeling best practices and similar crates:

```rust
use opendev_threat_model::{ThreatModel, Threat, StrideCategory, RiskLevel};

// Create threat model
let mut model = ThreatModel::new("My Application");

// Add components
model.add_component(Component {
    name: "Web Server".to_string(),
    component_type: ComponentType::Process,
    description: "Handles HTTP requests".to_string(),
});

// Add data flows
model.add_data_flow(DataFlow {
    from: "Client".to_string(),
    to: "Web Server".to_string(),
    description: "HTTP requests".to_string(),
});

// Analyze threats
let threats = model.analyze_stride();

// Get threats by category
let spoofing_threats = threats.by_category(StrideCategory::Spoofing);

// Calculate risk scores
let risk_score = model.calculate_risk_score();
```

---

### Assumed Data Structures

```rust
/// STRIDE threat categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StrideCategory {
    Spoofing,
    Tampering,
    Repudiation,
    InformationDisclosure,
    DenialOfService,
    ElevationOfPrivilege,
}

/// Risk level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RiskLevel {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Threat representation
#[derive(Debug, Clone)]
pub struct Threat {
    pub id: String,
    pub title: String,
    pub description: String,
    pub category: StrideCategory,
    pub risk_level: RiskLevel,
    pub affected_component: String,
    pub mitigation: String,
    pub cwe_id: Option<String>,
}

/// System component
#[derive(Debug, Clone)]
pub struct Component {
    pub name: String,
    pub component_type: ComponentType,
    pub description: String,
    pub trust_boundary: String,
}

/// Component types for DFD
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentType {
    ExternalEntity,
    Process,
    DataStore,
    DataFlow,
}

/// Threat model
#[derive(Debug, Clone)]
pub struct ThreatModel {
    pub name: String,
    pub version: String,
    pub components: Vec<Component>,
    pub data_flows: Vec<DataFlow>,
    pub threats: Vec<Threat>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

---

## Integration Points

### 1. Finding-to-Threat Correlation

**Goal:** Map Coax scan findings to STRIDE categories.

**Mapping Table:**

| Finding Type | STRIDE Category | Risk Level | Example |
|--------------|-----------------|------------|---------|
| AWS_ACCESS_KEY | Information Disclosure | Critical | Leaked cloud credentials |
| GITHUB_PAT | Information Disclosure + Spoofing | Critical | Token enables repo access + impersonation |
| DATABASE_URL | Information Disclosure + Tampering | High | DB access + potential data modification |
| PRIVATE_KEY | Spoofing + Information Disclosure | Critical | Enables impersonation |
| API_KEY | Information Disclosure + Elevation | High | Unauthorized API access |
| JWT_SECRET | Spoofing + Tampering | Critical | Token forgery |
| GENERIC_SECRET | Information Disclosure | Medium | Potential credential |
| HIGH_ENTROPY | Information Disclosure | Medium | Possible secret |

**Implementation:**
```rust
use coax_scanner::ScanResult;
use opendev_threat_model::{Threat, StrideCategory, RiskLevel};

pub fn correlate_finding_to_threat(finding: &ScanResult) -> Threat {
    let (category, risk_level) = match finding.pattern.as_str() {
        "AWS_ACCESS_KEY" => (StrideCategory::InformationDisclosure, RiskLevel::Critical),
        "GITHUB_PAT" => (StrideCategory::Spoofing, RiskLevel::Critical),
        "DATABASE_URL" => (StrideCategory::Tampering, RiskLevel::High),
        "PRIVATE_KEY" => (StrideCategory::Spoofing, RiskLevel::Critical),
        "API_KEY" => (StrideCategory::ElevationOfPrivilege, RiskLevel::High),
        "JWT_SECRET" => (StrideCategory::Spoofing, RiskLevel::Critical),
        "GENERIC_SECRET" => (StrideCategory::InformationDisclosure, RiskLevel::Medium),
        "HIGH_ENTROPY" => (StrideCategory::InformationDisclosure, RiskLevel::Medium),
        _ => (StrideCategory::InformationDisclosure, RiskLevel::Low),
    };
    
    Threat {
        id: format!("COAX-{}", finding.pattern),
        title: format!("{} detected in {}", finding.pattern, finding.file),
        description: format!(
            "Potential {} found at {}:{} - {}",
            finding.pattern,
            finding.file,
            finding.line,
            finding.recommendation
        ),
        category,
        risk_level,
        affected_component: finding.file.clone(),
        mitigation: finding.recommendation.clone(),
        cwe_id: Some(get_cwe_for_pattern(&finding.pattern)),
    }
}

fn get_cwe_for_pattern(pattern: &str) -> String {
    match pattern {
        "AWS_ACCESS_KEY" | "GITHUB_PAT" | "PRIVATE_KEY" => "CWE-798".to_string(),
        "DATABASE_URL" => "CWE-89".to_string(),  // SQL Injection risk
        "API_KEY" => "CWE-522".to_string(),  // Insufficiently protected credentials
        "JWT_SECRET" => "CWE-347".to_string(),  // Improper verification
        _ => "CWE-798".to_string(),
    }
}
```

---

### 2. Enhanced Threat Model from Scan Results

**Goal:** Generate threat model from codebase scan results.

**Implementation:**
```rust
use coax_scanner::{Scanner, ScanResult};
use opendev_threat_model::{ThreatModel, Component, ComponentType, Threat};

pub fn generate_threat_model_from_scan(
    scan_results: &[ScanResult],
    repo_path: &str,
) -> ThreatModel {
    let mut model = ThreatModel::new(repo_path);
    
    // Infer components from file structure
    let components = infer_components(scan_results);
    for component in components {
        model.add_component(component);
    }
    
    // Correlate findings to threats
    for finding in scan_results {
        let threat = correlate_finding_to_threat(finding);
        model.add_threat(threat);
    }
    
    // Calculate aggregate risk
    model.calculate_risk_score();
    
    model
}

fn infer_components(scan_results: &[ScanResult]) -> Vec<Component> {
    let mut components = Vec::new();
    let mut seen_files = std::collections::HashSet::new();
    
    for result in scan_results {
        if !seen_files.contains(&result.file) {
            seen_files.insert(result.file.clone());
            
            let component_type = infer_component_type(&result.file);
            components.push(Component {
                name: result.file.clone(),
                component_type,
                description: format!("File containing {}", result.pattern),
                trust_boundary: "internal".to_string(),
            });
        }
    }
    
    components
}

fn infer_component_type(file_path: &str) -> ComponentType {
    if file_path.ends_with(".yml") || file_path.ends_with(".yaml") {
        ComponentType::DataStore  // Config file
    } else if file_path.ends_with(".env") {
        ComponentType::DataStore  // Environment file
    } else if file_path.contains("src/") || file_path.contains("lib/") {
        ComponentType::Process  // Source code
    } else {
        ComponentType::Process
    }
}
```

---

### 3. Risk Scoring

**Goal:** Calculate aggregate risk score from findings.

**Risk Score Formula:**
```
Risk Score = Σ(Finding Weight × Severity Multiplier × Context Factor)

Where:
- Finding Weight: Based on STRIDE category
- Severity Multiplier: Critical=4, High=3, Medium=2, Low=1
- Context Factor: Production=2.0, Staging=1.5, Development=1.0
```

**Implementation:**
```rust
use opendev_threat_model::{RiskLevel, StrideCategory};

pub struct RiskScore {
    pub overall: f32,  // 0.0 - 100.0
    pub by_category: HashMap<StrideCategory, f32>,
    pub by_severity: HashMap<RiskLevel, usize>,
    pub recommendation: RiskRecommendation,
}

pub enum RiskRecommendation {
    Acceptable,      // Score < 20
    NeedsAttention,  // Score 20-50
    HighRisk,        // Score 50-80
    Critical,        // Score > 80
}

pub fn calculate_risk_score(
    findings: &[ScanResult],
    context: &ScanContext,
) -> RiskScore {
    let mut total_score = 0.0;
    let mut by_category: HashMap<StrideCategory, f32> = HashMap::new();
    let mut by_severity: HashMap<RiskLevel, usize> = HashMap::new();
    
    for finding in findings {
        let threat = correlate_finding_to_threat(finding);
        
        // Category weight
        let category_weight = match threat.category {
            StrideCategory::Spoofing => 1.5,
            StrideCategory::Tampering => 1.4,
            StrideCategory::Repudiation => 1.0,
            StrideCategory::InformationDisclosure => 1.3,
            StrideCategory::DenialOfService => 1.1,
            StrideCategory::ElevationOfPrivilege => 1.5,
        };
        
        // Severity multiplier
        let severity_multiplier = match threat.risk_level {
            RiskLevel::Critical => 4.0,
            RiskLevel::High => 3.0,
            RiskLevel::Medium => 2.0,
            RiskLevel::Low => 1.0,
            RiskLevel::Info => 0.5,
        };
        
        // Context factor
        let context_factor = match context.environment {
            Environment::Production => 2.0,
            Environment::Staging => 1.5,
            Environment::Development => 1.0,
        };
        
        let finding_score = category_weight * severity_multiplier * context_factor;
        total_score += finding_score;
        
        *by_category.entry(threat.category).or_insert(0.0) += finding_score;
        *by_severity.entry(threat.risk_level).or_insert(0) += 1;
    }
    
    // Normalize to 0-100 scale
    let normalized_score = (total_score / 10.0).min(100.0);
    
    let recommendation = match normalized_score {
        s if s < 20.0 => RiskRecommendation::Acceptable,
        s if s < 50.0 => RiskRecommendation::NeedsAttention,
        s if s < 80.0 => RiskRecommendation::HighRisk,
        _ => RiskRecommendation::Critical,
    };
    
    RiskScore {
        overall: normalized_score,
        by_category,
        by_severity,
        recommendation,
    }
}
```

---

## Command Design

### `coax threat-model` Command

**Usage:**
```bash
# Generate threat model from scan
coax threat-model generate --path . --output threat-model.yml

# Generate with correlation
coax threat-model correlate --path . --baseline .coax-baseline.json

# Show threat summary
coax threat-model summary --path .

# Export to different formats
coax threat-model generate --path . --format yaml
coax threat-model generate --path . --format json
coax threat-model generate --path . --format dfd  # ASCII DFD
```

---

### Command Implementation

```rust
use clap::{Parser, Subcommand};
use coax_scanner::Scanner;
use opendev_threat_model::{ThreatModel, OutputFormat};

#[derive(Parser)]
#[command(name = "coax")]
#[command(about = "Security scanner with threat modeling")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Scan for secrets
    Scan {
        #[arg(short, long)]
        path: Option<PathBuf>,
        
        #[arg(short, long)]
        format: Option<String>,
    },
    
    /// Threat modeling
    ThreatModel {
        #[command(subcommand)]
        action: ThreatModelAction,
    },
}

#[derive(Subcommand)]
enum ThreatModelAction {
    /// Generate threat model from codebase
    Generate {
        #[arg(short, long)]
        path: Option<PathBuf>,
        
        #[arg(short, long)]
        output: Option<PathBuf>,
        
        #[arg(short, long, default_value = "yaml")]
        format: String,
    },
    
    /// Correlate findings with threat model
    Correlate {
        #[arg(short, long)]
        path: Option<PathBuf>,
        
        #[arg(short, long)]
        baseline: Option<PathBuf>,
    },
    
    /// Show threat summary
    Summary {
        #[arg(short, long)]
        path: Option<PathBuf>,
    },
}

async fn run() -> anyhow::Result<()> {
    let args = Cli::parse();
    
    match args.command {
        Commands::ThreatModel { action } => {
            match action {
                ThreatModelAction::Generate { path, output, format } => {
                    let path = path.unwrap_or_else(|| std::env::current_dir()?);
                    
                    // Scan codebase
                    let scanner = Scanner::with_default_patterns();
                    let findings = scanner.scan_directory(&path);
                    
                    // Generate threat model
                    let mut model = ThreatModel::new(path.to_str().unwrap());
                    for finding in &findings {
                        let threat = correlate_finding_to_threat(finding);
                        model.add_threat(threat);
                    }
                    
                    // Output
                    let output_format = OutputFormat::from_str(&format)?;
                    let output_str = model.serialize(&output_format)?;
                    
                    if let Some(output_path) = output {
                        std::fs::write(&output_path, &output_str)?;
                        println!("Threat model written to {}", output_path.display());
                    } else {
                        println!("{}", output_str);
                    }
                }
                
                ThreatModelAction::Correlate { path, baseline } => {
                    // Scan and correlate
                    let scanner = Scanner::with_default_patterns();
                    let findings = scanner.scan_directory(&path.unwrap_or_else(|| std::env::current_dir()?));
                    
                    println!("🛡️  Threat Correlation Report\n");
                    println!("Total findings: {}", findings.len());
                    
                    let mut model = ThreatModel::new("correlated");
                    for finding in &findings {
                        let threat = correlate_finding_to_threat(finding);
                        model.add_threat(threat);
                    }
                    
                    // Print summary
                    print_threat_summary(&model);
                }
                
                ThreatModelAction::Summary { path } => {
                    let scanner = Scanner::with_default_patterns();
                    let findings = scanner.scan_directory(&path.unwrap_or_else(|| std::env::current_dir()?));
                    
                    let model = generate_threat_model_from_scan(&findings, "summary");
                    print_threat_summary(&model);
                }
            }
        }
        
        Commands::Scan { path, format } => {
            // Existing scan command
        }
    }
    
    Ok(())
}

fn print_threat_summary(model: &ThreatModel) {
    println!("\n📊 Threat Summary for: {}\n", model.name);
    
    // Count by category
    let mut by_category: HashMap<StrideCategory, usize> = HashMap::new();
    for threat in &model.threats {
        *by_category.entry(threat.category).or_insert(0) += 1;
    }
    
    println!("┌────────────────────────────────────────┐");
    println!("│  Threats by STRIDE Category            │");
    println!("├────────────────────────────────────────┤");
    for (category, count) in &by_category {
        let icon = match category {
            StrideCategory::Spoofing => "🎭",
            StrideCategory::Tampering => "🔧",
            StrideCategory::Repudiation => "❌",
            StrideCategory::InformationDisclosure => "👁️",
            StrideCategory::DenialOfService => "🚫",
            StrideCategory::ElevationOfPrivilege => "⬆️",
        };
        println!("│  {} {:<25} {:>5} │", icon, format!("{:?}", category), count);
    }
    println!("├────────────────────────────────────────┤");
    println!("│  {:<25} {:>5} │", "TOTAL", model.threats.len());
    println!("└────────────────────────────────────────┘");
    
    // Risk score
    let risk_score = calculate_risk_score_from_model(model);
    println!("\n📈 Risk Score: {:.1}/100", risk_score.overall);
    
    match risk_score.recommendation {
        RiskRecommendation::Acceptable => println!("   Status: ✅ Acceptable"),
        RiskRecommendation::NeedsAttention => println!("   Status: ⚠️  Needs Attention"),
        RiskRecommendation::HighRisk => println!("   Status: 🚨 High Risk"),
        RiskRecommendation::Critical => println!("   Status: 🚨🚨 CRITICAL"),
    }
}
```

---

## Output Format

### YAML Output

```yaml
# threat-model.yml
version: "1.0"
name: "My Application"
generated_at: "2026-03-15T16:00:00Z"
scanner_version: "0.3.0"

components:
  - name: "config.yml"
    type: "DataStore"
    description: "Configuration file containing AWS credentials"
    trust_boundary: "internal"
    
  - name: ".env"
    type: "DataStore"
    description: "Environment file containing GitHub PAT"
    trust_boundary: "internal"
    
  - name: "src/auth.rs"
    type: "Process"
    description: "Authentication module"
    trust_boundary: "internal"

data_flows:
  - from: "Client"
    to: "src/auth.rs"
    description: "Authentication requests"
    
  - from: "src/auth.rs"
    to: "config.yml"
    description: "Configuration access"

threats:
  - id: "COAX-AWS_ACCESS_KEY"
    title: "AWS Access Key detected in config.yml"
    description: "Potential AWS Access Key found at config.yml:45"
    category: "InformationDisclosure"
    risk_level: "Critical"
    affected_component: "config.yml"
    mitigation: "Remove immediately and rotate the key via AWS IAM Console"
    cwe_id: "CWE-798"
    
  - id: "COAX-GITHUB_PAT"
    title: "GitHub PAT detected in .env"
    description: "Potential GitHub Personal Access Token found at .env:12"
    category: "Spoofing"
    risk_level: "Critical"
    affected_component: ".env"
    mitigation: "Remove and revoke token via GitHub Settings"
    cwe_id: "CWE-798"

risk_score:
  overall: 72.5
  by_category:
    Spoofing: 25.0
    InformationDisclosure: 35.0
    Tampering: 12.5
  by_severity:
    Critical: 2
    High: 3
    Medium: 5
    Low: 1
  recommendation: "HighRisk"

recommendations:
  - priority: 1
    action: "Rotate all exposed AWS credentials immediately"
    threats: ["COAX-AWS_ACCESS_KEY"]
    
  - priority: 2
    action: "Revoke exposed GitHub tokens and audit access logs"
    threats: ["COAX-GITHUB_PAT"]
    
  - priority: 3
    action: "Implement secret management solution (e.g., AWS Secrets Manager)"
    threats: ["COAX-AWS_ACCESS_KEY", "COAX-GITHUB_PAT"]
```

---

### JSON Output

```json
{
  "version": "1.0",
  "name": "My Application",
  "generated_at": "2026-03-15T16:00:00Z",
  "scanner_version": "0.3.0",
  "components": [...],
  "data_flows": [...],
  "threats": [...],
  "risk_score": {
    "overall": 72.5,
    "by_category": {...},
    "by_severity": {...},
    "recommendation": "HighRisk"
  },
  "recommendations": [...]
}
```

---

### ASCII DFD Output

```
╔══════════════════════════════════════════════════════════════╗
║  Data Flow Diagram - Threat Model                            ║
║  Application: My Application                                 ║
╠══════════════════════════════════════════════════════════════╣
║                                                              ║
║     ┌──────────────┐                                         ║
║     │   Client     │                                         ║
║     │  (External)  │                                         ║
║     └──────┬───────┘                                         ║
║            │                                                  ║
║            │ HTTP Requests                                    ║
║            │ [🎭 Spoofing, 🔧 Tampering]                     ║
║            ▼                                                  ║
║     ┌──────────────┐                                         ║
║     │  src/auth.rs │                                         ║
║     │  (Process)   │───────┐                                 ║
║     └──────┬───────┘       │                                 ║
║            │               │ Config Access                   ║
║            │               │ [👁️ Info Disclosure]           ║
║            │               ▼                                 ║
║            │        ┌──────────────┐                         ║
║            │        │  config.yml  │                         ║
║            │        │  (DataStore) │                         ║
║            │        │ 🚨 AWS_KEY   │                         ║
║            │        └──────────────┘                         ║
║            │                                                 ║
║            │        ┌──────────────┐                         ║
║            └───────▶│    .env      │                         ║
║                     │  (DataStore) │                         ║
║                     │ 🚨 GITHUB_PAT│                         ║
║                     └──────────────┘                         ║
║                                                              ║
║  Legend:                                                     ║
║  🎭 Spoofing  🔧 Tampering  👁️ Info Disclosure  🚨 Finding  ║
║                                                              ║
╚══════════════════════════════════════════════════════════════╝
```

---

## Estimated Effort

| Task | Description | Effort |
|------|-------------|--------|
| **Create opendev-threat-model crate** | Core data structures, STRIDE analysis | 3 days |
| **Finding-to-threat correlation** | Map scan findings to STRIDE | 1 day |
| **Risk scoring implementation** | Calculate aggregate risk | 1 day |
| **`coax threat-model` command** | CLI implementation | 2 days |
| **YAML/JSON output** | Serialization | 1 day |
| **ASCII DFD generation** | Visual diagram | 2 days |
| **Integration testing** | End-to-end tests | 2 days |
| **Documentation** | User guide, examples | 1 day |
| **Total** | | 13 days (~3 weeks) |

---

## Integration with Other Phase 3 P1 Features

### With CFG-Based Slicing

```
Scan Results → CFG Slicing → Vulnerability Paths → Threat Model
                                      ↓
                              LLM Analysis
                                      ↓
                          Enhanced Threat Description
```

**Benefit:** CFG slicing provides precise vulnerability context for threat model.

---

### With LLM Integration

```
Threat Model → LLM Prompt → Enhanced Mitigation Recommendations
```

**Prompt Example:**
```
Analyze this threat model and provide detailed mitigation recommendations:

Threat Model:
{threat_model_yaml}

For each threat, provide:
1. Specific code changes needed
2. Configuration updates required
3. Monitoring recommendations
4. Testing approach to verify mitigation
```

---

### With TUI Dashboard

```
TUI Dashboard
    ├── Threat Summary Panel
    ├── STRIDE Category View
    ├── Risk Score Gauge
    └── DFD Visualization
```

**Benefit:** Interactive threat model exploration.

---

## Success Criteria

### End of Implementation

- [ ] `opendev-threat-model` crate created
- [ ] `coax threat-model generate` command works
- [ ] `coax threat-model correlate` command works
- [ ] `coax threat-model summary` command works
- [ ] YAML output generated correctly
- [ ] JSON output generated correctly
- [ ] ASCII DFD rendered correctly
- [ ] Risk scores calculated accurately
- [ ] Findings correlated to STRIDE categories
- [ ] Unit tests pass (15+ tests)

---

## Conclusion

**Recommendation:** Proceed with threat model integration as Phase 3 P1 feature.

**Benefits:**
- Transforms Coax from scanner to security assessment tool
- Provides actionable context for findings
- Enables risk-based prioritization
- Supports compliance requirements

**Implementation Priority:** P1 (after CFG foundation, before LLM integration)

**Timeline:** 3 weeks

---

## References

- **Microsoft STRIDE:** https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool
- **OWASP Threat Modeling:** https://owasp.org/www-community/Threat_Modeling
- **STRIDE per Element:** https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-stride
- **CWE Database:** https://cwe.mitre.org/

---

*Research completed: 2026-03-15*
