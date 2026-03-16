# Phase 3 Feature Proposal

**Date:** 2026-03-15
**Status:** Proposal
**Timeline:** 12-16 weeks (extended for VS Code Extension)

---

## Executive Summary

Based on comprehensive analysis of 5 state-of-the-art security scanners (TruffleHog, Gitleaks, GitGuardian, Semgrep, detect-secrets), we propose **8 features** for Phase 3 development.

**Phase 3 Goal:** Transform Coax from basic regex scanner to production-ready secret detection tool with enterprise capabilities AND native IDE integration.

**Top Priority:** 
1. VS Code Extension v0.8.0 (critical for developer adoption) - 4-5 weeks
2. Live secret verification (eliminates false positives, prioritizes real risks) - 2-3 weeks

**Revised Timeline:** 12-16 weeks total (extended from 8-12 weeks to accommodate VS Code Extension development)

---

## Proposed Features

### Feature 0: VS Code Extension (NEW - P0 Priority)

**Status:** P0 (Critical for Adoption)
**Effort:** 4-5 weeks
**Dependencies:** Coax CLI binary builds for all platforms
**Version:** v0.8.0

#### Description

Native VS Code extension providing real-time Unicode confusable and secret detection directly in the editor. Integrates Coax CLI with VS Code's diagnostic system for immediate developer feedback.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Developer adoption** | Meets developers where they work (VS Code) |
| **Prevention** | Stop secrets before commit, not after |
| **Real-time feedback** | Immediate awareness of issues |
| **Competitive parity** | Matches GitGuardian, Snyk IDE integration |
| **Low friction** | Automatic scanning, easy fixes |

**Expected Impact:**
- Developer adoption: High (native IDE experience)
- Secret prevention: Very high (before git history)
- Time to fix: Minutes vs. hours/days

#### Key Features

**Real-time Scanning:**
- Scan on file save
- Scan on file open
- Manual scan commands

**Inline Warnings:**
- Squiggly underlines with severity colors
- Red = Critical, Orange = High, Yellow = Medium, Green = Low
- Hover tooltips with details

**Problems Panel:**
- Full integration with VS Code Problems panel
- Filter by severity
- Click to navigate

**Quick-Fix Actions:**
- Remove secret
- Replace with environment variable
- Ignore for session
- Add to allowlist

**Status Bar:**
- Scan status indicator
- Finding count
- Quick settings access

**Command Palette:**
- `Coax: Scan Current File`
- `Coax: Scan Workspace`
- `Coax: Show Findings`
- `Coax: Settings`

#### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    VS Code Extension                     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  │
│  │ File Watcher │  │ Diagnostics  │  │ Code Actions │  │
│  └──────────────┘  └──────────────┘  └──────────────┘  │
│              ┌──────────────────────┐                   │
│              │   Coax CLI Binary    │                   │
│              │   (bundled)          │                   │
│              └──────────────────────┘                   │
└─────────────────────────────────────────────────────────┘
```

#### Implementation Approach

**Extension Structure:**
```
coax-vscode/
├── src/
│   ├── extension.ts          # Main entry
│   ├── scanner/              # CLI execution
│   ├── diagnostics/          # Squiggly underlines
│   ├── actions/              # Quick fixes
│   ├── statusbar/            # Status indicator
│   └── commands/             # Command handlers
├── bundled/                   # Platform binaries
│   ├── darwin-arm64/coax
│   ├── darwin-x64/coax
│   ├── linux-x64/coax
│   ├── linux-arm64/coax
│   └── win32-x64/coax.exe
└── package.json
```

**Binary Bundling:**
- Bundle Coax CLI for all 5 platforms
- Auto-detect platform at runtime
- Fallback to system `coax` command

**Key VS Code APIs:**
- `vscode.languages.createDiagnosticCollection()` - Squiggly underlines
- `vscode.workspace.createFileSystemWatcher()` - File monitoring
- `vscode.languages.registerCodeActionsProvider()` - Quick fixes
- `vscode.window.createStatusBarItem()` - Status bar
- `vscode.commands.registerCommand()` - Commands

#### Development Phases

| Phase | Duration | Features |
|-------|----------|----------|
| Phase 1: MVP | 1 week | Project setup, binary bundling, file watcher, basic scanning |
| Phase 2: Diagnostics | 1 week | Squiggly underlines, Problems panel, severity colors |
| Phase 3: Quick-Fixes | 1 week | Code actions, hover provider, status bar |
| Phase 4: Polish | 1 week | Commands, settings, testing, Marketplace release |

#### Estimated Effort

| Task | Time |
|------|------|
| Project setup | 2-3 days |
| Binary bundling | 2-3 days |
| File watcher + scanning | 2-3 days |
| Diagnostic collection | 2-3 days |
| Code actions | 2-3 days |
| Hover + status bar | 1-2 days |
| Commands + settings | 2-3 days |
| Testing + documentation | 3-5 days |
| Marketplace submission | 1-2 days |
| **Total** | **4-5 weeks** |

#### Success Criteria

- [ ] Install from VS Code Marketplace
- [ ] Scan on file save works
- [ ] Findings show in Problems panel
- [ ] Inline warnings visible
- [ ] Quick-fix actions work
- [ ] Zero crashes in 100+ test sessions
- [ ] 4.0+ stars on Marketplace

#### Resources

- `docs/VSCode-EXTENSION-SPEC.md` - Complete specification
- `docs/VSCode-EXTENSION-TIMELINE.md` - Development timeline
- `docs/research/vscode-extension-research.md` - Technical research

#### Competitive Analysis

| Feature | GitGuardian | Snyk | Coax (Planned) |
|---------|-------------|------|----------------|
| Real-time scanning | ✅ | ✅ | ✅ |
| Inline diagnostics | ✅ | ✅ | ✅ |
| Problems panel | ✅ | ✅ | ✅ |
| Quick fixes | ⚠️ Limited | ✅ | ✅ |
| Status bar | ✅ | ✅ | ✅ |
| Binary bundling | ✅ | ✅ | ✅ |
| Baseline files | ❌ | ❌ | ✅ (future) |
| Unicode detection | ❌ | ❌ | ✅ (unique) |

---

### Feature 1: Live Secret Verification

**Status:** P0 (Critical)
**Effort:** 2-3 weeks
**Dependencies:** Pattern detection complete, VS Code Extension v0.8.0 (for IDE integration)

#### Description

Verify if detected secrets are actually active/valid by making safe, read-only API calls to the respective service.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Eliminates false positives** | Revoked/expired secrets filtered out |
| **Prioritizes real risks** | Verified secrets = immediate action required |
| **Reduces noise** | Developers trust scanner output |
| **Competitive parity** | Matches TruffleHog/GitGuardian capability |

**Expected Impact:**
- False positive rate: <1% (for verified secrets)
- Developer trust: High
- Time savings: Hours per week not investigating false alarms

#### Implementation Approach

**Architecture:**
```
┌─────────────────┐     ┌──────────────────┐     ┌─────────────────┐
│  Pattern Match  │────▶│  Secret Classifier│────▶│   Verifier      │
│  (regex/entropy)│     │  (AWS, GitHub,   │     │   (API calls)   │
└─────────────────┘     │   Stripe, etc.)  │     └─────────────────┘
                        └──────────────────┘              │
                                                          ▼
                                                ┌─────────────────┐
                                                │  Verification   │
                                                │  Status:        │
                                                │  - verified     │
                                                │  - unverified   │
                                                │  - unknown      │
                                                └─────────────────┘
```

**Verifier Modules:**

| Secret Type | Verification Method | Safety |
|-------------|-------------------|--------|
| AWS Access Key | `sts:GetCallerIdentity` | ✅ Read-only |
| GitHub PAT | `GET /user` | ✅ Read-only |
| Stripe Key | `GET /v1/balance` | ✅ Read-only |
| Google API Key | OAuth token info endpoint | ✅ Read-only |
| Slack Token | `auth.test` API | ✅ Read-only |
| Twilio Key | `GET /Accounts` | ✅ Read-only |
| SendGrid Key | `GET /user/profile` | ✅ Read-only |
| Private Key | Passive (Driftwood-style GitHub/TLS matching) | ✅ Passive |

**Implementation Details:**

```rust
// src/verification/mod.rs

pub enum VerificationStatus {
    Verified,      // Confirmed valid
    Unverified,    // Not confirmed (may be invalid/expired)
    Unknown,       // Verification failed (network/API error)
}

pub trait SecretVerifier: Send + Sync {
    fn verify(&self, secret: &str) -> Result<VerificationStatus, VerificationError>;
    fn secret_type(&self) -> &'static str;
}

pub struct AwsVerifier {
    client: reqwest::Client,
    // AWS SDK client or custom STS implementation
}

impl SecretVerifier for AwsVerifier {
    fn verify(&self, secret: &str) -> Result<VerificationStatus, VerificationError> {
        // Call sts:GetCallerIdentity
        // Return Verified if successful, Unverified if InvalidClientTokenId
    }
    
    fn secret_type(&self) -> &'static str {
        "AWS_ACCESS_KEY"
    }
}

// Verification caching
pub struct VerificationCache {
    cache: DashMap<String, (VerificationStatus, Instant)>,
    ttl: Duration,
}

impl VerificationCache {
    pub fn get(&self, secret: &str) -> Option<VerificationStatus> {
        // Check cache, return if not expired
    }
    
    pub fn set(&self, secret: &str, status: VerificationStatus) {
        // Cache with TTL
    }
}
```

**CLI Integration:**
```bash
# Default: verification enabled
devshield scan secrets --path .

# Disable verification (faster, more FPs)
devshield scan secrets --path . --no-verify

# Only show verified secrets
devshield scan secrets --path . --verified-only

# Custom verification timeout
devshield scan secrets --path . --verification-timeout 30s

# Custom verifier endpoint (for custom regex)
devshield scan secrets --path . --verifier-url https://my-webhook/verify
```

**Rate Limiting Strategy:**
- Per-service rate limits (e.g., GitHub: 5000 req/hour authenticated)
- Exponential backoff on 429 responses
- Request queuing with priority (verified > unverified)
- Daily quota tracking

**Safety Guarantees:**
- All verification calls are read-only
- No write operations performed
- No secret rotation triggered
- Audit logging of all verification calls

#### Estimated Effort

| Task | Time |
|------|------|
| Verifier trait + architecture | 2 days |
| AWS verifier | 2 days |
| GitHub verifier | 1 day |
| Stripe/Google/Slack verifiers | 2 days |
| Verification caching | 2 days |
| Rate limiting + backoff | 2 days |
| CLI integration | 2 days |
| Testing + documentation | 3 days |
| **Total** | **2-3 weeks** |

---

### Feature 2: Baseline Files

**Status:** P0 (Critical)
**Effort:** 1-2 weeks
**Dependencies:** None

#### Description

Create and maintain baseline files that track known secrets, enabling incremental scanning and CI/CD integration.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Incremental scanning** | Only report new secrets, not historical |
| **CI/CD integration** | Block new secrets, allow existing |
| **False positive tracking** | Mark FPs once, ignore forever |
| **Competitive parity** | Matches Gitleaks/detect-secrets capability |

**Expected Impact:**
- CI/CD adoption: High
- Developer friction: Low (only new secrets block commits)
- False positive management: Streamlined

#### Implementation Approach

**Baseline File Format (JSON):**
```json
{
  "version": "1.0.0",
  "created_at": "2026-03-15T10:30:00Z",
  "updated_at": "2026-03-15T14:45:00Z",
  "plugins_used": [
    {"name": "AWSKeyDetector"},
    {"name": "GitHubTokenDetector"},
    {"name": "EntropyDetector", "threshold": 4.0}
  ],
  "results": {
    "config.yml": [
      {
        "type": "AWS_ACCESS_KEY",
        "hash": "sha256:abc123...",
        "line": 5,
        "column": 12,
        "secret_length": 20,
        "is_verified": false,
        "labels": ["false_positive"],
        "created_at": "2026-03-15T10:30:00Z"
      }
    ],
    "src/auth.rs": [
      {
        "type": "GITHUB_PAT",
        "hash": "sha256:def456...",
        "line": 42,
        "column": 20,
        "secret_length": 40,
        "is_verified": true,
        "labels": [],
        "created_at": "2026-03-15T10:30:00Z"
      }
    ]
  }
}
```

**Commands:**
```bash
# Create initial baseline
devshield baseline create --path . --output .devshield-baseline.json

# Scan against baseline (report only new secrets)
devshield scan secrets --path . --baseline .devshield-baseline.json

# Update baseline (add new, remove old, preserve labels)
devshield baseline update --path . --baseline .devshield-baseline.json

# Show baseline statistics
devshield baseline stats --baseline .devshield-baseline.json

# Migrate baseline (if format changes)
devshield baseline migrate --baseline .devshield-baseline.json --to-version 2.0.0
```

**Implementation Details:**

```rust
// src/baseline/mod.rs

#[derive(Serialize, Deserialize, Clone)]
pub struct Baseline {
    pub version: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub plugins_used: Vec<PluginConfig>,
    pub results: HashMap<String, Vec<Finding>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Finding {
    pub r#type: String,
    pub hash: String,  // SHA-256 of file:line:secret
    pub line: u32,
    pub column: u32,
    pub secret_length: usize,
    pub is_verified: bool,
    pub labels: Vec<String>,  // "false_positive", "revoked", etc.
    pub created_at: DateTime<Utc>,
}

impl Baseline {
    pub fn load(path: &Path) -> Result<Self, BaselineError>;
    pub fn save(&self, path: &Path) -> Result<(), BaselineError>;
    
    pub fn diff(&self, new_findings: &[Finding]) -> BaselineDiff {
        // Compare and return only new findings
    }
    
    pub fn update(&mut self, new_findings: &[Finding]) {
        // Add new, remove old, preserve labels
    }
    
    pub fn is_false_positive(&self, finding: &Finding) -> bool {
        finding.labels.contains(&"false_positive".to_string())
    }
}

pub struct BaselineDiff {
    pub new_findings: Vec<Finding>,
    pub removed_findings: Vec<Finding>,
    pub unchanged_findings: Vec<Finding>,
}
```

**Hash Algorithm:**
```rust
fn generate_finding_hash(file: &str, line: u32, secret: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{}:{}:{}", file, line, secret).as_bytes());
    format!("sha256:{:x}", hasher.finalize())
}
```

**Slim Baseline Mode:**
```json
{
  "version": "1.0.0",
  "results": {
    "config.yml": [
      "sha256:abc123...",  // Just hashes for minimal diff
      "sha256:def456..."
    ]
  }
}
```

#### Estimated Effort

| Task | Time |
|------|------|
| Baseline data structures | 1 day |
| JSON serialization | 1 day |
| Hash generation | 0.5 days |
| Diff algorithm | 2 days |
| Update algorithm | 2 days |
| CLI commands | 2 days |
| Slim mode | 1 day |
| Testing + documentation | 2 days |
| **Total** | **1.5-2 weeks** |

---

### Feature 3: SARIF Output

**Status:** P0 (Critical)
**Effort:** 3-5 days
**Dependencies:** None

#### Description

Generate output in Static Analysis Results Interchange Format (SARIF) for integration with GitHub Advanced Security and other enterprise tools.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **GitHub Advanced Security** | Native integration with code scanning alerts |
| **Enterprise adoption** | Required for many security workflows |
| **Tool interoperability** | Works with any SARIF-compatible viewer |
| **Competitive parity** | All major scanners support SARIF |

**Expected Impact:**
- Enterprise adoption: High
- GitHub integration: Seamless
- Security team workflows: Streamlined

#### Implementation Approach

**SARIF Format (v2.1.0):**
```json
{
  "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
  "version": "2.1.0",
  "runs": [
    {
      "tool": {
        "driver": {
          "name": "DevShield",
          "version": "0.3.0",
          "informationUri": "https://github.com/devshield/devshield",
          "rules": [
            {
              "id": "AWS_ACCESS_KEY",
              "name": "AWS Access Key",
              "shortDescription": {
                "text": "AWS Access Key detected"
              },
              "fullDescription": {
                "text": "AWS Access Keys provide programmatic access to AWS services. Leaked keys can lead to unauthorized access and data breaches."
              },
              "helpUri": "https://docs.aws.amazon.com/general/latest/gr/aws-sec-cred-types.html",
              "defaultConfiguration": {
                "level": "error"
              }
            }
          ]
        }
      },
      "results": [
        {
          "ruleId": "AWS_ACCESS_KEY",
          "level": "error",
          "message": {
            "text": "AWS Access Key detected"
          },
          "locations": [
            {
              "physicalLocation": {
                "artifactLocation": {
                  "uri": "config.yml"
                },
                "region": {
                  "startLine": 5,
                  "startColumn": 12,
                  "snippet": {
                    "text": "aws_access_key_id: AKIAIOSFODNN7EXAMPLE"
                  }
                }
              }
            }
          ],
          "properties": {
            "secretType": "AWS_ACCESS_KEY",
            "verified": false,
            "recommendation": "Remove immediately and rotate the key"
          }
        }
      ]
    }
  ]
}
```

**CLI Integration:**
```bash
# SARIF output
devshield scan secrets --path . --format sarif --output results.sarif

# GitHub Actions integration
devshield scan secrets --path . --format sarif --output results.sarif
# Then upload via github/codeql-action/upload-sarif

# Multiple formats
devshield scan secrets --path . --format json,sarif,text
```

**Implementation Details:**

```rust
// src/output/sarif.rs

use serde::Serialize;

#[derive(Serialize)]
pub struct SarifReport {
    #[serde(rename = "$schema")]
    pub schema: String,
    pub version: String,
    pub runs: Vec<SarifRun>,
}

#[derive(Serialize)]
pub struct SarifRun {
    pub tool: SarifTool,
    pub results: Vec<SarifResult>,
}

#[derive(Serialize)]
pub struct SarifTool {
    pub driver: SarifDriver,
}

#[derive(Serialize)]
pub struct SarifDriver {
    pub name: String,
    pub version: String,
    pub information_uri: String,
    pub rules: Vec<SarifRule>,
}

#[derive(Serialize)]
pub struct SarifRule {
    pub id: String,
    pub name: String,
    #[serde(rename = "shortDescription")]
    pub short_description: SarifMessage,
    #[serde(rename = "fullDescription")]
    pub full_description: SarifMessage,
    #[serde(rename = "helpUri")]
    pub help_uri: String,
    #[serde(rename = "defaultConfiguration")]
    pub default_configuration: SarifConfiguration,
}

#[derive(Serialize)]
pub struct SarifResult {
    #[serde(rename = "ruleId")]
    pub rule_id: String,
    pub level: String,  // "error", "warning", "note"
    pub message: SarifMessage,
    pub locations: Vec<SarifLocation>,
    pub properties: serde_json::Value,
}

impl SarifReport {
    pub fn from_findings(findings: &[Finding]) -> Self {
        // Convert findings to SARIF format
    }
    
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}
```

**GitHub Actions Integration:**
```yaml
# .github/workflows/security-scan.yml
name: Security Scan

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install DevShield
        run: cargo install devshield
      
      - name: Run DevShield
        run: devshield scan secrets --path . --format sarif --output results.sarif
      
      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v3
        with:
          sarif_file: results.sarif
```

#### Estimated Effort

| Task | Time |
|------|------|
| SARIF data structures | 1 day |
| Serialization | 1 day |
| Rule definitions | 1 day |
| CLI integration | 1 day |
| GitHub Actions example | 0.5 days |
| Testing + documentation | 1 day |
| **Total** | **5-6 days** |

---

### Feature 4: Pre-commit Hooks

**Status:** P0 (Critical)
**Effort:** 1 week
**Dependencies:** None

#### Description

Install git pre-commit hooks that automatically scan staged changes for secrets before commits are created.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Prevention** | Stop secrets before they enter git history |
| **Developer workflow** | Minimal friction, automatic scanning |
| **Industry standard** | All major scanners offer pre-commit hooks |
| **Cost savings** | Much cheaper than post-commit remediation |

**Expected Impact:**
- Secret prevention: High
- Developer adoption: High (if fast)
- False positive tolerance: Low (must be <5%)

#### Implementation Approach

**Installation:**
```bash
# Install pre-commit hook
devshield pre-commit install

# Install with custom config
devshield pre-commit install --config .devshield.toml

# Uninstall hook
devshield pre-commit uninstall

# Run hook manually
devshield pre-commit run
```

**Hook Script (installed to .git/hooks/pre-commit):**
```bash
#!/bin/bash
set -e

# DevShield pre-commit hook
# Generated by: devshield pre-commit install

echo "🛡️  Running DevShield secret scan..."

# Get list of staged files
STAGED_FILES=$(git diff --cached --name-only --diff-filter=ACM)

if [ -z "$STAGED_FILES" ]; then
    exit 0
fi

# Run scan on staged files
devshield scan secrets --staged --fail-on critical

if [ $? -ne 0 ]; then
    echo ""
    echo "❌ Secrets detected in staged changes!"
    echo "   Please remove secrets before committing."
    echo "   Use 'git commit --no-verify' to bypass (emergency only)"
    exit 1
fi

echo "✅ No secrets detected"
exit 0
```

**Configuration (.devshield.toml):**
```toml
[pre-commit]
# Fail on severity level
fail-on = "critical"  # critical, high, medium, low

# Enable/disable verification
verify = true

# Timeout for verification (seconds)
verification-timeout = 30

# Ignore patterns
ignore = [
    "*.test.rs",
    "test_*.py",
    "**/fixtures/**"
]

# Specific rules to enable/disable
enable-rules = ["AWS_ACCESS_KEY", "GITHUB_PAT"]
disable-rules = ["GENERIC_SECRET"]
```

**Implementation Details:**

```rust
// src/precommit/mod.rs

pub struct PreCommitHook {
    config: PreCommitConfig,
}

impl PreCommitHook {
    pub fn install(&self, config_path: Option<&Path>) -> Result<(), HookError> {
        // Create .git/hooks/pre-commit script
        // Make executable
    }
    
    pub fn uninstall(&self) -> Result<(), HookError> {
        // Remove .git/hooks/pre-commit
    }
    
    pub fn run(&self, config_path: Option<&Path>) -> Result<ScanResult, HookError> {
        // Get staged files
        // Run scan
        // Return results
    }
    
    fn get_staged_files() -> Result<Vec<PathBuf>, HookError> {
        // Run: git diff --cached --name-only --diff-filter=ACM
    }
}

pub struct PreCommitConfig {
    pub fail_on: Severity,
    pub verify: bool,
    pub verification_timeout: Duration,
    pub ignore: Vec<String>,
    pub enable_rules: Vec<String>,
    pub disable_rules: Vec<String>,
}
```

**Pre-commit Framework Integration:**
```yaml
# .pre-commit-config.yaml
repos:
  - repo: https://github.com/devshield/devshield
    rev: v0.3.0
    hooks:
      - id: devshield
        args: ['--fail-on', 'critical']
```

**Bypass Mechanisms:**
```bash
# Emergency bypass (all hooks)
git commit --no-verify -m "emergency fix"

# Skip specific hook (pre-commit framework)
SKIP=devshield git commit -m "commit message"

# Environment variable bypass
DEVSHIELD_BYPASS=1 git commit -m "commit message"
```

**Performance Requirements:**
- Normal commit (<10 files): <3 seconds
- Large commit (100+ files): <10 seconds
- False positive rate: <5%

#### Estimated Effort

| Task | Time |
|------|------|
| Hook installation logic | 1 day |
| Hook script generation | 1 day |
| Staged file detection | 1 day |
| Configuration parsing | 1 day |
| Pre-commit framework integration | 1 day |
| Bypass mechanisms | 0.5 days |
| Testing + documentation | 1.5 days |
| **Total** | **1 week** |

---

### Feature 5: Plugin Architecture

**Status:** P1 (High)
**Effort:** 2-3 weeks
**Dependencies:** Baseline files complete

#### Description

Extensible plugin system allowing users to add custom detectors, verifiers, and filters.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Extensibility** | Community can add new secret types |
| **Custom detection** | Company-specific secret patterns |
| **Future-proof** | New secret types added without core changes |
| **Competitive parity** | Matches detect-secrets capability |

**Expected Impact:**
- Community contributions: High
- Custom integrations: Enabled
- Long-term maintainability: Improved

#### Implementation Approach

**Plugin Types:**

| Type | Purpose | Example |
|------|---------|---------|
| **Detector** | Find potential secrets | AWSKeyDetector, CustomRegexDetector |
| **Verifier** | Validate secrets | AwsVerifier, GitHubVerifier |
| **Filter** | Reduce false positives | WordFilter, GibberishFilter |

**Plugin API (Rust):**
```rust
// src/plugin/mod.rs

use dyn_clone::DynClone;

/// Base trait for all detectors
pub trait Detector: DynClone + Send + Sync {
    /// Unique identifier for this detector
    fn id(&self) -> &'static str;
    
    /// Human-readable name
    fn name(&self) -> &'static str;
    
    /// Detect secrets in content
    fn detect(&self, content: &str, file_path: &str) -> Vec<Detection>;
    
    /// Configuration for this detector
    fn config(&self) -> Option<&DetectorConfig>;
}

dyn_clone::clone_trait_object!(Detector);

#[derive(Clone)]
pub struct Detection {
    pub detector_id: String,
    pub secret: String,
    pub line: u32,
    pub column: u32,
    pub confidence: f32,  // 0.0 - 1.0
    pub metadata: HashMap<String, String>,
}

/// Base trait for verifiers
pub trait Verifier: DynClone + Send + Sync {
    fn id(&self) -> &'static str;
    fn verify(&self, secret: &str) -> Result<VerificationStatus, VerificationError>;
    fn supported_detectors(&self) -> Vec<&'static str>;
}

dyn_clone::clone_trait_object!(Verifier);

/// Base trait for filters
pub trait Filter: DynClone + Send + Sync {
    fn id(&self) -> &'static str;
    fn should_exclude(&self, detection: &Detection) -> bool;
}

dyn_clone::clone_trait_object!(Filter);
```

**Plugin Loading:**

```rust
// src/plugin/loader.rs

pub struct PluginLoader {
    detectors: Vec<Box<dyn Detector>>,
    verifiers: Vec<Box<dyn Verifier>>,
    filters: Vec<Box<dyn Filter>>,
}

impl PluginLoader {
    pub fn load_builtin(&mut self) {
        // Load built-in plugins
        self.detectors.push(Box::new(AwsKeyDetector::default()));
        self.detectors.push(Box::new(GitHubTokenDetector::default()));
        // ... more built-ins
    }
    
    pub fn load_from_path(&mut self, path: &Path) -> Result<(), PluginError> {
        // Load .so/.dll/.dylib plugin files
        // Use libloading crate for dynamic loading
    }
    
    pub fn load_from_config(&mut self, config: &PluginConfig) -> Result<(), PluginError> {
        // Load plugins specified in config file
        match config.r#type {
            PluginType::Builtin(name) => self.load_builtin_plugin(&name),
            PluginType::External(path) => self.load_from_path(&path),
            PluginType::Wasm(module) => self.load_wasm_plugin(&module),
        }
    }
}
```

**Configuration (.devshield.toml):**
```toml
[[plugins]]
type = "builtin"
name = "AWSKeyDetector"

[[plugins]]
type = "builtin"
name = "GitHubTokenDetector"

[[plugins]]
type = "external"
path = "./plugins/custom_detector.so"

[[plugins]]
type = "wasm"
module = "./plugins/entropy_filter.wasm"

[[plugins]]
type = "regex"
id = "CustomAPIKey"
name = "Custom API Key Detector"
pattern = '''CUSTOM_[A-Za-z0-9]{32}'''
severity = "high"
keywords = ["custom", "api"]
```

**WASM Plugin Support (Future):**
```rust
// plugins/entropy_filter.rs (compiled to WASM)

use devshield_plugin_sdk::*;

#[plugin_entry]
pub fn filter(detection: &Detection) -> bool {
    // Return true to exclude, false to keep
    let entropy = calculate_entropy(&detection.secret);
    entropy < 3.5  // Exclude low-entropy secrets
}
```

**Plugin SDK:**
```toml
# Cargo.toml for plugin
[package]
name = "devshield-custom-detector"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
devshield-plugin-sdk = "0.1"
```

```rust
// Plugin implementation
use devshield_plugin_sdk::*;

#[derive(Clone)]
pub struct CustomDetector {
    pattern: Regex,
}

#[plugin_impl]
impl Detector for CustomDetector {
    fn id(&self) -> &'static str {
        "CUSTOM_API_KEY"
    }
    
    fn name(&self) -> &'static str {
        "Custom API Key Detector"
    }
    
    fn detect(&self, content: &str, file_path: &str) -> Vec<Detection> {
        self.pattern.find_iter(content)
            .map(|m| Detection {
                detector_id: self.id().to_string(),
                secret: m.as_str().to_string(),
                line: get_line_number(content, m.start()),
                column: get_column_number(content, m.start()),
                confidence: 0.9,
                metadata: HashMap::new(),
            })
            .collect()
    }
}
```

**CLI Integration:**
```bash
# List available plugins
devshield plugins list

# Install plugin
devshield plugins install ./my-detector.so

# Enable plugin
devshield plugins enable CustomAPIKey

# Disable plugin
devshield plugins disable GenericSecret

# Run with specific plugins
devshield scan secrets --path . --plugins AWSKeyDetector,CustomAPIKey
```

#### Estimated Effort

| Task | Time |
|------|------|
| Plugin trait definitions | 2 days |
| Plugin loader | 3 days |
| Built-in plugin migration | 3 days |
| Configuration parsing | 2 days |
| CLI commands | 2 days |
| Plugin SDK | 3 days |
| WASM support (optional) | 3 days |
| Documentation + examples | 3 days |
| **Total** | **2.5-3 weeks** |

---

### Feature 6: Encoded Secret Detection

**Status:** P1 (High)
**Effort:** 1 week
**Dependencies:** None

#### Description

Automatically detect and decode base64, hex, and percent-encoded secrets before pattern matching.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Evasion prevention** | Catch secrets hidden via encoding |
| **Real-world coverage** | Developers often encode secrets in config |
| **Competitive parity** | Matches Gitleaks capability |

**Expected Impact:**
- Detection rate: +10-15%
- Evasion resistance: High
- Performance impact: Minimal (<10% slowdown)

#### Implementation Approach

**Decoding Pipeline:**
```
Raw Content
    │
    ▼
┌─────────────────┐
│  Pattern Match  │  ← First pass on raw content
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Find Encoded   │  ← Look for encoded patterns
│  Candidates     │
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Decode         │  ← Base64, Hex, Percent
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Pattern Match  │  ← Second pass on decoded content
└─────────────────┘
    │
    ▼
┌─────────────────┐
│  Report         │  ← Include encoding info
└─────────────────┘
```

**Supported Encodings:**

| Encoding | Detection Pattern | Min Length |
|----------|------------------|------------|
| **Base64** | `[A-Za-z0-9+/]{16,}={0,2}` | 16 chars |
| **Base64URL** | `[A-Za-z0-9_-]{16,}` | 16 chars |
| **Hex** | `[0-9a-fA-F]{32,}` | 32 chars |
| **Percent** | `%[0-9a-fA-F]{2}` sequences | 3+ encoded chars |

**Implementation Details:**

```rust
// src/encoding/mod.rs

pub enum EncodingType {
    Base64,
    Base64Url,
    Hex,
    Percent,
}

pub struct EncodedSecret {
    pub original: String,
    pub decoded: String,
    pub encoding: EncodingType,
    pub line: u32,
    pub column: u32,
    pub nested_depth: u32,  // For recursively encoded secrets
}

pub fn find_encoded_secrets(content: &str, max_depth: u32) -> Vec<EncodedSecret> {
    let mut results = Vec::new();
    
    for (line_num, line) in content.lines().enumerate() {
        // Find base64 candidates
        for m in BASE64_PATTERN.find_iter(line) {
            if let Ok(decoded) = base64_decode(m.as_str()) {
                if is_potential_secret(&decoded) {
                    results.push(EncodedSecret {
                        original: m.as_str().to_string(),
                        decoded,
                        encoding: EncodingType::Base64,
                        line: line_num as u32,
                        column: m.start() as u32,
                        nested_depth: 0,
                    });
                }
            }
        }
        
        // Find hex candidates
        for m in HEX_PATTERN.find_iter(line) {
            if let Ok(decoded) = hex_decode(m.as_str()) {
                if is_potential_secret(&decoded) {
                    results.push(EncodedSecret {
                        original: m.as_str().to_string(),
                        decoded,
                        encoding: EncodingType::Hex,
                        line: line_num as u32,
                        column: m.start() as u32,
                        nested_depth: 0,
                    });
                }
            }
        }
        
        // Find percent-encoded candidates
        // ... similar logic
    }
    
    // Recursive decoding (if max_depth > 0)
    if max_depth > 0 {
        for encoded in &results {
            let nested = find_encoded_secrets(&encoded.decoded, max_depth - 1);
            results.extend(nested);
        }
    }
    
    results
}

fn is_potential_secret(decoded: &str) -> bool {
    // Check if decoded content looks like a secret
    // - Contains key patterns (KEY, TOKEN, SECRET, etc.)
    // - Has high entropy
    // - Matches known secret formats
}
```

**CLI Integration:**
```bash
# Enable encoded secret detection (default)
devshield scan secrets --path .

# Set max decode depth
devshield scan secrets --path . --max-decode-depth 2

# Disable encoded detection (faster)
devshield scan secrets --path . --no-decode

# Show encoding info in output
devshield scan secrets --path . --show-encoding
```

**Output Enhancement:**
```
🚨 CRITICAL: AWS_ACCESS_KEY (base64-encoded)
   File: ./config.yml:5
   Original: QktJQTEyMzQ1Njc4OTBBQkNERUY=
   Decoded:  BKIA1234567890ABCDEF
   Recommendation: Remove immediately and rotate the key
```

**Performance Optimization:**
- Cache decoded results
- Skip already-decoded content
- Parallel decoding for large files
- Early exit if decoded content doesn't match patterns

#### Estimated Effort

| Task | Time |
|------|------|
| Encoding detection | 2 days |
| Decoding functions | 1 day |
| Recursive decoding | 1 day |
| Pattern matching on decoded | 1 day |
| CLI integration | 1 day |
| Performance optimization | 1 day |
| Testing + documentation | 1 day |
| **Total** | **1 week** |

---

### Feature 7: Archive Scanning

**Status:** P2 (Medium)
**Effort:** 1-2 weeks
**Dependencies:** None

#### Description

Extract and scan compressed archives (ZIP, TAR, GZ, etc.) for secrets, including nested archives.

#### Why Valuable

| Benefit | Impact |
|---------|--------|
| **Complete coverage** | Secrets in archives are still secrets |
| **Real-world scenarios** | Config archives, backups, deployments |
| **Competitive parity** | Matches TruffleHog/Gitleaks capability |

**Expected Impact:**
- Detection coverage: +5-10%
- Security completeness: High
- Performance impact: Configurable

#### Implementation Approach

**Supported Formats:**
- ZIP
- TAR
- GZIP
- BZIP2
- XZ
- 7Z (read-only)
- RAR (read-only)

**Implementation Details:**

```rust
// src/archive/mod.rs

use zip::read::ZipArchive;
use tar::Archive as TarArchive;
use flate2::read::GzDecoder;

pub struct ArchiveScanner {
    max_size: u64,      // Max archive size to scan
    max_depth: u32,     // Max nested archive depth
    timeout: Duration,  // Max extraction time
}

impl ArchiveScanner {
    pub fn scan(&self, path: &Path) -> Result<Vec<Finding>, ArchiveError> {
        let file = File::open(path)?;
        let metadata = file.metadata()?;
        
        // Check size limit
        if metadata.len() > self.max_size {
            return Err(ArchiveError::TooLarge(metadata.len()));
        }
        
        self.extract_and_scan(&file, path, 0)
    }
    
    fn extract_and_scan(
        &self,
        file: &impl Read,
        path: &Path,
        depth: u32,
    ) -> Result<Vec<Finding>, ArchiveError> {
        if depth > self.max_depth {
            return Ok(Vec::new());
        }
        
        let mut findings = Vec::new();
        let start = Instant::now();
        
        // Detect archive type and extract
        match self.detect_archive_type(file)? {
            ArchiveType::Zip => {
                let mut archive = ZipArchive::new(file)?;
                for i in 0..archive.len() {
                    let mut entry = archive.by_index(i)?;
                    if entry.is_file() {
                        let mut content = Vec::new();
                        entry.read_to_end(&mut content)?;
                        
                        // Check if nested archive
                        if self.is_archive(&content) {
                            let nested = self.extract_and_scan(
                                &mut &content[..],
                                &path.join(entry.name()),
                                depth + 1,
                            )?;
                            findings.extend(nested);
                        } else {
                            // Scan file content
                            if let Ok(text) = String::from_utf8(content) {
                                let file_findings = self.scanner.scan_content(
                                    &text,
                                    &path.join(entry.name()).to_string_lossy(),
                                );
                                findings.extend(file_findings);
                            }
                        }
                    }
                }
            }
            ArchiveType::Tar => {
                // Similar logic for TAR archives
            }
            ArchiveType::Gzip => {
                // Similar logic for GZIP archives
            }
            // ... other formats
        }
        
        // Check timeout
        if start.elapsed() > self.timeout {
            return Err(ArchiveError::Timeout);
        }
        
        Ok(findings)
    }
}
```

**CLI Integration:**
```bash
# Scan with archive support (default)
devshield scan secrets --path .

# Set max archive size
devshield scan secrets --path . --archive-max-size 10MB

# Set max nested depth
devshield scan secrets --path . --archive-max-depth 3

# Set extraction timeout
devshield scan secrets --path . --archive-timeout 60s

# Skip archives (faster)
devshield scan secrets --path . --no-archives
```

**Output Enhancement:**
```
🚨 CRITICAL: AWS_ACCESS_KEY
   File: ./backup.zip:config/prod.yml:5
   Recommendation: Remove immediately and rotate the key
```

**Security Considerations:**
- Zip bomb protection (size limits)
- Path traversal protection (validate extracted paths)
- Symlink protection (don't follow symlinks in archives)
- Resource limits (memory, CPU, time)

#### Estimated Effort

| Task | Time |
|------|------|
| Archive type detection | 1 day |
| ZIP extraction | 1 day |
| TAR extraction | 1 day |
| GZIP/BZIP2/XZ extraction | 1 day |
| Nested archive handling | 2 days |
| Security protections | 2 days |
| CLI integration | 1 day |
| Testing + documentation | 2 days |
| **Total** | **1.5-2 weeks** |

---

## Phase 3 Roadmap

### Week 1-2: Live Verification
- Verifier trait architecture
- AWS, GitHub, Stripe verifiers
- Verification caching
- Rate limiting

### Week 3: Baseline Files
- Baseline data structures
- Diff/update algorithms
- CLI commands

### Week 4: SARIF Output
- SARIF format implementation
- GitHub Actions integration
- Documentation

### Week 5: Pre-commit Hooks
- Hook installation
- Staged file scanning
- Configuration

### Week 6-8: Plugin Architecture
- Plugin trait definitions
- Plugin loader
- Built-in migration
- Plugin SDK

### Week 9: Encoded Secret Detection
- Encoding detection
- Decoding pipeline
- Performance optimization

### Week 10-11: Archive Scanning
- Archive extraction
- Nested archive handling
- Security protections

### Week 12: Integration + Testing
- End-to-end testing
- Performance benchmarking
- Documentation

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| **False positive rate** | <1% (verified) | Manual review of 100 findings |
| **Detection rate** | >95% | Seeded test repos |
| **Scan speed** | <1s per 100 files | Benchmark suite |
| **Memory usage** | <100MB | Profiling tools |
| **Pre-commit time** | <3s (normal), <10s (large) | Git hook timing |
| **SARIF compatibility** | GitHub Advanced Security | Upload test |
| **Plugin adoption** | 5+ community plugins | GitHub search |

---

## Risks and Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| API rate limiting | Verification failures | Caching, backoff, quotas |
| Plugin security | Malicious plugins | Sandboxing, code review |
| Archive bombs | DoS attacks | Size limits, timeouts |
| False positives | Developer trust | Verification, audit mode |
| Performance regression | Slow scans | Benchmarking, profiling |

---

## Appendix: Feature Comparison

| Feature | Coax (Current) | Coax (Phase 3) | TruffleHog | Gitleaks |
|---------|---------------|----------------|------------|----------|
| Regex detection | ✅ | ✅ | ✅ | ✅ |
| Entropy detection | ✅ | ✅ | ✅ | ✅ |
| Live verification | ❌ | ✅ | ✅ | ❌ |
| Baseline files | ❌ | ✅ | ❌ | ✅ |
| SARIF output | ❌ | ✅ | ✅ | ✅ |
| Pre-commit hooks | ❌ | ✅ | ✅ | ✅ |
| Plugin architecture | ❌ | ✅ | ❌ | ❌ |
| Encoded secrets | ❌ | ✅ | ❌ | ✅ |
| Archive scanning | ❌ | ✅ | ✅ | ✅ |

---

*Document created: 2026-03-15*
*Next: Benchmark Plan (docs/BENCHMARK-PLAN.md)*
