# Coax Technical Deep-Dive: Where the Obvious Approach Goes Wrong

**Purpose:** This document covers areas where the straightforward implementation will cause problems that are painful to fix later. These are traps I've seen projects fall into — the kind of thing that works in testing but breaks in production, or works at small scale but collapses at large scale, or is subtly insecure in ways that erode user trust.

We trust Qwen Code will write good Rust. This document is about the decisions *around* the Rust.

---

## 1. Git History Scanning — The Minefield

Git history scanning looks simple: walk commits, diff each one, scan the diff. Three non-obvious problems will bite you.

---

### 1.1 The Merge Commit Problem

When you encounter a merge commit, it has multiple parents. The question is: what do you diff against?

**The naive approach:** Diff against all parents and union the results.

**Why it's wrong:** This double-counts. If a secret was introduced on a feature branch (commit A) and then merged to main (merge commit M), diffing M against both parents will "find" the secret again in the merge, even though it was already reported at commit A. Your scan results will have duplicates with different commit SHAs, confusing users.

**The correct approach:** For merge commits, diff against the **first parent only** (the branch being merged *into*). This is what `git log --first-parent` does, and it's what GitHub's PR diffs show. It means you see "what changed on main as a result of this merge" rather than re-scanning the entire feature branch.

```rust
// When walking history:
let parent = if commit.parent_count() > 1 {
    // Merge commit: diff against first parent only
    commit.parent(0)?
} else {
    commit.parent(0)?
};
let diff = repo.diff_tree_to_tree(
    Some(&parent.tree()?),
    Some(&commit.tree()?),
    None
)?;
```

**Exception:** You *should* also scan the individual commits on branches that were never merged. Use `revwalk` with `SORT_TOPOLOGICAL` and track which commits you've already visited.

---

### 1.2 The Shallow Clone Trap

In CI environments (GitHub Actions, GitLab CI, etc.), repos are almost always **shallow cloned** (`git clone --depth 1`). This means when someone eventually runs `coax scan --git-history` in CI, there's either one commit or a small window.

**What to do:**

- Detect shallow clones explicitly by checking for `.git/shallow` or using `repo.is_shallow()`
- Print a clear, actionable warning:

```
Warning: This appears to be a shallow clone (depth: 1).
Git history scanning requires full history. Run:
  git fetch --unshallow
Or use --commits 1 to scan only the latest commit.
```

- Don't silently scan one commit and report "no historical findings" — the user will think their history is clean when it isn't

---

### 1.3 The "Still Present" Computation

The spec mentioned a `still_present` field that tells users if a historical secret still exists at HEAD. The naive approach is: for each finding, grep the current tree for the same secret value.

**Why it's expensive:** If you find 500 secrets in history, you're doing 500 full-tree searches. On a large repo, that's slow.

**Better approach:** Invert the operation. Scan HEAD *first* and build a set of (secret_hash, file_path) tuples. Then, as you walk history, check each historical finding against this set. One HEAD scan, O(1) lookups per finding.

```rust
// Phase 1: Scan HEAD, collect all current secrets
let head_secrets: HashSet<SecretHash> = scan_tree(head_tree)
    .map(|finding| hash_secret_value(&finding.matched_text))
    .collect();

// Phase 2: Walk history, annotate each finding
for historical_finding in walk_history(repo) {
    let hash = hash_secret_value(&historical_finding.matched_text);
    historical_finding.still_present = head_secrets.contains(&hash);
}
```

---

### 1.4 The Memory Problem

Large repos have large blobs. A single commit might introduce a 50MB SQL dump or a vendored binary. If you load every blob into memory for scanning, you'll hit multi-GB memory usage on repos with large files in their history.

**Mitigation:**

- Skip binary files (check for null bytes in the first 8KB)
- Set a per-blob size limit (default 1MB, configurable via `--max-file-size`)
- Stream diffs rather than materializing full file content when possible — scan the diff hunks, not the complete new file. This is both faster and more memory-efficient
- For the `git2` crate specifically: use `diff.foreach()` with callbacks rather than collecting all deltas into a Vec

---

## 2. Credential Verification — The Security Landmines

This is the feature where a subtle mistake doesn't just cause a bug — it causes a **security incident** for your users. Treat the verification module as security-critical code.

---

### 2.1 Don't Let Secrets Leak Into Error Messages

This will happen naturally if you're not careful. Rust's `?` operator propagates errors, and HTTP client errors often include the request URL and headers. If the secret is in a header or URL parameter, it shows up in the error message, which shows up in logs.

```rust
// DANGEROUS: The error will contain the Authorization header value
let response = client
    .get("https://api.github.com/user")
    .header("Authorization", format!("token {}", secret))
    .send()
    .await?;  // If this fails, the error may contain the secret

// SAFE: Catch the error and redact before propagating
let response = client
    .get("https://api.github.com/user")
    .header("Authorization", format!("token {}", secret))
    .send()
    .await
    .map_err(|e| VerificationError::NetworkError {
        provider: "github",
        // Never include the original error directly — it may contain the secret
        message: "Failed to connect to api.github.com".into(),
    })?;
```

**Rule of thumb:** The verification module should have its own error type that *never* contains the secret value or raw HTTP error details. Log the sanitized error, not the original.

---

### 2.2 DNS as a Side Channel

When you verify a credential, you make an HTTP request to the provider. The DNS lookup for that request reveals *what kind of secret you found*, even to a passive network observer. If an attacker has compromised the network (which is plausible — you're a security tool), they learn "this repo has AWS keys" just from seeing a DNS query to `sts.amazonaws.com`.

**This is a real concern for enterprise users.** Mitigations:

- Document this clearly: "Verification makes network requests to provider APIs. These requests reveal which provider types were detected."
- Support `--verify-providers github,aws` to limit which providers are contacted
- In the future (daemon mode), consider DNS-over-HTTPS or pre-resolved addresses

---

### 2.3 Rate Limiting and Behavioral Detection

If you find 200 AWS keys in a repo's history and fire 200 `sts:GetCallerIdentity` calls from the same IP in 10 seconds, two things happen:

1. You get rate-limited and verification fails
2. AWS might flag this as credential stuffing and lock the keys or alert the account owner

**Mitigations:**

- Global rate limiter per provider (not just per verification call). Use a token bucket: e.g., max 5 requests/second to any single provider
- Deduplication before verification: if you find the same secret value in 50 different commits, verify it *once*
- Exponential backoff on 429/rate-limit responses
- Configurable concurrency (`--verify-concurrency`, default 3)

```rust
// Deduplicate secrets before verification
let unique_secrets: HashMap<SecretHash, Vec<Finding>> = findings
    .into_iter()
    .group_by(|f| hash_secret_value(&f.matched_text));

// Verify each unique secret once, apply result to all associated findings
for (hash, findings_group) in &unique_secrets {
    let result = rate_limiter
        .acquire(provider_name)
        .await
        .then(|| verifier.verify(&findings_group[0].matched_text))
        .await;
    
    for finding in findings_group {
        finding.verification = result.clone();
    }
}
```

---

### 2.4 The Verification-as-Attack-Vector Problem

Consider this scenario: An attacker plants a *fake* credential in a repo — a string that looks like an AWS key but is actually a URL to an attacker-controlled server formatted to match the AWS key regex. If your verifier naively makes a request using whatever it found, you might be making requests to arbitrary attacker-controlled infrastructure.

**This is unlikely with real provider verifiers** (you're calling `sts.amazonaws.com`, not an arbitrary URL), but it becomes a risk if you ever support custom verification endpoints (the `[rule.verify]` section in the custom rules spec).

**For custom verification endpoints:**

- Only allow HTTPS URLs
- Only allow URLs from an allowlist or the policy file (not from the scanned content itself)
- Set a strict timeout (5 seconds)
- Don't follow redirects
- Don't send the secret in query parameters (use headers only, so it's encrypted in transit)

---

## 3. The Source Provider Trait — Getting the Boundaries Right

This is the most important architectural decision in the near-term. Getting the trait right means every new source type is a few days of work. Getting it wrong means refactoring the core scanner for each new source.

---

### 3.1 Streaming vs. Buffering

The trait I sketched in the spec had `content: Vec<u8>` in the `ScanUnit`. This works for files and git blobs, but falls apart for large Docker layers or archives.

**Better approach:** Use a two-phase model:

```rust
trait SourceProvider {
    /// Enumerate what's available to scan (cheap, no content loading)
    fn enumerate(&self) -> Box<dyn Iterator<Item = ScanTarget>>;
}

struct ScanTarget {
    origin: ScanOrigin,
    size_hint: Option<u64>,  // For skip-if-too-large decisions
    content_type: Option<ContentType>,
}

/// The scan engine calls this to get content only for targets it decides to scan
trait ContentLoader {
    fn load(&self, target: &ScanTarget) -> Result<ScanContent>;
}

enum ScanContent {
    /// Small files: fully buffered
    Buffered(Vec<u8>),
    /// Large files: streamed in chunks
    Streamed(Box<dyn Read>),
}
```

**Why this matters:** Enumeration is cheap (just listing files/entries). Content loading is expensive. By separating them, the scan engine can make decisions (skip binary files, skip files above size limit, prioritize certain paths) *before* loading content. For a Docker image with 50,000 files across 30 layers, this avoids loading gigabytes of content you'll immediately skip.

---

### 3.2 Error Granularity

When scanning git history, a single corrupt blob shouldn't abort the entire scan. When scanning a Docker image, a single unreadable layer shouldn't prevent scanning other layers.

**Design the error model to be per-unit, not per-scan:**

```rust
enum ScanUnitResult {
    Scanned { findings: Vec<Finding> },
    Skipped { reason: SkipReason },  // Too large, binary, excluded by policy
    Failed { error: ScanError },      // Corrupt, unreadable, encoding issue
}
```

Collect and report failures at the end rather than propagating them. The summary should say: "Scanned 12,847 files, skipped 342 (binary), failed to read 3 (listed below)."

---

### 3.3 Progress Reporting

Different source types have very different progress characteristics. A filesystem scan can report progress by file count. A git history scan can report by commit count. A Docker image scan progresses by layer.

**Build progress reporting into the trait:**

```rust
trait SourceProvider {
    fn enumerate(&self) -> Box<dyn Iterator<Item = ScanTarget>>;
    
    /// Total expected units (for progress bar). None if unknown.
    fn total_units_hint(&self) -> Option<u64>;
    
    /// Human-readable description of current progress unit
    fn progress_unit_name(&self) -> &str;  // "files", "commits", "layers"
}
```

This matters for the TUI especially. Scanning a 50K-commit history with no progress indication is a bad user experience.

---

## 4. Unicode Detection — Protecting the Crown Jewel

This is Coax's strongest differentiator. These are the edge cases and decisions that keep it that way.

---

### 4.1 The Legitimate BiDi Problem

BiDi override characters (U+202A through U+202E, U+2066 through U+2069) are the core of Trojan Source attacks. But they're also *legitimately used* in:

- String literals containing Arabic or Hebrew text
- Comments in RTL languages
- Internationalization files
- Test fixtures for text rendering

**The crude approach:** Flag all BiDi overrides. This generates noise for i18n projects.

**The nuanced approach (what Coax should do):**

1. **Context-aware severity.** BiDi in a string literal inside an i18n file = low/info. BiDi in an identifier, control flow keyword, or comment that precedes code = critical.

2. **Scope analysis.** The Trojan Source attack works because BiDi overrides *span across syntactic boundaries* — a comment or string appears to end but the BiDi override makes the visual representation misleading. Check if BiDi push/pop pairs are balanced within their syntactic scope (string, comment, etc.). Unbalanced pairs crossing scope boundaries are almost always attacks.

3. **The policy file integration.** The `bidi_allowed_paths` config from the policy spec is essential. Let teams whitelist their i18n directories. But also consider:

```toml
[unicode.bidi]
# Allow BiDi only inside string literals (not comments, identifiers, or code)
allow_in_strings = true
allow_in_comments = false
allow_in_identifiers = false
```

This requires basic syntax awareness — knowing whether a BiDi character is inside a string, comment, or code context. You don't need a full parser; a state machine that tracks quote/comment nesting is sufficient for most languages.

---

### 4.2 Homoglyph Detection at Scale

Homoglyph detection (e.g., `а` Cyrillic vs `a` Latin) requires comparing characters against a confusables database. Unicode publishes this as `confusables.txt` (~4,000 entries).

**Performance concern:** Checking every character in every identifier against a 4,000-entry lookup table is slow at scale.

**Approach:**

- Precompute a `HashMap<char, char>` that maps each confusable to its canonical form (the "skeleton" in Unicode parlance)
- To check if two identifiers are confusable, compute both their skeletons and compare. If skeletons match but original strings differ, flag it
- **The real question is: confusable with *what*?** Just flagging "this character has a confusable" is useless — Cyrillic `а` is only a problem if there's also a Latin `a` identifier in scope. The high-value detection is: "this file uses `аdmin` (Cyrillic а) and there's an `admin` (Latin a) identifier elsewhere in the codebase." That requires cross-file analysis, which is a heavier operation

**Recommendation for now:** Flag mixed-script identifiers (an identifier mixing Latin and Cyrillic characters is almost always suspicious, unlike a fully-Cyrillic identifier which is just a Russian developer). This catches the real attacks without needing cross-file analysis.

---

### 4.3 Keep Up With Unicode Versions

Unicode releases a new version roughly annually. Each release can introduce new characters that could be abused. Your confusables database, script data, and category data need updating.

**Recommendation:** Don't embed a static Unicode database. Use the `unicode-security` and `icu4x` crates, which track Unicode releases. Pin to a specific version and update deliberately (each update is a potential source of new FPs). Document which Unicode version Coax supports.

---

## 5. Benchmark Suite — How to Not Deceive Yourself

---

### 5.1 The Goodhart's Law Risk

"When a measure becomes a target, it ceases to be a good measure." If you optimize Coax to score well on your own benchmark, the benchmark stops reflecting real-world performance.

**Mitigation:**

- Include a **held-out test set** that you never look at during development. Seal it. Run it only for releases. If your score improves on the development set but not the held-out set, you're overfitting.
- Include real-world samples, not just synthetic ones. Take snippets from real leaked credentials (sanitized) via sources like `shhgit` archives or public breach notifications. Synthetic patterns like `AKIA1234567890EXAMPLE` don't test the same edge cases as real secrets embedded in real code.
- Include **adversarial negatives**: strings designed to look like secrets but aren't. Password validation regex patterns, documentation examples with placeholder keys, Base64-encoded non-secret data. These test your false positive rate, which matters as much as detection rate.

---

### 5.2 What to Do When You Lose

You will lose to TruffleHog on some secret categories. The benchmark will show this. That's fine — here's how to present it:

**Don't hide it. Contextualize it.**

Structure the results as multiple dimensions, not a single score:

```
| Category              | Coax  | Gitleaks | TruffleHog | detect-secrets |
|----------------------|-------|----------|------------|----------------|
| Secrets (precision)   | 0.91  | 0.88     | 0.94       | 0.85           |
| Secrets (recall)      | 0.87  | 0.82     | 0.96       | 0.79           |
| Encoded secrets       | 0.93  | 0.41     | 0.72       | 0.38           |
| Unicode attacks       | 0.98  | —        | —          | —              |
| Invisible characters  | 0.97  | —        | —          | —              |
| Homoglyphs            | 0.95  | —        | —          | —              |
| Trojan Source          | 0.99  | —        | 0.12*      | —              |
| Scan speed (files/s)  | 250K  | 180K     | 95K        | 12K            |
```

The "—" cells are your positioning. No competitor even has a number. That's more powerful than winning every category by 2%.

---

## 6. Daemon Architecture — Decisions That Are Hard to Change Later

---

### 6.1 SQLite WAL Mode

If you use SQLite for the daemon's data store (which I recommend — it keeps the single-binary story intact), enable WAL (Write-Ahead Logging) mode from the start:

```rust
conn.execute_batch("PRAGMA journal_mode=WAL;")?;
conn.execute_batch("PRAGMA synchronous=NORMAL;")?;
```

**Why:** Without WAL, SQLite locks the entire database during writes. If a scan is writing results while the dashboard is querying findings, one blocks the other. WAL mode allows concurrent readers and a single writer, which is exactly the daemon's access pattern.

**Set this on first database creation.** Changing journal mode on an existing database with data is possible but annoying.

---

### 6.2 Never Store Secrets, Not Even Hashes

The spec mentioned storing `secret_hash` for deduplication. Be very careful here.

**The problem:** If you store SHA-256 hashes of secrets, and an attacker gets access to the Coax database, they can't directly read the secrets — but for short secrets (8-character passwords, short API keys), they can brute-force the hash. A rainbow table for all 8-character alphanumeric strings is trivially computed.

**Better approach:**

- Use HMAC-SHA-256 with a per-installation random key (generated on first run, stored in `~/.coax/hmac-key`)
- This means the hashes are installation-specific and can't be brute-forced without the key
- For deduplication, this works identically — same secret produces same HMAC within one installation
- Document that the HMAC key file is sensitive and should be protected

```rust
use hmac::{Hmac, Mac};
use sha2::Sha256;

fn hash_secret_for_storage(secret: &str, hmac_key: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(hmac_key)
        .expect("HMAC key can be any length");
    mac.update(secret.as_bytes());
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}
```

---

### 6.3 File Watching Limits

When the daemon watches directories for changes, you'll likely use `inotify` (Linux), `FSEvents` (macOS), or `ReadDirectoryChangesW` (Windows) via the `notify` crate.

**The trap:** Linux has a default `inotify` watch limit of **8,192** per user. A monorepo with 50,000 directories will exhaust this. The daemon will silently stop receiving events for some directories, and scans will miss changes.

**Mitigations:**

- Detect and warn when approaching the limit
- Use `notify` in **polling mode** as a fallback (slower but no limit)
- Document how to increase the limit: `echo 65536 | sudo tee /proc/sys/fs/inotify/max_user_watches`
- Consider watching only the top-level directories and using polling for subdirectories, or watching only `.git/` for commit events rather than the entire worktree

---

