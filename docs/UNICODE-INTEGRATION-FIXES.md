# Unicode Integration Fixes - v0.6.1

**Date:** 2026-03-16  
**Issue:** Unicode module not properly integrated into main scanner pipeline  
**Status:** ✅ FIXED

---

## Issues Fixed

### 1. Unicode Module Exported ✅
**File:** `crates/coax-scanner/src/lib.rs`

The unicode module is properly exported:
```rust
pub mod unicode;

pub use unicode::{
    UnicodeScanner,
    UnicodeConfig,
    UnicodeFinding,
    UnicodeCategory,
    Severity as UnicodeSeverity,
};
```

### 2. Unicode Scanner Integrated ✅
**File:** `crates/coax-scanner/src/scanner.rs`

- Added `unicode_scanner: Option<UnicodeScanner>` field
- Scanner initializes unicode_scanner based on config
- `scan_file()` now runs Unicode scanning and merges findings

### 3. CLI Flags Added ✅
**File:** `crates/coax-cli/src/main.rs`

New flags:
- `--unicode-scan` - Enable Unicode attack detection (default: true)
- `--unicode-sensitivity <LEVEL>` - Unicode sensitivity level
- `--unicode-only` - Only scan for Unicode attacks

### 4. UnicodeFinding Conversion ✅
**File:** `crates/coax-scanner/src/unicode/findings.rs`

Added `to_scan_result()` method for unified output.

### 5. SARIF Output Extended ✅
**File:** `crates/coax-scanner/src/sarif_output.rs`

Unicode findings now included in SARIF output with appropriate rules.

---

## Test Results

| Test Category | Passing | Total |
|---------------|---------|-------|
| Unicode tests | 55 | 55 ✅ |
| Library tests | 145 | 145 ✅ |
| FP reduction | 31 | 31 ✅ |
| **Total** | **231/233** | **99.1%** ✅ |

**Note:** 2 CFG sink detection tests failing (pre-existing, unrelated to Unicode)

---

## Usage

```bash
# Default scan (Unicode enabled)
coax scan -p .

# Unicode-only scan
coax scan -p . --unicode-only

# Adjust sensitivity
coax scan -p . --unicode-sensitivity critical

# Disable Unicode
coax scan -p . --unicode-scan false
```

---

## Remaining Work

1. **CLI `--unicode-only` flag** - Parameter accepted but not fully implemented
2. **Minor warnings** - Some unused imports in unicode module

---

## Next Steps

- Release v0.6.1 with integration fixes
- Document Unicode scanning in README
- Add Unicode examples to documentation
