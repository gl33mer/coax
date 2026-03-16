/**
 * Coax Security Scanner - VS Code Extension
 * 
 * Real-time Unicode confusable and secret detection integrated into VS Code.
 */

import * as vscode from 'vscode';
import { initializeScanner, scanFile, disposeScanner } from './scanner';
import { initializeCommands } from './commands';
import { initializeStatusBar, updateStatusBar, setScanningState } from './statusbar';
import { initializeHoverProvider } from './diagnostics/hoverProvider';
import { initializeCodeActionProvider } from './actions/codeActions';
import { diagnosticCollection } from './diagnostics';

let statusBarItem: vscode.StatusBarItem;
let scanTimeout: NodeJS.Timeout | null = null;

/**
 * Activate the extension
 */
export function activate(context: vscode.ExtensionContext) {
    console.log('Coax Security Scanner is now active');

    // Initialize core services
    initializeScanner(context);
    initializeStatusBar(context);
    initializeCommands(context, { scanFile, updateStatusBar, setScanningState });
    initializeHoverProvider(context);
    initializeCodeActionProvider(context);

    // Set up file watchers with debounce
    initializeFileWatchers(context);

    // Store disposables
    context.subscriptions.push({
        dispose: () => {
            if (scanTimeout) {
                clearTimeout(scanTimeout);
            }
            disposeScanner();
            diagnosticCollection.dispose();
        }
    });

    // Scan all open documents on activation
    scanOpenDocuments();
}

/**
 * Deactivate the extension
 */
export function deactivate() {
    console.log('Coax Security Scanner is now deactivated');
}

/**
 * Initialize file watchers for scan-on-save and scan-on-open
 */
function initializeFileWatchers(context: vscode.ExtensionContext): void {
    const config = vscode.workspace.getConfiguration('coax');
    const debounceDelay = config.get<number>('debounceDelay', 500);

    // Scan on save
    if (config.get<boolean>('scanOnSave', true)) {
        vscode.workspace.onDidSaveTextDocument((document) => {
            if (scanTimeout) {
                clearTimeout(scanTimeout);
            }

            scanTimeout = setTimeout(() => {
                scanFile(document.uri).then((findings) => {
                    updateStatusBar(findings);
                }).catch((error) => {
                    console.error('Scan failed:', error);
                });
            }, debounceDelay);
        }, null, context.subscriptions);
    }

    // Scan on open
    if (config.get<boolean>('scanOnOpen', true)) {
        vscode.window.onDidChangeActiveTextEditor(async (editor) => {
            if (editor) {
                try {
                    const findings = await scanFile(editor.document.uri);
                    updateStatusBar(findings);
                } catch (error) {
                    console.error('Scan failed:', error);
                }
            }
        }, null, context.subscriptions);
    }

    // Clear diagnostics on close
    vscode.workspace.onDidCloseTextDocument((document) => {
        diagnosticCollection.delete(document.uri);
    }, null, context.subscriptions);
}

/**
 * Scan all currently open documents
 */
async function scanOpenDocuments(): Promise<void> {
    const config = vscode.workspace.getConfiguration('coax');
    if (!config.get<boolean>('scanOnOpen', true)) {
        return;
    }

    const openDocuments = vscode.workspace.textDocuments;
    for (const document of openDocuments) {
        if (document.isUntitled || document.isDirty) {
            continue;
        }
        try {
            await scanFile(document.uri);
        } catch (error) {
            console.error(`Failed to scan ${document.fileName}:`, error);
        }
    }
}
