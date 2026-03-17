

---

**Context:** Coax secrets detection is at F1=0.96, beating Gitleaks (0.91). 185 tests passing. 0 real-world FPs. We're cleaning up the codebase and pushing to remote.

**Scope:** Cleanup and documentation of current state ONLY. No new features, no Phase 2 work.

---

### PHASE 1: Document Remaining Gaps

Before changing any code:

**1A.** Identify the **1 remaining FN**: What file? What secret type? Why missed?

**1B.** Identify the **4 remaining FPs**: What files? What triggers each? Why is each wrong?

**1C.** Report Unicode detection results after the underscore fix:

```
| Metric    | Before | After |
|-----------|--------|-------|
| TP        | 15     | ?     |
| FN        | 0      | ?     |
| FP        | 4      | ?     |
| TN        | 1      | ?     |
| F1        | 0.88   | ?     |
```

Print all gap findings — they go into HANDOFF.md.

---

### PHASE 2: Code Cleanup

**2A. Dead code:** Remove commented-out code, unused `use` imports, debug `println!`/`dbg!` statements, unnecessary `#[allow(dead_code)]`.

**2B. Pattern organization in `secrets.rs`:** Group patterns with section comments:
```rust
// ═══════════════════════════════════════════
// Cloud Providers (AWS, Azure, GCP)
// ═══════════════════════════════════════════

// ═══════════════════════════════════════════
// Version Control & CI/CD (GitHub, GitLab, etc.)
// ═══════════════════════════════════════════

// (Payment, Communication, Database, Generic — same style)
```

**2C. Add architecture comment** at the top of the scan pipeline (scanner.rs or equivalent):

```rust
// SCAN PIPELINE — FILTER HIERARCHY
//
// 1. Extension filter — skips binary/irrelevant file types
// 2. Binary check — skips files with null bytes in first 512 bytes
// 3. Pattern matching — ALL known patterns run against content
// 4. Heuristic filters — entropy, word filter, token efficiency
//    CRITICAL: Known pattern matches BYPASS all heuristic filters.
// 5. FP suppression — placeholder, hash context, PEM certs, vault encryption
// 6. File-type context — test/doc exclusion for non-pattern matches only
//
// Known patterns are PRIVILEGED — they bypass steps 4 and 6.
// This prevents recall regressions from heuristic filtering.
```

**2D. Quality checks:**
```bash
cargo clippy -- -W clippy::all    # fix all warnings
cargo fmt                          # format
cargo test                         # all 185+ must pass
```

---

### PHASE 3: Pre-Push Safety

**3A. Review `.gitignore`** — ensure `target/`, `*.log`, temp files are ignored.

**3B. Self-scan:**
```bash
cargo run -- scan . --format json
```
Investigate anything that looks like a real secret (not test fixtures).

**3C. Print `git status` and `git diff --stat`** so we can see what's being committed.

---

### PHASE 4: Write HANDOFF.md

Create `HANDOFF.md` in the project root. This documents current state for future development.

```markdown
# HANDOFF.md — Coax Scanner State

## What Is Coax?
Coax is a code trust scanner: secrets detection + Unicode/Trojan Source detection.
Rust CLI + TUI + VS Code extension.
Answers: "Is this code what it appears to be?"

## Performance (as of [date])

### Secrets Detection
[Paste final benchmark table: Coax before/after, Gitleaks, TruffleHog]
[Note: 0 FPs on fastify + express real-world repos]

### Unicode Detection
[Paste unicode before/after table from Phase 1C]

### Tests
[X] tests passing. Run: `cargo test`

## Architecture

### Scan Pipeline
[Paste the filter hierarchy comment from Phase 2C]

### Key Principle
Known secret patterns are PRIVILEGED — they bypass all heuristic filters
and file-type exclusions. This was established after fixing three stacked
bugs where heuristic filters silently discarded legitimate findings.

### Code Map
Brief description of each key source file and what it does.
[List the actual files — scanner, secrets, context, unicode, encoded, etc.]

## Known Gaps

### Remaining FN (1)
[From Phase 1A — file, type, why missed, fix approach]

### Remaining FPs (4)
[From Phase 1B — each file, trigger, fix approach]

### Encoded Detection
Base64/hex/URL-encoded secret detection exists but has gaps.
Not yet benchmarked against a dedicated corpus.

### Unicode Finding Grouping
Per-character findings, not per-line. UX issue, not detection issue.

## Benchmark Infrastructure
- Corpus location: [actual path]
- Ground truth: `ground-truth.yaml`
- How to reproduce: [commands]

## Development
\```bash
cargo build --release
cargo test
cargo clippy
\```
```

Keep it factual. No Phase 2 roadmap details — those will be provided separately.

---

### PHASE 5: Commit & Push

```bash
git add -A
git commit -m "feat: close benchmark gaps — F1 0.86 → 0.96

- Expand extension filter: .csv, .gradle, .ipynb, .npmrc, .pp, .sls
- Add extensionless file handling: Jenkinsfile, Makefile, Vagrantfile
- Add binary detection, placeholder detection, PEM/vault/hash exclusions
- Fix unicode FP on safe internationalization
- 185 tests passing, 0 real-world FPs
- Add HANDOFF.md documenting scanner state and architecture"

git push origin main
```

Confirm push succeeded. Print final summary.

---

### Checklist

```
- [ ] 1 FN and 4 FP documented with remediation paths
- [ ] Unicode before/after table produced
- [ ] Dead code removed
- [ ] Patterns organized with section comments
- [ ] Architecture comment added to scan pipeline
- [ ] cargo clippy — no warnings
- [ ] cargo fmt — clean
- [ ] cargo test — all passing
- [ ] Self-scan clean
- [ ] HANDOFF.md written
- [ ] Committed and pushed
```

**Work through phases sequentially. Print gap documentation and HANDOFF.md content before pushing.**

---

