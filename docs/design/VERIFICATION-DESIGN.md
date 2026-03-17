
---

```markdown
# Verification Design

> **Status:** Draft
> **Depends on:** `DETECTOR-MODEL-DESIGN.md` (Detector trait + VerificationStatus enum)
> **Read first:** `HANDOFF.md`, then `DETECTOR-MODEL-DESIGN.md`

## Problem

A pattern match tells you "this looks like an AWS key." Verification tells you "this
AWS key is currently active and has S3 access." Without verification, security teams
drown in findings they have to manually triage. Verification is the highest-value
feature a secret scanner can add.

## Scope

Phase 2 targets the top 5 providers by frequency in real-world leaks:

| Provider | Verification Approach | Risk Level |
|---|---|---|
| AWS | STS `GetCallerIdentity` — read-only, no side effects, no permissions needed | Safe |
| GitHub | `GET /user` with token as Bearer — read-only | Safe |
| Stripe | `GET /v1/balance` with secret key — read-only, minimal data | Safe |
| GCP | OAuth2 `tokeninfo` endpoint — read-only, no permissions needed | Safe |
| Azure | MS Graph `GET /me` with Bearer token — read-only | Safe |

All five use read-only endpoints that don't mutate state, don't require special
permissions, and don't expose sensitive data beyond "yes this credential works."

All other detectors remain pattern-only (`Unverified`) and are unaffected.

## Design Decisions

### 1. Verification Lives Inside the Detector

Verification is called by the detector's `verify()` method, not by a separate
post-processing pass. This keeps the detector self-contained: it owns its patterns,
its verification logic, and its provider-specific quirks (auth header format, endpoint
URL, response parsing).

The scan pipeline calls `verify()` after a pattern match, before emitting the finding.
The pipeline controls *whether* to call verify (based on config and profiles), but the
detector controls *how*.

### 2. Fail-Open Principle

Verification failure must never block scan results.

- Network timeout → `VerificationStatus::Error("timeout")` → finding still emitted
- Rate limited → `Error("rate_limited")` → finding still emitted
- Unexpected API response → `Error("unexpected_response")` → finding still emitted
- Verification disabled → `Unverified` → finding still emitted

A scan without network access produces the same findings as today, just with
`Unverified` status on everything. Verification is purely additive.

### 3. Per-Detector Rate Limiting

Each provider has different rate limits. AWS STS is generous. GitHub's API is
stricter. A global rate limiter either starves generous APIs or overloads strict ones.

Each verification-capable detector owns its rate limiter. The rate limiter should:
- Respect provider-documented limits with a safety margin
- Use exponential backoff on 429/5xx responses
- Stop attempting verification for a detector if it hits repeated failures
  (circuit-breaker pattern — after N consecutive errors, skip verification for
  that detector for the rest of the scan, log a warning)

Specific rate limits per provider are an implementation detail — the agent should
research current documented limits for each API.

### 4. Caching

Don't verify the same secret twice. If the same AWS key appears in 15 files, verify
once and cache the result.

**Cache key:** The secret value itself (the matched credential text, not the file
location or pattern name).

**Cache scope:** Per-scan by default (in-memory, discarded after scan completes).
Persistent cross-scan caching (file-based) is a future enhancement — don't build it
now, but don't make it impossible either.

**Cache behavior:**
- `Verified` → cache for remainder of scan
- `Invalid` → cache for remainder of scan
- `Error` → do NOT cache (retry on next occurrence)

### 5. Verification Profiles

Different contexts need different verification aggressiveness:

| Profile | Behavior | Use Case |
|---|---|---|
| `none` | Skip all verification. All findings are `Unverified`. | Air-gapped environments, fastest scan |
| `safe` | Verify using only read-only endpoints (all 5 current providers qualify) | Default for interactive use |
| `thorough` | Reserved for future. Could include write-test verification for providers where read-only isn't possible | Not implemented in Phase 2 |

Configured via CLI flag (`--verify=safe`) and/or config file. Default is `none` —
verification is opt-in. This is a deliberate choice: users should consciously decide
to make network calls during a scan.

### 6. Credential Management

Coax never stores or manages credentials for verification endpoints. It reads them
from environment variables following each provider's standard convention:

- AWS: `AWS_ACCESS_KEY_ID` + `AWS_SECRET_ACCESS_KEY` (or ambient credentials — STS
  GetCallerIdentity works with the *found* key directly, no separate auth needed)
- GitHub: The found token IS the credential being tested
- Stripe: The found key IS the credential being tested
- GCP: The found credential IS what's being tested
- Azure: The found token IS the credential being tested

**Important nuance:** For most providers, verification means "use the found secret
itself to call the API." You're testing whether the leaked credential works, so the
leaked credential IS the auth. No separate config needed.

The exception would be if a provider required a separate privileged credential to
look up whether another credential is valid. None of the top 5 require this.

### 7. Timeouts

Every verification HTTP call must have a hard timeout. Suggested default: 5 seconds
per call. Configurable via:

```toml
[verification]
timeout_seconds = 5
```

A scan of 1000 files should not take 30 minutes because a verification endpoint is
slow. The circuit-breaker (Decision 3) is the backstop: if a provider is consistently
slow or failing, stop trying.

### 8. What Verification Results Mean for Output

In the Finding model (from DETECTOR-MODEL-DESIGN.md):

| VerificationStatus | Effective Severity | SARIF Level | User Message |
|---|---|---|---|
| `Verified` | Promoted to Critical | error | "Active credential — immediate rotation required" |
| `Invalid` | Demoted or filtered | note (or omit) | "Credential is revoked/expired" |
| `Unverified` | Base severity unchanged | warning | "Potential credential — verification unavailable" |
| `Error(reason)` | Base severity unchanged | warning | "Potential credential — could not verify: {reason}" |

**Policy question:** Should `Invalid` findings be shown by default? Leaning yes-but-quiet
(shown with low severity, filterable). A revoked key in code is still a hygiene issue,
and knowing it's revoked is useful context. But this could be a config option.

## What This Design Does NOT Cover

| Topic | Why | Where |
|---|---|---|
| Detector model, trait, registry | Separate concern | `DETECTOR-MODEL-DESIGN.md` |
| Provider-specific API details | Implementation detail — agent should research current API docs | Agent task |
| HTTP client selection | Rust ecosystem choice | Agent decides |
| Rate limiter algorithm | Token bucket, leaky bucket, etc. | Agent decides |
| Async runtime | tokio, async-std, etc. | Agent decides (check what the codebase already uses) |
| Persistent cross-scan cache | Future enhancement | Phase 3 |
| Write-test verification | Future (`thorough` profile) | Phase 3 |

## Verification Provider Notes

Brief notes for each provider to help the agent get started. These are based on
publicly documented APIs as of early 2025 — the agent should verify these are still
current.

**AWS:**
- Endpoint: STS `GetCallerIdentity`
- Auth: Use the found access key + secret key pair to sign the request
- Success (200) → `Verified`. Response includes account ID and ARN (useful metadata).
- Auth failure (403) → `Invalid`
- Note: Requires BOTH access key ID and secret key. If only one is found, cannot verify.
  Consider correlating findings — if both `AWS_ACCESS_KEY` and `AWS_SECRET_KEY` are found
  nearby (same file or adjacent lines), pair them for verification.

**GitHub:**
- Endpoint: `GET https://api.github.com/user`
- Auth: `Authorization: Bearer <token>`
- Success (200) → `Verified`. Response includes username and scopes.
- 401 → `Invalid`
- Watch rate limits: 60 req/hour unauthenticated, 5000 authenticated

**Stripe:**
- Endpoint: `GET https://api.stripe.com/v1/balance`
- Auth: `Authorization: Bearer <secret_key>`
- Success (200) → `Verified` (live key) or note test vs live mode
- 401 → `Invalid`
- Note: Stripe test keys (`sk_test_`) vs live keys (`sk_live_`) — both verifiable

**GCP:**
- Endpoint: `GET https://oauth2.googleapis.com/tokeninfo?access_token=<token>`
- No auth header needed — token is in the URL
- Success (200) → `Verified`
- 400 → `Invalid`
- Note: GCP API keys (AIza...) are different from OAuth tokens. API keys can be
  verified via a simple API call (e.g., `GET https://maps.googleapis.com/maps/api/...`)
  but this varies by enabled service. Start with OAuth tokens only.

**Azure:**
- Endpoint: `GET https://graph.microsoft.com/v1.0/me`
- Auth: `Authorization: Bearer <token>`
- Success (200) → `Verified`
- 401 → `Invalid`
- Note: Azure tokens are typically short-lived JWTs. Many found tokens will be expired.
  Check JWT expiry before making the call to avoid unnecessary API hits.

## Success Criteria

- [ ] AWS, GitHub, and Stripe verification working (minimum 3 of 5)
- [ ] Rate limiting prevents API abuse (no provider rate limit exceeded in tests)
- [ ] Circuit breaker activates after repeated failures
- [ ] Cache prevents duplicate verification of same secret value
- [ ] `--verify=none` produces identical output to current scanner
- [ ] `--verify=safe` only adds verification metadata, never removes findings
- [ ] Verification timeout doesn't block scan completion
- [ ] Each verification provider has unit tests with mocked HTTP responses
```

---
