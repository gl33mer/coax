# Unicode CLI Flags Status

**Date:** 2026-03-16  
**Version:** v0.6.1  
**Status:** ⚠️ PARTIALLY IMPLEMENTED

---

## Current Status

### ✅ What Works

1. **Unicode Module** - Fully implemented and exported
2. **Unicode Scanner** - Integrated into main Scanner pipeline
3. **UnicodeFinding Conversion** - Converts to ScanResult properly
4. **SARIF Output** - Extended for Unicode findings
5. **Default Behavior** - Unicode scanning ENABLED by default

### ⚠️ What's Incomplete

**CLI Flags** - The following flags were defined but not fully wired up:
- `--unicode-scan` - Currently has no effect (Unicode always enabled)
- `--unicode-sensitivity` - Currently has no effect (uses default "high")
- `--unicode-only` - Not implemented

---

## Why CLI Flags Aren't Working

The `ScannerConfig` struct needs:
1. `unicode: UnicodeConfig` field
2. `with_unicode_enabled(bool)` builder method
3. `with_unicode_sensitivity(SensitivityLevel)` builder method

These require changes to the core ScannerConfig which would need:
- Updating ScannerConfig struct
- Updating Scanner struct to use the config
- Testing all scanner functionality

This is a **1-2 hour task** that was underestimated.

---

## Workaround

**Unicode scanning IS enabled by default!** You can use it right now:

```bash
# Standard scan (includes Unicode detection)
./target/release/coax scan -p .

# The scan will detect:
# - Glassworm patterns
# - Homoglyphs
# - Invisible characters
# - Bidirectional overrides
# - Unicode tags
```

---

## Next Steps (v0.6.2)

1. Add `unicode: UnicodeConfig` field to ScannerConfig
2. Implement builder methods
3. Wire up CLI flags properly
4. Test all combinations
5. Release v0.6.2

---

## Test Results

| Feature | Status |
|---------|--------|
| Unicode module exported | ✅ Working |
| Unicode scanner integrated | ✅ Working |
| UnicodeFinding conversion | ✅ Working |
| SARIF output | ✅ Working |
| CLI flags | ⚠️ Defined but not wired |
| Default Unicode scanning | ✅ ENABLED |

**Bottom Line:** Unicode detection works perfectly, you just can't disable it via CLI yet.
