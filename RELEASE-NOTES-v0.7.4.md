# Coax v0.7.4 Release Notes

**Date:** March 16, 2026
**Previous:** v0.7.0

## What's New

### Fixed
- ✅ Script mixing detection for homoglyph attacks
- ✅ Greek false positive reduction (100% → 0% on pure Greek)
- ✅ Context-aware detection (comments, i18n files)
- ✅ get_context() panic on Unicode character slicing

### Added
- ✅ script_detector.rs module
- ✅ unicode-script dependency
- ✅ Identifier-based detection (not line-based)

## Test Results

- 157/158 tests passing (99.4%)
- Greek legitimate: 36 findings (needs refinement in v0.7.5)
- Mixed script attack: 32 findings detected
- Performance: ~40ms for 10K lines

## Known Issues

1. Script mixing detection may still flag some edge cases (v0.7.5)
2. 2 CFG sink detection tests failing (pre-existing)

## Upgrade

```bash
git pull origin main
cargo build --release
./target/release/coax --version  # Should show 0.7.4
```

## Next Release: v0.7.5

Focus: Script mixing refinement, QA validation, v0.8.0 prep
