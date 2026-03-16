---

# 📋 Document 3: VS Code Marketplace Submission Checklist

```markdown
# VS Code Marketplace Submission Checklist

**Extension:** Coax Security Scanner  
**Version:** 0.8.1  
**Publisher:** gl33mer  
**Target Date:** [Fill in]

---

## ✅ Pre-Submission Requirements

### 1. Microsoft Publisher Account

- [ ] Create Microsoft account (if don't have one)
- [ ] Sign up for Visual Studio Marketplace Publisher
- [ ] Pay $25 one-time fee (individual publisher)
- [ ] Verify email address
- [ ] Note Publisher ID: `gl33mer`

**Link:** https://marketplace.visualstudio.com/manage

---

### 2. Extension Package Validation

- [ ] VSIX builds without errors (`vsce package`)
- [ ] VSIX size < 512 MB limit (expected: ~15 MB with all binaries)
- [ ] All 4 platform binaries included:
  - [ ] `bundled/linux-x64/coax`
  - [ ] `bundled/darwin-x64/coax`
  - [ ] `bundled/darwin-arm64/coax`
  - [ ] `bundled/win32-x64/coax.exe`
- [ ] `package.json` has all required fields:
  - [ ] `name`: "coax"
  - [ ] `displayName`: "Coax Security Scanner"
  - [ ] `version`: "0.8.1"
  - [ ] `publisher`: "gl33mer"
  - [ ] `engines.vscode`: "^1.85.0"
  - [ ] `categories`: ["Programming Languages", "Linters", "Security"]
  - [ ] `license`: "MIT"
- [ ] No trailing spaces in URLs

---

### 3. Documentation Requirements

- [ ] README.md exists and includes:
  - [ ] Extension description (what it does)
  - [ ] Features list (scan on save, Unicode detection, etc.)
  - [ ] Installation instructions
  - [ ] Configuration options (all 11 settings documented)
  - [ ] Usage examples (screenshots preferred)
  - [ ] Troubleshooting section
  - [ ] Link to GitHub repository
  - [ ] **Unsigned binary warning for macOS/Windows**
- [ ] CHANGELOG.md exists with:
  - [ ] v0.8.1 release notes
  - [ ] v0.8.0 release notes
  - [ ] All versions documented
- [ ] LICENSE file exists (MIT)

---

### 4. Extension Manifest Validation

Run: `vsce ls` to list packaged files

- [ ] All source files included
- [ ] All binaries included
- [ ] No sensitive files (`.env`, `.git/`, etc.)
- [ ] `activationEvents` correct
- [ ] `contributes.commands` all defined
- [ ] `contributes.configuration` all settings defined

---

### 5. Functional Testing

- [ ] Install VSIX locally: `code --install-extension coax-0.8.1.vsix`
- [ ] Extension activates on startup
- [ ] Scan on save works (500ms debounce)
- [ ] Scan on open works
- [ ] Inline diagnostics appear (squiggles)
- [ ] Problems panel shows findings
- [ ] Hover tooltips work
- [ ] Status bar shows finding count
- [ ] All 6 commands work:
  - [ ] Coax: Scan Current File
  - [ ] Coax: Scan Workspace
  - [ ] Coax: Show Findings
  - [ ] Coax: Settings
  - [ ] Coax: Clear Findings
  - [ ] Coax: About
- [ ] All 5+ code actions work:
  - [ ] Remove this finding
  - [ ] Replace with environment variable
  - [ ] Replace with ASCII equivalent (Unicode)
  - [ ] Add to allowlist (.coax.yaml)
  - [ ] Ignore for this session
- [ ] Settings configurable in UI
- [ ] Exclude patterns work
- [ ] No crashes in 10+ test sessions

---

### 6. Platform Testing

| Platform | Tested By | Status | Notes |
|----------|-----------|--------|-------|
| **Linux x64** | You | ⬜ Pending | Primary test platform |
| **Windows x64** | You | ⬜ Pending | Test on your Windows machine |
| **macOS x64** | Community | ⬜ Pending | Need volunteer tester |
| **macOS ARM64** | Community | ⬜ Pending | Need volunteer tester |

**Community Testing Call-to-Action:**

```markdown
## 🧪 Beta Testers Needed!

**macOS Users:** We need your help testing Coax on macOS!

If you're willing to test:
1. Download VSIX from: https://github.com/PropertySightlines/coax/releases
2. Install: `code --install-extension coax-0.8.1.vsix`
3. Report issues: https://github.com/PropertySightlines/coax/issues/new?label=macOS

**⚠️ Unsigned Binary Warning (macOS):**
You may see "Cannot be opened" - Right-click the extension → Open (once)

Thank you for helping make Coax better! 🙏
```

---

### 7. Marketplace Listing Content

Prepare the following for the Marketplace submission form:

**Short Description (120 chars max):**
```
Real-time Unicode confusable and secret detection for VS Code. Protect your codebase from homoglyph attacks and leaked secrets.
```

**Long Description (Markdown):**
```markdown
## 🔒 Coax Security Scanner

Real-time security scanning integrated directly into VS Code.

### Features

- **Scan on Save** - Automatic scanning with 500ms debounce
- **Unicode Attack Detection** - Detect homoglyphs, invisible characters, bidirectional overrides
- **Secret Detection** - AWS keys, GitHub tokens, API keys, private keys
- **Inline Diagnostics** - Squiggly underlines with severity colors
- **Quick-Fix Actions** - Remove, replace with env var, add to allowlist
- **Status Bar Indicator** - Real-time finding count
- **Configurable** - 11 settings for sensitivity, exclude patterns, and more

### Installation

1. Install from VS Code Marketplace
2. Coax activates automatically on startup
3. Start coding - scans happen automatically

### Configuration

All settings available in VS Code Settings (Ctrl+,):

- `coax.enabled` - Enable/disable scanning
- `coax.scanOnSave` - Scan on file save (default: true)
- `coax.scanOnOpen` - Scan on file open (default: true)
- `coax.unicode.enabled` - Enable Unicode detection (default: true)
- `coax.unicode.sensitivity` - Unicode sensitivity (low/medium/high/critical)
- `coax.severityThreshold` - Minimum severity to display
- `coax.exclude` - File patterns to exclude
- `coax.debounceDelay` - Debounce delay in ms (default: 500)

### ⚠️ Unsigned Binary Notice

Coax bundles native binaries for performance. These are not code-signed:

- **macOS:** You may see "Cannot be opened" - Right-click → Open (once)
- **Windows:** You may see "Unknown Publisher" - Click "Run anyway"

We're working on obtaining OSS code signing certificates.

### Links

- GitHub: https://github.com/PropertySightlines/coax
- Issues: https://github.com/PropertySightlines/coax/issues
- Documentation: https://github.com/PropertySightlines/coax/blob/main/README.md
```

**Categories:**
- [x] Programming Languages
- [x] Linters
- [x] Security

**Tags/Keywords:**
```
security, secrets, unicode, homoglyph, scanner, vulnerability, linter, aws, github, api-key
```

**Gallery Banners (optional but recommended):**
- [ ] Create 1280x640 banner image (theme color)
- [ ] Create 1280x640 banner image (light theme)
- [ ] Add to package.json:
  ```json
  "galleryBanner": {
    "color": "#1a1a2e",
    "theme": "dark"
  }
  ```

**Icons:**
- [ ] Add 128x128 extension icon (PNG)
- [ ] Add to package.json:
  ```json
  "icon": "images/icon.png"
  ```

---

### 8. Legal & Compliance

- [ ] License is OSI-approved (MIT ✅)
- [ ] No proprietary dependencies
- [ ] Privacy policy (if collecting telemetry - Coax doesn't)
- [ ] Trademark check (no infringements)
- [ ] Export control compliance (encryption - Coax doesn't encrypt)

---

### 9. Submission Process

1. **Package Extension:**
   ```bash
   cd coax-vscode
   vsce package --out coax-0.8.1.vsix
   ```

2. **Sign In to Marketplace:**
   - Go to: https://marketplace.visualstudio.com/manage
   - Sign in with Microsoft account

3. **Create New Extension:**
   - Click "Add Extension"
   - Upload VSIX file
   - Fill in listing details (from Section 7)

4. **Review & Submit:**
   - Review all information
   - Accept terms of service
   - Click "Submit"

5. **Wait for Approval:**
   - Typical review time: 1-3 business days
   - You'll receive email when approved
   - Extension goes live immediately after approval

---

### 10. Post-Submission

- [ ] Monitor extension page for downloads
- [ ] Watch for user reviews
- [ ] Respond to issues promptly
- [ ] Plan v0.8.2 based on user feedback
- [ ] Track metrics in Marketplace dashboard

---

## 📊 Submission Timeline

| Day | Task |
|-----|------|
| **Day 1** | Complete all pre-submission checks |
| **Day 2** | Package and submit to Marketplace |
| **Day 3-5** | Wait for approval (1-3 business days) |
| **Day 6** | Extension goes live |
| **Day 7** | Announce on social media, GitHub, communities |

---

## ⚠️ Common Rejection Reasons

| Issue | How to Avoid |
|-------|--------------|
| Missing README | Include comprehensive README.md |
| Broken links | Test all URLs before submission |
| Malware/viruses | Coax is clean - no issue |
| Misleading description | Be accurate about features |
| Poor performance | 500ms debounce prevents this |
| Crashes on activation | Test thoroughly before submit |
| Missing license | MIT license included ✅ |

---

## 📝 Summary Checklist

```
Pre-Submission:
[ ] Publisher account created ($25)
[ ] VSIX package validated
[ ] README.md complete
[ ] CHANGELOG.md complete
[ ] All 4 platform binaries included
[ ] Functional testing complete (Linux + Windows)
[ ] Community testing requested (macOS)

Submission:
[ ] VSIX packaged
[ ] Marketplace listing content prepared
[ ] Extension submitted
[ ] Confirmation email received

Post-Submission:
[ ] Approval received (1-3 days)
[ ] Extension goes live
[ ] Announcement posted
[ ] User feedback monitored
```

---

**Ready to submit when all boxes are checked! 🚀**
```

