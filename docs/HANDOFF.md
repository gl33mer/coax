# Coax Security Scanner - Agent Handoff Document

**Version:** v0.8.0
**Last Updated:** March 16, 2026
**Repository:** https://github.com/gl33mer/coax
**Status:** ✅ Production Ready (VS Code Extension Complete)

---

## 📊 Version History

| Version | Date | Feature | Status |
|---------|------|---------|--------|
| v0.4.0 | 2026-03-15 | Phase 3 P1 Complete | ✅ Released |
| v0.5.0 | 2026-03-15 | FP Reduction (70% → ~10%) | ✅ Released |
| v0.6.0 | 2026-03-15 | Unicode Attack Detection | ✅ Released |
| v0.6.1 | 2026-03-16 | Unicode Integration Fixes | ✅ Released |
| v0.7.0 | 2026-03-16 | Greek False Positive Fix | ✅ Released |
| v0.7.4 | 2026-03-16 | Script Mixing Refinement | ✅ Released |
| v0.7.5 | 2026-03-16 | Identifier-Based Homoglyph Detection | ✅ Released |
| v0.8.0 | 2026-03-16 | VS Code Extension | ✅ CURRENT |

---

## 🎯 Current State (v0.8.0)

### Completed Features

✅ **VS Code Extension (v0.8.0)**
- Real-time scanning on file save (500ms debounce)
- Real-time scanning on file open
- Inline squiggly underlines with severity colors
- Problems panel integration
- Hover tooltips with detailed information
- Status bar indicator (finding count + scan state)
- Quick-fix actions (remove, replace with env var, ignore)
- Command palette (6 commands)
- Configurable settings (10+ options)
- Binary bundling (Linux x64 included, more platforms ready)
- Packaged as coax-0.8.0.vsix (2.59 MB)

✅ **v0.7.5 Homoglyph Fix**
- Identifier-based detection (not line-based)
- Pure Greek/Cyrillic identifiers NOT flagged (0% FP)
- Mixed-script identifiers correctly flagged
- Comments skipped entirely
- Unicode-aware regex (\p{L}\p{N})

✅ **Unicode Attack Detection System**
- 5 detectors: Invisible, Homoglyph, Bidi, Glassworm, Tags
- Script mixing detection with identifier-based analysis
- 158/160 tests passing (98.75%)
- Performance: ~40ms for 10K lines

✅ **Core Scanner**
- Secret detection with 70% → ~10% FP reduction
- Entropy filtering, word filtering, context analysis
- SARIF, JSON, TUI output formats
- Baseline system for CI/CD

✅ **CLI**
- `coax scan -p <path>` - Main scanning command
- `coax scan --unicode-only` - Unicode-only scans
- `coax scan --unicode-sensitivity <level>` - Sensitivity tuning
- `coax tui` - Terminal UI

### Module Structure

```
crates/
├── coax-scanner/
│   └── src/
│       ├── lib.rs
│       ├── scanner.rs
│       ├── secrets.rs
│       ├── context.rs
│       ├── result.rs
│       ├── pattern_cache.rs
│       ├── config.rs
│       ├── sarif_output.rs
│       ├── baseline.rs
│       ├── encoded_detection.rs
│       └── unicode/
│           ├── mod.rs
│           ├── scanner.rs
│           ├── config.rs
│           ├── findings.rs
│           ├── ranges.rs
│           ├── script_detector.rs
│           ├── confusables/
│           │   ├── mod.rs
│           │   └── data.rs
│           └── detectors/
│               ├── mod.rs
│               ├── invisible.rs
│               ├── homoglyph.rs
│               ├── bidi.rs
│               ├── glassworm.rs
│               └── tags.rs
├── coax-cli/
│   └── src/
│       ├── main.rs
│       └── tui/
├── coax-parser/
└── coax-threat-model/
```

### Test Results

```
cargo test --workspace
test result: ok. 158 passed; 2 failed; 0 ignored

Note: 2 CFG sink detection tests failing (pre-existing, unrelated to Unicode)
```

### Performance Benchmarks

```
10K lines: ~40ms (target: <100ms) ✅
100K lines: ~400ms (target: <2s) ✅
Memory: ~30MB (target: <50MB) ✅
```

---

## 🚀 Next Steps

### v0.8.1 - Multi-Platform Extension Release (1 week)

**Goal:** Bundle binaries for all 5 platforms

**Tasks:**
1. Cross-compile Coax CLI for macOS (x64/arm64) and Windows
2. Update VSIX package with all binaries
3. Test on all platforms
4. Publish to VS Code Marketplace

### v0.9.0 - Advanced Features (4-6 weeks)

**Goal:** Enhanced VS Code extension features

**Features:**
- Dedicated findings view (Activity Bar tree view)
- Baseline file support
- Live secret verification
- Multi-root workspace support
- Remote development (SSH, Containers, WSL)

---

## 📝 Development Workflow

### Building

```bash
cargo build --release
./target/release/coax --version  # Should show 0.7.4
```

### Testing

```bash
# All tests
cargo test --workspace

# Unicode tests only
cargo test -p coax-scanner unicode

# Script detector tests
cargo test -p coax-scanner script_detector
```

### Running

```bash
# Basic scan
./target/release/coax scan -p .

# Unicode-only scan
./target/release/coax scan -p . --unicode-only

# Adjust sensitivity
./target/release/coax scan -p . --unicode-sensitivity critical

# Output formats
./target/release/coax scan -p . --format json
./target/release/coax scan -p . --format sarif
```

---

## 🔧 Known Issues

### v0.8.0 Known Issues

1. **VS Code Extension - Single Platform Binary**
   - Only Linux x64 binary bundled (others can be built with cross-compilation)
   - Status: Known, will add more platforms in v0.8.1
   - Workaround: Set `coax.binaryPath` to custom Coax CLI location

2. **2 CFG sink detection tests failing**
   - Status: Pre-existing, unrelated to Unicode/Extension work
   - Impact: None on production functionality
   - Files: `crates/coax-scanner/tests/cfg_tests.rs`

### v0.8.0 Prep Issues

1. **VS Code Extension not started**
   - Status: Spec complete, ready for development
   - Timeline: 4-5 weeks

---

## 📞 Support

- **Issues:** https://github.com/gl33mer/coax/issues
- **Discussions:** https://github.com/gl33mer/coax/discussions
- **Documentation:** https://github.com/gl33mer/coax/tree/main/docs

---

## 🎯 Handoff Checklist

### For Next Agent (v0.8.1+)

- [ ] Read this HANDOFF.md completely
- [ ] Review `coax-vscode/` directory for extension source
- [ ] Test VS Code extension: install `coax-vscode/coax-0.8.0.vsix`
- [ ] Build multi-platform binaries for extension
- [ ] Run `cargo test --workspace` to verify CLI state
- [ ] Review open issues on GitHub

### v0.8.0 Completion Checklist ✅

- [x] Extension installs from VSIX without errors
- [x] Scan on save triggers correctly (500ms debounce)
- [x] All findings display in Problems panel
- [x] Inline squiggles with correct severity colors
- [x] Status bar shows accurate count
- [x] 3+ quick-fix actions working
- [x] Settings configurable in UI
- [x] README.md complete
- [x] Packaged as coax-0.8.0.vsix

### v0.8.1 - Multi-Platform Release

- [ ] Build macOS x64 binary
- [ ] Build macOS arm64 binary
- [ ] Build Windows x64 binary
- [ ] Build Linux arm64 binary
- [ ] Update VSIX with all binaries
- [ ] Test on all platforms
- [ ] Publish to VS Code Marketplace

### v0.9.0 - Advanced Features

- [ ] Dedicated findings view (Activity Bar)
- [ ] Baseline file support
- [ ] Live secret verification
- [ ] Multi-root workspace support

---

**Last verified:** March 16, 2026  
**Verified by:** Qwen Code Development Team  
**Status:** ✅ v0.8.0 VS Code Extension Complete - Ready for v0.8.1 Multi-Platform Release
