# 📋 Coax Feature Specification & Roadmap

**Document Version:** 1.0
**Date:** March 16, 2026
**Product:** Coax — Code Trust & Supply-Chain Attack Scanner
**Mission:** *"Is this code what it appears to be? Are there hidden secrets or invisible attacks?"*

---

## 🎯 Current State Summary

| Capability | Status | Notes |
|---|---|---|
| Unicode/Trojan Source detection | ✅ Shipping | Best-in-class depth |
| Secrets detection (800+ patterns) | ✅ Shipping | Via secrets-patterns-db |
| Shannon entropy detection | ✅ Shipping | FP issues reportedly resolved |
| Encoded secret detection (B64/hex/URL) | ✅ Shipping | Differentiator |
| CLI + TUI | ✅ Shipping | |
| VS Code extension | ✅ Shipping | Inline diagnostics + quick-fixes |
| SARIF output | ✅ Shipping | |
| Git history scanning | ❌ Not started | Next priority |
| Credential verification | ❌ Not started | High-value feature |
| GitHub Action / CI publishing | ⏸️ Deferred | Pending binary signing |
| Web dashboard | ❌ Not started | Requires architectural rethinking |
| AI/LLM triage | ⏸️ Deferred | Needs hosting solution |

---

## 📐 Architecture Principle: The Source Provider Abstraction

Before detailing features, one architectural decision should inform everything that follows. **Git-history scanning and non-git source scanning should share a common framework.** Here's the thinking:

Your scan engine currently does:

```
Files on disk → Scan engine → Findings
```

After git-history, you want:

```
Git commits/diffs → Scan engine → Findings
```

For non-git sources later:

```
npm tarball / Docker layer / S3 object / zip → Scan engine → Findings
```

**Recommendation:** Introduce a **Source Provider** trait early, during the git-history work:

```rust
/// A source provider yields scannable units (files, diffs, blobs)
trait SourceProvider {
    /// Iterate over scannable content units
    fn scan_units(&self) -> Box<dyn Iterator<Item = ScanUnit>>;
    
    /// Metadata about the source (for reporting context)
    fn metadata(&self) -> SourceMetadata;
}

struct ScanUnit {
    /// The content to scan
    content: Vec<u8>,
    /// Where it came from (path, commit, layer, etc.)
    origin: ScanOrigin,
    /// Content type hint (for parser selection)
    content_type: Option<ContentType>,
}

enum ScanOrigin {
    FileSystem { path: PathBuf },
    GitBlob { commit: String, path: PathBuf, author: String, date: DateTime },
    GitDiff { commit: String, path: PathBuf, diff_side: DiffSide },
    Archive { archive_path: PathBuf, entry_path: String },
    DockerLayer { image: String, layer_hash: String, path: String },
    // Future: NpmPackage, S3Object, etc.
}
```

This way, the core scan engine never changes — you just add new `SourceProvider` implementations. The git-history provider becomes your first non-filesystem provider, and every subsequent source type is just another implementation of the same trait.

**To answer your question directly:** Non-git source scanning after git-history should be **moderate effort** (days per source type, not weeks) *if* you build the abstraction right during the git-history phase. They share the same scan engine; they differ only in how content is enumerated and how results are contextualized.

---

## 🗺️ Phased Roadmap

### **Phase 1: Foundation & Git History** (Weeks 1–6)

These features strengthen the core scanner and establish the architectural patterns everything else builds on.

---

#### 1.1 Git History Scanning

**Priority:** 🔴 Critical
**Effort estimate:** 2–3 weeks
**Why:** Every competitor has this. Without it, Coax can only find *current* secrets, not the ones that were committed and then "deleted" (still in history). This is where the majority of real-world secret leaks live.

**Specification:**

**Core behavior:**
- Scan all commits in a repository's history for secrets and Unicode attacks
- For each commit, scan the **diff** (not the full tree) to identify what was *introduced* in that commit
- Report findings with full commit context: SHA, author, date, commit message, file path at that point in history

**CLI interface:**

```bash
# Scan full history of current repo
coax scan --git-history

# Scan last N commits
coax scan --git-history --commits 50

# Scan a specific range
coax scan --git-history --since 2025-01-01
coax scan --git-history --range main..feature-branch

# Scan only specific branches
coax scan --git-history --branch main

# Combine with existing flags
coax scan --git-history --format sarif --output results.sarif
```

**Implementation guidance:**

- Use the `git2` crate (libgit2 bindings) rather than shelling out to `git` CLI. It's faster, doesn't require git to be installed, and gives structured access to the object database
- Walk the commit graph with `revwalk`, compute diffs between each commit and its parent(s) using `Repo::diff_tree_to_tree`
- For merge commits, diff against the first parent (the branch being merged into) — this is what GitHub and most tools do
- Extract the `+` side of each diff hunk — that's what was introduced
- **Performance consideration:** Large repos (Linux kernel: 1M+ commits) need incremental scanning. Add a `--commits` limit and consider a `--incremental` flag that stores a watermark (last scanned commit SHA) in `.coax/history-state.json`

**Output enrichment for git-history findings:**

```json
{
  "finding_id": "...",
  "rule_id": "aws-access-key",
  "severity": "critical",
  "source": {
    "type": "git_history",
    "commit": "a1b2c3d",
    "author": "dev@example.com",
    "date": "2025-03-10T14:22:00Z",
    "message": "add config for staging",
    "file_path": "config/staging.yml",
    "line": 12,
    "still_present": false
  },
  "secret_preview": "AKIA...REDACTED"
}
```

The `still_present` field is important — it tells the user whether the secret is still in the current HEAD or was removed in a later commit. Compute this by checking if the same secret (by value, not location) appears in the HEAD version of any file.

**Testing:**
- Create a test repo with known secrets committed and then removed in subsequent commits
- Verify that scanning finds secrets in old commits even after deletion
- Benchmark: target <30 seconds for a repo with 10K commits

---

#### 1.2 Source Provider Abstraction

**Priority:** 🔴 Critical (do alongside 1.1)
**Effort estimate:** 1 week (refactor during git-history implementation)
**Why:** Architectural investment that makes Phase 2 and 3 features dramatically easier.

**Specification:**

- Refactor the existing filesystem scanner into a `FileSystemProvider` implementing the `SourceProvider` trait
- Implement `GitHistoryProvider` as the second provider
- The scan engine accepts any `SourceProvider` and produces findings with origin-aware context
- SARIF output adapter maps `ScanOrigin` variants to appropriate SARIF location types

---

#### 1.3 Entropy Detection Hardening

**Priority:** 🟡 Medium
**Effort estimate:** 1 week
**Why:** You've reported FP issues were resolved, but entropy detection is tricky. This spec defines a hardening pass to make sure it's robust before the benchmark suite goes public.

**Specification:**

**Validation checklist:**
- Confirm Shannon entropy threshold is tunable (recommended default: 4.5 for hex strings, 4.0 for base64 strings, adjustable via config)
- Confirm minimum string length threshold exists (recommended: 20+ characters) to avoid flagging short high-entropy strings like CSS colors or UUIDs
- Confirm the following are excluded from entropy scanning or have allowlists:
  - UUIDs (`[0-9a-f]{8}-[0-9a-f]{4}-...`)
  - CSS hex colors
  - Hashes in lock files (`package-lock.json`, `yarn.lock`, `Cargo.lock`, `go.sum`)
  - SRI hashes in HTML (`integrity="sha384-..."`)
  - Git commit SHAs in known contexts
  - Test fixtures and mock data directories
  - Binary file extensions
  - Minified JS/CSS files (high entropy by nature)
- Add a `--entropy-threshold` CLI flag for user tuning
- Confirm entropy scanner interacts correctly with the BPE/tokenizer filter (if still present) — the filter should suppress entropy findings on content that looks like natural language or code identifiers

**Test suite for entropy:**
- True positives: leaked AWS keys, random API tokens, base64-encoded passwords
- True negatives: lock file hashes, minified JS, UUIDs, base64-encoded images, long CSS class names, i18n strings

---

#### 1.4 Public Benchmark Suite

**Priority:** 🟡 Medium
**Effort estimate:** 1–2 weeks
**Why:** As you noted, this creates a QA feedback loop and becomes a marketing asset as Coax improves.

**Specification:**

**Repository structure:**
```
coax-benchmarks/
├── README.md                    # Methodology, how to run, results table
├── run-benchmarks.sh           # Script to run all tools
├── datasets/
│   ├── secrets/
│   │   ├── true-positives/     # Files with known secrets (labeled)
│   │   ├── true-negatives/     # Files that look suspicious but aren't
│   │   └── encoded/            # Base64, hex, URL-encoded secrets
│   ├── unicode/
│   │   ├── bidi-attacks/       # BiDi override/reorder attacks
│   │   ├── homoglyphs/         # Confusable character substitutions
│   │   ├── invisible-chars/    # Zero-width joiners, variation selectors
│   │   ├── script-mixing/      # Mixed script identifiers
│   │   ├── trojan-source/      # Classic Trojan Source patterns
│   │   └── safe-i18n/          # Legitimate multilingual code (should NOT flag)
│   └── git-history/            # Repo with secrets in old commits
├── tools/
│   ├── coax.sh
│   ├── gitleaks.sh
│   ├── trufflehog.sh
│   └── detect-secrets.sh
└── results/
    ├── latest.md               # Auto-generated comparison table
    └── history/                # Historical results for trend tracking
```

**Metrics to report:**

| Metric | Definition |
|---|---|
| True Positive Rate (Recall) | Correctly identified issues / Total real issues |
| False Positive Rate | False alarms / Total clean samples |
| Precision | True positives / (True positives + False positives) |
| F1 Score | Harmonic mean of precision and recall |
| Scan Speed | Time to scan the full dataset |
| Unicode Coverage | Number of distinct Unicode attack categories detected |

**Key principle:** Be honest. If Coax loses on secrets recall against TruffleHog initially, that's fine — show the Unicode categories where no competitor even has a score. The benchmark should feel credible, not cherry-picked.

---

### **Phase 2: Verification & Intelligence** (Weeks 7–14)

These features transform Coax from "detector" to "actionable intelligence tool."

---

#### 2.1 Credential Verification

**Priority:** 🔴 Critical
**Effort estimate:** 3–4 weeks for first 10 providers, then ~1–2 days per additional provider
**Why:** This is TruffleHog's killer feature. Without verification, every finding requires manual investigation. With it, you can tell the user "this AWS key is **live and has S3 access**" — that changes the urgency calculus entirely.

**How difficult is this? (Answering your question directly):**

It's **moderate complexity, high surface area.** Each individual verifier is simple (usually 1 API call), but the system around it needs care:

- **The easy part:** Most cloud provider APIs let you make a harmless authenticated call (e.g., AWS `sts:GetCallerIdentity`, GitHub `/user`, Slack `/auth.test`). If it returns 200, the credential is live.
- **The tricky parts:**
  - Rate limiting: You might find 500 AWS keys in a history scan. You need backoff and concurrency control.
  - Network policy: Some users will be scanning in air-gapped environments. Verification must be opt-in.
  - False verification: Some APIs return 200 for invalid keys (rare but happens). Need to validate the response body.
  - Security: You're sending potentially-live secrets over the network. Must use HTTPS, should warn users, and should never log the full secret.
  - Scope detection: Knowing a key is live is good; knowing what it can access is better (but much harder — Phase 3 territory).

**Specification:**

**CLI interface:**

```bash
# Scan with verification enabled (opt-in, never default)
coax scan --verify

# Scan git history with verification
coax scan --git-history --verify

# Verify only (skip detection, verify previously found secrets)
coax verify --input previous-results.sarif

# Control concurrency
coax scan --verify --verify-concurrency 5

# Timeout per verification attempt
coax scan --verify --verify-timeout 10s
```

**Architecture:**

```rust
/// Result of attempting to verify a credential
enum VerificationResult {
    /// Credential is confirmed active
    Active { 
        identity: Option<String>,  // e.g., AWS account ID, GitHub username
        scopes: Vec<String>,       // e.g., ["repo", "read:org"] for GitHub
        verified_at: DateTime<Utc>,
    },
    /// Credential is confirmed inactive/revoked
    Inactive,
    /// Verification was attempted but inconclusive
    Inconclusive { reason: String },
    /// Verification was not attempted
    NotAttempted { reason: String },  // e.g., "no verifier for this secret type"
}

trait SecretVerifier: Send + Sync {
    /// Which rule IDs this verifier handles
    fn handles(&self) -> &[&str];
    
    /// Attempt to verify the secret. Must be safe (read-only API calls only).
    async fn verify(&self, secret: &str) -> Result<VerificationResult>;
}
```

**Provider priority (order of implementation):**

| Priority | Provider | Verification Method | Difficulty |
|---|---|---|---|
| 1 | AWS Access Keys | `sts:GetCallerIdentity` | Easy |
| 2 | GitHub Tokens | `GET /user` + `GET /rate_limit` for scopes | Easy |
| 3 | GitLab Tokens | `GET /api/v4/user` | Easy |
| 4 | Slack Tokens | `POST /auth.test` | Easy |
| 5 | Google Cloud API Keys | `GET /v1/apikeys/{key}:lookupKey` or test against an API | Medium |
| 6 | Stripe Keys | `GET /v1/charges?limit=1` | Easy |
| 7 | SendGrid Keys | `GET /v3/scopes` | Easy |
| 8 | Twilio Keys | `GET /2010-04-01/Accounts` | Easy |
| 9 | npm Tokens | `GET /-/whoami` | Easy |
| 10 | Azure AD / Service Principal | Token exchange against `login.microsoftonline.com` | Medium |

**Most verifiers are 50–100 lines of Rust.** The real work is:
1. Building the verifier framework and concurrency manager (~1 week)
2. Implementing the first 3–4 verifiers to validate the framework (~1 week)
3. Adding the remaining 6–7 from the list (~1 week)
4. Testing, edge cases, timeouts, rate limiting (~1 week)

**Output enrichment:**

```json
{
  "rule_id": "aws-access-key",
  "secret_preview": "AKIA...REDACTED",
  "verification": {
    "status": "active",
    "identity": "arn:aws:iam::123456789012:user/deploy-bot",
    "scopes": ["s3:*", "ec2:Describe*"],
    "verified_at": "2026-03-16T14:00:00Z"
  }
}
```

**Safety requirements:**
- Verification is **always opt-in** (`--verify` flag). Never verify by default.
- All verification calls must be **read-only** — never write, modify, or delete anything
- Log a clear warning when verification is enabled: "Coax will make network requests using discovered credentials to check if they are active."
- Never log, cache, or persist the full secret value — only the redacted preview
- Support `--verify-proxy` for users who need to route through a corporate proxy
- Respect `NO_PROXY` and `HTTPS_PROXY` environment variables

---

#### 2.2 Policy Engine (Basic)

**Priority:** 🟡 Medium
**Effort estimate:** 2 weeks
**Why:** Allows teams to customize Coax behavior without forking. Enables "secrets policy as code" workflows.

**Specification:**

**Configuration file:** `.coax/policy.toml`

```toml
[policy]
# Fail the scan (exit code 1) if any finding matches these criteria
fail_on = "critical"  # "critical", "high", "medium", "low", "any"

# Treat verified-active secrets as critical regardless of rule severity
escalate_verified = true

[rules]
# Disable specific rules
disable = ["generic-password", "generic-api-key"]

# Enable only specific rule categories
# enable_only = ["aws", "gcp", "azure", "github"]

# Override severity for specific rules
[rules.severity_overrides]
"slack-webhook" = "critical"    # Your org considers these critical
"generic-password" = "low"      # Too many FPs in your codebase

[ignore]
# Paths to ignore (glob patterns)
paths = [
    "vendor/**",
    "node_modules/**",
    "**/*.test.js",
    "**/*_test.go",
    "docs/examples/**",
]

# Ignore specific findings by hash (for acknowledged false positives)
# Generate with: coax acknowledge <finding-id>
finding_hashes = [
    "sha256:abc123...",  # FP in config/example.yml - acknowledged 2026-01-15
]

[unicode]
# Control Unicode detection strictness
strictness = "standard"  # "strict", "standard", "relaxed"

# Allow specific script combinations (for i18n projects)
allowed_script_pairs = [
    ["Latin", "Han"],        # Chinese variable names OK
    ["Latin", "Hiragana"],   # Japanese OK
]

# Allow BiDi characters in specific file patterns (e.g., RTL language files)
bidi_allowed_paths = [
    "locales/ar/**",
    "locales/he/**",
]
```

**CLI integration:**

```bash
# Use policy file
coax scan --policy .coax/policy.toml

# Override fail threshold
coax scan --policy .coax/policy.toml --fail-on high

# Acknowledge a false positive (adds to policy file)
coax acknowledge <finding-id> --reason "test fixture, not a real key"

# Validate policy file
coax policy validate .coax/policy.toml
```

---

#### 2.3 Custom Rules (Lightweight)

**Priority:** 🟡 Medium
**Effort estimate:** 1–2 weeks
**Why:** Organizations have internal secrets formats (internal API keys, custom tokens) that no pattern database will cover. This is table-stakes for enterprise adoption.

**Specification:**

**Custom rules file:** `.coax/rules.toml`

```toml
[[rule]]
id = "internal-api-key"
description = "Internal platform API key"
severity = "critical"
# Regex pattern (Rust regex syntax)
pattern = '''(?i)x-platform-key["\s:=]+["']?([a-z0-9]{40})["']?'''
# Optional: keywords that must appear nearby (performance optimization)
keywords = ["x-platform-key", "X-PLATFORM-KEY"]
# Optional: file patterns to limit scope
file_patterns = ["*.yml", "*.yaml", "*.json", "*.env"]
# Optional: entropy threshold for the captured group
entropy_min = 3.5

[[rule]]
id = "internal-db-password"
description = "Production database connection string"
severity = "critical"
pattern = '''postgres://[^:]+:([^@]+)@prod-db\.internal\.company\.com'''
keywords = ["prod-db.internal.company.com"]

[[rule]]
id = "deploy-token"
description = "Internal deployment token"
severity = "high"
pattern = '''DEPLOY_[A-Z]+_TOKEN=["']?([A-Za-z0-9+/]{64,})["']?'''
```

**Implementation notes:**
- Compile custom rule patterns at startup using the `regex` crate
- Validate patterns during `coax policy validate`
- Custom rules should participate in the same verification framework — allow a `verifier` field that specifies a URL to call:

```toml
[[rule]]
id = "internal-api-key"
pattern = '''...'''
# Optional: custom verification endpoint
[rule.verify]
method = "GET"
url = "https://auth.internal.company.com/verify"
header = "Authorization: Bearer {secret}"
expect_status = 200
```

---

### **Phase 3: Ecosystem Expansion** (Weeks 15–24)

---

#### 3.1 Non-Git Source Scanning

**Priority:** 🟡 Medium
**Effort estimate:** 1–2 weeks per source type (thanks to the Source Provider abstraction)
**Why:** Supply-chain attacks happen in packages, containers, and artifacts — not just git repos.

**Source providers to implement (in order):**

| Order | Source Type | Effort | Notes |
|---|---|---|---|
| 1 | Archive files (zip, tar.gz) | 3 days | Use `zip` and `flate2` crates. Recursive: archives within archives. |
| 2 | npm packages (tarballs) | 3 days | `npm pack` produces tarballs. Also scan from registry URLs. |
| 3 | Docker images (layers) | 1 week | Use `oci-distribution` or shell out to `skopeo`. Each layer is a tar. Scan layer diffs. |
| 4 | PyPI packages | 3 days | `.whl` files are zips. `.tar.gz` for sdists. |
| 5 | Crates.io packages | 2 days | You're Rust-native, this is on-brand. Tarballs from the registry. |

**CLI interface:**

```bash
# Scan an archive
coax scan --archive package.tar.gz

# Scan a Docker image
coax scan --docker-image nginx:latest
coax scan --docker-image ghcr.io/org/app:v2.1.0

# Scan an npm package
coax scan --npm-package lodash@4.17.21

# Scan from a URL
coax scan --url https://registry.npmjs.org/pkg/-/pkg-1.0.0.tgz
```

**This is where your positioning shines:** Scanning a Docker image for homoglyph attacks or hidden BiDi overrides in vendored dependencies is something **no other tool does.** This is the supply-chain trust story made concrete.

---

#### 3.2 JetBrains Plugin

**Priority:** 🟢 Low (but high impact for adoption)
**Effort estimate:** 3–4 weeks
**Why:** ~30% of professional developers use JetBrains IDEs. The VS Code extension is a differentiator — bringing it to IntelliJ/PyCharm/WebStorm doubles the addressable market.

**Specification:**
- Implement as an IntelliJ Platform Plugin (Kotlin/Java) that wraps the Coax CLI binary
- Ship the Coax binary bundled in the plugin, or download on first run
- Features should match VS Code extension: inline diagnostics, quick-fix suggestions, gutter icons for findings
- Use the External Annotator API for non-blocking background scanning
- Support the same `.coax/policy.toml` configuration

---

#### 3.3 Daemon Mode (Prerequisite for Dashboard)

**Priority:** 🟡 Medium
**Effort estimate:** 2–3 weeks
**Why:** You correctly identified that a web dashboard requires rethinking from CLI to daemon. This spec defines that architecture.

**Specification:**

```bash
# Start daemon
coax daemon start --port 7070 --data-dir ~/.coax

# Stop daemon
coax daemon stop

# Check status
coax daemon status
```

**Daemon responsibilities:**
- REST API for triggering scans, querying results, managing policies
- Persistent storage of scan results (SQLite via `rusqlite` — keeps it single-binary)
- Scheduled/watched scanning (file watcher for continuous monitoring)
- Webhook notifications (Slack, Teams, email) on new findings

**API sketch:**

```
POST   /api/v1/scans              # Trigger a new scan
GET    /api/v1/scans              # List past scans
GET    /api/v1/scans/:id          # Get scan results
GET    /api/v1/findings           # Query findings across scans
PATCH  /api/v1/findings/:id       # Update finding status (triage)
GET    /api/v1/stats              # Dashboard summary stats
POST   /api/v1/policies           # Upload/update policy
GET    /api/v1/health             # Health check
```

**Data model (SQLite):**

```sql
CREATE TABLE scans (
    id TEXT PRIMARY KEY,
    source_type TEXT,  -- 'filesystem', 'git_history', 'docker_image', etc.
    source_path TEXT,
    started_at TIMESTAMP,
    completed_at TIMESTAMP,
    status TEXT,       -- 'running', 'completed', 'failed'
    total_findings INTEGER,
    critical_count INTEGER,
    high_count INTEGER
);

CREATE TABLE findings (
    id TEXT PRIMARY KEY,
    scan_id TEXT REFERENCES scans(id),
    rule_id TEXT,
    severity TEXT,
    file_path TEXT,
    line_number INTEGER,
    secret_hash TEXT,  -- For deduplication; NEVER store the actual secret
    verification_status TEXT,
    triage_status TEXT DEFAULT 'open',  -- 'open', 'acknowledged', 'fixed', 'false_positive'
    first_seen_at TIMESTAMP,
    last_seen_at TIMESTAMP
);
```

---

#### 3.4 Web Dashboard

**Priority:** 🟢 Low (depends on 3.3)
**Effort estimate:** 4–6 weeks
**Why:** Enterprise teams need a shared view of security findings. This is the path to commercial viability.

**Specification (brief — design in detail when closer):**
- Single-page app (consider Leptos for Rust WASM, or pragmatically use React/Vue)
- Connects to daemon REST API
- Views: Dashboard overview, Findings list with filters, Scan history, Finding detail with code context, Policy editor
- Finding triage workflow: open → investigating → false positive / acknowledged / fixed
- Trend charts: findings over time, mean-time-to-remediation
- Team features: assign findings, comment threads

---

### **Phase 4: Intelligence & AI** (Weeks 25+)

---

#### 4.1 AI-Powered Triage (When Hosting is Resolved)

**Priority:** 🟢 Low (blocked on infrastructure)
**Effort estimate:** 3–4 weeks once hosting is available

**Options for LLM hosting (to explore):**

| Option | Pros | Cons |
|---|---|---|
| NVIDIA NIM | Optimized for GPU inference, self-hostable | Requires NVIDIA GPU infrastructure |
| Ollama | Dead simple local hosting, many model options | Performance varies, needs decent hardware |
| vLLM on cloud GPU | High throughput, OpenAI-compatible API | Cost, operational overhead |
| Hugging Face Inference Endpoints | Managed, easy scaling | Cost, vendor dependency |
| Local GGUF models via `llama.cpp` | No external dependency, runs on CPU | Slower, quality tradeoffs |

**Recommended approach:** Design the AI triage interface as a pluggable backend with an OpenAI-compatible API contract. This way users can point it at any hosted model (including future NVIDIA-hosted options). Start with a local Ollama integration for development/testing.

**Use cases for AI triage:**
1. **FP reduction:** "Is this high-entropy string a real secret or a hash/UUID/test value?" — an LLM with code context can make this judgment well
2. **Finding prioritization:** "This AWS key is in a Terraform module that provisions production infrastructure" vs. "This key is in a commented-out example"
3. **Remediation suggestions:** "This secret should be moved to AWS Secrets Manager. Here's how for your framework..."
4. **Unicode attack explanation:** "This BiDi override makes the code appear to check `isAdmin` but actually checks `isUser`" — generate human-readable explanations of what the attack does

---

## 🏗️ Implementation Priority Matrix

| # | Feature | Phase | Effort | Impact | Dependencies |
|---|---|---|---|---|---|
| 1 | Git history scanning | 1 | 2–3 wks | 🔴 Critical | None |
| 2 | Source Provider abstraction | 1 | 1 wk | 🔴 Critical | Build during #1 |
| 3 | Entropy hardening & test suite | 1 | 1 wk | 🟡 Medium | None |
| 4 | Public benchmark suite | 1 | 1–2 wks | 🟡 Medium | None |
| 5 | Credential verification | 2 | 3–4 wks | 🔴 Critical | None |
| 6 | Policy engine | 2 | 2 wks | 🟡 Medium | None |
| 7 | Custom rules | 2 | 1–2 wks | 🟡 Medium | None |
| 8 | Archive/package scanning | 3 | 1–2 wks | 🟡 Medium | #2 |
| 9 | Docker image scanning | 3 | 1 wk | 🟡 Medium | #2 |
| 10 | JetBrains plugin | 3 | 3–4 wks | 🟢 Nice-to-have | None |
| 11 | Daemon mode | 3 | 2–3 wks | 🟡 Medium | None |
| 12 | Web dashboard | 3 | 4–6 wks | 🟢 Nice-to-have | #11 |
| 13 | AI triage | 4 | 3–4 wks | 🟢 Nice-to-have | Hosting resolved |

---

## 📌 Deferred Items (Tracked)

| Item | Reason | Revisit When |
|---|---|---|
| GitHub Action / CI publishing | Pending binary signing for VS Code + GitHub | Binary signing infrastructure is in place |
| "AI-Powered" branding | LLM removed, no substantive AI features currently | Phase 4 AI triage ships |
| Scope detection for verified creds | Complex, provider-specific | After basic verification is proven (Phase 2+) |

---

## 🧭 Strategic Guardrails

These are the "always true" principles that should guide every feature decision:

1. **Stay in the trust lane.** Every feature should answer: "Is this code what it appears to be?" If a feature doesn't serve that question, it's out of scope.

2. **Detection → Verification → Intelligence.** This is the maturity ladder. Phase 1 improves detection, Phase 2 adds verification, Phase 3 expands surface area, Phase 4 adds intelligence. Don't skip ahead.

3. **Opt-in over opt-out.** Any feature that makes network requests (verification), accesses sensitive data (git history), or changes behavior (policy) should be explicitly opted into.

4. **Single binary stays single binary.** Coax's deployment simplicity (one Rust binary) is a feature. The daemon mode and dashboard are the first place this gets challenged — consider embedding the web UI as static assets compiled into the binary (Rust's `include_dir!` or `rust-embed`).

5. **Benchmark everything.** Every new detection capability should be added to the benchmark suite simultaneously. If you can't write a test case for it, you can't prove it works.

---

**Next step:** Begin Phase 1 with git history scanning + the Source Provider refactor in parallel. 