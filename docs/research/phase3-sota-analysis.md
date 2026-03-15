# Phase 3 SOTA Security Scanner Analysis

**Date:** 2026-03-15
**Author:** DevShield Research Team
**Status:** Complete

---

## Executive Summary

This document analyzes 5 state-of-the-art open-source security scanners to inform Phase 3 feature planning for Coax/DevShield. We examined **TruffleHog**, **Gitleaks**, **GitGuardian (ggshield)**, **Semgrep**, and **Yelp detect-secrets** to identify feature gaps and opportunities.

**Key Findings:**
- **Live verification** is the #1 differentiator (TruffleHog, GitGuardian)
- **AST-based analysis** provides superior accuracy (Semgrep)
- **Baseline files** are essential for CI/CD workflows (Gitleaks, detect-secrets)
- **Plugin architecture** enables extensibility (detect-secrets)
- **SARIF output** is table stakes for enterprise integration (all except detect-secrets)

---

## 1. TruffleHog Analysis

**Repository:** https://github.com/trufflesecurity/trufflehog
**Language:** Go (99.9%)
**License:** GPL-3.0
**Stars:** 20K+
**Downloads:** 10M+

### 1.1 Detection Methods

| Method | Description | Implementation Complexity |
|--------|-------------|--------------------------|
| **Pattern-based** | 800+ credential detectors for specific secret types | Medium |
| **Custom regex** | User-defined regular expressions with keyword matching | Low |
| **Entropy filtering** | Shannon entropy filtering for unverified results (recommended: 3.0) | Low |
| **Generic JWT** | Detects JWTs using public-key cryptography (non-HMAC) | Medium |
| **Canary tokens** | Static detection of `https://canarytokens.org/` tokens | Low |
| **Multi-format** | Scans binaries, documents, archives, compressed files | High |

**Key Insight:** TruffleHog uses **classification + verification** architecture. First classify the secret type, then verify via API.

### 1.2 Live Verification Approach

**Verification Statuses:**
- `verified` - Credential confirmed valid by API testing
- `unverified` - Credential detected but not confirmed
- `unknown` - Verification attempted but failed (network/API errors)

**Verification Methods by Secret Type:**

| Secret Type | Verification Method | Safety |
|-------------|-------------------|--------|
| AWS Keys | `GetCallerIdentity` API call | ✅ Read-only |
| GitHub Tokens | `/user` API endpoint | ✅ Read-only |
| Stripe Keys | Balance retrieval API | ✅ Read-only |
| Google API | OAuth token info | ✅ Read-only |
| Private Keys | Driftwood technology (GitHub/TLS cert matching) | ✅ Passive |
| Custom Regex | Webhook endpoint (200 OK = verified) | ⚠️ User-defined |

**Verification Caching:**
- Enabled by default
- Can be disabled with `--no-verification-cache`
- Custom verification endpoints via `--verifier` flag

**Key Insight:** TruffleHog verifies **700+ credential types** via live API calls. This is their primary false positive reduction mechanism.

### 1.3 False Positive Handling

| Technique | Description | Effectiveness |
|-----------|-------------|---------------|
| **Active verification** | API testing confirms validity | ⭐⭐⭐⭐⭐ |
| **Entropy filtering** | `--filter-entropy` flag (start with 3.0) | ⭐⭐⭐ |
| **Filter unverified** | `--filter-unverified` outputs only first unverified per chunk | ⭐⭐⭐ |
| **Inline ignore** | `trufflehog:ignore` comment on line | ⭐⭐⭐⭐ |
| **Detector exclusion** | `--exclude-detectors` for fine-tuning | ⭐⭐⭐⭐ |
| **Results filtering** | `--results` flag (verified/unverified/unknown) | ⭐⭐⭐⭐ |

**Published FP Rate:** Not publicly disclosed, but verification reduces FP rate to <1% for verified secrets.

### 1.4 Key Features Coax Lacks

| Feature | Description | Priority |
|---------|-------------|----------|
| **Live verification** | API-based secret validation | P0 |
| **800+ detectors** | Comprehensive secret type coverage | P1 |
| **Deep analysis** | Identifies creator, permissions, accessible resources | P1 |
| **Multi-source scanning** | Git, GitHub, GitLab, S3, GCS, Docker, etc. | P1 |
| **Verification caching** | Improves performance on re-scans | P2 |
| **Driftwood technology** | Private key verification against GitHub/TLS certs | P2 |
| **Archive scanning** | Configurable max size, depth, timeout | P2 |
| **IAM role assumption** | Cross-account S3 scanning | P2 |
| **Deleted commit scanning** | Experimental feature for historical commits | P3 |
| **Cross-fork references** | Experimental GitHub object discovery | P3 |

### 1.5 Architecture

```
┌─────────────────────────────────────────────────────────┐
│                    TruffleHog CLI                       │
│  (Go-based, v3.0+ complete rewrite)                     │
└─────────────────────────────────────────────────────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐  ┌────────────────┐  ┌───────────────┐
│   Sources     │  │   Detectors    │  │   Verifiers   │
│  (Input)      │  │  (Classification)│ │  (Validation) │
├───────────────┤  ├────────────────┤  ├───────────────┤
│ • Git repos   │  │ • 800+ secret  │  │ • API calls   │
│ • GitHub API  │  │   type detectors│  │ • Webhook     │
│ • S3/GCS      │  │ • Custom regex │  │   endpoints   │
│ • Docker      │  │ • Entropy      │  │ • Driftwood   │
│ • Filesystem  │  │   filtering    │  │   (keys/certs)│
│ • CI/CD       │  │ • Classification│  │               │
│ • Logs/Chats  │  │                │  │               │
└───────────────┘  └────────────────┘  └───────────────┘
        │                   │                   │
        └───────────────────┼───────────────────┘
                            ▼
                  ┌─────────────────┐
                  │    Analysis     │
                  │  (Permissions,  │
                  │   Resources)    │
                  └─────────────────┘
```

### 1.6 Performance Characteristics

| Metric | Value |
|--------|-------|
| **Concurrency** | Default 12 workers (configurable) |
| **Detector timeout** | Configurable per-detector (e.g., 30s) |
| **Archive limits** | Configurable max size/depth/timeout |
| **Scan duration** | 20 min to several hours (large repos) |
| **Binary skipping** | Optional for performance |
| **Verification caching** | Enabled by default |

---

## 2. Gitleaks Analysis

**Repository:** https://github.com/gitleaks/gitleaks
**Language:** Go
**License:** MIT
**Stars:** 19K+
**Downloads:** 20M+ Docker

### 2.1 Detection Patterns and Methods

| Method | Description | Complexity |
|--------|-------------|------------|
| **Regex-based** | Golang regular expressions (no lookahead) | Low |
| **Entropy analysis** | Shannon entropy on regex groups (configurable threshold) | Low |
| **Keyword pre-filter** | Quick string matching before regex | Low |
| **Path-based matching** | Regex patterns for file paths | Low |
| **Composite rules** | Multi-part rules with proximity constraints | Medium |
| **Encoded secret detection** | Auto-decoding (percent, hex, base64) | Medium |
| **Archive scanning** | Nested archive extraction | High |

**Composite Rules Example:**
```toml
[[rules]]
id = "composite-example"
description = "Multi-part secret"

[[rules.required]]
id = "primary"
regex = '''pattern1'''

[[rules.required]]
id = "auxiliary"
regex = '''pattern2'''
withinLines = 5  # Must be within 5 lines
```

**Encoded Secret Detection:**
- Percent encoding
- Hex encoding (≥32 characters)
- Base64 encoding (≥16 characters)
- Recursive decoding supported
- "Minimally increases scan times"

### 2.2 Configuration System

**Configuration Loading (Order of Precedence):**
1. `--config/-c` flag
2. Environment variable `GITLEAKS_CONFIG` (file path)
3. Environment variable `GITLEAKS_CONFIG_TOML` (file content)
4. `.gitleaks.toml` in target path
5. Default built-in config

**Configuration Structure (TOML):**
```toml
title = "Custom Gitleaks configuration"

[extend]
useDefault = true  # Extend default config
# OR
path = "common_config.toml"
disabledRules = ["generic-api-key"]

[[rules]]
id = "awesome-rule-1"
description = "awesome rule 1"
regex = '''pattern'''
secretGroup = 3
entropy = 3.5
path = '''file-path-regex'''
keywords = ["auth", "password"]
tags = ["tag"]

[[rules.allowlists]]
condition = "OR"  # or "AND"
commits = ["commit-A"]
paths = ['''go\.mod''']
regexes = ['''pattern''']
stopwords = ["client"]

[[allowlists]]  # Global allowlists (higher precedence)
targetRules = ["awesome-rule-1"]
commits = ["commit-A"]
paths = ['''gitleaks\.toml''']
regexes = ['''pattern''']
stopwords = ["client"]
```

**Allowlist Features:**
- Rule-specific (`[[rules.allowlists]]`) vs Global (`[[allowlists]]`)
- Condition logic: "OR" (any matches) or "AND" (all must match)
- Criteria: commits, paths, regexes, stopwords
- Target rules: Assign allowlists to specific rules

**Inline Overrides:**
- `#gitleaks:allow` comment to ignore specific lines
- `.gitleaksignore` file for fingerprint-based ignoring

### 2.3 Performance Characteristics

| Aspect | Details |
|--------|---------|
| **Decoding overhead** | "Minimally increases scan times" |
| **Archive scanning** | Recursion stops when no new archives |
| **File size limits** | `--max-target-megabytes` flag |
| **Timeout control** | `--timeout` flag (default "0" = no timeout) |
| **Diagnostics** | `--diagnostics` for profiling (cpu, mem, trace, http) |
| **Scan types** | git, dir, stdin |

**Built-in Rules:** 50+ secret detection rules

### 2.4 Key Features Coax Lacks

| Feature | Description | Priority |
|---------|-------------|----------|
| **Baseline comparison** | `--baseline-path` for incremental scanning | P0 |
| **Composite rules** | Multi-part detection with proximity | P1 |
| **Encoded secret detection** | Auto-decoding (base64, hex, percent) | P1 |
| **Archive scanning** | Nested archive extraction | P2 |
| **Fingerprint-based ignoring** | `.gitleaksignore` file | P2 |
| **Custom report templates** | Go text/template support | P2 |
| **Redaction** | `--redact` flag (0-100%) | P3 |
| **Diagnostics mode** | Profiling for performance tuning | P3 |

### 2.5 Output Formats

| Format | Description |
|--------|-------------|
| **JSON** | Standard JSON report |
| **CSV** | Comma-separated values |
| **JUnit** | JUnit XML format for CI/CD |
| **SARIF** | Static Analysis Results Interchange Format |
| **Template** | Custom format via Go text/template |

**Verbose Output Example:**
```
Finding:     "export BUNDLE_ENTERPRISE__CONTRIBSYS__COM=cafebabe:deadbeef"
Secret:      cafebabe:deadbeef
RuleID:      sidekiq-secret
Entropy:     2.609850
File:        cmd/generate/config/rules/sidekiq.go
Line:        23
Commit:      cd5226711335c68be1e720b318b7bc3135a30eb2
Author:      John
Email:       john@users.noreply.github.com
Date:        2022-08-03T12:31:40Z
Fingerprint: cd5226711335c68be1e720b318b7bc3135a30eb2:cmd/generate/config/rules/sidekiq.go:sidekiq-secret:23
```

---

## 3. GitGuardian (ggshield) Analysis

**Repository:** https://github.com/GitGuardian/ggshield
**Language:** Python
**License:** MIT
**Stars:** 2K+
**Company:** GitGuardian (commercial backing)

### 3.1 CLI Features and Commands

**Main Command Structure:**
```bash
ggshield secret scan <mode>
```

**Scan Modes:**
- `path -r .` - Scan files recursively
- `repo .` - Scan repositories
- `docker ubuntu:22.04` - Scan Docker images
- `pypi flask` - Scan PyPI packages
- `pre-commit` - Scan as pre-commit hook
- `pre-push` - Scan as pre-push hook
- `pre-receive` - Scan as pre-receive hook

**Additional Commands:**
- `ggshield auth login` - Authenticate with GitGuardian (automates PAT provisioning)

### 3.2 Integrations

| Integration Type | Details |
|-----------------|---------|
| **Git Hooks** | Pre-commit, pre-push, pre-receive |
| **CI/CD** | GitHub Actions, general CI environments |
| **Package Managers** | Homebrew (macOS), Chocolatey (Windows), Deb/RPM (Linux), pipx/pip |
| **Docker** | Docker image scanning, available as container |
| **IDE** | Not explicitly documented |

### 3.3 Secret Verification Approach

**Detection Method:**
- Uses GitGuardian public API via `py-gitguardian` library
- Detects **500+ types of secrets**
- Validates secrets against GitGuardian servers

**Authentication:**
- Requires personal access token (PAT)
- Stored in `GITGUARDIAN_API_KEY` environment variable
- Or via `ggshield auth login` command

**Data Storage Policy:**
- Only metadata stored (call time, request size, scan mode)
- **Secrets are NOT displayed on dashboard**
- **Files and secrets are NOT stored**

### 3.4 Enterprise Features

**Dashboard Capabilities:**
- **Unified Incident Management** - Centralize incidents across source control and productivity tools
- **Detailed Incident Investigation** - Trace connections, context, and impact via "secrets exploration map"
- **Prioritized Remediation** - Automated severity scoring, AI-enriched contextual tagging
- **Enhanced Collaboration** - Granular access and member permissions for teams
- **Automated Playbooks** - Time-saving automation for remediation workflows
- **Custom Remediation Guidelines** - Align with internal processes and knowledge bases
- **Centralized NHI Inventory** - Visibility across vaults and identity sources
- **Actionable Analytics** - Compliance tracking across NHI ecosystem

**Key Metric:** Volume of *fixed* incidents (not just detected)

### 3.5 Key Features Coax Lacks

| Feature | Description | Priority |
|---------|-------------|----------|
| **Cloud API verification** | GitGuardian API-based validation | P0 |
| **Docker image scanning** | Scan container images for secrets | P1 |
| **PyPI package scanning** | Scan Python packages | P2 |
| **Enterprise dashboard** | Incident management, analytics | P2 |
| **Automated playbooks** | Remediation workflows | P3 |
| **NHI governance** | Non-Human Identity management | P3 |

### 3.6 Output Format

**Text Output (Default):**
- Shows filename with incidents
- Displays patch with line numbers
- Highlights detected secrets with underlines
- Shows secret type label (e.g., "AWS Keys")
- Truncates long lines (unless verbose mode)

**Exit Codes:**
- `0` - No secrets found
- Non-zero - Secrets detected

---

## 4. Semgrep Analysis

**Repository:** https://github.com/returntocorp/semgrep
**Language:** OCaml (77.4%), Python (19.2%)
**License:** LGPL-2.1
**Stars:** 9K+
**Company:** Semgrep (commercial backing)

### 4.1 Rule Format and Syntax

**Key Characteristics:**
- Rules look like the code you already write
- Uses pattern matching with metavariables (e.g., `$X == $X`)
- Supports language-specific patterns via `--lang` flag
- No abstract syntax trees, regex wrestling, or painful DSLs (for basic rules)

**Rule Sources:**
- **Semgrep Registry:** 2,000+ community-driven rules
- **Pro Rules:** 20,000+ proprietary rules (SAST, SCA, secrets)
- **Custom Rules:** User-defined via `-e` flag or rule files

**Rule Categories:**
- Ban dangerous APIs
- Search routes and authentication
- Enforce secure defaults
- Enforce project best-practices
- Codify project-specific knowledge
- Apply automatic fixes
- Migrate from deprecated APIs
- Audit configuration files
- Audit security hotspots

### 4.2 AST-Based Analysis Approach

**Core Engine:**
- "Semantic grep for code" - understands code semantics
- While `grep "2"` only matches exact string "2", Semgrep matches `x = 1; y = x + 1` when searching for `2`

**Analysis Capabilities by Edition:**

| Edition | Analysis Scope |
|---------|---------------|
| **Community** | Single function/file boundaries |
| **AppSec Platform (Pro)** | Cross-file, cross-function, data-flow reachability |

**Post-Processing:**
- Semgrep Assistant (AI) performs contextual post-processing
- Reduces noise by ~20%
- Provides tailored, step-by-step remediation guidance

### 4.3 Performance Characteristics

| Aspect | Details |
|--------|---------|
| **Speed** | Marketed as "CODE SCANNING AT LUDICROUS SPEED" |
| **Execution** | Local execution (code never uploaded by default) |
| **Language Support** | 30+ languages for code analysis |
| **Dependency Scanning** | 12 languages across 15 package managers |
| **Metrics** | Pseudonymous rule metrics reported (can disable) |

### 4.4 Key Features for Security Scanning

| Feature | Description | Performance |
|---------|-------------|-------------|
| **SAST** | Static application security testing with AI triage | 25% ↓ false positives |
| **SCA** | Detects reachable vulnerabilities in third-party libraries | 250% ↑ true positives |
| **Secrets Scanning** | Semantic analysis + improved entropy + validation | Not disclosed |
| **Assistant Auto-Triage** | AI-powered finding validation | 97% agreement with humans |
| **Remediation Guidance** | Step-by-step fix instructions | 80% rated actionable |

### 4.5 Key Features Coax Lacks

| Feature | Description | Priority |
|---------|-------------|----------|
| **AST-based analysis** | Semantic code understanding | P0 |
| **Cross-file analysis** | Data flow across files (Pro) | P1 |
| **Taint tracking** | Track untrusted data to sinks | P1 |
| **Reachability analysis** | Detect if vulnerable code is actually called | P1 |
| **AI triage** | Contextual post-processing of findings | P2 |
| **2,000+ registry rules** | Community-driven rule library | P2 |
| **Automatic fixes** | Apply fixes via `--autofix` | P3 |
| **Dependency scanning** | SCA for vulnerable packages | P3 |

### 4.6 Output Formats

| Format | Description |
|--------|-------------|
| **Terminal/CLI** | Default text format with file paths and line numbers |
| **IDE diagnostics** | Editor integration |
| **PR comments** | GitHub/GitLab integration |
| **SARIF** | Supported for GitHub Advanced Security |
| **JSON** | Machine-readable output |

---

## 5. Yelp detect-secrets Analysis

**Repository:** https://github.com/Yelp/detect-secrets
**Language:** Python
**License:** Apache-2.0
**Stars:** 6K+
**Company:** Yelp (open source)

### 5.1 Plugin Architecture

**Three Detection Strategies:**

| Plugin Type | Description | Examples |
|-------------|-------------|----------|
| **Regex-based Rules** | Pattern matching with optional network verification | AWSKeyDetector, GitHubTokenDetector |
| **Entropy Detector** | "Secret-looking" strings via heuristics | Base64HighEntropyString, HexHighEntropyString |
| **Keyword Detector** | Variable names associated with secrets | KeywordDetector |

**Built-in Plugins (27 total):**
- ArtifactoryDetector, AWSKeyDetector, AzureStorageKeyDetector
- BasicAuthDetector, CloudantDetector, DiscordBotTokenDetector
- GitHubTokenDetector, GitLabTokenDetector
- Base64HighEntropyString, HexHighEntropyString
- IbmCloudIamDetector, IbmCosHmacDetector
- JwtTokenDetector, KeywordDetector, MailchimpDetector
- NpmDetector, OpenAIDetector, PrivateKeyDetector
- PypiTokenDetector, SendGridDetector, SlackDetector
- SquareOAuthDetector, StripeDetector, TelegramBotTokenDetector
- TwilioKeyDetector, IPPublicDetector

**Custom Plugins:**
- Added via `--plugin` flag
- Local file paths or Python module paths
- Full API for custom detection logic

### 5.2 Baseline File Handling

**Baseline Concept:** Snapshot of existing secrets for separation of concerns

| Command | Purpose |
|---------|---------|
| `detect-secrets scan > .secrets.baseline` | Create initial baseline |
| `detect-secrets scan --baseline .secrets.baseline` | Update baseline (add new, remove old) |
| `detect-secrets-hook --baseline .secrets.baseline` | Block new secrets not in baseline |
| `detect-secrets audit .secrets.baseline` | Label results (real/false positive) |

**Baseline Features:**
- Backwards compatible (versions <0.9 need recreation)
- **Slim baselines** (`--slim` flag) for minimal diff differences
- Stores plugin configurations and filter settings
- Can be audited to distinguish true/false positives

**Baseline File Format (JSON):**
```json
{
  "version": "1.5.0",
  "plugins_used": [
    {"name": "AWSKeyDetector"},
    {"name": "Base64HighEntropyString", "limit": 4.5}
  ],
  "filters_used": [...],
  "results": {
    "file.py": [
      {
        "type": "Secret Type",
        "hash": "sha1_hash",
        "line_number": 42,
        "is_verified": false
      }
    ]
  }
}
```

### 5.3 Entropy Detection Method

**Two Entropy Plugins:**

| Plugin | Default Limit | Range | Algorithm |
|--------|---------------|-------|-----------|
| **Base64HighEntropyString** | 4.5 | 0.0-8.0 | Shannon entropy |
| **HexHighEntropyString** | 3.0 | 0.0-8.0 | Shannon entropy |

**Configuration:**
```bash
detect-secrets scan --base64-limit 5.0 --hex-limit 4.0
```

**Extensions:**
- **Gibberish Detector:** ML model to determine if secret is gibberish (requires `gibberish-detector` package)
- **Wordlist:** Exclude secrets containing words from list (requires `pyahocorasick`)

### 5.4 Key Features Coax Lacks

| Feature | Description | Priority |
|---------|-------------|----------|
| **Plugin architecture** | Extensible detection system | P0 |
| **Baseline files** | Incremental scanning support | P0 |
| **Audit mode** | Interactive labeling of findings | P1 |
| **Gibberish detection** | ML-based FP reduction | P2 |
| **Wordlist filtering** | Exclude known words | P2 |
| **Slim baselines** | Diff-friendly baseline format | P2 |
| **Programmatic API** | Python library for integration | P3 |
| **Statistics & reporting** | `--stats` and `--report` flags | P3 |

### 5.5 Configuration Options

**Plugin Configuration:**
```bash
--disable-plugin PLUGIN_NAME     # Disable specific plugins
--plugin PATH                    # Add custom plugin
--list-all-plugins               # Show all enabled plugins
--base64-limit [0.0-8.0]         # Adjust Base64 entropy threshold
--hex-limit [0.0-8.0]            # Adjust Hex entropy threshold
```

**Filter Configuration:**
```bash
--exclude-lines REGEX            # Ignore lines matching pattern
--exclude-files REGEX            # Ignore files matching pattern
--exclude-secrets REGEX          # Ignore secret values matching pattern
--word-list FILE                 # Use wordlist for exclusion
--filter PATH                    # Add custom filter
--disable-filter FILTER_NAME     # Disable specific filter
--no-verify                      # Disable network verification
--only-verified                  # Only flag verifiable secrets
```

### 5.6 Integration Capabilities

**Pre-commit Hook (recommended):**
```yaml
# .pre-commit-config.yaml
repos:
-   repo: https://github.com/Yelp/detect-secrets
    rev: v1.5.0
    hooks:
    -   id: detect-secrets
        args: ['--baseline', '.secrets.baseline']
        exclude: package.lock.json
```

**CI/CD Integration:**
```bash
# Scanning staged files
git diff --staged --name-only -z | xargs -0 detect-secrets-hook --baseline .secrets.baseline

# Scanning all tracked files
git ls-files -z | xargs -0 detect-secrets-hook --baseline .secrets.baseline
```

**Programmatic API:**
```python
from detect_secrets import SecretsCollection
from detect_secrets.settings import default_settings, transient_settings

# Basic use
secrets = SecretsCollection()
with default_settings():
    secrets.scan_file('test_data/config.ini')
    print(json.dumps(secrets.json(), indent=2))

# Advanced configuration
with transient_settings({
    'plugins_used': [{'name': 'Base64HighEntropyString', 'limit': 5.0}],
    'filters_used': [{'path': 'file://filters/example.py::is_identified_by_ML_model'}]
}) as settings:
    secrets.scan_file('test_data/config.ini')
```

---

## 6. Feature Gap Analysis

### 6.1 Comparison Matrix

| Feature | Coax | TruffleHog | Gitleaks | GitGuardian | Semgrep | detect-secrets |
|---------|------|------------|----------|-------------|---------|----------------|
| **Regex detection** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Entropy detection** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Token efficiency** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **Word filter** | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Live verification** | ❌ | ✅ | ❌ | ✅ | ❌ | ✅ (optional) |
| **AST analysis** | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **Pre-commit hooks** | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **CI/CD integration** | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Baseline files** | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ |
| **TUI dashboard** | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **SARIF output** | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Plugin architecture** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Custom rules** | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ |
| **Multi-source scan** | ❌ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **Archive scanning** | ❌ | ✅ | ✅ | ❌ | ❌ | ❌ |
| **Encoded secrets** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Cross-file analysis** | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **Taint tracking** | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **AI triage** | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ |
| **Docker scanning** | ❌ | ✅ | ❌ | ✅ | ❌ | ❌ |
| **PyPI scanning** | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ |
| **Baseline audit mode** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Gibberish detection** | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **Fingerprint ignoring** | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ |
| **Custom templates** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Redaction** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| **Diagnostics mode** | ❌ | ❌ | ✅ | ❌ | ❌ | ❌ |
| **IAM role assumption** | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Driftwood technology** | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |
| **Verification caching** | ❌ | ✅ | ❌ | ❌ | ❌ | ❌ |

### 6.2 Top 10 Missing Features (Prioritized)

| Rank | Feature | Why Valuable | Effort |
|------|---------|--------------|--------|
| 1 | **Live verification** | Eliminates false positives, prioritizes real risks | 2-3 weeks |
| 2 | **Baseline files** | Essential for CI/CD workflows, incremental scanning | 1-2 weeks |
| 3 | **SARIF output** | Enterprise integration (GitHub Advanced Security) | 3-5 days |
| 4 | **Pre-commit hooks** | Prevent secrets from leaving developer machines | 1 week |
| 5 | **Plugin architecture** | Extensibility, community contributions | 2-3 weeks |
| 6 | **AST-based analysis** | Superior accuracy, semantic understanding | 3-4 weeks |
| 7 | **Encoded secret detection** | Detects base64/hex/percent-encoded secrets | 1 week |
| 8 | **Archive scanning** | Scan compressed files, nested archives | 1-2 weeks |
| 9 | **Custom rules** | User-defined detection patterns | 1 week |
| 10 | **Audit mode** | Interactive labeling for FP reduction | 1 week |

---

## 7. Competitive Analysis Summary

### 7.1 Market Positioning

| Tool | Target Audience | Strengths | Weaknesses |
|------|----------------|-----------|------------|
| **TruffleHog** | Security teams, enterprises | Live verification, 800+ detectors, deep analysis | GPL license, complex setup |
| **Gitleaks** | Developers, DevOps | Fast, MIT license, easy config, baseline support | No verification, limited to regex |
| **GitGuardian** | Enterprises | Cloud API verification, enterprise dashboard | Requires account, commercial backing |
| **Semgrep** | Developers, security teams | AST analysis, cross-file, AI triage | Complex, Pro features paid |
| **detect-secrets** | Python shops, Yelp ecosystem | Plugin architecture, baseline, audit mode | Python-only, slower |
| **Coax** | Rust enthusiasts, performance seekers | Token efficiency, word filter, fast | Missing key features |

### 7.2 Coax Differentiators

**Current Advantages:**
- ✅ Token efficiency (unique)
- ✅ Word filter (unique)
- ✅ Rust performance
- ✅ Simple CLI

**Needed to Compete:**
- ❌ Live verification (critical)
- ❌ Baseline files (critical)
- ❌ Pre-commit hooks (critical)
- ❌ SARIF output (enterprise requirement)
- ❌ Plugin architecture (extensibility)

### 7.3 Strategic Recommendations

**Phase 3 Priority:**
1. **Live verification** - Match TruffleHog/GitGuardian capability
2. **Baseline files** - Match Gitleaks/detect-secrets capability
3. **Pre-commit hooks** - Table stakes for any scanner
4. **SARIF output** - Enterprise integration requirement
5. **Plugin architecture** - Long-term extensibility

**Phase 4 Consideration:**
- AST-based analysis (Semgrep-level sophistication)
- Cross-file data flow analysis
- AI-powered triage

**Long-term Vision:**
- Combine TruffleHog verification + Semgrep AST + detect-secrets plugins
- All in Rust for performance
- Unique token efficiency + word filter advantages

---

## 8. References

- TruffleHog: https://github.com/trufflesecurity/trufflehog
- Gitleaks: https://github.com/gitleaks/gitleaks
- GitGuardian: https://github.com/GitGuardian/ggshield
- Semgrep: https://github.com/returntocorp/semgrep
- detect-secrets: https://github.com/Yelp/detect-secrets

---

*Document created: 2026-03-15*
*Next: Phase 3 Proposal (docs/PHASE3-PROPOSAL.md)*
