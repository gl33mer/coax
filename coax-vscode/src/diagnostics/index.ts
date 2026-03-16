/**
 * Diagnostic Collection
 * 
 * Manages VS Code diagnostics (squiggly underlines, Problems panel).
 */

import * as vscode from 'vscode';
import type { Finding } from '../scanner';

/**
 * Global diagnostic collection for Coax findings
 */
export const diagnosticCollection = vscode.languages.createDiagnosticCollection('coax');

/**
 * Map Coax severity to VS Code DiagnosticSeverity
 */
const severityMap: Record<string, vscode.DiagnosticSeverity> = {
    'critical': vscode.DiagnosticSeverity.Error,
    'high': vscode.DiagnosticSeverity.Error,
    'medium': vscode.DiagnosticSeverity.Warning,
    'low': vscode.DiagnosticSeverity.Information,
};

/**
 * Update diagnostics for a file
 */
export function updateDiagnostics(uri: vscode.Uri, findings: Finding[]): void {
    const config = vscode.workspace.getConfiguration('coax');
    const threshold = config.get<string>('severityThreshold', 'medium');

    const diagnostics: vscode.Diagnostic[] = [];

    for (const finding of findings) {
        // Filter by severity threshold
        if (shouldFilterByThreshold(finding.severity, threshold)) {
            continue;
        }

        // VS Code uses 0-indexed lines and columns
        const range = new vscode.Range(
            Math.max(0, finding.line - 1),
            Math.max(0, finding.column - 1),
            Math.max(0, finding.endLine - 1),
            Math.max(0, finding.endColumn - 1)
        );

        const diagnostic = new vscode.Diagnostic(
            range,
            finding.message,
            severityMap[finding.severity] || vscode.DiagnosticSeverity.Information
        );

        diagnostic.code = finding.code;
        diagnostic.source = 'Coax';

        // Add finding data for code actions
        diagnostic.relatedInformation = [
            new vscode.DiagnosticRelatedInformation(
                new vscode.Location(uri, range),
                `Type: ${finding.type}`
            )
        ];

        diagnostics.push(diagnostic);
    }

    diagnosticCollection.set(uri, diagnostics);
}

/**
 * Check if a finding should be filtered based on severity threshold
 */
function shouldFilterByThreshold(severity: string, threshold: string): boolean {
    const severityOrder = ['critical', 'high', 'medium', 'low'];
    const thresholdIndex = severityOrder.indexOf(threshold);
    const severityIndex = severityOrder.indexOf(severity);

    if (thresholdIndex === -1 || severityIndex === -1) {
        return false;
    }

    return severityIndex > thresholdIndex;
}

/**
 * Clear diagnostics for a file
 */
export function clearDiagnostics(uri: vscode.Uri): void {
    diagnosticCollection.delete(uri);
}

/**
 * Clear all diagnostics
 */
export function clearAllDiagnostics(): void {
    diagnosticCollection.clear();
}

/**
 * Get all current findings
 */
export function getAllFindings(): Finding[] {
    const findings: Finding[] = [];
    
    for (const [uri, diagnostics] of diagnosticCollection) {
        for (const diagnostic of diagnostics) {
            findings.push({
                file: uri.fsPath,
                line: diagnostic.range.start.line + 1,
                column: diagnostic.range.start.character + 1,
                endLine: diagnostic.range.end.line + 1,
                endColumn: diagnostic.range.end.character + 1,
                type: diagnostic.relatedInformation?.[0]?.message.replace('Type: ', '') || 'Unknown',
                severity: getSeverityFromDiagnostic(diagnostic.severity),
                message: diagnostic.message,
                recommendation: '',
                code: diagnostic.code as string || 'UNKNOWN',
            });
        }
    }

    return findings;
}

/**
 * Get severity string from DiagnosticSeverity
 */
function getSeverityFromDiagnostic(severity: vscode.DiagnosticSeverity): 'critical' | 'high' | 'medium' | 'low' {
    switch (severity) {
        case vscode.DiagnosticSeverity.Error:
            return 'high';
        case vscode.DiagnosticSeverity.Warning:
            return 'medium';
        case vscode.DiagnosticSeverity.Information:
            return 'low';
        default:
            return 'low';
    }
}
