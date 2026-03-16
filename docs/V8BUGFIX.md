# 📋 Document 1: v0.8.1 Bug Fix Prompt for Qwen Code

```markdown
# 🎯 Coax v0.8.1 - Critical Bug Fixes

**Mission:** Fix 4 critical issues identified in code review before Marketplace submission.

**Repository:** https://github.com/PropertySightlines/coax
**Current Version:** 0.8.0
**Target Version:** 0.8.1
**Timeline:** 1-2 days

---

## 🚨 Issues to Fix

### Issue #1: Unicode Flag Too Restrictive (CRITICAL)

**File:** `coax-vscode/src/scanner/index.ts`

**Problem:** `--unicode-only` scans ONLY Unicode attacks, NOT secrets. Users want BOTH.

**Current Code:**
```typescript
if (unicodeEnabled) {
    args.push('--unicode-only');
    args.push('--unicode-sensitivity', unicodeSensitivity);
}
```

**Fix Required:**
```typescript
if (unicodeEnabled) {
    args.push('--unicode-scan', 'true');  // Enable Unicode + secrets
    args.push('--unicode-sensitivity', unicodeSensitivity);
}
```

**Verification:**
```bash
# After fix, this should scan BOTH secrets AND Unicode
./target/release/coax scan -p test.js --unicode-scan true
```

---

### Issue #2: Missing "Add to Allowlist" Code Action (HIGH)

**File:** `coax-vscode/src/actions/codeActions.ts`

**Problem:** Users cannot false-positive allowlist findings from the editor.

**Fix Required:** Add new code action:

```typescript
private createAllowlistAction(
    document: vscode.TextDocument,
    diagnostic: vscode.Diagnostic
): vscode.CodeAction {
    const action = new vscode.CodeAction(
        'Add to allowlist (.coax.yaml)',
        vscode.CodeActionKind.Refactor
    );

    action.command = {
        command: 'coax.addToAllowlist',
        title: 'Add to Allowlist',
        arguments: [document.uri, diagnostic],
    };

    return action;
}
```

**Also add command handler in `src/commands/index.ts`:**
```typescript
vscode.commands.registerCommand('coax.addToAllowlist', async (uri, diagnostic) => {
    // Open or create .coax.yaml in workspace root
    // Add finding pattern to allowlist section
    // Reload scanner
});
```

---

### Issue #3: Trailing Spaces in URLs (MEDIUM)

**File:** `coax-vscode/package.json`

**Problem:** URLs have trailing spaces - will break links in Marketplace.

**Current:**
```json
"repository": {
  "url": "https://github.com/gl33mer/coax.git  "
},
"homepage": "https://github.com/gl33mer/coax  ",
"bugs": {
  "url": "https://github.com/gl33mer/coax/issues  "
}
```

**Fix:**
```json
"repository": {
  "url": "https://github.com/gl33mer/coax.git"
},
"homepage": "https://github.com/gl33mer/coax",
"bugs": {
  "url": "https://github.com/gl33mer/coax/issues"
}
```

---

### Issue #4: Missing endLine/endColumn Handling (MEDIUM)

**File:** `coax-vscode/src/diagnostics/index.ts`

**Problem:** CLI may not provide endLine/endColumn - will crash if undefined.

**Current:**
```typescript
const range = new vscode.Range(
    Math.max(0, finding.line - 1),
    Math.max(0, finding.column - 1),
    Math.max(0, finding.endLine - 1),
    Math.max(0, finding.endColumn - 1)
);
```

**Fix:**
```typescript
const endLine = finding.endLine ?? finding.line;
const endColumn = finding.endColumn ?? finding.column;

const range = new vscode.Range(
    Math.max(0, finding.line - 1),
    Math.max(0, finding.column - 1),
    Math.max(0, endLine - 1),
    Math.max(0, endColumn - 1)
);
```

---

## 🧪 Verification Commands

```bash
# 1. Build extension
cd coax-vscode
npm install
npm run compile
npm run package

# 2. Install locally
code --install-extension coax-0.8.1.vsix

# 3. Test scan on save (500ms debounce)
# 4. Test all 5+ code actions appear
# 5. Test allowlist action creates .coax.yaml
# 6. Test hover tooltips work
# 7. Test status bar shows count
```

---

## ✅ Completion Checklist

- [ ] Issue #1 fixed (Unicode flag)
- [ ] Issue #2 fixed (Allowlist action)
- [ ] Issue #3 fixed (URL spaces)
- [ ] Issue #4 fixed (endLine/endColumn)
- [ ] All TypeScript compiles without errors
- [ ] Extension packages successfully
- [ ] Tested on Linux (you)
- [ ] Version bumped to 0.8.1 in package.json
- [ ] CHANGELOG.md updated
- [ ] Git commit: "v0.8.1 - Critical bug fixes"
- [ ] Git tag: v0.8.1

---

**Ready to begin? Start with Issue #1 (most critical), then proceed in order.**
```

