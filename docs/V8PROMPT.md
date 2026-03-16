

```markdown
# 🎯 Coax v0.8.0 - VS Code Extension Development

**Mission:** Build a production-ready VS Code extension that brings Coax's Unicode + secret detection into the developer workflow.

**Repository:** https://github.com/gl33mer/coax
**Prerequisite:** v0.7.5 must be complete (script mixing fix verified)
**Timeline:** 4-5 weeks
**Target Release:** VS Code Marketplace

---

## 📚 Reference Documents

Read these FIRST:
1. `docs/VSCode-EXTENSION-SPEC.md` - Full technical specification
2. `docs/VSCode-EXTENSION-TIMELINE.md` - Week-by-week plan
3. `docs/HANDOFF.md` - Current project state (v0.7.5)

---

## 🔴 Must-Have Features (v0.8.0 Core)

### Week 1-2: Foundation
- [ ] Extension scaffold (`yo code`, TypeScript)
- [ ] Binary bundling (win32-x64, darwin-x64, darwin-arm64, linux-x64)
- [ ] File watcher (`onDidSaveTextDocument`, **500ms debounce**)
- [ ] CLI execution (`child_process.spawn`)
- [ ] JSON output parsing
- [ ] DiagnosticCollection integration
- [ ] Problems panel integration
- [ ] Settings schema (see below)
- [ ] **Exclude patterns** (node_modules/, vendor/, *.lock, etc.)

### Week 3: Real-time Feedback
- [ ] Inline squiggly underlines (DiagnosticSeverity mapping)
- [ ] Hover tooltips (full finding details)
- [ ] Status bar item (finding count + scan state)
- [ ] **Side panel with findings list** (TreeView API) - CRITICAL UX
- [ ] Progress indicator for long scans

### Week 4: Quick-Fixes
- [ ] Code action provider registration
- [ ] Remove/revoke secret action
- [ ] Replace with environment variable action
- [ ] Add to allowlist action (.coax.yaml)
- [ ] Ignore for session action
- [ ] View documentation action

### Week 5: Polish & Release
- [ ] Command palette (Coax: Scan Workspace, Coax: Clear Findings)
- [ ] Error handling (user-friendly messages)
- [ ] Output channel logging
- [ ] README.md (installation, configuration, troubleshooting)
- [ ] CHANGELOG.md
- [ ] VSIX packaging (`vsce package`)
- [ ] Marketplace submission (`vsce publish`)

---

## ⚙️ Settings Schema (package.json)

```json
"contributes": {
    "configuration": {
        "title": "Coax Security Scanner",
        "properties": {
            "coax.sensitivity": {
                "type": "string",
                "enum": ["low", "medium", "high", "critical"],
                "default": "high"
            },
            "coax.unicode.enabled": {
                "type": "boolean",
                "default": true
            },
            "coax.unicode.sensitivity": {
                "type": "string",
                "enum": ["low", "medium", "high", "critical"],
                "default": "high"
            },
            "coax.exclude": {
                "type": "array",
                "items": { "type": "string" },
                "default": [
                    "**/node_modules/**",
                    "**/vendor/**",
                    "**/*.lock",
                    "**/dist/**",
                    "**/build/**"
                ]
            },
            "coax.scanOnSave": {
                "type": "boolean",
                "default": true
            },
            "coax.scanOnOpen": {
                "type": "boolean",
                "default": false
            }
        }
    }
}
```

---

## 🔧 Technical Requirements

### Debounce Implementation (500ms)

```typescript
let scanTimeout: NodeJS.Timeout | null = null;

vscode.workspace.onDidSaveTextDocument((doc) => {
    if (!config.get('scanOnSave')) return;
    if (scanTimeout) clearTimeout(scanTimeout);
    
    scanTimeout = setTimeout(() => {
        runScan(doc);
    }, 500);  // 500ms - industry standard (ESLint, Prettier)
});
```

### Diagnostic Severity Mapping

| Coax Severity | VS Code DiagnosticSeverity | Color |
|--------------|---------------------------|-------|
| Critical | Error | 🔴 Red |
| High | Error | 🔴 Red |
| Medium | Warning | 🟡 Yellow |
| Low | Information | 🔵 Blue |

### Binary Bundling

```
extension/
├── binaries/
│   ├── coax-win32-x64.exe
│   ├── coax-darwin-x64
│   ├── coax-darwin-arm64
│   └── coax-linux-x64
├── src/
├── package.json
└── ...
```

**Selection at runtime:**
```typescript
const platform = process.platform;
const arch = process.arch;
const binary = `coax-${platform}-${arch}${platform === 'win32' ? '.exe' : ''}`;
```

---

## 🧪 Success Criteria

| Metric | Target |
|--------|--------|
| Install from Marketplace | ✅ No errors |
| Scan on save trigger | ✅ Within 500ms |
| Scan completion (100KB file) | ✅ <2s |
| All findings in Problems panel | ✅ Visible |
| Inline squiggles | ✅ Correct severity colors |
| Quick-fix actions | ✅ 5+ working |
| Status bar count | ✅ Accurate |
| Side panel findings | ✅ Complete list |
| Crash-free sessions | ✅ 100+ |

---

## 🚫 Out of Scope (v0.9.0+)

- Git hook integration (better as CLI: `coax install-git-hook`)
- Multi-root workspace support
- Remote development (SSH, Containers, WSL)
- Coax Cloud integration
- Real-time collaboration

---

## 📁 Key Files to Review

| File | Purpose |
|------|---------|
| `docs/VSCode-EXTENSION-SPEC.md` | Full specification (100+ sections) |
| `docs/VSCode-EXTENSION-TIMELINE.md` | Week-by-week timeline |
| `target/release/coax` | Binary to bundle (v0.7.5+) |
| `crates/coax-scanner/` | Rust scanner (for JSON output format) |

---

## ✅ Completion Checklist

- [ ] Extension installs from VSIX without errors
- [ ] Scan on save triggers correctly (500ms debounce)
- [ ] All findings display in Problems panel
- [ ] Inline squiggles with correct colors
- [ ] Side panel shows all findings (TreeView)
- [ ] Status bar shows accurate count
- [ ] 5+ quick-fix actions working
- [ ] Settings configurable in UI
- [ ] README.md complete
- [ ] Published to VS Code Marketplace

---

**Ready to begin? Start with:**
1. Read `docs/VSCode-EXTENSION-SPEC.md`
2. Initialize extension with `yo code`
3. Set up binary bundling structure
4. Implement file watcher with 500ms debounce

**First milestone:** Week 1 complete (foundation working) in 5-7 days.
```

---

