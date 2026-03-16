# Coax vs Gitleaks vs TruffleHog - Benchmark Comparison

**Date:** March 16, 2026
**Test Corpus:** coax-benchmarks datasets (secrets and unicode)

## Executive Summary

This benchmark compares three secret detection tools across two test datasets:
- **Secrets Dataset**: Contains true positives, true negatives, and encoded secrets
- **Unicode Dataset**: Contains bidirectional attacks, homoglyphs, invisible characters, and safe i18n content

## Summary Table

| Tool | Secrets Found | Unicode Found | Total | Scan Time (Secrets) | Scan Time (Unicode) |
|------|--------------|---------------|-------|---------------------|---------------------|
| Coax | 260 | 683 | 943 | 1,712ms | 765ms |
| Gitleaks | 37 | 1 | 38 | 749ms | 465ms |
| TruffleHog | 5 | 0 | 5 | 5,092ms | 9.65ms |

## Detailed Results

### Secrets Detection

#### Coax (260 findings)
- **UNICODE-HOMOGLYPH**: 250 findings
- **GENERIC_SECRET**: 6 findings
- **HIGH_ENTROPY_STRING**: 4 findings

Coax detected a high number of homoglyph attacks embedded within the secrets dataset, in addition to traditional secret patterns.

#### Gitleaks (37 findings)
Gitleaks found 37 secrets using its rule-based detection. The tool focuses on known secret patterns and API key formats.

#### TruffleHog (5 findings)
TruffleHog found 5 verified secrets:
- 1x SQLServer credential
- 1x Postgres connection string
- 1x Google Gemini API Key
- 1x NPM Token
- 1x MongoDB connection string

TruffleHog uses verification by attempting connections, which explains the lower count but higher confidence.

### Unicode Detection

#### Coax (683 findings)
- **UNICODE-HOMOGLYPH**: 436 findings
- **UNICODE-INVISIBLE_CHARACTER**: 145 findings
- **UNICODE-BIDIRECTIONAL_OVERRIDE**: 98 findings
- **GENERIC_SECRET**: 2 findings
- **UNICODE-UNICODE_TAG**: 1 finding
- **GENERIC_PASSWORD**: 1 finding

Coax has comprehensive unicode attack detection including:
- Homoglyph detection (visually similar characters)
- Invisible character detection (zero-width spaces, etc.)
- Bidirectional override detection (text reversal attacks)
- Unicode tag detection

#### Gitleaks (1 finding)
- 1x Generic API Key detected in unicode dataset (sk_live_abc123)

Gitleaks has minimal unicode-specific detection capabilities.

#### TruffleHog (0 findings)
No findings in the unicode dataset. TruffleHog focuses on credential verification rather than unicode attack detection.

## Analysis

### Detection Capabilities

1. **Coax**
   - **Strengths**: Comprehensive unicode attack detection, high sensitivity to homoglyphs and invisible characters
   - **Weaknesses**: May produce more false positives due to aggressive homoglyph detection
   - **Best for**: Security audits requiring unicode attack detection, code review for supply chain attacks

2. **Gitleaks**
   - **Strengths**: Fast scanning, well-known secret patterns, good for CI/CD integration
   - **Weaknesses**: Limited unicode attack detection
   - **Best for**: Git repository scanning, CI/CD pipelines, known secret patterns

3. **TruffleHog**
   - **Strengths**: Credential verification (reduces false positives), high confidence findings
   - **Weaknesses**: Slowest scan time, no unicode attack detection
   - **Best for**: Production environments where verified secrets are critical

### Performance Comparison

| Metric | Coax | Gitleaks | TruffleHog |
|--------|------|----------|------------|
| Total Findings | 943 | 38 | 5 |
| Secrets Scan Time | 1,712ms | 749ms | 5,092ms |
| Unicode Scan Time | 765ms | 465ms | 9.65ms |
| Fastest Overall | - | ✓ | - |
| Most Findings | ✓ | - | - |

### Key Observations

1. **Finding Volume**: Coax found significantly more findings (943) compared to Gitleaks (38) and TruffleHog (5). This is primarily due to Coax's unicode attack detection capabilities.

2. **Scan Speed**: Gitleaks was the fastest for secrets scanning (749ms), while TruffleHog was the slowest (5,092ms) due to its verification process.

3. **Unicode Detection**: Only Coax has comprehensive unicode attack detection. Gitleaks found 1 incidental finding, and TruffleHog found none.

4. **False Positive Considerations**: 
   - Coax's high finding count includes many homoglyph detections that may be intentional (i18n) or false positives
   - TruffleHog's verification process eliminates false positives but may miss unverified secrets
   - Gitleaks provides a middle ground with pattern-based detection

## Errors and Issues Encountered

1. **Gitleaks**: Initially failed with git repository error. Resolved by adding `--no-git` flag for filesystem scanning.

2. **TruffleHog**: Initially failed with updater error. Resolved by adding `--no-update` flag.

3. **Coax**: No errors encountered. Exit code 1 on unicode scan indicates findings were detected (expected behavior).

## Recommendations

1. **For comprehensive security scanning**: Use Coax for its unicode attack detection capabilities
2. **For CI/CD integration**: Use Gitleaks for fast, reliable secret detection
3. **For verified credentials**: Use TruffleHog when credential verification is required
4. **For defense in depth**: Consider running all three tools in combination

## Files Generated

- `/tmp/coax-benchmarking/results/coax-secrets.json`
- `/tmp/coax-benchmarking/results/coax-unicode.json`
- `/tmp/coax-benchmarking/results/gitleaks-secrets.json`
- `/tmp/coax-benchmarking/results/gitleaks-unicode.json`
- `/tmp/coax-benchmarking/results/trufflehog-secrets.json`
- `/tmp/coax-benchmarking/results/trufflehog-unicode.json`
- `/tmp/coax-benchmarking/scripts/analyze-results.sh`

---
*Report generated on March 16, 2026*
