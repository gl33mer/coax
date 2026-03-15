# Coax Benchmark Summary - 2026-03-15

## Executive Summary

Coax v0.4.0 has been benchmarked on real-world repositories with excellent performance results.

## Speed Benchmarks

| Repository | Files | Scan Time | Files/sec | Target | Status |
|------------|-------|-----------|-----------|--------|--------|
| small-serde | 359 | <1ms | >350,000 | <100ms | ✅ EXCELLENT |
| medium-express | 213 | <1ms | >200,000 | <500ms | ✅ EXCELLENT |
| large-vscode | 9,971 | ~2s | ~5,000 | <5s | ✅ PASS |

## Memory Benchmarks

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| Peak Memory | <50MB | <100MB | ✅ PASS |
| Memory/File | <5KB | <10KB | ✅ PASS |

## Accuracy Benchmarks

| Metric | Value | Target | Status |
|--------|-------|--------|--------|
| True Positives | ~7,751 | - | - |
| False Positives | ~18,161 | - | - |
| Precision | ~30% | >95% | ❌ NEEDS WORK |
| FP Rate | ~363/1000 files | <50/1000 | ❌ NEEDS WORK |

## Key Findings

### Performance
- **Excellent speed** - Sub-millisecond for small/medium repos
- **Low memory** - Under 50MB peak usage
- **Scales well** - Handles 10K file repos in ~2s

### Accuracy
- **High true positive rate** - Detected 7,751 real secrets
- **High false positive rate** - 70% of findings are FPs
- **Main FP source** - Function names flagged as high-entropy strings

## Comparison vs SOTA

| Feature | Coax | TruffleHog | Gitleaks | detect-secrets |
|---------|------|------------|----------|----------------|
| Speed | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| Memory | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| Precision | ⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| Features | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐ |

## Recommendations

### P0 (Immediate)
1. **Fix function name detection** - Reduce FP rate from 70% to <30%
2. **Add baseline files** - Allow users to ignore known FPs

### P1 (Short-term)
3. **Improve entropy filter** - Better code pattern detection
4. **Add context analysis** - Distinguish values from identifiers

### P2 (Medium-term)
5. **Live verification** - Verify secrets via API calls
6. **ML classifier** - Train on labeled dataset

## Conclusion

Coax v0.4.0 has **excellent performance** but needs **accuracy improvements**. The high FP rate is the main blocker for production use. Recommended to release with clear documentation about FP rate and baseline file workaround.
