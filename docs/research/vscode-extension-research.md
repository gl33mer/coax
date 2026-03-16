# VS Code Extension Research Summary

**Date:** 2026-03-16  
**Purpose:** Inform Coax v0.8.0 VS Code Extension development  
**Research Scope:** Architecture, APIs, competitive analysis, best practices

---

## 1. VS Code Extension Architecture Overview

### Extension Model

VS Code extensions follow a **hosted process model**:

```
┌─────────────────────────────────────────────────────────┐
│                    VS Code Main Process                  │
│  ┌────────────────────────────────────────────────────┐ │
│  │           Extension Host (Utility Process)          │ │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────┐ │ │
│  │  │  Extension   │  │  Extension   │  │ Extension│ │ │
│  │  │  (Coax)      │  │  (GitGuardian)│  │ (Snyk)   │ │ │
│  │  └──────────────┘  └──────────────┘  └──────────┘ │ │
│  └────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
                          │
                          │ Extension API
                          ▼
┌─────────────────────────────────────────────────────────┐
│                    VS Code Renderer                      │
│  • Editor UI      • Problems Panel    • Status Bar      │
│  • Diagnostics    • Command Palette   • Hover Tooltips  │
└─────────────────────────────────────────────────────────┘
```

### Key Architectural Patterns

| Pattern | Description | Use Case |
|---------|-------------|----------|
| **Activation Events** | Extensions load on-demand | `onLanguage`, `onCommand`, `onStartupFinished` |
| **Disposable Resources** | All subscriptions tracked | Prevent memory leaks |
| **Diagnostic Collections** | Centralized issue tracking | Squiggly underlines, Problems panel |
| **Code Action Providers** | Quick-fix registration | Lightbulb suggestions |
| **File System Watchers** | Monitor file changes | Real-time scanning |

---

## 2. Key VS Code APIs for Coax

### 2.1 Diagnostic Collection (Core)

**Purpose:** Display squiggly underlines and Problems panel entries

```typescript
// Create collection
const diagnosticCollection = vscode.languages.createDiagnosticCollection('coax');

// Create diagnostics
const diagnostics: vscode.Diagnostic[] = [];
const range = new vscode.Range(line, column, endLine, endColumn);
const diagnostic = new vscode.Diagnostic(
    range,
    'AWS Access Key detected',
    vscode.DiagnosticSeverity.Error  // Error, Warning, Information, Hint
);
diagnostic.code = 'AWS_ACCESS_KEY';  // Links to code actions
diagnostic.source = 'Coax';

// Apply to file
diagnosticCollection.set(document.uri, diagnostics);

// Clear on close
diagnosticCollection.delete(document.uri);
```

**Severity Mapping:**

| Severity | Squiggly Color | Problems Icon | Use Case |
|----------|---------------|---------------|----------|
| `Error` | 🔴 Red | ❌ | Critical secrets (AWS keys, private keys) |
| `Warning` | 🟡 Yellow | ⚠️ | High-risk (GitHub tokens, Stripe keys) |
| `Information` | 🔵 Blue | ℹ️ | Medium-risk (generic secrets) |
| `Hint` | 🟢 Green | 💡 | Low-risk (potential secrets) |

**API Reference:** `vscode.languages.createDiagnosticCollection()`

---

### 2.2 File System Watcher

**Purpose:** Trigger scans on file changes

```typescript
// Create watcher with pattern
const pattern = new vscode.RelativePattern(workspaceFolder, '**/*.{rs,py,js,ts,yml,yaml,json,env}');
const watcher = vscode.workspace.createFileSystemWatcher(pattern);

// Register event handlers
watcher.onDidCreate(uri => scanFile(uri));
watcher.onDidChange(uri => scanFile(uri));
watcher.onDidDelete(uri => diagnosticCollection.delete(uri));

// Filter events (optional)
const watcher = vscode.workspace.createFileSystemWatcher(
    pattern,
    false,  // ignoreCreateEvents
    false,  // ignoreChangeEvents
    false   // ignoreDeleteEvents
);

// Cleanup
context.subscriptions.push(watcher);
```

**Best Practices:**
- Use `RelativePattern` for precision
- Filter by file extensions to reduce overhead
- Dispose watchers when no longer needed
- Consider `files.watcherExclude` user settings

**API Reference:** `vscode.workspace.createFileSystemWatcher()`

---

### 2.3 Code Actions Provider (Quick Fixes)

**Purpose:** Provide lightbulb quick-fix actions

```typescript
class CoaxCodeActionProvider implements vscode.CodeActionProvider {
    public static readonly providedKind = [
        vscode.CodeActionKind.QuickFix,
        vscode.CodeActionKind.Refactor
    ];

    public provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range | vscode.Selection,
        context: vscode.CodeActionContext,
        token: vscode.CancellationToken
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];

        // Create quick fix for each diagnostic
        for (const diagnostic of context.diagnostics) {
            if (diagnostic.code === 'AWS_ACCESS_KEY') {
                const action = new vscode.CodeAction(
                    'Remove AWS Access Key',
                    vscode.CodeActionKind.QuickFix
                );
                
                action.edit = new vscode.WorkspaceEdit();
                action.edit.replace(document.uri, diagnostic.range, '[REDACTED]');
                action.diagnostics = [diagnostic];
                action.isPreferred = true;  // Auto-fix candidate
                
                actions.push(action);
            }
            
            // Add "Ignore for session" action
            const ignoreAction = new vscode.CodeAction(
                'Ignore for this session',
                vscode.CodeActionKind.QuickFix
            );
            ignoreAction.command = {
                command: 'coax.ignoreFinding',
                title: 'Ignore Finding',
                arguments: [document.uri, diagnostic.range]
            };
            actions.push(ignoreAction);
        }

        return actions;
    }
}

// Register provider
context.subscriptions.push(
    vscode.languages.registerCodeActionsProvider(
        { language: '*', scheme: '*' },  // All languages
        new CoaxCodeActionProvider(),
        { providedCodeActionKinds: CoaxCodeActionProvider.providedKind }
    )
);
```

**Code Action Kinds:**
- `vscode.CodeActionKind.QuickFix` - Bug fixes
- `vscode.CodeActionKind.Refactor` - Code restructuring
- `vscode.CodeActionKind.Source` - Source actions
- `vscode.CodeActionKind.SourceFixAll` - Fix all issues

**API Reference:** `vscode.languages.registerCodeActionsProvider()`

---

### 2.4 Hover Provider

**Purpose:** Show detailed info on hover

```typescript
class CoaxHoverProvider implements vscode.HoverProvider {
    public provideHover(
        document: vscode.TextDocument,
        position: vscode.Position,
        token: vscode.CancellationToken
    ): vscode.Hover | undefined {
        const range = new vscode.Range(position.line, position.character, position.line, position.character + 1);
        
        // Check if hovering over a finding
        const finding = getFindingAtPosition(document.uri, position);
        
        if (finding) {
            const contents = new vscode.MarkdownString();
            contents.appendMarkdown(`#### 🔒 ${finding.type}`);
            contents.appendMarkdown(`\n\n**Severity:** ${finding.severity}`);
            contents.appendMarkdown(`\n\n**Description:** ${finding.description}`);
            contents.appendMarkdown(`\n\n**Recommendation:** ${finding.recommendation}`);
            contents.appendMarkdown(`\n\n[Documentation](https://coax.dev/docs/${finding.type})`);
            
            return new vscode.Hover(contents);
        }
        
        return undefined;
    }
}

// Register
context.subscriptions.push(
    vscode.languages.registerHoverProvider(
        { language: '*', scheme: '*' },
        new CoaxHoverProvider()
    )
);
```

**API Reference:** `vscode.languages.registerHoverProvider()`

---

### 2.5 Status Bar Item

**Purpose:** Display scan status and quick access

```typescript
// Create status bar item
const statusBarItem = vscode.window.createStatusBarItem(
    vscode.StatusBarAlignment.Right,
    100  // Priority (higher = more left)
);

// Configure appearance
statusBarItem.text = '$(shield) Coax: Secure';
statusBarItem.tooltip = 'Coax Security Scanner - No issues found';
statusBarItem.command = 'coax.showFindings';
statusBarItem.color = undefined;  // Use theme color
statusBarItem.backgroundColor = undefined;
statusBarItem.show();

// Update on scan results
function updateStatusBar(findings: number, severity: string) {
    if (findings === 0) {
        statusBarItem.text = '$(shield) Coax: Secure';
        statusBarItem.tooltip = 'No security issues found';
        statusBarItem.color = undefined;
    } else {
        const icon = severity === 'critical' ? '$(error)' : '$(warning)';
        statusBarItem.text = `${icon} Coax: ${findings} issue${findings > 1 ? 's' : ''}`;
        statusBarItem.tooltip = `Found ${findings} security issue(s). Click to view.`;
        statusBarItem.color = severity === 'critical' ? 'red' : 'yellow';
    }
}

// Cleanup
statusBarItem.dispose();
```

**Status Bar Alignment:**
- `vscode.StatusBarAlignment.Left` - Left side (after workspace name)
- `vscode.StatusBarAlignment.Right` - Right side (before notifications)

**Codicons:** `$(shield)`, `$(error)`, `$(warning)`, `$(check)`, `$(sync~spin)`

**API Reference:** `vscode.window.createStatusBarItem()`

---

### 2.6 Command Palette

**Purpose:** Register user commands

```typescript
// Register command
context.subscriptions.push(
    vscode.commands.registerCommand(
        'coax.scanCurrentFile',
        async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('No active editor');
                return;
            }
            
            const findings = await scanDocument(editor.document);
            displayFindings(findings);
            
            vscode.window.showInformationMessage(
                `Scan complete: ${findings.length} issue${findings.length !== 1 ? 's' : ''} found`
            );
        }
    )
);

context.subscriptions.push(
    vscode.commands.registerCommand(
        'coax.scanWorkspace',
        async () => {
            vscode.window.withProgress(
                {
                    location: vscode.ProgressLocation.Notification,
                    title: 'Scanning workspace...',
                    cancellable: true
                },
                async (progress, token) => {
                    // Perform workspace scan
                }
            );
        }
    )
);

context.subscriptions.push(
    vscode.commands.registerCommand(
        'coax.showFindings',
        () => {
            // Open findings panel/webview
        }
    )
);

context.subscriptions.push(
    vscode.commands.registerCommand(
        'coax.settings',
        () => {
            vscode.commands.executeCommand('workbench.action.openSettings', 'Coax');
        }
    )
);
```

**Show Messages:**
- `vscode.window.showInformationMessage()` - Blue info icon
- `vscode.window.showWarningMessage()` - Yellow warning icon
- `vscode.window.showErrorMessage()` - Red error icon

**API Reference:** `vscode.commands.registerCommand()`

---

## 3. Competitive Analysis

### 3.1 GitGuardian VS Code Extension

**Architecture:**
```
┌─────────────────────────────────────────────────────────┐
│                    VS Code Extension                     │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────┐  │
│  │   UI Layer  │  │  Command    │  │  Configuration  │  │
│  │  (Diags,    │◄─┤  Palette    │◄─┤    Settings     │  │
│  │   Status)   │  │  Handler    │  │   (settings.json)│ │
│  └──────┬──────┘  └─────────────┘  └─────────────────┘  │
│         │                                                │
│         ▼                                                │
│  ┌─────────────────────────────────────────────────────┐ │
│  │           Bundled ggshield CLI (Scanner)            │ │
│  │         • 500+ secret pattern detection             │ │
│  │         • Real-time scanning engine                 │ │
│  └─────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────┘
```

**Key Features:**
| Feature | Implementation |
|---------|---------------|
| Real-time detection | Scan on save |
| 500+ secret types | Pattern-based + entropy |
| Inline diagnostics | Squiggly underlines |
| Problems panel | Full integration |
| Hover tooltips | Secret type + remediation |
| Status bar | Warning icon + count |
| Configuration | `.gitguardian.yaml` |
| Authentication | OAuth flow (SaaS) or API key (self-hosted) |

**Technical Details:**
- **Bundled CLI:** `ggshield` included in extension package
- **Platform support:** win32, linux, darwin (x64/arm64)
- **Trigger:** On-save (not on-every-keystroke)
- **Performance:** Local scanning, no network latency for core detection

**Lessons for Coax:**
1. Bundle Coax CLI binary (same architecture)
2. Scan on save, not on every keystroke
3. Support both local-only and cloud-enhanced modes
4. Use `.coax.yaml` for configuration
5. Provide clear remediation guidance

**Source:** https://docs.gitguardian.com/ggshield-docs/integrations/ide-integrations/vscode

---

### 3.2 Snyk VS Code Extension

**Architecture:**
- **Open Source:** https://github.com/snyk/vscode-extension
- **Language:** TypeScript + bundled Snyk CLI
- **Scanning:** Vulnerabilities, secrets, IaC, dependencies
- **Display:** Tree view + inline diagnostics + Problems panel

**Key Features:**
| Feature | Implementation |
|---------|---------------|
| Multi-scan types | Vulnerabilities, secrets, IaC, licenses |
| Tree view | Dedicated Snyk panel in Activity Bar |
| Code actions | Fix vulnerability, ignore, open docs |
| Authentication | Snyk API token |
| CI/CD integration | Snyk Monitor on save |

**Lessons for Coax:**
1. Consider dedicated view container for findings
2. Support multiple scan types (secrets now, more later)
3. Provide "ignore" functionality with expiry
4. Link to documentation for each finding type

**Source:** https://github.com/snyk/vscode-extension

---

### 3.3 Comparison Table

| Feature | GitGuardian | Snyk | Coax (Planned) |
|---------|-------------|------|----------------|
| **Real-time scanning** | ✅ On save | ✅ On save | ✅ On save + on open |
| **Inline diagnostics** | ✅ Squiggly | ✅ Squiggly | ✅ Squiggly |
| **Problems panel** | ✅ | ✅ | ✅ |
| **Hover tooltips** | ✅ | ✅ | ✅ |
| **Quick fixes** | ⚠️ Limited | ✅ | ✅ Remove, replace, ignore |
| **Status bar** | ✅ | ✅ | ✅ |
| **Command palette** | ✅ | ✅ | ✅ |
| **Dedicated view** | ❌ | ✅ | ⏳ Phase 2 |
| **Binary bundling** | ✅ ggshield | ✅ Snyk CLI | ✅ Coax CLI |
| **Configuration file** | ✅ `.gitguardian.yaml` | ✅ `.snyk` | ✅ `.coax.yaml` |
| **Baseline files** | ❌ | ❌ | ✅ Planned |
| **Verification** | ✅ API-based | ⚠️ Limited | ✅ Planned |

---

## 4. Binary Bundling Strategy

### 4.1 File Structure

```
coax-vscode/
├── package.json
├── src/extension.ts
├── bundled/
│   ├── darwin-arm64/
│   │   └── coax
│   ├── darwin-x64/
│   │   └── coax
│   ├── linux-x64/
│   │   └── coax
│   ├── linux-arm64/
│   │   └── coax
│   └── win32-x64/
│       └── coax.exe
├── scripts/
│   └── build-binaries.sh
└── .vscodeignore
```

### 4.2 Platform Detection Code

```typescript
import * as path from 'path';
import { platform, arch } from 'os';

function getBinaryPath(context: vscode.ExtensionContext): string {
    const config = vscode.workspace.getConfiguration('coax');
    const customPath = config.get<string>('binaryPath', '');
    
    // Use custom path if provided
    if (customPath && fs.existsSync(customPath)) {
        return customPath;
    }
    
    // Use bundled binary
    const plat = platform();
    const architecture = arch();
    
    let platformFolder: string;
    switch (plat) {
        case 'darwin':
            platformFolder = `darwin-${architecture}`;
            break;
        case 'linux':
            platformFolder = `linux-${architecture}`;
            break;
        case 'win32':
            platformFolder = 'win32-x64';
            break;
        default:
            throw new Error(`Unsupported platform: ${plat}`);
    }
    
    const exeName = process.platform === 'win32' ? 'coax.exe' : 'coax';
    
    return path.join(context.extensionPath, 'bundled', platformFolder, exeName);
}
```

### 4.3 .vscodeignore Configuration

```json
{
  "src/**": true,
  "**/*.ts": true,
  "**/*.map": true,
  ".gitignore": true,
  "scripts/**": true,
  "test/**": true,
  "**/*.md": true,
  "!bundled/**/*",
  "!out/**/*",
  "!package.json",
  "!README.md",
  "!LICENSE",
  "!CHANGELOG.md"
}
```

### 4.4 Making Binary Executable

```typescript
// On extension activation
if (process.platform !== 'win32') {
    fs.chmodSync(binaryPath, '755');
}
```

### 4.5 Executing Binary

```typescript
import { execFile } from 'child_process';

function scanFile(filePath: string): Promise<ScanResult> {
    return new Promise((resolve, reject) => {
        const binaryPath = getBinaryPath(context);
        
        execFile(
            binaryPath,
            ['scan', 'secrets', '--path', filePath, '--format', 'json'],
            { timeout: 30000 },
            (error, stdout, stderr) => {
                if (error) {
                    reject(error);
                    return;
                }
                
                try {
                    const result = JSON.parse(stdout);
                    resolve(result);
                } catch (e) {
                    reject(new Error(`Failed to parse scan results: ${e}`));
                }
            }
        );
    });
}
```

---

## 5. Extension Manifest (package.json)

### 5.1 Complete Example

```json
{
  "name": "coax",
  "displayName": "Coax Security Scanner",
  "description": "Real-time Unicode confusable and secret detection for VS Code",
  "version": "0.8.0",
  "publisher": "coax-security",
  "engines": {
    "vscode": "^1.85.0"
  },
  "categories": [
    "Security",
    "Linters",
    "Programming Languages"
  ],
  "keywords": [
    "security",
    "secrets",
    "unicode",
    "homoglyph",
    "scanner",
    "vulnerability"
  ],
  "activationEvents": [
    "onStartupFinished"
  ],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "coax.scanCurrentFile",
        "title": "Scan Current File",
        "category": "Coax",
        "icon": "$(shield)"
      },
      {
        "command": "coax.scanWorkspace",
        "title": "Scan Workspace",
        "category": "Coax",
        "icon": "$(folder-opened)"
      },
      {
        "command": "coax.showFindings",
        "title": "Show Findings",
        "category": "Coax",
        "icon": "$(list-unordered)"
      },
      {
        "command": "coax.settings",
        "title": "Settings",
        "category": "Coax",
        "icon": "$(gear)"
      }
    ],
    "menus": {
      "editor/title": [
        {
          "command": "coax.scanCurrentFile",
          "group": "navigation",
          "when": "resourceSet"
        }
      ],
      "explorer/context": [
        {
          "command": "coax.scanCurrentFile",
          "group": "security",
          "when": "resourceSet"
        }
      ],
      "commandPalette": [
        {
          "command": "coax.scanCurrentFile"
        },
        {
          "command": "coax.scanWorkspace"
        },
        {
          "command": "coax.showFindings"
        },
        {
          "command": "coax.settings"
        }
      ]
    },
    "configuration": {
      "title": "Coax Security Scanner",
      "properties": {
        "coax.enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable Coax security scanning"
        },
        "coax.binaryPath": {
          "type": "string",
          "default": "",
          "description": "Custom path to Coax binary (leave empty to use bundled)"
        },
        "coax.scanOnSave": {
          "type": "boolean",
          "default": true,
          "description": "Automatically scan files on save"
        },
        "coax.scanOnOpen": {
          "type": "boolean",
          "default": true,
          "description": "Automatically scan files when opened"
        },
        "coax.severityThreshold": {
          "type": "string",
          "enum": ["critical", "high", "medium", "low"],
          "default": "medium",
          "description": "Minimum severity level to display"
        },
        "coax.enableVerification": {
          "type": "boolean",
          "default": false,
          "description": "Enable live secret verification (requires API access)"
        },
        "coax.ignoredPatterns": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "default": [],
          "description": "File patterns to ignore (glob syntax)"
        },
        "coax.configFile": {
          "type": "string",
          "default": ".coax.yaml",
          "description": "Path to Coax configuration file"
        }
      }
    },
    "languages": [
      {
        "id": "coax-findings",
        "aliases": ["Coax Findings"],
        "extensions": [".coax-findings"]
      }
    ]
  },
  "scripts": {
    "vscode:prepublish": "npm run compile",
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./",
    "package": "vsce package",
    "lint": "eslint src --ext ts"
  },
  "devDependencies": {
    "@types/node": "^20.10.0",
    "@types/vscode": "^1.85.0",
    "@typescript-eslint/eslint-plugin": "^6.13.0",
    "@typescript-eslint/parser": "^6.13.0",
    "@vscode/vsce": "^2.22.0",
    "eslint": "^8.54.0",
    "typescript": "^5.3.0"
  },
  "dependencies": {
    "node-fetch": "^2.7.0"
  },
  "icon": "images/coax-icon.png",
  "galleryBanner": {
    "color": "#1a1a2e",
    "theme": "dark"
  },
  "license": "MIT",
  "homepage": "https://coax.dev",
  "repository": {
    "type": "git",
    "url": "https://github.com/coax-security/coax-vscode"
  },
  "bugs": {
    "url": "https://github.com/coax-security/coax-vscode/issues"
  }
}
```

---

## 6. Estimated Development Effort

### 6.1 By Component

| Component | Effort | Complexity | Dependencies |
|-----------|--------|------------|--------------|
| **Project Setup** | 2-3 days | Low | None |
| **Binary Bundling** | 2-3 days | Medium | Coax CLI builds |
| **File Watcher** | 1-2 days | Low | None |
| **Diagnostic Collection** | 2-3 days | Low | File watcher |
| **Code Actions** | 2-3 days | Medium | Diagnostics |
| **Hover Provider** | 1 day | Low | Diagnostics |
| **Status Bar** | 1 day | Low | None |
| **Command Palette** | 1-2 days | Low | All above |
| **Configuration UI** | 2-3 days | Medium | None |
| **Testing** | 3-5 days | Medium | All above |
| **Documentation** | 2-3 days | Low | All above |
| **Total** | **17-26 days** | | |

### 6.2 By Phase

| Phase | Duration | Features |
|-------|----------|----------|
| **Phase 1: MVP** | 1 week | Project setup, binary bundling, file watcher, basic scanning |
| **Phase 2: Diagnostics** | 1 week | Diagnostic collection, Problems panel, severity colors |
| **Phase 3: Quick Fixes** | 1 week | Code actions, hover provider, status bar |
| **Phase 4: Polish** | 1 week | Command palette, settings, testing, documentation |
| **Total** | **4 weeks** | |

---

## 7. Technical Challenges & Solutions

### 7.1 Binary Bundling Complexity

**Challenge:** Bundle Coax CLI for multiple platforms (win32, linux, darwin x64/arm64)

**Solution:**
- Use platform-specific folders in `bundled/` directory
- Auto-detect platform at runtime
- Provide fallback to system `coax` binary
- Set executable permissions on Unix systems

**Risk:** Large extension package size (~50-100MB with all binaries)

**Mitigation:**
- Use `.vscodeignore` to exclude unnecessary files
- Consider separate extensions per platform (not recommended)
- Compress binaries (UPX for executables)

---

### 7.2 Performance Impact

**Challenge:** Scanning large files/folders without blocking UI

**Solution:**
- Scan on save, not on every keystroke
- Use debouncing for rapid saves
- Run scans in background thread (Node.js worker threads)
- Implement file size limits (skip files >10MB by default)
- Cache scan results for unchanged files

**Performance Targets:**
- Small file (<100KB): <500ms
- Medium file (100KB-1MB): <2s
- Large file (1MB-10MB): <10s
- Workspace scan: Progress indicator with cancellation

---

### 7.3 False Positive Noise

**Challenge:** Too many warnings annoy developers

**Solution:**
- Configurable severity threshold
- "Ignore for session" quick fix
- `.coax.yaml` for permanent ignores
- Baseline files for known findings
- Smart filtering (test files, fixtures, examples)

---

### 7.4 Binary Execution Failures

**Challenge:** Bundled binary fails to execute (permissions, missing libraries, etc.)

**Solution:**
- Graceful fallback to system `coax` command
- Clear error messages with troubleshooting steps
- Health check on extension activation
- Logging for debugging

---

## 8. Best Practices

### 8.1 Extension Development

1. **Use TypeScript** - Type safety, better IDE support
2. **Follow VS Code UI guidelines** - Consistent with native experience
3. **Respect user settings** - Allow customization
4. **Provide clear activation events** - Don't load unnecessarily
5. **Dispose resources properly** - Prevent memory leaks
6. **Use progress indicators** - For long-running operations
7. **Handle errors gracefully** - Show helpful messages
8. **Test across platforms** - Windows, macOS, Linux

### 8.2 Security Considerations

1. **Don't transmit secrets** - Keep scanning local
2. **Secure storage** - Use VS Code secrets API for tokens
3. **Validate binary integrity** - Check signatures if possible
4. **Minimal permissions** - Request only necessary capabilities
5. **Audit dependencies** - Regular security updates

---

## 9. References

### 9.1 Official Documentation

- [VS Code Extension API](https://code.visualstudio.com/api)
- [Programmatic Language Features](https://code.visualstudio.com/api/language-extensions/programmatic-language-features)
- [Contribution Points](https://code.visualstudio.com/api/references/contribution-points)
- [Extension Manifest](https://code.visualstudio.com/api/references/extension-manifest)
- [When Clause Contexts](https://code.visualstudio.com/api/references/when-clause-contexts)

### 9.2 Example Extensions

- [GitGuardian VS Code Extension](https://docs.gitguardian.com/ggshield-docs/integrations/ide-integrations/vscode)
- [Snyk VS Code Extension](https://github.com/snyk/vscode-extension)
- [VS Code Extension Samples](https://github.com/microsoft/vscode-extension-samples)
- [Code Actions Sample](https://github.com/microsoft/vscode-extension-samples/tree/main/code-actions-sample)

### 9.3 Technical References

- [File Watcher Internals](https://github.com/microsoft/vscode/wiki/File-Watcher-Internals)
- [Extension API Guidelines](https://github.com/microsoft/vscode/wiki/Extension-API-guidelines)
- [Bundling Binaries](https://stackoverflow.com/questions/78370139/how-do-i-bundle-an-executable-in-my-vscode-extension-and-use-it)

---

**Research Completed:** 2026-03-16  
**Next Step:** Create extension specification document
