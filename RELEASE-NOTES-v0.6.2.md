# Coax v0.6.2 Release Notes

**Release Date:** 2026-03-16
**Version:** 0.6.2
**Major Feature:** Unicode CLI Integration Complete

## What's New

### Unicode Attack Detection CLI Flags

The Unicode scanner is now fully integrated into the CLI with the following flags:

- `--unicode-scan` - Enable/disable Unicode detection (default: true)
- `--unicode-sensitivity` - Adjust sensitivity level (low, medium, high, critical) [default: high]
- `--unicode-only` - Scan only for Unicode attacks (skip secret detection)

### Detected Unicode Attacks

The Unicode scanner detects:

- **Invisible Characters**: Zero-width characters, variation selectors
- **Homoglyph Attacks**: Confusable characters from Cyrillic, Greek scripts
- **Bidirectional Overrides**: Dangerous BIDI control characters (RLO, LRO, etc.)
- **Glassworm Patterns**: Specialized decoder/eval-based obfuscation
- **Unicode Tags**: Tag characters used for metadata injection

## Architecture Changes

### ScannerConfig

Added `unicode: UnicodeConfig` field to `ScannerConfig` with:
- `enabled` - Enable Unicode scanning
- `sensitivity` - SensitivityLevel (low/medium/high/critical)
- `detectors` - Per-detector enable/disable configuration
- `allowlist` - For legitimate i18n usage
- `performance` - Performance tuning options

### Scanner Struct

Added `unicode_scanner: Option<UnicodeScanner>` field for optional Unicode scanning.

### Integration

Unicode scanning is now integrated into:
- `scan_file()` - Single file scanning
- `scan_content()` - Content scanning
- `scan_files_parallel()` - Parallel directory scanning

## Bug Fixes

- Unicode scanner properly integrated into main pipeline
- UnicodeFinding properly converts to ScanResult via `to_scan_result()`
- SARIF output includes Unicode rules
- Fixed SensitivityLevel parsing in CLI

## Test Results

- All existing tests passing
- Unicode detection working end-to-end
- Tested with RLO (U+202E), zero-width characters, homoglyphs

## Usage Examples

### Basic Unicode Scan

```bash
# Scan with default settings (Unicode enabled, high sensitivity)
coax scan -p ./src

# Disable Unicode scanning
coax scan -p ./src --unicode-scan false

# Only scan for Unicode attacks
coax scan -p ./src --unicode-only

# Adjust sensitivity
coax scan -p ./src --unicode-sensitivity critical
```

### Detection Example

```
$ printf 'const x = "test\xe2\x80\xae";' > /tmp/test.js
$ coax scan -p /tmp/test.js

🚨 /tmp/test.js:1:20 - UNICODE-BIDIRECTIONAL_OVERRIDE [CRITICAL]
   Recommendation: IMMEDIATE ACTION: Remove this RLO character...
```

## Installation

```bash
cargo build --release
sudo cp target/release/coax /usr/local/bin/
```

## Compatibility

- Backward compatible with v0.6.x
- Unicode scanning enabled by default
- No breaking changes to existing APIs

## Contributors

- Unicode scanner module integration
- CLI flag implementation
- End-to-end testing

---

**Previous Version:** v0.6.1
**Next Version:** v0.7.0 (planned)
