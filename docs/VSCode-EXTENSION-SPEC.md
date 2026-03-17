# Coax VS Code Extension Specification

**Version:** 0.8.0  
**Date:** 2026-03-16  
**Status:** Ready for Development  
**Priority:** P0 (Critical for Adoption)

---

## Overview

Real-time Unicode confusable character and secret detection integrated directly into the VS Code editor. The Coax VS Code Extension provides developers with immediate feedback on security issues as they code, preventing secrets and homoglyph attacks from entering the codebase.

### Value Proposition

| Benefit | Impact |
|---------|--------|
| **Prevention** | Stop secrets before commit |
| **Real-time feedback** | Immediate awareness of issues |
| **Developer-friendly** | Native VS Code experience |
| **Low friction** | Automatic scanning, easy fixes |
| **Enterprise-ready** | SARIF output, CI/CD integration |

---

## Features

### 1. Real-time Scanning

#### 1.1 Scan Triggers

| Trigger | Description | Configurable |
|---------|-------------|--------------|
| **File Save** | Scan when file is saved (`Ctrl+S`) | `coax.scanOnSave` (default: true) |
| **File Open** | Scan when file is opened | `coax.scanOnOpen` (default: true) |
| **Manual** | User-triggered via command | N/A |
| **Workspace** | Full workspace scan | N/A |

#### 1.2 Scanning Behavior

```typescript
// Pseudocode for scan flow
async function scanDocument(document: TextDocument): Promise<void> {
    // Check if enabled
    if (!config.get('coax.enabled')) return;
    
    // Check file type (skip binary, large files)
    if (shouldSkipFile(document)) return;
    
    // Check ignored patterns
    if (matchesIgnoredPattern(document.uri)) return;
    
    // Execute Coax CLI
    const results = await executeCoaxBinary(document.uri.fsPath);
    
    // Update diagnostics
    updateDiagnostics(document.uri, results);
    
    // Update status bar
    updateStatusBar(results);
}
```

#### 1.3 File Type Filtering

**Default Extensions Scanned:**
```
Source Code: .rs, .py, .js, .ts, .jsx, .tsx, .go, .java, .c, .cpp, .h, .hpp
Config: .yml, .yaml, .json, .toml, .ini, .cfg, .conf
Environment: .env, .env.*, .bashrc, .zshrc, .profile
Scripts: .sh, .bash, .zsh, .ps1
Web: .html, .css, .scss, .vue, .svelte
Data: .sql, .xml, .proto, .graphql
```

**Skip by Default:**
```
Binary: .bin, .exe, .dll, .so, .dylib, .pdf, .zip, .tar, .gz
Images: .png, .jpg, .jpeg, .gif, .bmp, .svg, .ico, .webp
Fonts: .ttf, .otf, .woff, .woff2
Lock files: package-lock.json, yarn.lock, Cargo.lock, Gemfile.lock
Minified: *.min.js, *.min.css
Generated: node_modules/, dist/, build/, target/, __pycache__/
```

---

### 2. Inline Warnings

#### 2.1 Squiggly Underlines

**Severity Colors:**

| Severity | Squiggly Color | Use Case |
|----------|---------------|----------|
| **Critical** | 🔴 Red (#FF0000) | AWS keys, private keys, database passwords |
| **High** | 🟠 Orange (#FFA500) | GitHub tokens, Stripe keys, JWT secrets |
| **Medium** | 🟡 Yellow (#FFFF00) | Generic secrets, API keys |
| **Low** | 🟢 Green (#00FF00) | Potential secrets, low-confidence detections |

**Implementation:**
```typescript
const severityMap = {
    'critical': vscode.DiagnosticSeverity.Error,
    'high': vscode.DiagnosticSeverity.Warning,
    'medium': vscode.DiagnosticSeverity.Information,
    'low': vscode.DiagnosticSeverity.Hint,
};

const diagnostic = new vscode.Diagnostic(
    range,
    message,
    severityMap[finding.severity]
);
```

#### 2.2 Hover Tooltips

**Content Structure:**
```markdown
#### 🔒 [Finding Type]

**Severity:** [Critical/High/Medium/Low]

**Description:** [Detailed explanation]

**Location:** [File:line:column]

**Recommendation:** [Actionable remediation steps]

**Documentation:** [Link to coax.dev/docs/finding-type]

**Quick Actions:**
- Remove secret
- Replace with environment variable
- Ignore for this session
- Add to allowlist
```

**Example:**
```markdown
#### 🔒 AWS Access Key

**Severity:** Critical

**Description:** AWS Access Key ID detected. This key provides programmatic 
access to AWS services and should never be committed to version control.

**Location:** config.yml:5:12

**Recommendation:** 
1. Remove this key immediately
2. Rotate the key via AWS IAM Console
3. Use IAM roles or environment variables instead
4. Scan git history for previous exposures

**Documentation:** https://coax.dev/docs/aws-access-key

**Quick Actions:**
- Remove secret
- Replace with ${AWS_ACCESS_KEY_ID}
- Ignore for this session
```

#### 2.3 Gutter Icons

**Implementation:** VS Code automatically shows warning/error icons in the gutter 
(left margin) next to lines with diagnostics.

**Icon Mapping:**
- Error (Critical): Red circle with X
- Warning (High): Yellow triangle with !
- Information (Medium): Blue circle with i
- Hint (Low): Green checkmark

---

### 3. Problems Panel Integration

#### 3.1 Automatic Integration

Diagnostics automatically appear in VS Code Problems panel (`Ctrl+Shift+M`).

**Panel Columns:**
| Column | Content |
|--------|---------|
| Severity | Icon (error/warning/info) |
| Description | Finding type + brief message |
| Source | "Coax" |
| File | File path |
| Line | Line number |
| Column | Column number |

#### 3.2 Filtering

**Built-in VS Code Filters:**
- Filter by severity (Error, Warning, Info)
- Filter by file
- Filter by text search

**Coax-Specific Filters (Future):**
- Filter by finding type (AWS, GitHub, Unicode, etc.)
- Filter by verification status
- Filter by confidence score

#### 3.3 Grouping

**Default Grouping:** By file

**Future Grouping Options:**
- By severity
- By finding type
- By verification status

---

### 4. Quick-Fix Actions

#### 4.1 Available Actions

| Action | Description | Finding Types |
|--------|-------------|---------------|
| **Remove invisible character** | Delete zero-width/joiner characters | Unicode confusables |
| **Replace homoglyph with ASCII** | Convert to visually similar ASCII | Unicode confusables |
| **Remove secret** | Delete the detected secret | All secrets |
| **Replace with env var** | Convert to environment variable reference | API keys, tokens |
| **Add to allowlist** | Add to `.coax.yaml` ignore list | Any finding |
| **Ignore for this session** | Temporarily suppress | Any finding |
| **Rotate secret** | Open rotation documentation | AWS, GitHub, Stripe |
| **View documentation** | Open finding-specific docs | All findings |

#### 4.2 Implementation

```typescript
class CoaxCodeActionProvider implements vscode.CodeActionProvider {
    public provideCodeActions(
        document: vscode.TextDocument,
        range: vscode.Range,
        context: vscode.CodeActionContext
    ): vscode.CodeAction[] {
        const actions: vscode.CodeAction[] = [];
        
        for (const diagnostic of context.diagnostics) {
            // Remove secret action
            const removeAction = new vscode.CodeAction(
                'Remove secret',
                vscode.CodeActionKind.QuickFix
            );
            removeAction.edit = new vscode.WorkspaceEdit();
            removeAction.edit.replace(document.uri, diagnostic.range, '');
            removeAction.diagnostics = [diagnostic];
            removeAction.isPreferred = true;
            actions.push(removeAction);
            
            // Replace with env var action
            if (diagnostic.code === 'AWS_ACCESS_KEY') {
                const envAction = new vscode.CodeAction(
                    'Replace with ${AWS_ACCESS_KEY_ID}',
                    vscode.CodeActionKind.Refactor
                );
                envAction.edit = new vscode.WorkspaceEdit();
                envAction.edit.replace(
                    document.uri, 
                    diagnostic.range, 
                    '${AWS_ACCESS_KEY_ID}'
                );
                actions.push(envAction);
            }
            
            // Ignore for session action
            const ignoreAction = new vscode.CodeAction(
                'Ignore for this session',
                vscode.CodeActionKind.QuickFix
            );
            ignoreAction.command = {
                command: 'coax.ignoreFinding',
                title: 'Ignore Finding',
                arguments: [document.uri, diagnostic.range, diagnostic.code]
            };
            actions.push(ignoreAction);
        }
        
        return actions;
    }
}
```

#### 4.3 Auto-Fix on Save (Future)

```typescript
// Configuration option
{
  "coax.autoFixOnSave": false  // Default: false
}

// When enabled, automatically apply preferred quick fixes
async function applyAutoFix(document: TextDocument): Promise<void> {
    const actions = await getCodeActions(document);
    const preferredActions = actions.filter(a => a.isPreferred);
    
    for (const action of preferredActions) {
        if (action.edit) {
            await vscode.workspace.applyEdit(action.edit);
        }
    }
}
```

---

### 5. Status Bar Indicator

#### 5.1 Display States

| State | Text | Icon | Color | Tooltip |
|-------|------|------|-------|---------|
| **Idle** | `$(shield) Coax: Secure` | Shield | Default | No security issues found |
| **Critical** | `$(error) Coax: 3 issues` | Error | Red | Click to view critical issues |
| **Warning** | `$(warning) Coax: 5 issues` | Warning | Yellow | Click to view issues |
| **Scanning** | `$(sync~spin) Coax: Scanning...` | Sync (spinning) | Default | Scanning in progress... |
| **Disabled** | `$(shield) Coax: Disabled` | Shield | Gray | Click to enable |
| **Error** | `$(alert) Coax: Error` | Alert | Red | Click for details |

#### 5.2 Click Actions

**Default Click:** Open findings panel / show problems

**Right-Click Menu:**
- Scan Current File
- Scan Workspace
- Show Findings
- Settings
- Enable/Disable
- About Coax

#### 5.3 Implementation

```typescript
let statusBarItem: vscode.StatusBarItem;

function initializeStatusBar(context: vscode.ExtensionContext): void {
    statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );
    statusBarItem.command = 'coax.showFindings';
    statusBarItem.tooltip = 'Coax Security Scanner';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);
}

function updateStatusBar(findings: Finding[]): void {
    const criticalCount = findings.filter(f => f.severity === 'critical').length;
    const highCount = findings.filter(f => f.severity === 'high').length;
    const totalCount = findings.length;
    
    if (totalCount === 0) {
        statusBarItem.text = '$(shield) Coax: Secure';
        statusBarItem.tooltip = 'No security issues found';
        statusBarItem.color = undefined;
    } else if (criticalCount > 0) {
        statusBarItem.text = `$(error) Coax: ${totalCount} issue${totalCount > 1 ? 's' : ''}`;
        statusBarItem.tooltip = `${criticalCount} critical, ${highCount} high severity issues`;
        statusBarItem.color = 'red';
    } else if (highCount > 0) {
        statusBarItem.text = `$(warning) Coax: ${totalCount} issue${totalCount > 1 ? 's' : ''}`;
        statusBarItem.tooltip = `${highCount} high severity issues`;
        statusBarItem.color = 'yellow';
    }
}
```

---

### 6. Command Palette

#### 6.1 Registered Commands

| Command ID | Title | Description | Icon |
|------------|-------|-------------|------|
| `coax.scanCurrentFile` | Scan Current File | Scan active file for secrets | `$(shield)` |
| `coax.scanWorkspace` | Scan Workspace | Scan entire workspace | `$(folder-opened)` |
| `coax.showFindings` | Show Findings | Display all current findings | `$(list-unordered)` |
| `coax.settings` | Settings | Open Coax settings | `$(gear)` |
| `coax.ignoreFinding` | Ignore Finding | Ignore specific finding | `$(mute)` |
| `coax.addToAllowlist` | Add to Allowlist | Add pattern to allowlist | `$(checklist)` |
| `coax.exportResults` | Export Results | Export findings to file | `$(download)` |
| `coax.about` | About Coax | Show extension info | `$(info)` |

#### 6.2 Menu Placements

**Editor Title Menu:**
```json
{
  "command": "coax.scanCurrentFile",
  "group": "navigation",
  "when": "resourceSet"
}
```

**Explorer Context Menu:**
```json
{
  "command": "coax.scanCurrentFile",
  "group": "security",
  "when": "resourceSet"
}
```

**Command Palette:**
- All commands visible
- Categorized under "Coax"

---

## Architecture

### High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         VS Code Extension                        │
│                                                                  │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    Extension Core                         │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │ File Watcher │  │   Scanner    │  │   Diagnostic   │   │   │
│  │  │   Service    │◄─┤   Service    │◄─┤  Collection    │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │   Code       │  │    Hover     │  │   Status     │   │   │
│  │  │  Actions     │  │   Provider   │  │    Bar       │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                              │                                   │
│                              ▼                                   │
│  ┌──────────────────────────────────────────────────────────┐   │
│  │                    Coax CLI Binary                        │   │
│  │                   (bundled executable)                    │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│  │  │   Pattern    │  │   Entropy    │  │   Unicode    │   │   │
│  │  │  Detection   │  │   Detection  │  │  Detection   │   │   │
│  │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│  └──────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
                               │
                               ▼
┌─────────────────────────────────────────────────────────────────┐
│                         VS Code APIs                             │
│  • Diagnostics    • Problems Panel    • Command Palette         │
│  • Code Actions   • Status Bar        • File Watchers           │
│  • Hover Provider • Configuration     • Progress Indicators     │
└─────────────────────────────────────────────────────────────────┘
```

### Component Responsibilities

| Component | Responsibility |
|-----------|---------------|
| **File Watcher Service** | Monitor file changes, trigger scans |
| **Scanner Service** | Execute Coax CLI, parse results |
| **Diagnostic Collection** | Manage squiggly underlines, Problems panel |
| **Code Actions Provider** | Provide quick-fix actions |
| **Hover Provider** | Show detailed info on hover |
| **Status Bar** | Display scan status, finding count |
| **Command Handler** | Process user commands |
| **Configuration Manager** | Load/save settings, `.coax.yaml` |

### Data Flow

```
File Change (Save/Open)
        │
        ▼
┌──────────────────┐
│ File Watcher     │
│ (vscode API)     │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Scanner Service  │
│ (execute Coax)   │
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Parse JSON Output│
└────────┬─────────┘
         │
         ▼
┌──────────────────┐
│ Update           │
│ Diagnostics      │
└────────┬─────────┘
         │
         ├──────────────┬──────────────┬──────────────┐
         ▼              ▼              ▼              ▼
┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌─────────────┐
│ Squiggly    │ │ Problems    │ │ Status Bar  │ │ Code        │
│ Underlines  │ │ Panel       │ │ Indicator   │ │ Actions     │
└─────────────┘ └─────────────┘ └─────────────┘ └─────────────┘
```

---

## Technical Requirements

### Extension Manifest (package.json)

#### Activation Events

```json
{
  "activationEvents": [
    "onStartupFinished"
  ]
}
```

**Rationale:** Activate after VS Code finishes startup to avoid slowing initial load.

#### Commands

```json
{
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
    ]
  }
}
```

#### Configuration Settings

```json
{
  "contributes": {
    "configuration": {
      "title": "Coax Security Scanner",
      "properties": {
        "coax.enabled": {
          "type": "boolean",
          "default": true,
          "description": "Enable Coax security scanning",
          "scope": "window"
        },
        "coax.binaryPath": {
          "type": "string",
          "default": "",
          "description": "Custom path to Coax binary (leave empty to use bundled)",
          "scope": "application"
        },
        "coax.scanOnSave": {
          "type": "boolean",
          "default": true,
          "description": "Automatically scan files on save",
          "scope": "window"
        },
        "coax.scanOnOpen": {
          "type": "boolean",
          "default": true,
          "description": "Automatically scan files when opened",
          "scope": "window"
        },
        "coax.severityThreshold": {
          "type": "string",
          "enum": ["critical", "high", "medium", "low", "none"],
          "default": "medium",
          "description": "Minimum severity level to display",
          "scope": "window"
        },
        "coax.enableVerification": {
          "type": "boolean",
          "default": false,
          "description": "Enable live secret verification (requires API access)",
          "scope": "window"
        },
        "coax.ignoredPatterns": {
          "type": "array",
          "items": {
            "type": "string"
          },
          "default": ["**/test/**", "**/fixtures/**", "**/*.test.*"],
          "description": "File patterns to ignore (glob syntax)",
          "scope": "window"
        },
        "coax.configFile": {
          "type": "string",
          "default": ".coax.yaml",
          "description": "Path to Coax configuration file",
          "scope": "window"
        },
        "coax.maxFileSize": {
          "type": "number",
          "default": 10485760,
          "description": "Maximum file size to scan in bytes (default: 10MB)",
          "scope": "window"
        },
        "coax.scanTimeout": {
          "type": "number",
          "default": 30000,
          "description": "Scan timeout in milliseconds (default: 30s)",
          "scope": "application"
        }
      }
    }
  }
}
```

### Dependencies

#### Runtime Dependencies

```json
{
  "dependencies": {
    "node-fetch": "^2.7.0"
  }
}
```

**Purpose:**
- `node-fetch`: HTTP client for future cloud integration (optional)

#### Development Dependencies

```json
{
  "devDependencies": {
    "@types/node": "^20.10.0",
    "@types/vscode": "^1.85.0",
    "@typescript-eslint/eslint-plugin": "^6.13.0",
    "@typescript-eslint/parser": "^6.13.0",
    "@vscode/vsce": "^2.22.0",
    "eslint": "^8.54.0",
    "typescript": "^5.3.0"
  }
}
```

**Purpose:**
- `@types/vscode`: VS Code API type definitions
- `@vscode/vsce`: Extension packaging tool
- `typescript`: TypeScript compiler
- `eslint`: Code linting

### Binary Bundling

#### Supported Platforms

| Platform | Architecture | Binary Name |
|----------|-------------|-------------|
| Windows | x64 | `coax.exe` |
| macOS | x64 | `coax` |
| macOS | arm64 | `coax` |
| Linux | x64 | `coax` |
| Linux | arm64 | `coax` |

#### Bundle Structure

```
bundled/
├── darwin-arm64/
│   └── coax
├── darwin-x64/
│   └── coax
├── linux-x64/
│   └── coax
├── linux-arm64/
│   └── coax
└── win32-x64/
    └── coax.exe
```

#### Platform Detection

```typescript
import { platform, arch } from 'os';
import * as path from 'path';

function getBinaryPath(context: ExtensionContext): string {
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

#### Fallback Strategy

```typescript
async function executeScan(filePath: string): Promise<ScanResult> {
    try {
        // Try bundled binary first
        const bundledPath = getBinaryPath(context);
        return await executeBinary(bundledPath, filePath);
    } catch (bundledError) {
        // Fallback to system coax
        try {
            return await executeBinary('coax', filePath);
        } catch (systemError) {
            throw new Error(
                'Coax binary not found. Please install Coax CLI or check extension settings.'
            );
        }
    }
}
```

---

## Development Phases

### Phase 1: MVP (Week 1-2)

**Goal:** Basic scanning functionality with Problems panel integration

**Features:**
- [ ] Project setup (TypeScript, VS Code extension template)
- [ ] Binary bundling (all platforms)
- [ ] File watcher (on save)
- [ ] CLI execution and result parsing
- [ ] Diagnostic collection
- [ ] Problems panel integration
- [ ] Basic configuration settings

**Success Criteria:**
- Extension installs and activates
- File save triggers scan
- Findings appear in Problems panel
- Zero crashes in basic testing

**Deliverables:**
- Working VSIX package
- Basic README
- Installation instructions

---

### Phase 2: Real-time Feedback (Week 3)

**Goal:** Inline warnings and status indicators

**Features:**
- [ ] Squiggly underlines with severity colors
- [ ] Hover tooltips with detailed information
- [ ] Status bar indicator
- [ ] Scan on file open
- [ ] Progress indicators for long scans

**Success Criteria:**
- Inline warnings visible on save
- Hover shows finding details
- Status bar reflects scan state
- Performance acceptable (<2s for typical files)

**Deliverables:**
- Updated VSIX package
- Screenshot documentation
- Performance benchmarks

---

### Phase 3: Quick-Fixes (Week 4)

**Goal:** Actionable remediation

**Features:**
- [ ] Code action provider registration
- [ ] Remove secret action
- [ ] Replace with env var action
- [ ] Ignore for session action
- [ ] Add to allowlist action
- [ ] View documentation action

**Success Criteria:**
- Lightbulb appears next to findings
- Quick fixes apply correctly
- Session ignore works until restart
- Allowlist persists to `.coax.yaml`

**Deliverables:**
- Updated VSIX package
- Quick-fix documentation
- Demo video

---

### Phase 4: Polish & Release (Week 5)

**Goal:** Production-ready release

**Features:**
- [ ] Command palette integration
- [ ] Settings UI refinement
- [ ] Error handling and user feedback
- [ ] Logging and diagnostics
- [ ] Comprehensive testing
- [ ] Documentation completion
- [ ] Marketplace submission

**Success Criteria:**
- All commands functional
- Settings UI intuitive
- Error messages helpful
- Documentation complete
- Marketplace listing approved

**Deliverables:**
- v0.8.0 release on VS Code Marketplace
- README with full documentation
- Changelog
- Troubleshooting guide

---

## Estimated Effort

### By Phase

| Phase | Duration | Developer Days | Testing Days |
|-------|----------|----------------|--------------|
| Phase 1: MVP | 2 weeks | 8 days | 2 days |
| Phase 2: Real-time | 1 week | 4 days | 1 day |
| Phase 3: Quick-Fixes | 1 week | 4 days | 1 day |
| Phase 4: Polish | 1 week | 3 days | 2 days |
| **Total** | **5 weeks** | **19 days** | **6 days** |

### By Role

| Role | Effort |
|------|--------|
| TypeScript Developer | 19 days |
| QA Engineer | 6 days |
| Technical Writer | 3 days |
| Designer (icons, UI) | 2 days |
| **Total** | **30 person-days** |

### Resource Requirements

| Resource | Quantity | Duration |
|----------|----------|----------|
| TypeScript Developer | 1 FTE | 5 weeks |
| QA Engineer | 0.5 FTE | 3 weeks |
| Technical Writer | 0.2 FTE | 2 weeks |
| Designer | 0.2 FTE | 1 week |
| Coax CLI Builds | 5 platforms | Before Phase 1 |

---

## Success Criteria

### Functional Requirements

| Requirement | Acceptance Criteria |
|-------------|---------------------|
| **Installation** | Installs from VS Code Marketplace without errors |
| **Activation** | Activates on startup, no manual intervention |
| **Scan on Save** | Triggers scan within 500ms of file save |
| **Scan on Open** | Scans file within 2s of opening |
| **Inline Warnings** | Squiggly underlines appear for all findings |
| **Severity Colors** | Critical=red, High=orange, Medium=yellow, Low=green |
| **Problems Panel** | All findings visible in Problems panel |
| **Hover Tooltips** | Detailed info shown on hover |
| **Quick Fixes** | At least 3 quick-fix actions per finding type |
| **Status Bar** | Accurate finding count and status |
| **Commands** | All 8 commands functional |
| **Configuration** | All settings respected |
| **Performance** | <2s scan time for 100KB file |
| **Stability** | Zero crashes in 100+ test sessions |

### Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| **Extension Size** | <50MB (with bundled binaries) |
| **Memory Usage** | <100MB RAM during scan |
| **CPU Usage** | <20% during scan |
| **Activation Time** | <500ms |
| **Compatibility** | VS Code >=1.85.0 |
| **Platforms** | Windows x64, macOS x64/arm64, Linux x64/arm64 |

### User Experience Requirements

| Requirement | Target |
|-------------|--------|
| **False Positive Rate** | <10% for high/critical findings |
| **Developer Friction** | <5% of users disable extension |
| **Time to First Finding** | <10s from installation |
| **Quick-Fix Adoption** | >50% of users apply at least one quick fix |
| **User Satisfaction** | >4.0 stars on Marketplace |

---

## Risks & Mitigations

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Binary bundling fails on some platforms | Medium | High | Test on all platforms early, provide fallback |
| Performance issues with large files | Medium | Medium | Implement file size limits, progress indicators |
| False positives annoy users | High | High | Configurable severity threshold, easy ignore |
| Binary execution permissions (Linux/macOS) | Low | Medium | Set permissions on activation, clear error messages |
| VS Code API changes | Low | Medium | Pin VS Code engine version, monitor deprecations |

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Coax CLI builds delayed | Medium | High | Start binary builds before extension development |
| TypeScript learning curve | Low | Low | Use experienced developer, leverage examples |
| Testing takes longer than expected | Medium | Medium | Start testing early, automate where possible |
| Marketplace approval delays | Low | Low | Submit early, follow guidelines strictly |

### Adoption Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Developers find it noisy | Medium | High | Default to medium severity, easy to configure |
| Performance concerns | Medium | Medium | Benchmark and publish performance data |
| Competition (GitGuardian, Snyk) | High | Medium | Emphasize unique features (Unicode, local-only) |
| Lack of awareness | High | High | Documentation, blog posts, community engagement |

---

## Future Enhancements (Post-v0.8.0)

### Phase 5: Advanced Features (v0.9.0)

- [ ] Dedicated findings view (tree view in Activity Bar)
- [ ] Baseline file support
- [ ] Live secret verification
- [ ] Cross-file analysis
- [ ] Custom rule configuration UI

### Phase 6: Enterprise Features (v1.0.0)

- [ ] Team configuration sharing
- [ ] Centralized policy management
- [ ] Audit logging
- [ ] SSO integration
- [ ] SARIF export for GitHub Advanced Security

### Phase 7: AI Integration (v1.1.0)

- [ ] LLM-based false positive reduction
- [ ] Intelligent remediation suggestions
- [ ] Context-aware secret detection
- [ ] Automated PR comments

---

## References

### Documentation

- [VS Code Extension API](https://code.visualstudio.com/api)
- [Programmatic Language Features](https://code.visualstudio.com/api/language-extensions/programmatic-language-features)
- [Contribution Points](https://code.visualstudio.com/api/references/contribution-points)

### Example Extensions

- [GitGuardian VS Code Extension](https://docs.gitguardian.com/ggshield-docs/integrations/ide-integrations/vscode)
- [Snyk VS Code Extension](https://github.com/snyk/vscode-extension)
- [VS Code Extension Samples](https://github.com/microsoft/vscode-extension-samples)

### Internal Documents

- `docs/research/vscode-extension-research.md` - Comprehensive research
- `docs/VSCode-EXTENSION-TIMELINE.md` - Development timeline
- `docs/PHASE3-P1-TASKS.md` - Phase 3 task list

---

**Document Created:** 2026-03-16  
**Status:** Ready for Development  
**Next Step:** Begin Phase 1 implementation
