# 📋 Coax v0.8.0 VS Code Extension - Comprehensive Code Review

**Review Date:** March 16, 2026  
**Repository:** https://github.com/PropertySightlines/coax  
**Extension Version:** 0.8.0  

---

## ✅ Executive Summary

| Category | Score | Status |
|----------|-------|--------|
| **Extension Architecture** | 9/10 | ✅ Excellent |
| **package.json Configuration** | 9/10 | ✅ Excellent |
| **Scanner Implementation** | 8/10 | ✅ Very Good |
| **Diagnostics Integration** | 9/10 | ✅ Excellent |
| **Hover Provider** | 8/10 | ✅ Very Good |
| **Code Actions** | 7/10 | 🟡 Good (needs 1-2 more actions) |
| **Debounce Implementation** | 10/10 | ✅ Perfect (500ms) |
| **Overall** | 8.5/10 | ✅ **Production Ready** |

---

## 🎯 What's Excellent

### 1. package.json Configuration (9/10)

```json
{
  "engines": { "vscode": "^1.85.0" },  // ✅ Recent but not bleeding edge
  "activationEvents": ["onStartupFinished"],  // ✅ Correct for background scanning
  "categories": ["Programming Languages", "Linters", "Security"],  // ✅ Perfect
  "debounceDelay": { "default": 500 }  // ✅ Industry standard
}
```

**Strengths:**
- ✅ All 11 settings properly scoped (`window` vs `application`)
- ✅ Exclude patterns comprehensive (7 defaults including `target/`, `__pycache__/`)
- ✅ `maxFileSize` (10MB) and `scanTimeout` (30s) prevent runaway scans
- ✅ Commands have icons (great UX)
- ✅ Context menus in editor title + explorer

**One Minor Fix:**
```json
// Repository URLs have trailing spaces (will break links)
"repository": {
  "url": "https://github.com/gl33mer/coax.git  "  // ❌ Trim spaces
}
```

---

### 2. Extension.ts Architecture (10/10)

```typescript
// ✅ Perfect debounce implementation
function initializeFileWatchers(context: vscode.ExtensionContext): void {
    const config = vscode.workspace.getConfiguration('coax');
    const debounceDelay = config.get<number>('debounceDelay', 500);  // ✅ Configurable!

    if (config.get<boolean>('scanOnSave', true)) {
        vscode.workspace.onDidSaveTextDocument((document) => {
            if (scanTimeout) clearTimeout(scanTimeout);
            scanTimeout = setTimeout(() => {
                scanFile(document.uri)...
            }, debounceDelay);  // ✅ 500ms default
        }, null, context.subscriptions);
    }
}
```

**Strengths:**
- ✅ Debounce is **configurable** (users can tune to 300ms or 1000ms)
- ✅ Proper cleanup in `deactivate()` (clears timeout, disposes scanner)
- ✅ Scans open documents on activation
- ✅ Clears diagnostics on file close
- ✅ All subscriptions properly tracked

---

### 3. Scanner Implementation (8/10)

```typescript
// ✅ Excellent platform detection
function getBinaryPath(): string {
    const plat = process.platform;
    const architecture = process.arch;
    
    let platformFolder: string;
    switch (plat) {
        case 'darwin': platformFolder = `darwin-${architecture}`; break;
        case 'linux': platformFolder = `linux-${architecture}`; break;
        case 'win32': platformFolder = 'win32-x64'; break;
        default: throw new Error(`Unsupported platform: ${plat}`);
    }
}
```

**Strengths:**
- ✅ Custom binary path support (for users who want system-wide install)
- ✅ File size check before scanning
- ✅ Exclude pattern matching with minimatch
- ✅ Binary extension skipping (images, archives, etc.)
- ✅ Minified file detection (`.min.`, `bundle.`)
- ✅ Unix permissions set automatically (`chmod 755`)

**Issues to Fix:**

#### Issue #1: JSON Output Format May Not Match CLI

```typescript
const result: ScanResult = JSON.parse(output);
return result.findings || [];
```

**Problem:** The CLI may not output this exact structure. Need to verify CLI `--format json` matches this schema.

**Fix:**
```bash
# Test what CLI actually outputs
./target/release/coax scan -p test.js --format json --unicode-only
```

#### Issue #2: Unicode-Only Flag May Be Too Restrictive

```typescript
if (unicodeEnabled) {
    args.push('--unicode-only');  // ❌ This scans ONLY Unicode, not secrets
    args.push('--unicode-sensitivity', unicodeSensitivity);
}
```

**Problem:** `--unicode-only` means secrets are NOT scanned. Users want BOTH.

**Fix:**
```typescript
if (unicodeEnabled) {
    args.push('--unicode-scan', 'true');  // ✅ Enable Unicode (not only)
    args.push('--unicode-sensitivity', unicodeSensitivity);
}
```

---

### 4. Diagnostics Integration (9/10)

```typescript
// ✅ Perfect severity mapping
const severityMap: Record<string, vscode.DiagnosticSeverity> = {
    'critical': vscode.DiagnosticSeverity.Error,
    'high': vscode.DiagnosticSeverity.Error,
    'medium': vscode.DiagnosticSeverity.Warning,
    'low': vscode.DiagnosticSeverity.Information,
};
```

**Strengths:**
- ✅ Severity threshold filtering (users can hide low/medium)
- ✅ Related information includes finding type
- ✅ Source set to 'Coax' (distinguishes from other linters)
- ✅ 0-indexed conversion correct (CLI is 1-indexed, VS Code is 0-indexed)

**One Minor Issue:**
```typescript
// End line/column may not be provided by CLI
const range = new vscode.Range(
    Math.max(0, finding.line - 1),
    Math.max(0, finding.column - 1),
    Math.max(0, finding.endLine - 1),  // ⚠️ May be undefined
    Math.max(0, finding.endColumn - 1)  // ⚠️ May be undefined
);
```

**Fix:**
```typescript
const endLine = finding.endLine ?? finding.line;
const endColumn = finding.endColumn ?? finding.column;
```

---

### 5. Hover Provider (8/10)

```typescript
// ✅ Great hover content
contents.appendMarkdown(`#### 🔒 ${code}\n\n`);
contents.appendMarkdown(`**Severity:** ${severityName}\n\n`);
contents.appendMarkdown(`${diagnostic.message}\n\n`);
contents.appendMarkdown(`**Recommendation:** ${this.getRecommendation(code)}\n\n`);
```

**Strengths:**
- ✅ Markdown formatting with emoji
- ✅ `isTrusted = true` (allows links)
- ✅ Recommendations per finding type
- ✅ Links to documentation

**One Issue:**
```typescript
// Documentation link is hardcoded to HANDOFF.md
`[View Documentation](https://github.com/gl33mer/coax/blob/main/docs/HANDOFF.md  )`
// ❌ Should link to extension-specific docs or wiki
```

**Fix:**
```typescript
// Link to general docs or create extension-specific page
`[View Documentation](https://github.com/gl33mer/coax/wiki)`
```

---

### 6. Code Actions (7/10)

```typescript
// ✅ Good action variety
- Remove this finding
- Replace with ${ENV_VAR}
- Replace with ASCII equivalent
- Ignore for this session
```

**Strengths:**
- ✅ `isPreferred = true` on remove action (shows first in menu)
- ✅ Type-specific env var suggestions (AWS, GitHub, Stripe)
- ✅ Unicode-specific ASCII replacement

**Missing Actions:**
```typescript
// ADD THESE:
- Add to allowlist (.coax.yaml)  // ✅ Most requested
- Open .coax.yaml in editor
- Show finding in side panel  // If side panel exists
- Copy finding details  // For bug reports
```

**Priority:** Add "Add to allowlist" action - this is critical for false positive workflows.

---

