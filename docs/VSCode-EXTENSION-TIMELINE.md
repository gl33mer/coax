# VS Code Extension Development Timeline

**Version:** 0.8.0  
**Start Date:** 2026-03-17  
**Target Release:** 2026-04-21  
**Duration:** 5 weeks (35 calendar days)

---

## Overview

This timeline details the development plan for the Coax VS Code Extension v0.8.0, from project initialization to Marketplace release.

### Key Milestones

| Milestone | Target Date | Deliverable |
|-----------|-------------|-------------|
| **Kickoff** | Week 1, Day 1 | Project setup complete |
| **MVP Demo** | Week 2, Day 5 | Basic scanning works |
| **Feature Complete** | Week 4, Day 5 | All features implemented |
| **QA Complete** | Week 5, Day 3 | Testing complete |
| **Marketplace Release** | Week 5, Day 5 | v0.8.0 published |

---

## Week 1: Project Setup & Binary Integration

### Goals
- Initialize extension project structure
- Set up TypeScript build pipeline
- Bundle Coax CLI binaries
- Create basic "Hello World" extension
- Establish development workflow

### Day 1: Project Initialization

**Tasks:**
- [ ] Install VS Code Extension Development Tools
  ```bash
  npm install -g yo generator-code
  ```
- [ ] Generate extension scaffold
  ```bash
  yo code
  # Select: New Extension (TypeScript)
  # Name: Coax Security Scanner
  # Identifier: coax
  # Description: Real-time Unicode and secret detection
  ```
- [ ] Initialize Git repository
  ```bash
  git init
  git add .
  git commit -m "Initial extension scaffold"
  ```
- [ ] Create project directory structure
  ```
  coax-vscode/
  ├── src/
  │   ├── extension.ts          # Main entry point
  │   ├── scanner/
  │   │   ├── index.ts          # Scanner service
  │   │   └── binary.ts         # Binary execution
  │   ├── diagnostics/
  │   │   ├── index.ts          # Diagnostic collection
  │   │   └── provider.ts       # Diagnostic provider
  │   ├── actions/
  │   │   └── codeActions.ts    # Code action provider
  │   ├── statusbar/
  │   │   └── statusBarItem.ts  # Status bar management
  │   └── commands/
  │       └── commands.ts       # Command handlers
  ├── bundled/                   # Platform-specific binaries
  ├── package.json
  ├── tsconfig.json
  └── .vscodeignore
  ```

**Acceptance Criteria:**
- `npm install` succeeds
- `npm run compile` succeeds
- F5 launches Extension Development Host
- Basic "Hello World" command works

---

### Day 2: Extension Manifest Configuration

**Tasks:**
- [ ] Configure `package.json` activation events
  ```json
  {
    "activationEvents": ["onStartupFinished"]
  }
  ```
- [ ] Define commands in `package.json`
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
- [ ] Define configuration settings
  ```json
  {
    "contributes": {
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
            "description": "Custom path to Coax binary"
          },
          "coax.scanOnSave": {
            "type": "boolean",
            "default": true,
            "description": "Scan files on save"
          }
        }
      }
    }
  }
  ```
- [ ] Create extension icon (128x128 PNG)
- [ ] Update README.md with basic information

**Acceptance Criteria:**
- Commands appear in Command Palette
- Settings appear in Settings UI
- Extension activates on startup

---

### Day 3-4: Binary Bundling

**Tasks:**
- [ ] Create bundled binary directory structure
  ```bash
  mkdir -p bundled/{darwin-arm64,darwin-x64,linux-x64,linux-arm64,win32-x64}
  ```
- [ ] Build Coax CLI for all platforms
  ```bash
  # macOS arm64
  cargo build --release --target aarch64-apple-darwin
  cp target/aarch64-apple-darwin/release/coax bundled/darwin-arm64/
  
  # macOS x64
  cargo build --release --target x86_64-apple-darwin
  cp target/x86_64-apple-darwin/release/coax bundled/darwin-x64/
  
  # Linux x64
  cargo build --release --target x86_64-unknown-linux-gnu
  cp target/x86_64-unknown-linux-gnu/release/coax bundled/linux-x64/
  
  # Linux arm64
  cargo build --release --target aarch64-unknown-linux-gnu
  cp target/aarch64-unknown-linux-gnu/release/coax bundled/linux-arm64/
  
  # Windows x64
  cargo build --release --target x86_64-pc-windows-msvc
  cp target/x86_64-pc-windows-msvc/release/coax.exe bundled/win32-x64/
  ```
- [ ] Create `.vscodeignore` file
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
- [ ] Implement binary path detection
  ```typescript
  // src/scanner/binary.ts
  import { platform, arch } from 'os';
  import * as path from 'path';
  
  export function getBinaryPath(context: ExtensionContext): string {
      const config = workspace.getConfiguration('coax');
      const customPath = config.get<string>('binaryPath', '');
      
      if (customPath && existsSync(customPath)) {
          return customPath;
      }
      
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
- [ ] Implement binary execution
  ```typescript
  import { execFile } from 'child_process';
  
  export async function executeScan(filePath: string): Promise<ScanResult> {
      const binaryPath = getBinaryPath(context);
      
      // Set executable permissions on Unix
      if (process.platform !== 'win32') {
          chmodSync(binaryPath, '755');
      }
      
      return new Promise((resolve, reject) => {
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
                      reject(new Error(`Failed to parse results: ${e}`));
                  }
              }
          );
      });
  }
  ```
- [ ] Test binary execution on current platform

**Acceptance Criteria:**
- All 5 platform binaries built and placed in `bundled/`
- Binary executes successfully on development platform
- `getBinaryPath()` returns correct path
- Extension package size <50MB

---

### Day 5: Basic Extension Logic

**Tasks:**
- [ ] Implement extension activation
  ```typescript
  // src/extension.ts
  import * as vscode from 'vscode';
  import { initializeScanner } from './scanner';
  import { initializeCommands } from './commands';
  import { initializeStatusBar } from './statusbar';
  
  export function activate(context: ExtensionContext) {
      console.log('Coax extension is now active');
      
      initializeScanner(context);
      initializeCommands(context);
      initializeStatusBar(context);
  }
  
  export function deactivate() {
      // Cleanup
  }
  ```
- [ ] Implement basic command handler
  ```typescript
  // src/commands/commands.ts
  export function initializeCommands(context: ExtensionContext) {
      context.subscriptions.push(
          commands.registerCommand('coax.scanCurrentFile', async () => {
              const editor = window.activeTextEditor;
              if (!editor) {
                  window.showWarningMessage('No active editor');
                  return;
              }
              
              window.showInformationMessage(`Scanning ${editor.document.fileName}...`);
              
              try {
                  const results = await executeScan(editor.document.fileName);
                  window.showInformationMessage(
                      `Scan complete: ${results.findings.length} findings`
                  );
              } catch (error) {
                  window.showErrorMessage(`Scan failed: ${error}`);
              }
          })
      );
  }
  ```
- [ ] Test command execution
- [ ] Add basic error handling

**Acceptance Criteria:**
- Extension activates without errors
- `coax.scanCurrentFile` command works
- Success/error messages display correctly

---

## Week 2: Core Scanning & Diagnostics

### Goals
- Implement file watcher service
- Integrate CLI binary execution
- Parse scan results
- Display findings in Problems panel

### Day 1-2: File Watcher Implementation

**Tasks:**
- [ ] Create file watcher service
  ```typescript
  // src/scanner/watcher.ts
  import * as vscode from 'vscode';
  
  export function initializeFileWatcher(context: ExtensionContext): void {
      const config = workspace.getConfiguration('coax');
      
      if (!config.get('scanOnSave') && !config.get('scanOnOpen')) {
          return;
      }
      
      // Watch relevant file types
      const pattern = new RelativePattern(
          workspace.workspaceFolders?.[0]?.uri || Uri.file('/'),
          '**/*.{rs,py,js,ts,jsx,tsx,yml,yaml,json,env,toml,ini,sh}'
      );
      
      const watcher = workspace.createFileSystemWatcher(pattern);
      
      // Scan on save
      if (config.get('scanOnSave')) {
          watcher.onDidChange(async (uri) => {
              await scanFile(uri);
          });
      }
      
      // Scan on open
      if (config.get('scanOnOpen')) {
          window.onDidChangeActiveTextEditor(async (editor) => {
              if (editor) {
                  await scanFile(editor.document.uri);
              }
          });
      }
      
      // Clear diagnostics on delete
      watcher.onDidDelete((uri) => {
          diagnosticCollection.delete(uri);
      });
      
      context.subscriptions.push(watcher);
  }
  ```
- [ ] Implement file filtering
  ```typescript
  function shouldSkipFile(document: TextDocument): boolean {
      const config = workspace.getConfiguration('coax');
      const ignoredPatterns = config.get<string[]>('ignoredPatterns', []);
      const maxFileSize = config.get<number>('maxFileSize', 10485760);
      
      // Check file size
      const stat = fs.statSync(document.uri.fsPath);
      if (stat.size > maxFileSize) {
          return true;
      }
      
      // Check ignored patterns
      for (const pattern of ignoredPatterns) {
          if (minimatch(document.uri.fsPath, pattern)) {
              return true;
          }
      }
      
      // Skip binary files
      const binaryExtensions = ['.bin', '.exe', '.dll', '.pdf', '.zip'];
      const ext = path.extname(document.uri.fsPath).toLowerCase();
      if (binaryExtensions.includes(ext)) {
          return true;
      }
      
      return false;
  }
  ```
- [ ] Test file watcher triggers

**Acceptance Criteria:**
- File save triggers scan
- File open triggers scan
- Ignored files are skipped
- Binary files are skipped

---

### Day 3-4: CLI Integration & Result Parsing

**Tasks:**
- [ ] Define scan result types
  ```typescript
  // src/scanner/types.ts
  export interface ScanResult {
      findings: Finding[];
      metadata: ScanMetadata;
  }
  
  export interface Finding {
      file: string;
      line: number;
      column: number;
      endLine: number;
      endColumn: number;
      type: string;
      severity: 'critical' | 'high' | 'medium' | 'low';
      message: string;
      recommendation: string;
      code: string;  // e.g., 'AWS_ACCESS_KEY'
  }
  
  export interface ScanMetadata {
      filesScanned: number;
      duration: number;
      timestamp: string;
  }
  ```
- [ ] Implement scanner service
  ```typescript
  // src/scanner/index.ts
  export async function scanFile(uri: Uri): Promise<Finding[]> {
      const document = await workspace.openTextDocument(uri);
      
      if (shouldSkipFile(document)) {
          return [];
      }
      
      try {
          const result = await executeScan(document.uri.fsPath);
          updateDiagnostics(document.uri, result.findings);
          return result.findings;
      } catch (error) {
          console.error(`Scan failed for ${uri.fsPath}:`, error);
          return [];
      }
  }
  ```
- [ ] Implement result parsing
  ```typescript
  function parseScanOutput(stdout: string): ScanResult {
      const raw = JSON.parse(stdout);
      
      return {
          findings: raw.findings.map((f: any) => ({
              file: f.file,
              line: f.line,
              column: f.column,
              endLine: f.end_line || f.line,
              endColumn: f.end_column || f.column,
              type: f.type,
              severity: f.severity,
              message: f.message,
              recommendation: f.recommendation,
              code: f.code,
          })),
          metadata: {
              filesScanned: raw.metadata.files_scanned,
              duration: raw.metadata.duration_ms,
              timestamp: raw.metadata.timestamp,
          },
      };
  }
  ```
- [ ] Add timeout handling
- [ ] Add error handling for binary execution failures

**Acceptance Criteria:**
- CLI executes successfully
- JSON output parses correctly
- Findings extracted accurately
- Errors handled gracefully

---

### Day 5: Problems Panel Integration

**Tasks:**
- [ ] Create diagnostic collection
  ```typescript
  // src/diagnostics/index.ts
  import * as vscode from 'vscode';
  
  export const diagnosticCollection = vscode.languages.createDiagnosticCollection('coax');
  
  const severityMap = {
      'critical': vscode.DiagnosticSeverity.Error,
      'high': vscode.DiagnosticSeverity.Warning,
      'medium': vscode.DiagnosticSeverity.Information,
      'low': vscode.DiagnosticSeverity.Hint,
  };
  
  export function updateDiagnostics(uri: Uri, findings: Finding[]): void {
      const diagnostics: Diagnostic[] = [];
      
      for (const finding of findings) {
          const range = new Range(
              finding.line - 1,  // VS Code uses 0-indexed lines
              finding.column - 1,
              finding.endLine - 1,
              finding.endColumn - 1
          );
          
          const diagnostic = new Diagnostic(
              range,
              finding.message,
              severityMap[finding.severity]
          );
          
          diagnostic.code = finding.code;
          diagnostic.source = 'Coax';
          
          diagnostics.push(diagnostic);
      }
      
      diagnosticCollection.set(uri, diagnostics);
  }
  
  export function clearDiagnostics(uri: Uri): void {
      diagnosticCollection.delete(uri);
  }
  ```
- [ ] Wire up diagnostic updates
- [ ] Test Problems panel display
- [ ] Verify severity colors

**Acceptance Criteria:**
- Findings appear in Problems panel (`Ctrl+Shift+M`)
- Severity icons correct (error/warning/info)
- Click on problem navigates to correct line
- Diagnostics clear when file is closed

---

## Week 3: Inline Warnings & Status

### Goals
- Implement squiggly underlines with severity colors
- Add hover tooltips
- Create status bar indicator
- Add progress indicators

### Day 1-2: Squiggly Underlines & Severity Colors

**Tasks:**
- [ ] Verify diagnostic underlines appear (automatic from Week 2)
- [ ] Test severity color mapping
- [ ] Customize colors via theme (optional)
  ```typescript
  // Note: VS Code uses theme colors by default
  // Custom colors require theme contribution
  ```
- [ ] Test with various finding types

**Acceptance Criteria:**
- Red squiggly for critical findings
- Yellow squiggly for high findings
- Blue squiggly for medium findings
- Green squiggly for low findings

---

### Day 3: Hover Tooltips

**Tasks:**
- [ ] Create hover provider
  ```typescript
  // src/diagnostics/hoverProvider.ts
  import * as vscode from 'vscode';
  
  export class CoaxHoverProvider implements HoverProvider {
      public provideHover(
          document: TextDocument,
          position: Position,
          token: CancellationToken
      ): Hover | undefined {
          const diagnostics = diagnosticCollection.get(document.uri);
          if (!diagnostics) return undefined;
          
          for (const diagnostic of diagnostics) {
              if (diagnostic.range.contains(position)) {
                  const contents = new MarkdownString();
                  contents.appendMarkdown(`#### 🔒 ${diagnostic.code}`);
                  contents.appendMarkdown(`\n\n**Severity:** ${this.getSeverityName(diagnostic.severity)}`);
                  contents.appendMarkdown(`\n\n${diagnostic.message}`);
                  contents.appendMarkdown(`\n\n**Recommendation:** ${this.getRecommendation(diagnostic.code)}`);
                  contents.appendMarkdown(`\n\n[Documentation](https://coax.dev/docs/${diagnostic.code.toLowerCase()})`);
                  
                  return new Hover(contents);
              }
          }
          
          return undefined;
      }
      
      private getSeverityName(severity: DiagnosticSeverity): string {
          switch (severity) {
              case DiagnosticSeverity.Error: return 'Critical';
              case DiagnosticSeverity.Warning: return 'High';
              case DiagnosticSeverity.Information: return 'Medium';
              case DiagnosticSeverity.Hint: return 'Low';
          }
      }
      
      private getRecommendation(code: string): string {
          const recommendations: Record<string, string> = {
              'AWS_ACCESS_KEY': 'Remove immediately and rotate via AWS IAM Console',
              'GITHUB_PAT': 'Revoke token and generate new one',
              'STRIPE_KEY': 'Rotate key in Stripe Dashboard',
          };
          return recommendations[code] || 'Remove this secret and use environment variables';
      }
  }
  
  export function initializeHoverProvider(context: ExtensionContext): void {
      context.subscriptions.push(
          languages.registerHoverProvider(
              { language: '*', scheme: '*' },
              new CoaxHoverProvider()
          )
      );
  }
  ```
- [ ] Register hover provider
- [ ] Test hover functionality

**Acceptance Criteria:**
- Hover over finding shows tooltip
- Tooltip includes severity, description, recommendation
- Documentation link works

---

### Day 4: Status Bar Indicator

**Tasks:**
- [ ] Create status bar service
  ```typescript
  // src/statusbar/statusBarItem.ts
  import * as vscode from 'vscode';
  
  let statusBarItem: StatusBarItem;
  let currentFindings: Finding[] = [];
  
  export function initializeStatusBar(context: ExtensionContext): void {
      statusBarItem = window.createStatusBarItem(
          StatusBarAlignment.Right,
          100
      );
      
      statusBarItem.command = 'coax.showFindings';
      statusBarItem.tooltip = 'Coax Security Scanner';
      statusBarItem.show();
      
      context.subscriptions.push(statusBarItem);
      
      updateStatusBar();
  }
  
  export function updateStatusBar(findings?: Finding[]): void {
      if (findings) {
          currentFindings = findings;
      }
      
      const config = workspace.getConfiguration('coax');
      
      if (!config.get('enabled')) {
          statusBarItem.text = '$(shield) Coax: Disabled';
          statusBarItem.tooltip = 'Coax is disabled. Click to enable.';
          statusBarItem.color = undefined;
          return;
      }
      
      const criticalCount = currentFindings.filter(f => f.severity === 'critical').length;
      const highCount = currentFindings.filter(f => f.severity === 'high').length;
      const totalCount = currentFindings.length;
      
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
      } else {
          statusBarItem.text = `$(info) Coax: ${totalCount} issue${totalCount > 1 ? 's' : ''}`;
          statusBarItem.tooltip = `${totalCount} low/medium severity issues`;
          statusBarItem.color = 'blue';
      }
  }
  
  export function setScanningState(isScanning: boolean): void {
      if (isScanning) {
          statusBarItem.text = '$(sync~spin) Coax: Scanning...';
          statusBarItem.tooltip = 'Scan in progress...';
      } else {
          updateStatusBar();
      }
  }
  ```
- [ ] Wire up status bar updates
- [ ] Test state transitions

**Acceptance Criteria:**
- Status bar shows "Secure" when no findings
- Status bar shows count and severity when findings exist
- Click status bar triggers command
- Scanning state shows spinner

---

### Day 5: Progress Indicators

**Tasks:**
- [ ] Add progress indicator for workspace scans
  ```typescript
  // src/commands/commands.ts
  export async function scanWorkspace(): Promise<void> {
      await window.withProgress(
          {
              location: ProgressLocation.Notification,
              title: 'Scanning workspace...',
              cancellable: true,
          },
          async (progress, token) => {
              const files = await getAllWorkspaceFiles();
              const total = files.length;
              let scanned = 0;
              
              for (const file of files) {
                  if (token.isCancellationRequested) {
                      window.showInformationMessage('Scan cancelled');
                      return;
                  }
                  
                  progress.report({
                      message: `${scanned}/${total} files scanned`,
                      increment: (1 / total) * 100,
                  });
                  
                  await scanFile(file);
                  scanned++;
              }
              
              window.showInformationMessage(`Workspace scan complete: ${total} files`);
          }
      );
  }
  ```
- [ ] Test progress display
- [ ] Test cancellation

**Acceptance Criteria:**
- Progress bar shows during workspace scan
- Cancellation works
- Completion message displays

---

## Week 4: Quick-Fix Actions

### Goals
- Implement code action provider
- Create quick-fix actions for common findings
- Wire up diagnostics with code actions

### Day 1-2: Code Action Provider

**Tasks:**
- [ ] Create code action provider
  ```typescript
  // src/actions/codeActions.ts
  import * as vscode from 'vscode';
  
  export class CoaxCodeActionProvider implements CodeActionProvider {
      public static readonly providedKinds = [
          CodeActionKind.QuickFix,
          CodeActionKind.Refactor,
      ];
      
      public provideCodeActions(
          document: TextDocument,
          range: Range | Selection,
          context: CodeActionContext,
          token: CancellationToken
      ): CodeAction[] {
          const actions: CodeAction[] = [];
          
          for (const diagnostic of context.diagnostics) {
              if (diagnostic.source !== 'Coax') {
                  continue;
              }
              
              // Remove secret action
              const removeAction = this.createRemoveAction(document, diagnostic);
              actions.push(removeAction);
              
              // Type-specific actions
              if (diagnostic.code === 'AWS_ACCESS_KEY') {
                  const envAction = this.createEnvVarAction(
                      document,
                      diagnostic,
                      'AWS_ACCESS_KEY_ID'
                  );
                  actions.push(envAction);
              }
              
              if (diagnostic.code === 'GITHUB_PAT') {
                  const envAction = this.createEnvVarAction(
                      document,
                      diagnostic,
                      'GITHUB_TOKEN'
                  );
                  actions.push(envAction);
              }
              
              // Ignore action
              const ignoreAction = this.createIgnoreAction(document, diagnostic);
              actions.push(ignoreAction);
          }
          
          return actions;
      }
      
      private createRemoveAction(
          document: TextDocument,
          diagnostic: Diagnostic
      ): CodeAction {
          const action = new CodeAction(
              'Remove secret',
              CodeActionKind.QuickFix
          );
          
          const edit = new WorkspaceEdit();
          edit.replace(document.uri, diagnostic.range, '');
          action.edit = edit;
          action.diagnostics = [diagnostic];
          action.isPreferred = true;
          
          return action;
      }
      
      private createEnvVarAction(
          document: TextDocument,
          diagnostic: Diagnostic,
          envVar: string
      ): CodeAction {
          const action = new CodeAction(
              `Replace with \\${${envVar}}`,
              CodeActionKind.Refactor
          );
          
          const edit = new WorkspaceEdit();
          edit.replace(document.uri, diagnostic.range, `\\${${envVar}}`);
          action.edit = edit;
          
          return action;
      }
      
      private createIgnoreAction(
          document: TextDocument,
          diagnostic: Diagnostic
      ): CodeAction {
          const action = new CodeAction(
              'Ignore for this session',
              CodeActionKind.QuickFix
          );
          
          action.command = {
              command: 'coax.ignoreFinding',
              title: 'Ignore Finding',
              arguments: [document.uri, diagnostic.range, diagnostic.code],
          };
          
          return action;
      }
  }
  
  export function initializeCodeActions(context: ExtensionContext): void {
      context.subscriptions.push(
          languages.registerCodeActionsProvider(
              { language: '*', scheme: '*' },
              new CoaxCodeActionProvider(),
              { providedCodeActionKinds: CoaxCodeActionProvider.providedKinds }
          )
      );
  }
  ```
- [ ] Register code action provider
- [ ] Test lightbulb appearance

**Acceptance Criteria:**
- Lightbulb appears next to findings
- Code actions listed in menu
- Actions categorized correctly

---

### Day 3-4: Quick-Fix Implementation

**Tasks:**
- [ ] Implement ignore finding command
  ```typescript
  // Session-level ignore storage
  const ignoredFindings = new Map<string, Set<string>>();
  
  export function initializeCommands(context: ExtensionContext): void {
      context.subscriptions.push(
          commands.registerCommand(
              'coax.ignoreFinding',
              async (uri: Uri, range: Range, code: string) => {
                  const key = `${uri.fsPath}:${range.start.line}:${range.start.character}`;
                  
                  if (!ignoredFindings.has(uri.fsPath)) {
                      ignoredFindings.set(uri.fsPath, new Set());
                  }
                  ignoredFindings.get(uri.fsPath)!.add(key);
                  
                  // Remove diagnostic
                  const diagnostics = diagnosticCollection.get(uri) || [];
                  const filtered = diagnostics.filter(d =>
                      !`${d.range.start.line}:${d.range.start.character}`.includes(
                          `${range.start.line}:${range.start.character}`
                      )
                  );
                  diagnosticCollection.set(uri, filtered);
                  
                  window.showInformationMessage('Finding ignored for this session');
              }
          )
      );
  }
  ```
- [ ] Implement add to allowlist command
  ```typescript
  context.subscriptions.push(
      commands.registerCommand(
          'coax.addToAllowlist',
          async (uri: Uri, range: Range, code: string) => {
              const document = await workspace.openTextDocument(uri);
              const line = document.lineAt(range.start.line);
              const pattern = line.text.trim();
              
              // Add to .coax.yaml
              const coaxConfigPath = Uri.joinPath(
                  workspace.workspaceFolders?.[0]?.uri || uri,
                  '.coax.yaml'
              );
              
              let configContent = '';
              try {
                  configContent = await fs.readFile(coaxConfigPath.fsPath, 'utf-8');
              } catch {
                  configContent = 'allowlist:\n  patterns: []\n';
              }
              
              // Append pattern (simple YAML manipulation)
              const updated = configContent.replace(
                  /patterns:\s*\[\]/,
                  `patterns:\n    - "${pattern}"`
              );
              
              await fs.writeFile(coaxConfigPath.fsPath, updated);
              
              window.showInformationMessage('Pattern added to allowlist');
          }
      )
  );
  ```
- [ ] Test quick-fix application
- [ ] Test workspace edit application

**Acceptance Criteria:**
- "Remove secret" deletes the secret
- "Replace with env var" inserts correct syntax
- "Ignore for session" removes diagnostic until restart
- "Add to allowlist" updates `.coax.yaml`

---

### Day 5: Documentation Action

**Tasks:**
- [ ] Add view documentation action
  ```typescript
  private createDocumentationAction(diagnostic: Diagnostic): CodeAction {
      const action = new CodeAction(
          'View documentation',
          CodeActionKind.Empty
      );
      
      action.command = {
          command: 'vscode.open',
          title: 'Open Documentation',
          arguments: [
              Uri.parse(`https://coax.dev/docs/${diagnostic.code!.toLowerCase()}`)
          ],
      };
      
      return action;
  }
  ```
- [ ] Test documentation opening
- [ ] Verify all quick-fixes work together

**Acceptance Criteria:**
- Documentation opens in browser
- All 4+ quick-fixes available per finding
- Quick-fixes apply correctly

---

## Week 5: Polish & Release

### Goals
- Finalize all features
- Comprehensive testing
- Documentation completion
- Marketplace submission

### Day 1: Settings UI & Configuration

**Tasks:**
- [ ] Test all settings in Settings UI
- [ ] Verify settings are respected
- [ ] Add setting descriptions and examples
- [ ] Test configuration file loading (`.coax.yaml`)

**Acceptance Criteria:**
- All settings visible in UI
- Settings affect behavior correctly
- `.coax.yaml` loads on startup

---

### Day 2: Error Handling & Logging

**Tasks:**
- [ ] Add comprehensive error handling
  ```typescript
  try {
      await scanFile(uri);
  } catch (error) {
      const message = error instanceof Error ? error.message : 'Unknown error';
      
      // Log to output channel
      outputChannel.appendLine(`[ERROR] ${new Date().toISOString()} - ${message}`);
      
      // Show user-friendly message
      if (message.includes('ENOENT') || message.includes('not found')) {
          window.showErrorMessage(
              'Coax binary not found. Please ensure Coax CLI is installed.'
          );
      } else if (message.includes('timeout')) {
          window.showWarningMessage(
              'Scan timed out. Consider increasing coax.scanTimeout setting.'
          );
      } else {
          window.showErrorMessage(`Coax scan failed: ${message}`);
      }
  }
  ```
- [ ] Create output channel
  ```typescript
  const outputChannel = window.createOutputChannel('Coax Security');
  
  export function initializeOutputChannel(context: ExtensionContext): void {
      context.subscriptions.push(outputChannel);
      
      outputChannel.appendLine(`[INFO] ${new Date().toISOString()} - Coax extension activated`);
  }
  ```
- [ ] Add logging throughout
- [ ] Test error scenarios

**Acceptance Criteria:**
- Errors handled gracefully
- User-friendly error messages
- Output channel shows detailed logs

---

### Day 3: Comprehensive Testing

**Tasks:**
- [ ] Create test files with various secrets
  ```
  test-secrets/
  ├── aws-keys.txt
  ├── github-tokens.txt
  ├── stripe-keys.txt
  ├── unicode-confusables.txt
  └── clean-file.txt
  ```
- [ ] Test all finding types
- [ ] Test all quick-fixes
- [ ] Test performance with large files
- [ ] Test on all supported platforms
- [ ] Test edge cases (empty files, binary files, etc.)

**Test Cases:**

| Test | Expected Result |
|------|-----------------|
| Save file with AWS key | Red squiggly, Problems panel entry |
| Hover over finding | Tooltip with details |
| Click lightbulb | Quick-fix menu appears |
| Apply "Remove secret" | Secret deleted |
| Status bar shows count | Accurate count |
| Workspace scan | Progress indicator, all files scanned |
| Ignore pattern | File skipped |
| Large file (>10MB) | Skipped or timeout |
| Binary file | Skipped |
| Extension reload | Settings persist |

**Acceptance Criteria:**
- All test cases pass
- Zero crashes
- Performance acceptable

---

### Day 4: Documentation & Packaging

**Tasks:**
- [ ] Write comprehensive README.md
  ```markdown
  # Coax Security Scanner
  
  Real-time Unicode confusable and secret detection for VS Code.
  
  ## Features
  
  - 🔍 Real-time scanning on save and open
  - 🎨 Inline warnings with severity colors
  - 💡 Quick-fix actions
  - 📊 Problems panel integration
  - ⚡ Status bar indicator
  
  ## Installation
  
  1. Install from VS Code Marketplace
  2. Extension activates automatically
  3. Start coding - Coax scans in the background
  
  ## Configuration
  
  ```json
  {
    "coax.enabled": true,
    "coax.scanOnSave": true,
    "coax.scanOnOpen": true,
    "coax.severityThreshold": "medium"
  }
  ```
  
  ## Commands
  
  - `Coax: Scan Current File` - Scan active file
  - `Coax: Scan Workspace` - Scan entire workspace
  - `Coax: Show Findings` - Display all findings
  - `Coax: Settings` - Open settings
  
  ## Quick Fixes
  
  - Remove secret
  - Replace with environment variable
  - Ignore for session
  - Add to allowlist
  
  ## Troubleshooting
  
  ### Binary not found
  Ensure Coax CLI is installed or check coax.binaryPath setting.
  
  ### Too many false positives
  Increase severity threshold or add ignore patterns.
  ```
- [ ] Create CHANGELOG.md
  ```markdown
  # Changelog
  
  ## [0.8.0] - 2026-03-21
  
  ### Added
  - Real-time scanning on save and open
  - Inline warnings with severity colors
  - Problems panel integration
  - Quick-fix actions
  - Status bar indicator
  - Command palette commands
  - Configuration settings
  
  ### Changed
  - Initial release
  ```
- [ ] Create LICENSE file (MIT)
- [ ] Package extension
  ```bash
  npx vsce package
  ```
- [ ] Test VSIX installation

**Acceptance Criteria:**
- README complete and clear
- CHANGELOG accurate
- VSIX package created
- VSIX installs successfully

---

### Day 5: Marketplace Submission

**Tasks:**
- [ ] Create Marketplace publisher account
  ```bash
  npx vsce login coax-security
  ```
- [ ] Prepare marketplace listing
  - Description
  - Tags: security, secrets, unicode, scanner, linter
  - Category: Security, Linters
  - Repository link
  - Bug tracker link
- [ ] Upload extension
  ```bash
  npx vsce publish
  ```
- [ ] Verify listing on Marketplace
- [ ] Share announcement

**Acceptance Criteria:**
- Extension published on Marketplace
- Listing complete with screenshots
- Installation from Marketplace works

---

## Dependencies

### External Dependencies

| Dependency | Required By | Status |
|------------|-------------|--------|
| Coax CLI builds (5 platforms) | Week 1, Day 3 | ⏳ Pending |
| VS Code Extension API docs | Week 1, Day 1 | ✅ Available |
| Example extensions (GitGuardian, Snyk) | Week 1, Day 1 | ✅ Available |
| Extension icon design | Week 1, Day 2 | ⏳ Pending |

### Internal Dependencies

| Dependency | Required By | Status |
|------------|-------------|--------|
| `docs/research/vscode-extension-research.md` | Week 1, Day 1 | ✅ Complete |
| `docs/VSCode-EXTENSION-SPEC.md` | Week 1, Day 1 | ✅ Complete |
| `docs/VSCode-EXTENSION-TIMELINE.md` | Week 1, Day 1 | ✅ Complete |
| Coax CLI JSON output format | Week 2, Day 3 | ⏳ Needs verification |

---

## Risks & Mitigations

### Schedule Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Coax CLI builds delayed | Medium | High | Start builds before Week 1, use cross-compilation |
| Binary bundling issues | Medium | Medium | Test early on each platform, provide fallback |
| TypeScript learning curve | Low | Low | Use experienced developer, reference examples |
| Testing takes longer | Medium | Medium | Start testing early, automate where possible |

### Technical Risks

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| Performance issues | Medium | Medium | Implement file size limits, progress indicators |
| False positives | High | High | Configurable threshold, easy ignore |
| Binary execution failures | Low | Medium | Fallback to system binary, clear errors |

---

## Success Metrics

### Development Metrics

| Metric | Target |
|--------|--------|
| On-time delivery | 100% of milestones met |
| Bug count at release | 0 critical, <5 minor |
| Test coverage | >80% of core logic |
| Documentation completeness | 100% of features documented |

### Quality Metrics

| Metric | Target |
|--------|--------|
| Extension size | <50MB |
| Activation time | <500ms |
| Scan time (100KB file) | <2s |
| Memory usage | <100MB during scan |
| Crash rate | 0% in testing |

### Adoption Metrics (Post-Release)

| Metric | Target (30 days) |
|--------|------------------|
| Downloads | 1,000+ |
| Active users | 500+ |
| Marketplace rating | 4.0+ stars |
| Issue reports | <10 critical |

---

## Post-Release Tasks

### Week 6: Monitoring & Support

- [ ] Monitor crash reports
- [ ] Respond to user issues
- [ ] Track download metrics
- [ ] Gather user feedback
- [ ] Plan v0.8.1 bug fixes

### Week 7-8: v0.9.0 Planning

- [ ] Collect feature requests
- [ ] Prioritize backlog
- [ ] Plan advanced features:
  - Dedicated findings view
  - Baseline file support
  - Live secret verification
  - Cross-file analysis

---

**Timeline Created:** 2026-03-16  
**Development Start:** 2026-03-17  
**Target Release:** 2026-04-21  
**Status:** Ready to Begin
