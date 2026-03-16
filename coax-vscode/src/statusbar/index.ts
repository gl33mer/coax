/**
 * Status Bar Service
 * 
 * Manages the Coax status bar indicator.
 */

import * as vscode from 'vscode';
import type { Finding } from '../scanner';

let statusBarItem: vscode.StatusBarItem | null = null;
let currentFindings: Finding[] = [];

/**
 * Initialize the status bar item
 */
export function initializeStatusBar(context: vscode.ExtensionContext): void {
    statusBarItem = vscode.window.createStatusBarItem(
        vscode.StatusBarAlignment.Right,
        100
    );

    statusBarItem.command = 'coax.showFindings';
    statusBarItem.tooltip = 'Coax Security Scanner - Click to view findings';
    statusBarItem.show();

    context.subscriptions.push(statusBarItem);

    updateStatusBar();
}

/**
 * Update the status bar with current findings
 */
export function updateStatusBar(findings?: Finding[]): void {
    if (!statusBarItem) {
        return;
    }

    if (findings !== undefined) {
        currentFindings = findings;
    }

    const config = vscode.workspace.getConfiguration('coax');

    if (!config.get<boolean>('enabled', true)) {
        statusBarItem.text = '$(shield) Coax: Disabled';
        statusBarItem.tooltip = 'Coax is disabled. Click to enable.';
        statusBarItem.color = undefined;
        return;
    }

    const criticalCount = currentFindings.filter(f => f.severity === 'critical').length;
    const highCount = currentFindings.filter(f => f.severity === 'high').length;
    const mediumCount = currentFindings.filter(f => f.severity === 'medium').length;
    const lowCount = currentFindings.filter(f => f.severity === 'low').length;
    const totalCount = currentFindings.length;

    if (totalCount === 0) {
        statusBarItem.text = '$(shield) Coax: Secure';
        statusBarItem.tooltip = 'No security issues found';
        statusBarItem.color = undefined;
    } else if (criticalCount > 0) {
        statusBarItem.text = `$(error) Coax: ${totalCount} issue${totalCount > 1 ? 's' : ''}`;
        statusBarItem.tooltip = `${criticalCount} critical, ${highCount} high, ${mediumCount} medium, ${lowCount} low severity issues`;
        statusBarItem.color = 'red';
    } else if (highCount > 0) {
        statusBarItem.text = `$(warning) Coax: ${totalCount} issue${totalCount > 1 ? 's' : ''}`;
        statusBarItem.tooltip = `${highCount} high, ${mediumCount} medium, ${lowCount} low severity issues`;
        statusBarItem.color = 'yellow';
    } else {
        statusBarItem.text = `$(info) Coax: ${totalCount} issue${totalCount > 1 ? 's' : ''}`;
        statusBarItem.tooltip = `${mediumCount} medium, ${lowCount} low severity issues`;
        statusBarItem.color = 'blue';
    }
}

/**
 * Set the scanning state (shows spinner)
 */
export function setScanningState(isScanning: boolean): void {
    if (!statusBarItem) {
        return;
    }

    if (isScanning) {
        statusBarItem.text = '$(sync~spin) Coax: Scanning...';
        statusBarItem.tooltip = 'Scan in progress...';
        statusBarItem.color = undefined;
    } else {
        updateStatusBar();
    }
}

/**
 * Get the current findings count
 */
export function getFindingsCount(): number {
    return currentFindings.length;
}

/**
 * Get findings by severity
 */
export function getFindingsBySeverity(): { critical: number; high: number; medium: number; low: number } {
    return {
        critical: currentFindings.filter(f => f.severity === 'critical').length,
        high: currentFindings.filter(f => f.severity === 'high').length,
        medium: currentFindings.filter(f => f.severity === 'medium').length,
        low: currentFindings.filter(f => f.severity === 'low').length,
    };
}
