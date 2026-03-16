/**
 * Command Handlers
 * 
 * Implements Coax command palette commands.
 */

import * as vscode from 'vscode';
import { scanFile, scanWorkspace } from '../scanner';
import { updateDiagnostics, clearAllDiagnostics, getAllFindings } from '../diagnostics';
import { updateStatusBar, setScanningState } from '../statusbar';

interface ScanCallbacks {
    scanFile: typeof scanFile;
    updateStatusBar: typeof updateStatusBar;
    setScanningState: typeof setScanningState;
}

/**
 * Initialize command handlers
 */
export function initializeCommands(context: vscode.ExtensionContext, callbacks: ScanCallbacks): void {
    // Scan Current File
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.scanCurrentFile', async () => {
            const editor = vscode.window.activeTextEditor;
            if (!editor) {
                vscode.window.showWarningMessage('No active editor');
                return;
            }

            callbacks.setScanningState(true);

            try {
                const findings = await callbacks.scanFile(editor.document.uri);
                callbacks.updateStatusBar(findings);
                
                if (findings.length === 0) {
                    vscode.window.showInformationMessage(`✓ No issues found in ${editor.document.fileName}`);
                } else {
                    vscode.window.showWarningMessage(
                        `⚠ Found ${findings.length} issue${findings.length > 1 ? 's' : ''} in ${editor.document.fileName}`
                    );
                }
            } catch (error) {
                vscode.window.showErrorMessage(`Scan failed: ${error}`);
            } finally {
                callbacks.setScanningState(false);
            }
        })
    );

    // Scan Workspace
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.scanWorkspace', async () => {
            callbacks.setScanningState(true);

            try {
                await vscode.window.withProgress(
                    {
                        location: vscode.ProgressLocation.Notification,
                        title: 'Scanning workspace...',
                        cancellable: true,
                    },
                    async (progress, token) => {
                        const findings = await scanWorkspace();
                        callbacks.updateStatusBar(findings);

                        if (findings.length === 0) {
                            vscode.window.showInformationMessage('✓ Workspace scan complete. No issues found.');
                        } else {
                            vscode.window.showWarningMessage(
                                `⚠ Workspace scan complete. Found ${findings.length} issues.`
                            );
                        }
                    }
                );
            } catch (error) {
                vscode.window.showErrorMessage(`Workspace scan failed: ${error}`);
            } finally {
                callbacks.setScanningState(false);
            }
        })
    );

    // Show Findings
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.showFindings', () => {
            const findings = getAllFindings();
            
            if (findings.length === 0) {
                vscode.window.showInformationMessage('No security issues found. Your code is secure! 🛡️');
                return;
            }

            // Open Problems panel
            vscode.commands.executeCommand('workbench.actions.view.problems');
        })
    );

    // Settings
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.settings', () => {
            vscode.commands.executeCommand('workbench.action.openSettings', 'Coax');
        })
    );

    // Clear Findings
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.clearFindings', () => {
            clearAllDiagnostics();
            updateStatusBar([]);
            vscode.window.showInformationMessage('All findings cleared.');
        })
    );

    // Ignore Finding
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.ignoreFinding', async (uri: vscode.Uri, range: vscode.Range, code: string) => {
            // For now, just clear the diagnostic at this location
            const diagnostics = vscode.languages.getDiagnostics(uri);
            const filtered = diagnostics.filter(d =>
                !d.range.isEqual(range) || d.code !== code
            );
            vscode.languages.createDiagnosticCollection('coax').set(uri, filtered);
            vscode.window.showInformationMessage('Finding ignored for this session.');
        })
    );

    // Add to Allowlist
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.addToAllowlist', async (uri: vscode.Uri, diagnostic: vscode.Diagnostic) => {
            const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
            if (!workspaceFolder) {
                vscode.window.showErrorMessage('No workspace folder open. Please open a workspace to use allowlist.');
                return;
            }

            const allowlistPath = vscode.Uri.joinPath(workspaceFolder.uri, '.coax.yaml');
            const code = diagnostic.code as string || 'UNKNOWN';

            try {
                // Try to read existing allowlist
                let allowlistContent = '';
                try {
                    const existingData = await vscode.workspace.fs.readFile(allowlistPath);
                    allowlistContent = new TextDecoder().decode(existingData);
                } catch {
                    // File doesn't exist, create new
                    allowlistContent = '# Coax Security Scanner Allowlist\n# Patterns here will be ignored\n\nallowlist:\n  patterns: []\n';
                }

                // Add pattern to allowlist (simple approach - append to patterns list)
                const pattern = `  - pattern: ".*"  # ${code} at ${diagnostic.range.start.line + 1}:${diagnostic.range.start.character + 1}`;
                const newContent = allowlistContent.replace(/(allowlist:\n\s*patterns: \[\])/, `$1\n${pattern}`);

                // Write updated allowlist
                await vscode.workspace.fs.writeFile(allowlistPath, new TextEncoder().encode(newContent));

                // Open the file for user review
                const doc = await vscode.workspace.openTextDocument(allowlistPath);
                await vscode.window.showTextDocument(doc);

                vscode.window.showInformationMessage(`Pattern added to .coax.yaml. Edit the pattern as needed and save.`);

                // Clear the diagnostic
                const diagnostics = vscode.languages.getDiagnostics(uri);
                const filtered = diagnostics.filter(d =>
                    !d.range.isEqual(diagnostic.range) || d.code !== diagnostic.code
                );
                vscode.languages.createDiagnosticCollection('coax').set(uri, filtered);

            } catch (error) {
                vscode.window.showErrorMessage(`Failed to update allowlist: ${error}`);
            }
        })
    );

    // About
    context.subscriptions.push(
        vscode.commands.registerCommand('coax.about', () => {
            vscode.window.showInformationMessage(
                'Coax Security Scanner v0.8.0\n\n' +
                'Real-time Unicode confusable and secret detection for VS Code.\n\n' +
                'Repository: https://github.com/gl33mer/coax',
                'Documentation'
            ).then(selection => {
                if (selection === 'Documentation') {
                    vscode.env.openExternal(vscode.Uri.parse('https://github.com/gl33mer/coax'));
                }
            });
        })
    );
}
