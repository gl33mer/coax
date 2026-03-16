# 📋 Coax v0.8.0 - VS Code Extension Review & Cross-Platform Strategy

**Review Date:** March 16, 2026  
**Repository:** https://github.com/PropertySightlines/coax  
**Status:** ⚠️ **GitHub Rate Limited** - Cannot fetch source files directly

---

## ✅ What Looks Good (From Summary)

| Aspect | Status | Notes |
|--------|--------|-------|
| **Extension Structure** | ✅ Complete | `coax-vscode/` with proper subdirectories |
| **VSIX Package** | ✅ Built | 2.59 MB (reasonable size) |
| **Test Results** | ✅ 98.75% | 158/160 passing (2 CFG pre-existing) |
| **Binary Bundling** | ✅ Linux x64 | 5.65 MB binary included |
| **Documentation** | ✅ Updated | HANDOFF.md, RELEASE-NOTES current |
| **Version Tags** | ✅ All present | v0.4.0 through v0.8.0 |

---

## ⚠️ Items to Verify (When Rate Limit Clears)

### Critical Checks

| File | What to Verify | Why |
|------|---------------|-----|
| `coax-vscode/package.json` | Engine version, activation events, contributes | Marketplace rejection if wrong |
| `coax-vscode/src/extension.ts` | 500ms debounce implementation | Must match spec |
| `coax-vscode/src/scanner/index.ts` | Binary path resolution, error handling | Cross-platform compatibility |
| `coax-vscode/src/diagnostics/` | DiagnosticCollection, severity mapping | UX quality |
| `coax-vscode/src/actions/codeActions.ts` | 5+ quick-fix actions | Feature completeness |
| `coax-vscode/bundled/` | Only linux-x64 present (for now) | Expected for v0.8.0 |

---

## 🎯 GitHub Actions for Cross-Platform Builds: **YES, Absolutely**

### Why GitHub Actions is the Right Choice

| Factor | GitHub Actions | Manual Build |
|--------|---------------|--------------|
| **Consistency** | ✅ Same environment every time | ❌ Depends on developer machine |
| **Automation** | ✅ Trigger on tag/release | ❌ Manual process |
| **Multi-Platform** | ✅ ubuntu, macos, windows runners | ❌ Need 3 different machines |
| **Artifact Storage** | ✅ Automatic upload to release | ❌ Manual upload |
| **Reproducibility** | ✅ Exact same binary every time | ❌ Varies by build environment |
| **Time** | ✅ 15-20 min parallel builds | ❌ 1-2 hours manual |
| **Cost** | ✅ Free for OSS (2000 min/month) | ❌ Developer time |

---

## 📋 Recommended GitHub Actions Workflow

**File:** `.github/workflows/build-vscode-extension.yml`

```yaml
name: Build VS Code Extension

on:
  push:
    tags:
      - 'v0.8.*'
  workflow_dispatch:

jobs:
  build-cli:
    name: Build Coax CLI (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
            artifact: coax-linux-x64
          - os: macos-latest
            target: x86_64-apple-darwin
            artifact: coax-darwin-x64
          - os: macos-latest
            target: aarch64-apple-darwin
            artifact: coax-darwin-arm64
          - os: windows-latest
            target: x86_64-pc-windows-msvc
            artifact: coax-win32-x64.exe

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-action@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build Release Binary
        run: |
          cargo build --release --target ${{ matrix.target }}
          
      - name: Prepare Binary
        run: |
          mkdir -p coax-vscode/bundled/${{ matrix.artifact }}
          # Copy binary based on platform
          if [ "${{ runner.os }}" = "Windows" ]; then
            cp target/${{ matrix.target }}/release/coax.exe coax-vscode/bundled/${{ matrix.artifact }}/
          else
            cp target/${{ matrix.target }}/release/coax coax-vscode/bundled/${{ matrix.artifact }}/
          fi

      - name: Upload Binary Artifact
        uses: actions/upload-artifact@v4
        with:
          name: ${{ matrix.artifact }}
          path: coax-vscode/bundled/${{ matrix.artifact }}

  build-extension:
    name: Build VSIX Package
    runs-on: ubuntu-latest
    needs: build-cli

    steps:
      - uses: actions/checkout@v4

      - name: Download All Binaries
        uses: actions/download-artifact@v4
        with:
          path: coax-vscode/bundled/
          merge-multiple: true

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install Dependencies
        run: |
          cd coax-vscode
          npm ci

      - name: Package Extension
        run: |
          cd coax-vscode
          npm install -g @vscode/vsce
          vsce package --out coax-${{ github.ref_name }}.vsix

      - name: Upload VSIX
        uses: actions/upload-artifact@v4
        with:
          name: coax-extension
          path: coax-vscode/coax-*.vsix

  publish-release:
    name: Publish GitHub Release
    runs-on: ubuntu-latest
    needs: [build-cli, build-extension]
    if: startsWith(github.ref, 'refs/tags/')

    steps:
      - uses: actions/checkout@v4

      - name: Download VSIX
        uses: actions/download-artifact@v4
        with:
          name: coax-extension

      - name: Download All Binaries
        uses: actions/download-artifact@v4
        with:
          path: binaries/
          merge-multiple: true

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            coax-*.vsix
            binaries/*
          generate_release_notes: true
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

## 📊 Build Matrix

| Platform | Target | Runner | Build Time |
|----------|--------|--------|------------|
| **Linux x64** | `x86_64-unknown-linux-gnu` | ubuntu-latest | ~5 min |
| **macOS x64** | `x86_64-apple-darwin` | macos-latest | ~8 min |
| **macOS ARM64** | `aarch64-apple-darwin` | macos-latest | ~8 min |
| **Windows x64** | `x86_64-pc-windows-msvc` | windows-latest | ~10 min |
| **VSIX Package** | N/A | ubuntu-latest | ~3 min |
| **Total (parallel)** | | | **~15 min** |

---

## 🎯 v0.8.1 Release Plan

```
┌─────────────────────────────────────────────────────────────────┐
│                    v0.8.1 CROSS-PLATFORM RELEASE                  │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Week 1: GitHub Actions Setup                                    │
│  ─────────────────────────────                                   │
│  - [ ] Create .github/workflows/build-vscode-extension.yml      │
│  - [ ] Test workflow with manual trigger                        │
│  - [ ] Verify all 4 platform binaries build correctly           │
│                                                                  │
│  Week 2: Testing & Validation                                    │
│  ─────────────────────────────                                   │
│  - [ ] Test VSIX on Linux (existing)                            │
│  - [ ] Test VSIX on macOS (x64 + ARM)                           │
│  - [ ] Test VSIX on Windows                                     │
│  - [ ] Verify binary execution on all platforms                 │
│                                                                  │
│  Week 3: Marketplace Submission                                  │
│  ─────────────────────────────                                   │
│  - [ ] Create Microsoft publisher account                       │
│  - [ ] Prepare marketplace listing                              │
│  - [ ] Submit for review                                        │
│  - [ ] v0.8.1 release on GitHub                                 │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---
