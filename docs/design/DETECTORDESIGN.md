
---

```markdown
# Detector Model Design

> **Status:** Draft
> **Prerequisite for:** Credential verification, structured reporting, per-detector config
> **Read first:** `HANDOFF.md`

## Problem

Secret detection patterns are currently organized as const arrays of `SecretPattern` structs
grouped by category (AWS, GITHUB, CLOUD_PROVIDERS, etc.). Each `SecretPattern` carries name,
pattern, severity, recommendation, description, and cwe_id.

This works for pattern matching but can't support three capabilities we need:

1. **Verification.** No way to attach a "check if this credential is live" function to a
   pattern or group of patterns. Verification is what separates a finding from an alert.

2. **Full metadata on findings.** Some metadata from `SecretPattern` gets dropped before
   it reaches findings. Downstream consumers (SARIF, TUI, VS Code extension) need the
   full picture: description, CWE, detector identity, verification state.

3. **Per-detector configuration.** Can't enable/disable detection or verification for a
   specific provider without code changes.

## Goals

- Introduce a Detector abstraction that groups related patterns and optionally verifies
- Add VerificationStatus to the Finding model
- Ensure all SecretPattern metadata flows through to findings and SARIF output
- Support per-detector enable/disable and verify on/off via config
- **Zero test regressions** — all 185 existing tests must pass at every step
- **No performance regression** on fastify/express benchmarks

## Non-Goals

- Implementing actual verification logic (see `VERIFICATION-DESIGN.md`)
- Changing the unicode/trojan-source detection pipeline
- Custom user-defined detectors (Phase 3, `.coax/rules.toml`)

## Design Decisions

### 1. Detector as a Trait

Each detector represents a provider or category (AWS, GitHub, Stripe, generic/entropy).
It owns its patterns and optionally knows how to verify findings.

A trait (not just a bigger struct) because verification-capable detectors need to hold
state: HTTP clients, rate limiters, cached auth. A pattern-only detector that needs no
state should be trivially constructable from the existing const arrays with minimal
boilerplate.

**The existing `SecretPattern` struct and const arrays should not need to change.** The
detector layer wraps them. This minimizes blast radius.

### 2. VerificationStatus Enum

Four states:

| State | Meaning |
|---|---|
| `Unverified` | No verification attempted (unsupported, disabled, or not yet run) |
| `Verified` | Credential confirmed active against live service |
| `Invalid` | Credential confirmed revoked/expired/invalid |
| `Error(String)` | Verification attempted but inconclusive (rate limit, network error, timeout) |

This enum gets added to the Finding model. Default is `Unverified` so all existing
code paths are unaffected.

### 3. Severity Model (3-Tier)

| Level | Meaning |
|---|---|
| `Critical` | Known high-privilege pattern, or any pattern with `Verified` status |
| `High` | Known provider pattern, unverified |
| `Medium` | Generic/heuristic detection (entropy, assignment pattern) |

**Severity promotion:** `High` + `Verified` → `Critical`. `High` + `Invalid` → filtered
or demoted (policy-dependent). The Finding model should carry both the base severity
(from the pattern definition) and the effective severity (after verification adjustment).

### 4. Detector Registry

A central registry holds all detector instances. It replaces any current approach of
manually chaining pattern arrays. The scanner asks the registry for all patterns (to
build its regex/Aho-Corasick set) and can look up which detector owns a matched pattern
(for verification dispatch).

### 5. Per-Detector Config

In `.coax/config.toml` (or whatever config mechanism exists):

```toml
[detectors.aws]
enabled = true
verify = true

[detectors.github]
enabled = true
verify = false
```

Disabled detectors contribute no patterns to the scan. Verify=false skips the
verification call but still reports findings as `Unverified`.

## Finding Model Changes

Add these fields to the existing Finding struct (with backward-compatible defaults):

- `verification: VerificationStatus` (default: `Unverified`)
- `detector_id: String` — which detector produced this finding
- `base_severity` and `effective_severity` — if not already separated
- Ensure `description` and `cwe_id` from `SecretPattern` reach the Finding (audit
  the current pipeline for any metadata that gets dropped during conversion)

## Migration Strategy

**Principle: additive changes first, wiring second, cleanup last.**

1. **Add new types** — Detector trait, VerificationStatus, registry. No existing code
   changes. All tests pass.
2. **Wrap existing pattern arrays** — Create detector instances that reference the
   existing const arrays. Build registry. All tests pass.
3. **Wire registry into scanner** — Replace direct pattern array usage with registry
   calls. Populate new Finding fields. All tests pass.
4. **Clean up** — Remove any intermediate conversion layers that are now redundant.
   All tests pass.

Each step should be a separate commit. If any step breaks tests, stop and fix before
proceeding.

## SARIF Output

SARIF findings should include:
- `verification_status` in the result properties
- `cwe_id` in the rule metadata (may already be there — verify)
- `detector_id` for grouping/filtering

## Implementation Notes for Agent

- Explore the current conversion path from `SecretPattern` → scanner match → `Finding`
  → output. Identify where metadata gets dropped and fix it.
- Decide whether `verify()` should be async (it almost certainly needs to be for HTTP
  calls — use `async_trait` or Rust's native async traits if the MSRV supports it).
- The existing `SecretPattern` struct is good. Don't redesign it. Wrap it.
- Pattern-only detectors (no verification) should be expressible with minimal code — a
  convenience struct or macro, not a full trait impl per detector.
- The top-5 verification detectors (AWS, GitHub, Stripe, GCP, Azure) get full trait
  implementations. All others are pattern-only wrappers. Actual verification logic is
  specified in `VERIFICATION-DESIGN.md`.

## Success Criteria

- [ ] All 185 existing tests pass after migration
- [ ] Finding carries: verification status, detector_id, description, cwe_id, base + effective severity
- [ ] SARIF output includes verification status and CWE
- [ ] Detectors can be enabled/disabled via config
- [ ] No performance regression on fastify/express scans
- [ ] At least one detector (pick AWS) has a `verify()` stub that returns `Unverified`
```

---

